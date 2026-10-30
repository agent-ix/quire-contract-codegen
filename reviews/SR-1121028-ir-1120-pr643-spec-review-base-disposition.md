---
id: SR-1121028
title: "IR-1120 CG #643 spec-review/base disposition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #643 head f8b6b0573c71cb0476a67253c00c1cb3a34005e6; spec-review/base disposition of SR-1121024 and the spec fixes since 3f40b53ab057c52d72fdf49ca29faf2bcef89572"
review_set: subset
---

## Summary

Disposition of SR-1121024 at head f8b6b0573c71cb0476a67253c00c1cb3a34005e6. All substantive findings are fixed. FR-045 Status and the matrix now claim only what the tests show, and they list every residual the lead named. One low new item: two review artifacts share IDs with other artifacts.

## Verdict

PASS on spec honesty and on criteria wording, subject to `make spec` accepting the duplicate IDs in FND-001. AC5 is PARTIAL, and nothing in the spec claims Verified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two artifacts reuse existing review IDs: `reviews/ir-1120-code-review.md` has SR-1121011, as does `reviews/SR-1121011-ir1124-pr631-spec-review-ears-conformance.md`, and `reviews/ir-1120-gap-analysis.md` has SR-1121012, as does `reviews/SR-1121012-ir1124-pr631-spec-review-integrity.md`. Neither `ir-1120-*` file is in the local main checkout, so they appear to come from this PR. No spec validation was run, so it is unknown whether the validator refuses this. | reviews/ir-1120-code-review.md:2; reviews/ir-1120-gap-analysis.md:2 |

## Coverage

Original finding outcomes at this head:

| Original | Outcome | Evidence |
| --- | --- | --- |
| SR-1121024 FND-001 | accepted-no-change | Placeholder, carried. |
| SR-1121024 FND-002 | fixed | spec/routed/matrix/tests.md:34 says "focused controls with synthetic proof output" rather than "covered". tests.md:66 and FR-045 Status (lines 209-217) list the production driver, zero-step and step 8 controls. |
| SR-1121024 FND-003 | fixed | FR-045:125 now reads "complete parenthesized SMT-LIB commands", which matches `valid_proof_commands` (src/routed/smt/mod.rs:289-312). This is Behavior prose, not a criterion, and it records stricter refusal that the code already applied. |
| SR-1121024 FND-004 | fixed | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:18-21 says partial and spec/spec.md:96 says partial. |
| SR-1121024 FND-005 | fixed | FR-045:214 names IR-1139. Linear was not read, so the ticket's own state is unverified. |

Status checklist against the required residuals:

| Residual | Stated at |
| --- | --- |
| Production FR-315 driver absent | FR-045:212; tests.md:34; tests.md:66; TC-059:20 |
| Recompiled zero-step control | FR-045:212-213; tests.md:34; tests.md:66; TC-059:20 |
| Step 8 refusal and non-success controls | FR-045:213-214; tests.md:34; tests.md:66; TC-059:20 |
| AC5 live cvc5/QSL reader blocker IR-1139 | FR-045:214-217; tests.md:34; tests.md:66 |

TC-059:21 says only "live proof readback" without the ticket; the other locations name it, so this is not a finding.

No narrowing: AC4 and AC5 matched the local main checkout by pattern and AC6-7 are identical. TC-059 steps 6 and 8 gained real-cvc5 and proof-mode requirements at this head and no step lost any. The Status keeps the bar that CG must pass the exact interior bytes through the reader before claiming AC-5 or TC-059 step 6 covered. No row or status text claims Verified.

This reviewer ran no `make spec` or Quire validation; the lead will run that on this artifact. The `/spec-review` skill was not loadable here, so the base method was applied from the brief. Only the base analysis was performed. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
- id: FR-045-AC-5
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'Given seven completed `Unsat` step records and one completed `Unsat` base
    record with present Alethe proofs, CG emits one deterministic handoff sequence
    in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input
    permutation. Every entry has separate length-delimited query and proof byte fields
    equal to the original FR-315 query and emitted Alethe proof, including a present
    empty proof. Repeating serialization of the same typed fields gives identical
    bytes; changing a proof byte changes only that entry''s proof field. The QSL adapter
    can construct `RefinementSmtCertificate` wi'
- id: FR-045-AC-7
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, making row 3's query or proof field absent
    yields a typed missing-field refusal and no handoff; making the proof present
    but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat`
    or `Unknown` answer, a missing or malformed response, a failed solver invocation
    or a reached limit remains typed non-proof evidence and produces no proof-bearing
    handoff.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | still-open: the lead's `make spec` decides; renumber the two `ir-1120-*` artifacts if it refuses them | Static read of the `id:` lines in `reviews/`. |
