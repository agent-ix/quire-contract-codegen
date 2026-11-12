---
id: SR-17940486709564847047720
title: "IR-1022 PR #800 spec-review (FR-015-AC-143..148, TC-025 step 55)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #800 at 2a45f2e8; IR-1022; spec/kani/functional/FR-015-bounded-kani-obligations.md (AC-143..148; context AC-41, AC-119, AC-126, AC-132), spec/kani/matrix/TC-025-bounded-kani-obligations.md (step 55); evidence logs scratchpad ir1022-final/*.log and ir1022-kani/*.log"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1022. Reviews the six new FR-015 rows and TC-025 step 55. Every number and verdict in
AC-147 and AC-148 was checked against the run logs (`ir1022-final/{healthy,mutant,oracle,helper,helperhealthy}.{log,done,peak}`,
`ir1022-kani/healthy.{log,done}`, `run.sh`, `kr.sh`, journald) and against the crates the runs
built.

Checked as stated:

- Flags: `cargo kani -Z function-contracts --harness ...::prove_clause --exact --unwind 3 --solver cadical`;
  for the mutants also `-Z concrete-playback --concrete-playback print`. The cgroup is
  `systemd-run --scope -p MemoryMax=24G -p MemorySwapMax=0`, with a 1800 s timeout. Kani 0.68.0.
- Healthy: VERIFICATION SUCCESSFUL, `1 of 1 cover properties satisfied`, 675,940 steps, Runtime
  Symex 20.18 s, Verification Time 38.49 s, wall 43.11 s, peak 2,765,918,208 B. quire-exact
  e82db487 in the crate's lock.
- Bignum schedule (`ir1022-kani/healthy`, quire-exact e20118a, harness byte-identical to the final
  healthy one): 1,971,414 steps. Killed after Convert SSA: journald records
  `proofs-healthy-953871.scope: Failed with result 'oom-kill'`, 24G peak.
- Subject mutant (`amount + 2 <= 1000`, the only diff from healthy): FAILED on
  "the produced V2 Boolean result agrees with the independent checked condition". Playback 999
  (`[231,3,..]`). 92.65 s, 11,829,952,512 B. The second playback 1000 is labelled by Kani itself
  `Check for cover: "the V2 result postcondition reaches its admitted domain"`, so the
  cover-witness claim holds.
- Oracle mutant (`scalar_8` `LessOrEqual` to `Less`, the only diff): FAILED on
  "the V2 postcondition holds for the produced result". Playback 999, 92.11 s,
  11,744,514,048 B, second playback 1000 is the cover witness.
- Helper mutant (`rt-mut` patch, where `implies_short_circuit` returns `R::from(false)`; the
  subject calls the helper): FAILED on the produced-result comparison. Playback 1000, 80.55 s,
  10,791,886,848 B. Here the cover is `0 of 1 satisfied (1 unreachable)`, which the row does not
  say but does not contradict. Helper-healthy control (same subject, unpatched runtime): VERIFIED,
  42.77 s, 2,769,129,472 B, 675,979 steps.
- The FR-015-AC-41 kernel `+` mutants and the sealed playback are PLANNED with owner "the IR lead".

`make spec`: exit 0 with the 6 pre-existing warnings, none added.

## Verdict

The numbers and verdicts are supported by the logs. **Not mergeable as written**, because the rows
claim more than the evidence in kind (FND-001, FND-002) and leave older rows contradicted
(FND-003). These are text fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-147 calls the run "the healthy `Int[0, 1000]` clause of FR-015-AC-119", and AC-148 "the same harness". What ran is a scratch crate: the CG-generated harness for the `test:state-frame-twin` fixture, plus a hand-written subject `crate::request::unrelated` marked "not generated". The healthy and subject-mutant subjects compute `!(amount < 1000) \|\| (amount + 1 <= 1000)` directly and do not call `implies_short_circuit`, though AC-119's compiled subject must. Only the helper control and the helper mutant used a helper-calling subject. The run was raw `cargo kani`, not CG's runner: no FR-017-AC-63 two-phase run, no identity record, and no regeneration from QSL source (AC-119 is still PLANNED). The rows should name the fixture, the hand-written subject and the direct invocation, and should cite the helper-calling control (42.8 s, 2.77 GB) as the AC-119-shaped healthy run | spec/kani/functional/FR-015-bounded-kani-obligations.md:962-963 |
| FND-002 | medium | The evidence behind AC-147/148 is not durable. Crates, scripts and logs exist only under a session `/tmp` scratchpad, and the Linear record (IR-1022 comment 08c1f8a0) cites those `/tmp` paths. Once the scratchpad is cleaned, "recorded on IR-1022" leaves nothing to re-check, and the repo holds no script or crate that reproduces the runs. Attach the logs (or a reduced log with the verdict, checks, playback and resource lines) and the harness crates or patches to IR-1022, or commit a reproducible recipe, before the rows read Implemented | spec/kani/functional/FR-015-bounded-kani-obligations.md:962-963, spec/kani/matrix/TC-025-bounded-kani-obligations.md:1024-1036 |
| FND-003 | medium | AC-147 contradicts older rows that were not updated. AC-132's rationale says the enumeration is accepted "because the healthy clause and the exact-999 mutant were measured `Inconclusive` with `TimedOut` and no fit is claimed". AC-41's fit caveat says "no healthy-proof fit is claimed". Both are now false for this harness, and AC-147 cites neither as superseded. AC-126 requires the healthy profile to verify "at 16 GiB". These runs were capped at 24 GiB (peaks of 2.77 GB and at most 11.8 GB fit under 16 GiB, but nothing ran at that ceiling). AC-147 should say so or state the 16 GiB fit, and AC-41/AC-132 should be amended or cross-referenced | spec/kani/functional/FR-015-bounded-kani-obligations.md:862, spec/kani/functional/FR-015-bounded-kani-obligations.md:943, spec/kani/functional/FR-015-bounded-kani-obligations.md:947, spec/kani/functional/FR-015-bounded-kani-obligations.md:962 |
| FND-004 | low | "peak RSS 2.77 GB" (and the mutants' "peak") is the cgroup's `memory.peak` (`kr.sh` reads `$CG/memory.peak`), which includes page cache. It is not process RSS. Say "cgroup peak memory" | spec/kani/functional/FR-015-bounded-kani-obligations.md:962-963 |
| FND-005 | low | "the scratch shortcut `retain_boolean` is not delivered behavior" is ambiguous. The delivered harness still calls `rt::retain_boolean` on the implication's short-circuit branch (`scalar_9`). The shortcut meant is replacing the Boolean equality kernel by `retain_boolean(left == right, ..)` (the `diag` run). Say that | spec/kani/functional/FR-015-bounded-kani-obligations.md:962 |
| FND-006 | low | TC-025 step 55 is headed "FR-015-AC-143 to FR-015-AC-145" but also carries AC-146 to AC-148. Its installed-Kani part omits the cgroup (24 GiB, swap 0), the crate setup (hand-written subject; isolated runtime checkout for the helper mutant) and the helper-calling healthy control, so it cannot be re-run as written | spec/kani/matrix/TC-025-bounded-kani-obligations.md:1022-1036 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The record says "Each failing run prints a second playback with `amount_current = 1000` labelled `Check for cover: ...`". That holds for mutant 1 and mutant 2 only. `helper.log` prints one playback (1000, the counterexample), and its cover is `0 of 1 cover properties satisfied (1 unreachable)`. AC-148 states this correctly ("in the first two runs"); the record should say "mutants 1 and 2" and note the helper mutant's cover is unreachable | docs/qualification/ir1022-kani-clause-measurements.md:62-64 |

## Dispositions

Round 1, fix head 4a2849b9. `make spec` exit 0 with the 6 pre-existing warnings. I checked every
number in `docs/qualification/ir1022-kani-clause-measurements.md` against `ir1022-final/*.log`,
`*.done`, `*.peak` and `ir1022-kani/healthy.log` plus journald:

- Steps 675,940/675,979/675,940/675,938/675,937.
- Symex 20.2/20.1/20.2/20.1/20.1 s.
- Verification 38.5/38.2/87.9/87.3/75.8 s.
- Wall 43.1/42.8/92.6/92.1/80.6 s.
- memory.peak 2.77/2.77/11.8/11.7/10.8 GB.
- Failing checks, playbacks 999/999/1000 and bytes 231,3.
- Bignum run: 1,971,414 steps and oom-kill.
- Toolchain nightly-2026-08-21.

All match, except the playback sentence in new FND-007. The record cites no `/tmp` path and no
commit hash, digest or pin (quire-exact is named as "the commit CG's `Cargo.lock` records").

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | AC-147/148 now say: "Partial installed-Kani evidence, not the installed control of FR-015-AC-41 or FR-015-AC-126". They name the `test:state-frame-twin` fixture harness, the hand-written subject that does not call `implies_short_circuit`, raw `cargo kani` and the 24 GiB cgroup with swap 0, and they cite the helper-calling control. New PLANNED FR-015-AC-151 (owner: the IR lead, upgrade path IR-1509) lists CG's runner, the two-phase run and identity record, the pipeline subject and QSL regeneration, the kernel `+` mutants and the sealed 999 playback. |
| FND-002 | fixed | Durable record `docs/qualification/ir1022-kani-clause-measurements.md` is committed: setup, exact command, results table, failing checks and playback, and what is not established. No `/tmp` path and no pin, hash or digest. Every number checked against the logs. |
| FND-003 | fixed | AC-41 ("no healthy-proof fit is claimed for that meter harness" plus the IR-1022 partial-evidence sentence), AC-126 (24 GiB cap, 2.77 GB, does not discharge the 16 GiB sentence), AC-132 (`TimedOut` at 16 GiB; the later partial measurements do not replace the controls) and FR-017-AC-63 are reconciled, and none claims more than the evidence. |
| FND-004 | fixed | AC-147/148 and the record say "cgroup `memory.peak`" and "(not process RSS)". |
| FND-005 | fixed | The `retain_boolean` sentence is removed from AC-147. |
| FND-006 | fixed | TC-025 step 55 is headed "FR-015-AC-143 to FR-015-AC-150". It names the scratch-crate test and the M1 site, and gives the installed-Kani setup (generated crate, hand-written subject, command, 24 GiB cgroup with swap 0, playback flags) and the FR-015-AC-151 PLANNED items. |

Round 2, head 9bbc7d89. `make spec` exit 0 with the 6 pre-existing warnings. FND-007 was fixed in
1f071146. The re-measured numbers that 9bbc7d89 puts in AC-147, AC-148, AC-41, AC-126 and the
record were each checked against `ir1022-final/r2{healthy,helperhealthy,mutant,oracle,helper}.{log,done}`:

- Steps 675,942/675,981/675,942/675,940/675,939.
- Symex 21.2/20.6/21.2/20.6/20.6 s.
- Verification 39.9/39.0/95.3/90.5/78.2 s.
- Wall 44.6/43.7/100.2/95.4/83.0 s.
- `memory.peak` 2.78/2.78/11.8/11.7/10.7 GB.
- Failing checks and playbacks 999/999/1000, and cover witness 1000 for mutants 1 and 2 only.
- r2helper prints one playback and has its cover unreachable.
- "Changed the step count by two" holds for every run.

All match. The r2 crates differ from each other only by the stated mutation. AC-143 and AC-149
and TC-025 step 55 now describe the cover-last order, and FR-015-AC-7/53/58 are unchanged.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-007 | fixed | The record now says "Mutants 1 and 2 each print a second playback with `amount_current = 1000` labelled `Check for cover: ...`". It adds that the helper mutant prints one playback (`1000`, the counterexample) and that its cover is unreachable, which matches `helper.log` and `r2helper.log`. |
