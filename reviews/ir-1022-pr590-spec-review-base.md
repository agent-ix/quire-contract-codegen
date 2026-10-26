---
id: SR-590001
title: 'spec-review/base review of IR-1022 CG PR #590'
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-007-v2-symbolic-width-tractability.md
review_set: subset
---

## Summary

Independent Claude Opus 5.5 review of CG PR #590 (Ticket: IR-1022) at 1e3c2936b997e047198791674400f7224e096295, base 4788b46723d7454af86fdbe9c7b4f4ad1154cbb6. The diff adds one file, proposed AD-007 (126 lines). Examined against FR-015-AC-39, AC-41, AC-47, AC-48, AC-119, AC-125 and AC-126, FR-016-AC-8, FR-017-AC-60, AD-003 and AD-006. Context only: `src/kani/generate/clause.rs` (`symbolic_arguments`), `src/kani/identity.rs` (`ObligationBinding`), `src/kani/abi.rs` (`KaniPrimitiveType`) and `src/replay/witness.rs` (`byte_width` and `decode_falsification`). Scoped `quire validate` exited 0, and the engineering-assurance advisory lint reported no document findings. No Cargo, Kani or make ci was run, because the change is spec-only.

AD-007 correctly labels itself an unapproved proposal. It names the AD-006 i64-draw conflict as blocking, keeps the 16 GiB ceiling, solver and unwind, preserves the original [0,1000] domain, and treats MemoryExhausted, timeout, unknown and missing cover as inconclusive. It makes no 16 GiB success claim and creates no criterion id.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-007's acceptance plan covers only FR-015-AC-41's two addition-body mutants. It omits FR-015-AC-119's runtime-helper mutant, in which implies_short_circuit's parsed body returns false and must be Falsified with in-domain playback. It also omits FR-017-AC-60's healthy installed playback test. It does not cite FR-015-AC-126, which already owns the IR-917 clause profile at 16 GiB and fixes the time settings as well as solver and unwind. AD-007's scope claims AC-119, so following its plan could close IR-1022 without the AC-119 and AC-60 controls. Under the new two-byte witness, those controls also need the new decoder. | spec/assurance/AD-007-v2-symbolic-width-tractability.md:99 |
| FND-002 | low | AD-007 says FR-015-AC-125 "expressly" preserves original i64 symbolic draws. In AC-125 the phrase "with original domains and i64 symbolic draws unchanged" sits in the native-ineligible clause, the honest general route or typed refusal. The general statement is AD-006's execution view. The conflict is real in AD-006, but the owner should see AC-125's exact scope so the amendment does not widen or misread it. | spec/assurance/AD-007-v2-symbolic-width-tractability.md:55 |

## Verdict

The proposal is not ready for the owner decision until FND-001 is resolved. FND-002 should be corrected in the same revision. It is spec-only and must not be read as approval or a normative override of FR-015-AC-125 or AD-006.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at df1848b3854ec37b4986751518e40109a8093070, still present at e81c561c8df206e5f0b6cafada5df10adcfacdec. The installed-proof row now requires the FR-015-AC-119 runtime implication helper-body mutant Falsified with in-domain playback. A separate row requires FR-017-AC-60 healthy installed playback. The row cites FR-015-AC-41/119/126 under the original 16 GiB, 600-second, CaDiCaL, unwind-3 profile. |
| FND-002 | fixed | Round 1 at df1848b3, unchanged at e81c561c. AD-007 now says the AC-125 unchanged-draw wording applies to its native-ineligible general route, and that the governing statement is AD-006's execution view. |
