---
id: SR-7021
title: "Gap analysis — IR-629 process backend routing"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@3939583852769ced9b3cb21793a82b260be70f04; src/lib.rs, src/routed/capability.rs, src/routed/generate.rs, tests/it/capability_settlement.rs, tests/it/kani_obligations.rs, tests/it/routed_generation.rs"
review_set: subset
---

## Summary

Ticket: IR-629. Reviewed PR #349 at 3939583852769ced9b3cb21793a82b260be70f04 against 45a56a638a3184bdabe94ecb4c32dc7ab7a6dfd0.

## Verdict

**FAIL** — two changed-scope trace gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed matrix reports FR-019-AC-1 untagged: the dispatch test has only Trace: TC-030. This trace gap predates PR #349, but this PR adds the Process variant whose exhaustive settlement AC-1 now requires, so the new arm still lacks an AC-1 binding. | tests/it/capability_settlement.rs:261 |
| FND-002 | high | The computed matrix reports FR-019-AC-19 untagged. The fixture always creates the same DomainKey, but no test holds that key fixed while changing only explicit ProofBound.kind and checking different outcomes, so deriving a kind from DomainKey would survive this suite. | tests/it/capability_settlement.rs:110 |

## Scope

The following criteria were examined; an omitted finding means the criterion raised no defect in this pass.

- FR-019-AC-1 (`examined`): Settlement dispatches one arm per variant of the closed backend kind, and every variant reaches an arm that settles rather than falling through.
- FR-019-AC-11 (`examined`): PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL` lists finite built-in kinds only, while CG enumerates process candidates from descriptors. CG's descriptor-to-kind conversion returns `Process(id)` exactly when a descriptor has process origin, including identity text `kani`. A named, registered process backend reaches that arm and receives one disposition; no process item disappears for being absent from `ALL`.
- FR-019-AC-12 (`examined`): PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and admit-side enforcement). A bounded process item with an uncovered `extent.bounds[].kind` settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming the offending domain kind and candidate (QSpec FR-290-AC-13). An item that passes the domain check, including one with empty `bounds`, settles `supported` exactly when its descriptor advertises `bounded` for its kind; otherwise it settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`.
- FR-019-AC-13 (`examined`): Settling a process-provider item reaches no plugin: a descriptor whose identity names a non-existent executable settles identically to one with an ordinary identity, apart from the backend each names, and starts nothing, and a descriptor whose identity names an executable that records its own start leaves no record. A mutant arm that starts or resolves the identity as a process either changes the first disposition or leaves the record, and fails this.
- FR-019-AC-16 (`examined`): PLANNED (IR-629). An unbounded process item settles `supported` when its descriptor advertises `unbounded` for its kind, regardless of whether any `extent.domains[].kind` belongs to manifest `domains`.
- FR-019-AC-17 (`examined`): PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item settles `requires-bound` exactly when `finite_bound_available` is true and every boundable `extent.domains[].kind` is in manifest `domains` (QSpec FR-290-AC-13).
- FR-019-AC-18 (`examined`): PLANNED (IR-629). Given the same process advertisements and item extent, changing only backend identity, manifest position, manifest run-limit defaults or ambient state leaves the disposition and cause unchanged apart from the backend named in the output.
- FR-019-AC-19 (`examined`): PLANNED (IR-629; bounded row depends on QSL-654's producer). The process arm reads each explicit `extent.bounds[].kind` and never derives a domain kind from `DomainKey`; two otherwise equal bounded items with the same `DomainKey` and different explicit kinds can settle differently under one manifest `domains` set.
- FR-019-AC-20 (`examined`): PLANNED (IR-629). CG's process descriptor retains the FR-331 manifest's advertised (kind, mode) pairs, `domains` and `bounds` beside the IR-633 `id` and `origin` projection.
- FR-019-AC-21 (`examined`): PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with any unadvertised boundable `extent.domains[].kind` settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability` naming the offending domain kind and candidate, regardless of `finite_bound_available`; non-boundable kinds are not compared (QSpec FR-290-AC-13).
- FR-019-AC-22 (`examined`): PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded item with every boundable `extent.domains[].kind` advertised and the item's `finite_bound_available=false` settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`. CG reads that flag from the item and does not derive it from the domain kinds (QSpec FR-290-AC-13).
- FR-019-AC-23 (`examined`): PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded item's proof-bound numeric maximum leaves its process-provider `supported` or `unsupported` disposition and cause unchanged.
- FR-022-AC-17 (`examined`): PLANNED (IR-629). A routed `Process(id)` item appears exactly once with its own identity and empty `KindOutput::Process`, without a generation context or membership in `BackendKind::ALL`.
- FR-022-AC-18 (`examined`): PLANNED (IR-629). A crate-internal routed `Process(id)` item whose private backend and kind disagree refuses the whole call as `BackendKindDisagrees` with `converted=Some(Process(item.backend.identity))` and no generated output.
- FR-022-AC-19 (`examined`): PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple process identities leaves its request-index-ordered output unchanged.
- FR-022-AC-20 (`examined`): PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private while `request_index` and `node_id` remain public; external callers cannot create an item with a hand-built `Process(id)` or reassign its backend after `from_descriptor` construction.
- FR-022-AC-21 (`examined`): PLANNED (IR-629). `from_descriptor` constructs Kani for a linked `kani` descriptor and `Process(id)` for a process descriptor with the same identity text. An unknown linked identity never produces Process.
- FR-022-AC-22 (`examined`): PLANNED (IR-629). `from_descriptor` rejects a backend identity different from the descriptor identity as `DescriptorBackendMismatch`, naming both; it rejects an unknown linked identity as `UnknownLinkedBackend`, naming the backend. Neither refusal produces a routed item.

## Coverage

- Reconciliation: quoin matrix (quoin 0.28.3, quire 0.36.2), whole repository; changed criteria reviewed individually.
- Plan completion: not assessed
- Whole-repository matrix: tagged 357, untagged 283, tagged-by-ignored-test 11, method-without-symbol 37. Most gaps predate this PR; the two findings identify relevant obligations.
- Evidence: no run evidence in Quoin store for the examined criteria.
- Semantic review: skipped; no opt-in was supplied.
- Reverse gap and stub scan of changed code found no additional defect.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 62d24f831b1e9c55995a35be1ff8377d6a43ba25 |
| FND-002 | fixed | 62d24f831b1e9c55995a35be1ff8377d6a43ba25 |

All recorded findings were rechecked against the fix commit; no finding remains open.
