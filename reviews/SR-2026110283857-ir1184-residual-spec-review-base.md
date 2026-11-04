---
id: SR-2026110283857
title: "IR-1184 residual spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1184; src/replay/scalar_builder.rs; tests/it/scalar_replay_residuals.rs; tests/it/composite_record_residuals.rs; shared native test helpers; spec/replay/functional/FR-032; spec/replay/functional/FR-033; spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-1184. One independent residual review of the native authority slice. Acceptance requirements remain intact and native/proof distinctions are explicit in the additions. One medium status contradiction remains in the same FR: typed scalar-kind retention is still described as absent despite the corrected introduction and real admitted operator route.

## Verdict

**CONDITIONAL** — Acceptance requirements remain intact and native/proof distinctions are explicit in the additions. One medium status contradiction remains in the same FR: typed scalar-kind retention is still described as absent despite the corrected introduction and real admitted operator route.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The status correction still says CG has no typed scalar-kind source and the route refuses, contradicting the retained implementation and the new passing native operator tests. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:207 |

## Coverage

No applicable AssuranceProfile was found in spec/. Source-only review: no Cargo, Kani, CBMC or proof job was run. Existing external qualification logs were inspected: scalar residuals 3 PASS, composite residuals 1 PASS, canonical subtraction red before the fix, focused Clippy and spec validation completed. The full final integration gate remains the lead’s next step. No acceptance wording was changed; same-proved-artifact and shadow duties remain Planned under IR-631 and IR-806. New TC-047/TC-048 tags are partial suite evidence and do not discharge whole ACs.

Plan completion: not assessed

Other spec lenses were not selected: no acceptance statement, domain object, relationship edge, architecture boundary, or structural requirement was changed. Evidence was selected for the changed native-status claims; base includes consistency and link inspection. Optional standalone semantic gap fan-out was not run; intent/code/test agreement was examined in the required code review. No new agent was spawned.

Examined scope:

```yaml
scope:
- id: FR-032-AC-15
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. The admitted IR reader supplies the original application
    structural identity and authentic expression occurrence. CG typed generation supplies
    and retains the scalar kind and one argument for each operand position in actual
    draw order, with typed graph-child or inline-literal provenance and the harness
    range. Two draws of one parameter remain two entries. Changing a package, path
    or producer ordinal changes the structural identity; changing an included body
    member while the identity stays equal is caught by complete retained-body comparison
    and refuses. A source-span or n
- id: FR-032-AC-16
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. An independent fixture obtains the same application
    `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact
    structural identity and complete positional body, and the actual QSL operator
    route accepts the matching claim. No CG-local identity derivation, copied encoder,
    stand-in verifier or synthetic acceptance passes.
- id: FR-032-AC-17
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. With a valid original claim retained, altering only
    the claimed package, path or producer ordinal, or changing one otherwise membership-valid
    retained body member while reusing the old structural identity, produces a typed
    stale-dependency/revision-mismatch refusal before scalar comparison. A body-only
    edit cannot pass because the node identity is equal. No claimed or recomputed
    digest is required.
- id: FR-032-AC-18
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. If the admitted IR read lacks a valid application
    identity or authentic occurrence, or CG typed generation lacks a required retained
    body member, the CG builder returns typed setup refusal without replay or fabricated
    identity. A caller string or usable sentinel cannot replace the absent authority.
- id: FR-033-AC-18
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED / CODE-GATED. CG obtains the original equality/inequality application
    structural identity from IR admission and retains the authentic occurrence, kind
    and one argument per operand position in original order. A self-comparison keeps
    two graph-child entries with each drawn Node-keyed bound; distinct parameters
    and composite literals keep their own graph-child entries, with empty Bounds for
    literals. Population-keyed metadata stays in the full proving record; an admitted
    QSL claim containing it refuses as HarnessUnknownKey before identity/body comparison.
    Bounds key ordering and duplicate
- id: FR-032
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: Current CG has no typed scalar-kind source. Until generation implements
    and retains this allocation, the route returns the named unavailable-capability
    refusal without minting an identity.
```
