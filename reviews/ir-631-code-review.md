---
id: SR-1607
title: "IR-631 code review: PR #290 spec-only diff"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#290; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
---

# SR-1607: IR-631 code review

## Summary

Ticket: IR-631. Reviewer: claude-opus-5-5, run d75eb678-590e-4bb0-b33c-ca7f9c5bde50. The diff (3 files, +276/-1) changes no executable code: no `.rs`, `Cargo.toml` or lockfile. The rust-review lane does not apply, and the Python and React lanes do not apply. I ran the language-independent sections: duplication and vendoring, unasked-for ceremony, integrity, and spec-code faithfulness. Two duplication findings remain. Both are `high` because the skill's duplication rule fixes that severity.

## Verdict

**FAIL**: two high duplication findings. The following are clean:
- No vendored or copied upstream types. FR-032 explicitly excludes copied QSL types and names no speculative upstream Rust API.
- No file-tracking ledger, pin or manifest. The proof-content identity is the permitted canonical identity digest.
- No lowered threshold or suppressed warning.
- Every source fact FR-032 cites was re-measured and matches the source.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-032 restates the scalar harness proposition and literal-operand rule that FR-015 owns, and does not cite them. "The parity projection is exactly the current harness assertion: a generated `Completed` value equals the native mathematical result inside the result range, and a generated `Refused` outcome occurs outside it ..." restates FR-015-AC-37, and "a recorded literal has singleton bounds" restates FR-015-AC-16. The prerequisites section also re-describes renderer internals (the `u64::MAX` meter). The two statements will drift when FR-015 changes. Fix: cite FR-015-AC-37 and FR-015-AC-16 as the authority, and keep only FR-032's own delta (the projection adds no refusal-cause or counter equality, and the four-operator profile). | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:107, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:156, spec/kani/functional/FR-015-bounded-kani-obligations.md:570, spec/kani/functional/FR-015-bounded-kani-obligations.md:549 |
| FND-002 | high | FR-032-AC-8's last sentence, "Omitting settlement yields `TerminalPairError::MissingSettlement` and no terminal value", restates FR-029-AC-15. This contradicts FR-032's own intent table ("Reuse in the routed integration without duplicating the terminal requirement") and Behavior text ("already FR-029 AC-15"). Fix: remove the sentence from AC-8; TC-047 step 6 already exercises FR-029-AC-15 by reference. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:140, spec/kani/functional/FR-029-run-outcome-terminal-record.md:233 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #290. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The restated harness proposition and renderer description are gone. FR-032 now says the generation proposition is owned solely by FR-015-AC-37, with its literal rule at AC-16, and that the replay projection refers to them rather than redefining them. FR-032 keeps only its own delta: no refusal-cause or counter equality, and the four-operator profile. The remaining note that today's renderer limits are `u64::MAX` describes current source in support of the new limits-context input; FR-015 states no such requirement, so it duplicates nothing. |
| FND-002 | fixed | AC-8's `MissingSettlement` sentence is removed. TC-047 step 6 verifies FR-029-AC-15 directly, and its closing note says no duplicate FR-032 criterion is added. |
