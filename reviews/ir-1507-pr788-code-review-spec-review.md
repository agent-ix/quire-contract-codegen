---
id: SR-788003
title: "IR-1507 PR #788 spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #788 at 12d9a54c; IR-1507; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-48), spec/kani/matrix/TC-025-bounded-kani-obligations.md (step 14), spec/kani/matrix/tests.md (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1507. The spec edits move FR-015-AC-48 from PLANNED to "Implemented by IR-1507 for the scalar V2 clause route and the whole-object collection clause route", and name `tests/it/v2_clause_obligation_identity.rs` as the AC-48 evidence in TC-025 step 14. Validation at the head is clean: `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with the same 6 warnings as origin/main, so no new warnings, and `quire matrix` shows AC-48 `tagged`. This review also rules on the author's five unresolved spec gaps.

Rulings on the five author gaps:

1. Occurrence structure: (ii) acceptable implementer choice, with a (iii) clarification follow-up (FND-004). The author compared (node id, role, ordinal) against the clause's source-map entries and ignored the span.
2. Parameter identities: (i) fix in this PR (FND-001 here; SR-788001 FND-001).
3. Domain of a sequence-valued state read: (iii) follow-up (FND-002), with a whole-object test required in this PR (SR-788002 FND-002).
4. NodeKey against the clause node: (iii) follow-up (FND-003).
5. Mismatch as a per-item Unsupported result rather than rejecting the request: (ii) acceptable. AC-48 says "refuses", the sibling refusals of FR-015-AC-43 are per-item, and request rejection is reserved for malformed requests. No finding.

## Verdict

**CHANGES REQUIRED.** The TC-025 step 14 text is accurate for the scalar route. The AC-48 row overclaims (FND-005), and "parameter identities" must be defined before the code can be judged against it (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "Parameter identities" is undefined, and the implementation filled the gap with CG-internal names: `parameter_<name>` on the scalar route and the bare name on the collection route. Neither is a producer-issued identity; QSL keys declared domains by the parameter's `NodeKey` (`DomainKey::Node`). Edit AC-48 in this PR: "a parameter is identified by its checked declaration node id (equivalently its declared name on the clause's operation), never by a generated identifier; a state-field read is compared on its declaring node id, field name and inclusive domain only". Code consequence: compare those members on both routes and drop `identifier`/`slot` from the comparison (SR-788001 FND-001). | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 |
| FND-002 | medium | The domain of a sequence-valued state read in a whole-object collection clause is undefined. The author used the inclusive cardinality range and treats every drawn object field as a read. That drops the element domain, so a claim whose element domain differs (for example `Int[0,10]` against `Int[0,11]`) is not refused, although AC-48 requires read domains to be compared. Follow-up: file a spec ticket so that AC-48 (or AC-39) states the read domain of a sequence field (cardinality and element domain) and whether an unread drawn field counts as a read. Changing the code waits on that ticket; the test for the current choice belongs in this PR (SR-788002 FND-002). | spec/kani/functional/FR-015-bounded-kani-obligations.md:860; src/kani/generate/collection.rs:621-663 |
| FND-003 | medium | AC-48 is silent on whether the carried `ObligationIdentity`'s `NodeKey` must denote the selected clause node, and no CG mapping from `NodeKey` to `CheckedNodeId` is specified. The test helper builds one ad hoc in `tests/common/v2_claim.rs`. As written, a claim carrying another obligation's identity with a correct record is carried onto this harness, so the evidence is attributed to the wrong obligation. Follow-up: file a spec ticket to decide either "the record is the sole binding and NodeKey is opaque to CG" (state it in AC-48) or a specified node-binding check with its own refusal member. | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 |
| FND-004 | low | "Complete source occurrence" does not name its members. The author compared `RequirementOccurrence` (node id, role, ordinal) against any of the clause's source-map entries and ignored the span. That is consistent with the AC listing the span apart from the occurrence, but a reader could take "complete" to include the span. Follow-up (can be folded into the FND-002/003 ticket): "the occurrence compared is (node id, occurrence role, ordinal); the span is not compared". | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 |
| FND-005 | medium | The AC-48 status "Implemented by IR-1507 for the scalar V2 clause route and the whole-object collection clause route" overclaims. The collection route has no AC-48 test (SR-788002 FND-002), span-only invariance is unevidenced (SR-788002 FND-003), and argument and read mismatches are decided after rendering, not "before harness generation" (SR-788001 FND-002). Either close those in this PR or mark the row PARTIAL and name what remains. In the same edit, update `spec/kani/matrix/tests.md:46`, which still lists "IR-1507 for AC-48" under Planned. | spec/kani/functional/FR-015-bounded-kani-obligations.md:860; spec/kani/matrix/tests.md:46 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 | A V2 clause claim carries QSL's producer-issued structural `ObligationIdentity` unchanged: the obligation node's `NodeKey`, optional owning `ModelIdentity`, and producer-assigned ordinal for that node/model pair (AD-003 E-1). ... CG compares the complete source occurrence, obligation kind, parameter identities and declared domains, and any state-field read identities and domains ... Implemented by IR-1507 for the scalar V2 clause route and the whole-object collection clause route (specified by IR-489). |
| TC-025-step-14 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:547-571 | AC-48 evidence is `tests/it/v2_clause_obligation_identity.rs`: the carried identity equals the supplied one for a changed node, model and ordinal; unwind and subject changes leave it unchanged while the harness identity changes; and a changed occurrence, kind, parameter identity or domain, or state-field read identity or domain refuses with the named record member and no harness, while the matching control is admitted. |
| FR-015-AC-47 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:859 | Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). |
| FR-015-statement-AC-48 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:504-507 | The generator shall carry the QSL producer-issued structural `ObligationIdentity` of a V2 clause claim unchanged, compare its source occurrence, kind and argument identities and domains separately on the obligation record, and keep it distinct from the harness identity record (AD-003 E-1; FR-015-AC-48). |
| tests.md-FR-015-V2 | context_only | spec/kani/matrix/tests.md:46 | FR-015-AC-38 through FR-015-AC-43, FR-015-AC-46 through FR-015-AC-49 ... Planned/partial V2 clause-family work (... IR-1507 for AC-48 ...) |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head b1fa48c8: AC-48 now says that a parameter is identified by its checked declaration node id (equivalently its declared name), never by a generated identifier, and that a state-field read is compared on its declaring node id, field name and inclusive domain only. The code matches. |
| FND-002 | deferred | IR-1515 (Backlog) covers the read domain of a sequence-valued field and whether an unread drawn field counts as a read. The AC-48 row names the current cardinality-range choice as open under IR-1515, and the whole-object test pins it. |
| FND-003 | deferred | IR-1515 covers whether the carried NodeKey must denote the selected clause node. The AC-48 row says that the code does not check it. |
| FND-004 | fixed | Round 1: AC-48 now reads 'The occurrence compared is its node id, occurrence role and ordinal; the source span is not compared.' |
| FND-005 | fixed | Round 1: the AC-48 status is now PARTIAL. It names what is implemented and tested on both routes and what stays open under IR-1515. tests.md:46 no longer lists AC-48 as Planned. |
