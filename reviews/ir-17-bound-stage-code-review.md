---
id: SR-14700
title: IR-17 bound-stage Spec fidelity and scoped code alignment
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-17 PR489; spec/oracle/functional/FR-018-composite-equality-oracles.md,
  spec/oracle/matrix/TC-029-composite-equality-oracles.md
review_set: subset
---

## Summary

The candidate is docs-only at c06a2a25b58c9e625ba1f1bd5f45dde2fe8d8c37 against 8071696e8e0c154184818131fa1544541c1baf7e. Ticket IR-17; PR489. Both changed documents pass installed Quire validation. No native, Cargo, formatter or backend job ran; no implementation or proof acceptance is claimed.

## Scope

```yaml
scope:
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes and a duplicate
    record field; and each generated oracle calls `TypeEnvironment::check_type` on
    both operand comparison types before `check_equality`, so calling it with an empty
    environment, one omitting a key its operand types reach, or one declaring that
    closure under a different key, yields `Outcome::Refused(Refusal::CheckedInvariant)`
    with no charge admitted, rather than a panic or a completed Boolean.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Admitted `bounded_domain` nodes an operand type reaches retain the exact
    named bound values FR-014 specifies. For `integer_range` and `collection_bounds`,
    Contract IR FR-038 admission requires exactly `min` then `max` bindings with their
    canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted
    binding control is an IR-intake refusal before CG, with no admitted package or
    public CG `UnreadableBound` result. CG private parser tests retain their precise
    malformed-bound defenses separately and do not count as public-generator acceptance
    for these '
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: reader-ineligible inputs. For `rational_range`, `decimal_range` and `text_bounds`,
    binding lookup remains by name in any admitted order, and genuinely reader-admitted
    bare/missing/duplicate/unlisted-name controls retain the public unreadable-bound
    refusal; this criterion makes no new admission claim for those mutations. Existing
    valid-bound reconstruction, values/domains and missing-bound behavior remain unchanged.
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`,
    with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`;
    removing or renaming `optional`, adding a second nested member, or targeting a
    non-option node refuses only that item as malformed, never as a required field
    or a generated oracle. PLANNED (IR-643).
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Emit only `check_equality`, omitting the `check_type` calls; `check_equality`
    consults the environment solely through `contains_ieee`, which returns `false`
    for an absent composite key, so an empty environment completes with a Boolean
    instead of refusing. Or emit `.expect(..)`, which panics instead.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Treat reordered or malformed integer/collection bounds as reader-admitted
    generator inputs, count private parser defense as their public acceptance, or
    read an admitted bound under a wrong name/value so it becomes another range.
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Expect `next`'s value to be a direct reference and reject QSpec's nested
    optional binding, or ignore the wrapper and treat the field as required; the List
    item refuses or generates with the wrong presence.
- id: TC-029-step-8-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "   For FR-018-AC-14, separate bound reconstruction from intake and defensive\n\
    \   controls. First read valid `integer_range` and `collection_bounds` nodes\n\
    \   whose bodies contain exactly `min` then `max`, each a canonical integer\n\
    \   literal typed at the owning Integer node. Through normal lowering and\n  \
    \ public generation, verify the exact original bound values and domains;\n   do\
    \ not widen, reorder or substitute them. Keep the existing missing-bound\n   request/root-selection\
    \ controls unchanged."
- id: TC-029-step-8-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "   At Contract IR intake, independently reorder these bindings, replace\
    \ one\n   with a bare literal, remove one, repeat one, and introduce an unlisted\
    \ name.\n   Use an ungrouped bound node in otherwise valid wire. Refresh the\n\
    \   package-level canonical identity and matching identity-projection mirrors\n\
    \   while retaining the offending node key, so earlier identity checks do not\n\
    \   mask the derived closed-shape refusal. Do not ask a candidate-key helper\n\
    \   to manufacture a key for an invalid closed shape.\n   Require its `invalid_package`\
    \ / `stale-node-key` refusal at the offending\n   "
- id: TC-029-step-8-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "node's `node_id`, with no admitted package. These rejected inputs never\n\
    \   call public CG generation and have no CG `UnreadableBound` disposition.\n\
    \   Do not attach an unrelated genuine context to claim a later refusal."
- id: TC-029-step-8-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "   Retain private CG bound-parser/type-resolution tests with the same\
    \ precise\n   malformed-bound assertions as defense against inconsistent internal\
    \ input;\n   label them separately from the public valid-bound and IR-intake controls.\n\
    \   A private defense does not supply public-generator acceptance credit for\n\
    \   an input the reader rejects. This stage split applies only to\n   `integer_range`\
    \ and `collection_bounds`. For `rational_range`,\n   `decimal_range` and `text_bounds`,\
    \ preserve named lookup in any admitted\n   order and each genuinely reader-admitted\
    \ unreadable-bound negative; "
- id: TC-029-step-8-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "do\n   not infer their admission or reader refusal from the two closed\
    \ forms.\n"
- id: TC-029-step-16-bound-stage
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "    The `collection_bounds` malformed-name/order/shape controls at this\
    \ private\n    seam are defensive parser evidence, not public-generator acceptance;\n\
    \    step 8 owns their separate IR-intake refusal and valid-bound controls.\n\
    \    This clarification does not reclassify the record optional-wrapper or\n \
    \   cycle controls.\n"
- id: TC-029-planned-status
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: context_only
  excerpt: 'The FR-018 admitted-context and direct-owner procedure amendments are

    PLANNED/UNRUN until CG implements them and qualifies the generated ABI.

    The existing full record/tuple/recursive/collection corpus remains required;

    the enum-only owner facade does not excuse dropping it.

    '
- id: IR-FR-038-closed-shape
  path: /home/peter/.cargo/git/checkouts/quire-contract-ir-c717a33c309ad29d/aa01d0d/crates/quire-contract-model/src/checked_package/v2/derived_keys.rs
  role: context_only
  excerpt: "bound(lower, \"min\", &typed, over).is_some()\n                && bound(upper,\
    \ \"max\", &typed, over).is_some()"
- id: FR-322-AC-48
  path: /home/peter/dev/quire-specification/spec/objects/interfaces/FR-322-checked-package-artifact.md
  role: context_only
  excerpt: 'A published package whose ungrouped structural node is tampered, with
    its `node_id` kept, its `identity_projection` entry patched the same way and its
    `package_id` recomputed, is schema-valid and identity-consistent and refuses `invalid_package`/`stale-node-key`
    at that node''s `node_id`: an `integer_range` `max` narrowed to `2` and widened
    to `5000`, its `min` raised to `1` and its bounds swapped, a `reference` target
    re-pointed at the `Integer` node, a `set` element re-pointed at the `Integer`
    node, and the declared record `Example::Point`''s field `x` re-pointed from the
    `Integer` '
- id: FR-322-AC-48
  path: /home/peter/dev/quire-specification/spec/objects/interfaces/FR-322-checked-package-artifact.md
  role: context_only
  excerpt: node at the `Int[0, 9]` node. The untampered package is admitted.
- id: FR-322-AC-49
  path: /home/peter/dev/quire-specification/spec/objects/interfaces/FR-322-checked-package-artifact.md
  role: context_only
  excerpt: 'The recursive-records package (`record List { next?: List; }` and `record
    Tree { kids: Sequence<Tree>[0, 3]; }`, each one recursion group keyed by QSL FR-092)
    is admitted. With one in-group node tampered, its `node_id` kept, its `identity_projection`
    entry patched and its `package_id` recomputed, it refuses `invalid_package`/`stale-node-key`
    at that node''s `node_id`: `Tree`''s `collection_bounds` `max` widened from `3`
    to `1000` and its `min` raised to `1`, `Tree`''s `Sequence` element re-pointed
    from the record at the `collection_bounds` node, and `List`''s `Option` re-pointed
    at the '
- id: FR-322-AC-49
  path: /home/peter/dev/quire-specification/spec/objects/interfaces/FR-322-checked-package-artifact.md
  role: context_only
  excerpt: '`Integer` node. The `Tree` group relabelled with a label that is not its
    group digest, each member rekeyed under that label and every reference rewritten,
    refuses `invalid_package`/`stale-node-key` at the group member with the lowest
    node-id digest.'
- id: FR-014-bound-members
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: '| `integer_range` | `min`, `max` |'
```

## Analysis

The owner reader was independently inspected: derived_keys.rs Shape::of excludes rational/decimal/text from this closed-shape rule; closed_shape requires integer/collection min then max; valid_derived_shape precedes structural key comparison and emits InvalidPackage/StaleNodeKey at node_id; FR-330 contract_package.rs invokes that validator after structural/application validation. The controlled procedure retains other valid wire, refreshes the canonical package identity and projection mirrors, and retains the offending node key. It therefore targets derived shape rather than an earlier package-identity mismatch. The amendment retains valid bound values/domains, missing-bound behavior, private precise defenses, record AC8/26 and the merged genuine-context ABI. Consumer acceptance consumes an external precondition by reference; it does not create a competing producer definition. No code is changed. Existing private named-binding tests remain private evidence, not newly accepted public controls.

## Verdict

**PASS** — No defect found in the changed stage-allocation contract. Qualification remains outstanding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
