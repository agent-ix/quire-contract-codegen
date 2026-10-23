---
id: SR-15716
title: "IR-980 CG544 code-review independent review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-980; PR544; exact 99f2b7e9c43a40e3de90080c7e1ac9d1af887f37; test-only namespace observation controls"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-017", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-040", type: references }
---

## Summary

Independent non-author PR-diff review of CG #544 / IR-980 at exact `99f2b7e9c43a40e3de90080c7e1ac9d1af887f37`, base `5f52fca2a3481d791230447933aa3a5ff83ce4c4`. Only src/kani/run/execute.rs and src/kani/run/namespace.rs change. Both changed ranges are within existing cfg(test) modules; each complete production prefix compares byte-for-byte with the base. src/kani/run/launch.rs has no diff. No author source/branch edit was made. No applicable AssuranceProfile found.


## Verdict

CONDITIONAL — one medium trace-binding finding; no production behavior defect found. Final runtime/parallel qualification remains separate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Both new confirmation-refusal tests falsely bind to AC-24's live-grandchild cleanup behavior. | src/kani/run/execute.rs:2801; src/kani/run/namespace.rs:525 |

## Coverage

Source review checked the genuine same-owned-pidfd readiness boundary, errno propagation, clearing of revents before delegation, and typed missing-readiness refusal. PreconfirmedPoll waits up to30seconds for real IN, clears revents, then delegates ControlledPoll with the owner's original timeout. ControlledPoll still performs a real poll and its existing synchronization gate. The namespace adverse seam returns empty only on the first call; its separately invoked second confirmation waits for actual readiness and then delegates the unchanged poll. The public adverse seam first confirms the real pidfd, intentionally omits that observation, asserts the specific public MemoryObservationFailed prefix and two owner calls (failure plus Drop retry), then checks actual recorded process disappearance with namespace-aware identity. No fixture reports an invented successful pidfd result.

The unchanged cancellation test still executes both withhold=false/true arms, checks shared first-cause semantics, preserves the independent c::check group Verified outcome, distinct members and launcher count, and uses release/abort/join cleanup. Production init-kill/group-kill ordering, single (5,0) confirmation poll, cleaned/gate mutation and Drop behavior are byte-identical to base; test decorators may add readiness coordination as IR-980 explicitly authorizes, not a production wait/retry.

Reviewer independently ran only cheap read-only checks: fmt --check, PR diff --check, current computed matrix, and production-prefix/launch byte comparison. No Cargo build/test, gate, Kani/engine or namespace execution. Author's red investigation and focused greens/lint were read as reported evidence, not locally rerun or certified; raw qualification-logs/ir980 are remote/unavailable on Titan. Final default-parallel make ci and repeated focused default-thread runs remain author/lead qualification obligations; this review grants no such receipt.

Plan completion: not assessed

## Examined scope and bindings

```yaml
{
  "head": "99f2b7e9c43a40e3de90080c7e1ac9d1af887f37",
  "base": "5f52fca2a3481d791230447933aa3a5ff83ce4c4",
  "clean": false,
  "scope": [
    {
      "id": "IR-980-AC-1",
      "path": "src/kani/run/execute.rs",
      "role": "examined",
      "excerpt": "A real-namespace adverse control injects missing confirmation at the existing poll seam and asserts the public typed MemoryObservationFailed with no silent success."
    },
    {
      "id": "IR-980-AC-2",
      "path": "src/kani/run/execute.rs",
      "role": "examined",
      "excerpt": "Test-side coordination may wait for genuine pidfd readiness before the production confirmation poll; it may not fabricate readiness or change production cleanup."
    },
    {
      "id": "FR-017-AC-24",
      "path": "spec/kani/functional/FR-017-kani-execution-evidence.md",
      "role": "examined",
      "excerpt": "A launcher that exits on its own after leaving a real grandchild in its process group has that grandchild killed by the time the run returns, the same as on a timeout."
    },
    {
      "id": "FR-040-AC-4",
      "path": "spec/kani/functional/FR-040-cancellation-producer.md",
      "role": "examined",
      "excerpt": "Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not overwritten by later completion or elapsed time."
    },
    {
      "id": "FR-017-AC-55",
      "path": "spec/kani/functional/FR-017-kani-execution-evidence.md",
      "role": "context_only",
      "excerpt": "The confirmation seam sees the same claimed pidfd and exactly one poll with timeout `(5, 0)` per cleanup invocation."
    },
    {
      "id": "FR-017-AC-56",
      "path": "spec/kani/functional/FR-017-kani-execution-evidence.md",
      "role": "examined",
      "excerpt": "Inspection of the cleanup change shows that init-pidfd KILL still precedes wrapper-group KILL, the single confirmation poll retains its `(5, 0)` timeout, signal-error/poll-error/missing-readiness precedence is unchanged"
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "medium",
      "confidence": "high",
      "method": "code-review",
      "check": "trace",
      "artifact_id": "FR-017-AC-24",
      "related_ids": [
        "IR-980-AC-1"
      ],
      "path": "src/kani/run/execute.rs",
      "lines": "2801-2804; src/kani/run/namespace.rs:525-527",
      "excerpt": "A launcher that exits on its own after leaving a real grandchild in its process group has that grandchild killed by the time the run returns, the same as on a timeout.",
      "finding": "Both new confirmation-omission tests carry FR-017-AC-24, but neither creates a launcher that exits while leaving a live real grandchild in its process group. The namespace test launches true; the public test runs a finite report-producing shell and records only its own PID. They test missing-confirmation refusal, so they can stay green without exercising AC-24's surviving-grandchild condition. Remove that false trace claim and bind the tests only to a reviewed requirement that owns the injected confirmation-refusal behavior; request a scoped FR binding if the ticket obligation has no suitable repository criterion. Existing actual-grandchild tests remain unchanged.",
      "test_id": "omitted_namespace_confirmation_refuses_public_evidence",
      "related_test_ids": [
        "empty_namespace_confirmation_refuses_before_later_real_confirmation"
      ],
      "claimed_ac": "FR-017-AC-24",
      "correct_ac": "IR-980-AC-1 (owning ticket obligation; repository FR binding must be identified or authored, not invented)"
    }
  ],
  "bindings": [
    {
      "test_id": "omitted_namespace_confirmation_refuses_public_evidence",
      "ac_id": "FR-017-AC-24",
      "trace": "wrong"
    },
    {
      "test_id": "omitted_namespace_confirmation_refuses_public_evidence",
      "ac_id": "FR-040-AC-4",
      "trace": "correct"
    },
    {
      "test_id": "empty_namespace_confirmation_refuses_before_later_real_confirmation",
      "ac_id": "FR-017-AC-24",
      "trace": "wrong"
    },
    {
      "test_id": "empty_namespace_confirmation_refuses_before_later_real_confirmation",
      "ac_id": "FR-040-AC-4",
      "trace": "correct"
    },
    {
      "test_id": "controlled_caller_order_distinguishes_cancelled_and_completed_batch",
      "ac_id": "FR-040-AC-3",
      "trace": "correct"
    },
    {
      "test_id": "controlled_caller_order_distinguishes_cancelled_and_completed_batch",
      "ac_id": "FR-040-AC-5",
      "trace": "correct"
    }
  ],
  "matrix": {
    "FR-017-AC-24": {
      "status": "tagged",
      "binders": 4
    },
    "FR-017-AC-55": {
      "status": "untagged",
      "binders": 0
    },
    "FR-017-AC-56": {
      "status": "method-without-symbol",
      "binders": 0
    },
    "FR-040-AC-3": {
      "status": "tagged",
      "binders": 4
    },
    "FR-040-AC-4": {
      "status": "tagged",
      "binders": 9
    },
    "FR-040-AC-5": {
      "status": "tagged",
      "binders": 3
    }
  },
  "production_prefixes_byte_identical": true,
  "launch_unchanged": true
}
```


## Dispositions

### Round 1 — exact `0205b325bdedb2cfce747b0c1a208cef47c34ad3`

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both omission controls remove FR-017-AC-24 and FR-040-AC-4 and bind only FR-017-AC-55's confirmation/refusal/cleanup boundary. |

After excerpt, verbatim in both changed tests:

```rust
#[trace("FR-017-AC-55")]
```

Narrow same-reviewer disposition: PASS, no new findings in `99f2b7e9..0205b325bdedb2cfce747b0c1a208cef47c34ad3`. The new AC-55 bindings are partial confirmation-refusal/cleanup coverage; these two controls alone do not establish its complete cancellation-barrier, diagnostic, timeout and errno contract. Historical matrix values and original finding above remain historical, not current coverage claims. No full-criterion acceptance is implied.

The intervening `02638c20` extension makes test-only PreconfirmedPoll generic over the existing PollOperation and installs it over real SystemPoll only for the 8-second and 30-second batch timing stand-ins. It first obtains genuine IN from the same pidfd with a bounded real poll, clears revents, then delegates with the unchanged owner timeout. The existing controlled-order use retains ControlledPoll and both independent withholding gates. It neither fabricates readiness nor adds production polling/retries. Existing timing assertions and independent batch results remain unchanged. Complete production prefixes in both files compare byte-identically with the prior reviewed head; launch.rs has no change in this revision range.

Independently checked exact fetched head, complete narrow diff, AC-55 text, unchanged production prefixes, fmt check and diff check. No Cargo build/test, gate, engine, namespace execution or author edits. The reported batch 54/54 and lint evidence was not locally rerun; `/home/peter/dev/qualification-logs/ir980/batch-module-extended.log` is absent on this host, so author runtime evidence is reported, not independently certified. Final qualification remains separate. Plan completion: not assessed.
