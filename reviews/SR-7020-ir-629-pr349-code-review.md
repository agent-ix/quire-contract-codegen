---
id: SR-7020
title: "Code review — IR-629 process backend routing"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@3939583852769ced9b3cb21793a82b260be70f04; src/lib.rs, src/routed/capability.rs, src/routed/generate.rs, tests/it/capability_settlement.rs, tests/it/kani_obligations.rs, tests/it/routed_generation.rs"
review_set: subset
---

## Summary

Ticket: IR-629. Reviewed PR #349 at 3939583852769ced9b3cb21793a82b260be70f04 against 45a56a638a3184bdabe94ecb4c32dc7ab7a6dfd0.

## Verdict

**CONDITIONAL** — three test-evidence defects; no source behavior defect established.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The test tagged FR-019-AC-16 keeps a bounded request throughout, so it cannot detect a regression in the unbounded advertisement path. The unbounded assertion at lines 235-241 belongs to a different test without that tag. | tests/it/capability_settlement.rs:117 |
| FND-002 | medium | The test tagged FR-019-AC-23 uses only unbounded requests and never varies a proof-bound numeric maximum. The bounded maximum comparison is asserted by the preceding test, which lacks the AC-23 tag. | tests/it/capability_settlement.rs:190 |
| FND-003 | medium | The test uses only a nonexistent executable path. It never provides a real executable that records a start, so settlement could spawn a real plugin and this test would still pass. TC-046 step 4 and AC-13 require the start-recording control. | tests/it/capability_settlement.rs:244 |

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

- Rust review covered all six changed files, including Kani regression paths.
- Targeted process tests: 4 passed, 1 unrelated ignored; `make fmt-check` passed.
- Terra Kani receipt on this exact head: 27/27 passed; independently executing Kani was outside this review.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 62d24f831b1e9c55995a35be1ff8377d6a43ba25 |
| FND-002 | fixed | 62d24f831b1e9c55995a35be1ff8377d6a43ba25 |
| FND-003 | fixed | 62d24f831b1e9c55995a35be1ff8377d6a43ba25 |

All recorded findings were rechecked against the fix commit; no finding remains open.
