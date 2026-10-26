---
id: AD-007
title: "Proposed full-domain V2 symbolic proof tractability"
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
# AD-007: Proposed full-domain V2 symbolic proof tractability

## System Boundary

This is an **unapproved proposal**, not a change to FR-015 acceptance or proof credit.
IR-1022 owns tractability of the same QSL-authored V2 postcondition clause required by
FR-015-AC-41 and AC-119: `amount < 1000 implies amount + 1 <= 1000`, with the checked
parameter domain `amount: Int[0, 1000]`. CG owns the generated symbolic draw, sealed
harness record and V2 counterexample reader. The admitted package, byte-identical FR-014
oracle, genuine quire-exact operations and Meter, compiled subject, independent native
arithmetic assertion, final cover and source-owner mutations retain their owners.

The original installed healthy proof exhausted the existing 16 GiB ceiling; a separate
24 GiB diagnostic also exhausted memory. Neither produced a healthy verdict. An isolated,
**no-credit** experiment on commit `e1074c9c603c8d80fd34224b0bc3b601e91fc822` kept the
`i64` draw, assumed `[0,1000]` and rebound the semantic value through `u16` after that
assumption. The focused generator test passed. One installed healthy run reached CBMC with
CaDiCaL and unwind 3, then the test failed after 619.43 seconds with
`MemoryObservationFailed(namespace cleanup failed: namespace teardown was not confirmed)`.
The configured wall-clock limit was 600 seconds. No Kani verdict, cover result, peak RSS or
mutant result was exposed; timeout before teardown is plausible but unproven. This run
provides **no proof credit and no measured tractability benefit**. The accepted 16 GiB,
wall-clock, solver and unwind settings remain fixed. No smaller semantic domain, altered
assertion, substitute evaluator or fixture-only route is proposed.

## Views

The existing obligation, two representation candidates and acceptance evidence are
separate views of the same checked clause. Both candidates preserve every declared
valuation; neither has demonstrated an installed healthy verdict at the accepted limit.

### Existing contract and primary candidate

FR-015-AC-39 requires one nondeterministic draw per drawn input and an inclusive assumption
**equal** to its declared domain. FR-015-AC-40 requires the embedded clause oracle to match
FR-014 byte for byte. FR-015-AC-41 requires the independent arithmetic assertion, healthy
and both addition-body mutant verdicts, in-domain playback and selected 999 control.
FR-015-AC-119 additionally requires the runtime implication helper-body mutant.
FR-015-AC-126 owns the 16 GiB clause profile, including its existing time, solver and
unwind settings. FR-016-AC-8 binds persisted symbolic argument positions to witnesses;
FR-017-AC-60 owns healthy installed playback. AD-003 distinguishes semantic obligation
identity from the generated harness record.

The **primary candidate** preserves the original `i64` `kani::any()` draw and exact
inclusive `raw >= 0 && raw <= 1000` assumption. Only *after* that assumption, shadow the
semantic binding with `let amount: i64 = i64::from(raw as u16);`. Every allowed `raw` fits
in `u16`; conversion back to `i64` is the identity on all 1001 checked values. It adds
no draw or domain restriction. The oracle, genuine subject, independent expected arithmetic
and cover all consume that same rebound `i64` value. Downstream bit-vector arithmetic can
see zero high bits, which is the hypothesized SAT reduction; the failed diagnostic has not
established a resource improvement. For any other V2 interval, apply this transformation
only when **both** checked endpoints fall within `[0, u16::MAX]`; otherwise retain the
existing generation. Selection depends on the checked domain, never fixture identity or
proof outcome.

The physical draw remains `i64`. V2's `V2ClauseObligationIdentity.arguments` currently
holds `ScalarObligationArgument` identifiers and bounds without a type;
`decode_v2_clause_falsification` constructs temporary `ObligationBinding` values with
`KaniPrimitiveType::I64`. That reader, its eight-byte witness and the `i64` subject ABI
remain unchanged for this candidate; exact-999 playback still uses eight bytes. The
generated source changes, but today's V2 record serializes the identity and Rust path,
not the Rust bytes or their digest. The isolated diagnostic therefore does **not** rekey
the record and cannot support proof credit. Before an accepted implementation, add a
required, versioned `symbolic_encoding_policy` member to `V2ClauseObligationIdentity`
and its serialized sealed record, distinguishing `original_i64_v1` from
`post_assumption_u16_rebind_v1`. A record without the member or with a different policy
or revision cannot identify or reuse this proof encoding. Regeneration
from the same checked package and policy must yield byte-identical source and record
under FR-015-AC-47. AD-003's semantic obligation identity remains independent of this
generated record. Append a concrete 999 playback to the sealed proof, then remove only
that appended test to recover the original source byte for byte.

### Secondary physical-width candidate, pending a separate decision

If a clean installed result leaves the primary candidate insufficient, a physical `u16`
draw is a separate candidate, **not** the approved implementation. It would still make
exactly one draw and an explicit inclusive assumption `draw >= 0 && draw <= 1000`, even
when the carrier's lower bound makes one comparison tautological. Conversion to the
semantic `i64` binding must be injective and surjective over the full checked interval.
For generic carrier choice, choose the narrowest supported type containing both endpoints;
at equal width choose unsigned for a nonnegative interval, signed for an interval with a
negative endpoint. Intervals crossing zero must use a signed carrier. Retain `i64` if no
narrower supported type contains both endpoints.

AD-006's execution view says original `i64` symbolic draws remain unchanged. The wording
in FR-015-AC-125 about unchanged draws applies specifically to its native-ineligible
general route; it does not independently govern this native-eligible clause. A physical
`u16` draw conflicts with AD-006 and requires an owner decision before normative change
or implementation. V2's `ScalarObligationArgument` has no physical-type member and its
decoder currently constructs `ObligationBinding` with hardcoded `I64`; neither can safely
describe a two-byte draw. This candidate must add a required physical carrier type per
V2 argument to its identity record while retaining the semantic `i64` subject type, and
make the V2 decoder use that sealed physical type for width, signedness, byte order and
rendered-comment checks. Its 999 playback would use two witness bytes but
call the unchanged `i64` subject. Malformed, mismatched or out-of-domain encoding must
receive a named typed refusal, without replay or proof credit. No reader may guess width
from byte length or silently reinterpret an old harness record. The physical choice
would rekey the generated harness record, not AD-003's semantic obligation identity.

### Proposed acceptance and evidence plan

| Proposed check | Falsifiable control and required evidence | Existing owner |
| --- | --- | --- |
| Full checked domain | One draw and an exact inclusive `[0,1000]` assumption; prove the post-assumption transformation is identity for all 1001 values, including 0, 999 and 1000. A missing lower-bound comparison, narrowed interval, second draw or fixture-only selection fails. Test signed, crossing-zero and full-`i64` boundaries for any generic policy. | FR-015-AC-39 / TC-025 |
| Sealed identity and witness | For the primary candidate, add a required symbolic-encoding policy and revision to the V2 harness record, retain `i64` width and eight-byte typed playback, and refuse missing or stale policy records. Refuse malformed width, wrong signedness, mismatched rendered comment, extra value and out-of-domain value by name. If the physical candidate is later authorized, separately prove two-byte typing and explicit V2 carrier/semantic fields. | FR-015-AC-47; FR-016-AC-8 / TC-025/026 |
| Exact-999 and healthy playback | Run 999 through the actual generated proof and independent arithmetic assertion; remove only the appended concrete test to restore sealed source byte for byte. Run healthy installed playback and require the expected in-domain result. | FR-015-AC-41; FR-017-AC-60 / TC-025/027 |
| Installed proof and mutations | Under the **original 16 GiB, 600-second, CaDiCaL, unwind-3 profile**, require genuine healthy `Verified` with final cover, both addition-body mutants `Falsified` at the independent arithmetic assertion with in-domain playback, and the same-clause runtime implication helper-body mutant `Falsified` with in-domain playback. Record elapsed time, process-tree peak RSS, Kani verdict and cover for each run. Timeout, OOM, cleanup refusal, missing result or missing mutation remains inconclusive. | FR-015-AC-41/119/126 / TC-025; IR-917 and IR-1022 remain open until measured |

Focused source, decoder and boundary tests establish representation and refusal behavior;
only installed backend evidence establishes 16 GiB tractability. A no-credit experiment
must remain distinct from acceptance. Tests for any new normative criterion must trace
its formal criterion id after owner approval; this proposal creates no new criterion id.

## Decisions

| Pending decision | Recommendation | Consequence |
| --- | --- | --- |
| Does the primary `i64` draw and lossless rebind improve the installed proof? | Keep it as the first candidate, but make no acceptance claim from the teardown-failed diagnostic. Inspect retained runner evidence before any repeat; require a clean healthy result under the original profile. | No AD-006 draw amendment, new witness schema or changed semantic identity is needed. A required V2 symbolic-encoding policy revision must rekey the generated record before accepted proof evidence. |
| Should a physical `u16` draw be attempted? | Decide only after the primary result and a separate AD-006/FR-015 owner review. Do not amend normative requirements solely on a hypothesized benefit. | Requires an explicit V2 physical-carrier member per argument, typed decoder and identity contract, then fresh installed evidence. The native-ineligible clause of AC-125 must retain its stated conditions unless separately reviewed. |
| Does any candidate meet acceptance? | Decide only from healthy, cover and all required installed mutant/playback controls at the original limits. | If not, retain `Inconclusive`; no higher ceiling, smaller range, weaker assertion or substitute owner follows. |

## Risks

- **Tractability unproven.** The diagnostic ended in a namespace teardown refusal after
  619.43 seconds; it did not expose the wait conclusion or peak RSS. Owner: IR-1022
  implementation lead. Next review: after runner evidence and a clean installed result.
- **Independent evaluator cost.** Both the oracle and native assertion use genuine owner
  operations and Meter; reducing downstream bit width might not control their expansion.
  Owner: CG architecture reviewer. Next review: measured result under the fixed profile.
- **Physical-width contract conflict.** Only the secondary candidate changes AD-006's
  original `i64` draw and V2's current hardcoded `I64` witness decoding. Owner: AD-006 and
  FR-015 maintainer. Next review: before any normative edit or physical-width code.
- **Witness and identity confusion.** The primary candidate changes generated source but
  today's V2 record bytes remain equal; a required symbolic-encoding revision must change
  that record before proof credit while keeping the eight-byte witness and AD-003 semantic
  identity rules. Owner: CG identity reviewer. Next review: record and decoder evidence
  before installed mutant playback.
