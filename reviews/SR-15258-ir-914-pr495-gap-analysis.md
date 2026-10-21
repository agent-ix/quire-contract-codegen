---
id: SR-15258
title: "IR-914 PR #495 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #495 head 94b9d41bb7f57bf0f6222520c978391aa6690ea6; src/kani/run/execute.rs and tests/it/kani_scalar_native_private.rs; FR-017-AC-42 and FR-040-AC-3/4"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-040
    type: references
---

## Summary

Ticket: IR-914. Exact reviewed head: 94b9d41bb7f57bf0f6222520c978391aa6690ea6. Diff-scoped review of the two changed Rust test functions and their existing requirements.

## Scope

```yaml
{
  "scope": [
    {
      "id": "FR-017-AC-42",
      "path": "spec/kani/functional/FR-017-kani-execution-evidence.md",
      "role": "examined",
      "excerpt": "`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`, `Inconclusive` `TimedOut`, `MemoryExhausted` or"
    },
    {
      "id": "FR-040-AC-3",
      "path": "spec/kani/functional/FR-040-cancellation-producer.md",
      "role": "examined",
      "excerpt": "A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline` event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that result into `Cancelled`."
    },
    {
      "id": "FR-040-AC-4",
      "path": "spec/kani/functional/FR-040-cancellation-producer.md",
      "role": "examined",
      "excerpt": "Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not overwritten by later completion or elapsed time."
    }
  ]
}
```

## Verdict

**CONDITIONAL** — the touched scalar control still exercises FR-017-AC-42, but the capture control can satisfy its output-limit assertion before cancellation is ever sent. Plan completion: not assessed. Computed matrix status for the touched ACs is separately recorded; no repository-wide gap claim is made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The capture precedence test checks receiver.try_recv() before calls.ready and cancels only after seeing calls.ready. An OutputOverLimit result can arrive first and satisfy the assertion without any cancellation event. Thus this tagged control can pass without exercising AC-4 cancellation competition; assert a positive cancellation event, staged before the output stop, or narrow the AC-4 binding. | src/kani/run/execute.rs:2205-2225 |

## Coverage

Plan completion: not assessed. Computed matrix and test trace bindings for FR-017-AC-42 and FR-040-AC-3/4 were examined. No production source, spec, or trace tag was added. Optional repository-wide semantic review was not requested.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At c9a22a6f, `OwnerControl::entered()` acknowledges the real pidfd confirmation poll, then the test records `Requested` cancellation before releasing the gate and joining the worker. The result cannot return before cancellation is sent. |
| FND-002 | still-open | At ff25c9d4 the test still calls `paused.entered()`, which uses `recv_timeout(T)` with T=30 seconds, although its local launcher ceiling is 90 seconds; the later `OwnerGate::enter()` release wait is also 30 seconds. |
| FND-002 | fixed | At 510d683b, the test calls `paused.entered.recv_timeout(ceiling)` with ceiling 90 seconds, and `OwnerGate::within(ceiling)` uses the same ceiling for its release wait. |
| FND-003 | fixed | At 510d683b, stdout uses `SystemPoll` throughout. The backend waits for `calls.start` before writing `CAPTURE_LIMIT + 1`; the controller starts it only after the real memory-observation gate is held, waits for `calls.written`, sends Requested, then releases that gate. No paused stdout reader participates in a circular wait. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The new test waits for the real cleanup gate with `OwnerControl::entered()`, whose `recv_timeout(T)` is still 30 seconds. The same test raises the launcher ceiling to 90 seconds specifically for loaded setup and capture. A valid run that reaches cleanup after 30 seconds but before its 90 second ceiling now panics at the control wait, so the hotfix can still flake under the load it addresses. Use the local 90 second ceiling for this acknowledgement wait. | src/kani/run/execute.rs:2217,2520-2523 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The stdout `ControlledPoll` gates whenever `calls.ready` exists, but that file proves the backend wrote `CAPTURE_LIMIT` bytes, not that capture drained them. If the pipe is full when the next poll is gated, the backend blocks on `printf 'x'` and cannot write `calls.extra`; the controller waits for that marker before releasing the poll. This creates a circular wait (eventually the 90-second assertion and 30-second gate panic) under a valid pipe scheduling interleaving. Stage the extra-byte write without requiring an already-paused full pipe, or release capture before waiting for its marker. | src/kani/run/execute.rs:2191-2193,2205-2207,2219-2227 |

## Current verdict (disposition pass 3)

**PASS** at 510d683bf0fbb18b10e475ed7d175d551b6ff294. FND-001 remains fixed and FND-002/FND-003 are fixed. The real memory-observation gate holds the monitor while unpaused stdout capture processes the over-limit byte. The capture thread can set its failure flag before the backend writes the `calls.written` marker, but it does not stop the launcher; `wait_until_at` is blocked in the observation gate, so the marker can be written. After the controller sees the marker it records Requested, releases the monitor, and the settled OutputOverLimit refusal takes precedence. Focused exact-head execution is owned by the author and was not rerun in this review.
