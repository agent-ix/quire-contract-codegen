---
id: SR-12952
title: IR598 arithmetic spec-review/base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; IR-598 AC109 PR444; spec/assurance/AD-004-cg-crate-layout.md,
  spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/spec.md, src/kani/generate/corpus/bounded_kani_corpus.rs,
  src/kani/generate/lower/definedness_arithmetic.rs, src/kani/generate/lower/mod.rs,
  src/lib.rs, tests/it/bounded_kani_corpus.rs, tests/it/cover_last.rs, tests/it/kani_obligations.rs,
  tests/it/layout.rs, tests/it/routed_generation.rs
review_set: subset
---

## Summary

Ticket IR-598, PR444; spec-review/base of the arithmetic-only AC109 implementation. One low status synchronization finding.

## Verdict

CONDITIONAL

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015 still describes three finite-input lowerers and AC109 as planned in its current interim ownership paragraph, although this PR deletes the arithmetic lowerer and marks AC109 implemented. Update that current-state paragraph to distinguish the two retained lowerers and remaining planned slices. | spec/kani/functional/FR-015-bounded-kani-obligations.md:73-78 |

## Coverage

Reviewed the five actual Markdown changes against IDs, links, contract consistency, retirement scope, verification methods, current API inventory and the six coverage rules. No new requirement statement, user story, option, schema, relationship edge or state transition is authored. AC109's deletion/refusal contract remains unchanged apart from implementation status. Interface001 removes only prepare_checked_arithmetic; the feature inventory agrees. Spec-map and AD004 progress note preserve all partial-retirement gates. TC023 honestly records ground arithmetic coverage loss, retained controls and backend limits. The canonical-content exception is respected; no new stored digest/tool pin is introduced. Existing corpus canonical identities are allowed by repository instructions.

One low current-state documentation finding remains in the adjacent owning Description: it still says there are three current lowerers and treats the arithmetic slice as planned. This is newly stale after the deletion and should be synchronized with the changed AC109 status; it does not request a new specification or product decision. Existing unrelated stale HarnessSpec historical prose is not duplicated as a new PR finding.

Frozen PR444 candidate: eda0168937522b6347d33732e5ae31960519de11; assigned fresh base: 2079a1ef1c77bc44e06be9cd1f629739df1d09b7. Ticket: IR-598. Source/spec/tests/branch stayed read-only. Review artifacts live only in the assigned scratch directory. No applicable AssuranceProfile exists under spec/. No additional agents were used.

Author receipts were inspected, not rerun: focused units 30 passed; focused integration 20 passed/7 ignored after the initial expected arithmetic cover-inventory failure was corrected by removing only its obsolete entry. Format, locked clippy, compile and confined Markdown checks passed. Actual author make kani-gate passed 35/35 on clean 2f9508c486d9b0eb697c2d6edbfe1759f7ac6ea3, Kani 0.68.0, 751.15 test seconds/766 gate seconds. The separate private ignored LIB CoverUnsatisfied control passed 1/1 and requires successful checks plus exact satisfied=0,total=1. Independent diff confirms only AD004/interface001/spec-map Markdown changed afterward; source/tests/build/scripts/schema/embedded Kani inputs are unchanged. No backend PASS is attributed to eda0168. Lead owes distinct final-head full Kani and aggregate make ci after review, fixes and artifact custody.

No compiler/backend probe was needed. Exact-name pgrep guard for cargo|rustc|cbmc|kani-driver|cargo-kani|kani was empty when the reviewer explicitly released its exclusive heavy grant through the harness. Both target directories are retained. IR-598 and IR-348 remain open; AC110/111, held steps 4c–4f/5/6 and IR-854 retain their separate gates.

## Examined units

Full verbatim excerpts follow. Tracker excerpts may be explicitly declared verbatim prefixes; paths are exact repository-relative paths.

```yaml
scope:
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
- id: TC-023-retirement-status
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'Arithmetic retirement status (IR-598, FR-015-AC-109 only): the finite-request

    arithmetic API, corpus arm and renderer are deleted. Shared canonical-preimage,

    dependency, refusal, collision, emission-order, ceiling and exact wide `i128`

    checks use retained collection requests. Graph/collection source-backed replay,

    cover controls, schemas, census validation and V1 generation remain. The admitted

    `quire.op.integer.div` test requires `Unsupported(OperationNotRendered)` with
    no

    harness; the separate `quire.op.integer.rem` test requires an `Underived`, refused

    claim with no generated scalar family or harness.


    Coverage retired from FR-015-AC-84/85 includes finite-input `i128` checked

    add/subtract/multiply/divide/remainder admission and range/refusal behavior,

    including requests outside `i64`. Divide/remainder had only ground Rust

    `checked_div`/`checked_rem` checks for exact result, nonzero divisor and range,

    with no mutation control. Their retained QSL text used `/` and `%`; this slice

    claims no QSL division/modulo proof and passes none to IR-854. V2 supports symbolic

    `i64` add/subtract/multiply/negate with independent `i128` assertions; it does
    not

    replace the old finite-request API. IR-858''s differential reference renderer,

    authentic QSL-emitted AC-103 inputs, real healthy Add/Add-to-Subtract mutant and

    routed healthy/bound-violation controls remain; this does not claim dedicated

    real Subtract/Multiply/Negate mutants.'
- id: TC-023-future-gates
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: IR-598 and IR-348 remain open.
- id: interface-001-retired-operation-status
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: 'IR-598 / FR-015-AC-109 retired the finite-input checked-arithmetic auxiliary
    and

    its request/result types. The collection and graph auxiliaries and corpus API

    remain public until their separately gated retirement slices.'
- id: spec-current-lowerer-map
  path: spec/spec.md
  role: examined
  excerpt: '| `kani/generate/lower/{bounded_kani_profile,finite_reference_graphs,bounded_collections}`
    and their three public classifier/lowering operations | FR-015-AC-82 to AC-89
    | Arithmetic auxiliary retired by AC-109; remaining live finite-input auxiliaries
    retire per slice: `bounded_collections` at AC-110, `finite_reference_graphs`,
    `bounded_kani_profile` and `lower/mod.rs` at AC-111. |

    | `kani/generate/corpus/bounded_kani_corpus` and `generate_bounded_kani_corpus_case`
    | FR-015-AC-51 to AC-58 and AC-90 to AC-95 and AC-109 to AC-111 | Live graph/collection
    finite-input corpus; arithmetic retired at AC-109, with collection AC-110 and
    graph/the whole API AC-111 still pending under AD-004 step 4g (end state AC-95),
    each slice gated on IR-858, and the graph slice on IR-805; its generated-case
    classification ends with the last slice, and its replay is dropped with the final
    slice (AC-111), not planned. |'
- id: AD-004-arithmetic-progress
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "     Arithmetic retirement (FR-015-AC-109) is implemented. The collection\
    \ and graph\n     corpus arms, replay and shared auxiliaries remain; this does\
    \ not complete step 4g."
- id: FR-015-current-interim-ownership
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Current interim ownership (IR-329): the bounded-profile classifier, the
    three

    finite-input semantic-family lowerers and the bounded-corpus emitter are auxiliary

    operations of this Kani generator, governed today by FR-015-AC-82 to

    FR-015-AC-94 and, at the step 4g boundary, by the planned per-family retirement

    slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111

    (graph and the whole corpus API), with FR-015-AC-95 as their end state.'
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
bindings: []
```
