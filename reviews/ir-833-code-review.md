---
id: SR-12810
title: IR-833 code-review supplemental review
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; IR-833; PR #416 own delta against #415; reviews/ir-833-spec-review-evidence.md,
  reviews/ir-833-spec-review-integrity.md, reviews/ir-833-spec-review-scope-boundary.md,
  reviews/ir-833-spec-review.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md,
  spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md,
  spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md,
  src/oracle/function/mod.rs, tests/exact_function_support/agreement_cases.rs, tests/exact_scalar_support/agreement.rs,
  tests/it/exact_function_agreement.rs'
review_set: subset
---

## Summary

Ticket: IR-833. Supplemental independent review of the frozen candidate, scoped to this PR’s own delta. PASS for this PR's own delta. All four Rust deltas change module documentation only. No executable source, test bodies, trace bindings, dependencies, CI, unsafe blocks, bounds, async or error contracts change. Existing historical agreement assertions and retained direct-runtime comparison match the narrowed boundary wording. No copied implementation, new schema, version catalog or weakened gate is introduced. Existing four review artifacts are historical evidence and remain verbatim; SR-12085 through SR-12088 already record all 12 findings as fixed.

## Reviewed scope

```yaml
scope:
- id: reviews/ir-833-spec-review-evidence.md
  path: reviews/ir-833-spec-review-evidence.md
  role: context_only
  excerpt: '---

    id: SR-12087

    title: "IR-833 evidence review: verification methods and matrix rows for the converted
    criteria"

    type: SpecReview

    analysis: evidence

    scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076
    (PR #416); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md,
    spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md,
    spec/oracle/matrix/tests.md, tests/it/exact_function_agreement.rs, tests/exact_function_support/agreement_cases.rs,
    src/'
- id: reviews/ir-833-spec-review-integrity.md
  path: reviews/ir-833-spec-review-integrity.md
  role: context_only
  excerpt: '---

    id: SR-12086

    title: "IR-833 integrity review: FR-021 and TC-031 internal consistency after
    the boundary conversion"

    type: SpecReview

    analysis: integrity

    scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076
    (PR #416); spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md,
    spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md,
    spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md"

    revie'
- id: reviews/ir-833-spec-review-scope-boundary.md
  path: reviews/ir-833-spec-review-scope-boundary.md
  role: context_only
  excerpt: "---\nid: SR-12088\ntitle: \"IR-833 scope-boundary review: the QSL value-authority\
    \ boundary and FR-014-AC-47\"\ntype: SpecReview\nanalysis: scope-boundary\nscope:\
    \ \"agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076 (PR\
    \ #416); spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md,\
    \ spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md,\
    \ spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/tests.md\"\
    \nreview_set: subset\nrelationships:\n  - target"
- id: reviews/ir-833-spec-review.md
  path: reviews/ir-833-spec-review.md
  role: context_only
  excerpt: '---

    id: SR-12085

    title: "IR-833 spec review: QSL value-authority lines become Inspection boundaries"

    type: SpecReview

    analysis: base

    scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076
    (PR #416, diff vs origin/peter/ir-818-quantity-convert-spec); spec/oracle/functional/FR-014-exact-scalar-oracles.md,
    spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md,
    spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md,
    spec/oracle/matrix/TC-031-function-ap'
- id: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '| FR-014-AC-37 | A differential corpus covering every node FR-014-AC-35
    names compiles against the runtime alone, and each oracle''s outcome, admitted
    charges and consumed counters equal direct runtime execution. This criterion claims
    no agreement with the QSL value authority: that boundary is owned by quire-integration
    IR-842 (generated-oracle against QSL agreement), and FR-014-AC-47 keeps CG''s
    artifacts from claiming it. | Test (TC-024) |

    | FR-014-AC-47 | No test name, trace tag, test output line or doc comment of every
    oracle-agreement corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186,'
- id: spec/oracle/functional/FR-018-composite-equality-oracles.md
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'cross-universe references. Generated oracles therefore call it and decide

    nothing themselves. Agreement with the quire-spec-language value authority is

    outside CG''s boundary and not claimed by CG (FR-014-AC-47); generated-oracle

    against QSL agreement is owned by quire-integration IR-842.

    | FR-018-AC-2 | On every vector of the composite-equality corpus, the generated
    oracle''s `Outcome<bool>`, its admitted charge sequence and its consumed counters
    are equal to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality`
    plus `CheckedEquality::evaluate` invoked directly on an indepen'
- id: spec/oracle/functional/FR-021-function-application-oracles.md
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "classifies function application. Runtime outcome and charge amounts are\
    \ not\ndecided here, and CG claims no agreement with the QSL value authority on\
    \ them\n(FR-014-AC-47; owned by quire-integration IR-842), while this requirement\n\
    decides the schedule's operand and call order.\n  Nested calls retain source operand\
    \ order and the\n  charge-before-work order of FR-021-AC-5. The schedule records\
    \ charge *points*, not charge\n  and meter charge sequences are planned owner-path\
    \ obligations; comparison\n  with the QSL value authority is outside CG's boundary\
    \ (FR-014-AC-47; owned by\n  quire-integration IR-842"
- id: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  excerpt: "byte-deterministic, and agree with direct runtime execution. Agreement\
    \ with the QSL value authority is outside CG's boundary and not claimed by CG\
    \ (FR-014-AC-47); generated-oracle against QSL agreement is owned by quire-integration\
    \ IR-842.\nordering) oracles are checked against direct runtime execution only.\
    \ Agreement\nwith the QSL value authority is outside CG's boundary (FR-014-AC-47)\
    \ and owned\nby quire-integration IR-842.\n   over the bounded domain beside direct\
    \ runtime execution. Outcomes, charges and counters agree on\n   every input (FR-014-AC-37).\
    \ No comparison with the QSL value authority"
- id: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "title: \"Verify composite equality oracle generation and agreement with\
    \ direct runtime execution\"\n# TC-029: Verify composite equality oracle generation\
    \ and agreement with direct runtime execution\nexecution. Agreement with the QSL\
    \ value authority is outside CG's boundary and\nnot claimed by CG (FR-014-AC-47);\
    \ it is owned by quire-integration IR-842.\n   charge sequence and the consumed\
    \ counters against a direct call to\n   request, not by or from the generator.\
    \ No comparison with the QSL value\n   authority runs here (FR-014-AC-47). The\
    \ descriptor\n   generated crate would make the comparison follow "
- id: spec/oracle/matrix/TC-031-function-application-oracles.md
  path: spec/oracle/matrix/TC-031-function-application-oracles.md
  role: examined
  excerpt: "    charge sequences and consumed counters with an independent application\n\
    \    assembled from the request, never read back from generated code (AC-2,\n\
    \    AC-3), and with the shallow equivalent for the deep case (AC-31); no\n  \
    \  comparison with a QSL-owned package runs here (outside CG's boundary, owned\n\
    \    by quire-integration IR-842; FR-014-AC-47); deny each charge in turn and\n\
    \    AC-9, AC-16, AC-17, AC-19, AC-20, AC-26 and AC-31; schedule compilation in\
    \ step\n12a. Inspect (AC-18, FR-014-AC-47): scan the function-application corpus's\
    \ test\n    names, `Trace:` tags, output lines and doc comments,"
- id: spec/oracle/matrix/tests.md
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: '| FR-014 | FR-014-AC-35 through FR-014-AC-37 | TC-024 | 🚧 Planned; Boolean
    and integer `eq`/`ne` nodes are refused as `OperationNotDerivable`; agreement
    with the QSL value authority is outside CG''s boundary, owned by quire-integration
    IR-842, and not claimed by CG (FR-014-AC-47) |

    | FR-018 | FR-018-AC-2 | TC-029 | ⚠️ Partially covered; every admitted shape agrees
    with `quire_contract_runtime`''s own check and evaluation, the recursive composite
    `E_SELF` (`{ next: Option<R_SELF> }`, no text, for which QSL emits `leaves: []`)
    included: Contract IR now admits an equality over a type that reaches i'
- id: src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  role: examined
  excerpt: '//! ## AC-18 (QSL authority agreement): outside CG''s boundary

    //! No test in this module''s suite claims agreement against a QSL authority:
    this repository

    //! depends only on `qsl-replay`''s public API (QSL arch-lint T12-A), and that
    agreement is owned

    //! by quire-integration IR-842 (FR-014-AC-47). AC-2''s two legs (generated oracle,
    direct runtime'
- id: tests/exact_function_support/agreement_cases.rs
  path: tests/exact_function_support/agreement_cases.rs
  role: examined
  excerpt: '//! Agreement with the QSL application authority (FR-021-AC-18) is outside

    //! CG''s boundary and not claimed here: this repository reaches QSL only

    //! through `qsl-replay`''s public API, and that agreement is owned by

    //! quire-integration IR-842 (FR-014-AC-47). This file''s agreement legs are

    //! exactly two: the generated oracle and a direct Contract Runtime call.'
- id: tests/exact_scalar_support/agreement.rs
  path: tests/exact_scalar_support/agreement.rs
  role: examined
  excerpt: //! owned by quire-integration IR-842.
- id: tests/it/exact_function_agreement.rs
  path: tests/it/exact_function_agreement.rs
  role: examined
  excerpt: '//! Agreement with the QSL application authority (FR-021-AC-18) is outside

    //! CG''s boundary and not claimed here: this repository reaches QSL only

    //! through `qsl-replay`''s public API, and that agreement is owned by

    //! quire-integration IR-842 (FR-014-AC-47). The agreement legs are exactly

    //! two: the generated oracle and a direct Contract Runtime call.'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this PR's own delta. All four Rust deltas change module documentation only. No executable source, test bodies, trace bindings, dependencies, CI, unsafe blocks, bounds, async or error contracts change. Existing historical agreement assertions and retained direct-runtime comparison match the narrowed boundary wording. No copied implementation, new schema, version catalog or weakened gate is introduced. Existing four review artifacts are historical evidence and remain verbatim; SR-12085 through SR-12088 already record all 12 findings as fixed.

## Coverage

Rust review was folded into this method. Read AGENTS.md and CLAUDE.md; no applicable AssuranceProfile exists in spec/. Reviewed all own changed paths including clean ones. Coder receipts show make fmt-check, locked all-target lint, and serialized focused agreement tests (10 passed, 0 failed). No Cargo was repeated. The root still owns the aggregate exact-head gate. Gap-analysis was assessed and skipped: the production .rs edit is module prose only and adds no executable behavior, test binder or claim of newly implemented coverage. Repository-wide SR-12730 deficits remain routed to IR-726 and the assigned remediation lanes; this review makes no repository-wide PASS claim. Existing legacy Trace lines outside the delta remain IR-865 scope.
