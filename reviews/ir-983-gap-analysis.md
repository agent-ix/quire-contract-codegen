---
id: SR-17940486709564847047725
title: "IR-983 CG #802 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-983; PR #802; src/replay/function.rs; spec/replay/functional/FR-024-counterexample-envelope-intake.md; spec/replay/matrix/TC-035-counterexample-envelope-intake.md"
review_set: subset
---

## Summary

Ticket: IR-983. Diff-scoped gap analysis of head 0b4c1a801458970cd028a923ea7e4b9c8bbf7f84 against origin/main. Plan completion: not assessed. Examined FR-024-AC-44 (value-parity half), FR-024-AC-46 and TC-035 against the four new `#[trace]`-tagged tests in src/replay/function.rs. Tagged and exercised: QSL-minted obligation from two real non-Boolean functions, QSL admission, typed request readback of target/occurrence/obligation, `replay_value_parity` Agrees with claim retention, supplied-limit default equivalence and explicit A/H/B, and typed refusals for target (second admitted claim), occurrence ordinal, subject, source arm, obligation and missing key, plus Boolean NoValueParityClaim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-46's package member has no value-path test or check: no test shows a packet minted from a different package is refused before a request, and the AC still names a packet `package_id` that `WitnessPacket` does not carry | src/replay/function.rs:934-943; src/replay/function.rs:2020-2098 |
| FND-002 | low | The occurrence substitution control varies only the ordinal; AC-46 names key node, role or ordinal | src/replay/function.rs:2044-2054 |

## Verdict

Coverage is good for the obligation, target, subject, arm and limits halves. The author's package deviation (binding by building the request from the checked package) is plausible because the packet has no package member and the QSL value-parity obligation preimage is package-derived, but it is untested and unrecorded: FND-001 needs either a foreign-package packet refusal test or an AC-46 amendment stating that the package binds by construction. FND-002 is a judgement call because whole-key equality makes role and node mismatches fail by construction.
