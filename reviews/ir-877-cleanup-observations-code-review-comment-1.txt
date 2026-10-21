<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15290 method=code-review lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15290 code-review, part 1/1. CONDITIONAL: one low test-fixture duplication finding; no production semantic defect identified. The final aggregate remains pending.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-001 | low | other | The new test repeats the exact finishable namespace launcher fixture already present in cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence at execute.rs:2946-2952. Changing the shared ready/finish/report protocol now requires two edits and can leave one cancellation control exercising stale setup. Give this fixture one test-owned home and reuse it in both controls without weakening their distinct assertions. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-017-AC-54
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A successful claimed-init confirmation poll with no `IN` yields public `MemoryObservationFailed`
    with the existing outer and nested prefixes and labelled observations of that attempt: returned poll
    count, raw revents bits, monotonic poll elapsed duration with its unit, captured init PID/start ticks,
    captured wrapper PID or explicit absence, init-handle presence, gate presence and cleaned state. Test
    controls through the existing `PollOperation` seam delegate first to real poll on the real owned pidfd,
    record its positive count and `IN`, then clear revents and return either zero or that genuine'
- id: FR-017-AC-55
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Through the real public execution entry and real owned namespace, an unchanged real-poll control
    retains its original public result and confirmed cleanup; each AC-54 omission refuses even when cancellation
    is submitted before or after the cleanup barrier. The confirmation seam sees the same claimed pidfd
    and exactly one poll with timeout `(5, 0)` per cleanup invocation. Failure retains gate/cleaned state
    and the existing `Drop` retry, and actual backend processes/namespace are gone after return. The existing
    real-poll-delegating errno control retains its public refusal rather than cancellation
- id: FR-017-AC-56
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Inspection of the cleanup change shows that init-pidfd KILL still precedes wrapper-group KILL,
    the single confirmation poll retains its `(5, 0)` timeout, signal-error/poll-error/missing-readiness
    precedence is unchanged, syscall errors retain their original kind and errno, and only successful
    cleanup closes the gate and marks cleaned. Existing `Drop` behavior is unchanged. Diagnostic observations
    come only from that poll and existing owned fields; no added wait, retry, skip, PID reopen, procfs/cgroup
    rescan or invented host-state field is introduced.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result; measured memory exhaustion,
    memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing
    result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first
    and causing stop is not overwritten by later completion or elapsed time.
- id: NamespaceOwner::cleanup
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: Kill init on every conclusion, then confirm kernel namespace teardown before accepting it.
- id: ObservationOmittingPoll::poll
  path: src/kani/run/execute.rs
  role: examined
  excerpt: Omit only an observation after the actual supplied pidfd has confirmed teardown.
- id: TC-043-step-14
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: examined
  excerpt: Drive the real public execution entry with a launcher stand-in and a genuine owned namespace,
    using the existing confirmation PollOperation seam.
findings:
- fnd: FND-001
  severity: low
  confidence: high
  method: code-review
  check: other
  artifact_id: src/kani/run/execute.rs::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  related_ids:
  - FR-017-AC-55
  path: src/kani/run/execute.rs
  lines: 3023-3029
  excerpt: "let body = format!(\n    r#\"printf '%s' \"$(readlink /proc/self/ns/pid)\" > \"$CALLS.namespace\"\
    \nprintf '%s' \"$$\" > \"$CALLS.child\"\nprintf 'ready' > \"$CALLS.ready\"\nwhile [ ! -f \"$CALLS.finish\"\
    \ ]; do sleep 0.01; done\n{VERIFY_EVERY_HARNESS}\"#\n);"
  finding: The new test repeats the exact finishable namespace launcher fixture already present in cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence
    at execute.rs:2946-2952. Changing the shared ready/finish/report protocol now requires two edits and
    can leave one cancellation control exercising stale setup. Give this fixture one test-owned home and
    reuse it in both controls without weakening their distinct assertions.
bindings:
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-54
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-55
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-040-AC-4
  trace: correct
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'CONDITIONAL: one low test-fixture duplication finding; no production semantic defect
  identified. The final aggregate remains pending.'
part: 1
parts: 1
scope_kind: changed code and owning criteria
```

+++