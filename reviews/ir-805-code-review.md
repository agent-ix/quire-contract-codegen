---
id: SR-12830
title: Code review of IR-805 symbolic graph specification
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-805; spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md;
  five preserved IR-805 review artifacts
review_set: subset
---

## Summary

Supplemental code-review of the frozen IR-805 specification candidate. The earlier base, failure-domain, integrity, EARS and evidence analyses remain preserved; this method finds duplicated producer contracts and stale upstream availability claims.

## Verdict

**FAIL** — one high duplication finding and one medium availability finding. This is a specification review, not a runtime implementation or proof verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015 duplicates producer-owned contracts: the complete QSL reachability node/member wire shape and the nine-field CheckedRelationshipEnd API record are restated locally. Replace these upstream definitions with references to QSL FR-365/FR-105 and Contract IR FR-038-AC-262..268, retaining CG-owned consumption, request identity, harness and disposition obligations. | spec/kani/functional/FR-015-bounded-kani-obligations.md:118-140 |
| FND-002 | medium | Current availability prose says IR-851 code is unmerged and IR has no claim-to-population mapping, although IR PRs #438 and #442 are merged. Update availability notes across FR-015, TC-025 and tests.md to distinguish landed IR accessors, the outstanding CG dependency/integration work, and QSL-820 real-package population emission. Preserve conditional refusal semantics and Planned graph proof status. | spec/kani/functional/FR-015-bounded-kani-obligations.md:140-160 |

## Method scope and evidence

All eight changed files were inspected. No applicable AssuranceProfile exists in the spec tree. No production source, Rust, dependencies, test bodies, coverage thresholds or warning controls change. No Rust lane or production gap-analysis applies. The existing five semantic methods were not rerun because their payload and terminal dispositions are preserved. No new source/test execution or Cargo/Kani job was run, and no whole-repository pass is claimed.

Ownership checks compared the added QSL shape against producer FR-365 and FR-105 on its origin/main and the nine-field API against Contract IR FR-038:3202–3213. The producer definitions are the authoritative homes; the local CG specification need only state how it consumes them. Upstream merge state was independently read from GitHub: Contract IR #438 and #442 are merged. Private IR-805 planner comment 04b1b2cc-3e5a-40c5-836b-3b788e3dea65 separately identifies QSL-820 as the remaining real-package population dependency. The CG integration remains Planned.

The original 19 findings across SR-12100..12104 retain fixed as their latest outcomes. Prior review artifact text and outcomes were not edited.

## Examined units

```yaml
scope:
- id: FR-015 / graph dependency status
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Planned (IR-805, graph half of IR-268): FR-015-AC-98, -105, -106 and -107,
    with

    the graph clauses of -99 to -102, specify bounded reference-graph reachability

    claims in the same V2 item path. QSL-804 is Done: QSL (`quire-spec-language`)
    lowers a checked `reaches` to a V2 expression node, and

    Contract IR (`quire-contract-ir`, IR #422) admits it, including

    a finite multi-valued destination end. IR-268 earns no graph implementation or

    proof credit. The finite-input `GraphRequest` and corpus are not evidence for

    symbolic graph proofs, and caller `max_expansions` is not a domain input.'
- id: FR-015 / upstream wire contract
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The claim is one reachability application inside an admitted `state` /

    `state_clause` node (a `reaches` in a function body is refused by QSL). The

    application is either of two forms. The relationship-end form has

    `semantic_form` `reachability`, `operator` `reaches`, `operation.identity`

    `quire.op.model.reaches`, `operation.member` `{kind: "relationship_end",

    declaration: <the relationship''s model node>, name: <the end''s role>}`, no laws,

    leaves or mode, `arguments` `[reference(a), reference(b)]` and a `Boolean`

    `result_type`. The field-edge form has `operation.identity`

    `quire.op.model.reaches'
- id: FR-015 / typed upstream accessor
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Typed Contract IR inputs: reachability is homogeneous. Both operands conform
    to

    the receiver endpoint''s type `E` (IR FR-038-AC-252), so one object type and one

    population cover the start and the target (a subtype shares that population''s

    domain key). Edge multiplicity for the field-edge form is the selected field''s

    `CheckedMemberType` (`Reference`, `Option` or bounded `Collection`) from

    `model_object_fields(D)`; for the relationship-end form it is the destination

    end''s finite multiplicity from `CheckedPackageV2::relationship_end(&reaches application)
    -> CheckedRelationshipEnd { application, re'
- id: FR-015 / disposition availability
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: "Disposition sequence for a graph claim, in the order that holds:\n1. Today\
    \ IR has no mapping from a reaches claim to its population selection\n   (QSL\
    \ requirement records are not on the V2 wire, a domain-package\n   population\
    \ has no V2 node, and `select_population` returns `Unbounded` only\n   from a\
    \ declaration selection), so the selection is `Unavailable` and every\n   claim\
    \ settles `unsupported`, citing IR-852.\n2. IR-852 lands only after QSL-819 (bounded\
    \ domain population for reaches),\n   because QSL today gives a state-clause `reaches`\
    \ no finite population (QSL\n   FR-104 records an unbounded e"
- id: FR-015 / graph request input
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: "(IR-805) is one admitted `state_clause` node id, the argument path to\
    \ one\n  reachability application (`quire.op.model.reaches` or\n  `quire.op.model.reaches_field`),\
    \ the customer subject and request ceilings;\n  its population is read from the\
    \ admitted package, never named or sized by\n  the caller; the selected edge's\
    \ multiplicity comes from\n  typed `CheckedMemberType` (`Reference`, `Option`\
    \ or bounded `Collection`) or,\n  for a relationship end, from IR's typed end\
    \ multiplicity. Concrete values and caller"
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its
    `state_clause` node id and argument path) generates a harness when the admitted
    package gives its covering object type `E` a finite population `[0, N]` through
    `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches`
    the destination end's finite multiplicity). Every symbolic object and edge is
    bounded inclusively by those declarations, including zero and maximum where admitted.
    Reacha
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic
    inputs drawn from the selected model field's typed domain, ends with the FR-015-AC-7
    cover after its assertions, and never replaces those inputs with the finite-input
    corpus's concrete `Vec<i128>`. The independent native oracle replays the same
    authored function through `qsl_replay::replay`; it does not substitute a separately
    authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani
    `Verified` with its cover satisfied; a seeded claim or subject mutant yields `Falsified`
    with a concrete in-dom
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality
    or element bound, admitted body claim, customer subject or unwind changes the
    harness identity; equal admitted inputs regenerate byte-identically. The identity
    records every symbolic collection domain and the solver and complete ordered option
    vector, and FR-017 run evidence records the actual Kani, Rust and solver versions,
    unwind and every option used by the installed backend. For the graph family (AC-98),
    changing the claim's clause id or argument path, the reachability form, relationship
    or field, its role or nam
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing or malformed selected declaration likewise refuses
    with its typed reason and no harness. For a graph claim, a population or edge
    domain whose symbolic object or edge count exceeds the ceiling likewise settles
    `unsupported` naming the domain, with no harness and no reduced population or
    multiplicity. PLANNED (IR-268 collection; '
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply
    the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. PLANNED (IR-268 collection; IR-805 graph).
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and symbolic objects within it; for every object, the
    selected edge as its shape states: a `Reference` field has exactly one target,
    an `Option` field an absent or one target, a `Collection` field or finite multi-valued
    relationship end a count in `[lower, upper]` (a unique end admits no repeated
    target, a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at
    least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each
    concrete graph through QSL''s replay (`qsl_replay`), and does not substitute a
    separately authored reachability. With the installed backend, a QSL-emitted, IR-admitted
    healthy case yields `Verified` with its cover satisfied for one field-edge claim
    and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique
    end; each seeded mutant yields `Falsified` with an in-domain playback: reachability
    that allows a zero-length path (so an isolated `start = target` reads reachable),
    one that rejects a self-loop, one f'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice in one condition yields two items at positions
    0 and 1; one node under two clauses yields two items with different identities;
    a field-edge and a relationship-end reachability yield separate items; a collection
    claim, a clause with no reachability and a path that does not name a reachability
    application each settle `unsupported` naming the missing claim with no harness.
    A mixed request settles one disposition per item and a supported item''s harness
    is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
    or caller `max_expansions` substitute. While the population selection for the
    claim is `Unavailable` (no IR mapping from the claim to a population, IR-852),
    the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`,
    that is a claim whose population has no finite authored bound (after QSL-819 and
    IR-852), settles `requires_bound` naming the population; an unbounded `Collection`
    edge settles '
- id: TC-025 / Purpose
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'Verify that bounded contracts produce one harness per obligation kind
    and

    refuse non-finite obligations. IR-268 covers the collection half of planned V2

    symbolic obligations: the claim is an admitted QSL clause/function body over a

    selected model collection field. Graph reachability is required by

    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108 (IR-805); it is not IR-268

    evidence. The finite-input collection and graph corpus is not proof evidence

    for either symbolic family.'
- id: TC-025 / graph fixture
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'For the graph family (IR-805), author a QSL unit whose model has an object

    type `Order` with an optional `Reference<Order>` field, a `Reference` field and

    a bounded `Sequence<Reference<Order>>` field, and relationships `next`

    (`{0,1}`), `kids` (`{0,3}` unique), `litter` (`{0,3}` not unique) and a `{2,3}`

    end; write invariant/precondition/postcondition clauses of the form

    `reaches(self, target, <name>)` and `not reaches(self, self, <name>)`. Emit

    through QSL and admit through Contract IR (no copied wire or hand-built

    builder). QSL gives such a clause no finite population (it is a state-clause

    `'
- id: TC-025 / inspection boundary
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'Inspection: the three `relationship_end` refusals (`UnknownNode`,

    `NotReachesApplication`, `UnresolvedEnd`; Contract IR FR-038-AC-262 to

    FR-038-AC-268) cannot arise from a QSL-emitted, IR-admitted package

    (`UnresolvedEnd` needs a crate-internal post-admission mutation), so their

    `unsupported` mapping in FR-015-AC-108 is checked by inspection of the

    generator''s error mapping, or by a named in-crate accessor test double, and is

    not a testable-now step. AC-107''s location refusal runs before the accessor,
    so

    a path naming no reachability is never `NotReachesApplication`. Also, QSL refuses
    `reaches'
- id: TC-025 / future proof controls
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'Steps that wait on the QSL ticket (unbacked):

    positive-length semantics (an isolated `start = target` unreachable, a self-loop

    reachable) in the generated draw and the `qsl_replay` oracle; inclusive

    populations `N = 0` and the maximum, edge counts `lower`, `upper` and between,

    `kids` uniqueness, `litter` repeats and `{2,3}`; the healthy `Verified` case and

    each FR-015-AC-106 mutant `Falsified` with an in-domain playback; and identity

    changes for the clause path, form, field or role, population extent, edge

    multiplicity, subject and unwind, with byte identity on equal inputs. The

    finite-input `'
- id: TC-025 / symbolic graph matrix
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 |
    FR-015-AC-96, FR-015-AC-97, FR-015-AC-98, FR-015-AC-99, FR-015-AC-100, FR-015-AC-101,
    FR-015-AC-102, FR-015-AC-105, FR-015-AC-106, FR-015-AC-107, FR-015-AC-108 | 🚧
    Planned (IR-268); symbolic collection and graph draws, typed refusals, inclusive
    bound and identity controls, installed-Kani verified and mutant counterexample
    runs are unbacked; graph success (IR-805) awaits IR-852 (claim-to-population mapping,
    itself blocked on QSL-819) and QSL-819 (finite population for a state-clause reachability),
    and IR-851 (relationship-end'
- id: SR-12100
  path: reviews/ir-805-spec-review-base.md
  role: context_only
  excerpt: 'Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.

    Method: spec-review/base (checklist, ID formats, coverage rules, hash/pin antipattern).

    Diff reviewed: `git diff origin/main...HEAD`. Ground truth read via `git show`
    at

    quire-spec-language 46aa650be and quire-contract-ir 64f884e7 / 299a126.'
- id: SR-12104
  path: reviews/ir-805-spec-review-ears-conformance.md
  role: context_only
  excerpt: 'Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.

    Method: spec-review/ears-conformance over the new and rewritten criteria.'
- id: SR-12103
  path: reviews/ir-805-spec-review-evidence.md
  role: context_only
  excerpt: 'Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.

    Method: spec-review/evidence (can each control be produced with the declared Test
    method and

    TC-025''s QSL-emission-only rule).'
- id: SR-12101
  path: reviews/ir-805-spec-review-failure-domain.md
  role: context_only
  excerpt: 'Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.

    Method: spec-review/failure-domain. Ground truth: QSL 46aa650be (FR-104, FR-365,

    qsl-semantics/src/check/lowering.rs:3669-3727, qsl-package/src/emit/tests/reachability.rs)
    and

    IR 64f884e7 (FR-053, selected_population.rs, model_fields.rs, model_members.rs,
    FR-038-AC-252..257).'
- id: SR-12102
  path: reviews/ir-805-spec-review-integrity.md
  role: context_only
  excerpt: 'Ticket: IR-805. PR: agent-ix/quire-contract-codegen#417 at edbe6aabdd756a864482c2b73b1c0c113b101894.

    Method: spec-review/integrity (completeness, internal consistency, atomicity).'
```
