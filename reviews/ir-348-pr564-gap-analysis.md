---
id: SR-17016
title: "IR-348 CG564 gap-analysis (AD-004 step 3 scalar split)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-348; PR564; exact 1aa7807ffd12c4582ae9c5e0a2767b9bf1792ec9; base 4eccd682545a240cf2289bf2040e820002bdf2e5; src/oracle/scalar/**, scalar callers, tests/it scanners, spec FR-014/TC-024/AD-004 as context"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-024", type: references }
---

## Summary

Ticket: IR-348 (AD-004 step 3, `exact_scalar` split). This is a planless gap analysis of CG #564 at the frozen head `1aa7807f`. `quire coverage --scope .` was run at head and over a `git archive` export of base `4eccd682`. Both report 626/979 rows backed. The outputs are identical except for the line number of one pre-existing `oracle-resembles-implementation` note, which follows the moved test. No trace tag or `// Implements:` comment was lost: the normalised line-multiset comparison of base `mod.rs` against the four head files shows no removed line.

Evidence for the PR's own behavior:

- With `QSPEC_DIR` set to a read-only export of quire-specification `origin/main` `9f782921` (`proposals/checked-package-v2`), `exact_scalar_generation::` passes 79 of 79 at head.
- The coder's 3 failures are environmental and are neither inherited code failures nor caused by this PR. `/home/peter/dev/quire-specification` is at `a286b81f`, which lacks `extension-definitions.json`. Without `QSPEC_DIR` the same 3 fail at the env lookup instead.
- The coder's lib QSpec-fixture `MalformedWire` result and the "20 passed / 1 passed" lib results cannot be reproduced at this head: the lib test target does not compile (SR-17015 FND-001). Lib-level scalar behavior evidence is therefore absent at the frozen head.

## Verdict

FAIL. Not mergeable until SR-17015 FND-001 is fixed and the lib scalar tests are re-run. Gap findings: the spec text still names `mod.rs` alone as the scanned scalar generator and the home of the one `Failed` arm (FND-001); one must-scan guard was not widened (FND-002); and the step 3 walker move is not done or explicitly deferred (FND-003). AC coverage is otherwise intact and no AC was narrowed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-014-AC-39, FR-014-AC-42 and their TC-024 steps still name `src/oracle/scalar/mod.rs` alone. AC-42 requires "exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/mod.rs`", but the arm is now at `lower.rs:164`. The tests were widened to the four files without a spec update or a recorded deferral, so the AC text is false as written. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:525; spec/oracle/functional/FR-014-exact-scalar-oracles.md:575; spec/oracle/functional/FR-014-exact-scalar-oracles.md:578; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:309; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:318; src/oracle/scalar/lower.rs:164 |
| FND-002 | low | `no_generation_panics` `MUST_SCAN` still lists only `src/oracle/scalar/mod.rs` among scalar files. The emitted-source generators now live in `render.rs` and `lower.rs`, which the walk covers but the must-scan guard does not pin. | tests/it/no_generation_panics.rs:25-37 |
| FND-003 | low | AD-004 step 3 says the `exact_scalar` PR also "moves shared checked-node walkers into `core/ir`". This PR moves none: `equality/mod.rs` now imports the shared range and text-bound readers and `COLLECTION_BOUNDS_MEMBERS` from `oracle::scalar::lower`. The PR body says only that "remaining AD-004 work stays open", without naming this as deferred. | src/oracle/equality/mod.rs:82-85; spec/assurance/AD-004-cg-crate-layout.md:1030; spec/assurance/AD-004-cg-crate-layout.md:297 |

## Coverage

- Gates the reviewer ran at head:
  - fmt: not re-run, coder reported exit 0.
  - `clippy --all-targets -D warnings`: see Gate addendum.
  - `make rustdoc` equivalent: see Gate addendum.
  - `cargo test --lib oracle::scalar::`: fails to compile.
  - `--test it exact_scalar_generation::`: 79/79 with current QSpec.
- No aggregate gate and no Kani were run, per the brief.
- spec-review was not applicable and was skipped: the diff touches no `spec/**` or `plan/**` file. The spec drift the code causes is recorded here as FND-001.
- AC bindings checked:
  - `tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site` → FR-014-AC-39 is correct; it now scans all four files.
  - `tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind` → FR-014-AC-42 is correct; it concatenates the four files for the scalar slot and still asserts one `Failed` arm.
  - `oracle_generators_have_no_panicking_arms` is widened to the four files.

Plan completion: not assessed

## Gate addendum

- `cargo clippy --locked --all-targets -- -D warnings`: exit 101 (lib test E0433 x4; SR-17015 FND-001).
- `make rustdoc` equivalent: exit 101 (5 unresolved intra-doc links; SR-17015 FND-002).
- `QSPEC_DIR=<origin/main 9f782921 export> cargo test --locked --test it exact_scalar_generation::`: 79 passed, 0 failed.

## Examined scope and bindings

```yaml
clean: false
scope:
  - {id: FR-014-AC-39, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "The generated `src/lib.rs` of every crate the TC-024 corpus generates, and the non-test code of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]` item removed wherever the item sits, string literals the generator emits counted), contain zero panic tokens"}
  - {id: FR-014-AC-42, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one shared classifier ... has exactly one such direct-lowering `Failed` arm in `src/oracle/scalar/mod.rs`, which binds nothing (`Failed { .. }`) and calls `classify_lowering_failure`"}
  - {id: AD-004-step-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "It moves shared checked-node walkers into `core/ir`, moves descriptor derivation to `oracle/scalar/derive.rs`, node lowering, bounds and operation checks to `oracle/scalar/lower.rs`, and source and artifact rendering to `oracle/scalar/render.rs`."}
  - {id: AD-004-L-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "`oracle/scalar/` declares `derive`, `lower` and `render` from `mod.rs`, and the scalar-specific logic resides in those files as the target tree assigns."}
  - {id: FR-039, path: spec/core/functional/FR-039-contract-package-consumption.md, role: examined, excerpt: "The crate-internal scalar consumers `generate_contract_scalar_oracles` and `preflight_contract_scalar_claim` in `src/oracle/scalar/mod.rs` ... also take `&ContractPackageV1`."}
  - {id: TC-024, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: context_only, excerpt: "Take the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/equality/mod.rs` and `src/oracle/function/mod.rs` ... and in each exactly one `Failed` arm"}
bindings:
  - {test_id: tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site, ac_id: FR-014-AC-39, trace: correct}
  - {test_id: tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind, ac_id: FR-014-AC-42, trace: correct}
```
