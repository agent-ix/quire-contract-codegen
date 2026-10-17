---
id: SR-11903
title: "Evidence review of IR-809 NoVerdict diagnostics"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; FR-017-AC-32 to FR-017-AC-36, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md (PR #403)"
review_set: subset
---

## Summary

Ticket: IR-809. Checked that each new obligation's verification method fits and that every row
can fail. AC-32's real-Kani half belongs in the `make kani` lane and its named markers were
reproduced; AC-33/AC-34 are unit tests over stand-ins and the excerpt builder; AC-36 belongs with
the batch stand-ins. The matrix honestly records all five as PLANNED/UNRUN with no test. Two low
findings.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | AC-35 requires the record to be "not deserializable" under method Test, but a missing trait impl cannot be observed by a runtime test; without a named mechanism (a compile-fail test or a static `assert_not_impl_any!`-style assertion) the clause is untestable as written and will be checked by inspection only. No such mechanism exists in the repo for the existing serialize-only per-check view either. | spec/kani/functional/FR-017-kani-execution-evidence.md:426-427, :470; spec/kani/matrix/TC-027-kani-execution-evidence.md:186 | correct-requirement-no-evidence |
| FND-002 | low | The bullet that makes `backend`/`solver` unreachable by a real CBMC failure (bad option, OOM, signal still export a `Failure` report and classify FailedWithoutCounterexample) is a measured claim that no AC or lane test pins. If a Kani upgrade stops exporting that report, CBMC failures silently become NoVerdict with phase `backend`/`solver` and nothing fails. Add one lane control (e.g. an unknown CBMC option) asserting FailedWithoutCounterexample and no `diagnostics` member. | spec/kani/functional/FR-017-kani-execution-evidence.md:386-391 | correct-requirement-no-evidence |

## Scope examined

AC-32 to AC-36 and their TC-027/TC-043 procedure and expected-result text, and the four tests.md
rows. Clean: every phase in AC-33 has a stand-in input that distinguishes it from its neighbours
in the order (including the "other solver wording -> backend" under-claim and the
marker-outside-window case); AC-34's 8192/8193/long-line/straddle cases each fail a wrong cut;
AC-32's non-compiling harness and silent missing-report stand-in are concrete and falsifiable.

## Verdict

Evidence plan is adequate; two low gaps.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (fe3a4e1): rustdoc compile_fail example on the record type with a compiling to_string control; `make test` runs `cargo test` (doctests included) and src/core/identity.rs:96 is a working precedent. |
| FND-002 | fixed | Round 1 (fe3a4e1): The CBMC-failure measurement is now non-normative rationale and 'retaining its cause is outside this requirement', so no AC rests on it. |
