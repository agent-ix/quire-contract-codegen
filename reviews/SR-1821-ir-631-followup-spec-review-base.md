---
id: SR-1821
title: "IR-631 follow-up spec review: scalar obligation identity and authority"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-1821: IR-631 follow-up spec review

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only, base `5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60`. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af.

This is the base spec review. The sub-analyses are:

- SR-1822: EARS
- SR-1823: integrity
- SR-1824: failure domain
- SR-1825: scope boundary
- SR-1826: dependency

The code review is SR-1820. This base review adds one high and one medium finding.

## Method

I read FR-032, TC-047 and AD-003 E-1 at the head for quality, consistency and completeness. My focus was the CG-owned scalar `ObligationIdentity`, which must use only the fixed members of QSL ADR-013 O-09 as stated in merged QSL FR-357. I checked:

- the authentic claimed node and occurrence
- the obligation kind
- the operand identities and declared proving domains
- that no invented digest member is added
- the retained code gates: IR-648 metadata, driver observation authentication, and the full-claim comparison on every outcome

The upstream facts were measured at merged QSL `30d7beb7483b721bcb1ec98926e8b717e9a76fb7` (see SR-1820). For the CG side I measured `ObligationKind` and `ScalarObligationIdentity` in `src/kani/identity.rs`, FR-015-AC-38/69/76 and FR-022.

Skipped methods:

- **Criterion strength:** Jev is not installed (`command -v jev` exit 1). No calibration was attempted or faked.
- **Architecture evaluation:** no implemented architecture changed. The AD-003 identity boundary is covered in SR-1824.
- **Object, evidence and risk:** not justified by this diff, which adds no domain object or verification method and changes only planned work.

## Verdict

**FAIL, because of FND-001 (high).** What the PR gets right:

- The preimage is restricted to O-09's four members, and transcript, native outcome, content identity, operator label, counters and tool/version fields are explicitly excluded.
- The positional `GraphChild`/`InlineLiteral` allocation matches merged `parity_identity.rs`.
- Repeated parameter draws are kept as separate positions.
- CG must use the shared public mint function, with no copied encoder.
- The typed encoder refusal is kept.
- The full-claim comparison is required on every outcome, and agreement-only and digest-only checks are explicitly rejected.
- The driver is named as the observation-authentication authority, and QSL is not credited with authenticating the artifact.
- The IR-648 metadata, CG retention, driver and consumer-conformance code gates are all kept.
- All ten criteria stay Planned, with no coverage claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The scalar preimage requires "the existing obligation kind", with `obligation_kind` as "the existing CG kind's wire string". AC-9 and AC-10 require byte-exact conformance on it, and missing kind must refuse. But CG's `ObligationKind` is only `precondition`/`postcondition`/`invariant`/`frame`, which are clause roles. `ScalarObligationIdentity` carries no kind, and an exact-scalar claim "has no QSL clause". No spec names which value a routed scalar claim uses or where it is read from. The composite sibling names its kind (`bounded_shadow`). QSL treats the string as opaque, so two implementers can choose different values and both pass QSL while producing different identities. The "missing kind refuses" rule cannot be tested because no kind exists to retain. The spec should name the scalar `obligation_kind` value and its authoritative source, or record it as an explicit gate. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:200, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:230, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:300, spec/assurance/AD-003-evidence-chain.md:207 |
| FND-002 | medium | AC-1, setup-precedence step 5 and Outputs still require an "unsupported-capability refusal while the gate holds" for "all upstream capabilities needed for an admitted plan". The QSL operator arm, full-report `claim()` and scalar causes are now recorded as merged, and the remaining gates are CG retention, IR-648 and the driver. The spec no longer says which upstream capability step 5 checks at run time, or what condition ends the refusal. An implementer cannot tell whether a builder that refuses every valid request satisfies AC-1 indefinitely, or when the admitted path must replace it. The capability step 5 tests (for example IR-648 metadata availability) and AC-1's exit condition should be named. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:292, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:281, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:112 |

## Dispositions

Round 1 re-check of fix commit `832633d7afa778e8a3688601595beb5d38917cb5` (run 733cf463-44a3-4ff9-83f7-a1a0a32f9463, model claude-opus-5-5). Each finding was verified against the spec text at that commit, not against the author's receipt. The reviewed content of `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8` was confirmed unchanged after the rebase onto main. The planned criteria remain unrun; no implementation, mutation coverage or settlement proof is claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: FR-032 allocates the sole scalar-family obligation_kind `operator_parity` for add/subtract/multiply/negate, bans clause roles, operation_identity derivation and free caller strings, and CODE-gates the typed source with a named refusal. The QSL #655 owner vector uses the same string; its 350-byte text independently rehashes to the stated identity, and its test execution is unverified. |
| FND-002 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: AC-1 and setup step 5 now name the capabilities (IR-648 accessor, scalar-kind source, unique occurrence, proving-context record, driver completion seam), and require an admitted plan once all are available; a permanently refusing builder fails. |
