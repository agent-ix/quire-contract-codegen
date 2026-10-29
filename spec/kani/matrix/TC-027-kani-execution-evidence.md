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

NoVerdict diagnostics (FR-017-AC-32 to FR-017-AC-35). In the `make kani` lane,
assemble a crate whose `src/lib.rs` holds a generated harness byte for byte and, beside it, a
function that reads an undefined name, and run it through `execute_kani_obligation` (the
deliberately non-compiling harness). With single-run launcher stand-ins, run one that exits 1,
exports no report and prints nothing (the missing report); ones that exit 1 with no report and
print, on stdout, the Kani banner with `Checking harness proofs::check...` and
`Solving with CaDiCaL 3.0.0`, the banner with the `Checking harness` line alone, the banner with
`error: Failed to execute cargo (exit status: 101). Found 1 compilation errors.`, the banner alone,
a solver line in another wording after `Checking harness`, and a `Checking harness` line for a path
the launch did not pass; the `solver` stdout again with `-j 2` in the option vector; one with the
banner and Cargo's `error: could not compile` on stderr; one with a usage error on stderr and
nothing on stdout; and one that exports a report of success and exits 1 with
`Solving with CaDiCaL 3.0.0` on stdout. Put a marker more than 8192 bytes before the end of its
stream. Feed the excerpt builder redacted streams of exactly 8192 and of 8193 bytes, one whose final
line alone exceeds 8192 bytes, one 10000-byte line followed by a line feed, a 10000-byte line
followed by `error: last` and a line feed, one with a 4-byte character straddling the cut, invalid
UTF-8, ANSI escapes, a stream of only line feeds and a stream at its 8 MiB limit. Supply crate
directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home directory `/h/u` in the
request and print every input of FR-017-AC-35; repeat with home directory `/`, with none, under two
process `HOME` values, and with each redacted value changed. Classify a verified,
cover-unsatisfied, falsified, timed-out, memory-exhausted and other inconclusive run and serialize
each evidence.

CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40). Build reports
in the shape the measured real reports have: one result `proofs::check` with status `Failure` and no
checks, and one `error_details` entry `{ "harness_id": "proofs::check", "has_errors": true,
"error_type": "verification_failure", "failed_properties_type": "Other", "exit_status": <value> }`.
With a single-run stand-in that exits 1, prints `Checking harness proofs::check...` and
`CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, run each
exit status `exit_code_64`, `exit_code_15`, `exit_code_0`, `exit_code_-1`, `exit_code_2147483647`,
`exit_code_-2147483648`, `out_of_memory`,
`timeout`, `cbmc_exploded`, `exit_code_`, `exit_code_064`, `exit_code_+64`, `exit_code_-0`,
`exit_code_2147483648`, `exit_code_64x`, `EXIT_CODE_64`, a 64-byte, a 65-byte and a 100-byte
ASCII value, a value whose 64th byte falls inside a 4-byte character, `/h/u/ws/crate/x` with crate
directory `/h/u/ws/crate` in the request, a value holding an ANSI escape, and
`killed pid 4242 after 3.5s`. Then run a `Failure` report listing a failed assertion
with no playback on the console and exit status `properties_failed`; the same report with no
`error_details` member, with an entry for another harness only, with two entries for the harness,
with a null `exit_status`, and with a result that names no `harness_id`; an `exit_code_64` entry
beside a verified and a cover-unsatisfied report (exit 0), a falsified report with its playback and
an unwind-bound-exhausted report (exit 1), and a success report beside exit 1; a timed-out and a
memory-exhausted run; and a report whose `exit_status` is the number `64` and one whose
`exit_status` is the string `"\ud800"`. Run three stand-ins with
one `Failure` report and exit and the entries `exit_code_64`, `out_of_memory` and `cbmc_exploded`,
each printing different stream text, and map each evidence through FR-029 with and without its
`failureCause` and `diagnostics`. In the `make kani` lane, put a generated harness in a crate whose
manifest carries `[package.metadata.kani]` with `unstable = { unstable-options = true }` and
`flags = { cbmc-args = ["--no-such-option"] }` (the manifest route generated oracle crates already
use for CBMC arguments), leave its option vector unchanged, and run it through
`execute_kani_obligation`. The option is not appended to the option vector: the generator appends
`-Z unstable-options --export-json <path>` after the options, and `--cbmc-args` there would take
those arguments as CBMC's, so no report would be exported.

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

ScalarLimits owner identity (FR-017-AC-51), **Planned / UNRUN** with IR-17 kernel adoption:
compile a consumer that constructs `quire_contract_codegen::ScalarLimits` using only CG paths
and its unchanged fields, types the `KaniScalarNativeRequest::scalar_limits` accessor with that
same CG type, and assigns the value directly to `quire_exact::ScalarLimits`. Require actual type
identity without conversion, wrapper or compatibility shim; a source import alone is insufficient.
Current-main delivery is not credited by the specification amendment.

Private native observer custody (FR-017-AC-46/48/49/52/53), **Planned / UNRUN**:
construct the isolated resolver-2 observer package inside the private snapshot, with the example
root at `../src/bin/ir791_native.rs`. Verify that this is the single canonical adapter, loading
`../lib.rs`, with no wrapper, copy or symlink. Compare all four original emitted artifacts
byte for byte under FR-017's rename map, explicitly including `source-maps/lib.json` and the
unmodified production manifest, plus the original harness and record. Inspect the production
and companion dependency graphs: only the observer's exact dev edge enables `test-support`;
both observer dev crates use their owning source spellings. Compile the required feature-free
production library/proof targets, independently freshly resolve and compile the observer example,
and remove its dev feature in a negative control that must fail to compile. Do not use a copied
parent lock or cached resolution in place of the fresh resolver regression. Independently assert
both literal `quire-contract-runtime` and `quire-exact` names and authoritative repository sources
in each staged lock, and compare each owning package's resolved source identity between the two
freshly resolved locks; do not derive both package expectations from the renderer's list or read
codegen's build lock for an expected revision. Report a fresh dependency-resolution or cross-lock
consistency failure as a failure, without a downgrade or forced build revision.

Run the actual bounded native command with `--manifest-path observer/Cargo.toml --example
ir791_native` from the sealed root and its target outside that root. Verify actual nonzero ordered
charges, limits, counters and typed incomplete outcomes from the real meter. Run the real production
proof with its retained harness/ordered options and all claims/covers, including the existing
non-verified/no-native controls; no `--all-targets` compile assumption or skipped proof substitutes
for those checks. Exercise cancellation and limits at the companion resolution/native boundaries
through the existing bounded-launch controls. Extend complete path/byte measurement and comparison
to all ten v3 entries before proof, after proof and after native execution. Mutate or replace
each entry independently and inject an extra file in each package root at both execution
boundaries; require a typed failure and no receipt. Retain the existing source-tree write controls.
Check non-UTF-8 bytes separately in each lock, the observer manifest and canonical adapter, with
`snapshot_io` / `EnvironmentFault` and no receipt. Extend the entry-by-entry byte comparison, cleanup-survival and cleanup-failure controls below
to the whole staged set. Neither this procedure nor source inspection establishes Cargo acceptance of
the nested isolated package/external example path, fresh resolution, or sealed execution; those
checks remain UNRUN until the implementation executes them. The separate Unicode resolution
failure and IR-705/IR-844 qualification are not resolved by this amendment.

## Concrete playback execution (IR-917, planned)

These controls qualify the public playback entry and its installed
`amount_current = 999` execution; TC-025 step 11 owns the separate
unconstrained clause proof, mutant Falsified verdict and in-domain witness.

1. From a seal-validated generated V2 clause harness, append one parseable
   `#[test]` function immediately before its module's closing brace, outside
   the proof function, and name its exact libtest `module::function` path. Treat
   its attributes, signature, body and separators as one appended byte span;
   removing only that span must restore the unique full sealed rust artifact,
   including its header, oracle source and generated module, byte for byte.
   Through the public
   `execute_kani_concrete_playback` entry, inspect the exact-selection
   `kani playback --lib -Z function-contracts -Z concrete-playback -- <test_name> --exact --nocapture`
   launch. With a launcher stand-in that reports the named test as the sole
   pass, require `Completed { succeeded: true }`, the actual exit code,
   complete bounded stdout/stderr and observed memory; do not classify it as
   `Verified` (FR-017-AC-58, FR-017-AC-59).
2. Submit each non-V2 harness kind (Contract, Scalar, StateFrame and Collection)
   and require `ConcretePlaybackUnsupportedHarness { harness_path }` before
   backend launch. Alter the retained V2 generated harness source or identity
   record and require `HarnessProvenanceMismatch`. Separately omit or duplicate
   the appended test, supply malformed Rust or a function whose name or module
   disagrees with `test_name`, alter the generated source beside the appended
   test, remove or make `src/lib.rs` unreadable, or place the test so removing
   the exact inserted byte span does not restore one unique full sealed rust
   artifact. Also change the embedded oracle bytes outside the module while
   retaining the appended test; require the same source refusal before launch.
   Each source case requires `ConcretePlaybackSourceMismatch` and zero backend
   calls (FR-017-AC-57).
3. Let a stand-in exit successfully after running zero matching tests, name a
   different passing test, print two libtest summaries (including an unrelated
   zero-test summary), or interleave `--nocapture` text through the selected
   test-result line. Require `ConcretePlaybackTestNotObserved { test_name }`,
   never successful playback. Accept one intact exact selected-test pass line
   (`test <name> ... ok` or `test <name> - should panic ... ok`) and exactly one one-pass/zero-failure
   summary from the `--lib` target. Let the selected test exit unsuccessfully
   at an assertion, and separately fail compilation or a dependency before
   test execution: each retains `Completed { succeeded: false }`, actual exit
   code and bounded streams, but neither raw outcome alone attributes failure
   to a generated assertion or claims a proof verdict (FR-017-AC-59).
4. Reuse the existing bounded-launch controls for wall-clock timeout,
   cancellation, memory exhaustion, output over-limit/unread and owned
   teardown with the concrete-playback entry. Require distinct `TimedOut`,
   `Cancelled { source, memory }`, `MemoryExhausted` and existing typed output
   refusals; verify the backend tree is gone and no case is classified as a
   proof outcome (FR-017-AC-58).
5. With installed Kani, call this exact public entry for a healthy V2 clause
   playback and for the plus-one owner mutant's `amount_current = 999`
   expected-panic playback. Require the selected test's observed pass line and
   sole compatible summary in both; the mutant's test must execute the same
   sealed generated arithmetic assertion and identify its unique check message
   or source location. Confirm the selected fixture's subject returns true
   normally at 999 and the generated harness reaches that arithmetic assertion
   before the clause verdict; an assumption, clause/postcondition or cover
   failure cannot satisfy this control. Bind the mutant's restored sealed
   generated source and body, identity record, oracle bytes, clause identity
   and native expectation to TC-025 step 11's unconstrained mutant run.
   Retain bounded output and memory observations and prove
   owned teardown after each run. Inspect the installed command and require
   `-Z function-contracts`, the option present in this harness's proof-time
   `adapter_options`. Keep the mutant's separate unconstrained Kani Falsified
   run and in-domain playback under TC-025 step 11; neither this native test
   nor a stand-in supplies that proof verdict (FR-015-AC-41, FR-017-AC-60).

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

NoVerdict diagnostics (FR-017-AC-32 to FR-017-AC-35): the non-compiling harness
exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict`, with phase `compile`, the
rustc error code and Kani's `error: Failed to execute cargo` line in the stdout excerpt and Cargo's
`error: could not compile` line, its crate path as `<CRATE_DIR>`, in the stderr excerpt; the silent
missing-report stand-in is `NoVerdict` with phase `launcher` and two empty excerpts with
`omittedBytes` 0. The stdout stand-ins are, in the order listed, `solver`, `backend`, `compile`,
`unknown`, `backend` and `unknown`; the `-j 2` run is `unknown`; then `compile`; `launcher`; and
`unknown` for the report of success beside exit 1. A marker outside the excerpt window still
decides the phase. The 8192-byte stream is whole with `omittedBytes` 0; the 8193-byte stream keeps
its longest line-aligned suffix of at most 8192 bytes with `omittedBytes` the bytes before it; the
long final line keeps at most 8192 bytes ending at its last byte from a character boundary; the
10000-byte line with its line feed keeps its last 8192 bytes ending in the line feed; the line
followed by `error: last` keeps exactly `error: last` and its line feed; no character is split;
invalid UTF-8 becomes U+FFFD; escapes are gone; the line-feed-only stream is kept; no excerpt
exceeds 8192 bytes. Every FR-017-AC-35 input yields its stated output; home `/` gives no `<HOME>`,
no home leaves the `.kani` path as `<PATH>`, the two process `HOME` values give the same record,
and the runs that differ only in redacted values serialize byte-identical records as
`diagnostics { phase, stdout { text, omittedBytes }, stderr { text, omittedBytes } }`. The
`compile_fail` rustdoc example that deserializes the record fails to compile while its
`serde_json::to_string` control compiles. No evidence of a non-`NoVerdict` run has a `diagnostics`
member, apart from the `FailedWithoutCounterexample` evidence of a CBMC failure cause below.

CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40): every exit-status run
is `Inconclusive` `FailedWithoutCounterexample`. `exit_code_64`, `exit_code_15`, `exit_code_0` and
`exit_code_-1`, `exit_code_2147483647` and `exit_code_-2147483648` carry
`failureCause { kind: "exit_code" }` with `code` 64, 15, 0, -1, 2147483647 and -2147483648;
`out_of_memory` and `timeout` carry their own `kind`; `cbmc_exploded` and each malformed
`exit_code` spelling carry `{ kind: "unknown", exitStatus }` with the value verbatim; the 64-byte
value is kept whole, the 65-byte and 100-byte values keep their first 64 bytes and the straddling
value is cut before the character; the path value is `<CRATE_DIR>/x`, the escape is removed, and `killed pid 4242 after 3.5s` is
`killed pid <PID> after <DURATION>`. Each of them
carries a `diagnostics` record of phase `unknown` whose stderr excerpt holds
`Unknown option: --no-such-option`. The `properties_failed` run carries
`{ kind: "properties_failed" }` and no `diagnostics` member; the runs with no entry, another
harness's entry, two entries, a null exit status and no `harness_id` are
`FailedWithoutCounterexample` with neither member. The verified, cover-unsatisfied, falsified,
unwind-bound-exhausted, `NoVerdict`, timed-out and memory-exhausted runs keep their outcomes and have
no `failureCause` member. The numeric `exit_status` and the lone-surrogate string are each refused as malformed with no
evidence. The
`compile_fail` rustdoc example that deserializes `KaniFailureCause` fails to compile while its
`serde_json::to_string` control compiles. The three runs differing in cause and stream text have
equal outcome, checks, SUCCESS count and playback, equal evidence in every field but
`failureCause`, `diagnostics`, `memory` and the report-path argument, and FR-029 terminal value
`Incomplete(ResourceExhausted)` for the `out_of_memory` run and `Failed` for the `exit_code_64` and
`cbmc_exploded` runs (FR-029-AC-32), as asserted by
`cbmc_failure_terminal_value_reads_the_retained_cause` in `tests/it/kani_obligations.rs` and
`bounded_cause_changes_only_terminal_projection_and_retains_evidence` in
`tests/it/kani_terminal_private.rs`. The real run whose manifest passes `--no-such-option` to CBMC exits
unsuccessfully, exports a report and is `FailedWithoutCounterexample` with
`{ kind: "exit_code", code: 64 }` and a stderr excerpt holding `Unknown option: --no-such-option`.

The absent launcher is a typed tool refusal naming the launcher and its path.

The routed scalar harness's crate missing its `src/lib.rs` source is `HarnessNotInCrate`; its
cover classifies identically to a contract harness's; and, in the `make kani` lane, it runs with
its evidence carrying `None` for obligation kind.

The paired routed-scalar run with operand `4` produces Kani `Verified` and an actual native
`Outcome::Completed(5)`. Its receipt retains the selected scalar identity, operand vector, exact
request limits used by the native meter, captured staged entries and Kani evidence. Any unsuccessful proof, native execution fault,
malformed native response or changed staged tree yields no receipt. Request-context mismatch is
typed and occurs before either child is started; changing an otherwise valid operand produces a
receipt for that exact ordered request, which cannot be substituted for a prior receipt.

Paired routed-scalar proof/native receipt (FR-017-AC-27 to AC-31): generate a genuine routed
`x + 1` harness for an admitted integer domain, retain its complete `oracle_artifacts` and
request `ScalarLimits` and request concrete operand `4` within the generated range. In the
`make kani` lane, require the paired runner to stage the package and harness, run real Kani on the
staged harness, and report `Verified`. Require the runner to execute the staged public oracle at
that operand with those exact limits; the opaque receipt must contain actual typed
`Outcome::Completed(5)`, the exact operand and limits, the scalar identity, captured staged entries and Kani evidence. No result derived from the Kani verdict or supplied by the test caller
counts.

Order-sensitive binary receipt (FR-017-AC-31): generate a genuine routed subtraction operation
with two symbolic operands over a domain admitting both concrete vectors `[4, 1]` and `[1, 4]`,
and one exact request `ScalarLimits` value.
Run each vector through the paired runner separately in the `make kani` lane. Each run must execute
real Kani and report `Verified`, then invoke the staged public oracle with the exact ordered vector
and request limits. Require native `Outcome::Completed(3)` for `[4, 1]` and
`Outcome::Completed(-3)` for `[1, 4]`; each opaque receipt must carry its own exact ordered vector
and actual typed result. The receipts retain the same staged path and byte set because the proved and executed bytes are unchanged. Both runs and both receipt assertions are
mandatory; neither the unary `x + 1` case nor an optional binary case substitutes for them.

Request-boundary controls (FR-017-AC-28): before either child starts, separately change the
generated library bytes, generated artifact path, selected scalar identity, selected oracle symbol,
operand count and operand value outside its retained range. Each inconsistent request must return
a typed mismatch/refusal, produce no receipt, and leave a launch marker absent. For the limits
control, first run the same generated harness, artifacts and operand `[4]` with the normal request
limits; require Kani `Verified`, native `Outcome::Completed(5)`, and the exact request limits in the
receipt and meter observation. Then change only `result_units` to `0` and repeat against the same
generated staged bytes. Require Kani `Verified` again, the same staged path and byte set as the
`Completed(5)` receipt, and `Outcome::Incomplete` with `limit_kind = ResultUnits`, `limit = 0`,
`consumed = 0`, `next_charge = 1`, and charge point `IntegerArithmeticResultRetain`. The meter
observation must report zero consumed result units and no admitted result-retain charge. In both
runs, assert that the exact supplied `ScalarLimits`, the limit value observed at meter
initialization, and the receipt's returned limits agree. This proves supplied-to-used-to-returned
equality while the same staged artifact bytes are retained across distinct native outcomes; it does not
assert that either request matches the upstream generation context.
For a two-operand scalar identity, change only the
operand order and confirm the receipt retains the exact changed order; it must not be possible to
reuse an earlier receipt as evidence for the reordered request. The mandatory AC-31 binary runs
must demonstrate this with the distinct subtraction results above. A different in-range operand
vector may be run as a distinct request, but its receipt must bind that vector separately from the
unchanged staged artifact bytes. No caller-supplied artifact digest can make an inconsistent request acceptable. The IR-773 consumer's comparison against its retained original limits, and its
local refusal/skip behavior on mismatch, are consumer-owned checks outside this CG test. If the
driver has no defined typed outcome for a mismatch, its owner must specify and plan that outcome
before implementation. No driver adversarial coverage is claimed here.

Scalar native error discriminant (FR-017-AC-41 to AC-44): build one `KaniScalarNativeError` of each
of the ten variants, the nested `Kani` error once for each `KaniExecutionRefusal` variant, and
`ProofNotVerified` once for each evidence outcome.
(1) Assert `code()` of each equals its table code, that the ten codes are pairwise distinct, that
`Kani` gives `kani_execution` for every nested refusal, and that the type, the class enum and the
code constants are reachable at the crate root. (2) Assert `class()` of each variant, of each nested
`Kani` refusal, and of `ProofNotVerified` for `Falsified`, `CoverUnsatisfied`, each inconclusive
reason, `Cancelled`, `Verified` (an internal inconsistency, requiring `EnvironmentFault` and no panic), and `FailedWithoutCounterexample` with no `failureCause`, with `properties_failed` (a real single-run stand-in of AC-38's failed assertion without playback, requiring `CompletionRefusal`) and with each of `timeout`, `out_of_memory`, `exit_code` and `unknown` (requiring `EnvironmentFault`), against
the FR-017 table. Produce real errors by the runner for a request mismatch, an oracle stop and a falsified proof, and require `CompletionRefusal`;
produce a real single run cancelled by its retained source and a real single run stopped by its
wall-clock timeout (launcher stand-ins), each `ProofNotVerified`, and require `EnvironmentFault`
with the evidence outcome unchanged; through an in-crate seam run each of four separate staged-lock
failure cases: make one staged lock unreadable, omit an owning package, change its repository source,
and give the two staged locks different owning package source identities. Require `DependencyLock` /
`EnvironmentFault` and no receipt for every case;
inject a changed staged byte, a native launch failure and malformed helper output, and require
`EnvironmentFault`. (3) Feed malformed helper output and require `NativeResult` / `native_result` /
`EnvironmentFault`, with no receipt. (4) Keep a frozen list of all variants and codes and of the mapping to class: one class for a fixed
variant, and for `ProofNotVerified` and `Kani` the class of every `KaniRunOutcome`,
`KaniInconclusiveReason`, `KaniFailureCause` and nested-refusal value, with exhaustive `match`es (no
wildcard) over those types inside the defining crate that stop compiling when one is added, and
assert the nested `OutputOverLimit` and `OutputUnread` codes are unchanged. (5) Non-exhaustive and
closed class: a rustdoc `compile_fail` example matches `KaniScalarNativeError` exhaustively without
a wildcard arm and must fail to compile, with a compiling control that adds the wildcard arm; the
class enum is closed by a compiling control that matches `KaniScalarNativeErrorClass` exhaustively
with exactly `CompletionRefusal` and `EnvironmentFault` and no wildcard, together with the absence of
`#[non_exhaustive]` on it; the doctests run by `cargo test --doc`, the pattern of the AC-38
serialize-only check.
These controls are implemented in the scalar runner's unit tests and public rustdoc examples.
The crate-root private scalar-native tests exercise cancellation and timeout with bounded
single-run launcher stand-ins. Their real-Kani seeded false-bound fixture keeps the routed
package and embedded oracle intact, changes the asserted bound before the normal renderer and
origin seal, and requires the paired runner to return real `Falsified` evidence without starting
native execution. It is a deliberately defective proof property, not a naturally failing admitted
production claim. A separate real-proof control selects a bounded native-helper protocol
stand-in through an immutable private installation dependency; it records the actual native
arguments and emits an oracle-stop record, and the public paired runner must return
`OracleStop` / `oracle_stop` / `CompletionRefusal`. This exercises runner propagation without
claiming an actual runtime-produced stop for a supported admitted integer operation. Fabricated
proof evidence is confined to the exhaustive classification table.

Snapshot controls (FR-017-AC-29 and AC-30): measure the staged path/byte set for the complete
generated package (including source map), retained harness, canonical adapter, observer manifest
and both independently generated dependency locks;
attempt a source-tree write from each child execution boundary, and require that neither child can
change it. Inject a changed byte in each execution-relevant staged item, a replaced path and an
unexpected file after staging and across the Kani/native boundary;
each must produce a typed failure and no receipt carrying the earlier staged byte set. Check that changes
to the concrete operand vector, its order, request limits or scalar identity cannot be represented
by reusing an earlier receipt. Inspect the public receipt surface to verify that it has read-only access to
the actual Kani evidence, exact scalar invocation context, captured staged entries and typed native
outcome, and that callers cannot construct, deserialize or mutate it. The generated package is
staged for this run and is not copied into a repository.

Every generation-time refusal above exposes no harness, so `execute_kani_obligation` — which
takes a `KaniObligationHarness` — has nothing to run for it: a generation-time classification can
never surface as one of FR-017's execution outcomes because no code path converts one into the
other.

For the scalar error discriminant, each variant has its table code and class, the codes are distinct
and frozen, the three refusal variants, the nested request-fixed `Kani` refusals and a `ProofNotVerified` carrying a
property or proof verdict are `CompletionRefusal`; the five fixed fault variants, every other nested refusal and a
`ProofNotVerified` of a cancelled, timed-out, memory-exhausted or no-verdict run are
`EnvironmentFault`, and no identity encoder runs in the paired runner.

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
`tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` for FR-017-AC-18 (none of
them covers FR-017-AC-19, which is Planned), with the `src/kani/classify.rs` test
`tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified` (FR-017-AC-18) and the
`src/kani/output/report.rs` test `tc_027_a_class_spelled_cover_or_unwind_is_never_other`
(FR-017-AC-20); and `tests/it/kani_obligations.rs` for the refusals, the
generation/execution boundary and the `make kani` lane. The lane is `#[ignore]`d and runs through
`make kani` under a host-wide lock, because Kani and CBMC are memory-heavy and must run one harness
at a time.

FR-017-AC-32 to FR-017-AC-35 are covered by: the `make kani` lane test
`tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile` in
`tests/it/kani_obligations.rs` (real Kani on a crate holding the harness beside an item that does
not compile); the `src/kani/run/execute.rs` stand-in tests in `batch_tests::no_verdict_diagnostics`
(`tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher`,
`tc_027_marker_stand_ins_decide_the_failure_phase`,
`tc_027_a_runs_streams_are_redacted_against_the_requests_directories`,
`tc_027_runs_differing_only_in_redacted_values_serialize_identical_records`,
`tc_027_the_same_run_under_two_process_homes_gives_the_same_record` and
`tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member`); and the `src/kani/output/diagnostics.rs`
tests of the phase rules, the excerpt bounds and every redaction pair. The serialize-only check is
the rustdoc `compile_fail` example on `KaniNoVerdictDiagnostics`, run by `cargo test --doc`, with
its compiling `serde_json::to_string` control beside it.

The CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40) is covered by the bounded
report controls in `tests/it/kani_terminal_private.rs`, which reuse
`batch_tests::no_verdict_diagnostics::failure_cause` in `src/kani/run/execute.rs`, and the
remaining execution tests `tc_027_the_cause_is_kept_for_no_other_outcome` and
`tc_027_a_malformed_exit_status_still_refuses_the_report`. For the FR-029 terminal value,
`cbmc_failure_terminal_value_reads_the_retained_cause` in `tests/it/kani_obligations.rs`
passes `&KaniExecutionEvidence` to `run_terminal_value`: `out_of_memory` gives
`Incomplete(ResourceExhausted)`, `timeout` gives `Incomplete(TimedOut)`, and `exit_code_64` and
`cbmc_exploded` give `Failed` (FR-029-AC-32, FR-029-AC-33). The map never reads the diagnostic
record. The cause is also covered by the tests of the exit-status
vocabulary, the 64-byte cut and the one-entry rule in `src/kani/output/report.rs`; by the serialize-only
check, which is the rustdoc `compile_fail` example on `KaniFailureCause` run by `cargo test --doc`
beside its `serde_json::to_string` control and the `tc_027_the_cause_implements_no_deserialize`
autoref probe; and, in the `make kani` lane, by
`tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause` in
`tests/it/kani_obligations.rs`, which runs real Kani on a crate whose manifest hands CBMC
`--no-such-option`.

The previously named-file AC-19 tests demonstrate the superseded internal allocation only; they do
not cover the unnamed writer bound, descriptor roundtrip or kernel reclamation above. No current
main executable test covers FR-017-AC-27 to AC-31; those paired-run and mutation-control tests are
required as part of the IR-791 implementation, not credited by this specification amendment.


Evidence staging follows TC-049's explicit delivery allocation. Slice 1 uses ordinary normal-library
report/export, collector and kernel descriptor seams without fixture extension. Slices are internal
commit stages of ONE lifecycle CODE PR, not separate merges. IR-655 SPEC may use real unmerged O
source but must merge before its fixture CODE and the single CODE PR merge. All internal stages
and required pre-PR/full gates remain mandatory. Its assertions do not tag a mixed FR-034
collector/lifetime criterion whose O-origin or independent termination
witness remains owed to IR-655 slice 2. Both slices remain PLANNED/UNRUN; actual export or unit-test
compile/adaptation gaps are reported, not bypassed with named compatibility, helper identity
override, skipped tests or invented coverage. All guarantees precede complete IR-639/MVP acceptance.

Staged scalar custody and meter limits (FR-017-AC-45 to AC-50), **Planned / UNRUN**:
Run the paired runner on a genuine routed `x + 1` package. Assert that the receipt contains no generated-content `DigestRecord` or scalar proof-content digest. Compare the ten captured staged entries with the AC-46 path set and original emitted bytes under the rename map. Mutate, replace or add an entry before and after each execution boundary and require a typed refusal with no receipt. Verify that the entries remain readable after snapshot cleanup, that cleanup failure only adds a diagnostic, and that no entry accessor reads the filesystem. Compare driver-owned originals directly by path and bytes. Verify the meter-observed `ScalarLimits` equal the request and `scalar_limits()`; a helper reporting a different limit yields `NativeResult` / `EnvironmentFault` and no receipt. These extensions remain Planned; current test names or trace tags do not establish that they ran.

## Admitted native integer qualification (IR-705 option 2)

After reviewed [kernel FR-384](ix://agent-ix/quire-exact/FR-384) refinement evidence, regenerate the authentic TC-027 healthy/bound-mutant controls, TC-025 arithmetic mutant, IR-917 clause implication and IR-948 V2 contract pre/post/invariant profiles through the public generator under [AD-006](../../assurance/AD-006-admitted-native-integer-oracles.md). Record the reviewed kernel identity and native-capability selection without copying runtime source or mutating historical fixtures. Preserve original populations, semantic bounds, clauses, independent oracle/expectation, compiled subject control, assertions and final completion cover. Generated bytes and ABI may change for the admitted production route and must be disclosed.

Run one backend job at a time with the original per-profile solver, unwind and time settings and a measured 16 GiB memory envelope. Require healthy Verified with cover satisfied and every specified mutant Falsified with concrete original-domain native playback; a successful compiler or generation Boolean is insufficient. Record actual terminal outcome, peak and confirmed cleanup per proof. Diagnostic runs with larger authorized ceilings are attributed separately and cannot establish 16 GiB fit. Exercise real cumulative Meter exact/one-less limits, cancellation, each named injected denial and domain refusal; preserve all counters, ordered admitted prefix and complete stop fields. No skipped/unknown/resource-exhausted healthy proof is accepted. These prospective procedures add no current proof credit.

For IR-948, retain the authentic impossible-precondition package and require CoverUnsatisfied at the same 16 GiB ceiling and original unwind8. This is an adverse cover control rather than an arithmetic falsification witness. Healthy Verified alone does not complete the profile pair.
