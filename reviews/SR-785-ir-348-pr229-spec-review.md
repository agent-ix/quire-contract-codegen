---
id: "SR-785"
title: "CG PR 229 spec review: AD-004 step 2f item map"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@8280a5667498f2edae563a98f127eaec01ed98d3; spec/assurance/AD-004-cg-crate-layout.md (target tree, module map, dependency direction, L-3, L-6, L-12, migration steps 2, 4f and 5, Step 2f item map, Risks, Not verified), checked against src/ at origin/main 2130cd9"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-785: CG PR 229 spec review

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#229 at 8280a56, base origin/main 2130cd9.
The PR changes only the spec: it edits `spec/assurance/AD-004-cg-crate-layout.md` (+498/-45).
This review covers integrity, consistency between sections, ids, and consistency with the code.
No requirement statement changed, so EARS has nothing to review.

- Ids. No requirement id is added, removed or renamed. The L-labels are unchanged, and no
  criterion is rewritten. No SHA or commit pin was added. The only `file:line` cite on an added
  line, IR `src/kani/mod.rs:19`, was already in the AD.
- `make spec` at head prints the same 8 module-loader warnings as the baseline, with no errors.
- The updated sections agree with the new item map: the tree, the module map, the `kani/` order
  rule, the run table, L-3, L-6, steps 2, 4f and 5, the Risks and Not verified. Stale text from
  the first draft is gone: the PR 210 "parked" note, the `kani_transcript.rs:261` cite, and the
  claim that the test back-edge goes to `tests/it/`.
- Planner guardrail (pure placement). Each item in `outcome.rs`, `clause.rs` and `record.rs`
  exists in `src/kani_obligations.rs` today. The AD moves them and adds nothing: no new type,
  trait, function, re-export shim or merged helper, and `generate/mod.rs` holds declarations
  only. `spec.rs`, `render.rs` and `terminal.rs` are explicitly not created. Template text stays
  in place. Two of the new files are needed to keep the imports acyclic:
  - `outcome.rs`: its vocabulary is named by every family and by `negotiate`.
  - `clause.rs`: it is shared by `negotiate`, `precondition` and `contract`. If it sat in
    `negotiate`, the import cycle would be `negotiate` to `precondition` to `negotiate`.

  `record.rs` is not needed for acyclicity. `scalar.rs` could hold it, with `negotiate` importing
  it downward. It is still pure placement of existing items, so it passes the planner's test.
- What remains: four statements that contradict the code or another part of the AD (FND-001 to
  FND-004), and one forward claim about step 4b (FND-005).
- IR-501 (no step owns the `oracle/scalar/` split) is not covered here. The tree still says
  "split along derivation, lowering and rendering". It belongs to the oracle subsystem and has
  nothing to do with the 2f `kani/` move, so it should stay a separate small AD PR.

## Verdict

Consistent, but not mergeable on the spec-review side until FND-001 and FND-002 are fixed. Both
are one-line edits. The guardrail holds.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | L-12 still says, with no condition, that `BoundPackage` and `BoundClause` are named nowhere in `kani/`. After this PR's 2f placement they are named in `kani/generate/outcome.rs` (the `ObligationItem::BoundClause` variant holds `&BoundPackage`), in `negotiate.rs` (`classify_clause`, `named_oracle_source`, `reject_duplicates_and_mixtures`, and the `render_probe_*` test fixture, whose row says "which `kani/` may name until 4f") and in `clause.rs` (`LoweredClause`, `lower_clause`). The dependency-rules bullet allows "the V1 arm that step 4f deletes", so the AD contradicts itself, and a layout test for L-12 would fail on the 2f head. L-12 needs the interim form L-3 and L-6 have: "after step 4f", or the files listed. The interim-exceptions row should name `outcome.rs` too, not only `clause.rs` and `classify_clause`. | spec/assurance/AD-004-cg-crate-layout.md:638-639, :364-366, :1015, :1065 |
| FND-002 | medium | The order inside `kani/generate/` lists `census_validation` after `corpus` and `v1_bundle`, yet the same bullet says `corpus` and `v1_bundle` both import `census_validation`. The rule just above (a file imports only earlier names in that order) then forbids both edges. The bullet also says `negotiate` "imports every family", but in the code it imports none of `frame`, `lower`, `corpus`, `v1_bundle` or `census_validation`. Move `census_validation` before `corpus` and `v1_bundle` (with `lower` before `corpus`), and say that `negotiate` imports `outcome`, `record`, `scalar`, `clause`, `precondition` and `contract`. | spec/assurance/AD-004-cg-crate-layout.md:335-341 |
| FND-003 | low | Step 4f says 4f "edits nothing in `census_validation.rs`" and, in the same parenthesis, that the V1-only `KaniErrorCode` variants it leaves unused "are 4f's to prune". That is an edit to `census_validation.rs`. Say either "prunes the V1-only variants" or "leaves them for 4g". | spec/assurance/AD-004-cg-crate-layout.md:745-747 |
| FND-004 | low | D-3 justifies `outcome.rs` by "`Outcome` holds the lowered form of every family". In the code, `Outcome` holds `Lowered(Box<LoweredClause>)` and `LoweredScalar(Box<LoweredScalarClaim>)` only. Precondition and contract share the clause form, and frame has none. The placement is right; the reason should name the two lowered forms. `Outcome` itself goes to `negotiate.rs`, not `outcome.rs`, and the sentence reads as if it did not. | spec/assurance/AD-004-cg-crate-layout.md:933-936 |
| FND-005 | low | Three places decide part of step 4b's design ahead of time: "`record.rs` ... absorbed by step 4b", "Step 4b absorbs them into `HarnessSpec`", and "step 4b (into `HarnessSpec`)". The planner's guardrail holds the `HarnessSpec` shape behind QSL-370. Separately, the tree line "each returns a HarnessSpec from step 4b on" is wrong for `precondition`, `contract` and `frame`, which are ported at 4c and 4d. Say "step 4b decides their fate" and "from the step that ports the family (4b, 4c, 4d)". | spec/assurance/AD-004-cg-crate-layout.md:207-213, :954, :1016 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a91ed1b (round 1, reviewed at a91ed1b: L-12 now carries an "until step 4f" interim list naming outcome.rs, negotiate.rs with its test fixture, and clause.rs; the interim-exceptions row names outcome.rs and negotiate.rs; the code names BoundPackage/BoundClause in no other kani file) |
| FND-002 | fixed | a91ed1b (round 1: the order is outcome, record, census_validation, then the families; negotiate's imports are listed exactly) |
| FND-003 | fixed | a91ed1b (round 1: the V1-only KaniErrorCode variants now "stay there until 4g or the V2 census input removes the file", which agrees with "edits nothing") |
| FND-004 | fixed | a91ed1b (round 1: D-3 names Lowered and LoweredScalar, and says Outcome itself is in negotiate.rs) |
| FND-005 | fixed | a91ed1b (round 1: the record.rs "absorbed by 4b / into HarnessSpec" claims are gone, replaced by "later steps may move them" and "not scheduled"; the tree line follows 4b/4c/4d; the template destination is left to its owning steps; the remaining 4b mentions only restate the existing schedule for spec.rs/render.rs) |
