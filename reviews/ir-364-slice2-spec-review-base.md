---
id: SR-11770
title: IR-364 slice 2 spec review of the V2 strategy chain criteria
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen@f30f17b1a086211cd0ab87f2445eb5565fac74b5 (PR 400, diff vs origin/main 007417d); spec/strategy/functional/FR-008..FR-012, spec/strategy/non-functional/NFR-004, spec/strategy/matrix/TC-017..TC-021 and tests.md, spec/assurance/AD-001, spec/assurance/AD-004
review_set: subset
---

## Summary

Ticket: IR-364 (slice 2). The PR rewrites FR-009 to FR-012, NFR-004 and TC-018 to TC-021 from
`BoundPackage` to a selected claim of an admitted `CheckedPackageV2`. It adds the planned criteria
FR-008-AC-7, AC-8 and AC-9 and FR-009-AC-7, adds TC-017 steps 6 and 7, and fixes cross-references
and stale status text in AD-001 and AD-004.

The review checked these points:

- Each rewritten AC against its main text.
- The criteria against IR main 32e433a. The checked file was
  `crates/quire-contract-model/src/checked_package/v2/comparison_operands.rs`: the
  `CheckedComparisonOperator` Eq|Ne|Lt|Le|Gt|Ge, `CheckedComparisonClauseKind`, `CheckedComparisonRead`,
  `CheckedComparisonObservation`, `CheckedComparisonOperand.range` (i128) and `CheckedComparisonError`.
  The IR spec text checked was FR-038-AC-202..209, all marked Implemented (IR-703).
- The current CG oracle code: `src/oracle/scalar`, `src/oracle/operator_catalog.rs`,
  `src/oracle/bound_v1.rs` and `src/oracle/boolean_v1.rs`.
- FR-014-AC-35.
- `quire validate`, which is clean on the changed files.
- `quire coverage --strict`, which moved from 214 to 218 unbacked rows with 0 contradicted. The 4
  new rows are FR-008-AC-7, AC-8 and AC-9 and FR-009-AC-7.

## Verdict

**NOT MERGEABLE as written: one high finding and two medium findings.**

These checks passed:

- No AC is renumbered, and no AC is dropped.
- FR-009-AC-1 and AC-6, FR-010-AC-2 and AC-3, FR-011-AC-1 and AC-2, FR-012-AC-1 and AC-3, and
  NFR-004-AC-1 keep their obligations. They are retargeted to V2 fixtures, and some are tightened to
  the QSL-produced control.
- V1 coverage is labelled as historical evidence in tests.md. It is not counted as V2 backing.
- No `BoundPackage` or `ClauseRef` fallback or compatibility wording was added. The new Inputs bullets
  exclude both outright.
- The six-operator claim matches IR's `CheckedComparisonOperator`. Nothing confuses it with IR-803's
  `CheckedCompositeOperator` (Eq|Ne, `composite_operands.rs`), and nothing in this spec references
  the composite operator.
- The FR-008-AC-9 field names (operator, ordinals, provenance, observation, bounds, refusal) all exist
  on the accessor.
- The FR-011 clause-kind source matches `CheckedClauseComparison.kind`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-008-AC-8 forbids taking the integer eq/ne comparison oracle from `generate_exact_scalar_oracles`, "which have no integer equality or inequality descriptor". That reason is wrong in two ways. First, FR-014-AC-35 (planned, and being reworded in CG #397) requires `generate_exact_scalar_oracles` to derive integer `eq`/`ne` descriptors, so the reason is a statement about today's code that FR-014 is scheduled to falsify. AC-8 then binds CG to a second integer eq/ne oracle path beside FR-014's, which conflicts with the one-evaluator direction. Second, the V1 bound oracle does evaluate integer Equal/NotEqual: `generate_bound_oracles` calls `generate_named_boolean_oracle`, and `boolean_v1.rs:1279-1287` renders `rt::EqualityOperator::{Equal,NotEqual}`. The V1 exclusion is right, because that oracle reads `BoundPackage`, but the stated reason is false. Restate AC-8 by input and ownership rather than by a descriptor gap. For example: the oracle takes only the accessor's typed result, never a V1 bound oracle. Also decide, without assuming the answer, whether the strategy oracle reuses FR-014-AC-35's eq/ne path once it exists. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:122, spec/oracle/functional/FR-014-exact-scalar-oracles.md:455, src/oracle/boolean_v1.rs:1279-1287 |
| FND-002 | medium | FR-009-AC-2 changed "For every comparison operator" to "For every comparison operator of the V2 selected claim". A selected claim has exactly one operator, so the new wording can be read as that claim's single operator. That is narrower than the six-operator sweep that the main text and TC-018 step 3 ("each of the six operators") require. FR-012-AC-1 uses the clear form, "every V2 comparison operator". Use the same form here, or name integer.eq/ne/lt/le/gt/ge. | spec/strategy/functional/FR-009-constructive-correlated-populations.md:72 |
| FND-003 | medium | FR-011 lost its V1 source for the `construct_verdict` execution point (`quire_contract_ir::ExecutionPoint` serialized name) and now says "the runtime execution point (`initialization`, `handler`, `pre`, or `post`) of the selected checked clause" with no source. IR's V2 accessor returns only `CheckedComparisonClauseKind` {Invariant, Precondition, Postcondition}, and `checked_package/v2` has no execution-point type on IR main. Three clause kinds cannot pick one of four runtime points, and an invariant could be `initialization` or `handler`. The "requirement and revision identity" passed to `construct_verdict` came from V1 `ClauseRef`, and has no stated V2 source either. Name the V2 source of both, or record them as an open question with an owner. As written, two implementers would emit different verdict identities. | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:69-72 |

## Examined units

```yaml
scope:
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. For each of integer.eq, ne, lt, le, gt and ge, the selected claim's V2 comparison oracle returns, for every valuation of a 1- to 4-member domain (read-versus-read and read-versus-literal, the literal at each member and in either authored position), the result of an independently evaluated ordered relation."
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. The V2 comparison oracle for integer.eq and for integer.ne is produced by the V2 comparison-oracle path and not by `generate_exact_scalar_oracles` or the V1 bound oracle, which have no integer equality or inequality descriptor."
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: "PLANNED. A CG integration test calls IR's public clause-context accessor through its public API only, and the generator's census, ordered relation and diagnostics equal the accessor's operator, ordinals, provenance, observation, bounds and refusal fields for each closed operator, provenance and observation variant, with no `CheckedSemanticNodeV2.body` read."
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (FR-008-AC-1) over 0..=1000, every `Satisfying` case has post = pre, every `Violating` case has post ≠ pre, and every value lies in 0..=1000."
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "For every comparison operator of the V2 selected claim, every domain of 1, 2, 3, and 4 members, and both a two-read relation and a read-versus-literal relation with the literal at each domain member, the generatable set of each population equals the independently enumerated satisfying or violating set: none missing, none extra."
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "Repeated generation for the same `CheckedPackageV2`, selected clause id, authentic claim occurrence, and population is byte-identical."
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "PLANNED. For each noncommutative operator, a comparison written with the literal in the first position produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently."
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: "For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim (primary post, partner pre; FR-008-AC-1) over 0..=1000, the in-domain census is exactly the 10 pairs (post, pre)..."
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: "A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX` (representable under FR-008-AC-6) compiles as generated Rust without overflow or panic, its out-of-domain array holds no case for either outer edge, and its unrepresentable-edge array lists both."
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "The runner shall construct each verdict through the runtime `construct_verdict` operation (quire-contract-runtime interface-001) with the requirement and revision identity, the runtime execution point (`initialization`, `handler`, `pre`, or `post`) of the selected checked clause, and the clause observations."
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes; ..."
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: "A V2 comparison oracle deliberately swapped for its negation fails each campaign with `ConformanceMismatch` carrying the case values, expected tag, and observed verdict kind."
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: "For every V2 comparison operator and every domain of 1 to 4 members from FR-009-AC-2, an exhaustive walk of every valid `simplify` and `complicate` path ... visits only in-domain candidates that keep their original tag"
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: "A `Violating` V2 `VersionUnchanged` campaign whose comparison oracle is replaced by `post == pre || post == pre + 1` ... reports a minimal counterexample with post = pre + 1, both in 0..=1000, tagged `Violated`."
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: "Fresh 10,000-case and 256-case campaigns of every population over the V2 selected claims `VersionUnchanged`, `amount < 7`, and a `Precondition` fixture record zero discards, zero proptest global rejects, and no `Exhausted` conclusion."
- id: AD-001-codegen-architecture
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "IR-703 is implemented; a CG-side real QSL producer fixture must exist before CG V2 implementation can claim the corresponding runtime tests."
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: "Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate an `ir_confirmed` oracle ..."
bindings: []
```

## Dispositions

Round 1 reviewed d8f2f8a6af1c791f5b0b48e5bb32eafae4474b6a (diff f30f17b..d8f2f8a).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

## Disposition round 1 verdict

**PASS.**

- **FND-001.** FR-008-AC-8 is now stated by input and ownership: the oracle takes only the accessor's
  typed result, and no V1 bound oracle or `BoundPackage` is an input to it. The false descriptor
  reason is gone. The FR-008 Description records reuse of the FR-014-AC-35 eq/ne derivation as an
  open decision owned by the planner. It is recorded, not decided.
- **FND-002.** FR-009-AC-2 now reads "For every V2 comparison operator", which matches FR-012-AC-1.
- **FND-003.** FR-011 no longer claims an execution-point source. OQ-1 (execution point) and OQ-2
  (requirement and revision identity) record the gap with the planner as owner. FR-011-AC-1 to AC-5
  assert verdict kinds, counters and rates, not execution-point or identity values, so no AC depends
  on either open question.
- **Gates.** `quire validate` is clean on the changed files. `quire coverage --strict` reports 219
  unbacked and 0 contradicted; the +1 is FR-008-AC-10.
