---
id: SR-788001
title: "IR-1507 PR #788 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #788 at 12d9a54c; IR-1507; src/kani/identity.rs, src/kani/generate/clause/v2.rs, src/kani/generate/collection.rs, src/kani/generate/collection/operation.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/run/execute.rs, src/kani/test_support.rs, src/lib.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1507. PR #788 (one commit 12d9a54c on origin/main a55a9cc3) adds a required `claim: &V2ClauseObligationClaim` to `ObligationItem::V2Clause`, carries QSL's `qsl_replay::ObligationIdentity` unchanged and sealed on `KaniV2ClauseObligationHarness.obligation`, and refuses a record mismatch as `UnsupportedObligation::V2ClauseClaimMismatch`. Occurrence and kind are checked in `classify_v2_clause`; arguments and state reads are checked in `seal_with_selected`, after the Rust source and the identity record are rendered. Reviewed read-only in /home/peter/dev/worktrees/IR-1507-review, scoped to `git diff origin/main...HEAD`. The rust-review lane is folded in here.

## Verdict

**CHANGES REQUIRED.** The carry is clean: the identity is sealed (`has_valid_provenance` includes it), it is kept out of the AC-47 identity record (`record()` serialises only `V2ClauseObligationIdentity`; the ordinal test pins equal record bytes), no hash, digest or pin is added, no new `unsafe`, and the only new `expect`/`panic!` are in `#[cfg(test)]` code. The new public types in `src/lib.rs` (`V2ClauseClaimField`, `V2ClauseObligationClaim`, `V2ClauseObligationRecord`, `V2ClauseSelectedReads`) are named consistently with the existing `V2Clause*` family and avoid the existing `oracle::clause::V2ClauseClaim`. Two defects in the cross-check: it compares CG-generated identifiers, which no independent producer can know, and it decides argument and state-read mismatches only after rendering. Overlap: open PR #583 (IR-917) adds three-argument `KaniV2ClauseObligationHarness::generated(...)` calls and `ObligationItem::V2Clause` items without `claim`; whichever lands second needs a compile-level rebase. The other listed PRs (#787, #553, #535, #474, #413, #381) touch shared files (`lib.rs`, `outcome.rs`, `identity.rs`) but only textually.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The record cross-check compares CG-generated identifiers, so an independently selected claim cannot match. `mismatch()` compares whole `ScalarObligationArgument` and `V2ClauseStateField` values. That includes `identifier`, which is CG's generated name: `parameter_<name>` or `parameter_<name>_current` on the scalar route, the bare `parameter.name` on the collection route, and the generated `slot` or input name for state reads. AC-48 says a state-field read is named by its declaring node id and field name. A claim with the right node, field and domain but a different `identifier` is refused as `StateReads`. The same parameter also has different identities on the two routes. | src/kani/identity.rs:421-448; src/kani/generate/clause/v2.rs:740-753; src/kani/generate/collection.rs:621-663 |
| FND-002 | medium | The argument and state-read mismatch is decided after the harness is rendered: `seal_with_selected` runs on the finished `rust` and `record` artifacts. AC-48 and the ticket say the mismatch refuses before harness generation. One consequence is visible: when rendering fails first (for example `V2ClauseConditionNotRendered`), a claim with a mismatched argument or read reports the render refusal, not `V2ClauseClaimMismatch`. Another: `refuse_unrendered_postcondition_siblings` renders each sibling precondition a second time just to learn whether it seals. Derive the selected parameters and reads at lowering time (`LoweredV2Clause` / `LoweredOperation`) and check every record field in `classify_v2_clause`, before any render. | src/kani/generate/clause/v2.rs:659-685; src/kani/generate/negotiate.rs:965-992; src/kani/generate/negotiate.rs:578-586 |
| FND-003 | low | The public refusal `V2ClauseClaimMismatch { selected: Some(V2ClauseSelectedReads) }` returns the generator's own selection, and that type is exported. Any caller can therefore build a matching claim by probing once, which is exactly what `tests/common/v2_claim.rs` does, and the "independently selected" premise of the check becomes self-referential. Keep the diagnostic if wanted, but name only the differing member, or keep tests from bootstrapping on it (see SR-788002 FND-001). | src/kani/generate/outcome.rs:231-241; src/lib.rs:274-275; tests/common/v2_claim.rs:56-77 |

## Coverage

Rust idioms and repo conventions: the new code follows them. It uses the existing sealed-output pattern, and `pub(in crate::kani::generate)` keeps the seal helpers internal. Sorting before comparison makes the check independent of list order, and a duplicate entry still differs in length. No integer conversion at a wire boundary changed. In `operation_selected_reads`, the `u64`/`usize` cardinality bounds sit in `i64` fields because the existing `SymbolicBounds::Sequence` already types them that way; no new cast was added. One focused test job ran at the PR head (19 passed: `v2_clause_obligation_identity` and `kani_collection_operation`). No Kani run, no lint run, no full suite; the coder reports 874 integration and 535 lib tests.

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 | CG compares the complete source occurrence, obligation kind, parameter identities and declared domains, and any state-field read identities and domains, as separate obligation-record fields against the independently selected claim; a state-field read is named by its declaring node id and field name, never by a node of its own. A mismatch refuses before harness generation even when structural identity is unchanged. |
| FR-015-AC-47 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:859 | Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). |
| src/kani/identity.rs | examined | src/kani/identity.rs:370-497 | V2ClauseObligationRecord, V2ClauseObligationClaim, V2ClauseSelectedReads, V2ClauseClaimField, mismatch(), sealed obligation field |
| src/kani/generate/negotiate.rs | examined | src/kani/generate/negotiate.rs:905-992 | classify_v2_clause occurrence/kind check |
| src/kani/generate/clause/v2.rs | examined | src/kani/generate/clause/v2.rs:645-685 | seal_claimed_harness / seal_with_selected |
| src/kani/generate/collection.rs | examined | src/kani/generate/collection.rs:612-663 | operation_selected_reads |
| src/lib.rs | examined | src/lib.rs:271-276 | new public re-exports |
