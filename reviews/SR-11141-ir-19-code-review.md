---
id: SR-11141
title: "IR-19 claimed-module proof gate code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@486f11c56db3f0c3db656ceb8ba3b7299eed0384; Cargo.toml, Cargo.lock, scripts/kani_claims.json, scripts/kani_gate.sh, tests/it/kani_gate.rs, tests/it/kani_generation_claims.rs, tests/it/main.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-041", type: references }
---

## Summary

Ticket: IR-19. Reviewed the frozen seven-file diff against merged FR-041, TC-056 and NFR-006-AC-19, including the Rust classifier contract and existing gate behavior. The single-module positive control is present, but the acceptance path has multi-module, inconclusive-mutation, call-graph and stub-executor defects.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-041-AC-1 | examined | A nonempty reviewed declaration is checked against the current compiled-module census and the merge-base declaration. An absent compiled module, a removed claim, and an empty list each name the claim or condition and fail; adding a compiled claim does not erase an older one. |
| FR-041-AC-2 | examined | With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside only the first and an UNREACHABLE check inside the second reports the second `unreached` and fails. A SUCCESS check inside each passes this census; a SATISFIED cover, unknown location or non-SUCCESS status cannot supply the missing check. |
| FR-041-AC-3 | examined | The current build's transitive expectation/subject reachability yields the shared-helper intersection; the gate fails when the checked-in generated evidence omits or adds a helper, and records an explicit empty set when none is shared. For an exact operation, the expectation uses QSpec vectors or a model with no `quire-exact` call. |
| FR-041-AC-4 | examined | Each claimed module and computed shared helper has a positive unmodified real-Kani proof and a source mutation that compiles, executes, and reports a failed proposition check. Deleting a control or making its mutant verify, fail to compile, time out, skip execution, or merely trip the dirty-tree guard fails the gate; evidence names the exact diff, command and failed check from this run. |
| FR-041-AC-5 | examined | A stubbed executor, predetermined outcome or build-only run produces no qualifying module evidence and cannot pass the gate. |
| NFR-006-AC-19 | examined | On a clean current candidate, `make kani-gate` runs FR-041's module census and real mutation controls in its one required lane, and a failure of any claim or shared-helper control makes the gate exit non-zero. An optional mutation target or saved earlier result cannot substitute. |
| TC-056 | examined | Verify claimed-module discharge and real mutation controls in the existing real-Kani lane. |

## Verdict

**FAIL** — the four high-severity findings permit invalid acceptance or block a valid multi-module claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Adding a second claimed harness is rejected before its module census: `classify_kani_run` accepts exactly one report result, while this control runs all harnesses without selecting one | tests/it/kani_generation_claims.rs:364 |
| FND-002 | high | A mutant with a reported failed check but typed Inconclusive outcome can pass; the control never requires `KaniRunOutcome::Falsified` | tests/it/kani_generation_claims.rs:470 |
| FND-003 | high | An unmapped GOTO call-graph identifier is treated as a resolved raw name, so an unresolved reachable edge can escape the fail-closed check and hide a shared helper | tests/it/kani_generation_claims.rs:275 |
| FND-004 | high | The shell gate accepts a caller-supplied stand-in command that prints the expected test lines and writes two nonempty report files, without checking real Kani report contents | scripts/kani_gate.sh:102 |

## Coverage

The declaration is nonempty and compared with the merge-base list; SUCCESS attribution uses exact source file and module line extent, excluding UNREACHABLE and SATISFIED. The checked-in helper map is compared with the current graph intersection, and the one current module receives a real mutation in the author's focused run. The Cargo.lock delta adds only the declared proc-macro2 dev dependency; no generated artifact or copied file was added. Existing lock, freshness and six-filter count logic remains in place. IR-494's open branch overlaps only `tests/it/main.rs` and does not change this review's Kani runner path. No CI or full Kani lane was run in this review.
