---
id: "SR-1367"
title: "CG PR 255 spec review (integrity): NFR-005, TC-042, FR-022, interface-001 and matrix consistency"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@3b08c1752f8209a46ea2f3605a769a1916f5a025; spec/core/non-functional/NFR-005-no-generation-panics.md, spec/core/matrix/TC-042-no-generation-panics.md, spec/core/matrix/tests.md, spec/tests.md, spec/routed/functional/FR-022-routed-generation.md, spec/core/functional/interface-001-codegen-api.md (diff origin/main...HEAD, base 3d13550)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: references
---

# SR-1367: CG PR 255 spec review (integrity)

## Summary

Ticket: IR-577. I checked how the six changed files fit together.

- interface-001 `generate_routed` output names `KaniDuplicatePositionOutOfRange{first_index usize,
  items usize} (planned, IR-577)`. FR-022's new If/then bullet and NFR-005's behavior row and
  NFR-005-AC-6 use the same name and fields. The semantics clause of interface-001 adds the same
  condition.
- The FR-022 bullet traces to NFR-005-AC-6. This follows the precedent of the
  `KaniRecordCountMismatch` bullet, which traces to NFR-005-AC-5, so FR-022 needs no AC of its own.
- The core matrix splits NFR-005 into a ✅ row for AC-1 to AC-5 and a 🚧 row for AC-6 to AC-8. The
  TC-042 row lists all eight ACs with a mixed 🚧 status. `spec/tests.md` names the new planned
  ACs.
- TC-042 steps 6, 7 and 8 map one to one onto NFR-005-AC-6, AC-7 and AC-8, and each is marked
  `PLANNED (IR-577)`. The Measurement table adds rows for both new metrics.
- No number is reused. origin/main has no NFR-005-AC-6, AC-7 or AC-8 (`git log -S`), and no TC
  number was minted.
- NFR-005's Constraints paragraph now says the change adds one public variant and no refusal
  code. This matches FR-022 and interface-001.
- `make spec` passes. `quire coverage --strict` reports 66 unbacked rows, unchanged from main.

TC-042 Expected Results says the remaining sites are "guarded inside their own function". That
is a false claim about the code, recorded as SR-1366 FND-001, not an inconsistency between files.

## Verdict

The files agree with each other. Clean for integrity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
