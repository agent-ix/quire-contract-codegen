---
id: SR-11908
title: "Test-oracle strength of IR-809 tests (PR 406)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/output/diagnostics.rs tests, src/kani/run/execute.rs batch_tests::no_verdict_diagnostics, tests/it/kani_obligations.rs IR-809 tests"
review_set: subset
---

## Summary

Ticket: IR-809. For each new test, judged whether its failure case exercises the intended path or
would also pass on a setup error or a no-op. Read the coder's mutation diffs (sets 1 to 4) and red
logs 1 to 7: each mutation turned its target tests red. Re-ran two mutations of my own. The first
made `finish_group` pass `report_exported = false` and was caught by
`tc_043_a_group_report_decides_between_unknown_and_no_record`. The second reduced the path-token
opener and closer sets and survived all 27 tests (FND-003). Strong oracles include the HOME probe:
if the probe filter is wrong, the output file is missing and the test panics; if `HOME` were read,
`<PATH>` would be missing. Also strong: the `compile_fail` doctest beside its compiling control,
the over-limit tests, which `unwrap_err` and so fail if evidence is returned, and the
marker-table stand-ins, which assert `NoVerdict` before the phase.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | In `tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics`, the "equal with and without `diagnostics`" half cannot fail. `run_terminal_value(outcome, success_checks, settlement)` never receives the evidence or its record, and `with` is computed after `diagnostics.take()`, so both calls are the same call on the same values. Only the `Failed` assertion has power: red7 is caught there. The property holds by the function's signature, so TC-043's claim to cover it should cite the signature, not this comparison. | tests/it/kani_obligations.rs:2223-2235; src/kani/terminal.rs:90-94 | correct-requirement-no-evidence |
| FND-002 | low | The AC-32 real-Kani fixture replaces the subject with `NON_COMPILING_SUBJECT`, which defines no `withdraw`, so the generated harness fails to compile too: red6.log shows `error[E0425]: cannot find function withdraw` beside the intended `undefined_name` error. AC-32 describes the generated harness beside one non-compiling item. The test still reaches phase `compile`, but it does not isolate the beside-item failure. Use `HEALTHY_SUBJECT` plus the broken function. | tests/it/kani_obligations.rs:2168, :2252 | correct-requirement-no-evidence |
| FND-003 | low | No test pins the path-token delimiters beyond space, newline, `(`, `)` and `:`. A mutation that drops the openers `[ { ' " backtick =` and the closers `] } ' " backtick , ;` passes all 27 diagnostics tests. These sets are a spec-body rule (FR-017 path-token bullet), not an AC input, so `"/h/u/x"`, `=/opt/a/b` or `[/opt/a/b]` could regress silently. | src/kani/output/diagnostics.rs:325-339; spec/kani/functional/FR-017-kani-execution-evidence.md:429-434 | correct-requirement-no-evidence |

## Verdict

Low findings only. The other new tests have oracles that a no-op or a setup error would fail.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (818d8c6): d5c6bd6: the tautological with/without comparison is removed; the test keeps the Failed assertion, which has power (red7). |
| FND-002 | fixed | Round 1 (818d8c6): d5c6bd6: fixture is HEALTHY_SUBJECT plus BROKEN_ITEM; test asserts exactly one E0425, the undefined_name error, and no withdraw error. Re-ran under real Kani 0.68 in the 16G scope: passes. |
| FND-003 | fixed | Round 1 (818d8c6): d5c6bd6/8923356: opener and closer tests over every listed character plus non-listed controls; reviewer re-ran the reduced-set mutation and both tests go red. |


## Follow-up review (disposition pass 3)

Follow-up of the frozen pushed candidate for IR-809 PR #406 on current main after #403.
The 13 replayed commits retain the reviewed change; range-diff shows only inherited main
annotation/mechanism context. The final 32 test annotation conversions preserve every
criterion id and normalize to the same code after removing trace attrs/import, doc trace
lines and the one production documentation citation. No behavior changed in that conversion.
The six spec patch payloads (AD-004, interface-001, FR-017, TC-027, TC-043 and tests.md)
are byte-identical to the original reviewed candidate. There is no new normative change
requiring a fresh spec direction or another spec analysis artifact. Existing spec review
SR-11900..SR-11904 is context; this follow-up verifies the implementation and bindings.
No original finding text or disposition row was edited. Every prior finding is terminal;
there are no prior still-open/undisposed findings, so this round appends no outcome row
and publishes `dispositions: []`. No new finding was identified.

Test-oracle follow-up: PASS for the PR delta. The terminal test keeps the meaningful
Failed assertion without the removed identical-call comparison; compile-failure fixture
retains HEALTHY_SUBJECT plus BROKEN_ITEM, exactly-one-E0425 and absence-of-withdraw-error
assertions; delimiter tests retain all listed openers/closers and non-listed controls.
The non-Deserialize probe retains a positive String control and all three negative checks.
The report-path exception test compares actual received argv, asserts report-path shape
and uniqueness, then compares all remaining evidence except memory/diagnostics. Group
phase/report split, home grouping, exact refusal-code and output-cap checks remain real
production-path assertions. Original mutation receipts remain historical supporting
evidence, not rerun mutations. Prior FND-001/002/003 remain fixed.

Focused executable evidence was read directly from the coder's current logs under
/home/peter/dev/worktrees/kani-lead-codex/ir809-code/: fmt-check, lint and spec pass;
92 focused library tests pass; 145 focused integration tests pass with 17 ignored;
2 diagnostics doctests pass, including the compile-fail example and compiling control.
These are coder results, not newly executed reviewer runs. The aggregate gate excludes
ignored real-Kani tests and does not by itself discharge FR-017-AC-32's real-console clause.
Actual corrected-fixture execution exists at
/tmp/claude-1000/-home-peter-dev/b52901b3-d3c2-43a5-b0e5-9f273608f02a/scratchpad/ir809/f1-real-green.log:
`tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile` passes (1 passed,
0 failed, 0 ignored). The original SR-11908 disposition and its published round-1 record
confirm this is the healthy-subject plus BROKEN_ITEM fixture, run under the 16G scope.
Current fixture and test body retain exactly that one E0425/undefined_name, no withdraw
error, Compile/NoVerdict, stdout Kani cargo-failure line, stderr Cargo failure line and
crate-path redaction assertions. Its own change since the prior run is only canonical trace;
main's imported fixture adds deletes_scoped. This is retained prior real-tool evidence,
not a claim that the current frozen candidate's ignored test was re-executed. No missing
receipt remained to justify another Kani run; no Cargo/Kani job or aggregate gate ran here.

Current outcome: ready for custody of these latest review artifacts, followed by the
leader's aggregate gate on the exact resulting head. This review is not gate approval.
IR-705/IR-844 holds and unrelated IR-397 dependency blockers are unchanged.

### Round 3 examined scope and bindings

```yaml
scope:
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'In the `make kani` lane, a crate whose library source holds the generated
    harness byte for byte and, beside it, an item that does not compile (a reference
    to an undefined name) is run through `execute_kani_obligation`: the run exits
    unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit
    code, and its evidence carries a diagnostic record whose phase is `compile`, whose
    stdout excerpt holds the rustc error code and Kani''s `error: Failed to execute
    cargo` line and whose stderr excerpt holds Cargo''s `error: could not compile`
    line with the crate path as `<CRATE_DIR>`. A '
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside
    the unsuccessful exit is `unknown` even when its stdout holds `Solving with CaDiCaL
    3.0.0`; with no report, stdout holding the Kani banner, `Checking harness proofs::check...`
    and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and the `Checking harness`
    line alone is `backend`; the banner and `error: Failed to execute cargo (exit
    status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error:
    could not compile` alone with the banner; a usage error on stderr and nothing
    on stdout is `launcher`; the ba'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes`
    0; one of 8193 bytes whose final line starts within the window keeps the longest
    line-aligned suffix of at most 8192 bytes with `omittedBytes` the byte count before
    it; a stream whose final line alone exceeds 8192 bytes keeps at most 8192 bytes
    ending at its last byte, starting on a character boundary; a stream that is one
    10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`,
    never an empty excerpt; a stream of a 10000-byte line, `\n`, `error: last\n` keeps
    exactly `error: last\n`; a 4-byte ch'
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target`
    and home directory `/h/u` supplied in the request, a stand-in whose streams print
    `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`, `/h/u/ws/crate/target/debug/x.out`,
    `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`,
    `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`,
    `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242`
    and `thread ''main'' (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`,
    `<TARGET_'
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The diagnostic record never feeds classification: two runs with the same
    report and exit and different stream text classify to the same outcome, checks,
    SUCCESS count and playback, and two `NoVerdict` runs with equal exit and no report
    whose streams differ have equal evidence in every field but `diagnostics`, `memory`
    (the observed peak resident bytes vary run to run) and the run''s report-path
    argument (the last argument, unique to each run); the FR-029 terminal value of
    a `NoVerdict` evidence carrying a record is `Failed`, the value FR-029-AC-4 gives
    the no-verdict reason, equal to the termin'
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher()\
    \ {\n            let stand_in = printing(\"diag-silent\", None, \"\", \"\", 1);\n\
    \            let evidence = run(&stand_in, &proofs());\n            assert_eq!(evidence.outcome,\
    \ NO_VERDICT);\n            assert_eq!(evidence.exit_code, Some(1));\n       \
    \     let record = diagnostics(&evidence);\n            assert_eq!(record.phase,\
    \ KaniFailurePhase::Launcher);\n            for excerpt in [&record.stdout, &record.stderr]\
    \ {\n                assert_eq!((excerpt.text.as_str(), excerpt.omitted_bytes),\
    \ (\"\", 0));\n            }"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member()\
    \ {\n            let harness = proofs();\n            let noisy = format!(\"{BANNER}error:\
    \ could not compile\\n{SOLVING}\");\n            let falsifying = block(\"proofs::check\"\
    , \"assertion\", \"falsify\");\n            let runs: [(&str, Option<Vec<u8>>,\
    \ String, i32); 4] = [\n                (\n                    \"verified\",\n\
    \                    Some(report(\"Success\", &[PASSED, COVER_OK])),\n       \
    \             noisy.clone(),\n                    0,"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_over_limit_and_unread_streams_stay_refused_with_no_record
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_043_over_limit_and_unread_streams_stay_refused_with_no_record()\
    \ {\n            let flood = StandIn::running(\n                \"diag-flood\"\
    ,\n                &format!(\"head -c {} /dev/zero\\nexit 1\\n\", CAPTURE_LIMIT\
    \ + 1),\n            );\n            let refusal = execute_kani_obligation(&flood.request(&proofs(),\
    \ T)).unwrap_err();\n            assert_eq!(refusal.code(), Some(\"kani_output_over_limit\"\
    ));\n\n            let members = StandIn::running(\n                \"diag-flood-group\"\
    ,"
- id: tests/it/kani_obligations.rs::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "fn tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics()\
    \ {\n    let cancel = quire_exact::Cancel::new();\n    let package = bound_package(1000);\n\
    \    let harness = supported_contract_harnesses(&package, \"crate::withdraw\"\
    ).remove(1);\n    let directory = write_crate(&harness, HEALTHY_SUBJECT);\n  \
    \  let launcher = directory.join(\"cargo-kani\");\n    write_failing_launcher(&launcher);\n\
    \    let installation = KaniInstallation::new(launcher);\n    let mut evidence\
    \ = execute_kani_obligation(&KaniExecutionRequest {\n        cancel: &cancel,"
- id: tests/it/kani_obligations.rs::tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "fn tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile()\
    \ {\n    let cancel = quire_exact::Cancel::new();\n    let installation = KaniInstallation::discover().expect(\"\
    cargo-kani is installed\");\n    let package = bound_package(1000);\n    let harness\
    \ = supported_contract_harnesses(&package, \"crate::withdraw\").remove(1);\n \
    \   let subject = format!(\"{HEALTHY_SUBJECT}\\n{BROKEN_ITEM}\");\n    let crate_directory\
    \ = write_crate(&harness, &subject);\n    let target = crate_directory.join(\"\
    target\");\n    let evidence = execute_kani_obligation(&KaniExecutionRequest {\n\
    \        cancel: &cancel,"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver()\
    \ {\n        let members = names(&[\"a::check\", \"b::check\"]);\n        let\
    \ stdout = format!(\n            \"{BANNER}Solving with CaDiCaL 3.0.0\\nChecking\
    \ harness a::check...\\nChecking harness b::check...\\nSolving with CaDiCaL 3.0.0\\\
    n\"\n        );\n        let process = output(&stdout, \"\", &members, &[], false);\n\
    \        assert_eq!(process.phase_of(\"a::check\"), KaniFailurePhase::Backend);\n\
    \        assert_eq!(process.phase_of(\"b::check\"), KaniFailurePhase::Solver);\n\
    \    }\n"
- id: src/kani/output/diagnostics.rs::tests::tc_027_parallel_jobs_make_segment_phases_unknown_but_not_compile_or_launcher
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_parallel_jobs_make_segment_phases_unknown_but_not_compile_or_launcher()\
    \ {\n        let members = names(&[\"a::check\"]);\n        let stdout = format!(\"\
    {BANNER}Checking harness a::check...\\nSolving with CaDiCaL 3.0.0\\n\");\n   \
    \     for arguments in [\n            names(&[\"-j\", \"2\"]),\n            names(&[\"\
    --jobs\", \"2\"]),\n            names(&[\"--jobs=2\"]),\n            names(&[\"\
    -j2\"]),\n            names(&[\"-j=2\"]),\n            names(&[\"-j16\"]),"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_marker_before_the_excerpt_window_still_decides_the_phase
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_marker_before_the_excerpt_window_still_decides_the_phase()\
    \ {\n        let members = names(&[\"a::check\"]);\n        let stdout = format!(\n\
    \            \"{BANNER}Checking harness a::check...\\nSolving with CaDiCaL 3.0.0\\\
    n{}\",\n            \"filler line\\n\".repeat(2000)\n        );\n        let process\
    \ = output(&stdout, \"\", &members, &[], false);\n        let record = process.diagnostics_of(\"\
    a::check\");\n        assert_eq!(record.phase, KaniFailurePhase::Solver);\n  \
    \      assert!(record.stdout.omitted_bytes > 0);"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_marker_stand_ins_decide_the_failure_phase
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_marker_stand_ins_decide_the_failure_phase() {\n    \
    \        let checking = \"Checking harness proofs::check...\\n\";\n          \
    \  let cargo_failed =\n                \"error: Failed to execute cargo (exit\
    \ status: 101). Found 1 compilation errors.\\n\";\n            let filler = \"\
    filler line\\n\".repeat(2000);\n            let solver_stdout = format!(\"{BANNER}{checking}{SOLVING}\"\
    );\n            let long_after = format!(\"{solver_stdout}{filler}\");\n     \
    \       let success = report(\"Success\", &[PASSED, COVER_OK]);\n            let\
    \ cases = [\n                case("
- id: src/kani/output/diagnostics.rs::tests::tc_027_ansi_escapes_go_and_controls_and_invalid_bytes_become_replacement_characters
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_ansi_escapes_go_and_controls_and_invalid_bytes_become_replacement_characters()\
    \ {\n        let colored = \"\\u{1b}[1;31merror\\u{1b}[0m: \\u{1b}]0;title\\u{7}bad\\\
    u{1b}(Bx\\tz\\ry\\u{7}\";\n        assert_eq!(kept(colored).text, \"error: badx\\\
    tz\\u{fffd}y\\u{fffd}\");\n        assert_eq!(\n            excerpt(b\"a\\xffb\\\
    xc3\", &directories(None)).text,\n            \"a\\u{fffd}b\\u{fffd}\"\n     \
    \   );\n    }\n\n    /// A stream of exactly the limit is whole; one byte more\
    \ keeps the longest line-aligned"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_limit_is_whole_and_one_over_keeps_a_line_aligned_suffix
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_stream_at_the_limit_is_whole_and_one_over_keeps_a_line_aligned_suffix()\
    \ {\n        let whole = format!(\"{}\\n\", \"a\".repeat(8191));\n        let\
    \ at_limit = kept(&whole);\n        assert_eq!((at_limit.text.len(), at_limit.omitted_bytes),\
    \ (8192, 0));\n\n        let over = format!(\"first\\n{}\\n\", \"b\".repeat(8190));\n\
    \        assert_eq!(over.len(), 8197);\n        let tail = kept(&over);\n    \
    \    assert_eq!(tail.text, format!(\"{}\\n\", \"b\".repeat(8190)));\n        assert_eq!(tail.omitted_bytes,\
    \ 6);"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_final_line_over_the_limit_is_cut_on_a_character_boundary
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_final_line_over_the_limit_is_cut_on_a_character_boundary()\
    \ {\n        let long = format!(\"x\\n{}\", \"y\".repeat(10_000));\n        let\
    \ tail = kept(&long);\n        assert_eq!(tail.text, \"y\".repeat(8192));\n  \
    \      assert_eq!(tail.omitted_bytes, 1810);\n\n        // The cut falls inside\
    \ the 4-byte character at offset 3809, so the tail starts at the\n        // next\
    \ character, 3813.\n        let emoji = format!(\"a{}b\", \"\\u{1f980}\".repeat(3000));\n\
    \        let tail = kept(&emoji);"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_long_line_with_its_line_feed_is_never_an_empty_excerpt
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_long_line_with_its_line_feed_is_never_an_empty_excerpt()\
    \ {\n        let one = format!(\"{}\\n\", \"z\".repeat(10_000));\n        let\
    \ tail = kept(&one);\n        assert_eq!(tail.text.len(), 8192);\n        assert!(tail.text.ends_with('\\\
    n'));\n        assert_eq!(tail.omitted_bytes, 1809);\n\n        let last = format!(\"\
    {}\\nerror: last\\n\", \"z\".repeat(10_000));\n        let tail = kept(&last);\n\
    \        assert_eq!(tail.text, \"error: last\\n\");"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_line_feed_only_stream_is_kept_and_an_empty_one_is_empty
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_line_feed_only_stream_is_kept_and_an_empty_one_is_empty()\
    \ {\n        let tail = kept(&\"\\n\".repeat(9000));\n        assert_eq!((tail.text.len(),\
    \ tail.omitted_bytes), (8192, 808));\n        let short = kept(\"\\n\\n\\n\");\n\
    \        assert_eq!((short.text.as_str(), short.omitted_bytes), (\"\\n\\n\\n\"\
    , 0));\n        let empty = kept(\"\");\n        assert_eq!((empty.text.as_str(),\
    \ empty.omitted_bytes), (\"\", 0));\n    }\n\n    /// A stream of the 8 MiB limit\
    \ still yields an excerpt of at most 8192 bytes, counting the"
- id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_eight_mebibyte_limit_still_yields_a_bounded_excerpt
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_a_stream_at_the_eight_mebibyte_limit_still_yields_a_bounded_excerpt()\
    \ {\n        let line = \"stream line of text that Kani printed\\n\";\n      \
    \  let stream = line.repeat(8 * 1024 * 1024 / line.len() + 1);\n        let tail\
    \ = excerpt(&stream.as_bytes()[..8 * 1024 * 1024], &directories(None));\n    \
    \    assert!(tail.text.len() <= 8192);\n        assert!(tail.text.starts_with(\"\
    stream line\"));\n        assert_eq!(\n            usize::try_from(tail.omitted_bytes).unwrap()\
    \ + tail.text.len(),\n            8 * 1024 * 1024\n        );"
- id: src/kani/output/diagnostics.rs::tests::tc_027_every_redaction_input_yields_its_stated_output
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_every_redaction_input_yields_its_stated_output() {\n   \
    \     let pairs = [\n            (\"(/h/u/ws/crate)\", \"(<CRATE_DIR>)\"),\n \
    \           (\n                \"/h/u/ws/crate/src/lib.rs:6:21\",\n          \
    \      \"<CRATE_DIR>/src/lib.rs:6:21\",\n            ),\n            (\n     \
    \           \"/h/u/ws/crate/target/debug/x.out\",\n                \"<TARGET_DIR>/debug/x.out\"\
    ,"
- id: src/kani/output/diagnostics.rs::tests::tc_027_known_directories_match_whole_components_and_unusable_homes_redact_nothing
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_known_directories_match_whole_components_and_unusable_homes_redact_nothing()\
    \ {\n        let sibling = KnownDirectories {\n            crate_directory: Path::new(\"\
    /x/crate\"),\n            target_directory: Path::new(\"/x/crate/target/\"),\n\
    \            home_directory: Some(Path::new(\"/x\")),\n        };\n        let\
    \ text = \"/x/crate2/lib.rs /x/crate/lib.rs /x/crate/target /x/other/y /x\";\n\
    \        assert_eq!(\n            excerpt(text.as_bytes(), &sibling).text,\n \
    \           \"<HOME>/crate2/lib.rs <CRATE_DIR>/lib.rs <TARGET_DIR> <HOME>/other/y\
    \ <HOME>\""
- id: src/kani/output/diagnostics.rs::tests::tc_027_the_longest_known_directory_wins_and_a_tie_goes_crate_target_home
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_the_longest_known_directory_wins_and_a_tie_goes_crate_target_home()\
    \ {\n        let same = KnownDirectories {\n            crate_directory: Path::new(\"\
    /a/b\"),\n            target_directory: Path::new(\"/a/b\"),\n            home_directory:\
    \ Some(Path::new(\"/a/b\")),\n        };\n        assert_eq!(excerpt(b\"/a/b/c\"\
    , &same).text, \"<CRATE_DIR>/c\");\n        let nested = KnownDirectories {\n\
    \            crate_directory: Path::new(\"/a/b\"),\n            target_directory:\
    \ Path::new(\"/a\"),"
- id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_opener_starts_a_token_and_no_other_character_does
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_every_listed_path_opener_starts_a_token_and_no_other_character_does()\
    \ {\n        for opener in ['\\n', ' ', '\\t', '(', '[', '{', '\\'', '\"', '`',\
    \ '='] {\n            let text = format!(\"x{opener}/h/u/ws/crate/src/lib.rs\"\
    );\n            assert_eq!(\n                redacted(&text, Some(HOME)),\n  \
    \              format!(\"x{opener}<CRATE_DIR>/src/lib.rs\"),\n               \
    \ \"{opener:?}\"\n            );\n        }\n        for other in ["
- id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_closer_ends_a_token_and_no_other_character_does
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_every_listed_path_closer_ends_a_token_and_no_other_character_does()\
    \ {\n        for closer in [\n            ' ', '\\t', '\\n', ')', ']', '}', '\\\
    '', '\"', '`', ',', ';', ':',\n        ] {\n            let text = format!(\"\
    /opt/a/b{closer}rest\");\n            assert_eq!(\n                redacted(&text,\
    \ None),\n                format!(\"<PATH>{closer}rest\"),\n                \"\
    {closer:?}\"\n            );"
- id: src/kani/output/diagnostics.rs::tests::tc_027_duration_pid_and_timestamp_rules_respect_their_boundaries
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_duration_pid_and_timestamp_rules_respect_their_boundaries()\
    \ {\n        for (input, output) in [\n            (\"took 5ms.\", \"took <DURATION>.\"\
    ),\n            (\"1.5e+3s done\", \"<DURATION> done\"),\n            (\"0.5s\"\
    , \"<DURATION>\"),\n            (\"x1.5s\", \"x1.5s\"),\n            (\"1.2.3s\"\
    , \"1.2.3s\"),\n            (\"5sec\", \"5sec\"),\n            (\"5s_x\", \"5s_x\"\
    ),\n            (\"30m\", \"30m\"),"
- id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_and_its_parts_implement_no_deserialize
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_the_record_and_its_parts_implement_no_deserialize() {\n\
    \        assert!(\n            implements_deserialize!(String),\n            \"\
    the probe sees a Deserialize impl\"\n        );\n        assert!(!implements_deserialize!(KaniNoVerdictDiagnostics));\n\
    \        assert!(!implements_deserialize!(KaniStreamExcerpt));\n        assert!(!implements_deserialize!(KaniFailurePhase));\n\
    \    }\n"
- id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_serializes_with_the_stated_member_names
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_027_the_record_serializes_with_the_stated_member_names() {\n\
    \        let members = names(&[\"a::check\"]);\n        let record = output(\"\
    out\\n\", \"\", &members, &[], false).diagnostics_of(\"a::check\");\n        assert_eq!(\n\
    \            serde_json::to_value(&record).unwrap(),\n            serde_json::json!({\n\
    \                \"phase\": \"launcher\",\n                \"stdout\": { \"text\"\
    : \"out\\n\", \"omittedBytes\": 0 },\n                \"stderr\": { \"text\":\
    \ \"\", \"omittedBytes\": 0 },\n            })"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_runs_streams_are_redacted_against_the_requests_directories
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_a_runs_streams_are_redacted_against_the_requests_directories()\
    \ {\n            let stand_in = StandIn::scaffold(\"diag-redact\");\n        \
    \    stand_in.install(&print_body(&stand_in, None, 1));\n            let (crate_directory,\
    \ target, home) = (\n                stand_in.crate_directory.display().to_string(),\n\
    \                stand_in.target_directory.display().to_string(),\n          \
    \      stand_in.directory.display().to_string(),\n            );\n           \
    \ let stdout = format!(\n                \"{BANNER}({crate_directory})\\n{crate_directory}/src/lib.rs:6:21\\\
    n{target}/debug/x.out\\n{h"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_runs_differing_only_in_redacted_values_serialize_identical_records
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_runs_differing_only_in_redacted_values_serialize_identical_records()\
    \ {\n            let stand_in = StandIn::scaffold(\"diag-identical\");\n     \
    \       stand_in.install(&print_body(&stand_in, None, 1));\n            let dir\
    \ = stand_in.crate_directory.display().to_string();\n            let harness =\
    \ proofs();\n            let mut records = Vec::new();\n            for (stamp,\
    \ seconds, pid) in [\n                (\"2026-10-16T22:27:04.275975036Z\", \"\
    0.00331318s\", 4242),\n                (\"2025-01-02T03:04:05+01:00\", \"1.385e-05s\"\
    , 17),\n            ] {"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_the_same_run_under_two_process_homes_gives_the_same_record
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_027_the_same_run_under_two_process_homes_gives_the_same_record()\
    \ {\n            let exe = std::env::current_exe().unwrap();\n            let\
    \ scratch = discover_scratch(\"diag-home\");\n            let mut records = Vec::new();\n\
    \            for home in [\"/ir809-home-a\", \"/ir809-home-b\"] {\n          \
    \      let out = scratch.join(home.trim_start_matches('/'));\n               \
    \ let status = std::process::Command::new(&exe)\n                    .args([\"\
    --exact\", PROBE, \"--test-threads=1\"])\n                    .env(\"HOME\", home)\n\
    \                    .env(PROBE_OUT, &out)"
- id: src/kani/output/diagnostics.rs::tests::tc_043_each_member_takes_the_phase_of_its_own_segment
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    fn tc_043_each_member_takes_the_phase_of_its_own_segment() {\n   \
    \     let members = names(&[\"a::check\", \"b::check\", \"c::check\"]);\n    \
    \    let stdout = format!(\n            \"{BANNER}Checking harness a::check...\\\
    nSolving with CaDiCaL 3.0.0\\nChecking harness b::check...\\n\"\n        );\n\
    \        let process = output(&stdout, \"\", &members, &[], false);\n        let\
    \ phases: Vec<_> = members.iter().map(|m| process.phase_of(m)).collect();\n  \
    \      assert_eq!(\n            phases,\n            ["
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_stream_text_never_changes_the_classification
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_043_stream_text_never_changes_the_classification() {\n \
    \           let harness = proofs();\n            let exported = report(\"Failure\"\
    , &[FAILED, COVER_OK]);\n            let stand_in = printing(\n              \
    \  \"diag-independent\",\n                Some(&exported),\n                &block(\"\
    proofs::check\", \"assertion\", \"falsify\"),\n                \"\",\n       \
    \         1,\n            );"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts()\
    \ {\n            let stdout = format!(\n                \"{BANNER}Checking harness\
    \ a::check...\\n{SOLVING}Checking harness b::check...\\n\"\n            );\n \
    \           let stand_in = printing(\"diag-group\", None, &stdout, \"warning:\
    \ x\\n\", 1);\n            let evidence = group_evidence(&stand_in);\n       \
    \     assert!(evidence.iter().all(|e| e.outcome == NO_VERDICT));\n           \
    \ let phases: Vec<_> = evidence.iter().map(|e| diagnostics(e).phase).collect();\n\
    \            assert_eq!(\n                phases,"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_a_group_report_decides_between_unknown_and_no_record
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_043_a_group_report_decides_between_unknown_and_no_record()\
    \ {\n            let all_success = batch_report(&[\n                entry(\"a::check\"\
    , \"Success\", &[PASSED, COVER_OK]),\n                entry(\"b::check\", \"Success\"\
    , &[PASSED, COVER_OK]),\n                entry(\"c::check\", \"Success\", &[PASSED,\
    \ COVER_OK]),\n            ]);\n            let stdout = format!(\"{BANNER}Checking\
    \ harness a::check...\\n{SOLVING}\");\n            let stand_in = printing(\"\
    diag-group-success\", Some(&all_success), &stdout, \"\", 1);\n            let\
    \ evidence = group_evidence(&stand_in);\n            assert!(evidence"
- id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_requests_with_different_home_directories_are_not_grouped
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "        fn tc_043_requests_with_different_home_directories_are_not_grouped()\
    \ {\n            let (u, v) = (PathBuf::from(\"/h/u\"), PathBuf::from(\"/h/v\"\
    ));\n            let pair = pair();\n            let processes = |first: Option<&Path>,\
    \ second: Option<&Path>| {\n                let stand_in = printing(\"diag-homes\"\
    , None, &format!(\"{BANNER}/h/u/x\\n\"), \"\", 1);\n                let requests\
    \ = [\n                    KaniExecutionRequest {\n                        home_directory:\
    \ first,\n                        ..stand_in.request(&pair[0], T)\n          \
    \          },"
- id: src/kani/run/launch.rs::tests::the_merged_text_is_the_lossy_streams_joined_by_one_line_feed
  path: src/kani/run/launch.rs
  role: examined
  excerpt: "    fn the_merged_text_is_the_lossy_streams_joined_by_one_line_feed()\
    \ {\n        let mut command = Command::new(\"sh\");\n        command\n      \
    \      .arg(\"-c\")\n            .arg(\"printf 'a\\\\377b'; printf 'c\\\\n' 1>&2;\
    \ exit 1\");\n        let LaunchOutcome::Completed { streams, .. } =\n       \
    \     run_launcher_with_timeout(command, Duration::from_secs(5)).unwrap()\n  \
    \      else {\n            panic!(\"a fast process completes\");\n        };"
- id: src/kani/output/diagnostics.rs::ProcessOutput::analyse
  path: src/kani/output/diagnostics.rs
  role: examined
  excerpt: "    pub(crate) fn analyse(&self) -> ProcessDiagnostics {\n        let\
    \ stdout = String::from_utf8_lossy(self.stdout);\n        let stderr = String::from_utf8_lossy(self.stderr);\n\
    \        ProcessDiagnostics {\n            stdout: excerpt(self.stdout, &self.directories),\n\
    \            stderr: excerpt(self.stderr, &self.directories),\n            stages:\
    \ Stages::read(&stdout, self.selections),\n            report_exported: self.report_exported,\n\
    \            parallel: runs_in_parallel(self.arguments),\n            cargo_failed:\
    \ has_line_starting(\n                &stdout,\n                \"error: Failed\
    \ to exe"
- id: src/kani/run/execute.rs::finish_group
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "fn finish_group(\n    group: &[(usize, &KaniExecutionRequest<'_>)],\n\
    \    arguments: Vec<String>,\n    exited_successfully: bool,\n    exit_code: Option<i32>,\n\
    \    streams: &CapturedStreams,\n    report: Option<Vec<u8>>,\n) -> Result<Vec<ReportedExecution>,\
    \ KaniExecutionRefusal> {\n    let Some((_, first)) = group.first() else {\n \
    \       return Ok(Vec::new());\n    };\n    let views: Vec<_> = group\n      \
    \  .iter()\n        .map(|(_, request)| request.harness.view())\n        .collect();\n\
    \    let selections: Vec<_> = views.iter().map(|view| view.selection.clone()).collect();\n\
    \    let invocation = KaniBatchInvo"
- id: src/kani/run/execute.rs::finish_single
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "fn finish_single(\n    request: &KaniExecutionRequest<'_>,\n    arguments:\
    \ Vec<String>,\n    outcome: LaunchOutcome,\n    report: Option<Vec<u8>>,\n) ->\
    \ Result<ReportedExecution, KaniExecutionRefusal> {\n    let harness = request.harness.view();\n\
    \    let (run, exit_code, streams) = launch_evidence(outcome, report.as_deref(),\
    \ harness.kind)?;\n    let diagnostics = no_verdict_streams(&run, streams.as_ref()).map(|streams|\
    \ {\n        process_output(\n            request,\n            streams,\n   \
    \         std::slice::from_ref(&harness.selection),\n            &arguments,\n\
    \            report.is_some(),\n        )"
- id: src/kani/run/execute.rs::shares_process
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "fn shares_process(first: &KaniExecutionRequest<'_>, other: &KaniExecutionRequest<'_>)\
    \ -> bool {\n    first.cancel == other.cancel\n        && first.harness.view().ceilings\
    \ == other.harness.view().ceilings\n        && first.installation == other.installation\n\
    \        && first.crate_directory == other.crate_directory\n        && first.target_directory\
    \ == other.target_directory\n        && first.home_directory == other.home_directory\n\
    \        && without_selection(first.harness.view().options)\n            == without_selection(other.harness.view().options)\n\
    }\n\n/// `options` with every `--harness <path> --e"
- id: src/kani/run/launch.rs::CapturedStreams::merged_text
  path: src/kani/run/launch.rs
  role: examined
  excerpt: "    pub(super) fn merged_text(&self) -> String {\n        format!(\n \
    \           \"{}\\n{}\",\n            String::from_utf8_lossy(&self.stdout),\n\
    \            String::from_utf8_lossy(&self.stderr)\n        )\n    }\n}\n\n///\
    \ How the launcher's run within its caller-declared budget\n/// (the wall-clock\
    \ ceiling of its identity for a bounded backend launch)\n/// concluded.\n#[derive(Debug)]\n\
    #[non_exhaustive]\npub(super) enum LaunchOutcome {\n    /// The process exited\
    \ on its own within the budget.\n    Completed {\n        /// `ExitStatus::success()`.\n\
    \        exited_successfully: bool,\n        /// `ExitStatus::code("
- id: src/kani/run/scalar.rs::execute_native
  path: src/kani/run/scalar.rs
  role: examined
  excerpt: "fn execute_native(\n    request: &KaniScalarNativeRequest<'_>,\n    root:\
    \ &Path,\n) -> Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError>\
    \ {\n    let mut command = BackendCommand::new(\"cargo\");\n    let mut arguments\
    \ = vec![\n        \"run\".to_owned(),\n        \"--quiet\".to_owned(),\n    \
    \    \"--locked\".to_owned(),\n        \"--bin\".to_owned(),\n        \"ir791_native\"\
    .to_owned(),\n        \"--\".to_owned(),\n    ];\n    arguments.extend(request.operands.iter().map(ToString::to_string));\n\
    \    arguments.extend(limits_array(request.scalar_limits).map(|value| value.to_string()));\n\
    \    command.ar"
- id: spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "      diagnostics.rs          the bounded NoVerdict record: failure phase\
    \ from the console markers and\n                              the redacted stream\
    \ tails (FR-017-AC-32 to AC-36)\n      report.rs               typed report parse\
    \ (--export-json)\n      playback.rs             the concrete-playback block:\
    \ the console-text extraction\n                              (`counterexample_playback`)\
    \ and, from step 2f, the block scan\n                              `kani_witness_join`\
    \ held, with `DecodeFailure`; typed entries from step 5\n    classify.rs     \
    \          KaniRunOutcome, KaniInconclusiveReason,"
bindings:
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_over_limit_and_unread_streams_stay_refused_with_no_record
  ac_id: FR-017-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics
  ac_id: FR-017-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile
  ac_id: FR-017-AC-32
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_parallel_jobs_make_segment_phases_unknown_but_not_compile_or_launcher
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_marker_before_the_excerpt_window_still_decides_the_phase
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_marker_stand_ins_decide_the_failure_phase
  ac_id: FR-017-AC-33
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_ansi_escapes_go_and_controls_and_invalid_bytes_become_replacement_characters
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_limit_is_whole_and_one_over_keeps_a_line_aligned_suffix
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_final_line_over_the_limit_is_cut_on_a_character_boundary
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_long_line_with_its_line_feed_is_never_an_empty_excerpt
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_line_feed_only_stream_is_kept_and_an_empty_one_is_empty
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_a_stream_at_the_eight_mebibyte_limit_still_yields_a_bounded_excerpt
  ac_id: FR-017-AC-34
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_redaction_input_yields_its_stated_output
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_known_directories_match_whole_components_and_unusable_homes_redact_nothing
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_longest_known_directory_wins_and_a_tie_goes_crate_target_home
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_opener_starts_a_token_and_no_other_character_does
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_every_listed_path_closer_ends_a_token_and_no_other_character_does
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_duration_pid_and_timestamp_rules_respect_their_boundaries
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_and_its_parts_implement_no_deserialize
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_027_the_record_serializes_with_the_stated_member_names
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_a_runs_streams_are_redacted_against_the_requests_directories
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_runs_differing_only_in_redacted_values_serialize_identical_records
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_027_the_same_run_under_two_process_homes_gives_the_same_record
  ac_id: FR-017-AC-35
  trace: correct
- test_id: src/kani/output/diagnostics.rs::tests::tc_043_each_member_takes_the_phase_of_its_own_segment
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_over_limit_and_unread_streams_stay_refused_with_no_record
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_stream_text_never_changes_the_classification
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_a_group_report_decides_between_unknown_and_no_record
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::tc_043_requests_with_different_home_directories_are_not_grouped
  ac_id: FR-017-AC-36
  trace: correct
- test_id: src/kani/run/launch.rs::tests::the_merged_text_is_the_lossy_streams_joined_by_one_line_feed
  ac_id: FR-017-AC-36
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics
  ac_id: FR-017-AC-36
  trace: correct
```
