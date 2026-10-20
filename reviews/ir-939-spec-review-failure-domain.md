---
id: SR-14205
title: IR939 spec-review/failure-domain review
type: SpecReview
analysis: failure-domain
scope: FR-015-AC-106, its supplemental domain contract and TC-025 graph procedure;
  adjacent gates context
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

PASS. Extension boundary is the emitted-sampler observer: absence/inadequate capacity rejects G without out-of-bounds access; absence of actual owner inputs leaves control unqualified. Claim identity includes original declaration, clause/path and selected edge; unrelated role/field may not supply expected truth. D and original package remain unchanged under mutation, preventing shared mutated-reader expectations. Candidate constraints are carrier-only and cannot exclude the compared domains; constant-false predicates, dormant graph additions and metadata-only comparisons cannot count. Fixed finite storage and existing checked integer/ceiling refusals bound topology without trimming the original domain. Zero population has no live operands; self-loop, disconnected/isolated start=target, cycles and wrong-role semantics remain in original-domain native replay, distinct from domain oracle evidence. N>0 is required for narrowing and unique U+1 witnesses require enough objects. N4/U3 satisfies all named boundary preconditions.

## Verdict

**PASS (normative specification scope).** No code completion or backend acceptance is implied.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new normative defect found (placeholder) | - |

## Coverage

The reviewed candidate is pure Markdown: only FR015 and TC025 change. No source, lock, schema, dependency, architecture or CI edit exists. No applicable AssuranceProfile/review_selection was found. Rust/React/Python language lanes are inapplicable to this diff; no heavy Cargo/rustc/fmt/Kani/CBMC command or reservation ran. Static source inspection is not compiled feasibility or backend proof. Original PR474 and its SR14100/FND001 stay open; this review grants no code acceptance, field positive qualification, parent IR805 completion or AC111 retirement. All new execution remains PLANNED/UNRUN. QSL820's real field population link and the IR typed field gate remain missing in the supplied measured269/335/6ca source evidence; those reports are source research, not new backend runs. The frozen original474 graph source is context, never adopted into this branch. External PR477 remains unmerged/unadopted: its implication Behavior/AC119 and later TC Expected Results/runtime helper content are disjoint from this graph subsection, though shared file context needs fresh merge checking. Root owns fresh origin/main and latest custody verification and merge. Same author receives every new normative finding and same reviewer its dispositions.

Skipped methods: dependency (no relationship edges added), object-review (no owner entity/schema added or changed), architecture evaluation (no architecture edit), security (no changed security surface), optional broad semantic whole-repo review (not requested), optional criterion-strength/Jev (not selected; no jev executable found). Scoped domain intent and feasibility analysis ran directly in the requested lenses. No severity quota was used.

Checked source head 5b207149d64f6e69634c3c1afcb60c94bc4a0496; assigned base 4b578b4557b0e31f42880c09d63d7d58d2db4581; actual model gpt-6.1-sol; run 99b4caa9-581a-40c6-8697-906f6d0391c6.

## Examined Scope and Bindings

Full ordered scope and binding multiset follow. Source excerpts are nonempty verbatim substrings of their recorded paths, <=600 characters, with source head and location. Structural associations are not semantic discharge.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-939
pr: quire-contract-codegen#479
checked_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
assigned_base: 4b578b4557b0e31f42880c09d63d7d58d2db4581
model: gpt-6.1-sol
run: 99b4caa9-581a-40c6-8697-906f6d0391c6
artifact_id: SR-14205
method: spec-review/failure-domain
clean: true
normative_delta_verdict: PASS
whole_repo_verdict: not assessed by this scoped method
scope:
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-7
  excerpt_start_line: 715
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the
    precondition holds within the IR bounds, and a contract harness''s cover stands after the contract call, so
    a run that satisfies every check without satisfying the cover is not a proof. The cover is the last statement
    of the harness, after every assertion the harness carries: the state-clause and frame-effect harnesses and the
    scalar harness place it after their assertions, and a contract harness''s checks are inside the contract call
    its cover follows. A cover that preceded an assertion would '
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-98
  excerpt_start_line: 806
  excerpt: A graph claim (one reachability application of either form above, at its `state_clause` node id and argument
    path) generates a harness when the admitted package gives its covering object type `E` a finite population `[0,
    N]` through `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches` the destination
    end's finite multiplicity). Every symbolic object and edge is bounded inclusively by those declarations, including
    zero and maximum where admitted. Reacha
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-100
  excerpt_start_line: 808
  excerpt: For the collection family, changing the selected field's authored cardinality or element bound, admitted
    body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically.
    The identity records every symbolic collection domain and the solver and complete ordered option vector, and
    FR-017 run evidence records the actual Kani, Rust and solver versions, unwind and every option used by the installed
    backend. For the graph family (AC-98), changing the claim's clause id or argument path, the reachability form,
    relationship or field, its role or nam
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-101
  excerpt_start_line: 809
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational or resource ceiling
    settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness; it never tightens
    the model domain to make a proof tractable. A missing or malformed selected declaration likewise refuses with
    its typed reason and no harness. For a graph claim, a population or edge domain whose symbolic object or edge
    count exceeds the ceiling likewise settles `unsupported` naming the domain, with no harness and no reduced population
    or multiplicity. PARTIAL (IR-268, CG #396 fe2'
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-105
  excerpt_start_line: 813
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the one covering population and symbolic
    objects within it; for every object, the selected edge as its shape states: a `Reference` field has exactly
    one target, an `Option` field an absent or one target, a `Collection` field or finite multi-valued relationship
    end a count in `[lower, upper]` (a unique end admits no repeated target, a non-unique end repeats, and `lower
    > 0`, such as `{2,3}`, forces at least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 814
  excerpt: 'The independent native oracle replays the same authored clause over each original-domain concrete graph
    through QSL''s replay (`qsl_replay`), and does not substitute a separately authored reachability. With the installed
    backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge
    claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end. Each semantic mutant
    yields `Falsified` at the clause assertion with an original-domain playback and native replay of that same authored
    clause: reachability that allows a zero-len'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-107
  excerpt_start_line: 815
  excerpt: 'Graph claims are located by `state_clause` node id and argument path: one reachability written twice
    in one condition yields two items at positions 0 and 1; one node under two clauses yields two items with different
    identities; a field-edge and a relationship-end reachability yield separate items; a collection claim, a clause
    with no reachability and a path that does not name a reachability application each settle `unsupported` naming
    the missing claim with no harness. A mixed request settles one disposition per item and a supported item''s
    harness is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-108
  excerpt_start_line: 816
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition sequence
    for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute. While the population
    selection for the claim is `Unavailable` (no available population selection for this admitted claim), the claim
    settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`, that is a claim
    whose population has no finite authored bound (once a selection is available), settles `requires_bound` naming
    the population; an unbounded `Collection` edg
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: acceptance_criterion
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-111
  excerpt_start_line: 819
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b, on FR-015-AC-109 and FR-015-AC-110
    having landed, and on IR-805, whose graph claims need CG integration of the merged IR-851/IR-852 accessors and
    QSL-820 real-package population emission (after QSL-819)). Graph claims are served by FR-015-AC-98 and FR-015-AC-105
    to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay is the replacement for the corpus graph replay;
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice PR's head
    that is green, cited in the PR (the real-Kani
- id: FR-015-AC-106/domain-paragraph-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 853
  excerpt: 'The graph verification controls distinguish clause semantics from domain

    conformance and completeness. This clarification defines a test oracle, not an

    additional production generator, public input model or proof of delivery. The

    IR-805 symbolic-graph criteria remain planned until their required evidence

    exists.'
- id: FR-015-AC-106/domain-paragraph-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 859
  excerpt: 'For a selected claim, let `D(g)` mean that the concrete active graph `g`

    belongs to the original authored domain: its population count is in `[0,N]`,

    every active row has degree in the selected edge''s inclusive `[lower,U]`,

    every active target belongs to that population, and its uniqueness and operand

    constraints hold. Required and optional fields retain their respective exact

    one and zero-or-one multiplicities. A zero population has no live operands;

    any representation sentinel is not an object or a path. Only active rows and

    active edge slots describe the graph; padding is not an extra objec'
- id: FR-015-AC-106/domain-paragraph-3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 868
  excerpt: 'The expected predicate `D` is independently implemented from the retained

    original QSL model declarations and their admitted typed owner projections:

    `reaches_population` / `select_population`, and `relationship_end` or

    `model_object_fields` / `CheckedMemberType`, as applicable. The fixture checks

    these original projections against its authored population, selected

    role/field, multiplicity and uniqueness declarations before mutation. Neither

    `D` nor its expected endpoints may be obtained from the generated harness, the

    mutated bound, or the generator''s mutated lowering result. Mutating a reade'
- id: FR-015-AC-106/domain-paragraph-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 879
  excerpt: 'Let `G(g)` mean that the actual generated sampler can represent and admit that

    same active graph, before evaluating the clause. Its observation includes its

    real storage extents, input ranges, active-count interpretation and all

    sampler assumptions, not just identity metadata or the intended bound. For the

    fixed-array encoding, population capacity, row capacity and degree capacity

    must cover every active row and slot of `g`; absent capacity means `G(g)` is

    false, without indexing outside an array. Dormant padding may be normalized

    only after checking that this preserves admission of every acti'
- id: FR-015-AC-106/domain-paragraph-5
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 893
  excerpt: 'With installed Kani, the separate domain oracle asserts both `G(g) implies

    D(g)` (conformance) and `D(g) implies G(g)` (completeness) over a common

    bounded candidate carrier. The healthy oracle classifies `Verified` with a

    satisfied final cover witnessing `D(g) and G(g) and count = N`.

    Separate boundary-probe runs each retain exactly one final cover and witness

    `D(g) and G(g)` with the selected edge''s lower and upper degrees where

    feasible; no harness gains an additional cover. These candidates are drawn independently of `G` and `D`:

    neither predicate, their conjunction nor the original sample'
- id: FR-015-AC-106/domain-paragraph-6
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 906
  excerpt: 'The candidate carrier is verification storage, distinct from the production

    sampler''s arrays. It represents the union of the healthy domain and all three

    mutated domains, including `N+1` active objects and `U+1` active edges per

    row, without pretending the production `[N][U]` arrays can hold them. The

    observer reports unrepresentable candidates as rejected rather than truncating

    them. All original inclusive ranges remain quantified in full; ceiling or

    integer/array representability failures remain refusals, never a smaller

    replacement domain. A control whose union carrier cannot be represented'
- id: FR-015-AC-106/domain-paragraph-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 917
  excerpt: 'Each pure-bound mutant changes only the generated domain construction for the

    selected population maximum or selected edge upper bound, including any

    storage extents derived from that bound, while keeping the original package,

    expected predicate, clause evaluation rules and candidate carrier unchanged.

    Storage and unrolling dimensions may follow the changed bound, but active-

    count interpretation and positive-length search semantics do not change. The

    population-minus-one control uses `N > 0`. Fixtures make the changed boundary

    satisfiable; in particular a unique `U+1` edge witness needs at le'
- id: FR-015-AC-106/domain-paragraph-8
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 936
  excerpt: "- Population `N+1`: a graph with `N+1` active objects is admitted by `G` and\n  rejected by `D`; this\
    \ is a mutant-only conformance witness.\n- Population `N-1`: an original-valid graph with `N` active objects\
    \ satisfies\n  `D` and is rejected by `G`; this is an omitted original-domain completeness\n  witness, even\
    \ when the smaller production array cannot encode it.\n- Edge upper `U+1`: an otherwise valid graph with one\
    \ active row of degree\n  `U+1` is admitted by `G` and rejected by `D`; this is a mutant-only\n  conformance\
    \ witness, preserving original lower bound and uniqueness."
- id: FR-015-AC-106/domain-paragraph-9
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  unit_type: criterion_supplement
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: FR-015-AC-106
  excerpt_start_line: 945
  excerpt: 'An array bounds fault, changed traversal limit, added dormant row/edge,

    constant-false predicate, changed claim or an expectation copied from the

    mutant does not qualify. Pure assumption narrowing need not change clause

    truth on any retained graph. Mutant-only witnesses are not original-domain

    clause counterexamples and earn no original-domain replay credit. The separate

    semantic controls above still require original-domain playback and same-clause

    `qsl_replay` agreement. Missing real-package population links or typed field

    accessors retain AC-108''s refusal; a test-only carrier or a relationsh'
- id: TC-025/domain-procedure-1
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  unit_type: test_procedure
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Test Procedure
  excerpt_start_line: 119
  excerpt: 'For FR-015-AC-106''s separate domain controls (IR-939, planned), retain the

    original authored QSL source, admitted package, selected clause/path and typed

    population/edge projections. Independently check the expected domain against

    the authored declarations before seeding a mutation. Use an available real-

    package population profile with `N = 4`, lower degree `0` and upper degree `U

    = 3`, once with a unique selected end and once with a non-unique selected end;

    retain the existing field-edge, `{2,3}`, zero-population, identity and refusal

    cases above. The fixed verification carrier has capacity '
- id: TC-025/domain-procedure-2
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  unit_type: test_procedure
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Test Procedure
  excerpt_start_line: 136
  excerpt: 'Run the unmutated domain oracle with installed Kani, asserting both

    conformance and completeness as FR-015-AC-106 defines. Require `Verified`, a

    satisfied final cover with `D = G = true` at count four, and separate

    boundary-probe runs with `D = G = true` for degree zero and three, each with exactly one final cover. Run three
    separate

    mutants against the unchanged original expected domain, candidate carrier and

    clause evaluation rules: generated population maximum five, maximum three, and

    selected edge maximum four. Change associated production storage when the

    selected bound determines it; do '
- id: TC-025/domain-procedure-3
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  unit_type: test_procedure
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Test Procedure
  excerpt_start_line: 152
  excerpt: "- Population five: five objects, zero edges, operands at object zero;\n  `G = true`, `D = false`, violating\
    \ conformance.\n- Population three: four objects, zero edges, operands at object zero;\n  `D = true`, `G = false`,\
    \ violating completeness. A capacity rejection is an\n  observed omission, not a memory fault or a clause-result\
    \ change.\n- Edge maximum four: four objects, one row containing four targets and every\n  other row empty,\
    \ operands at object zero; `G = true`, `D = false`, violating\n  conformance. For the unique end use targets\
    \ zero through three; for the\n  non-unique end include repeated "
- id: TC-025/domain-procedure-4
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  unit_type: test_procedure
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Test Procedure
  excerpt_start_line: 163
  excerpt: 'Retain source, actual installed-backend evidence, failed property identity,

    non-vacuity/boundary cover results and native domain-witness checks for each

    profile and run. This is a separately identified domain oracle, not a new

    reachability assertion or a claim that the original QSL clause changes truth.

    Do not replay mutant-only graphs as original-domain semantic counterexamples.

    The semantic mutants still require same-authored-clause `qsl_replay` on

    original-valid graphs. No sampler filtering, reduced authored range, metadata

    comparison, generation-time Boolean or substituted traversal mutant'
- id: FR-015
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  unit_type: section
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Description
  excerpt_start_line: 21
  excerpt: 'When a caller selects Kani generation for complete-V1 contracts whose scalar

    oracles FR-014 generated, the code generator shall emit one bounded Kani

    obligation per precondition, postcondition, invariant and frame condition,

    each bounded by its model-domain bounds. It is the Kani

    backend''s one generator

    ([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),

    and every item it lowers is a claim over an admitted `quire.checked-package/v2`

    package.


    For production requests after the FR-330 consumer transition,

    [FR-039](../../core/functional/FR-039-contract-package-consump'
- id: TC-025
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: context_only
  unit_type: section
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Description
  excerpt_start_line: 15
  excerpt: 'Verify that bounded contracts produce one harness per obligation kind and

    refuse non-finite obligations. IR-268 covers the collection half of planned V2

    symbolic obligations: the claim is an admitted QSL clause/function body over a

    selected model collection field. Graph reachability is required by

    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108 (IR-805); it is not IR-268

    evidence. The finite-input collection and graph corpus is not proof evidence

    for either symbolic family.'
- id: StR-001
  path: spec/core/stakeholder/StR-001-traceable-generation.md
  role: context_only
  unit_type: section
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Stakeholder Need
  excerpt_start_line: 49
  excerpt: 'Assurance engineers require that the generator shall derive executable, property-test, proof, and

    coverage artifacts reproducibly from one validated contract without hiding unsupported semantics,

    vacuity, rejection, or assumptions.'
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  unit_type: section
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Description
  excerpt_start_line: 23
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.


    [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backe'
- id: FR-028
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: context_only
  unit_type: section
  source_head: 5b207149d64f6e69634c3c1afcb60c94bc4a0496
  anchor: Description
  excerpt_start_line: 25
  excerpt: 'The code generator shall bound every Kani harness so that its run can finish: each harness identity

    records a memory ceiling and a wall-clock ceiling, a run that exceeds either settles `inconclusive`

    with its own reason, and the evidence records the bounds and ceilings the run used. Proof coverage

    is maximised per family. A family proves the production code where its harness verifies within the

    ceilings, and a bounded shadow together with a refinement obligation where it does not

    ([ADR-003](../../decisions/ADR-003-kani-tractability.md)).


    Planned (IR-241): ADR-003 decides that a shadow exists '
- id: original474/actual-sampler
  path: /home/peter/dev/worktrees/ir805-code/src/kani/generate/graph.rs
  role: context_only
  unit_type: external_source
  source_head: 41cb3f8038f056bde7b9e666bdd320297c6655e6
  anchor: actual-sampler
  excerpt_start_line: 522
  excerpt: "let mut body = vec![\n        input(\"object_count\".into(), 0, n),\n        input(\"start\".into(),\
    \ 0, n.saturating_sub(1)),\n        input(\"target\".into(), 0, n.saturating_sub(1)),\n    ];\n    assume(\n\
    \        &mut body,\n        \"object_count == 0 || (start < object_count && target < object_count)\".into(),\n\
    \    )?;\n    if lowered.same_operand {\n        assume(&mut body, \"start == target\".into())?;\n    }\n  \
    \  let mut degrees = Vec::new();\n    let mut rows = Vec::new();\n    for row in 0..n {\n        let degree\
    \ = format!(\"degree_{row}\");\n        body.push(input(degree.clone(), 0, upper));\n        assu"
- id: original474/active-slots
  path: /home/peter/dev/worktrees/ir805-code/src/kani/generate/graph.rs
  role: context_only
  unit_type: external_source
  source_head: 41cb3f8038f056bde7b9e666bdd320297c6655e6
  anchor: active-slots
  excerpt_start_line: 536
  excerpt: "for row in 0..n {\n        let degree = format!(\"degree_{row}\");\n        body.push(input(degree.clone(),\
    \ 0, upper));\n        assume(\n            &mut body,\n            format!(\n                \"object_count\
    \ <= {row} || {degree} >= {}\",\n                lowered.edge.lower\n            ),\n        )?;\n        degrees.push(degree.clone());\n\
    \        let mut entries = Vec::new();\n        for column in 0..upper {\n            let name = format!(\"\
    edge_{row}_{column}\");\n            body.push(input(name.clone(), 0, n.saturating_sub(1)));\n            assume(\n\
    \                &mut body,\n                form"
- id: original474/actual-array-helper
  path: /home/peter/dev/worktrees/ir805-code/src/kani/generate/graph.rs
  role: context_only
  unit_type: external_source
  source_head: 41cb3f8038f056bde7b9e666bdd320297c6655e6
  anchor: actual-array-helper
  excerpt_start_line: 589
  excerpt: 'let oracle = format!("fn {helper}(count: i64, degrees: &[i64; {n}], edges: &[[i64; {upper}]; {n}], start:
    i64, target: i64) -> bool {{\nlet mut paths = [[false; {n}]; {n}];\nfor row in 0..{n} {{ for edge in 0..{upper}
    {{ if row < count as usize && edge < degrees[row] as usize {{ paths[row][edges[row][edge] as usize] = true;
    }} }} }}\nfor via in 0..{n} {{ for from in 0..{n} {{ for to in 0..{n} {{ paths[from][to] = paths[from][to] ||
    (paths[from][via] && paths[via][to]); }} }} }}\nlet reachable = if count == 0 {{ false }} else {{ paths[start
    as usize][target as usize] }};\nlet selected_reach'
findings: []
bindings: []
source_ownership: []
```
