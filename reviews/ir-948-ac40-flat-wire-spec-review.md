---
id: SR-202611020184
title: "IR-948 AC-40 flat-wire specification review"
type: SpecReview
analysis: base
review_set: base
scope: "agent-ix/quire-contract-codegen; IR-948 AC-40 spec-first diff in FR-015 and TC-025"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Independent base, integrity and scope review of the AC-40 wording correction in `spec/kani/functional/FR-015-bounded-kani-obligations.md` and TC-025 step 11. No applicable `AssuranceProfile` was found under this repository's `spec/` tree. The criterion remains planned; this review does not claim implementation or backend proof.

## Source check

| Source | Checked rule |
| --- | --- |
| QSpec FR-322 Body grammar and AC-39/40, `quire-specification` 68be77f87d726804c176b2e75f4f6cb5cad625a0 | A clause body application takes Member arguments; a nested operator application is refused as `malformed_wire`, and a composite condition is a reference to its own expression node. A Boolean literal is an admitted leaf. |
| QSpec FR-341 State clause body, same revision | The clause's third argument is the Boolean condition term, subject to FR-322's flat grammar. |
| Contract IR `checked_package/v2/flat_wire.rs`, f88eade3dc71bd4338adbe6502464fdaad62c829 | The reader rejects nested applications at their location, matching FR-322. |
| CG FR-014-AC-38 | Its inline-condition branch can consume the admitted Boolean literal; a referenced expression keeps the clause node as oracle identity. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder). | - |

## Verdict

PASS for this scoped spec diff. FR-015-AC-40 and TC-025 step 11 retain the byte-identical clause-identified oracle requirement and distinguish the admitted inline literal from operator applications at referenced node body roots. The change does not weaken FR-015-AC-41's arithmetic probe obligation, alter error classification, or add a hash, pin, copied artifact, new ID or verification method. The status stays PLANNED until the separate code PR supplies public-request evidence.
