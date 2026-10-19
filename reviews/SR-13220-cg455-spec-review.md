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
