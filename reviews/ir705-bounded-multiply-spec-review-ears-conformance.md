---
id: SR-15345
title: IR-705 bounded multiplication ears-conformance review
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen; IR-705; spec/kani/functional/FR-015-bounded-kani-obligations.md;
  spec/assurance/AD-006-admitted-native-integer-oracles.md
review_set: subset
---

## Summary

Independent source-only review of PR506 exact a3e249c5181e4d61cd0a8ef7d25a95001050f2ef. The changed requirement-bearing paragraphs name CG or Generated artifacts as subject and use SHALL for concrete checked-plan selection and identity binding. Existing clauses describe unwanted conditions and prohibit narrowing or fake success. No changed EARS engine warning identifies a missing subject, vague response or unclassifiable obligation; the several sentences all constrain the single additive native selection behavior. No grammar defect changes what an implementer must build.

## Verdict

PASS — No new specification defect found. No implementation, proof or memory-fit acceptance is implied.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined units

```yaml
scope:
- id: FR-015-AC-125
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs
    generate kernel-owned native carriers and checked operation plans for all intermediates,
    with original graph identities/domains retained. Add/Subtract/Negate/ordering
    retain their admitted signed-i128 capacities; Multiply is eligible exactly when
    both whole operand intervals fit signed32 and uses the exact four-corner signed64
    image retained in i128. A Multiply operand interval outside signed32 makes the
    whole requested arithmetic component native-ineligible, even with a zero opposite
    operand, fitting actual payload or
- id: FR-015-AC-126
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917
    clause implication and IR-948 V2 pre/post/invariant profiles regenerate through
    public CG with the reviewed admitted kernel API. Original semantic domains, selected
    propositions/clauses, independent expected values, subject controls, assertions,
    final completion covers and existing solver/unwind/time settings remain. Static
    plan-admission checks establish that the original TC-027, IR-917 and IR-948 fixture
    operand/intermediate intervals remain admitted under this policy, including their
    original 0..1000 domains; those ch
- id: FR-015-AC-127
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: End-to-end denied/cancelled admitted-native controls retain all real Meter
    counters, charge order, complete incomplete/refusal fields and admitted prefixes
    at every reached charge point, compared with independent normative expectations
    and the general exact route. A semantic result-domain failure remains after arithmetic
    charge and before result-retain, separately from generation-time native-capacity
    failure.
- id: FR-015#3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'CG SHALL use FR-384''s operator-specific capacity policy through the actual
    checked kernel plan: native i128 carriers and interval endpoints, full admitted
    signed-i128 Add/Subtract/Negate and ordering capacities, and Multiply only when
    both complete operand intervals lie within signed32 `[-2147483648, 2147483647]`,
    with the exact four-corner signed64 product image retained in i128. Semantic result-domain
    endpoints remain i128 and are distinct from the operation capacity. Eligibility
    SHALL derive from the original complete operand and intermediate intervals; a
    result-domain restriction or a fitt'
- id: FR-015#4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Generated artifacts and canonical identity inputs SHALL bind the actual
    selected operator-capacity policy and its revision together with the selected
    ABI and interval decisions, under the owning identity schema's compatibility rules.
    A policy revision changes that identity even when the original fixture domains
    remain admitted and the rendered arithmetic for them would otherwise be identical.
    New qualification uses fresh evidence for that policy and actual production kernel
    implementation, not an old unrestricted-i128 multiplication proof attempt.
- id: AD-006#5
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: 'CG reads operand intervals and every reachable arithmetic operation from
    the admitted graph through the existing IR-owned seams. It derives conservative
    interval bounds in expression order, preserving source identities and parameter
    relationships. The native capacity request includes intermediate results, not
    only the final declared domain. The kernel checks operator-specific capacity and
    creates a checked operation plan: Add/Subtract/Negate retain full signed-i128
    operands where their complete image fits i128, ordering retains full-i128 payloads,
    and Multiply requires both complete operand in'
- id: AD-006#6
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: CG records the selected kernel capability, actual operator-capacity policy
    and its revision, and input/result interval decisions in the generated artifacts
    and identity inputs using the repository's existing canonical identity mechanism.
    It does not invent checked node ids or equate a caller descriptor with package
    admission. Any public struct/serialized identity change follows the owning schema's
    compatibility rules rather than silently changing a digest interpretation. Changing
    the selected capacity policy/revision changes identity even when a fixture's source
    domains and rendered arithmetic
- id: AD-006#7
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: A kernel capacity refusal is a typed generation/capability disposition
    carrying the offending source and capacity reason, before artifact success. It
    is distinct from IntegerOutOfDomain for a completed mathematical result outside
    a declared semantic domain. The general exact route remains available under its
    existing capability rules; selection failure is not proof success. Eligibility
    is checked over the original complete operand/intermediate intervals of the whole
    requested arithmetic component. Any Multiply endpoint outside signed32 refuses
    native selection of that component, even if a samp
- id: AD-006#8
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: 'Kernel qualification supplies machine-checked full-capacity refinement
    against an independent mathematical abstraction of actual BigInt semantics; sampled
    differential tests alone are not a universal proof. Kernel qualification proves
    native admission and every operation against BigInt semantics over its full accepted
    operator capacity, including complete signed32 multiplication with widened signed64
    products and the unchanged full admitted i128 capacities of the other operations,
    plus real Meter/refusal refinement. Unsuccessful earlier unrestricted-i128 multiplication
    evidence stays retained '
- id: AD-006#9
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: '| Decision | Choice | Alternatives and trade-off |

    | --- | --- | --- |

    | Value ownership | Kernel Copy i128 carrier, checked per-operation interval plan
    and opaque per-call admission. | General Integer/cache and private Copy charge
    tuning preserve more call sites but have not established 16 GiB fit. A new capability
    creates an explicit admission/refinement obligation while removing successful
    native execution''s BigInt branch. |

    | Operator capacity | Kernel-owned signed32 Multiply operands and exact widened
    signed64 image, with i128 carriers/result-domain endpoints and unchanged full
    admitted i'
- id: TC-027
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: context_only
  excerpt: 'Verify that a bounded Kani obligation runs against the installed backend,
    that

    its outcome is read from the backend rather than defaulted, and that the

    returned evidence describes the invocation that actually happened.'
```

## Coverage

Plan completion: not assessed

Applicable AssuranceProfile: none found. Review set is the applicable subset selected by the lead. Rust review is inapplicable: no Rust/manifest code changed. Dependency analysis is inapplicable: no new relationship edge. Object review is inapplicable: no domain-object edits. Optional exhaustive semantic gap review, Jev strength review and architecture-evaluation were not requested and were not run. No Cargo, dependency resolver, compiler, Kani, namespace or host-security operation was run. Whole matrix was mechanically reconciled; untouched tagged criteria were not individually semantically reviewed.
