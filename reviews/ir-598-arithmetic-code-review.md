---
id: SR-12950
title: IR598 arithmetic code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-598 AC109 PR444; spec/assurance/AD-004-cg-crate-layout.md,
  spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/spec.md, src/kani/generate/corpus/bounded_kani_corpus.rs,
  src/kani/generate/lower/definedness_arithmetic.rs, src/kani/generate/lower/mod.rs,
  src/lib.rs, tests/it/bounded_kani_corpus.rs, tests/it/cover_last.rs, tests/it/kani_obligations.rs,
  tests/it/layout.rs, tests/it/routed_generation.rs
review_set: subset
---

## Summary

Ticket IR-598, PR444; code-review of the arithmetic-only AC109 implementation. No new delta findings.

## Verdict

PASS

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Inspected exact lowerer deletion, module/root re-export removal, corpus request/family/identity enum definitions and all associated match/import/checked-method/rendering arms. Exact deleted-symbol searches over src and tests, including unqualified variants, return no matches. No wrapper, new public operation or finite-request-to-V2 adapter appears. Graph/collection corpus, replay, canonical registry, profile/shared errors/admission, census_validation, V1 and published schema remain. Native API/receipt/private helpers and IR848 terminal mapping inputs are unchanged. L2 review includes test modules and qualified crate-root calls: no new generate-to-run/classify/output edge, public HarnessSpec, scanner weakening or root-import dodge.

Canonical PREIMAGE remains independently hand-authored; its expected name uses only the permitted canonical proof-content identity, with the stored digest removed. Adjacent 9007199254740993 and 9007199254740994 are asserted as literal complete canonical bytes in both values and ExistsEqual fields. Ceilings, owner serialization, reordered emission, distinct-content identities, duplicate collisions, required/missing dependencies, duplicate/assumed refusal, empty registry and valid retry remain adverse and discriminating. Request/profile identity variations use an inert admitted collection fixture; no expected oracle is calculated by calling the production operation. Removing canonical fields, rounding values, prematurely claiming identities, or accepting duplicate dependencies would fail these assertions.

The missing-cover control starts with healthy [1,2] ForAllNonNegative and removes only the cover; it requires MissingCoverSummary. The private LIB control separately requires CoverUnsatisfied, and neither accepts generic tool refusal. Explicit admitted div selects Generated + IrConfirmed quire.op.integer.div, then requires OperationNotRendered and zero harnesses. Routed rem independently requires Underived, Refused, NoDerivableClaim(OperationNotDerivable(rem)) and no harness. New AC109 trace ownership is on actual tests; no production/proof/doc-tag inflation or legacy IR865 migration.

Graph/collection QSL compile, schema and real replay/mismatched-package/harness checks remain. Only the retired arithmetic ignored test is removed, honestly lowering the canonical lane from 36 to 35. The cover guard changes only its family inventory; layout changes only the deleted file entry. Rust additions are test assertions, not new fallible production paths, casts, unsafe, async/lock/resource mechanisms or vendored rules. Existing IR858 private constructor/reference/differential/refusal tests, authentic QSL-emitted AC103, real healthy Add/Add-to-Subtract mutation and routed healthy/bound-violation controls are unchanged. There is no claim of four separate backend mutants or exact finite-request replacement.

Coverage loss is intentional under AC109: old finite-input i128 checked add/sub/mul/div/rem and wide-i128 admission/range behavior disappear. In particular ground Rust checked_div/rem exact-result, nonzero-divisor and range checks are retired without inherited QSL division/modulo proof or mutant evidence. Symbolic V2 i64 add/sub/mul/negate serves a different input domain; IR854 inherits no such proof.

Frozen PR444 candidate: eda0168937522b6347d33732e5ae31960519de11; assigned fresh base: 2079a1ef1c77bc44e06be9cd1f629739df1d09b7. Ticket: IR-598. Source/spec/tests/branch stayed read-only. Review artifacts live only in the assigned scratch directory. No applicable AssuranceProfile exists under spec/. No additional agents were used.

Author receipts were inspected, not rerun: focused units 30 passed; focused integration 20 passed/7 ignored after the initial expected arithmetic cover-inventory failure was corrected by removing only its obsolete entry. Format, locked clippy, compile and confined Markdown checks passed. Actual author make kani-gate passed 35/35 on clean 2f9508c486d9b0eb697c2d6edbfe1759f7ac6ea3, Kani 0.68.0, 751.15 test seconds/766 gate seconds. The separate private ignored LIB CoverUnsatisfied control passed 1/1 and requires successful checks plus exact satisfied=0,total=1. Independent diff confirms only AD004/interface001/spec-map Markdown changed afterward; source/tests/build/scripts/schema/embedded Kani inputs are unchanged. No backend PASS is attributed to eda0168. Lead owes distinct final-head full Kani and aggregate make ci after review, fixes and artifact custody.

No compiler/backend probe was needed. Exact-name pgrep guard for cargo|rustc|cbmc|kani-driver|cargo-kani|kani was empty when the reviewer explicitly released its exclusive heavy grant through the harness. Both target directories are retained. IR-598 and IR-348 remain open; AC110/111, held steps 4c–4f/5/6 and IR-854 retain their separate gates.

## Examined units

Full verbatim excerpts follow. Tracker excerpts may be explicitly declared verbatim prefixes; paths are exact repository-relative paths.

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
    fails when a file other than the ones the test drives spells a proof attribute,
    or a driven file spells more proof attributes than the test counts; a spelling
    is `#[kani::` other than `requires`, `ensures`, `stub` and `unwind`, `kani::proof`
    or `proof_for_contract` in one literal, with `\` line continuations joined, which
    includes a `format!` template and a split `concat!` whose first fragment holds
    `#[kani::`, and the scan does not see a spelling assembled from fragments none
    of which holds `#[kani::` or `kani::proof`.
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce
    equal artifact paths and contents regardless of emission order, and reordering
    the same finite graph's objects or references does not change them; changing a
    request field, selected profile, finite input, ceilings or normalized dependency
    census changes the case identity and artifact paths.
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004
    step 4b). After this slice no source under `src/` contains the corpus `Arithmetic`
    request variant and arm, `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`,
    `checked_method`, `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and
    their re-exports, and no test in `tests/` drives them. The slice deletes only
    what the arithmetic family owns: the corpus replay (`BoundedCorpusCase::replay_counterexample`,
    `compile_corpus_qsl_package`, `BoundedCorpusReplayError` and the replay-harness
    identity helpers) stays until FR-015-AC-111, because its only caller replays graph
    and collection cases (an arithmetic corpus case never falsifies, FR-015-AC-57),
    and the arithmetic slice adds no replay test. V2 add, subtract, multiply and negate
    serve the family (FR-015-AC-103, FR-015-AC-115, FR-015-AC-117). V2 does not serve
    divide or remainder, tested: a claim over `quire.op.integer.div` settles `Unsupported(OperationNotRendered)`
    with no harness (the internal `ScalarLoweringRefusal::NoRenderer`), and `quire.op.integer.rem`
    stays the refused catalogue identity with no family. IR-854 owns V2 `div` and
    `mod`. Recorded coverage loss: FR-015-AC-84 and FR-015-AC-85 covered divide and
    remainder only as a Rust `checked_div` and `checked_rem` literal check (exact
    result, zero-division and range refusals) with no mutation control, whose retained
    QSL text uses `/` and `%`; the original IR-598 review inferred from its then-resolved
    `qsl-forms` that `%` was not a QSL operator and that the remainder case''s QSL
    probably did not compile (not executed then and not rechecked by this spec refresh),
    hence the loss is not QSL division coverage and IR-854 inherits none of it. Inspection
    of the no-wrapper rule: the slice adds no `interface-001` operation, and no `src/`
    item converts a finite-input request type (`CheckedArithmeticRequest`, `CollectionQuery`,
    `GraphRequest`, `BoundedCorpusRequest`) into a V2 generator call.'
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock
    ceiling, and changing either ceiling changes the identity.
- id: NFR-005-AC-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Every `.rs` file under `src/`, except a file its parent module declares
    under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments,
    every `#[cfg(test)]` item and every string and character literal removed (`non_test_code_outside_literals`,
    a variant of `non_test_code` that `tests/common/panic_scan.rs` gains), including
    the body of `fn digest` of `impl CaseIdentity` in `src/kani/generate/corpus/bounded_kani_corpus.rs`;
    no `expect` in that file, `render_artifacts` included, is allowed. The tokens
    are the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`,
    `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however written (called,
    with whitespace before the paren, named on a path such as `Option::unwrap`, or
    imported); the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`,
    `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq` and `debug_assert_ne`
    in any delimiter form, with any whitespace before the `!` and any path prefix;
    and the identifier `abort` anywhere except as a method call (`.abort()`). The
    scan walks `src/` itself, so a file added later is scanned. It asserts that it
    read every file that holds a measured site (`src/oracle/function/mod.rs`, `src/kani/generate/scalar.rs`,
    `src/kani/generate/corpus/bounded_kani_corpus.rs`, `src/routed/capability.rs`,
    `src/routed/generate.rs`, `src/evidence/bound_coverage.rs` and `src/oracle/boolean_v1.rs`),
    the three files FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 name, and at least
    60 files.
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder
    returns the exact checked `i128` result only inside the request's inclusive result
    range.
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted result range is `InvalidInput`; zero division, checked overflow
    and a result outside the range are typed refusals, with no arithmetic result or
    generated artifact.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111
    (graph, which deletes the whole public corpus API and the generation-time `KaniOutcome`);
    this criterion is the end state they reach together and adds no deletion of its
    own. The end state is: QSL-emitted packages admitted through Contract IR feed
    the one V2 generator for arithmetic, collection and graph cases; the collection
    and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived
    symbolic bounds, and each proof-eligible admitted case renders through `HarnessSpec`
    and the shared `render.rs` with the cover after every assertion. A case without
    symbolic input emits no proof harness. Neither successful nor refused generation
    returns a `KaniOutcome` or a proof verdict; a successful case returns artifacts,
    and only an FR-017 backend run supplies the verdict. The retirement is staged
    with no compatibility wrapper and no second public generator, and every slice
    is gated on IR-858 (AD-004 step 4b, FR-015-AC-112 to FR-015-AC-117). Until FR-015-AC-109
    to FR-015-AC-111 all hold, FR-015-AC-90 to FR-015-AC-94 remain interim and this
    criterion remains PLANNED.'
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
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR''s head that is green, cited in the PR (the real-Kani tests
    are ignored kani-lane tests, so default CI does not run them). After this slice
    no source under `src/` contains the corpus `Collection` variant and arm, `prepare_bounded_collection_query`,
    `render_collection_qsl` or `lower/bounded_collections.rs` and their re-exports,
    and a collection claim settles only through FR-015-AC-96 to FR-015-AC-102, with
    no corpus fallback. The AC-99 real-Kani collection test is the replacement for
    the corpus collection replay. Recorded coverage loss: FR-015-AC-88 and FR-015-AC-89
    (finite query truth value, examined-item count and the `max_items` `ResourceExhausted`
    bound) have no V2 equivalent by design, since a caller `max_items` cannot stand
    in for the model domain (FR-015-AC-102). Inspection of the no-wrapper rule: the
    slice adds no `interface-001` operation, and no `src/` item converts a finite-input
    request type (`CheckedArithmeticRequest`, `CollectionQuery`, `GraphRequest`, `BoundedCorpusRequest`)
    into a V2 generator call.'
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package
    population emission (after QSL-819)). Graph claims are served by FR-015-AC-98
    and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106''s real-Kani graph replay
    is the replacement for the corpus graph replay; Evidence for a real-Kani precondition
    is a recorded `make kani` or `make kani-gate` run on the slice PR''s head that
    is green, cited in the PR (the real-Kani tests are ignored kani-lane tests, so
    default CI does not run them). After this slice no source or test under `src/`
    or `tests/` contains the corpus `Graph` variant and arm, `prepare_finite_graph_reaches`,
    `lower/finite_reference_graphs.rs`, `lower/bounded_kani_profile.rs` with `classify_bounded_kani_profile`
    and `BoundedKaniProfile` (deleted unconditionally; nothing in `src/` uses them
    except their `lib.rs` re-export), `lower/mod.rs`, `generate_bounded_kani_corpus_case`,
    the `BoundedCorpusRequest`, `BoundedCorpusCase`, `BoundedCorpusError`, `BoundedCorpusArtifacts`,
    `BoundedCorpusFamily`, `CorpusProofDependencyGraph` and `EmittedCorpusIdentities`
    types, `CORPUS_PROOF_GRAPH_SCHEMA` and `schemas/kani-corpus-proof-graph-v1.schema.json`,
    the corpus''s use of the census validation (its calls to `validate_dependencies`
    in the deleted corpus module; `census_validation.rs` itself is not deleted by
    this slice, because `validate_dependencies`, `validate_plain_identity`, `validate_path`,
    `single_diagnostic`, `KaniDiagnostic` and `KaniErrorCode` are also used by `v1_bundle.rs`,
    so no item in that file is corpus-only, and its items go once their last caller
    is gone, per AD-004 step 4f), `kani/generate/corpus/`, `tests/it/bounded_kani_corpus.rs`
    and the corpus replay (`BoundedCorpusCase::replay_counterexample`, `compile_corpus_qsl_package`,
    `BoundedCorpusReplayError`, and the replay-harness identity helpers `ReplayHarnessIdentity`,
    `ReplayPackage::harness_identity` and `replay_harness_counterexample` with its
    `_through` form, which only the corpus uses), together with their only caller
    `tests/it/bounded_kani_corpus.rs`; the crate exports no corpus API. Generation,
    successful or refused, returns no `KaniOutcome` (FR-015-AC-95), and a graph claim
    settles with no caller `max_expansions`. Recorded coverage loss: FR-015-AC-86
    and FR-015-AC-87 (finite sorted depth-first expansion bound) have no V2 equivalent
    by design. A search of `src/` and `tests/` for each deleted name finds nothing.
    Inspection of the no-wrapper rule: the slice adds no `interface-001` operation,
    and no `src/` item converts a finite-input request type (`CheckedArithmeticRequest`,
    `CollectionQuery`, `GraphRequest`, `BoundedCorpusRequest`) into a V2 generator
    call.'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Implemented (IR-858). After the scalar port, for each claim of a fixed
    set (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a
    literal operand; a claim with an inferred operand; a claim with each operand at
    an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path
    and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`,
    and the same digests derived from them, as a test-only reference copy of the pre-port
    `render_scalar` computes in the same test. No stored expectation file and no regenerate
    mode is involved. The port has no explicitly specified difference. The refusals
    `ResourceLimitExceeded` and `InvalidGeneratedSyntax` keep their values and their
    order, now raised by `render` (AD-004 "The HarnessSpec and render contract (step
    4b)", P-2 and P-3).
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port,
    the unmutated scalar harness classifies `Verified` with its cover satisfied and
    the existing scalar mutation control (the oracle's `Add` swapped for `Subtract`,
    FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable
    `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run is
    a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3).
- id: src/kani/generate/corpus/bounded_kani_corpus.rs
  path: src/kani/generate/corpus/bounded_kani_corpus.rs
  role: examined
  excerpt: '//! Integrated bounded-Kani corpus generation over Contract IR''s validated
    finite ABI.

    //!

    //! CG owns the semantic-family lowerers. This module owns the codegen-side

    //! vertical slice: once one lowering is admitted, it renders the corpus roles
    (oracle, strategy,

    //! Kani harness, QSL source and dependency, and proof-dependency graph) from
    the same profile'
- id: src/kani/generate/lower/definedness_arithmetic.rs
  path: src/kani/generate/lower/definedness_arithmetic.rs
  role: examined
  excerpt: "//! CG-owned checked arithmetic admission over IR's validated finite input\
    \ and typed outcome.\n\nuse quire_contract_ir::kani::{\n    DispatchIndex, KaniOutcomeKind,\
    \ KaniProfile, SemanticFamily, ValidatedFiniteInput,\n};"
  revision: assigned base (deleted at candidate)
- id: src/kani/generate/lower/mod.rs
  path: src/kani/generate/lower/mod.rs
  role: examined
  excerpt: '// Bounded collection query admission.

    pub(crate) mod bounded_collections;

    // The bounded-Kani profile and its classification.

    pub(crate) mod bounded_kani_profile;

    // Finite reference graph reachability admission.'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '//! Deterministic Rust, property-test, proof, and evidence generation
    from Quire contracts.


    #![forbid(unsafe_code)]

    #![deny(missing_docs)]

    '
- id: tests/it/bounded_kani_corpus.rs
  path: tests/it/bounded_kani_corpus.rs
  role: examined
  excerpt: '//! Public integration coverage for the bounded Kani corpus API.

    //!

    //! `--harness <name>` below is a substring filter over the fully qualified harness
    name, not an

    //! exact match (`--exact` is not passed), so e.g. `--harness corpus_case_collection`
    matches the

    //! generated `corpus_case_collection_<digest>` symbol. Each temporary crate here
    writes exactly'
- id: tests/it/cover_last.rs
  path: tests/it/cover_last.rs
  role: examined
  excerpt: '//! FR-015-AC-7, AC-53 and AC-58: every harness the generator emits ends
    with exactly one

    //! `kani::cover!`, as the last statement of its body.

    //!

    //! The guard has two halves. The inspection drives every emitting entry point,
    parses each

    //! emitted source with `syn` and checks every function attributed `#[kani::proof]`
    or'
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: '//! FR-015 separate bounded Kani obligations, FR-017 Kani execution, and
    Contract IR FR-036

    //! backend negotiation.

    //!

    //! The default lane checks negotiation, refusal and harness shape without running
    Kani. The

    //! `kani` lane (`make kani`, `#[ignore]` here) measures the installed backend,
    runs real'
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: '//! AD-004 layout decisions L-1 and L-2, checked over the source text
    of `src/`.

    //!

    //! L-1: `src/` holds the eight subsystem directories and `lib.rs`, and every
    module the AD''s

    //! module-to-subsystem map names sits in the directory the map names.

    //!'
- id: tests/it/routed_generation.rs
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: '//! FR-022 routed generation: `generate_routed` runs the routed kind''s
    generation arm keyed by the

    //! driver''s request index, and takes the routed backend and kind as given.

    //!

    //! The fixtures are TC-024/TC-025''s: `kani_obligations::scalar_package` and
    its claim map.

    '
bindings:
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_wide_request_values_are_exact_canonical_strings
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_unencodable_case_identity_refuses_without_claiming
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac1_the_literal_free_scan_keeps_code_and_drops_literals_comments_and_test_items
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: tests/it/no_generation_panics.rs::tc_042_ac1_src_holds_no_panic_token
  ac_id: NFR-005-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_reads_a_corpus_harness_without_its_cover_as_missing_the_cover_summary
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_every_corpus_family_ends_its_harness_with_one_cover_after_its_assertion
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_executes_the_generated_graph_harness
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_verifies_a_true_collection_and_falsifies_a_false_graph_harness
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_kani_falsifies_the_generated_false_collection_harness
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_real_corpus_playback_replays_through_its_qsl_source
  ac_id: FR-015-AC-57
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_cover_inspection_rejects_a_misplaced_duplicated_or_missing_cover
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_proof_spelling_counter_sees_split_formatted_and_continued_attributes
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_the_literal_scan_skips_a_test_item_with_an_array_type
  ac_id: FR-015-AC-58
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_no_proof_attribute_is_spelled_outside_what_the_inspection_drives
  ac_id: FR-015-AC-58
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_case_identity_is_independent_of_emission_order_and_run
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_distinct_requests_get_distinct_names_paths_and_proof_symbols
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_every_graph_request_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_every_collection_request_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_every_input_and_profile_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::tc_023_identity_is_canonical_over_the_input_population_order
  ac_id: FR-015-AC-94
  trace: correct
- test_id: tests/it/kani_obligations.rs::admitted_integer_division_is_unrendered_and_emits_no_harness
  ac_id: FR-015-AC-109
  trace: correct
- test_id: tests/it/routed_generation.rs::tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity
  ac_id: FR-015-AC-109
  trace: correct
- test_id: src/kani/generate/corpus/bounded_kani_corpus.rs::tests::either_ceiling_changes_the_canonical_corpus_identity
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_artifact_validates_against_its_published_schema
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/bounded_kani_corpus.rs::tc_023_proof_graph_with_a_declared_dependency_validates_against_its_published_schema
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/kani_generation.rs::bundle_proof_record_requires_and_records_each_request_ceiling
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/kani_obligations.rs::changing_either_request_ceiling_changes_the_generated_proof_identity
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/kani_obligations_state_frame.rs::both_state_obligation_identities_change_with_either_required_ceiling
  ac_id: FR-028-AC-1
  trace: correct
- test_id: tests/it/routed_generation.rs::routed_scalar_identity_records_both_required_ceilings
  ac_id: FR-028-AC-1
  trace: correct
```
