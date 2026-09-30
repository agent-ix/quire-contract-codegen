---
id: "SR-035"
title: "IR-303 code review (incl. rust-review lane): routed scalar Kani harness verifies"
type: SpecReview
scope: "agent-ix/quire-contract-codegen; src/exact_scalar.rs, src/oracle.rs, Cargo.toml, Cargo.lock, tests/it/kani_obligations.rs, spec/test/complete-v1/TC-027-kani-execution-evidence.md"
relationships: []
---

# SR-035: IR-303 code review

## Summary

Ticket: IR-303 ("CG: routed scalar Kani harness does not verify (unwind, RT layout, kani
flags)"). PR: agent-ix/quire-contract-codegen#183, branch `ir-303-routed-scalar-verifies`.

The PR moves to the runtime's explicit tags, boxed payloads and `i128`
small-integer paths (RT #79), raises the routed scalar harness's generated unwind from 2 to 3,
and adds `[package.metadata.kani] flags = { cbmc-args = ["--max-field-sensitivity-array-size",
"1024"] }` to the exact-scalar oracle crate's `manifest()`. `tests/it/kani_obligations.rs` is
refactored to share a `run_scalar_under_real_kani` helper between a renamed verifying test
(`tc_027_a_routed_scalar_harness_verifies`) and a new negative test
(`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`) that narrows the harness's
checked-domain upper literal from 9 to 5 while leaving the oracle's own enforced bound (compiled
into the harness body from `lowered.upper`, independent of the assertion-domain literal the test
mutates) untouched.

## Method

Read the full diff (`git diff origin/main...HEAD`) against `origin/main`. Traced the
mutation target (`rt::Integer::from(9_i64))`) back through `src/kani_obligations.rs::render_scalar`
to confirm it is the *checked-domain* literal used only by the harness's own `assert!(sound, ...)`
construction, not the oracle's runtime bound enforcement, so the negative test is a real
falsification and not vacuous. Compared `manifest()` across `src/exact_scalar.rs`,
`src/exact_function.rs` and `src/composite_equality.rs` to check whether the cbmc flag reaches
every oracle crate kind Kani can verify. Diffed all six touched golden fixtures against their
source functions for consistency. Read `spec/test/complete-v1/TC-027-...md` against the actual
test names/assertions. Confirmed via already-run logs (not re-run by this review, per the
dispatching brief) that both `#[ignore]` kani-lane tests pass under real Kani:
`tc_027_a_routed_scalar_harness_verifies ... ok` (288.87s) and
`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified ... ok` (216.44s).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `[package.metadata.kani]` cbmc flag added only to `exact_scalar::manifest()`; `exact_function::manifest()` and `composite_equality::manifest()` lack it although the code comment's own justification (RT's `Value`/`ValueType` exceed CBMC's default 64-byte field-sensitivity limit) is not specific to the scalar oracle | src/exact_function.rs:1419-1422, src/composite_equality.rs:1571-1574 |

### FND-001 detail

`src/exact_scalar.rs:2795-2811` adds a doc comment explaining the fix generically ("CBMC tracks
heap objects field by field only up to 64 bytes by default; RT's `Value` and `ValueType` are
larger... so the crate raises the limit the way RT's own `Cargo.toml` does") and then only
`exact_scalar::manifest()` gets the `[package.metadata.kani]` block. `exact_function::manifest()`
(src/exact_function.rs:1419) and `composite_equality::manifest()` (src/composite_equality.rs:1571)
build the identical manifest template minus that block. Today nothing exercises this gap: the only
real-Kani (`#[ignore = "kani lane"]`) tests in this repo are `tc_025` (a contract/withdraw
harness, whose manifest is built by different test-only crate-writing code, not one of these three
`manifest()` functions) and the two `tc_027` scalar tests this PR adds — there is no existing
kani-lane test for an `exact_function` or `composite_equality` oracle crate, so this PR introduces
no regression and breaks no current gate. It is a latent gap: if/when FR-017 coverage extends real
Kani execution to function or composite-equality oracle crates whose harness bodies read RT's
`Value`/`ValueType` the same way, whoever adds that lane will hit the same field-sensitivity
failure IR-303 diagnosed here and will have to rediscover this fix rather than reuse it. Scope
note: IR-303's acceptance criterion is specifically the routed scalar harness, so this is not a
blocker for this PR — recommend a follow-up ticket (or folding the block into a shared manifest
helper the three call) before the next oracle kind gets a real-Kani lane.

## Non-findings checked (clean)

- **Negative-test mutation is a real falsification, not vacuous.** `harness.rust.contents`'s only
  occurrence of `"rt::Integer::from(9_i64))"` (asserted via
  `.matches(domain_upper).count() == 1` at tests/it/kani_obligations.rs:2563) is the checked-domain
  upper bound built at `src/kani_obligations.rs:2069` for the harness's own `domain.contains(value)`
  soundness assertion. The oracle's actual enforced bound is compiled into the generated oracle
  body from `lowered.upper` at oracle-generation time and is untouched by this string replace, so
  narrowing the assertion window to `[0, 5]` while the oracle still legitimately produces values up
  to 9 is a genuine bound violation, confirmed by the real `Falsified` outcome in the run log.
- **Spec text (TC-027).** `spec/test/complete-v1/TC-027-kani-execution-evidence.md`'s routed
  scalar paragraph now says "generated with unwind 3", "confirm it is `Verified`", and "narrow the
  same harness's checked domain to `[0, 5]`... confirm the run is `Falsified` with a counterexample"
  — matches `tc_027_a_routed_scalar_harness_verifies` and
  `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` exactly.
- **Stack-size claim.** PR body states no stack-size change was needed at the default 8 MB stack;
  no stack/ulimit/rlimit code exists anywhere in the diff or the surrounding launcher, consistent
  with that claim.
- **Unwind bump (2 -> 3) blast radius.** `routed_scalar_increment()` is also used by three
  non-kani-lane tests (tests/it/kani_obligations.rs:2382, 2418, 2452); none asserts a literal
  unwind value, so the bump does not affect them (confirmed green in the lead's `make test` log).
- **Shared fixed target directory between the two real-Kani tests**
  (`run_scalar_under_real_kani`'s `target_directory` is always
  `$CARGO_TARGET_TMPDIR/kani-scalar`, shared by both `tc_027_a_routed_scalar_harness_verifies` and
  `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`) is safe only because `make
  kani` runs the ignored lane with `--test-threads=1` (Makefile:111-114); noted, not filed as a
  finding since that serial invariant is the documented entry point and pre-existing convention.

## Gates run by this review

| Gate | Command | Exit |
| --- | --- | --- |
| Format check | `cargo fmt --check` | 0 |

Not re-run (already measured by the lead, logs read and confirmed above): both real-Kani
`#[ignore]` tests, `make test` (345 passed, 0 failed), `make lint`.

## Verdict

No blockers. One low-severity, out-of-scope-for-this-ticket gap (FND-001) worth a follow-up
ticket. Mergeable once triaged (see disposition — `accepted-no-change`/`deferred` is a reasonable
outcome for FND-001 given the ticket's scalar-only scope).
