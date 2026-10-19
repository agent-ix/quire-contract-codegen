---
id: SR-13294
title: "IR-892 PR 459 code review (with Rust lane): typed routed reader outcomes"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@171768d7dac3204f3d52a175b35e21720f9aa18e vs origin/main merge-base; src/routed/capability.rs, src/oracle/function/mod.rs, src/lib.rs, src/routed/contract_package_tests.rs, tests/it/capability_settlement.rs, tests/exact_function_support/package.rs"
review_set: subset
---

## Summary

Ticket: IR-892. PR agent-ix/quire-contract-codegen#459. The Rust lane is folded into this file. The PR adds `EnvelopeRefusal::ContractPackageReadRefused` and `ContractPackageReadIncomplete` with CG-owned `Serialize` projections, threads a `Result<Disposition, EnvelopeRefusal>` through `negotiate_item`, and collects `negotiate_backend_provider` into a `Result` so the first failing item in request order wins.

Measured by the reviewer at the head in a fresh worktree with its own target dir:

- The seven TC-030 tests for FR-019-AC-48 to AC-52 pass (2 unit, 5 integration).
- `make fmt-check`, `make lint` and `make spec` pass.
- The seven IR-898 tests fail at the head with `MalformedWire` at `/semantic_graph/nodes/*` (with `QSPEC_DIR` set), matching IR-898's named set on main. The author reports Kani 35/35 passed, so there is no sandbox-only Kani failure set to compare.
- There is no other `match` on `EnvelopeRefusal` in this repo. quire-driver was not touched (IR-896).
- Tags are `#[trace(...)]` only. The PR adds no hash, pin, compatibility layer or new panic on untrusted input.

Reviewer production mutants, each run against the seven tests:

- Killed: dropping `path`, `cause`, `expected` or `actual` from the refusal projection; zeroing `limit` or `consumed`; collapsing `Refused` or `Incomplete` back to an item-level `Ok(false)`; mapping `Refused` to an `Incomplete`; reversing item iteration order.
- **Survived:** setting the refusal projection's `locus`, `contract_version` or `document_pointer` to `None`, and setting the incomplete projection's `path` to `None` (FND-003).

## Examined criteria

- FR-019-AC-48 (examined): "A `Refused(refusal)` outcome from the current Kani oracle-function preflight read produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection })` for the whole request, with the reader's `code`, `path`, `cause`, `locus`, `contract_version`, `document_pointer`, `expected` and `actual` copied into the named CG projection; the private `expected_node_id` is absent. No item settlement or artifact is returned."
- FR-019-AC-49 (examined): "An `Incomplete(incomplete)` outcome from the current Kani oracle-function preflight read produces `Err(EnvelopeRefusal::ContractPackageReadIncomplete { incomplete: ContractPackageIncompleteProjection })` for the whole request, with the reader's `limit_kind`, `limit`, `consumed` and `path` copied into the named CG projection. No item settlement or artifact is returned."
- FR-019-AC-50 (examined): "An `Admitted` Kani function-application preflight read whose exact generator produces a claim for an otherwise supported item retains the existing `supported` settlement."
- FR-019-AC-51 (examined): "Among several request items with non-`Admitted` preflight outcomes, the first failing item in manifest/item request order determines the whole-request `ContractPackageReadRefused` or `ContractPackageReadIncomplete` variant and projection, irrespective of outcome kind. Earlier successful items contribute no returned settlement or artifact."
- FR-019-AC-52 (examined): "An `Admitted` Kani function-application read whose exact generator emits no claim, and a structural preflight rejection before any read, each retain item-level `KaniFormRefused { OperationNotRendered }` without a reader envelope refusal."
- FR-019 IR-892 rationale (examined): the named adapters `project_contract_package_refusal` and `project_contract_package_incomplete`, the projection field types, and "The code PR SHALL use that admitted result without a re-read, re-lowering or checked-graph fallback."
- FR-039-AC-5 (examined): "A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source map, stale semantic identity or incomplete reader result reaches no CG consumer and emits no artifact. CG preserves the reader's typed refusal or limit result and performs no second parse or checked-graph fallback."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The projection API differs from FR-019's SHALL text. The spec names the adapters `project_contract_package_refusal` and `project_contract_package_incomplete`, but the code has associated `from_reader` functions. The spec types `path`, `contract_version` and `document_pointer` as `Option<String>`, `locus` as `Option<CheckedNodeId>` and `expected`/`actual` as `Option<CheckedSemanticId>`, but the code uses `Option<JsonPointer>`, `Option<Box<str>>`, `Option<Box<CheckedNodeId>>` and `Option<Box<CheckedSemanticId>>`. The JSON wire is the same, because `JsonPointer` serializes as its text and `Box` is transparent. The public Rust surface, which IR-896's driver code will be written against, is not. Fix: add the two named adapter functions and use the specified field types. If boxing is needed for enum size, amend FR-019 in this PR to state the boxed types and the adapter names. | src/routed/capability.rs:1027-1086 |
| FND-002 | high | `negotiate_contract_package` (the FR-330 consumer) still passes its envelope to `negotiate_backend_provider`. An item whose `kani_form` is `V2FunctionApplication` therefore re-lowers its `CheckedPackageV2` and re-reads the wire (oracle/function/mod.rs:1747-1762), and a `Refused` or `Incomplete` from that second read now surfaces as `ContractNegotiationError::Envelope(ContractPackageRead*)` on the FR-330 path. FR-019 says "On the FR-330 production path, only an `Admitted` package reaches CG ... `Refused` and `Incomplete` stop before negotiation. The code PR SHALL use that admitted result without a re-read, re-lowering or checked-graph fallback." The PR leaves this path unchanged and adds no test of it. Fix: on the `negotiate_contract_package` path, run the function-application check against the admitted `&ContractPackageV1`, or refuse a `V2FunctionApplication` form there as an invalid request. Add a TC-030 test showing that the FR-330 path performs no second read. | src/routed/capability.rs:1477-1484 |
| FND-003 | medium | Four projected fields have no test that can fail. Every reader refusal used by the tests has `locus`, `contract_version` and `document_pointer` set to `None`, and every incomplete result is a byte-ceiling result with `path` set to `None`. Reviewer mutants that hard-code each of these four fields to `None` in `from_reader` pass all seven TC-030 tests. Fix: add adapter cases with each field present: an unknown `contract_version` wire (`contract_version` Some), a node-located refusal (`locus` Some), a selected-document number past 2^53 (`document_pointer` Some), and a count or depth limit (incomplete `path` Some). Assert equality with the direct reader value. | tests/it/capability_settlement.rs:47-118 |
| FND-004 | low | The `///` doc comment for `checked_integer_function_application_is_supported` now sits above the new `FunctionPackageReadFailure` enum. The enum gets the function's doc, and the function has none. Fix: give the enum its own doc line and move the existing comment back above the `fn`. | src/oracle/function/mod.rs:1646-1658 |
| FND-005 | low | The AC-52 test matches `OperationNotRendered { .. }` with `..` for `form` and `backend`. It does not pin `operation_identity` to `quire.op.function.call` or the backend to `kani`, so a wrong operation in the item-level refusal would pass. Fix: assert the full `Cause::KaniFormRefused` value, or at least `operation_identity == "quire.op.function.call"` and `backend == "kani"`. | tests/it/capability_settlement.rs:283-298 |

## Verdict

**FAIL: not mergeable as is.** The core mechanism is sound. The first failing item wins through `Result` collection, item-level `OperationNotRendered` is kept, the wire shape is `{"code":..., "cause":{"refusal"|"incomplete":{...}}}` with explicit nulls, and production mutants for order, kind mapping and the four tested fields fail the tests. FND-001 and FND-002 are spec SHALL violations that must be fixed in code, or settled by an amendment to FR-019 in this PR. FND-003 leaves a third of the projected fields without a test that can fail.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | On the new admitted (FR-330) path, `body_index` is the body's index in the whole `positions` slice, so it counts `ContextRoot` entries. It is passed as a function position, which `check_position` and `PackageItems::driver_node` treat as a driver-item index. When a `ContextRoot` precedes the body's `DriverItem`, the index names the wrong driver node, the generator returns an identity error, and the preflight returns `Ok(false)`. A supported function application then settles as item-level `KaniFormRefused { OperationNotRendered }`. In a reviewer scratch test, `[ContextRoot(function), DriverItem(body), DriverItem(call)]` settles as OperationNotRendered, while `[DriverItem(body), DriverItem(call)]` settles as Supported. Fix: compute the driver index by counting only `DriverItem` positions, then add a TC-030 case with a context root before the body that asserts `Supported`. | src/oracle/function/mod.rs:1763-1771 |

## Dispositions

Disposition pass 1, against head d132db3510170a80379ed5975c8d36cc1419ab00. The reviewer measured the following in a fresh worktree with its own target dir:

- The 12 focused TC-030 tests pass: the original seven plus five new ones.
- `make fmt-check`, `make lint` and `make spec` pass.
- Reviewer mutants that set `locus`, `contract_version`, `document_pointer`, refusal `path` or incomplete `path` to `None` each fail a test.
- Passing `None` instead of the admitted package in `negotiate_contract_package` (the second-read mutant) fails `admitted_function_settlement_does_not_read_again`.
- Reversing item order fails the order and admitted-path tests.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The free functions `project_contract_package_refusal` and `project_contract_package_incomplete` are exported from lib.rs, and `from_reader` is removed. The fields are now `Option<String>`, `Option<CheckedNodeId>` and `Option<CheckedSemanticId>`, as FR-019 states. The wire is unchanged, because the pointers are copied as `as_str()` text |
| FND-002 | fixed | `negotiate_contract_package` calls `negotiate_backend_provider_with_package(envelope, Some((package, positions)))`. The function preflight then uses the admitted `ContractPackageV1` and does not lower or read. A test-only read counter asserts zero reads, and the second-read mutant fails the test |
| FND-003 | fixed | Four new adapter tests cover an unknown `contract_version`, a selected-document `document_pointer` of `/package/count`, a node-located `locus` from a duplicate function name, and a node-ceiling incomplete `path`. Each field-drop mutant now fails its test |
| FND-004 | fixed | The enum has its own doc, `/// Non-admitted result of the internal function-package reader.`, and the original doc is back above `checked_integer_function_application_is_supported` |
| FND-005 | fixed | The AC-52 match now pins `operation_identity == "quire.op.function.call"` and `backend == "kani"` |
| FND-006 | fixed | Disposition pass 2, against head 216b8d9915f3e765a1dd664533b110789ec8679e. `body_index` is now taken over `DriverItem` positions only (`filter_map` drops `ContextRoot`). The new test `admitted_function_settlement_with_context_root_is_supported` puts a `ContextRoot` first and asserts `Supported` from `kani`. The reviewer mutant that counts the `ContextRoot` again fails that test. The 13 focused TC-030 tests, `make fmt-check`, `make lint` and `make spec` pass, and the second-read mutant still fails both admitted-path tests |
