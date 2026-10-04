---
id: "SR-1478"
title: "CG PR 270 spec review (scope boundary): the Kani-touching path set and the evidence rule"
type: SpecReview
analysis: scope-boundary
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@3da228d9eccae9196372574bd16e58a1ef5264f6; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md (Statement, Scope, Rationale, Open decisions), measured against src/** and tests/** at the base (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
---

# SR-1478: CG PR 270 spec review (scope boundary)

## Summary

Ticket: IR-604. The Kani-touching set was measured against the code the 23 lane tests reach:
their `use quire_contract_codegen::` items, their `crate::` and `#[path]` includes, and the
`crate::` imports of the listed source modules.

Correct exclusions: `src/strategy/**` and `src/evidence/**` are not reached. No file under
`src/kani`, `src/oracle`, `src/routed`, `src/replay` or `src/core` imports them, and no lane test
uses an item they export. So AC-2's `src/strategy/mod.rs` case is right. The `Makefile` exclusion is
stated honestly in Open decisions, and AC-8 (which runs in the default suite) guards a dropped filter.
The runtime the lane's scratch crates compile is pinned through the copied `Cargo.lock`, so listing
`Cargo.lock` is load-bearing. The spec leaves `.github/` untouched, calls the hosting question an
owner decision, and claims no workflow enforcement. The PR does not touch `.github/`.

## Verdict

Changes requested. One medium (the path set misses code the lane runs), one low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Kani-touching set leaves out code that the real-Kani lane runs. (1) `src/publication/**`: `kani_witness_join` calls `write_bundle_atomic` (`tests/it/kani_witness_join.rs:373`) to put the bundle on disk that real Kani then runs on. (2) `tests/it/scratch_crate.rs`: it writes the runtime dependency and copies `Cargo.lock` into every lane scratch crate (`kani_generation.rs:8`, `kani_obligations.rs:23`, `kani_witness_join.rs:26`). It is the file that makes `Cargo.lock` reach the lane at all. (3) `tests/exact_scalar_support/package.rs`, included by `#[path]` from `kani_obligations.rs:14` and `kani_obligations_state_frame.rs:21`. (4) `tests/state_frame_support/**`, included from `kani_obligations_state_frame.rs:17-24`. A change to any of these can turn the lane red while `make kani-scope` prints `not required`. Add them (or `tests/*_support/**`). A smaller gap: a `Cargo.toml` feature or `[patch]` edit that does not move `Cargo.lock`; list it or say why not. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:18-21 |
| FND-002 | low | The evidence rule is honest about enforcement ("nothing in the repository enforces it mechanically"). But it presents the line as more than a claim ("is the lane's own output") without saying that a pasted line is typed text: it can be copied from another run or written by hand, and only the head match narrows that. It also cites "the repository's hash rule", which no file in the repository defines. And it does not say that a passing line at the PR head says nothing about the merge result when another Kani-touching PR lands in between: the pre-tag run is the only backstop for that. Say all three in one or two sentences. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:103-109, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:96-98 |

## New findings (disposition pass 1)

Found in round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The set still misses one file the lane compiles. `tests/exact_scalar_support/package.rs:403` has `include!("../checked_package_support/base.rs")`, outside any cfg, and `package.rs` is the `#[path]` module that the lane modules `kani_obligations.rs:14` and `kani_obligations_state_frame.rs:21` use for their packages. A change to `tests/checked_package_support/base.rs` alone changes the lane's fixtures, yet `make kani-scope` prints `not required`. FND-001 named `package.rs` but not what it includes, so this was missed in the review pass. Add `tests/checked_package_support/**` and an AC-1 path, or state the rule once: the set includes the `include!` and `#[path]` targets of every listed file. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:18-23, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:73-81 |

## New findings (disposition pass 2)

Found in round 2, reviewed at 1c26606b6e97ddc8795a19e5d3cdb04ed4e2fba4.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The include rule (statement and AC-18) covers `include!` and `#[path]` but not `include_str!`, yet a lane test compiles `include_str!` text into the real-Kani scratch crate: `kani_obligations_state_frame.rs:62` reads `SUBJECT_SOURCE` from `../state_frame_support/subject.rs`. Today that target is in the set through `tests/state_frame_support/**`, so nothing escapes now. A future `include_str!` of a subject or fixture outside the set would escape `make kani-scope`, and AC-18 would not catch it. Extend the statement, AC-18 and step 11 to `include_str!` and `include_bytes!` targets, or say why they are out. The schema `include_str!`s in `bounded_kani_corpus.rs:563,610` are in default (non-lane) tests and would need the same walk to be limited to files holding lane tests, as AC-18 already is. Not blocking. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:61-62, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:169 |

## Dispositions

Round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76 (fix commit d4e8ed1 on top of 3da228d).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d4e8ed1: the set now lists `src/publication/**`, `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`, `tests/state_frame_support/**` and `Cargo.toml`. Re-measured at d4e8ed1: `kani_witness_join.rs` calls `write_bundle_atomic` (src/publication/publish.rs); the lane modules use `crate::scratch_crate`; the `#[path]` modules exist. No file under src/kani, oracle, routed, replay, core or publication imports `crate::strategy` or `crate::evidence`, and no lane test calls their exports, so the Scope's exclusion holds. The transitive include it misses is FND-003. |
| FND-002 | fixed | d4e8ed1: "What the evidence line is" says the line can be copied or typed by hand and that nothing in the repository checks it. The "hash rule" reference is gone. Merge skew has a merger statement (lines 48-50), and AC-14 inspects that the merged head contains every set-touching commit of origin/main. The pre-tag run is named as the backstop. (That statement's scope is SR-1476 FND-005.) |

Round 2, reviewed at 1c26606b6e97ddc8795a19e5d3cdb04ed4e2fba4 (fix commit 1c26606 on top of d4e8ed1).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 1c26606: `tests/checked_package_support/base.rs` is in the set and in AC-1 (16 paths; step 1 says sixteen). A new statement says the set contains every `include!`/`#[path]` target of a file in the set. AC-18 and TC-045 step 11 walk those targets transitively from the lane files, check them against the patterns read from `scripts/kani_scope.sh`, and require `base.rs` to be found, so the check is failable and not vacuous. I searched myself: from the seven lane files, `package.rs` reaches `base.rs`, which includes nothing further, and the `#[path]` modules of `state_frame_support/` include nothing. No `include!` or `#[path]` target outside the set is reachable. src/ has no `include!` or `#[path]` (only `include_str!` of fixtures in unit tests). See FND-004 for `include_str!`. |

Round 3, reviewed at 9536fe910bf94cc323196b371b06883983339953 (fix commit 9536fe9 on top of 1c26606).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 9536fe9: the include statement (NFR-006:62-64), AC-18 and TC-045 step 11 now cover `include!`, `include_str!`, `include_bytes!` and `#[path]`, walked transitively from each `tests/it` file that holds a lane test. Step 11 must find `base.rs`, `subject.rs` and the three Kani schemas. I searched myself. The lane-file targets are `state_frame_support/{model,native_twin,subject}.rs`, `exact_scalar_support/package.rs` (which includes `checked_package_support/base.rs`), `schemas/kani-proof-graph-v2` and `generated-rust-kani-v2` (`kani_generation.rs:519,523` default; `709,713` inside the ignored lane test `numeric_state_bindings_are_normalized_bounded_and_schema_valid`; `1256` default), and `schemas/kani-corpus-proof-graph-v1` (`bounded_kani_corpus.rs:563,610`, default tests). No other targets. The new patterns `schemas/kani-*.schema.json` and `schemas/generated-rust-kani-*.schema.json` match those three files and leave out `generated-rust-oracle-v1`, `oracle-source-map-v1` and `bound-coverage-observations-v1`. No lane file includes those three, and no `src/` file in the set reads them (only out-of-set `src/evidence/bound_coverage.rs` includes the coverage schema). AC-1 lists 18 paths. The statement now matches AC-18's scope (lane files only). No regression: `quire validate` exits 0, strict unbacked is 86 against 67 on main, the matrix rows are unchanged and agree, and no `.github` file is touched. |
