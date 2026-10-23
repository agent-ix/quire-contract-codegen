---
id: SR-16031
title: "IR-974 CG #548 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #548; src/lib.rs; src/smt/mod.rs; src/smt/pair_tests.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: references }
---

## Summary

Independent diff-scoped gap-analysis of CG PR #548 at exact head 9204c1a379a95b6e2b11ae840f23480bee627dc9 on base aac7e5113c549fa58d59d9342e2144641369cc43. QSL owns query encoding and terminal settlement; the PR implements CG run evidence only.

## Verdict

FAIL — the AC-1 trace is partial and the two-process AC-3 branch is unbacked. No aggregate gate was run by this reviewer.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The AC-1 tagged test uses hand-built query bytes and synthetic QueryRun statuses; it never calls QSL encode_hyper_query or runs both selected queries. The computed matrix reports AC-1 tagged, but the actual end-to-end criterion remains unbacked pending QSL-1010. Mark this partial test scope explicitly and avoid presenting the AC as verified. | src/smt/pair_tests.rs:45-99 |
| FND-002 | medium | The process tests run one query at a time and the pair tests use synthetic records. None asserts two direct child invocations with distinct attempt identities and no stale response acceptance; this AC-3 branch remains unbacked. | src/smt/pair_tests.rs:166-279 |

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
      "method": "gap-analysis",
      "check": "trace",
      "artifact_id": "FR-045-AC-1",
      "related_ids": [],
      "path": "src/smt/pair_tests.rs",
      "lines": "45-99",
      "excerpt": "For an admitted HP-2 safety clause and `k = 1`, CG requests QSL's base and step queries using FR-315's clause, ordered subjects, instance and Meter shape, submits each canonical byte sequence unchanged to the selected solver, and retains the product, item, instance, depth, kind and exact bytes in each query record. Matching `Unsat`/`Unsat` records are eligible for QSL-owned `Inductive { depth: 1 }`; QSL FR-182-AC-7, rather than CG, settles that outcome as `proved`, `decisive-witness`, `Uncertified`.",
      "finding": "The AC-1 tagged test uses hand-built query bytes and synthetic QueryRun statuses; it never calls QSL encode_hyper_query or runs both selected queries. The computed matrix reports AC-1 tagged, but the actual end-to-end criterion remains unbacked pending QSL-1010. Mark this partial test scope explicitly and avoid presenting the AC as verified."
    },
    {
      "fnd": "FND-002",
      "severity": "medium",
      "confidence": "high",
      "method": "gap-analysis",
      "check": "coverage",
      "artifact_id": "FR-045-AC-3",
      "related_ids": [],
      "path": "src/smt/pair_tests.rs",
      "lines": "166-279",
      "excerpt": "CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits; it retains the executable, arguments, limits, completion, status, captured output and any emitted model/proof bytes in a CG-owned run record. An input-byte excess refuses before spawn, a missing executable gives `MissingTool`, and a reached limit gives `Stopped` with that limit; none becomes `Unsat` or a QSL outcome. Separate attempt and query identities prevent a stale or swapped response from satisfying ",
      "finding": "The process tests run one query at a time and the pair tests use synthetic records. None asserts two direct child invocations with distinct attempt identities and no stale response acceptance; this AC-3 branch remains unbacked."
    }
  ],
  "bindings": []
}
```

## Dispositions

| FND | Outcome | After excerpt | Reason |
| --- | --- | --- | --- |
| FND-001 | fixed | `#[trace("TC-059", "FR-045-AC-2", "FR-045-AC-3")]` | The false full-AC-1 tag was removed; full AC-1 remains open on QSL. |
| FND-002 | fixed | `assert_ne!(association(&base).attempt(), association(&step).attempt());` | Two real bounded child runs and a stale repeat are now asserted. |

## Final PR-head recheck

PASS for the CG-owned slice at exact head `a502daa91ddb6d6fb424bb80dffda0346ad97f2c`, unchanged base `aac7e5113c549fa58d59d9342e2144641369cc43`. All listed findings are fixed. Full FR-045-AC-1 remains open pending QSL FR-315 encoder and QSL settlement. Reviewer reran `cargo test --locked --lib smt::pair_tests`: 4 passed, 0 failed, and `git diff --check` passed. No aggregate gate or engine run was performed by this reviewer.

## AD-004 layout disposition

PASS at exact PR head `48cda22b5a384fe7c45f6396fe345bb5bc23c66c` on base `b68193997a15fd37ea3d25132c39f83138ed54dc`. The parent `1132871afe69cd255cdf0ebf1fe65fade697747b` to this head renames `src/smt/mod.rs` and `src/smt/pair_tests.rs` to `src/routed/smt/` with both file contents byte-identical; `src/routed/mod.rs` declares the module and `src/lib.rs` re-exports its public facade. No product logic or test body changed. The routed subsystem may import Kani under AD-004 L-2. Focused `cargo test --locked --test it layout::` passed all eight layout controls, including L-1 and L-2; `git diff --check` passed. The original finding rows and fixed dispositions remain unchanged. No aggregate gate or engine run is claimed by this reviewer.
