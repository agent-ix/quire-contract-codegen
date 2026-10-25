---
id: SR-17015
title: "IR-348 CG564 code-review and rust-review (AD-004 step 3 scalar split)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-348; PR564; exact 1aa7807ffd12c4582ae9c5e0a2767b9bf1792ec9; base 4eccd682545a240cf2289bf2040e820002bdf2e5; src/oracle/scalar/{mod,derive,lower,render,opaque_metadata_tests}.rs, src/oracle/{clause,equality/mod}.rs, src/kani/generate/{scalar,negotiate}.rs, src/routed/{capability,contract_package_tests}.rs, tests/it/{checked_term_consumers,exact_scalar_generation,layout}.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
---

## Summary

Ticket: IR-348 (AD-004 step 3, `exact_scalar` file split). This is an independent Opus review of CG #564 at the frozen head `1aa7807f` against base `4eccd682`, covering `git diff origin/main...HEAD` only. The Rust lane (rust-review with rust-style) is folded into this artifact.

Move fidelity was checked mechanically. Base `mod.rs` was compared with the head concatenation of `mod.rs`, `derive.rs`, `lower.rs` and `render.rs` as a whitespace-trimmed, visibility-normalised sorted line multiset. No base line is missing. The only added lines are licence headers, `use` lists, re-wrapped signatures, the `mod` declarations, one delegating `derive_exact_scalar_items` wrapper and one doc line. Claim/refusal logic and emitted-source text are therefore unchanged by construction. The `lib.rs` `pub use` set is untouched, and no moved item is re-exported from `mod.rs`, so there is no compatibility forwarding.

## Verdict

FAIL / not mergeable. The frozen head does not compile its lib test target (FND-001), so `make test` and `make lint` (`clippy --all-targets`) fail, and `make rustdoc` fails on 5 intra-doc links the move left unresolved (FND-002). The coder's reported scalar unit-test passes cannot have come from this head. There are also two medium AD-004 step 3 conformance gaps: `mod.rs` still holds stage implementations (FND-003), and tests and test helpers did not move with the behavior they exercise (FND-004). Production behavior is otherwise a faithful move.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Lib test target fails to compile: `lower.rs` calls the `#[cfg(test)]` `source_selection_work` module, which stays in `mod.rs` and is not imported (E0433 x4). `cargo test --lib`, `make test` and `make lint` fail at the frozen head. | src/oracle/scalar/lower.rs:2024; src/oracle/scalar/lower.rs:2027; src/oracle/scalar/lower.rs:2040; src/oracle/scalar/lower.rs:2046; src/oracle/scalar/mod.rs:1442 |
| FND-002 | medium | `make rustdoc` (`RUSTDOCFLAGS=-Dwarnings cargo doc --document-private-items`, part of `make ci`) fails: the move left 5 intra-doc links unresolved because the targets are no longer in scope in the new files: `catalogued_operation` (derive.rs x2), `generate_exact_scalar_oracles`, `derive_exact_scalar_items`, `SourceBuilder::finish` (lower.rs). | src/oracle/scalar/derive.rs:40; src/oracle/scalar/derive.rs:69; src/oracle/scalar/lower.rs:515; src/oracle/scalar/lower.rs:932; src/oracle/scalar/lower.rs:1802 |
| FND-003 | medium | `mod.rs` still holds stage implementations, against AD-004 step 3. Artifact rendering (provenance header, source-size limit, coverage-map and claim-map serialisation, artifact list) appears twice, in `render_exact_scalar_plan` and `build_contract_scalar_oracles`. The lowering helpers `node_implication_count` and `scalar_profile` also stay. | src/oracle/scalar/mod.rs:1044-1086; src/oracle/scalar/mod.rs:1373-1425; src/oracle/scalar/mod.rs:804-815; src/oracle/scalar/mod.rs:1494-1504 |
| FND-004 | medium | Tests did not move with the behavior they exercise, against AD-004 step 3. The 1,340-line `mod tests` in `mod.rs` unit-tests `derive`/`lower` internals. `opaque_metadata_tests` tests `lower::narrowing`/`references` but is declared from `mod.rs`. The `source_selection_work` instrumentation serves only `lower.rs`. `derive.rs`, `lower.rs` and `render.rs` have no test module. | src/oracle/scalar/mod.rs:1512-2851; src/oracle/scalar/mod.rs:2852; src/oracle/scalar/mod.rs:1442-1492 |
| FND-005 | low | Orphaned doc comment: `/// The lowered node and the bounds its descriptor was checked against.` (formerly on `type CheckedItem`) now documents `mod tests`. `CheckedItem` in `lower.rs` lost its doc. | src/oracle/scalar/mod.rs:1510; src/oracle/scalar/lower.rs:22 |
| FND-006 | low | The new stage modules import std, serde, quire_exact, quire_contract_model and checked_graph items through the parent's private `use` items (`use super::{BTreeMap, BigInt, Value, Serialize, aggregate_members, classify_lowering_failure, ...}`), not from their owners (rust-style: keep a `use` group to one module). The three new modules have no `//!` responsibility docs. | src/oracle/scalar/lower.rs:5-20; src/oracle/scalar/derive.rs:8-16; src/oracle/scalar/render.rs:5-12; src/oracle/scalar/mod.rs:51-53 |

## Coverage

The reviewer ran these at the frozen head in `target/` of this worktree:

- `cargo test --locked --lib oracle::scalar::` fails to compile: 4 × E0433 `source_selection_work` (FND-001).
- `cargo test --locked --test it exact_scalar_generation::` with no `QSPEC_DIR`: 76 passed, 3 failed. All 3 fail at `tests/it/exact_scalar_generation.rs:2359` with "QSPEC_DIR points at published QSpec". That is environmental: the env var is unset.
- The same filter was run with `QSPEC_DIR` set to a read-only `git archive` export of quire-specification `origin/main` `9f782921` `proposals/checked-package-v2`. Result: 79 passed, 0 failed.
- `cargo clippy --locked --all-targets -- -D warnings`: exit 101, the same 4 × E0433 in the lib test target (FND-001).
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --no-deps --document-private-items` (`make rustdoc`): exit 101, 5 unresolved links (FND-002).

Callers were checked as well. `clause.rs`, `equality/mod.rs`, `kani/generate/{scalar,negotiate}.rs`, `routed/capability.rs` and `routed/contract_package_tests.rs` now import from the owning stage module, with no root or forwarding path (L-2). The kani/generate edits are import-only, with no behavior change. CG #514 (not reviewed) also edits `negotiate.rs`, `scalar.rs` and `clause.rs`. `git merge-tree` of this head with #514's head `27d2cfd0` merges with no textual conflict. #514's new `clause/native.rs` reaches `render_clause_operation_helpers` via `super::` (clause.rs's private import), which still resolves after this PR. Whichever of #514 and #564 lands second must rebase and rebuild. That was not compiled here.

Panic and limit-kind scanners now cover all four scalar files. The ac42 test concatenates the four files for the `mod.rs` slot, so the "exactly one Failed arm" assertion still bites (the arm is now at `lower.rs:164`). No test assertion was narrowed.

Optional style note (not a finding): `derive.rs` and `lower.rs` import each other (`derive` → `lower::lowered`/`read_*`, `lower` → `derive::derive_operation_from_view`/`Derivation`). The stages are therefore not layered. AD-004 states no intra-module order for `oracle/scalar`.

Plan completion: not assessed

## Examined scope and bindings

```yaml
clean: false
scope:
  - {id: AD-004-step-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "It moves shared checked-node walkers into `core/ir`, moves descriptor derivation to `oracle/scalar/derive.rs`, node lowering, bounds and operation checks to `oracle/scalar/lower.rs`, and source and artifact rendering to `oracle/scalar/render.rs`. `mod.rs` keeps the public scalar types and entry points and orchestrates those stages; it does not retain their implementations. File-local helpers and tests move with the behavior they exercise."}
  - {id: AD-004-L-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The scalar file split is an additional step 3 exit check: `oracle/scalar/` declares `derive`, `lower` and `render` from `mod.rs`, and the scalar-specific logic resides in those files as the target tree assigns."}
  - {id: AD-004-L-2, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]` modules included, and no file imports an item through the crate root."}
  - {id: src/oracle/scalar/lower.rs, path: src/oracle/scalar/lower.rs, role: examined, excerpt: "#[cfg(test)]\n    source_selection_work::source(true);"}
  - {id: src/oracle/scalar/derive.rs, path: src/oracle/scalar/derive.rs, role: examined, excerpt: "pub(super) fn derive_exact_scalar_items("}
  - {id: src/oracle/scalar/render.rs, path: src/oracle/scalar/render.rs, role: examined, excerpt: "pub(super) fn standalone_oracle_source("}
  - {id: src/oracle/scalar/mod.rs, path: src/oracle/scalar/mod.rs, role: examined, excerpt: "pub(crate) mod derive;\npub(crate) mod lower;\npub(crate) mod render;"}
  - {id: callers, path: "src/oracle/{clause,equality/mod}.rs; src/kani/generate/{scalar,negotiate}.rs; src/routed/{capability,contract_package_tests}.rs", role: examined, excerpt: "use-path edits to oracle::scalar::{derive,lower,render}::..."}
  - {id: tests, path: "tests/it/{checked_term_consumers,exact_scalar_generation,layout}.rs", role: examined, excerpt: "scanner file lists widened from src/oracle/scalar/mod.rs to the four scalar files"}
bindings: []
```
