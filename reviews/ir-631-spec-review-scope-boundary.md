---
id: SR-1606
title: "IR-631 spec review (scope-boundary): CG, driver and QSL ownership in FR-032"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1606: IR-631 scope and boundary analysis

## Summary

Ticket: IR-631. Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. I checked how FR-032 allocates work between CG (builder and converter), the driver (native execution of the proved artifact, and the QSL call) and QSL (exact evaluation, envelope arm, causes), against AD-002 and the actual source. The allocation is internally coherent, and the driver-owns-the-call split matches the consumer requirement recorded on the ticket (which I treated as data). It contradicts AD-002 as written, however, while FR-032 claims that AD-002 needs no amendment.

## Verdict

**CONDITIONAL**: one medium finding. The following are clean:
- FR-032 excludes a CG-local exact evaluator, copied QSL types and any widened function selection.
- QSL is the external dependency for the operator arm, the observation contract and the harness-defect cause. FR-032 marks each as assumed and gated (QSL-641), not as landed. I confirmed none exists at the QSL revision CG's lockfile resolves or on QSL's main branch.
- The driver is the external caller.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-032 says "the AD-002 boundary remains" and "no amendment to the existing envelope or architecture rule is implied", but AD-002 conflicts with it in three places. (1) AD-002 states "CG calls `qsl_replay::replay`, `replay_frame` and `replay_state_clause` only"; the driver-owned QSL call is a new allocation AD-002 does not record. (2) AD-002 R-6 requires every request CG builds to carry a QSL `ObligationIdentity` computed by CG. (3) R-7 requires every request to name the `package_id` returned by `call_site`. FR-032 neither meets nor amends R-6 or R-7, and its own source note says `CallSiteSelection` has no scalar selection, so there is no `call_site` package id. Fix: amend AD-002 in this PR, or state how R-6 and R-7 apply to the scalar plan. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:43, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:193, spec/assurance/AD-002-cg-qsl-replay-seam.md:86, spec/assurance/AD-002-cg-qsl-replay-seam.md:131, spec/assurance/AD-002-cg-qsl-replay-seam.md:134 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The "AD-002 boundary remains" and "no amendment is implied" claims are gone. FR-032 now states the scalar plan is constrained by AD-002 R-6 and R-7 with no exception. Gate 4 requires a QSL-owned scalar `ObligationIdentity` that CG mints and carries unchanged. Gate 5 requires the original proving `call_site` package identity and source bytes, with QSL recompiling and checking membership; both are QSL-641-gated. The builder makes no execution call, so AD-002's CG call list still holds. If QSL-641 cannot meet R-6 and R-7, the route stays gated pending an owning decision. QSL's own `call_site` test confirms it locates an Integer-returning function, whose replay is refused with `NotAPredicate`. AD-002 is unchanged. |
