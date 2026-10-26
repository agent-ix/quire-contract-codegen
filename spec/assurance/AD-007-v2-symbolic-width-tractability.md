---
id: AD-007
title: "Proposed width-minimal V2 symbolic draw for bounded Kani tractability"
type: ArchitectureDescription
status: proposed
owner: quire-contract-codegen-maintainer
system: V2 clause Kani symbolic input, sealed harness identity and counterexample decoding
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-006
    type: references
---
# AD-007: Proposed width-minimal V2 symbolic draw for bounded Kani tractability

## System Boundary

This is an **unapproved proposal**, not a change to FR-015 acceptance or proof credit. IR-1022
owns tractability of the same V2 postcondition clause required by FR-015-AC-41 and AC-119:
`amount < 1000 implies amount + 1 <= 1000`, with the checked parameter domain
`amount: Int[0, 1000]`. CG owns the generated symbolic draw, its proof identity and its V2
counterexample reader. The admitted package, clause, byte-identical FR-014 oracle, genuine
quire-exact operations and Meter, compiled subject, independent native arithmetic assertion,
final cover, bounded Kani runner and source-owner mutations retain their existing owners.

The installed healthy clause currently ends `Inconclusive(MemoryExhausted)` under the existing
16 GiB ceiling. A diagnostic under 24 GiB also exhausted memory before any healthy verdict or
mutant run. These observations motivate a narrower symbolic representation; they do not show
that it fits 16 GiB. The accepted solver, unwind, wall-clock and memory limits remain fixed for
acceptance. No smaller semantic domain, changed assertion, stubbed owner evaluator or fixture-only
path is proposed.

## Views

The current input contract, the proposed representation and the acceptance observations are
separate views of the same clause. The representation view changes proof encoding only if the
contract owners approve it; the acceptance view remains an unmeasured plan.

### Existing contract and conflict

FR-015-AC-39 requires one nondeterministic draw per drawn input with an inclusive assumption
equal to its declared domain. FR-015-AC-40 requires the embedded clause oracle to match FR-014
byte for byte. FR-015-AC-41 requires the independent arithmetic assertion, genuine installed
healthy and two arithmetic-mutant verdicts, typed in-domain playback and the selected 999
control. FR-015-AC-119 requires the unchanged value to reach the runtime implication subject.
FR-016-AC-8 binds persisted symbolic argument positions to witness positions. AD-003 distinguishes
the semantic obligation identity from the generated harness record.

**Blocking conflict:** FR-015-AC-125 and AD-006's execution view expressly preserve the
original `i64` symbolic draws. The proposed physical `u16` `kani::any()` changes that draw,
although the semantic value passed to the subject and oracle remains `i64`. The V2 witness
reader also currently interprets every such draw as eight `i64` bytes. Implementing the
proposal before the FR-015-AC-125 and AD-006 owners decide this conflict would silently change
their contracts. Their other obligations, including the original domain and selected kernel
policy, must remain intact.

### Candidate generation and identity

For a V2 bounded integer input, select the narrowest supported signed or unsigned primitive
whose full range contains **both** checked endpoints; if none is narrower, retain `i64`.
Selection depends on the checked endpoints and a declared generator policy, never on a sample,
the proof outcome or a fixture name. For `[0,1000]`, draw one symbolic `u16`, assume its value
is at most 1000, then convert it without loss to the original `i64` parameter binding. The
conversion maps every one of the 1001 checked values from exactly one drawn value and admits
none outside the checked domain. The oracle, independent native expectation, subject and cover
consume that same converted binding. A negative interval needs a signed carrier that contains
both endpoints; an interval crossing zero cannot use an unsigned carrier.

Record the selected physical draw primitive in the **generated harness identity and persisted
witness schema**, with the original semantic parameter identity and declared bounds retained.
Changing the physical primitive changes the generated artifact and harness record. It does not
change AD-003's semantic clause obligation identity merely because proof representation changes.
Regeneration from the same checked input and policy remains byte-identical. The original
eight-byte V2 playback and a new two-byte playback must never be decoded by guessing from
length alone or silently reinterpreted under another harness identity.

The V2 counterexample reader uses the sealed harness's physical primitive to check arity,
width, signedness, byte order and Kani's rendered value comment, then converts losslessly to
the same typed `i128` witness value used by QSL replay and checks the **original** `[0,1000]`
domain. A malformed, mismatched or out-of-domain encoding receives a named typed refusal with
no replay or proof credit. The deterministic `999` playback uses the physical two-byte `u16`
encoding while the generated proof calls the original `i64` subject with 999. Its appended
playback test is removed to restore the complete sealed proof source byte for byte, as
FR-015-AC-41 and FR-017-AC-60 require.

### Proposed acceptance and test plan, pending contract decision

| Proposed criterion | Falsifiable control | Owner after decision |
| --- | --- | --- |
| Full-domain representation | For the admitted `[0,1000]` input, the generator draws one `u16`, retains exactly `[0,1000]`, proves conversion is injective and surjective over all 1001 values, and tests 0, 999 and 1000. An implementation drawing a smaller semantic interval or adding another draw fails. Test signed, zero-crossing and full-`i64` boundaries before making the selection generic. | FR-015 / TC-025 |
| Identity and witness typing | The sealed harness records physical width and original semantic bounds separately; changed width changes the harness identity while the AD-003 semantic obligation identity stays fixed. Two-byte V2 playback decodes to the same named typed amount, while eight-byte, wrong-sign, mismatched-comment, extra-value and out-of-domain forms refuse by name. | FR-015 and FR-016 / TC-025 and TC-026 |
| Exact-999 playback | The appended concrete test feeds two-byte 999 through the actual generated proof and its independent arithmetic assertion against the same plus-one owner mutation; removing only that test restores the sealed source, and the failure is attributed to that assertion. | FR-015 and FR-017 / TC-025 and TC-027 |
| Installed proof and resource result | At the **existing 16 GiB ceiling**, unchanged solver and unwind, the genuine healthy clause must classify `Verified` with the final cover satisfied; both authoritative addition-body mutants must independently classify `Falsified` at the arithmetic assertion with decoded in-domain playback. Record elapsed time and process-tree peak RSS per run. `MemoryExhausted`, timeout, unknown, missing cover, missing mutant or missing teardown is inconclusive. | FR-015 / TC-025; IR-1022 and IR-917 remain open until measured |

The first implementation experiment should exercise a generated proof with the width change
and original owner call graph before widening the change to other V2 domains. A focused source
and decoder test can establish representation correctness; only the installed backend can
establish the 16 GiB tractability claim. Tests for the new requirement must trace its formal
criterion id after the owner approves one; this proposal creates no criterion id or coverage.

## Decisions

| Pending decision | Recommendation | Consequence |
| --- | --- | --- |
| May the physical `kani::any()` type differ from the semantic `i64` subject argument? | Amend FR-015-AC-125 and AD-006 explicitly to permit a lossless physical draw selected from the complete checked domain, while retaining every other AC-125 native-kernel and proof condition. Add the planned FR-015/FR-016 criteria described above before code. | This is a reviewed contract change, not an interpretation that `i64` symbolic draws were unchanged. It requires a new sealed artifact and fresh evidence; historical proof results cannot be reused. |
| Does this proposal meet the 16 GiB acceptance ceiling? | Decide only from the installed healthy and mutant results. | The proposal may still fail because the unchanged oracle and independent evaluator both execute the real native kernel and Meter. If it fails, retain `Inconclusive` and escalate the measured result; no range, ceiling, assertion or owner substitution follows. |

## Risks

- **Tractability is unproven.** Width reduction may leave solver memory above 16 GiB. Owner:
  IR-1022 implementation lead. Next review: after the first installed healthy diagnostic with
  the original bounded runner.
- **Witness misbinding.** A two-byte draw decoded through the existing eight-byte schema could
  fabricate or reject a value. Owner: CG witness reviewer. Next review: before the first mutant
  proof; require sealed physical type, exact width and Kani-comment cross-checks.
- **Contract conflict.** FR-015-AC-125 and AD-006 currently require `i64` symbolic draws.
  Owner: FR-015/AD-006 maintainer. Next review: before any normative FR edit or implementation.
- **Identity confusion.** A physical representation change must rekey the harness record while
  leaving the AD-003 semantic obligation identity determined by its own inputs. Owner: CG
  identity reviewer. Next review: with the proposed FR-015 criterion and source diff.
