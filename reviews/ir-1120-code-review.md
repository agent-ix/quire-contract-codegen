---
id: SR-1121011
title: IR-1120 PR 630 code review
type: SpecReview
scope: "agent-ix/quire-contract-codegen; src/routed/smt/mod.rs, src/routed/smt/refinement.rs; FR-045-AC-4 through FR-045-AC-7"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: reviews }
---

# SR-1121011: IR-1120 code review

## Summary

Ticket: IR-1120. PR: quire-contract-codegen#630 at 2e84d64c. The Rust review lane is folded into this code review. I inspected only the PR diff, its existing solver execution and bounded process types, FR-045-AC-4..7 and TC-059. No Cargo or CI files changed. No applicable AssuranceProfile was found. Builds and tests were deferred because #627 holds the heavy-build lock.

## Verdict

**FAIL** at the reviewed head. A caller can supply proof bytes unrelated to the solver's captured output and obtain a proof-bearing handoff. Shape validation also allocates from a caller-supplied step count before checking the available records.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `RefinementRun::from_execution` accepts caller-provided `proof_bytes` independently of the completed process's captured output, and `refinement_handoff` serializes them as Alethe proof bytes. A completed `unsat` run with stdout `unsat\n(real)` and supplied proof `(fake)` yields a proof-bearing handoff with `(fake)`. The test uses `/usr/bin/cat` and embeds its proof in the submitted query before extracting it from stdout, so it cannot establish proof provenance. FR-045-AC-5 requires the exact proof emitted by that invocation. | src/routed/smt/refinement.rs:60-79; src/routed/smt/refinement.rs:139-179; src/routed/smt/refinement.rs:216-253 |
| FND-002 | medium | `refinement_handoff` constructs two `step_count + 1` vectors before checking `expected.len()` and `records.len()`. A caller can pass a very large step count with empty slices and trigger allocation failure or process abort instead of the typed `Shape` refusal; validate cardinality before allocating. | src/routed/smt/refinement.rs:108-116 |

## Coverage

Examined FR-045-AC-4, AC-5, AC-6 and AC-7. The role enum, ordering, duplicate/out-of-range rejection, record equality checks and typed non-Unsat refusal are present. The new public items are documented. The changed source adds no vendored file or duplicated upstream artifact. The only new trace tags are on the two unit tests in `refinement.rs`; see SR-1121012 for coverage gaps. Focused format, build, lint and test outcomes are not reviewer-verified because the shared heavy-build lock is held by #627.

## Examined scope

```yaml
scope:
  - {id: FR-045-AC-4, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "For a refinement with seven step rows, CG retains one `Base` and exactly seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically ordered `steps` array, even when records arrive out of order or the source rows were written in another order."}
  - {id: FR-045-AC-5, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Given seven completed `Unsat` step records and one completed `Unsat` base record with present Alethe proofs, CG emits one deterministic handoff sequence in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input permutation. Every entry has separate length-delimited query and proof byte fields equal to the original FR-315 query and emitted Alethe proof, including a present empty proof."}
  - {id: FR-045-AC-6, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Starting from AC-5's collection, removing `Base`, removing row 3, duplicating `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate handoff."}
  - {id: FR-045-AC-7, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Starting from AC-5's collection, making row 3's query or proof field absent yields a typed missing-field refusal and no handoff; making the proof present but empty preserves the empty bytes in the handoff."}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 3141a622, `from_execution` no longer accepts proof bytes and public `refinement_handoff` always returns `ProofProvenanceUnavailable` after validation. Unverified bytes cannot reach a public proof-bearing handoff. AC-5's positive path remains open in SR-1121012. |
| FND-002 | fixed | At 3141a622, cardinality is checked before allocating `ordered` and `selected`; a huge step count with short slices returns `Shape`. |
