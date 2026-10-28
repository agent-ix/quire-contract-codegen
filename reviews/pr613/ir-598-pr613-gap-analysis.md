---
id: SR-613002
title: 'Gap analysis — IR-598 PR #613 collection corpus retirement'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; PR #613 IR-598 head 72a6c7010ce496a2e835a88835937b887c8daf22;
  base fb50acdc82cd026c442cfa76681e195104b164e6; spec/assurance/AD-004-cg-crate-layout.md, spec/core/functional/interface-001-codegen-api.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md,
  spec/spec.md'
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

Method: quoin gap-analysis (planless; plan completion: not assessed) over the PR diff
and the directly affected criteria, with `quoin matrix --json` (quoin 0.29.1, quire 0.36.2 /
engine 0.50.2) on the exact head. Matrix: FR-015-AC-110 untagged (0 binders); AC-94 5, AC-96 7,
AC-99 6, AC-100 5, AC-101 8, AC-123 1, AC-124 4 binders; every row reports no run evidence.
AC-88/AC-89 lose their lowerer and become vacuous "until" criteria, which AC-110 records as
a by-design coverage loss. AC-52 and AC-93 remain backed by graph-based refusal tests with the
`r1`/`r2` revision context. FR-015-AC-100 is out of scope for change and remains PENDING the
IR-1090 owner ruling; it is not treated as satisfied. The installed Kani lane was not run by this
review, by instruction.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | interface-001, AC-123, spec.md, TC-023, AD-004 still declare the removed collection API live | spec/core/functional/interface-001-codegen-api.md:81-85, 439 |
| FND-002 | medium | FR-015-AC-110 has no tagged test and its real-Kani gate evidence is absent | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 |
| FND-003 | medium | AC-101 tag on a helper-internal test inflates the missing-declaration clause | src/kani/generate/collection.rs:1410-1418 |
| FND-004 | medium | AC-96 tag on an ordered-set refusal test does not back the uniqueness clause | tests/it/kani_obligations_symbolic_domains.rs:1446-1448 |
| FND-005 | low | Corpus proof-graph schema still admits the retired collection family | schemas/kani-corpus-proof-graph-v1.schema.json:35-43 |

## Finding Detail

### FND-001

The PR deletes the public `prepare_bounded_collection_query`, `CollectionQuery`, `CollectionLowering` and `QueryKind` (src/lib.rs), but interface-001 still declares `prepare_bounded_collection_query` a live operation (:81-85, index :439), FR-015-AC-123 still requires external calls to it to have the interface-001 signature (now unsatisfiable as written; its tagged test no longer calls it), spec/spec.md:136 still registers lower/bounded_collections.rs and the function as live, TC-023:31 still lists `Result<CollectionLowering, KaniOutcome>`, and AD-004's layout/move tables still list bounded_collections.rs and QueryKindIdentity. The spec now describes a public API the code no longer has; reconcile it in this PR without changing AC-100.

### FND-002

`quoin matrix` reports FR-015-AC-110 untagged with 0 binders on this head. Its Test half (a collection claim settles only through FR-015-AC-96 to AC-102 with no corpus fallback) has no AC-110-tagged test, unlike the arithmetic slice's FR-015-AC-109-tagged control, and the deleted-name absence is reviewer inspection only. Its gate evidence is also absent: no recorded green `make kani`/`make kani-gate` run on head 72a6c701 is cited in the PR, so AC-110's precondition (AC-99 installed-backend controls green on the slice head) is unmet.

### FND-003

The new FR-015-AC-101 tag on unresolved_projection_declaration_is_field_not_admitted inflates coverage: it calls the internal `admitted_declaration` helper and asserts `CollectionDomainRefusal::FieldNotAdmitted`, not that negotiation settles `unsupported` with `CollectionUnsupportedReason::FieldNotAdmitted` (negotiate.rs:707) and emits no harness. AC-101's own text already rejects a helper-internal test as backing for the missing-or-malformed-declaration clause.

### FND-004

The added FR-015-AC-96 tag on the ordered-set refusal test makes the matrix count AC-96's 'preserve the field's declared order and uniqueness rules' clause as tagged, but the test asserts a refusal with no harness (AC-97/AC-102 behaviour); no harness models uniqueness. AC-96's own text records that 'no AC-96-tagged test backs the uniqueness clause'; the tag now contradicts that note instead of closing it.

### FND-005

The published corpus proof-graph schema still admits the `collection` family, `bounded-collection-query` construct and `collection_` identity/proof-id patterns (and the already-retired arithmetic ones), which no generator can emit after this PR. The schema is deleted at FR-015-AC-111, so this is low, but narrowing it to `graph` now keeps FR-028-AC-1's validation meaningful.

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
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: '- name: prepare_bounded_collection_query status: live finite-input auxiliary under FR-015-AC-88
    and AC-89; deleted by the collection slice (FR-015-AC-110) inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput,
    CollectionQuery] output: Result<CollectionLowering, KaniOutcome> semantics: CG-owned bounded collection-query
    lowering; bound exhaustion and profile or dispatch refusal stay IR typed non-Boolean outcomes with
    no partial artifact'
- id: spec/spec.md
  path: spec/spec.md
  role: examined
  excerpt: '| `kani/generate/lower/{mod.rs,bounded_kani_profile.rs,finite_reference_graphs.rs,bounded_collections.rs}`;
    `classify_bounded_kani_profile`, `prepare_finite_graph_reaches`, `prepare_bounded_collection_query`
    and public types | FR-015-AC-82 to AC-89 and AC-109 to AC-111 | Remaining finite-input auxiliaries
    are live. The collection lowerer retires at AC-110; graph/profile lowerers and module entry retire
    at AC-111. |'
bindings:
- test_id: rendering_refuses_an_unrepresentable_element_domain_without_a_harness
  ac_id: FR-015-AC-101
  trace: correct
- test_id: domain_representability_failures_keep_the_typed_reason
  ac_id: FR-015-AC-101
  trace: correct
- test_id: unresolved_projection_declaration_is_field_not_admitted
  ac_id: FR-015-AC-101
  trace: wrong
- test_id: ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics
  ac_id: FR-015-AC-96
  trace: wrong
- test_id: public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: NFR-002-AC-1
  trace: correct
- test_id: public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes
  ac_id: FR-015-AC-123
  trace: correct
- test_id: direct_corpus_refusals_preserve_earliest_failure_and_identity_transactions
  ac_id: FR-015-AC-124
  trace: correct
- test_id: tc_023_every_input_and_profile_field_changes_the_identity
  ac_id: FR-015-AC-94
  trace: correct
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | AC-101 status note still calls the representability tests untagged | spec/kani/functional/FR-015-bounded-kani-obligations.md:851 |

### FND-006

FR-015-AC-101's status note still says the representational-ceiling clause is 'Not backed by a tagged test' because `domain_representability_failures_keep_the_typed_reason` 'carries no AC tag' and that 'Tagging :1070 is a follow-up'. This PR tags it (collection.rs:1271) and adds `rendering_refuses_an_unrepresentable_element_domain_without_a_harness` for AC-101. The note contradicts the matrix. It is a spec-text refresh for the AC-110 status-flip spec change or IR-1092; this code PR edits no criteria by design.

## Dispositions

Disposition round 1 at exact head `31cddd974579654eaaeed708fb60a6c828d3f888` (fix commit ab6f2eaf, rebased patch-identically onto merged spec-first CG #614, main `92ebb4e2943a47cc172a4237c82077c8b7da9859`; `git range-diff fb50acdc..ab6f2eaf 92ebb4e2..31cddd97` shows both commits `=`). Checked by reading the fix diff and source/spec context and by `quoin matrix`; no Cargo, Kani or full suite was run in this round, by instruction.

NOT MERGEABLE. Acceptance blockers remain: FR-015-AC-100 run evidence is blocked by the IR-1090 owner ruling; the FR-015-AC-110 real-Kani gate (green `make kani`/`make kani-gate` with the AC-99 controls on this head) has not run; IR-1091 (AC-96 uniqueness) and IR-1092 (AC-101 missing declaration) are open `blocks` relations on IR-598.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Resolved by merged spec-first CG #614 (92ebb4e2), not by #613: AC-123, interface-001 (:80-82), spec/spec.md:136, TC-023's procedure and AD-004's 2f inventory now bound the collection operation to 'until FR-015-AC-110'. After retirement AC-123 requires no exported replacement alias or wrapper; none exists (src/lib.rs exports no collection item). The current-state rows flip to retired when AC-110 is recorded implemented after its gate. |
| FND-002 | still-open | Tagging half fixed: `quoin matrix` on 31cddd97 shows FR-015-AC-110 tagged with 3 binders (tc_023_retired_collection_corpus_leaves_only_the_graph_request, tc_023_proof_graph_schema_rejects_retired_corpus_families, collection_claim_uses_the_v2_symbolic_harness_without_a_finite_corpus_request; the last asserts a Supported disposition, one harness, `kani::any` and no `Vec<i128>`). Gate half open: no recorded green `make kani`/`make kani-gate` run with the AC-99 controls on 31cddd97 is cited; the PR body says the installed-Kani gate has not run. |
| FND-003 | fixed | The overclaiming AC-101 tag is removed; the missing/malformed-declaration clause stays partial under IR-1092, a `blocks` relation on IR-598. The pre-existing AC-101 tag on the helper test a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted (collection.rs:1382, already on main fb50acdc) is outside this diff and falls to IR-1092. |
| FND-004 | fixed | AC-96 tag removed; the matrix no longer lists the ordered-set refusal under AC-96, and the uniqueness clause stays partial under IR-1091, a `blocks` relation on IR-598, matching AC-96's own note. |
| FND-005 | fixed | - |
| FND-006 | new (disposition pass 1) | AC-101 status note still calls the representability tests untagged |

### Disposition round 2

Disposition round 2 at exact head `731029ed4fb11f4b9dbacea74a5329e44b6ebb30`, reviewing only the fix diff `31cddd97..731029ed` (one commit, 731029ed: 1 Rust test line, 1 FR-015 status-note row) and its source/spec context. No Cargo, Kani, test suite or full gate was run, by instruction; `quire validate` of FR-015 passes (exit 0) and the spec diff changes only the FR-015-AC-101 row, leaving AC-100 and every other criterion untouched.

Code diff: review-clear; no open code or test finding remains in SR-613001 or SR-613002. Merge readiness: NOT MERGEABLE. The FR-015-AC-110 real-Kani gate has not run on this head, and IR-1090 (AC-100), IR-1091 (AC-96) and IR-1092 (AC-101) remain open `blocks` relations on IR-598.

No new findings in round 2.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | still-open | Unchanged since round 1: AC-110 tags remain (3 binders), but no recorded green `make kani`/`make kani-gate` run with the AC-99 controls on 731029ed is cited; the real-Kani gate remains unrun. This is a merge-gate action, not a code defect. |
| FND-006 | fixed | The AC-101 note now names both AC-101-tagged representability tests and keeps the missing-or-malformed-declaration clause unbacked at the generator boundary (IR-1092). Only the status note changed; the criterion statement is byte-identical. |
