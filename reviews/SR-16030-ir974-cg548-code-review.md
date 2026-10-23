---
id: SR-16030
title: "IR-974 CG #548 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #548; src/lib.rs; src/smt/mod.rs; src/smt/pair_tests.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: references }
---

## Summary

Independent diff-scoped code-review of CG PR #548 at exact head 9204c1a379a95b6e2b11ae840f23480bee627dc9 on base aac7e5113c549fa58d59d9342e2144641369cc43. QSL owns query encoding and terminal settlement; the PR implements CG run evidence only.

## Verdict

FAIL — solver evidence eligibility, parsing and resource/attempt boundaries have defects. No aggregate gate was run by this reviewer.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The public classifier accepts caller-constructed QueryRun values and checks no ProcessRun completion; two synthetic Unsat records produce InductionEligible without any solver process. Require completed associated execution evidence before eligibility. | src/smt/mod.rs:187-212 |
| FND-002 | high | After recognizing unsat, the parser ignores any unrecognized later line; a successful child printing unsat followed by junk is classified Unsat instead of MalformedResponse. Reject malformed trailing output while retaining valid model/proof output. | src/smt/mod.rs:135-159 |
| FND-003 | medium | With base Sat and step Unknown, the early non-proof filter skips Unknown and classify_pair returns BaseSatCandidate. The unknown step response loses its distinct non-proof disposition. | src/smt/mod.rs:199-216 |
| FND-004 | medium | QueryAssociation.attempt is a public caller-supplied field and run_solver_query never assigns or checks it. Two invocations may use the same attempt, so the promised distinct attempt identity cannot prevent stale response reuse. | src/smt/mod.rs:53-69 |
| FND-005 | medium | run_solver_query clones the entire query byte vector before run_bounded_process checks input_bytes. A far-over-limit query allocates a second unbounded buffer before the required typed pre-spawn refusal, risking OOM instead of Stopped(InputBytes). | src/smt/mod.rs:102-109 |

## Coverage

FR-045-AC-1..3 and TC-059 were examined. The computed matrix tags all three ACs but provides no run evidence. The QSL FR-315 encoder and QSL terminal settlement remain external dependencies; this review makes no full AC-1 completion claim.

Plan completion: not assessed. Optional whole-repository semantic audit not requested.

## Examined scope and bindings

```yaml
{
  "clean": false,
  "scope": [
    {
      "id": "FR-045-AC-1",
      "path": "spec/routed/functional/FR-045-hyper-smt-induction-map.md",
      "role": "examined",
      "excerpt": "For an admitted HP-2 safety clause and `k = 1`, CG requests QSL's base and step queries using FR-315's clause, ordered subjects, instance and Meter shape, submits each canonical byte sequence unchanged to the selected solver, and retains the product, item, instance, depth, kind and exact bytes in each query record. Matching `Unsat`/`Unsat` records are eligible for QSL-owned `Inductive { depth: 1 }`; QSL FR-182-AC-7, rather than CG, settles that outcome as `proved`, `decisive-witness`, `Uncertified`."
    },
    {
      "id": "FR-045-AC-2",
      "path": "spec/routed/functional/FR-045-hyper-smt-induction-map.md",
      "role": "examined",
      "excerpt": "A base `Sat` response remains only a candidate for QSL reconstruction and replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding refusal, resource stop, missing or malformed response, process failure and a response paired with another product, item, instance, depth, kind or query bytes remain distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as incomplete. Every branch retains its available identities and depth, and no CG branch claims terminal refutation or certification."
    },
    {
      "id": "FR-045-AC-3",
      "path": "spec/routed/functional/FR-045-hyper-smt-induction-map.md",
      "role": "examined",
      "excerpt": "CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying "
    }
  ],
  "findings": [
    {
      "fnd": "FND-001",
      "severity": "high",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-045-AC-3",
      "related_ids": [],
      "path": "src/smt/mod.rs",
      "lines": "187-212",
      "excerpt": "CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying ",
      "finding": "The public classifier accepts caller-constructed QueryRun values and checks no ProcessRun completion; two synthetic Unsat records produce InductionEligible without any solver process. Require completed associated execution evidence before eligibility."
    },
    {
      "fnd": "FND-002",
      "severity": "high",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-045-AC-2",
      "related_ids": [],
      "path": "src/smt/mod.rs",
      "lines": "135-159",
      "excerpt": "A base `Sat` response remains only a candidate for QSL reconstruction and replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding refusal, resource stop, missing or malformed response, process failure and a response paired with another product, item, instance, depth, kind or query bytes remain distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as incomplete. Every branch retains its available identities and depth, and no CG branch claims terminal refutation or certification.",
      "finding": "After recognizing unsat, the parser ignores any unrecognized later line; a successful child printing unsat followed by junk is classified Unsat instead of MalformedResponse. Reject malformed trailing output while retaining valid model/proof output."
    },
    {
      "fnd": "FND-003",
      "severity": "medium",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-045-AC-2",
      "related_ids": [],
      "path": "src/smt/mod.rs",
      "lines": "199-216",
      "excerpt": "A base `Sat` response remains only a candidate for QSL reconstruction and replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding refusal, resource stop, missing or malformed response, process failure and a response paired with another product, item, instance, depth, kind or query bytes remain distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as incomplete. Every branch retains its available identities and depth, and no CG branch claims terminal refutation or certification.",
      "finding": "With base Sat and step Unknown, the early non-proof filter skips Unknown and classify_pair returns BaseSatCandidate. The unknown step response loses its distinct non-proof disposition."
    },
    {
      "fnd": "FND-004",
      "severity": "medium",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-045-AC-3",
      "related_ids": [],
      "path": "src/smt/mod.rs",
      "lines": "53-69",
      "excerpt": "CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying ",
      "finding": "QueryAssociation.attempt is a public caller-supplied field and run_solver_query never assigns or checks it. Two invocations may use the same attempt, so the promised distinct attempt identity cannot prevent stale response reuse."
    },
    {
      "fnd": "FND-005",
      "severity": "medium",
      "confidence": "high",
      "method": "code-review",
      "check": "code-bug",
      "artifact_id": "FR-045-AC-3",
      "related_ids": [],
      "path": "src/smt/mod.rs",
      "lines": "102-109",
      "excerpt": "CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying ",
      "finding": "run_solver_query clones the entire query byte vector before run_bounded_process checks input_bytes. A far-over-limit query allocates a second unbounded buffer before the required typed pre-spawn refusal, risking OOM instead of Stopped(InputBytes)."
    }
  ],
  "bindings": []
}
```

## Dispositions

| FND | Outcome | After excerpt | Reason |
| --- | --- | --- | --- |
| FND-001 | fixed | `base: Option<&SolverExecution<Product, Item, Instance, Kind>>,` | Private execution evidence is required; completed, successful process status is checked. |
| FND-002 | fixed | `return if valid_solver_tail(&stdout[offset..]) {` | Unrecognized or error tail now refuses instead of retaining Unsat. |
| FND-003 | fixed | `SolverStatus::Unknown` | Unknown is handled in the non-proof precedence loop. |
| FND-004 | fixed | `attempt: next_attempt()?,` | CG assigns a checked distinct attempt during execution. |
| FND-005 | fixed | `if association.selection.query_bytes.len() > limits.input_bytes.get() {` | Input length is checked before cloning the query into ProcessRequest. |

## Final PR-head recheck

PASS for the CG-owned slice at exact head `a502daa91ddb6d6fb424bb80dffda0346ad97f2c`, unchanged base `aac7e5113c549fa58d59d9342e2144641369cc43`. All listed findings are fixed. Full FR-045-AC-1 remains open pending QSL FR-315 encoder and QSL settlement. Reviewer reran `cargo test --locked --lib smt::pair_tests`: 4 passed, 0 failed, and `git diff --check` passed. No aggregate gate or engine run was performed by this reviewer.
