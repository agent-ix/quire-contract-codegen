---
id: SR-597003
title: "IR-348 PR #597 spec-review/integrity review (AD-004 internal consistency)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #597 head 5e5245233fb8ab99000ce81563a6d66fb9ea5160 against origin/main 83721c07; spec/assurance/AD-004-cg-crate-layout.md (whole document re-read for stale 4f prerequisites); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

## Summary

Ticket: IR-348. This integrity lens re-read every mention of 4f, `generate_kani_bundle` and the V1
bundle in AD-004 at head. It checked whether steps 4a-4f, L-5, L-12, the generator-fate table, the
interim-exceptions table, the D-2 and D-3 rationale and the risks agree after the PR's edits.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AD contradicts itself on what 4f leaves of the shared V1 clause lowering and the precondition and contract renderers. D-3 says 4c reshapes `clause.rs` and 4f trims it, and step 4c says the precondition and contract families take the V2 input. The fate table and the interim row say 4f removes V1 precondition and contract rendering, and 4c adds a separate V2 lowering. Two implementers would delete different files. | spec/assurance/AD-004-cg-crate-layout.md:717, 1070, 1379, 1453 |
| FND-002 | low | Stale `census_validation.rs` statements contradict the new 4f text. D-2 says 4f "does not touch" the file, and 4g says every item there is also used by `v1_bundle.rs`. The new 4f text removes the V1-only error variants and helpers that no longer have a caller. | spec/assurance/AD-004-cg-crate-layout.md:1105-1107, 1133, 1351 |

## Verdict

Changes requested, for FND-001. Everything else checked agrees with the new decision:

- Step 4 intro, 4a, 4c, 4e and 4f.
- L-5 (now an open obligation) and L-12 (end-state kept absolute).
- The One input model bullet, the migration-order preamble, the risks and the decision rows.

None of the old wording that made deletion wait on 4c, 4d or 4e, the QSL exemplar move or the
installed control survives as a gate. The D-2 note that the bundle is "still live" is accurate
history for a step that has already landed.

## Coverage

```yaml
scope:
- {id: AD-004:generator-fate, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The scalar renderer becomes a family lowerer. Step 4f removes V1 precondition and contract rendering; step 4c later supplies V2 family lowerers through render.rs."}
- {id: AD-004:4c, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "the precondition and contract families take the V2 input and render through HarnessSpec ... The V1 arm and bundle are already deleted by 4f"}
- {id: AD-004:D-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "so the shared V1 lowering is its own file that 4c reshapes and 4f trims."}
- {id: AD-004:interim-clause-row, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "step 4f deletes V1-only lowering and the V1 arm; step 4c adds the separate V2 lowering"}
- {id: AD-004:D-2, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "kani/generate/census_validation.rs, which 4f does not touch."}
- {id: AD-004:4g, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "every item there is also used by v1_bundle.rs (public KaniDiagnostic and KaniErrorCode included), so its items go once their last caller is gone, as 4f says."}
- {id: AD-004:4a, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "4a: confirm the regression requirement (L-5) on the supported V2 path after the QSL-owned exemplar migration."}
- {id: AD-004:4e, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The V2 route and its tests must pass before the QSL-owned follow-up moves its control."}
- {id: AD-004:L-5, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "L-5 is retained as an open obligation, not a deletion gate."}
- {id: AD-004:L-12, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Step 4f removes these V1 paths and their V1-only tests immediately; the L-12 end-state remains absolute."}
- {id: AD-004:risks, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "QSL's exemplars currently call generate_kani_bundle. Step 4f makes that legacy call unavailable."}
```

## New findings (disposition pass 1)

Re-checked at head a6b88727da8e4d904b120f87a3cf698f766840ae against the previous head 5e524523.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The AD now says IR-948 delivered V2 precondition and invariant slices that survive 4f. The code does not support invariant: `negotiate.rs` refuses a V2 invariant clause with `V2ClauseConditionNotRendered`, and `clause/v2.rs` renders only precondition and native postcondition harnesses. | spec/assurance/AD-004-cg-crate-layout.md:700, 718, 1069, 1671; src/kani/generate/negotiate.rs:658-664; src/kani/generate/clause/v2.rs:1, 133 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | D-3, the layout tree, the generator-fate table, 4c and the interim row now agree on one reading. Step 4f deletes the V1-only contents of `clause.rs` and the V1 `precondition.rs` and `contract.rs` renderers. V2 lowering is separate and renders through `HarnessSpec`, as `clause/v2.rs` already does through `render`. |
| FND-002 | fixed | D-2 and 4g now say that 4f deletes the bundle-only helpers and error variants, and that the remaining `census_validation.rs` items stay only while a corpus caller is live. |
| FND-003 | fixed | Round 2, head 14e5808e: AD lines 700, 718, 1069, 1124 and 1671 now say IR-948 delivered V2 precondition and postcondition slices. They state that V2 invariant still refuses and stays open under IR-266, which exists in Linear (Backlog). This matches negotiate.rs:658-664 and clause/v2.rs. No "precondition and invariant" or "invariant slice" wording remains. |
