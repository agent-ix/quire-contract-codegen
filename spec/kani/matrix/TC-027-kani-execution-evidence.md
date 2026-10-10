---
id: TC-027
title: "Verify Kani obligation execution and its evidence"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-027: Verify Kani obligation execution and its evidence

## Description

Verify that a bounded Kani obligation runs against the installed backend, that
its outcome is read from the backend rather than defaulted, and that the
returned evidence describes the invocation that actually happened.

## Test Procedure

Classification, over reports shaped like the backend's own: a
successful run whose covers are all satisfied; a successful run whose cover is
unsatisfied; a successful run with a partial cover count; a failed run with a
concrete playback for the failed assertion and another for a satisfied cover; a
failed run with only a cover playback; a failed build with no report; a report of
success from a process that exited unsuccessfully; a successful run with no cover;
a failed unwinding assertion with playbacks present; a report in which an
unwinding check succeeded; and a successful run whose report lists zero successful checks.

Report parsing (FR-017-AC-12): parse real Kani captures (`tests/fixtures/kani-report/`, the
exported report and the stdout) of a verified run, a falsified run with a playback, an exhausted
unwind bound, a run whose only check is unreachable, a partly satisfied cover and a run with no
cover, and classify each; feed a console banner beside a contradicting report and confirm the report decides.

Refused reports (FR-017-AC-18): a report of another schema version, with an unknown check or
harness status, with a renamed top-level member, that is not JSON, and with zero or two harness
results are each refused with their own typed cause; a launcher stand-in that exits successfully
and exports nothing is refused, one that fails and exports nothing is `NoVerdict`.

Per-check view (FR-017-AC-20): classify the falsified and the exhausted-unwind captures and read each
check's id, class, file, line and status; a line of `unknown` is absent and a non-numeric line is refused; serialize a view and read the keys
`id`, `class`, `location { file, line }` and `status`.

Unnamed report storage (FR-017-AC-19): verify exact actual argv order and descriptor locator
`-Z unstable-options --export-json /proc/self/fd/N`, with actual N >= 5. Concurrent launches sharing
one target directory retain distinct descriptor authority; stale named files are untouched and
never read. Run genuine installed Kani 0.68 export through cargo inheritance into the mapped pipe,
including normal compiler outputs, proving the export neither seeks nor rereads its destination.
A failed pipe roundtrip stops CODE delivery and requires a measured spec revision; no runtime
named/direct-memfd fallback is permitted. These roundtrip tests are UNRUN, not satisfied by captured
JSON or source inspection of Kani's `std::fs::write` export.

Write exactly the report cap and then beyond it before Completed; inspect collector bounds and
concurrent pipe-capacity/kernel-backing charge, including unmapped backing. Over-limit writes must
cancel owned execution and yield single-run `KaniRunOutcome::Inconclusive` with
`KaniInconclusiveReason::MemoryExhausted`; record the report cap separately from the identity memory
ceiling. Batches retain whole-batch memory-exhausted refusal with no member classification. FR-029
maps ResourceExhausted; neither Failed nor truncated acceptance is allowed. Slow the collector and
fill the pipe while control and deadline events remain live; both slow and over-cap cases must end
within the original deadline without writer/collector wait cycles. Verify every writer copy,
including reopened procfd handles and descendants, closes before actual EOF, including O's closed
spawn writer and M termination/reap; no idle interval substitutes for EOF. Assert ordering of
authenticated Completed, original lease closure, confirmed inner teardown, bounded drain,
WRITE/GROW/SHRINK/SEAL seals, consumer F_GET_SEALS check and final OwnedFd handoff/read. The
separate report/control channel remains usable after lease closure. Fail consumer reading before
Completed and require bounded cancellation of O/I and final kernel storage release. Kill caller,
launcher, outer supervisor and guardian in separate and all-owner cases; kernel storage has no
pathname residue and is reclaimed after the last actual descriptor closes. Concurrent accounting,
seal races, inheritance/EOF and all-owner-death assertions are mandatory UNRUN CODE gates. Test
zero-byte EOF with successful and unsuccessful exits: missing-report refusal and NoVerdict
respectively, retaining independently established resource stops. Test partial/malformed nonempty
bytes before backend death without an established resource/deadline stop: typed report refusal,
never NoVerdict merely from nonzero exit. Repeat partial writes with independently established
memory/resource and deadline stops: preserve the existing single-run source outcomes and
whole-batch memory-exhausted/timed-out refusal without member classification. Test
valid nonempty report with normal/nonzero exits against existing classification rules. Reject a
received descriptor missing each seal independently before any consumer read. Compare actual owned
RSS plus finite caller-run buffers, reserved page-rounded pipe capacity and maximum memfd backing to
the original ceiling; attempt writer pipe resizing beyond the reservation and require capability
refusal if its bound cannot be enforced. No undercount is accepted from i_size, st_blocks, sparse
backing, unmapped pages or absent/overflowed accounting quantities. A success report listing a
failed, errored, undetermined or unknown check remains refused under FR-017-AC-18; class spelling
remains governed by FR-017-AC-20.

The output cap, capture failure, group cleanup and batching (FR-017-AC-14, FR-017-AC-21 to
FR-017-AC-25) are verified by TC-043, not here.

Refusals: request a run against an installation whose launcher is absent.

Routed scalar harness (FR-017-AC-11): build a routed exact-scalar harness with `generate_routed`
(`x + 1` over `Int[0, 9]`) and assemble the crate the way the driver does — the returned
`Cargo.toml` as the manifest and the harness's `rust.contents` as `src/lib.rs`, never the returned
`src/lib.rs`. Run it through `execute_kani_obligation`: a crate whose `src/lib.rs` lacks the harness
is `HarnessNotInCrate`, and the harness's covers classify a run the same way a contract harness's
do. In the `make kani` lane, generated with unwind 3, run it for real and confirm it is `Verified`
and its evidence carries `None` for obligation kind. Then narrow the same harness's checked domain
to `[0, 5]`, below the bound the oracle enforces, and confirm the run is `Falsified` with a
counterexample.

Caller Text-admission harness (FR-017-AC-26): use the generated FR-035
`Text[1,1]` harness and its typed caller identity. Run it through the same
production executor, singly and beside a compatible scalar harness in a
batch. A real successful report with a satisfied cover and successful
agreement assertion is `Verified`; an unsatisfied cover is
`CoverUnsatisfied` with its counts; the changed admission call of TC-050 is
`Falsified` with playback for this harness path. Check that evidence retains
the selected caller identity, path and `None` contract obligation kind, and
never assigns its playback to the other batch member. Present a crate whose
source lacks the harness, an absent launcher and a malformed report; each
keeps its existing typed refusal and produces no verified caller claim. A
report with no cover or a failed assertion without matching playback is
inconclusive. This route is Planned until FR-035 implementation supplies the
harness; no current TC-027 result is credited to it.

Generation/execution boundary: negotiate obligations that refuse before a harness exists —
unbounded, non-finite, model-dependent, frame and definedness-bearing items, and whole-request
refusals (`KaniObligationError`) such as an empty request, an unparsable subject path, an
out-of-range unwind bound and too many items — and confirm none of them produces a
`KaniObligationHarness`.

In the `make kani` lane, with a real installation: run the precondition, postcondition and
invariant harnesses of a healthy subject; run the postcondition harness against a seeded defect;
run a contract harness whose requires are jointly unsatisfiable; and run a harness against a crate
whose library source does not contain it.

## Expected Results

Only the all-covers-satisfied success is verified. The unsatisfied and partial
covers are cover-unsatisfied with their counts. The failed run with an
assertion playback is falsified carrying that playback and not the cover
playback; the failure with only a cover playback, the build failure, the
unsuccessfully exited process, and the success report with no cover property are
inconclusive with their own reasons. The failed unwinding assertion is
inconclusive as an exhausted bound and not falsified, and the succeeded
unwinding check in a listing is verified. The run with zero successful checks is inconclusive
with the vacuous-proof reason (FR-017-AC-13), unless it is a precondition harness, which its
cover summary decides.

For a caller Text-admission harness, the same classifications apply while the
evidence remains bound to its caller identity and harness path, never a
checked-package node. The caller route and its real-Kani outcomes are Planned
under FR-017-AC-26.

The launcher, exercised with real short-lived processes: a `Duration::MAX`
timeout does not panic; a capture thread told to stop returns what is already in its pipe, stops
while a write end is still open and idle, and stops within its drain limit while a straggler keeps
writing; and a run that times out has a real grandchild killed with it (FR-017-AC-15 through
FR-017-AC-17). The output cap (FR-017-AC-14) moved to TC-043.

Each capture's exported report parses to the expected typed report and classifies to verified, falsified with
the assertion playback passed through verbatim, exhausted bound, cover-unsatisfied 0 of 1,
cover-unsatisfied 1 of 2 and missing cover summary respectively (FR-017-AC-12).

The absent launcher is a typed tool refusal naming the launcher and its path.

The routed scalar harness's crate missing its `src/lib.rs` source is `HarnessNotInCrate`; its
cover classifies identically to a contract harness's; and, in the `make kani` lane, it runs with
its evidence carrying `None` for obligation kind.

Every generation-time refusal above exposes no harness, so `execute_kani_obligation` — which
takes a `KaniObligationHarness` — has nothing to run for it: a generation-time classification can
never surface as one of FR-017's execution outcomes because no code path converts one into the
other.

In the `make kani` lane the three healthy harnesses are verified, each with exit code zero and an
argument vector that begins, after the subcommand, with its harness identity's options; the seeded defect is
falsified with a concrete counterexample naming its harness symbol and a nonzero exit code; the
jointly unsatisfiable contract is cover-unsatisfied rather than verified; and the crate that does
not contain the harness is refused with no run.

## Implementation

`src/kani/classify.rs` unit tests for classification, including
`a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied` for
FR-017-AC-13, and the launcher tests in `src/kani/run/launch.rs` (`a_capture_thread_*`,
`a_timeout_of_duration_max_never_elapses_and_does_not_panic`,
`a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`) for FR-017-AC-15
through FR-017-AC-17. The two tests that asserted the silent tail an over-long stream used to
keep are deleted; the TC-043 refusal tests replace them;
the `src/kani/classify.rs` tests `tc_027_a_report_that_changed_shape_is_refused_not_classified`,
`tc_027_a_report_without_exactly_one_harness_is_refused`,
`tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status`,
`tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused`,
`tc_027_the_console_banner_never_decides_the_verdict` and the six `tc_027_real_kani_*` capture
tests for FR-017-AC-12 and FR-017-AC-18, with
`tc_027_the_per_check_view_has_one_serialized_wire_shape` (FR-017-AC-20) in
`src/kani/output/report.rs` and
`tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence` in
`src/kani/output/playback.rs`; the `src/kani/run/execute.rs` tests
`tc_027_execution_reads_only_the_report_its_own_run_exported`,
`tc_027_the_launch_exports_the_report_after_the_harness_options` and
`tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports`, the
`src/kani/run/report_file.rs` test `tc_027_the_report_is_read_bounded_and_refused_not_truncated`
and the `src/kani/classify.rs` test
`tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` for FR-017-AC-18 and
FR-017-AC-19, with the `src/kani/classify.rs` test
`tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified` (FR-017-AC-18) and the
`src/kani/output/report.rs` test `tc_027_a_class_spelled_cover_or_unwind_is_never_other`
(FR-017-AC-20); and `tests/it/kani_obligations.rs` for the refusals, the
generation/execution boundary and the `make kani` lane. The lane is `#[ignore]`d and runs through
`make kani` under a host-wide lock, because Kani and CBMC are memory-heavy and must run one harness
at a time.

The previously named-file AC-19 tests demonstrate the superseded internal allocation only; they do
not cover the unnamed writer bound, descriptor roundtrip or kernel reclamation above. No new
executable coverage is claimed by this specification amendment.


Evidence staging follows TC-049's explicit delivery allocation. Slice 1 uses ordinary normal-library
report/export, collector and kernel descriptor seams without fixture extension. Slices are internal
commit stages of ONE lifecycle CODE PR, not separate merges. IR-655 SPEC may use real unmerged O
source but must merge before its fixture CODE and the single CODE PR merge. All internal stages
and required pre-PR/full gates remain mandatory. Its assertions do not tag a mixed FR-034
collector/lifetime criterion whose O-origin or independent termination
witness remains owed to IR-655 slice 2. Both slices remain PLANNED/UNRUN; actual export or unit-test
compile/adaptation gaps are reported, not bypassed with named compatibility, helper identity
override, skipped tests or invented coverage. All guarantees precede complete IR-639/MVP acceptance.
