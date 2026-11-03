---
id: SR-202611020189
title: IR-1184 independent Rust code review
type: SpecReview
analysis: base
scope: 'agent-ix/quire-contract-codegen PR #702 at e09c787a0a86424f39bad9fc8e215d56951c228f;
  FR-032-AC-15..18 and FR-033-AC-18'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-032
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-033
  type: references
---

## Summary

Independent diff review of original scalar positional-body retention, same-artifact/context checks, full actual-sent QSL report binding, and native-only composite artifact authentication. Prior review FND-002..004 fixes remain present on the updated head. No new code or test finding was identified. The delta does not claim backend proof, pair counts, or full FR-033 native settlement.

## Verdict

PASS for the reviewed IR-1184 delta.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust/code review examined changed production and test files, callers, wire and persistence checks, generated artifact paths, and the prior findings. No CI workflow or dependency file changed. Final gate belongs to the code lead.

```yaml
clean: true
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
findings: []
bindings:
- test_id: original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-15
  trace: correct
- test_id: original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-16
  trace: correct
- test_id: original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-17
  trace: correct
- test_id: scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-17
  trace: correct
- test_id: original_scalar_driver_executes_independent_repeated_draws_and_binds_real_qsl
  ac_id: FR-032-AC-18
  trace: correct
- test_id: scalar_actual_sent_context_and_every_driver_tie_are_independently_checked
  ac_id: FR-032-AC-18
  trace: correct
- test_id: original_composite_body_and_actual_emitted_bytes_have_independent_authority
  ac_id: FR-033-AC-18
  trace: correct
```
