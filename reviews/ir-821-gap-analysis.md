---
id: "SR-11991"
title: "IR-821 gap analysis (manual): ticket acceptance against the scratch-crate tests"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@070a708a695077215c4cd2c1e51e1599ed7286cb; tests/it/scratch_crate.rs, tests/it/*.rs, src/kani/test_support.rs"
relationships: []
---

# SR-11991: IR-821 gap analysis (manual)

## Summary

Ticket: IR-821. PR: agent-ix/quire-contract-codegen#407, head 070a708. The change is test-only
and has no FR, so I did not run `quoin:gap-analysis` over a spec. Instead I checked the ticket's
acceptance by hand against the tests and the remaining scratch-crate sites.

I took the acceptance from the ticket text: "make the scratch location bounded and self-cleaning
for killed runs (e.g. a stable per-run dir under the target dir that the next run or make clean
removes ...), no change to what the test proves". The ticket covers bound_populations.rs. A later
team-leader comment extends it to six more files, and the PR converts 11. The PR title claims
"scratch crates in tests/it live under the target dir and are reclaimed after killed runs".

## Method

I compared each converted call site with its main version, ran the 11 modules plus
`scratch_crate::` (201 passed), and grepped tests/ and src/ for any remaining
`temp_dir()`-based scratch directory.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR title's claim, and the ticket's goal of bounded, self-cleaning scratch crates, do not hold for the remaining Kani-lane scratch crates. These still create pid- or nonce-named directories under /tmp that nothing reclaims after a killed run: kani_obligations.rs:2096 and :2759, kani_witness_join.rs:240, kani_batching.rs:113 and src/kani/test_support.rs:178 (all driving `cargo kani`), plus bound_coverage.rs:261, bound_generation.rs:228, kani_generation_claims.rs:767, exact_function_generation.rs:676 and :1035, capability_settlement.rs:751 and scratch_crate.rs:151. A killed `make kani` (the leak scenario in the ticket) still leaves kani_obligations and kani_witness_join crates in /tmp. Either convert these sites or narrow the PR title and file a follow-up ticket. | tests/it/kani_obligations.rs:2096, tests/it/kani_obligations.rs:2759, tests/it/kani_witness_join.rs:240, tests/it/kani_batching.rs:113, src/kani/test_support.rs:178, tests/it/bound_coverage.rs:261, tests/it/bound_generation.rs:228, tests/it/kani_generation_claims.rs:767, tests/it/exact_function_generation.rs:676, tests/it/exact_function_generation.rs:1035, tests/it/capability_settlement.rs:751, tests/it/scratch_crate.rs:151 |

## Verdict

Partially met. For the 11 converted modules the location is bounded (one directory per prefix
under CARGO_TARGET_TMPDIR, which `cargo clean` removes) and reclaimed on the next run. What the
tests prove is unchanged. I diffed every converted file, and apart from `.0` becoming `.path`
the manifests, commands and assertions are identical. The open gaps are the remaining
/tmp-based Kani-lane scratch crates (FND-001) and the lane hang recorded in SR-11990 FND-001.
