---
id: Plan-002
title: "IR-1114 — genuine CG consumer receipt readiness"
type: Plan
status: blocked
relationships:
  - { target: ix://agent-ix/quire-contract-codegen/FR-015, type: references }
  - { target: ix://agent-ix/quire-contract-codegen/FR-017, type: references }
  - { target: ix://agent-ix/quire-exact/FR-359, type: references }
  - { target: ix://agent-ix/quire-exact/FR-371, type: references }
---
# Implementation Plan: genuine CG consumer receipt readiness

## Requirements Summary

- [ ] Exact FR-359-AC-10: obtain individual genuine consumer proof and receipt outcomes without changing their domains, oracle, completion or counterexample checks.
- [ ] Exact FR-371-AC-6: qualify the genuine healthy, bound-mutant and arithmetic-mutant consumers, separately from kernel evidence.
- [ ] CG FR-015-AC-125/126 and FR-017: preserve authentic native generation, admission, execution and evidence boundaries.

## Readiness and Boundaries

This is the bounded IR-1114 preparation/receipt effort, not a second implementation of the completed numeric/state work in PLAN-001 Task-010. Exact owns arithmetic, Meter and the two acceptance criteria; CG owns these authentic consumer tests, generated artifacts and future receipts. Accepted AD-006 defines native API ownership, preserved semantic populations and independent qualification. Existing Exact FR-359, FR-371 and TC-919 define measurable healthy/mutant outcomes and refusal/inconclusive failure domains. These establish atomic obligations, owners, dependencies, verification and classified risks before tasking; this plan adds no requirement or public API.

Generation/profile inspection is low resource but can reveal admission or selector drift. Dependency adoption can invalidate existing evidence and must preserve owner type identity. Proof execution is high resource and currently prohibited: MemoryExhausted, timeout, compiler error, missing cover, missing counterexample or unconfirmed teardown earns no qualification. No prediction of fit, full-capacity proof, receipt, or completed criterion follows from a host check. The existing five consumer cases and native-route controls already exist; none should be copied or replaced.

## Dependency Graph

CG authentic generation and owner kernel adoption precede any current-kernel receipt: a valid result for an earlier proof subject cannot qualify a changed subject. Profile checks precede execution so an absent selector, changed proposition or altered resource envelope stops before backend launch. Explicit Peter authorization releasing the proof restriction is an independent prerequisite, not implied by this plan, its review, a planner message, or human acceptance of the plan. All proof work stays blocked while that prerequisite is absent. Individual receipts precede cross-criterion acceptance; the two attested cases add obligations beyond the three healthy/mutant controls.

## Test Plan

CG TC-025 and TC-027 own the executable obligations; Exact TC-919 step 6 explains the external consumer boundary. The non-Kani preparation selectors and five-row normative-to-current consumer mapping live in the task definitions. No trace tag, local Exact test, changed harness, stub typecheck or static profile check substitutes for a backend receipt. Any future native-route source/ABI change must preserve the original semantic domains, selected proposition, independent oracle, assertions, completion cover and substantive mutations under AD-006; record changed generated artifacts for independent review.

## Remaining Work

### Track A: Critical Path (serial)

- A1 = Task-011 preparation — small; exit: exact safe selectors are audited and the current consumer/profile mapping is grounded in its source move.
- Gate = Task-012 execution release and subject adoption — blocked; pass: Peter explicitly releases the prohibited proof work, current reviewed kernel adoption and consumer mapping are established, and the unchanged acceptance envelope is retained.
- A2 = Task-013 individual genuine receipts — blocked; exit: every required healthy, falsifying and native receipt result is retained and independently assessed, or each inconclusive outcome remains openly unqualified.

## Parallel Execution Summary

Task-011 → Task-012 → Task-013. No parallel proof track: the shared thermal lock permits one Cargo/proof job at a time, and no proof is authorized now.

## Task File Mapping

| Task | Track | Owns (references) | Verified by (verifies) | Status |
| --- | --- | --- | --- | --- |
| Task-011 | A | CG FR-015, FR-017; Exact FR-359, FR-371 | CG TC-025, TC-027 | not_started |
| Task-012 | Gate | CG FR-015, FR-017; Exact FR-359, FR-371 | CG TC-025, TC-027 | blocked |
| Task-013 | A | CG FR-017; Exact FR-359, FR-371 | CG TC-025, TC-027 | blocked |

## Coordination Rules

The current instruction authorizes specification/planning and non-heavy preparation only. No Kani, CBMC, 16 GiB proof, `make kani`, or `make ci` is an immediate action. Plan acceptance is a separate human workflow gate and never lifts that prohibition. Keep existing proof tests ignored; do not narrow criteria or alter domains/resources to manufacture qualification. Avoid unrelated open PRs, particularly TC-025 edits and exact-scalar support files; this bundle writes only its own plan files. No lock refresh merely to prepare this document, new pin, copied owner file/type/catalog, compatibility path or invented harness.

Every authorized Cargo command uses `flock /tmp/titan-cargo.lock nice -n10 ionice -c3 env RUSTUP_TOOLCHAIN=1.98.1 CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= CARGO_TARGET_DIR=<own-worktree>/target`. Inspect child Cargo launches before running a test: they must satisfy the same thermal policy or remain deferred. Run exact audited safe selectors, never a broad Kani filter or ignored-test set. Delete owned targets after work. Application versions and actual outcomes belong in operational evidence; this authored definition contains no run outcome or revision ledger. Runtime exact-head checks remain transient operator checks. Independent review and Luna's CG merge authority remain required for any future PR.
