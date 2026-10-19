---
id: "SR-13220"
title: "CG PR 455 spec review: typed routed reader outcomes (FR-019-AC-48 to AC-50)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@cfa6dcf6bf52f886e780e8234bffa249b8bd44b9; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md, spec/core/functional/FR-039-contract-package-consumption.md, spec/core/matrix/tests.md, spec/tests.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---

# SR-13220: CG PR 455 spec review

## Summary

Ticket: IR-892. PR: agent-ix/quire-contract-codegen#455 at cfa6dcf. Spec-only.

The PR adds FR-019-AC-48 to AC-50, an Outputs bullet and two paragraphs to FR-019,
a TC-030 procedure paragraph, matrix rows, and a sentence to FR-039-AC-5's Analysis.

Sources checked at the PR head:

- Reader call: `src/oracle/function/mod.rs:1745-1752`. `checked_integer_function_application_is_supported`
  returns `bool` and drops `Refused`/`Incomplete` with `_ => return false`.
- Caller chain, all private: `KaniForm::preflight` (`Result<KaniFormReadiness, Box<Cause>>`,
  `src/routed/capability.rs:867-884`), then `negotiate_kani`, `negotiate_arm`,
  `negotiate_single_candidate` and `negotiate_item` (each returns `Disposition`), then
  `negotiate_backend_provider` (`Result<Vec<ItemSettlement>, EnvelopeRefusal>`, line 1288).
  `negotiate_contract_package` wraps it in `ContractNegotiationError::Envelope`.
  `EnvelopeRefusal` is therefore a reachable carrier for the public result. Only private
  signatures must change, so the claim that `negotiate_backend_provider` keeps its result type holds.
- Reader types: quire-contract-model at Cargo.lock 299a126, plus IR origin/main 4213b98.
  `CheckedPackageRefusal` and `CheckedPackageIncomplete` derive `Clone, Debug, Eq, PartialEq`,
  with no `Serialize`. `CheckedPackageRefusal` also has public `expected` and `actual` fields
  and a private `expected_node_id` field.
- Refused reachability: `validate_dependencies` refuses `MissingImport`/`MissingSelection`
  for any `lock.dependency_selections` entry absent from the empty `ContractDependencies`.
  The TC-030 Refused route is therefore drivable. A wrong-version wire is not drivable
  through the public entry, because the bytes come from CG's own lowering.
- quire-driver origin/main: `src/refusal.rs:250` (`code`) and `:543-549` (`envelope_message`)
  match `EnvelopeRefusal` exhaustively. The driver pins CG at edade59, which predates
  `ManifestRepeatsBackendIdentity`. QSL origin/main has no `EnvelopeRefusal` match.

## Verdict

CHANGES REQUESTED: one high and four medium findings, plus one low.

The carrier choice is sound and the ids are next-free (FR-019 highest on main is AC-47, and
no open CG PR uses AC-48 to AC-50). The three criteria can fail. FR-039-AC-5's statement is
unchanged, and its Analysis still names the deviation.

The high finding is that the stated `Serialize` surface cannot compile as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-019 paragraph says the existing `Serialize` surface of `EnvelopeRefusal` stays available for both new variants, "with the reader's typed fields represented directly". `EnvelopeRefusal` derives `Serialize`, but `CheckedPackageRefusal` and `CheckedPackageIncomplete` do not implement `Serialize`, either at the locked quire-contract-model 299a126 or at IR main 4213b98. `CheckedPackageRefusal` also has a private `expected_node_id` field. The variants as specified (`{ refusal: CheckedPackageRefusal }`, `{ incomplete: CheckedPackageIncomplete }`) therefore do not compile under the derive. The spec records no quire-contract-ir change to add `Serialize`, and no CG adapter with a defined wire shape | spec/routed/functional/FR-019-capability-settlement.md:420-422; src/routed/capability.rs:954-956 |
| FND-002 | medium | The FR-019 paragraph lists `CheckedPackageRefusal`'s fields as `code`, `cause`, `path`, `locus`, `contract_version` and `document_pointer`. At the locked rev, the struct also has public `expected` and `actual` (digest-mismatch identities) and a private `expected_node_id`. FR-019-AC-48's "optional context" does not name them. The serialized shape and the driver rendering would follow an incomplete field set | spec/routed/functional/FR-019-capability-settlement.md:409-413; spec/routed/functional/FR-019-capability-settlement.md:464 |
| FND-003 | medium | AC-48 and AC-49 specify outcomes of the preflight's re-lowering and re-read from `CheckedPackageV2`. On the FR-330 production path (`negotiate_contract_package` to `negotiate_backend_provider` to `KaniForm::V2FunctionApplication { package: CheckedPackageV2 }`), that re-read is the second parse or checked-graph fallback that FR-039-AC-5 says CG does not perform. The spec should state whether AC-48 and AC-49 apply only outside FR-330, or whether the code PR removes the re-read on that path, where these outcomes could not arise | spec/routed/functional/FR-019-capability-settlement.md:464-465; spec/core/functional/FR-039-contract-package-consumption.md:188; src/routed/capability.rs:1365-1372 |
| FND-004 | medium | When several items' preflight reads fail (or one Refused and one Incomplete), no criterion says which refusal the whole-request result carries. Compare FR-019-AC-46's bytewise-first rule for repeated identities. Two implementers could disagree: first in request order, Refused before Incomplete, or something else | spec/routed/functional/FR-019-capability-settlement.md:464-465 |
| FND-005 | medium | AC-50 covers only Admitted plus a generated claim, which yields `supported`. No criterion keeps the item-level `KaniFormRefused { OperationNotRendered }` for the cases that are genuine backend refusals: an Admitted read whose exact generator produces no claim, and the structural `return false` checks before the read. TC-030 tests that control, but nothing requires it. An implementation that promotes every `false` to an envelope refusal passes AC-48 to AC-50 | spec/routed/functional/FR-019-capability-settlement.md:466; spec/routed/matrix/TC-030-capability-settlement.md:50-51 |
| FND-006 | low | The paired quire-driver record names the matches to extend, but not the FR-272 code to report for the two reader outcomes, unlike the `runtime_invariant` guidance given for `ManifestRepeatsBackendIdentity`. It also does not note that the driver's CG pin (edade59) predates `ManifestRepeatsBackendIdentity`. At the bump, the driver's exhaustive matches must therefore cover three new variants, while IR-894 tracks only one | spec/routed/functional/FR-019-capability-settlement.md:414-418 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | AC-48 and AC-49 add a second trigger, "or the caller's single FR-330 read where available". On the FR-330 path, `negotiate_contract_package` accepts only an admitted `ContractPackageV1`, and FR-039-AC-5 says a refused or incomplete reader result reaches no CG consumer. A `Refused` or `Incomplete` outcome from that caller read therefore never reaches CG, so no test can drive this trigger. "Where available" is not defined, and an implementer could read the clause as a request to route refused caller reads into negotiation, which contradicts FR-039-AC-5. TC-030's FR-330 step only checks that there is no second read. Either drop the disjunct or state that on the FR-330 path only `Admitted` reaches CG and the requirement is only the single-read rule | spec/routed/functional/FR-019-capability-settlement.md:513-514; spec/core/functional/FR-039-contract-package-consumption.md:188; src/routed/capability.rs:1365-1372 |

## Dispositions

Round 1, reviewed at b1712d769910cc57dfed050e7b9886508e44b0e7 (fix commit d365b00b over cfa6dcf6). I checked the reader types at quire-contract-model 299a126 (the locked rev). The code, cause and limit-kind enums, `CheckedNodeId` and `CheckedSemanticId` all derive `Serialize`. `JsonPointer::as_str` exists. `EnvelopeRefusal` is `#[serde(rename_all = "snake_case", tag = "code", content = "cause")]`, so the documented wire shape is right. `make spec` exits 0, and `quire validate` on the six changed spec files reports no warnings for them. The diff adds no hash, digest or pin. Driver PR quire-driver#83 (IR-894, Done) merged at 13:55Z. It pins CG fb380d29, which contains `ManifestRepeatsBackendIdentity`, and maps only that variant.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The variants now carry CG-owned `ContractPackageRefusalProjection` and `ContractPackageIncompleteProjection` built by named adapters. Every projected field type is `Serialize` at 299a126, and pointers become `String` through `JsonPointer::as_str`, so the `EnvelopeRefusal: Serialize` derive is satisfiable |
| FND-002 | fixed | The field list now matches the locked struct: `code`, `path`, `cause`, `locus`, `contract_version`, `document_pointer`, `expected` and `actual`. It names the private `expected_node_id` and its getter, and states that it is not projected. AC-48 lists all eight fields |
| FND-003 | fixed | FR-019 and the FR-039 Analysis now say that AC-48 and AC-49 describe the current oracle-function helper. On the FR-330 path the code PR must carry the caller's single read and must not re-read, re-lower or fall back to the checked graph. TC-030 instruments that path. A residual wording issue in the AC text is FND-007 |
| FND-004 | fixed | New FR-019-AC-51 and a SHALL paragraph pick the first failing item in request order, whatever its outcome kind, and give the reason it differs from AC-46. TC-030 covers both mixed orders and a preceding valid item |
| FND-005 | fixed | New FR-019-AC-52 and a SHALL paragraph keep item-level `KaniFormRefused { OperationNotRendered }` for an admitted read with no claim and for a structural rejection before the read. TC-030 asserts both cases |
| FND-006 | still-open | The fix leaves the FR-272 code choice to the driver owner, which is acceptable. It also says the driver change "tracked by IR-894 covers three variants" and that "the driver pin predates all three variants". Both claims are now false. IR-894 is Done, and its merged quire-driver#83 maps only `ManifestRepeatsBackendIdentity` and pins CG fb380d29, which already contains that variant. No open ticket tracks the driver's two reader variants. Name a new follow-up ticket for those two variants and drop the IR-894 and three-variant wording |

Round 2, reviewed at 09796e8a (fix commit 09796e8a over b1712d76). `make spec` exits 0. `quire validate` on the changed spec files is clean. `quire matrix` still lists AC-48 to AC-52 as `untagged`, which matches Planned. The round's spec diff adds no hash or pin, and the spec no longer cites IR-894.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-006 | fixed | The driver paragraph now names IR-896 (Backlog) for `ContractPackageReadRefused` and `ContractPackageReadIncomplete` and leaves the FR-272 code to the driver owner. The false IR-894 three-variant claim and the "pin predates all three" sentence are removed |
| FND-007 | fixed | AC-48 and AC-49 drop the "caller's single FR-330 read where available" disjunct. FR-019, FR-039's Analysis and TC-030 now say that on the FR-330 path only an `Admitted` package reaches CG, `Refused` and `Incomplete` stop before negotiation, and there is no re-read or fallback |
