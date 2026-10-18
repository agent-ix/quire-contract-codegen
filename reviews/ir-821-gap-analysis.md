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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Unit-test scratch crates in src/ still sit under /tmp with pid-only names, so a killed `cargo test --lib` leaks them. They are not named in IR-834. src/oracle/equality/mod.rs:2873 writes and builds a full cargo crate with a target directory. src/oracle/function/schedule.rs:1277 builds the 100k-node AC-7 schedule with rustc. The review pass missed them, because its grep did not recurse src/**. src/publication/publish.rs:273 and src/kani/run/scalar.rs:1052 are small file-only directories. src/kani/run/scalar.rs:912 is production code and out of scope. Add the two crate-building sites to IR-834, or convert them the way discover_scratch was converted. | src/oracle/equality/mod.rs:2873, src/oracle/function/schedule.rs:1277, src/publication/publish.rs:273, src/kani/run/scalar.rs:1052 |

## Dispositions

Disposition pass 1, over e3f3375.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred: IR-834 | Converted in e3f3375: bound_coverage.rs, bound_generation.rs, capability_settlement.rs, exact_function_generation.rs (both sites), scratch_crate.rs:151 and src/kani/test_support.rs, where discover_scratch now sits under <target>/tmp/quire-kani-discover/ with a dead-pid sweep. The remaining sites, kani_obligations.rs:2096 and :2759, kani_witness_join.rs:240, kani_batching.rs:113 and kani_generation_claims.rs:767, sit in files owned by CG #406 and adjacent to IR-808. They moved to the linked subticket IR-834, whose acceptance names each site. The PR title should narrow to match. |

Disposition pass 2, at head 102dfa9 (it only copies the round-1 SR files into reviews/; those
copies match the round-1 scratchpad files byte for byte). The PR title now reads "IR-821: most
tests/it scratch crates and src/kani test support live under the target dir and are reclaimed
after killed runs (rest: IR-834)", which resolves the title point recorded in FND-001's round-1
row.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | deferred: IR-834 | IR-834 comment 67c0a0a8 adds all four src/ sites: src/oracle/equality/mod.rs:2873, src/oracle/function/schedule.rs:1277, src/publication/publish.rs:273 and src/kani/run/scalar.rs:1052 (the last in CG #406's lane). The same reclaim rule as discover_scratch applies: the target directory found from current_exe, plus a dead-pid sweep. |

## Follow-up review after maintenance refresh

The frozen maintenance candidate was reviewed against its assigned parent. All 19 own paths
were read: 17 Rust test/helper files and these two historical SR artifacts. No production-code,
normative-specification, dependency, manifest/lock, Makefile or CI changes are in this delta.
`src/kani/mod.rs` retains `#[cfg(test)]` on `test_support`; a `src/` location alone does not
make the helper production code. No applicable AssuranceProfile was found under `spec/`.
Code review includes the Rust lane. A production Quoin gap audit and spec review methods do
not apply; SR-11991 remains a manual ticket-acceptance check, not a genuine whole-repository
gap analysis. No additional method or SR id was allocated.

Independent inspection confirmed the original fixes still hold: distinct generated Kani
prefixes (including simultaneously live contract/conditional crates); the eight local
manifests retain `[workspace]`; the sibling advisory lock remains held until after directory
removal; same-thread reacquisition is rejected before blocking; waiter diagnostics and the
six helper behavior tests remain. The removal assertion fails if Drop is deleted, the
live-lock probe detects an omitted lock, and waiter completion/cleared-content assertions
exercise release and reclaim. The waiter's 300ms absence check alone is not treated as proof
that the waiter reached lock acquisition; it is corroborated by the independent live-lock
probe and final completion. The previously recorded cross-thread-move/reacquire exposure
has no current caller and remains the historical observation, not a newly introduced defect.
All prefixes are trusted test constants or local finite fixture labels. No new untrusted path
surface, unsafe block, error boundary, production panic, asynchronous lock or suppression
was introduced. The existing nested Cargo sources, commands, lock seeding and assertions
are preserved apart from the documented path/helper and workspace changes.

The ten rebase joins retain main's trace imports/attributes and inherited helper changes.
A direct per-file parent/head comparison found all 272 `#[trace]` payloads identical across
the 17 Rust paths. The six new helper tests verify test infrastructure against this ticket;
there is no authored FR for that hygiene mechanism and they do not claim unrelated AC tags.
No existing AC binding was removed or redirected. `quire matrix --strict --format json`
executed: exit 1, 840 criteria (454 tagged, 326 untagged, 46 method-without-symbol,
14 tagged-by-ignored-test). This is a trace check, not a whole-repository semantic audit.
The own diff changes no spec text or trace payload, so its existing coverage deficits are
not newly introduced by this maintenance. No whole-repository PASS is asserted.

Focused validation receipts were read, not rerun by this reviewer: fmt check, all-target
Clippy with denied warnings and all-target compilation succeeded; 97 selected library tests
plus two one-test child probes passed; 312 affected integration tests passed with 14 ignored
in 141.79s. The historical real-Kani receipts remain historical evidence; no ignored proof
or Cargo invocation ran in this review. Exact-head aggregate gating remains the leader's
next obligation after final restack and custody.

No new defect was found. Original findings, original verdict text and all disposition rows
above are preserved verbatim; latest outcomes remain terminal. This follow-up appends no
finding or disposition table rows and does not reopen terminal findings.

### Scope examined in this follow-up

| Unit | Role | Excerpt |
| --- | --- | --- |
| `src/kani/test_support.rs` | examined | `pub(crate) fn discover_scratch(name: &str) -> PathBuf {` |
| `tests/it/bound_census.rs` | examined | `let temporary = TemporaryDirectory::new("quire-generated-bound-census");` |
| `tests/it/bound_coverage.rs` | examined | `use crate::scratch_crate::TemporaryDirectory;` |
| `tests/it/bound_generation.rs` | examined | `use crate::scratch_crate::TemporaryDirectory;` |
| `tests/it/bound_populations.rs` | examined | `let temporary = TemporaryDirectory::new("quire-generated-bound-populations");` |
| `tests/it/bound_strategy_generation.rs` | examined | `let temporary = TemporaryDirectory::new("quire-bound-integer-oracles");` |
| `tests/it/bounded_kani_corpus.rs` | examined | `let directory = TemporaryDirectory::new("quire-codegen-bounded-kani");` |
| `tests/it/capability_settlement.rs` | examined | `let guard = crate::scratch_crate::TemporaryDirectory::new("quire-codegen-process-settlement");` |
| `tests/it/composite_equality_generation.rs` | examined | `let directory = TemporaryDirectory::new("quire-composite-equality-oracles");` |
| `tests/it/exact_function_generation.rs` | examined | `let scratch = crate::scratch_crate::TemporaryDirectory::new("exact-function-schedule");` |
| `tests/it/exact_scalar_generation.rs` | examined | `let directory = TemporaryDirectory::new("quire-exact-scalar-oracles");` |
| `tests/it/harness_generation.rs` | examined | `let temporary = TemporaryDirectory::new("quire-generated-harness");` |
| `tests/it/kani_generation.rs` | examined | `) -> TemporaryDirectory {` |
| `tests/it/oracle_arithmetic.rs` | examined | `let directory = TemporaryDirectory::new(&format!("quire-arithmetic-{label}"));` |
| `tests/it/oracle_generation.rs` | examined | `let directory = TemporaryDirectory::new("quire-native-vacuity");` |
| `tests/it/scratch_crate.rs` | examined | `pub(crate) struct TemporaryDirectory {     pub(crate) path: PathBuf,     _lock: fs::File, }` |
| `tests/it/strategy_generation.rs` | examined | `let temporary = TemporaryDirectory::new("quire-generated-strategies");` |

### Current manual acceptance and deferred scope

The claimed delivery is the shared bounded, locked helper and its converted callers, with
no change to what their generated-code tests prove. Stable target placement, reclaim before
use, lifetime locking and Drop cleanup implement the bound_populations ticket goal. Unit
`discover_scratch` places process/call names under the executable's target ancestor and its
once-per-process sweep skips live pids; pid reuse can delay removal, not erase a live owner.
The intentionally partial migration remains explicit in the PR title/body. IR-834 was read
live and remains Backlog. Its integration follow-up covers kani_obligations (two sites),
kani_witness_join, kani_batching and kani_generation_claims; its recorded extension covers
unit scratch in oracle/equality, oracle/function/schedule, publication/publish and
kani/run/scalar. Both SR-11991 deferrals remain deferred to IR-834. This review does not
close that ticket or claim those sites were migrated. The leader must preserve its state
if the PR's separate closes attachment triggers automation.
