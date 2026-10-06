---
id: SR-1661
title: "IR-639 spec review (integrity): unmerged mechanism presented as existing, helper discovery, metadata and wrapping"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen#299; spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-049-caller-death-ownership.md, spec/kani/matrix/tests.md, spec/spec.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
---

# SR-1661: IR-639 spec review, integrity

## Summary

Ticket: IR-639. PR: quire-contract-codegen#299 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 44b8eed1-c733-42e1-b8f3-7818119a17d5. I checked FR-034 and TC-049 for consistency with measured `main`, completeness of the maintained metadata, testability of each criterion and layout. One high finding, one medium finding and three low findings.

## Method

I measured `main` (`git grep` over `spec` and `src`) for the mechanism FR-034 builds on, and checked the state of PR #295 with `gh pr view 295` (OPEN, not merged). I read FR-028-AC-2, AC-3, AC-21 and AC-24 and FR-017-AC-14, AC-24 and AC-25 at `main`. I compared TC-049's `verifies` edges and Expected Results authorities with its TestCaseIndex row and the Functional Requirement Coverage row. I ran `quire matrix --format tsv` and `quire validate` over the five changed files. Both passed with module warnings only. The computed matrix shows FR-034-AC-1 to AC-13 and AC-15 to AC-24 `untagged`, and AC-14 `method-without-symbol`. No coverage is claimed or inflated. I checked `Cargo.toml` for a `[[bin]]` target and checked line lengths in the prose against the repository's ~100-column wrap.

## Verdict

**FAIL: one high finding.** The following were clean:
- Reserved ids FR-034 and TC-049 collide with nothing on `main`.
- Every relative link in the changed files resolves.
- The matrix row, TestCaseIndex row, `spec.md` registry row and `spec/tests.md` subsystem row all say Planned (IR-639) and claim no executable coverage.
- AC-14's single Inspection method without a symbol is not presented as coverage.
- The research probe is explicitly disclaimed as production evidence.
- FR-028-AC-21, AC-24 and FR-017 capture/batching remain authoritative in the prose.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-034 presents an unmerged mechanism as existing. It says it "closes the caller-death window in the Linux PID-namespace mechanism selected for FR-028 AC-21", cites "the preceding ceiling candidate", and AC-14 says "required by the existing ceiling mechanism". At `main`, FR-028-AC-21 is explicitly mechanism-neutral ("the choice is the code change's, Open Questions"). No file under `spec` or `src` on `main` mentions bubblewrap or a PID namespace, and PR #295, which selects that mechanism, is OPEN. FR-034's monitor, INIT, namespace PID 1 and bubblewrap gate criteria (AC-2, AC-5, AC-7, AC-8, AC-14) therefore pin a mechanism that FR-028 on `main` leaves open. The status rows say only "Planned (IR-639)" and do not record that FR-034 is gated on IR-241/PR #295 landing that mechanism. Either state FR-034 as gated on the IR-241 ceiling mechanism in Description, the matrix row and the `spec/tests.md` row (for example "Gated on IR-241 PR #295"), and replace "existing"/"selected" with that gate, or land FR-034 after the mechanism is on `main`. | spec/kani/functional/FR-034-caller-death-ownership.md:17-18, spec/kani/functional/FR-034-caller-death-ownership.md:22-23, spec/kani/functional/FR-034-caller-death-ownership.md:81, spec/kani/matrix/tests.md:50 |
| FND-002 | medium | The helper discovery path in AC-13 and Inputs is undefined. The alternative to an explicit path is "this package's installation" / "the executable installed from this same Cargo package". `quire-contract-codegen` has no `[[bin]]` target, and it is consumed as a library dependency, so Cargo neither builds nor installs a dependency's binaries for the consumer. No location is defined that an implementation could probe without a PATH or global lookup, which AC-14 forbids ("ad hoc global installation"). Separately, no criterion makes the handshake refuse a helper built from a different revision of this package. A stale installed guardian that predates lease cancellation or kill-before-close would pass AC-4 and AC-15 and silently reopen the hole. Define the discovery rule concretely (explicit path only, or a named location relative to a named artifact). Require the typed handshake to refuse a guardian whose protocol identity does not match the executor's. | spec/kani/functional/FR-034-caller-death-ownership.md:32-33, spec/kani/functional/FR-034-caller-death-ownership.md:80 |
| FND-003 | low | The TC-049 metadata is inconsistent. Its frontmatter `verifies` FR-028 and FR-017, and Expected Results names FR-028-AC-2/3/21/24 and FR-017-AC-14/24/25 as authorities. Its TestCaseIndex row in `spec/kani/matrix/tests.md` lists only FR-034-AC-1 to AC-24, and the FR-028 and FR-017 coverage rows do not name TC-049. A reader of the index cannot tell that TC-049 also re-asserts those criteria through the new ownership path. List them in the index row, or drop the `verifies` edges and mark them as "unchanged authority" rather than as verified. | spec/kani/matrix/TC-049-caller-death-ownership.md:6-11, spec/kani/matrix/TC-049-caller-death-ownership.md:93, spec/kani/matrix/tests.md:79 |
| FND-004 | low | AC-20 says "An already expired deadline causes no Dispatch" but not how the run settles. TC-049 step 9 likewise expects only "no Dispatch" for expiry before startup. "Settles timed out" also does not name FR-028-AC-2's classification (`inconclusive` with the timed-out reason). State that an already expired deadline settles as FR-028-AC-2 states, or as a named typed refusal. Also, AC-1's "Before the guardian establishes ... monitor ownership, no backend monitor is created" is circular: monitor ownership cannot precede monitor creation. Reword it to "before the lease is established". | spec/kani/functional/FR-034-caller-death-ownership.md:87, spec/kani/functional/FR-034-caller-death-ownership.md:68 |
| FND-005 | low | The prose wrapping is inconsistent. Description line 17 (277 columns) and Behavior lines 48, 51, 57 and 58 (144 to 288 columns) are unwrapped, while the rest of FR-034 and every other spec file wrap at ~100 columns. Rewrap them at whitespace. | spec/kani/functional/FR-034-caller-death-ownership.md:17, spec/kani/functional/FR-034-caller-death-ownership.md:48, spec/kani/functional/FR-034-caller-death-ownership.md:51, spec/kani/functional/FR-034-caller-death-ownership.md:57, spec/kani/functional/FR-034-caller-death-ownership.md:58 |
