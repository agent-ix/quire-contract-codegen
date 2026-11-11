---
id: SR-17940486709564847047692
title: "IR-1519 PR #793 obligation-identity cross-check: code review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #793 at 48e18b45 (base 357ce2ae); IR-1519; src/replay/state_clause/invariant.rs, src/replay/state_clause.rs, src/replay/frame.rs, tests/it/state_clause_invariant_replay.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1519. This review covers the Rust change in `git diff origin/main...HEAD`, with the rust-review lane folded in.

The production change has three parts:

- `InvariantReplayInputs::decode_falsification` (src/replay/state_clause/invariant.rs:136-138) compares `harness.obligation` with the caller's `obligation_identity`.
- On a difference it returns the new unit variant `StateClauseReplayError::ObligationIdentityMismatch` (src/replay/state_clause.rs:255-257).
- The variant has a Display arm (:331-333) and maps to `ReplaySettlement::CgDefect` (:391).

The PR also corrects the "four labels" doc comments to two (authority, identity) in state_clause.rs and frame.rs, as the ticket comment asked. No "four labels" wording remains in src/ or tests/. invariant.rs on main carried none.

## Placement

The check runs in this order:

1. `PlaybackAlreadySupplied`
2. `NotInvariantHarness` (the let-else, then the V2 kind check)
3. The sealed-provenance check (`Decode(cg_witness_harness_provenance_mismatch)`)
4. The identity check
5. The `HarnessMismatch` checks for clause, object and state fields
6. Decode

FR-024-AC-65 orders only three things: NotInvariantHarness first, then the identity check, then HarnessMismatch. The placement meets that order. The spec says nothing about where the identity check sits relative to provenance. My before-provenance mutant passes all 10 tests, so the order between those two is spec-neutral, as expected.

Placing the check after provenance is the better choice. `has_valid_provenance` (src/kani/identity.rs:521-526) seals `obligation == generated_obligation`. So the identity check compares the caller against an authenticated harness claim.

A harness whose `obligation` was edited after generation is a forged or corrupt CG artifact, not a caller naming the wrong obligation. Reporting it as the provenance `Decode` refusal is consistent with AC-65, whose subject is the CG-generated harness's `obligation`. Both refusals settle as CG defects.

## Commands and mutation controls

I ran one focused job under flock (`cargo test --test it state_clause_invariant_replay`). The baseline passed 10/10. Mutants:

| Mutant | Result |
| --- | --- |
| Compare against a default identity (caller node, model None, ordinal 0) | killed: 2 IR-1519 tests fail; the 8 older tests pass because their harness claims are (node, None, 0) |
| Identity check moved after every HarnessMismatch check | killed by `tc_035_a_different_obligation_identity_is_refused_before_the_harness_members` |
| Identity check moved before the provenance check | survives; spec-neutral, see Placement |
| Identity check moved between the let-else and the V2 kind check | killed, but only by the AC-62-tagged `tc_035_the_refusal_set_is_closed_and_typed` (FND-001) |

The coder's four mutants (node and model only; field order first; map to HarnessMismatch; ignore identity) are reported as killed. Their kill sites agree with the assertions in the test diff.

## Verdict

**MERGEABLE.** The production change is minimal and correct, and it meets AC-65's order. FND-001 is one low test-intent finding and does not block.

Rust lane:

- No unsafe, panic, unwrap or integer conversion is added in src/.
- The variant is a unit variant with Display and a settlement mapping.
- `StateClauseReplayError` is not `#[non_exhaustive]`, so a new variant is semver-visible. That follows the crate's existing practice; no in-repo or QSL exhaustive match breaks.
- lib.rs is unchanged, because the enum is already re-exported. Layout L-2 is unaffected.
- Test tags use `#[trace("TC-035", ...)]` with no `/// Trace:` lines.
- No hash, pin, digest or Cargo change.

"Executor uncalled, no document built" holds by construction: `decode_falsification` takes no executor and builds no document. The refusal returns before `StateClauseReplay::new_invariant` can run. Step 46 phrases this as a property of the refusal site, and the test asserts the only observable part, that the playback stays unbound. That matches how steps 41-44 treat the same phrase.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-65-tagged test's "harness-kind check comes first" case uses a StateFrame harness. A StateFrame harness has no `obligation` and returns at the let-else, so the case cannot tell apart a mutant that puts the identity check between the let-else and the V2 kind check. I ran that mutant: it is killed only by the AC-62-tagged `tc_035_the_refusal_set_is_closed_and_typed`, whose V2 postcondition and precondition harnesses now carry a different obligation from the fixture's. The order is pinned, but the evidence sits under another criterion's tag. Using a V2 postcondition harness (`generate_with(.., native=true)`) in the AC-65 test would put the evidence on AC-65. | tests/it/state_clause_invariant_replay.rs:1184-1206; tests/it/state_clause_invariant_replay.rs:522-551; src/replay/state_clause/invariant.rs:119-138 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | The check runs after the harness-kind check (`NotInvariantHarness`) and before the `HarnessMismatch` checks of the clause node, declaring object and field order, so a harness differing in both identity and field order returns `ObligationIdentityMismatch`. |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | When the harness's `obligation` equals the caller's `obligation_identity` member for member, `decode_falsification` accepts it, and the packet and the request carry the caller's identity member for member |
| TC-035 step 46 | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:394-408 | Mutation controls: ignores the caller identity; compares only node and model; maps to HarnessMismatch; replaces with the harness's own; replaces with a default identity; checks field order first. |
| FR-029-AC-16 | context_only | spec/kani/functional/FR-029-run-outcome-terminal-record.md:393 | The state-clause replay path (FR-024) settles as the other replay paths do |
| src/replay/state_clause/invariant.rs decode_falsification | examined | src/replay/state_clause/invariant.rs:110-195 | if harness.obligation != self.obligation_identity { return Err(StateClauseReplayError::ObligationIdentityMismatch); } |
| src/kani/identity.rs has_valid_provenance | context_only | src/kani/identity.rs:521-526 | self.identity == self.generated_identity && self.obligation == self.generated_obligation |
