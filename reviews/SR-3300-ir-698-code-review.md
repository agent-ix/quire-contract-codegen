---
id: SR-3300
title: "Code review of IR-698 ProofBound adaptation"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #329; Cargo.lock, src/replay/state_clause.rs, tests/it/composite_parity_converter.rs, tests/it/kani_obligations_state_clause_replay.rs, tests/it/skeleton_spine.rs, spec/assurance/AD-002-cg-qsl-replay-seam.md"
review_set: subset
---

## Summary

IR-698 code and Rust review of PR #329. The QSL constructor and accessors are used with the correct integer and collection kinds; no code or test defect was found. One adjacent architecture document still describes the old shape and omits the new typed refusal.

## Verdict

**CONDITIONAL** — update the CG–QSL seam description to reflect the kind-bearing constructor and its typed failure before merge. The production/test adaptation otherwise reads correctly.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-002 still describes `ProofBound` with only domain and bound and omits the new `ProofBound` construction refusal from the CG-side failure boundary | spec/assurance/AD-002-cg-qsl-replay-seam.md:59,101; src/replay/state_clause.rs:225,938 |

## Coverage

- Examined source and tests: all four changed Rust paths, the five-path PR diff, and its coherent QSL/IR dependency resolution.
- Examined contracts: QSL `ProofBound::new`; CG FR-024-AC-11, AC-17, AC-18 and FR-033-AC-7, AC-12 as context; AD-002 seam description.
- Rust idioms: no new panic on the production path, no unsafe or lint suppression, and constructor refusal is propagated as a typed error and classified as a CG defect. Test `expect` calls guard fixed fixture invariants.
- Gate evidence: the author reported pre-PR `make ci` and real Kani runs; this reviewer did not repeat full gates.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-002 now states the constructor's required kind pairing and the typed construction refusal. |

The fix replaces AD-002's two-field description with `DomainKey`, `Option<DomainKind>` and `FiniteBound`, names `ProofBound::new`, specifies the integer and collection pairings, and records `StateClauseReplayError::ProofBound(ProofBoundRefusal)` as a pre-replay CG defect mapped to `Failed`. The original finding above is preserved.
