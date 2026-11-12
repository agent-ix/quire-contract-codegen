---
id: SR-17940486709564847047746
title: "IR-1447 PR #808 spec-review evidence"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen PR #808 at 56f1887d; IR-1447; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1447-kani-control-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1447. Honesty and sufficiency of docs/qualification/ir1447-kani-control-probe.md
against the probe directory scratchpad/probe-1447 (src/lib.rs, run.sh, out-*.log,
summary.txt), and of the verification methods named by AC-152..163. The probe is what it
claims to be: a hand-written scratch crate, no CG, kernel, meter or heap arm, and its caveat
against predicting real wall or memory is stated. The collection healthy re-run DID complete:
summary.txt, after "coll mutant retry", has `h::prove_coll mutant rc=1 ... FAILED WALL 0.27 s
... "collection if agrees"` and `h::prove_coll healthy rc=0 ... SUCCESSFUL WALL 0.31 s`.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | The record's collection figures are stale. It gives the healthy wall as 0.27 s and the Kani verification time as "not recorded", but lib.rs was edited (09:10) after the first healthy run. The surviving healthy log is the re-run (WALL 0.31 s, Verification Time 0.134 s, 1 of 1 cover), and the mutant log has Verification Time 0.112 s. The 0.27 s healthy figure no longer has a log behind it. Report the re-run figures. | docs/qualification/ir1447-kani-control-probe.md:50; scratchpad/probe-1447/summary.txt; out-h__prove_coll-healthy.log; out-h__prove_coll-mutant.log |
| FND-002 | low | The probe does not back the spec's if and case controls as specified. It compares the subject's integer selected value with the oracle, while FR-015-AC-153/157 use a Boolean clause `(...) >= 1` in which the probed if mutant is equivalent (base FND-001). Four of the five covers are `kani::cover!(true, "reached")`, so "cover satisfied" shows reachability only. Say both in "What this does and does not show". | scratchpad/probe-1447/src/lib.rs; docs/qualification/ir1447-kani-control-probe.md:55-69 |
| FND-003 | low | The record gives `--unwind 6` for every job, but `prove_let`, `prove_if`, `prove_case` and `prove_range` carry `#[kani::unwind(3)]`, which governs those harnesses. Only `prove_coll` uses 6. State the effective unwind per harness. | docs/qualification/ir1447-kani-control-probe.md:35-36; scratchpad/probe-1447/src/lib.rs |
| FND-004 | low | FR-015-AC-156 and TC-025 step 57 name the step `produced V2 Boolean result agrees with the independent checked condition`; the generated message is `the produced V2 Boolean result agrees with the independent checked condition` (leading "the"). Quote it exactly, since a test matches on it. | spec/kani/functional/FR-015-bounded-kani-obligations.md:984; src/kani/generate/clause/v2.rs:1114 |

## Verdict

The probe record is honest in kind, and the collection re-run is complete. The figures and
caveats above should be corrected. Test (TC-025) is the right method for every new AC.

## Dispositions

Round 1, reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10. The record was checked
against scratchpad/probe-1447/src/lib.rs, run2.sh, summary2.txt and v2-*.log. Every wall time,
Kani verification time and max-RSS figure in the v2 Results table matches its log: let 0.16/0.15 s,
0.024/0.021 s, 136 MB; if 0.14/0.17 s, 0.024/0.029 s, 135 MB; case 0.18/0.18 s,
0.040/0.041 s, 137 MB; nest 0.17/0.17 s, 0.030/0.028 s, 135 MB; coll 0.21/0.21 s,
0.054/0.056 s, 143 MB. Every failing check is the message named.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | Every harness was re-run as v2 (summary2.txt ends `DONE`). The collection row is now healthy 0.21 s / mutant 0.21 s, verification 0.054 / 0.056 s, from v2-coll-healthy.log and v2-coll-mutant.log. No figure without a log remains. |
| FND-002 | fixed | The v2 probe uses Boolean-result subjects asserted at `the produced V2 Boolean result agrees with the independent checked condition`, matching the ACs. The record says "The covers are weak ... 'cover satisfied' shows reachability, not non-vacuity of every branch". |
| FND-003 | fixed | run2.sh passes no `--unwind`. The record says "No `--unwind` flag was given: each harness carries `#[kani::unwind(n)]` ... `prove_coll` ... uses 6", and lib.rs agrees. |
| FND-004 | fixed | AC-156, AC-157, AC-158, AC-165, AC-170 and TC-025 step 57 all quote `the produced V2 Boolean result agrees with the independent checked condition`, matching src/kani/generate/clause/v2.rs:1114. |
