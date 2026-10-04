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

Report file (FR-017-AC-19): the launch's arguments are the harness options then the export flags;
two launches name different report files, a report another run left in the target directory is
neither read nor removed, runs sharing one target directory concurrently each read their own, and
a run removes its own file; a file over the read bound is refused. A success report listing a
failed, errored, undetermined or unknown check (any class) is refused for every obligation kind
(FR-017-AC-18), and a class spelled `cover` or `unwind` is always that class (FR-017-AC-20).

The output cap, capture failure, group cleanup and batching (FR-017-AC-14, FR-017-AC-21 to
FR-017-AC-25, IR-277) are verified by TC-043, not here.

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

`src/kani_execution.rs` unit tests for classification, including
`a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied` for
FR-017-AC-13, and the launcher tests (`a_stream_longer_than_the_capture_limit_keeps_only_its_tail`,
`a_capture_thread_*`, `a_launcher_printing_more_than_the_limit_completes_with_bounded_text`,
`a_timeout_of_duration_max_never_elapses_and_does_not_panic`,
`a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`) for FR-017-AC-15
through FR-017-AC-17. Two of them,
`a_stream_longer_than_the_capture_limit_keeps_only_its_tail` and
`a_launcher_printing_more_than_the_limit_completes_with_bounded_text`, still carry the tag
FR-017-AC-14 and assert the silent tail that AC-14 now forbids; they are not evidence for it, and
the IR-277 code change deletes them in favour of the TC-043 refusal tests;
the `src/kani_transcript.rs` tests `tc_027_a_report_that_changed_shape_is_refused_not_classified`,
`tc_027_a_report_without_exactly_one_harness_is_refused`,
`tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status`,
`tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused`,
`tc_027_the_per_check_view_has_one_serialized_wire_shape` (FR-017-AC-20),
`tc_027_the_console_banner_never_decides_the_verdict`,
`tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence` and the six
`tc_027_real_kani_*` capture tests for FR-017-AC-12 and FR-017-AC-18; the
`src/kani_execution.rs` tests `tc_027_execution_reads_only_the_report_its_own_run_exported`,
`tc_027_the_report_is_read_bounded_and_refused_not_truncated`,
`tc_027_the_launch_exports_the_report_after_the_harness_options`,
`tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports` and
`tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` for FR-017-AC-18 and FR-017-AC-19, with the
`src/kani_transcript.rs` tests `tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified`
(FR-017-AC-18) and `tc_027_a_class_spelled_cover_or_unwind_is_never_other` (FR-017-AC-20); and `tests/it/kani_obligations.rs` for the refusals, the
generation/execution boundary and the `make kani` lane. The lane is `#[ignore]`d and runs through
`make kani` under a host-wide lock, because Kani and CBMC are memory-heavy and must run one harness
at a time.
