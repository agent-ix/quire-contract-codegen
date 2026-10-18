---
id: SR-12870
title: Code review of IR-598 refreshed corpus retirement spec
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; IR-598, PR #423; spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md,
  spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md,
  spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/spec.md, spec/kani/matrix/tests.md'
review_set: subset
---

## Summary

Ticket: IR-598. Independent supplemental code-review of the frozen spec candidate. No new defects found.

## Verdict

**PASS** for spec delivery. Future retirement remains Planned and gated; IR598 code waits on IR858, collection additionally on 4h, and graph/final on the preceding slices and IR805.

## Review scope

```yaml
scope:
- id: FR-015-AC-51
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a finite input validated under a profile selection
    other than the offered profile's is refused as a typed `InvalidInput` `kani_profile_input_mismatch`
    result naming the request's source id, with no artifact and no case identity recorded.
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, the context of every outcome and refusal that
    path returns (generated-case `Proved` or `Counterexample`, `kani_corpus_dependency_invalid`,
    `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`)
    is the profile selection's revision. The generated-case values are not Kani run
    verdicts.
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic,
    FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness
    of that family ends with exactly one `kani::cover!` after its `assert!` of the
    case's oracle, which witnesses that the harness runs to its end past that assertion.
- id: FR-015-AC-57
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed
    backend, a bounded-corpus harness of each of the arithmetic, graph and collection
    families whose oracle is true classifies `Verified`, and a graph or collection
    harness whose oracle is false (an arithmetic case's oracle is always true) classifies
    `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes
    the corpus replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`),
    never `Inconclusive` for lack of a counterexample.
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test generates a harness through every emitting entry point (the precondition,
    V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bundle`,
    and the corpus generator for each of its families whose arm remains, per FR-015-AC-109
    to FR-015-AC-111), parses each emitted source, and fails for any function attributed
    `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain
    exactly one `kani::cover!`, which must be the last statement, with no assertion
    after it and no other cover; and a scan of the non-test string literals of `src/`
    fails when a file othe
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in request order, preserving each selected profile disposition (`Supported`, `Refused`
    or `Inconclusive`).
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder
    returns the exact checked `i128` result only inside the request's inclusive result
    range.
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted result range is `InvalidInput`; zero division, checked overflow
    and a result outside the range are typed refusals, with no arithmetic result or
    generated artifact.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a positive-length path in sorted depth-first order, and checks the expansion
    bound before entering each new object; a target edge is found without expanding
    its target.
- id: FR-015-AC-87
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the
    bound is `ResourceExhausted`, with no graph result or generated artifact.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains input order and duplicates and returns its truth value and examined-item
    count; a decisive item ends that count, otherwise every item counts.
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, an input longer than `max_items` is `ResourceExhausted`, with no query
    result or generated artifact.
- id: FR-015-AC-90
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic,
    graph or collection case emits all four artifacts (oracle, finite strategy, Kani
    harness and proof-dependency graph) from the same selected profile, validated
    finite input and family lowering.
- id: FR-015-AC-91
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, that path returns a generation-time case classification
    of `Proved` when the family lowering's Boolean oracle is true and `Counterexample`
    when false; neither value is an observed Kani run or native replay verdict. This
    interim classification ends when FR-015-AC-111 deletes the path.
- id: FR-015-AC-92
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), a declared dependency
    census with an empty or repeated identity, inconsistent kind/state/path or a kind
    other than `Required` refuses as typed `InvalidInput` with no artifact or case-identity
    claim; a valid census is normalized into the emitted proof graph.
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a profile/input mismatch, family-lowering
    refusal, proof-graph serialization refusal or duplicate case identity returns
    its typed refusal with the selected profile revision, emits no artifact and claims
    no new case identity.
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce
    equal artifact paths and contents regardless of emission order, and reordering
    the same finite graph's objects or references does not change them; changing a
    request field, selected profile, finite input, ceilings or normalized dependency
    census changes the case identity and artifact paths.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111
    (graph, which deletes the whole public corpus API and the generation-time `KaniOutcome`);
    this criterion is the end state they reach together and adds no deletion of its
    own. The end state is: QSL-emitted packages admitted through Contract IR feed
    the one V2 generator for arithmetic, collection and graph cases; the collection
    and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived
    symbolic bounds, and each'
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-598 arithmetic slice; gated on IR-858, AD-004 step 4b, merged
    first, because the V2 scalar path must already render through `HarnessSpec` and
    `render.rs` for the deletion to leave the family served). After this slice no
    source under `src/` contains the corpus `Arithmetic` request variant and arm,
    `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`, `checked_method`,
    `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and their re-exports,
    and no test in `tests/` drives them. The slice deletes only what the arithmetic
    family owns: the corpus replay (`Bo'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR's head that is green
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package
    population emission (after QSL-819)). Graph claims are served by FR-015-AC-98
    and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay
    is the replacement for the corpus graph replay; Evidence for a real-Kani precondition
    is a recorded `make kani` or `make kani-gate` run on the slice PR's head that
    is green, cited in the PR (the real-Kani
- id: FR-015-retirement-description
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Current interim ownership (IR-329): the bounded-profile classifier, the
    three

    finite-input semantic-family lowerers and the bounded-corpus emitter are auxiliary

    operations of this Kani generator, governed today by FR-015-AC-82 to

    FR-015-AC-94 and, at the step 4g boundary, by the planned per-family retirement

    slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111

    (graph and the whole corpus API), with FR-015-AC-95 as their end state.'
- id: FR-015-retirement-gates
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'FR-015-AC-96 to FR-015-AC-102 retain the collection implementation and
    partial

    statuses recorded below; graph clauses remain PLANNED. This retirement plan

    adds no proof credit or current backend results.

    FR-015-AC-109 to FR-015-AC-111 state the slices and FR-015-AC-95 their end

    state. QSL-353 establishes that

    QSL-emitted packages read through Contract IR; it does not supply the symbolic

    corpus or the graph population input. The graph slice also waits for IR-805: CG
    integration of the merged IR-851/IR-852

    accessors and QSL-820 real-package population emission (after QSL-819). Retirement
    of each'
- id: AD-001-corpus-replay
  path: spec/assurance/AD-001-codegen-architecture.md
  role: examined
  excerpt: "retains no counterexample packet. Its replay method (`BoundedCorpusCase::replay_counterexample`,\n\
    \  with `compile_corpus_qsl_package` in `src/replay/function.rs`) is live today,\
    \ serves only its\n  graph and collection cases, and is dropped, not re-homed,\
    \ by the final retirement slice\n  (FR-015-AC-111); the real-Kani replays of FR-015-AC-99\
    \ and FR-015-AC-106 replace it."
- id: AD-004-last-caller
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "it until 4g (`bounded_kani_corpus.rs`), so 4f leaves it in place. The\
    \ items of\n     `census_validation.rs` (`validate_dependencies`, `KaniDiagnostic`,\
    \ `KaniErrorCode`) are removed once their last caller is gone: the V1 bundle path\n\
    \     (removed at 4f) and the corpus (removed at 4g), or earlier when the V2 census\
    \ input replaces them. It is not in `kani/census.rs`"
- id: AD-004-4g
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- 4g: the corpus, retired per family in three slices with no compatibility\
    \ wrapper and no second\n     public generator (IR-598, FR-015-AC-109 to AC-111);\
    \ step 4b (IR-858) gates every slice. The\n     arithmetic slice deletes the `Arithmetic`\
    \ arm and `definedness_arithmetic.rs`; V2 does not\n     serve divide or remainder\
    \ (div settles `Unsupported(OperationNotRendered)`, rem stays a\n     refused\
    \ identity) until IR-854, which also owns `mod`. The collection slice, gated also\
    \ on\n     step 4h, deletes the `Collection` arm and `bounded_collections.rs`.\
    \ The graph slice, gated\n     on IR-805, deletes t"
- id: interface-001-corpus
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'status: live interim four-artifact corpus under FR-015-AC-51 to AC-58
    and AC-90 to AC-94; native replay of corpus cases is dropped by the final slice
    (FR-015-AC-111), not planned; AD-004 step 4g retires the arms per family (arithmetic
    FR-015-AC-109, collection FR-015-AC-110, the latter also after step 4h) and the
    graph slice FR-015-AC-111 removes this public entry, the BoundedCorpus types,
    the corpus replay and its generation-time KaniOutcome, each slice only after IR-858
    (step 4b) and after the one V2 generator serves that family''s QSL-emitted, IR-admitted
    cases under FR-015-AC-95 to AC-102, '
- id: TC-023-retirement
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'Retirement procedure (IR-598), per slice, only after IR-858 (step 4b)
    has landed. Arithmetic (FR-015-AC-109): require a div claim to settle `Unsupported(OperationNotRendered)`
    with no harness and a rem claim to stay a refused identity; search `src/` and
    `tests/` for each deleted arithmetic name, and confirm the corpus replay is still
    present; confirm the AC-84/85 divide and remainder loss is recorded as a Rust
    checked-arithmetic literal check and points to IR-854. Collection (FR-015-AC-110):
    require step 4h landed and a recorded `make kani` or `make kani-gate` run on the
    slice PR''s head with t'
- id: FR-024-corpus-status
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: "- The bounded-Kani corpus retains no counterexample packet, so no corpus\
    \ counterexample reaches\n  QSL as the `Input` arm and FR-024-AC-5 is unbacked.\
    \ The corpus path is to be removed, not\n  built on: FR-015-AC-109 to FR-015-AC-111\
    \ plan its deletion per family, and FR-015-AC-111 drops\n  its replay, so FR-024-AC-5\
    \ stays unbacked."
- id: spec-kani-ownership
  path: spec/spec.md
  role: examined
  excerpt: '| `kani/generate/lower/{bounded_kani_profile,definedness_arithmetic,finite_reference_graphs,bounded_collections}`
    and their four public classifier/lowering operations | FR-015-AC-82 to AC-89 |
    Live finite-input auxiliaries, deleted per slice: `definedness_arithmetic` at
    AC-109, `bounded_collections` at AC-110, `finite_reference_graphs`, `bounded_kani_profile`
    and `lower/mod.rs` at AC-111. |

    | `kani/generate/corpus/bounded_kani_corpus` and `generate_bounded_kani_corpus_case`
    | FR-015-AC-51 to AC-58 and AC-90 to AC-95 and AC-109 to AC-111 | Live four-artifact
    finite-input corpus; retired per fam'
- id: matrix-retirement
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-015 | FR-015-AC-82 through FR-015-AC-95, FR-015-AC-109 through FR-015-AC-111
    | TC-023 | 🚧 Planned; the current classifier, lowerers and corpus emitter have
    no criterion trace tags for AC-82 through AC-94; AC-95''s end state and the per-family
    retirement slices AC-109 to AC-111 (IR-598, each gated on IR-858; AC-111 also
    on IR-805; AC-109 records the divide/remainder loss, IR-854) are not implemented
    |'
- id: interface-001-classify_bounded_kani_profile
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "- name: classify_bounded_kani_profile\n    status: live finite-input auxiliary\
    \ under FR-015-AC-82 and AC-83; deleted by the final retirement slice (FR-015-AC-111,\
    \ AD-004 step 4g)"
- id: interface-001-prepare_checked_arithmetic
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "- name: prepare_checked_arithmetic\n    status: live finite-input auxiliary\
    \ under FR-015-AC-84 and AC-85; deleted by the arithmetic slice (FR-015-AC-109)"
- id: interface-001-prepare_bounded_collection_query
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "- name: prepare_bounded_collection_query\n    status: live finite-input\
    \ auxiliary under FR-015-AC-88 and AC-89; deleted by the collection slice (FR-015-AC-110)"
- id: interface-001-prepare_finite_graph_reaches
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: "- name: prepare_finite_graph_reaches\n    status: live finite-input auxiliary\
    \ under FR-015-AC-86 and AC-87; deleted by the graph slice (FR-015-AC-111)"
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall`
    or `quire.op.collection.exists` operation over a selected model collection field
    is a collection claim. For a field with typed inclusive cardinality `[minimum,
    maximum]` and supported finite elements, the generator emits a harness with symbolic
    length and symbolic elements. Its assumptions admit both cardinality endpoints
    and every length between them; for `IntRange` elements they admit both inclusive
    integer endpoints and every integer between them, and for `Boolean` elements they
    admit both `false` and `true
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: For an admitted collection claim whose selected field has multiplicity
    lower bound `0` and no finite upper bound, QSL admits an unbounded collection
    type; when its element type is supported, this generator settles `requires_bound`
    naming the selected field or unbounded collection type and emits no harness. An
    admitted field with no finite element domain likewise settles `requires_bound`
    naming that element type and emits no harness. A field with multiplicity lower
    bound greater than `0` and no finite upper bound has no QSL kernel type ([QSL
    FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-05
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
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
  role: context_only
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
  role: context_only
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
  role: context_only
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing or malformed selected declaration likewise refuses
    with its typed reason and no harness. For a graph claim, a population or edge
    domain whose symbolic object or edge count exceeds the ceiling likewise settles
    `unsupported` naming the domain, with no harness and no reduced population or
    multiplicity. PARTIAL (IR-268, CG #396 fe2'
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply
    the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG
    #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_negotiation_accounts_for_each_supported_or_refus'
- id: FR-015-AC-103
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0,
    20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x:
    Int[0, 9]` into `Int[-10, -1]` each settle `Supported` with an IR-confirmed outer
    oracle and a harness. Their respective argument ranges are `[1, 10]` and `[1,
    1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the
    outer declared result bounds. Each inferred range carries its source application
    id in the harness identity.'
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with
    the Integer `unbounded_type` and no harness; a direct reference to a raw application
    never borrows a separate narrowing conversion's domain, and an inferred endpoint
    outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming
    both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact
    scalar refusal, including its node, position or limit fields, and no harness.
    Refused items leave a suppor
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
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
  role: context_only
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
  role: context_only
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
  role: context_only
  excerpt: A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
    or caller `max_expansions` substitute. While the population selection for the
    claim is `Unavailable` (no available population selection for this admitted claim),
    the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`,
    that is a claim whose population has no finite authored bound (once a selection
    is available), settles `requires_bound` naming the population; an unbounded `Collection`
    edg
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness:
    a spec with no cover or more than one cover; a cover whose condition is the literal
    `true`; no `Call` or more than one; an `Input` or `Assume` after the `Call`; an
    `Assert` before it; a binding name bound twice (other than an input''s own rebinding);
    a name, `result` or `callee` that is not a valid Rust identifier; an input domain
    with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`;
    a message '
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-858). For every spec that constructs, `render` emits the header,
    the oracle source, the module wrapper and the proof attribute, then the spec''s
    statements in spec order, then its one cover as the last statement of the function:
    after every assumption, the subject call and every assertion, whatever order a
    family listed them in. A hand-built spec in each of several statement orders renders
    its cover last; a cover is not a field of `body` and no statement order moves
    it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7).'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: PLANNED (IR-858). `render` of one spec returns equal bytes on repeated
    calls and in a second process, and a spec whose `Input` list is reordered renders
    the reordered source and not the same bytes (AD-004 "The HarnessSpec and render
    contract (step 4b)", R-1).
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: PLANNED (IR-858). After the scalar port, for each claim of a fixed set
    (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal
    operand; a claim with an inferred operand; a claim with each operand at an `i64`
    extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`; claims
    at two ceilings values) the ported path renders the same artifact (path and source
    bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`, and the
    same digests derived from them, as a test-only reference copy of the pre-port
    `render_scalar` computes in the same test. No s
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-858). `render` holds no family-specific template text: for
    hand-built specs whose label, expressions and messages are not the scalar family''s,
    the rendered source consists of the fixed header, the spec''s `oracle_source`,
    the module wrapper, the proof attribute, the spec''s statements rendered from
    their typed fields, and the one cover, and contains no other text; and the ported
    scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives
    `render.rs` for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1
    item 3).'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: PLANNED (IR-858). With the installed backend, after the scalar port, the
    unmutated scalar harness classifies `Verified` with its cover satisfied and the
    existing scalar mutation control (the oracle's `Add` swapped for `Subtract`, FR-015-AC-37)
    classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable `Assume`,
    rendered by `render`, does not classify `Verified` (L-4). This run is a smoke
    over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3).
```

## Method and evidence

The frozen candidate changes Markdown only. No source, test, manifest, workflow or source-embedded input changes. No Cargo, Kani or measurements ran; no current proof result is claimed. No applicable AssuranceProfile exists in spec/. The language-independent checks covered duplication/ownership, integrity, spec-code faithfulness and test alignment for the changed lifecycle contracts. CG refers to upstream accessor/population authority; it does not copy a producer type or declare another generator. Source inspection confirms the current corpus replay exercises graph/collection, V1 bundle uses every named census-validation helper, div settles public Unsupported(OperationNotRendered), and rem has no derivable family. Arithmetic QSL text uses / and %, but the original non-compilation inference is historical and was not rerun. Existing collection implementation is distinguished from unimplemented retirement. No gate, warning, test or coverage threshold changes.

`quire matrix --scope . --strict` reports 337 untagged and 14 ignored-only criteria across the repository; AC109..111 are untagged as explicitly Planned, not claimed delivered. This is a spec-delivery review, not acceptance of future implementation. The changed AC55 remains tagged and AC57 ignored-only; no proof credit was promoted. No changed test↔AC binding was evaluated, so bindings is empty. The immutable original review records remain historical records under the dispatch's explicit custody instruction.

Rust review and production gap-analysis are inapplicable: no Rust or production code changed. Python/React lanes are inapplicable. Existing base, integrity, scope-boundary and evidence methods are refreshed in their original SR12200..12203 files. No relationship edges or domain type definitions changed; dependency/object analyses add no scoped check. No new security boundary, failure behavior, performance demand, architectural choice or property-test/fuzz obligation was introduced; security/failure-domain/risk/architecture/criterion-strength/correctness/fuzz methods are not newly required. This does not accept any held implementation step.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
