---
id: SR-613001
title: 'Code and Rust review — IR-598 PR #613 collection corpus retirement'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; PR #613 IR-598 head 72a6c7010ce496a2e835a88835937b887c8daf22;
  base fb50acdc82cd026c442cfa76681e195104b164e6; spec/assurance/AD-004-cg-crate-layout.md, spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/collection.rs, src/kani/generate/corpus/bounded_kani_corpus.rs,
  src/kani/generate/lower/bounded_collections.rs (deleted), src/kani/generate/lower/mod.rs, src/lib.rs,
  tests/it/bounded_kani_corpus.rs, tests/it/kani_obligations_symbolic_domains.rs, tests/it/layout.rs'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

Ticket: IR-598 (IR-348 AD-004 step 4g, collection slice). CG draft PR #613, frozen head
`72a6c7010ce496a2e835a88835937b887c8daf22`, compared with origin/main `fb50acdc82cd026c442cfa76681e195104b164e6` (merge base equals origin/main). Independent
Opus reviewer pass; reviewer-only, no edits to the author branch.

## Verdict

CHANGES REQUESTED, and not mergeable regardless: IR-598 is formally blocked on FR-015-AC-100 (backend version/option evidence pending owner ruling under IR-1090; not treated as satisfied), and FR-015-AC-110's gate (green installed-Kani AC-99 controls recorded on this head) has not run.

What holds: the diff removes the corpus `Collection` request variant and arm,
`BoundedCorpusFamily::BoundedCollection`, `RequestIdentity::Collection`/`QueryKindIdentity`,
`render_collection_qsl`, `prepare_bounded_collection_query`, `lower/bounded_collections.rs` and
their `lib.rs` re-exports; `grep` over `src/`, `tests/`, `scripts/` and `Cargo.toml` finds none of
those names (only the unrelated routed `KaniFormFamily::BoundedCollection` classifier and the
schema enum of FND-005 in the gap-analysis artifact). Graph preservation: the `Graph` variant,
`prepare_finite_graph_reaches`, identity, proof graph, replay and the FR-015-AC-55/57/58 guards
are kept and every converted test now drives a graph case; the corpus `false_cases` still yields
two distinct false graph cases. No-wrapper / no-second-generator: no `interface-001` operation is
added, and no `src/` item converts `GraphRequest`/`BoundedCorpusRequest` into a V2 generator
call. No unsafe, panics, conversions, async, locking or compatibility layer is introduced; the
change is deletion plus test retargeting, and the removed public items are a prerelease breaking
change by design.

Reviewer-run focused checks on the exact head (no Kani, no full suite, FR-359-AC-9 not run):
`cargo test --locked --lib -- kani::generate::corpus kani::generate::lower kani::generate::collection`
36 passed; `cargo test --locked --test it -- bounded_kani_corpus:: ordered_set_shape_is_refused layout::`
16 passed, 7 Kani-lane ignored. The PR body's focused-check claims are data and were not relied on.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Identity test no longer varies an object identity (AC-94 finite-input clause) | src/kani/generate/corpus/bounded_kani_corpus.rs:1559-1621 |
| FND-002 | medium | Hand-authored canonical preimage test deleted with no graph replacement | src/kani/generate/corpus/bounded_kani_corpus.rs:868-908 |
| FND-003 | medium | Compact-identity test's final assertion (digest length 64) is vacuous | src/kani/generate/corpus/bounded_kani_corpus.rs:868-908 |
| FND-004 | low | Single-element loops and a split `use super` import left after removal | src/kani/generate/corpus/bounded_kani_corpus.rs:709, 1067-1074 |

## Finding Detail

### FND-001

tc_023_every_input_and_profile_field_changes_the_identity dropped its `object identity` edit when the base request moved from collection to graph (renaming object b breaks the a->b endpoint), and no replacement varies an object identity, so FR-015-AC-94's finite-input clause no longer proves that object identity is bound into the live graph case identity. A variant that renames a third, non-endpoint object (or the two-object fixture plus request endpoints together) would restore it.

### FND-002

tc_023_case_name_matches_hand_authored_canonical_preimage was deleted with no graph equivalent. It was the only test binding a corpus case name to a hand-authored RFC 8785 preimage independent of the generator and encoder; the remaining tc_023_canonical_preimage_uses_owner_serializers compares against serde output only. The case-name digest is the content identity the repo keeps, and the graph arm still emits it until FR-015-AC-111, so a struct-order or key-order regression would now pass.

### FND-003

tc_023_compact_case_identity_fits_a_verbose_owner_input no longer emits a case: it ends on `identity.digest().unwrap().len() == 64`, which any successful digest satisfies, so the doc comment's 'Admission is decided by the final bytes' is untested. The pre-PR version emitted the case through generate_bounded_kani_corpus_case and asserted admission; the graph replacement should do the same (an unreachable 000000->000001 request still admits as a false case) or the claim should be dropped from the doc comment.

### FND-004

Removing the collection arm left one-element collections iterated by loops (`let cases = vec![...]; for request in cases` at bounded_kani_corpus.rs:1067-1074, tests/it/bounded_kani_corpus.rs:589-596 and guard_sources :554) and a separate `use super::GraphRequest;` beside the existing `use super::{...}` group (:709). Behaviour is unchanged; inline the single case and merge the import.

## Examined Scope

```yaml
scope:
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and on AD-004 step 4h, because
    the collection family renders through `HarnessSpec` and `render.rs` only after 4h). The gate opens
    when the AC-99 installed-backend controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice
    PR's head that is gree…
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic inputs drawn
    from the selected model field's typed domain, ends with the FR-015-AC-7 cover after its assertions,
    and never replaces those inputs with the finite-input corpus's concrete `Vec<i128>`. The independent
    native oracle replays the same authored function through `qsl_replay::replay`; it does not substitute
    a separately authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani `Verified`
    with its cover satisfied; a seeded claim or subject mutant yields `Falsified` with a concrete in-do…
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: For the collection family, changing the selected field's authored cardinality or element bound,
    admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs
    regenerate byte-identically. The identity records every symbolic collection domain and the solver
    and complete ordered option vector, and FR-017 run evidence records the actual Kani, Rust and solver
    versions, unwind and every option used by the installed backend. For the graph family (AC-98), changing
    the claim's clause id or argument path, the reachability form, relationship or field, its role or
    na…
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall` or `quire.op.collection.exists`
    operation over a selected model collection field is a collection claim. For a field with typed inclusive
    cardinality `[minimum, maximum]` and supported finite elements, the generator emits a harness with
    symbolic length and symbolic elements. Its assumptions admit both cardinality endpoints and every
    length between them; for `IntRange` elements they admit both inclusive integer endpoints and every
    integer between them, and for `Boolean` elements they admit both `false` and `tru…
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational or resource
    ceiling settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness;
    it never tightens the model domain to make a proof tractable. A missing or malformed selected declaration
    likewise refuses with its typed reason and no harness. For a graph claim, a population or edge domain
    whose symbolic object or edge count exceeds the ceiling likewise settles `unsupported` naming the
    domain, with no harness and no reduced population or multiplicity. PARTIAL (IR-268, CG #396 fe…'
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), equal case inputs produce equal artifact paths and contents regardless
    of emission order, and reordering the same finite graph's objects or references does not change them;
    changing a request field, selected profile, finite input, ceilings or normalized dependency census
    changes the case identity and artifact paths.
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches`
    and `generate_bounded_kani_corpus_case` have the direct result signatures specified by `interface-001`,
    with `KaniOutcome` as their error type and no public `FamilyLoweringError`, `BoundedCorpusError`,
    `OutcomeConstruction` branch, or replacement alias or wrapper.
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, the context of every successful `BoundedCorpusExpectation` and typed refusal that path returns
    (`kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`)
    is the profile selection's revision. The expectation is not a Kani run verdict.
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, a profile/input mismatch, family-lowering refusal, proof-graph serialization refusal or duplicate
    case identity returns its typed refusal with the selected profile revision, emits no artifact and
    claims no new case identity.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, a profile/dispatch-admitted
    `ForAllNonNegative` or `ExistsEqual` query retains input order and duplicates and returns its truth
    value and examined-item count; a decisive item ends that count, otherwise every item counts.
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, an input
    longer than `max_items` is `ResourceExhausted`, with no query result or generated artifact.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three per-family slices FR-015-AC-109
    (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111 (graph, which deletes the whole public
    corpus API, its generation-time expectation and typed refusal outcome); this criterion is the end
    state they reach together and adds no deletion of its own. The end state is: QSL-emitted packages
    admitted through Contract IR feed the one V2 generator for arithmetic, collection and graph cases;
    the collection and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived
    sym…'
- id: TC-023
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: Verify the current bounded-profile classifier, graph and collection finite-input lowerers and
    corpus emitter against FR-015-AC-82 through FR-015-AC-95, FR-015-AC-123/AC-124, and the retirement
    slices FR-015-AC-109 to FR-015-AC-111.
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '4g: the corpus, retired per family in three slices with no compatibility wrapper and no second
    public generator (IR-598, FR-015-AC-109 to AC-111); step 4b (IR-858) gates every slice. ... The collection
    slice can proceed after completed step 4h, and deletes the `Collection` arm and `bounded_collections.rs`
    only when its own symbolic replacement and control obligations pass.'
- id: src/kani/generate/collection.rs
  path: src/kani/generate/collection.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: src/kani/generate/corpus/bounded_kani_corpus.rs
  path: src/kani/generate/corpus/bounded_kani_corpus.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: src/kani/generate/lower/bounded_collections.rs (deleted)
  path: src/kani/generate/lower/bounded_collections.rs (deleted)
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: src/kani/generate/lower/mod.rs
  path: src/kani/generate/lower/mod.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: tests/it/bounded_kani_corpus.rs
  path: tests/it/bounded_kani_corpus.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: tests/it/kani_obligations_symbolic_domains.rs
  path: tests/it/kani_obligations_symbolic_domains.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: 'PR #613 diff fb50acdc..72a6c701'
bindings:
- test_id: tc_023_every_input_and_profile_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
- test_id: tc_023_graph_corpus_ends_its_harness_with_one_cover_after_its_assertion
  ac_id: FR-015-AC-55
  trace: correct
- test_id: tc_023_kani_falsifies_a_false_graph_harness
  ac_id: FR-015-AC-57
  trace: correct
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Irrefutable one-arm `match` trips clippy `infallible_destructuring_match` under `make lint` | src/kani/generate/corpus/bounded_kani_corpus.rs:742-747 |

### FND-005

The new AC-110 unit test destructures the now single-variant BoundedCorpusRequest with `let graph = match request { BoundedCorpusRequest::Graph(graph) => graph };`. That is the exact shape of clippy's default-warn `style` lint `infallible_destructuring_match`, and `make lint` runs `cargo clippy --all-targets -- -D warnings`, which compiles this `#[cfg(test)]` module. The PR's scoped clippy (`--lib --test it`) does not build lib unit tests, so it would not see it. Not executed here (Cargo was out of scope this round). Use `let BoundedCorpusRequest::Graph(graph) = request;`, which keeps the compile-time closed-family guarantee the test wants.

## Dispositions

Disposition round 1 at exact head `31cddd974579654eaaeed708fb60a6c828d3f888` (fix commit ab6f2eaf, rebased patch-identically onto merged spec-first CG #614, main `92ebb4e2943a47cc172a4237c82077c8b7da9859`; `git range-diff fb50acdc..ab6f2eaf 92ebb4e2..31cddd97` shows both commits `=`). Checked by reading the fix diff and source/spec context and by `quoin matrix`; no Cargo, Kani or full suite was run in this round, by instruction.

NOT MERGEABLE. Acceptance blockers remain: FR-015-AC-100 run evidence is blocked by the IR-1090 owner ruling; the FR-015-AC-110 real-Kani gate (green `make kani`/`make kani-gate` with the AC-99 controls on this head) has not run; IR-1091 (AC-96 uniqueness) and IR-1092 (AC-101 missing declaration) are open `blocks` relations on IR-598.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | new (disposition pass 1) | Irrefutable one-arm `match` trips clippy `infallible_destructuring_match` under `make lint` |

### Disposition round 2

Disposition round 2 at exact head `731029ed4fb11f4b9dbacea74a5329e44b6ebb30`, reviewing only the fix diff `31cddd97..731029ed` (one commit, 731029ed: 1 Rust test line, 1 FR-015 status-note row) and its source/spec context. No Cargo, Kani, test suite or full gate was run, by instruction; `quire validate` of FR-015 passes (exit 0) and the spec diff changes only the FR-015-AC-101 row, leaving AC-100 and every other criterion untouched.

Code diff: review-clear; no open code or test finding remains in SR-613001 or SR-613002. Merge readiness: NOT MERGEABLE. The FR-015-AC-110 real-Kani gate has not run on this head, and IR-1090 (AC-100), IR-1091 (AC-96) and IR-1092 (AC-101) remain open `blocks` relations on IR-598.

No new findings in round 2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | Irrefutable `let BoundedCorpusRequest::Graph(graph) = request;` replaces the one-arm match. It compiles only while the enum has exactly one variant (no `#[non_exhaustive]` at :66-67), so the closed-family guarantee is kept. Lint not executed this round. |
