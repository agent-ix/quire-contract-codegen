---
id: SR-17940486709564847047750
title: "IR-1437 PR #810 NFR-006 amendment integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #810; IR-1437; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md; spec/kani/matrix/TC-045-real-kani-lane-gate.md; spec/kani/functional/FR-055-kani-mutations-target.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
---

## Summary

Ticket: IR-1437. Integrity lens: is the NFR-006 amendment consistent with the rest of NFR-006,
with NFR-006-AC-19 and with FR-055? Before the PR, `make -n ci` ran no `kani` command. The PR adds
"`make ci` shall depend on `kani-mutations`" and rewrites AC-12 and TC-045 step 12. The rationale,
NFR-006-AC-16 and the "not charged to any other change" rule were not updated, and they now
contradict the new statement. The AC-19 intent (an optional target cannot stand in for the gate's
own controls) is preserved by FR-055's "neither replaces nor is replaced by" text.

## Verdict

**FAIL.** One high contradiction inside NFR-006, one medium untestable or vacuous check, and one
low overlap with AC-19.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New statement "`make ci` shall depend on `kani-mutations`" contradicts unamended NFR-006: the rationale rejects adding real Kani to `make ci` (it runs twice per change and would contend for the lock), keeps "`make ci` stays as it is" and "not charged to any other change", and AC-16 says a `not required` change has no mandatory real-Kani run. FR-055 also runs the target "under the host lock" | NFR-006, NFR-006-AC-16, NFR-006-AC-12, FR-055 |
| FND-002 | medium | AC-12 / TC-045 step 12 / FR-055-AC-3 match text tokens in `make -n ci` output, which prints recipe lines, not target names. "`kani` lane command" is undefined. A correct recipe such as `scripts/check_kani_mutations.py` fails step 12, and any line that names `kani-mutations` while running lane filters passes, so the amended AC no longer detects `make ci` running the lane | NFR-006-AC-12, FR-055-AC-3, FR-055-AC-8 |
| FND-003 | low | FR-055 draws its mutants from FR-041 controls as well, and those still run in `kani-gate` (AC-19), so the same real-Kani controls run in both `make ci` and the gate. AC-19's "an optional mutation target" no longer names this target, which is now mandatory | NFR-006-AC-19, FR-055 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| NFR-006 | examined | The `make ci` target shall not depend on `kani-gate` or `kani`. The `make ci` target shall depend on `kani-mutations` (FR-055). |
| NFR-006-AC-12 | examined | `make -n ci` expands to no `kani-gate` command and no `kani` lane command other than `kani-mutations` (FR-055). |
| NFR-006-AC-16 | examined | A pull request whose scope is `not required` has no mandatory real-Kani run under this requirement. |
| NFR-006-AC-19 | examined | `make kani-gate` runs FR-041's current-build module census and real mutation controls in its one required lane ... An optional mutation target, retired fixed inventory or saved earlier result cannot substitute. |
| NFR-006 Rationale | examined | Rejected: adding the lane to `make ci`. `make ci` is the gate for every change ... runs twice per change; the lane's cost would dominate it, contend for the lock and push agents to skip `make ci`. |
| TC-045 step 12 | examined | run `make -n ci` and assert no line contains `kani-gate` and no line contains `kani` other than `kani-mutations` (FR-055-AC-8). |
| TC-045 Expected Results | examined | `make ci` does not run the lane (it runs only `kani-mutations`). |
| FR-055 | context_only | It is run through the Kani lane, under the host lock and serially, as `make kani-gate` is. |
| Makefile ci recipe | context_only | ci: qspec-preflight; $(MAKE) -j1 fmt-check spec lint msrv deny rustdoc test |
