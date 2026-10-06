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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The new Bootstrap confirmation is a host-wide scan that the PR's own criteria forbid. FR-034 now mandates "an exceptional bounded host-procfs membership walk keyed only to the retained monitor's pinned PGID". Finding the members of a group through procfs means reading every host process entry. FR-034-AC-2 makes the walk mandatory even when an exact INIT pidfd is recovered: it "never replaces required startup-group observation". TC-049 step 5 then uses the walk's result as the passing observation. That contradicts FR-034-AC-23 ("without … wide host-scan authority") and TC-049 step 1 ("no production bypass, sleeps or host-wide scan establishes a successful assertion"). Under the new PID-1 topology the walk is also unnecessary for the safety property. Gate EOF starts only the trusted guardian bootstrap, which exits without a lease. A live bounded executor that cannot identify INIT can therefore close the gate and return a typed cleanup refusal. Either remove the walk and fail closed when INIT is unidentified, or reconcile AC-23 and TC-049 step 1 with an explicitly bounded exception, and drop the rule that an exact INIT pidfd never suffices. | spec/kani/functional/FR-034-caller-death-ownership.md:112-120, spec/kani/functional/FR-034-caller-death-ownership.md:151, spec/kani/functional/FR-034-caller-death-ownership.md:172, spec/kani/matrix/TC-049-caller-death-ownership.md:32, spec/kani/matrix/TC-049-caller-death-ownership.md:53-60 |
| FND-007 | low | The fix round introduced broken wrapping. The new cleanup-ownership paragraph has words stranded on their own lines: "before" / "their" / "creation." and "the isolated guardian" / "cleans" / "them on original-caller loss.". Rewrap the paragraph at whitespace to about 100 columns. | spec/kani/functional/FR-034-caller-death-ownership.md:130-134 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | The restored lease mutant cannot be run as specified without a production test bypass, which FR-034-AC-23 forbids. AC-24 and TC-049 step 12 require the fixture to "keep the original monitor-owning caller alive and close only its authenticated lease". This must happen after Dispatch and also before Dispatch while an authorization is queued, with no caller cleanup or controller INIT signal. In FR-034, the executor end and the unreaped monitor Child live in the same original-caller process. Every live-caller path the requirement defines (completion, explicit cancellation, refusal, timeout) also cancels through the claimed INIT pidfd. So no production-reachable path closes the lease alone while the caller survives. An implementer must either add a test-only hook, which breaks AC-23, or kill the caller, which brings in the parent-death teardown the fixture is meant to exclude. Name the production-reachable mechanism the fixture uses, for example a typed handle whose lease endpoint the owning caller can drop independently of the monitor and INIT handles. Or state the mutant oracle so that a production path can drive it. | spec/kani/functional/FR-034-caller-death-ownership.md:194, spec/kani/matrix/TC-049-caller-death-ownership.md:100-105, spec/kani/functional/FR-034-caller-death-ownership.md:193 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #299, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The Description now says FR-034 "extends the planned Linux PID-namespace containment code slice (PR #295), which is OPEN and unmerged", and that FR-028 AC-21 on `main` "remains mechanism-neutral". AC-14 cites "the planned containment code slice (PR #295)". The FR-034 matrix row, TC-049 index row and `spec/tests.md` Kani row each say "guardian CODE Gated on preceding containment code slice (PR #295)". `gh pr view 295` still reports OPEN, matching the labels. |
| FND-002 | fixed | A required explicit helper path replaces the undefined discovery rule. The Inputs state that Cargo does not build or install a dependency's binary, and the library consumer supplies the matched helper. The new FR-034-AC-25 refuses a stale helper by comparing the actual running library's build, protocol and lifecycle identity with the invoked executable, "not caller assertions or manually maintained tracking pins". TC-049 steps 4 and 8 exercise both checks. |
| FND-003 | fixed | The TC-049 index row now lists FR-028-AC-2, AC-3, AC-21, AC-24 and FR-017-AC-14, AC-24, AC-25. Planned FR-017 and FR-028 coverage rows name TC-049 and leave the TC-039 and TC-043 status unchanged. |
| FND-004 | fixed | AC-20 now reads "An already expired deadline causes no Dispatch and settles inconclusive with the timed-out reason as FR-028 AC-2 states". AC-1 now orders the lease, identity, readiness and Dispatch before any production backend instruction, so it no longer requires monitor ownership before the monitor exists. TC-049 step 11 matches. |
| FND-005 | fixed | FR-034 lines 17, 48, 51, 57 and 58 are rewrapped, and no prose line outside tables in FR-034 or TC-049 exceeds 100 columns at the candidate. A separate wrapping defect in new text is FND-007. |

### Round 2

Round 2 re-check of the second fix-round candidate of PR #299, covering FND-006 and FND-007 (the findings with no outcome yet) and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | The host-wide procfs membership walk is removed. Bootstrap recovery now uses bounded startup information or "a bounded walk of the live owned monitor's direct task children", with parent, start and namespace checked before the pidfd is opened. A recovered exact INIT pidfd's termination now suffices. An unrecoverable identity signals the pinned group, closes the exclusive lease and returns typed unconfirmed-cleanup refusal, and no host scan, signal success or monitor exit counts as confirmation. FR-034-AC-2, AC-7 and TC-049 step 5 now agree with AC-23 and TC-049 step 1. |
| FND-007 | fixed | The cleanup-ownership paragraph and every other changed prose paragraph in FR-034 and TC-049 are reflowed at whitespace. No prose line outside tables exceeds 100 columns, and no word is stranded mid-paragraph. |

### Round 3

Round 3 re-check of the third fix-round candidate of PR #299, covering FND-008 (the one finding with no outcome yet) and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | FR-034 now defines a production-reachable path, so the lease mutant needs no test hook. A non-clonable `CallerLease` owns only the executor stream, and a separate `RunOwner` keeps the unreaped monitor, the claimed INIT pidfd and identity, the captures and the original deadline. Both stay in the original caller. Explicit cancellation in InitReady or Dispatched calls the production `close_lease_and_observe` operation. It consumes and closes `CallerLease` and sends no independent INIT signal during a finite LeaseClosing phase clamped to the original deadline. It returns a typed `LeaseCloseObservation` (confirmed guardian termination, escalation required, or unavailable) before the normal driver escalates through the INIT pidfd. It never reports termination from stream EOF alone, and no proof is accepted from an escalation-required or unavailable observation. Urgent resource, deadline and identity cancellation keep their immediate INIT authority. AC-24 and TC-049 step 12 assert that observation and the pinned state of the escaped worker at that production boundary, before cleanup. An ignored-EOF mutant records escalation-required with a live worker and fails, and later escalation cannot turn that record into a pass. The spec forbids a cfg-test close hook, synthetic ownership and controller-blocked escalation. Because the operation is private, the fixture driving it must run inside the crate as the live caller. That is consistent with "no public cancellation API", and FR-034-AC-23 holds. |
