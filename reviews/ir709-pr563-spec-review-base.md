---
id: SR-17009
title: "IR-709 CG #563 spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#563; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md; spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md; spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-709. This is a spec review of the sequencing correction at exact head `97fc8c14d542399fed103a9acd1da7006404f7d1`. The review measured three things independently.

- **The IR-497 separation holds.** Linear IR-497 covers only RT's depth redesign: an explicit call-frame stack replacing RT `MAX_CALL_DEPTH`, iterative `ValueType`/`CollectionType`, and work-unit fuel. CG's `src/` and `tests/` contain no `CallDepthExceeded` reference. quire-exact FR-369's Status gates only the kernel enum removal on the IR-497 cutover. CG producer work does not depend on it.
- **The typed payload is already supplied.** CG's locked quire-exact (`72b254219fd6`, `src/outcome.rs`) exports `CheckedInvariantCause` with all nine `Generated*` variants. `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` carry `{ cause: IllTypedCause }`. `Refusal::CheckedInvariant` requires the payload. The removed text, "the dependency bump that supplies the typed payload", is therefore stale.
- **The AC and verification scope are unchanged.** FR-042-AC-1's text and its `Test` verification method are unchanged. So are the TC-057 procedure (all nine causes, nested `IllTypedCause`, charge/failure boundary, no unit constructor or invented default) and the Expected Results. Both FR-042 and TC-057 stay PLANNED, and no implementation completion is claimed.

## Examined units

- FR-042-AC-1 (examined, unchanged).
- FR-042 `## Status` (examined).
- TC-057 `## Description`, `## Test Procedure`, `## Expected Results`, `## Status` (examined).
- tests.md FR-042 and TC-057 rows (examined).
- quire-exact FR-369 second cause table and Status, origin/main `ab59015` (context_only).
- Linear IR-497 description (context_only, untrusted data).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-042 Status says the generated constructors and unit-constructor removal remain open, but CG main already emits typed causes for seven of the nine CG causes against a kernel whose `CheckedInvariant` requires the payload; the open work is the missing two producers, FR-018 AC-8/AC-17 text, and tagged TC-057 verification | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:41 |

## Verdict

**PASS WITH FINDINGS.** The core correction is sound: FR-042 and TC-057 are not gated on IR-497. FND-001 is a one-sentence status-accuracy fix. It does not touch AC-1, the TC-057 procedure or the PLANNED status.

Evidence for FND-001, measured on the PR head:

- These typed constructors already exist:
  - `GeneratedIntervalInvalid` at `src/oracle/boolean_v1.rs:913`.
  - `GeneratedDescriptorReconstructionFailed`, `GeneratedTypeCheckRejected { cause: error.cause }` and `GeneratedEqualityCheckRejected { cause: error.cause }` at `src/oracle/equality/mod.rs:1964`.
  - `GeneratedBodyPlaceholderInvoked` at `src/oracle/function/mod.rs:2910`.
  - `GeneratedArgumentShapeMismatch`, `GeneratedOperandKindUnsupported` and `GeneratedEqualityCheckRejected` at `src/oracle/function/mod.rs:3458,3477,3481,3486`.
- They landed with IR-17, commit `5e0dc646` (#513).
- `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` have no CG producer.
- No test carries an FR-042 or TC-057 tag.
- The kernel cannot construct a unit `Refusal::CheckedInvariant`. Only FR-018 AC-8/AC-17 still name it, in spec text.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head `e9ee60c5568a93c4148d54f4ffdc7a42939219b8`. FR-042 Status now says seven of the nine producers have landed and names the remaining work: the two missing producers, the FR-018 AC-8/AC-17 text, and tagged TC-057 verification. AC-1 and TC-057 are unchanged and the status is still PLANNED. |
