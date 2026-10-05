---
id: "SR-1484"
title: "CG PR 271 gap analysis: FR-029, FR-016-AC-24, FR-030, TC-040 and TC-041 against the terminal map, its conversions and its tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@06075846dcd8f0ce3710c174bb4b80f29f638b4f; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/replay/functional/FR-016-witness-native-replay.md (AC-12, AC-13, AC-24), spec/kani/matrix/{TC-040,TC-041,tests}.md, spec/replay/matrix/tests.md, src/kani/terminal.rs, src/replay/{function,frame}.rs, tests/it/{terminal_map,skeleton_spine}.rs (diff 250dc84...HEAD)"
---

# SR-1484: CG PR 271 gap analysis

## Summary

Ticket: IR-465 (code half), with IR-611. Plan completion: not assessed.

Coverage, measured with quire 0.36.1 on both trees rather than taken from the PR:

- `quire coverage --strict`: 67 unbacked rows on the merge base and 52 on the head, with 0
  contradicted statuses on both.
- `quire matrix --format tsv`, diffed per criterion: exactly FR-029-AC-1, 2, 4, 5, 6, 8, 9, 11,
  12, 13 and 14, and FR-016-AC-24, move from `untagged` to `tagged`. Each one binds to one
  test, which names that AC in its `Trace:` line. FR-029-AC-3 and AC-10 stay `untagged`: the
  AC-3 test is tagged `TC-040` only, and no test is tagged AC-10. quire does not count a bare
  `TC-040` tag toward any AC, so tracing TC-040 inflates nothing. Every FR-030 criterion stays
  `untagged`. No row anywhere else changes status; the other diff lines are FR-016 line-number
  shifts in `skeleton_spine.rs`.
- Status prose matches the measurement. FR-029 Status says every criterion is backed except AC-3
  and AC-10, with the right reasons: `KaniInconclusiveReason` has no memory-exhausted variant, and
  `qsl_replay` at 02530e7 does not re-export `InternalFault`. I checked that it lives only in
  `qsl-foundation/src/diagnostic.rs` and is absent from `qsl-replay/src/lib.rs`. The kani
  `tests.md` rows split AC-3 and AC-10 out as Planned, and the replay `tests.md` marks
  FR-016-AC-24 Covered.
- FR-030 is left planned, and that is honest. QSL 02530e7's `DeclineCode` has one arm,
  `Qsl(Code)` (`proof_result.rs:153-156`). No CG source or test names `DeclineCode`, `Declined`,
  TC-041 or FR-030, apart from a module doc line. Nothing builds against an arm that does not
  exist.

Each criterion claimed backed, against its test, with mutant results from SR-1483:

- AC-1, AC-2, AC-4, AC-5: exact-value assertions. Using `success_checks = 3` makes the vacuous
  and cover rows fail if they forward the count (M8 killed).
- AC-6: all 8 settlement-free outcomes plus 5 settled readings, asserted `!= Tested`.
- AC-8: all three `DisagreementCause` variants, each through `ReplayVerdict` and the conversion,
  equal to `ReplayParity(cause)` (M7 killed).
- AC-9: `SourceCount` bare, in `SpineReplayError::Refused` and in `FrameReplayError::Refused`.
- AC-11: every listed CG failure equals `Failed` (M10 killed).
- AC-12: five non-reproduced readings and the reproduced-without-violation verdict are
  `!= Refuted` (M5 killed).
- AC-13: backed but weak (SR-1483 FND-001 and FND-002).
- AC-14: a real `ReplayPackage::new` over a repeated-identity lock gives `DuplicateIdentity`,
  which maps to `ReplayRefused(invalid_package)`.
- FR-016-AC-24: `tc_026_a_lock_repeating_a_dependency_is_refused`. The change is the minimal one
  the AC forces: `Duplicate` becomes `Input(DuplicateIdentity)` with code `invalid_package`. No
  `#[ignore]` is added. It drops one assertion that could have been kept (FND-003).

The three spec ambiguities the PR records:

- (a) `TerminalPairError`. FR-029 restricts the settlement to falsified outcomes, so refusing a
  mis-paired input is a sound reading of the Description. But it widens the Outputs ("One
  `qsl_replay::TerminalValue`"), and no Behavior line or AC states it (FND-002).
- (b) `ReproducedWithEvaluatedWitness` in a non-violation category maps to `Failed` through
  `CgDefect`. This is consistent with FR-016-AC-13 ("never a reproduced failure"), and `Failed` is
  the right value. At 02530e7, QSL's `execute.rs` fixes `proved = Verdict::from_category(Violation)`,
  and `WitnessArmResult::settle` settles reproduced only when proved and replayed agree. So the
  state cannot come from a QSL result. It can only be built through CG's public
  `EvidenceFailureCause::Verdict`, which makes it a CG defect. But FR-029 calls its `Failed` list
  "exactly" AC-11's list, and this case is not on it (FND-001).
- (c) FR-016-AC-12 ("a typed unavailable result, and never ... a failure") is left alone. It was
  planned and untagged before this PR and still is. FR-016's "failure" means a clause failure
  verdict, not `TerminalValue::Failed`. QSL's `from_replay_refusal` sends `ReplayRefusal::Fault`
  to `Failed`, which matches FR-029's fault row. Leaving it is acceptable for this PR. When
  AC-12 is built, it would read better if it said that "unavailable" is a replay verdict and that
  its terminal value is `Failed` (FR-029).

## Verdict

The AC claims are not inflated. The 11 FR-029 rows and FR-016-AC-24 are each backed by a real
tagged test, and AC-3, AC-10 and all of FR-030 are honestly planned. Two of the three recorded
ambiguities are decisions the code makes beyond the FR text. Both decisions are defensible, but
the FR should state them as rows or criteria rather than leave them in Status prose.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029 says the CG-defect reading is "exactly the errors FR-029-AC-11 lists", and that the adapter "shall map to `Failed` exactly these failures". The conversion also sends `EvidenceFailureCause::Verdict { disagreement: None }`, a `ReproducedWithEvaluatedWitness` replay in a non-violation category, to `CgDefect`, so `Failed`. The FR tables have no row for it, and only Status mentions it. `Failed` is the right value. QSL proves `violation` on every replay, so the state is unreachable from a QSL result and is a CG defect. The spec needs a row: falsified, replay reproduced in a category other than `violation`, `Failed` (CG defect, unreachable from QSL). That case should be added to AC-11's list, and the test should assert `== Failed`, not only `!= Refuted`. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:42-44,128-136,161; src/replay/function.rs:573-592; tests/it/terminal_map.rs:352-361 |
| FND-002 | medium | `run_terminal_value` returns `Result<TerminalValue, TerminalPairError>`. FR-029's Outputs say "One `qsl_replay::TerminalValue`", its Description says "Every pair maps to exactly one terminal value", and no Behavior line or AC defines `MissingSettlement` or `UnexpectedSettlement`. The tagged test for it carries `TC-040` only, so the error type is behaviour beyond the spec. Fix it one of two ways. Amend FR-029's Outputs and Behavior and add an AC for the two refusals. Or make the bad pair unrepresentable, which removes the error: take one input enum whose falsified variant carries the settlement, so the map stays one function and one `match` and returns `TerminalValue`. | src/kani/terminal.rs:43-80; spec/kani/functional/FR-029-run-outcome-terminal-record.md:71-73,187-189; tests/it/terminal_map.rs:196-213 |
| FND-003 | low | FR-016-AC-24 says "code `invalid_package` with cause `conflicting-definition`". The tagged test asserts the variant and the code but not the cause, although `DependencyInputRefusal::cause()` is public at 02530e7. It also drops the old `identity == "test/units"` check, although `DuplicateIdentity` still carries `identity`. Assert `input.cause() == Some("conflicting-definition")` and the identity. | tests/it/skeleton_spine.rs:405-428; spec/replay/functional/FR-016-witness-native-replay.md:144 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new FR-029 Behavior bullet says the pair refusal must be typed "because `KaniRunOutcome` is the classifier's own type, so no input type of the map can state the pairing without a fallible constructor that moves the same refusal to the driver". Status repeats the claim. The decision is fine, but the impossibility claim is false. The map could take the replay as a callback that it calls only on a falsified outcome, for example `impl FnOnce(&str) -> ReplaySettlement` over the counterexample. Or the driver could build the settlement inside its own `Falsified` match arm. Either way the pair becomes total with no fallible constructor. Reword it as a design choice: the map takes the pair as the driver built it, and refuses a mis-built pair. A normative bullet should not assert a constraint that does not hold. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:150-154,199-202 |

## Dispositions

Round 1, reviewed at d58e94085841930693a14b565a6268843465fce1 (fix commit d58e940 plus the merge
of PR 270, checked to bring in nothing else).

Coverage, re-measured with quire 0.36.1:
- `--strict`: 86 unbacked rows on `main` f08b61a and 71 on the head, with 0 contradicted on both.
- Per criterion: FR-029-AC-1, 2, 4, 5, 6, 8, 9, 11, 12, 13, 14 and 15 are tagged, which is 12 of
  14. AC-3 and AC-10 are untagged and planned. FR-016-AC-24 is tagged. Every FR-030 criterion is
  untagged.
- No planned AC counts as backed.

AC-15 is a direct, failable assertion. `tc_040_a_settlement_accompanies_a_falsified_outcome_only`
checks `Err(MissingSettlement)` for a falsified outcome with no settlement, and
`Err(UnexpectedSettlement)` for all 8 settlement-free outcomes.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d58e940: FR-029's Description CG-defect reading, a new table row, the `Failed` Behavior list and AC-11 now all name a replay reproduced in a category other than `violation`. The AC-11 test asserts it `== Failed`, and mutant M5 is now killed by the AC-11 test as well as AC-12's |
| FND-002 | fixed | d58e940: `TerminalPairError` is kept and specified. Outputs name it, a Behavior shall-statement states it, FR-029-AC-15 and TC-040 step 14 are added, the matrix row is updated, and the test is tagged FR-029-AC-15. The finding offered amend-or-redesign, and the spec is amended. The rationale wording is FND-004 |
| FND-003 | fixed | d58e940: `tc_026_a_lock_repeating_a_dependency_is_refused` destructures `DuplicateIdentity`, asserts `identity == "test/units"` and asserts `input.cause() == Some("conflicting-definition")` |
| FND-004 | still-open | Raised this round: the impossibility rationale in FR-029's Behavior bullet and Status is still the text at d58e940. It is low severity, and a one-sentence rewording fixes it |

Round 2, reviewed at 27c2bbbfcb883b00ee0475e7617c69387f52a502 (commit 27c2bbb, on top of d58e940).
`diff d58e940..27c2bbb` is prose in `spec/kani/functional/FR-029-run-outcome-terminal-record.md`
only. No source, test or other spec file changes, so the round-1 gate and mutant results still
stand.

- `quire validate` over the spec, plan and review files exits 0, with no diagnostic on FR-029.
- `--strict`: 71 unbacked rows on the head and 86 on `main` f08b61a, with 0 contradicted on both.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 27c2bbb: the impossibility claim is gone from the Behavior bullet and from Status. Both now say the typed refusal is a design choice (one function, one shape, the pairing checked at the call, asserted by FR-029-AC-15). The Behavior bullet names a replay callback, called only for a falsified outcome, as one total design that was not chosen. That is accurate |
