---
id: "SR-13150"
title: "CG PR 452 code review (Rust lane): FR-024-AC-10 source-scan test in tests/it/layout.rs"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@88af6e6ea8fe51647515e5cd5e7d2c941e65a7ed; tests/it/layout.rs, spec/replay/matrix/tests.md, spec/tests.md (diff origin/main...HEAD, base 87db2f7); read for context: spec/replay/functional/FR-024-counterexample-envelope-intake.md"
---

# SR-13150: CG PR 452 code review (Rust lane)

## Summary

Ticket: IR-878. PR: agent-ix/quire-contract-codegen#452 at 88af6e6. This is a non-author
review. The diff adds `qsl_replay_types_are_neither_defined_nor_imported_from_contract_ir` to
`tests/it/layout.rs` with `#[trace("TC-035", "FR-024-AC-10")]`. It splits the FR-024 row of the
replay matrix and edits the Replay line of `spec/tests.md`. No `/// Trace:` line is added, and
there is no manifest change: `syn` 2.0.119 with `full` and `visit` is already a dependency.

The test uses a real parser. `syn::parse_file` reads every `.rs` file that `source_files()`
returns. That walk covers the whole `src/` tree, including files that are not in the module
tree. A `syn::visit::Visit` walk then flags any struct, enum, union, trait or type alias named
after one of the five listed types, at any depth: nested modules, `#[cfg(test)]` modules and
function bodies. It also flags `use` trees rooted at `quire_contract_ir`, or at a file-local
rename of it, that name a listed type directly, through `as`, or through a glob. The constant
lists all five names the criterion lists: `Witness`, `ReplaySource`, `WitnessEnvelope`,
`TerminalRecord` and `ObligationIdentity`.

Gates were run in a throwaway worktree with its own `CARGO_TARGET_DIR`, `--jobs 2` and
`nice -n 10`, under `flock /tmp/luna-ir-heavy-3.lock`. Results:

- `cargo fmt --all -- --check` passed.
- `make lint` (clippy `--all-targets -D warnings`): see the Verdict.
- `cargo test --locked --test it layout::` ran 7 tests: 7 passed, 0 failed. No `QSPEC_DIR`
  was needed for this filter.

Mutation checks were run the same way for each mutant. The mutant's source was written as a
new file under `src/replay/`, then the built `it` binary was run with the `layout::` filter,
and the file was removed. The scan reads the source tree at run time, so no rebuild was
needed.

| Mutant | Result |
| --- | --- |
| All five names defined in a nested `mod a { pub mod b { … } }` (struct, enum, union, trait, type alias) | killed, 5 violations |
| `use quire_contract_ir::Witness as W;` | killed |
| `use quire_contract_ir::kani::*;` (glob of a module that re-exported the witness types in older IR) | killed |
| `pub type TerminalRecord = u8;` | killed |
| `#[cfg(test)] mod tests { struct ReplaySource; }` | killed |
| `fn f() { struct ObligationIdentity; }` | killed |
| `use ::quire_contract_ir::kani::witness::Witness;` | killed |
| `use quire_contract_ir::{self as q}; use q::kani::Witness;` | killed |
| `macro_rules! def { () => { pub struct Witness; } } def!();` | **survived** |
| `some_macro! { struct WitnessEnvelope; }` | **survived** |
| `use quire_contract_ir::kani as k; use k::Witness;` | **survived** |
| `extern crate quire_contract_ir as ir; use ir::Witness;` | **survived** |
| `use quire_contract_ir as ir; use self::ir::kani::Witness; mod c { use super::ir::kani::ReplaySource; }` | **survived** |
| `use quire_contract_ir::kani::witness::Witness::{self};` | **survived** |
| `fn f(_: quire_contract_ir::kani::Witness) {}` | **survived** |
| `type W = quire_contract_ir::kani::Witness;` | **survived** |
| `pub use quire_contract_ir as ir;` in one file and `use super::zz_a::ir::Witness;` in another | the new test misses it; only the existing AD-004 test `l_2_no_file_but_the_crate_root_re_exports_an_item` kills it |

At the IR revision in `Cargo.lock` (299a126), `quire_contract_ir` defines none of the five
names. So the import half guards against the types being reintroduced. It does not guard a
live import today.

## Verdict

The approach is sound. A real syn parse over the whole `src/` tree is the right tool. It
handles nested modules, `cfg(test)` modules, function-body items, type aliases, `as` renames
of the type, root aliases and globs. The scope is not what the criterion and the matrix row
claim, though. Definitions produced inside macro bodies, and imports reached through a
submodule alias, an `extern crate` alias or a `self::`/`super::` prefix, all pass. The test
states none of these limits. The `tests/` tree is outside the criterion ("under `src/`"), so
not scanning it is correct, not a gap.

`make lint` result: passed (clippy `--all-targets -D warnings`). There are two medium findings
on the new test, so it should not merge as is. Either close the escapes or write the limits
into the test's doc comment and the matrix row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The import half checks `path.first()` against `quire_contract_ir` and its file-local root renames only. An import reached any other way passes. Survivors: a submodule alias (`use quire_contract_ir::kani as k; use k::Witness;`), an `extern crate quire_contract_ir as ir;` alias, a `self::ir::…` or `super::ir::…` path after a root alias, and `…::Witness::{self}`. `collect_contract_ir_aliases` should record every rename whose resolved path starts at an IR root, and read `ItemExternCrate`. Matching should strip leading `self`/`super` segments, and a `self` leaf should be treated as the last path segment | tests/it/layout.rs:31-79 |
| FND-002 | medium | The definition half sees only items syn parses. A type defined in a `macro_rules!` body or a macro invocation (`def!()` expanding to `pub struct Witness;`, `some_macro! { struct WitnessEnvelope; }`) passes, yet FR-024-AC-10 says no such type "is defined under `src/`". Either scan `Item::Macro` / `ItemMacro` token streams for `struct`/`enum`/`union`/`trait`/`type` followed by a listed ident, or state this limit where the test and the matrix row claim coverage | tests/it/layout.rs:96-124, spec/replay/matrix/tests.md:17 |
| FND-003 | low | Path-qualified use without a `use` item (`fn f(_: quire_contract_ir::kani::Witness)`, `type W = quire_contract_ir::kani::Witness;`) passes. Read literally, the criterion speaks of imports only, so this is a judgement call. It is the same provenance leak, though, and a `syn::visit::visit_path` check costs little | tests/it/layout.rs:96-124 |
| FND-004 | low | The test has no doc comment saying what it asserts and what escapes it, unlike the module header's Risks note for L-2. The module header still describes the file as AD-004 L-1/L-2 only. The import violation names the file but not the type or the line, so a failure must be re-diagnosed by hand | tests/it/layout.rs:1-13, tests/it/layout.rs:81-83, tests/it/layout.rs:116-121 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The fix adds a `/// Trace: FR-024-AC-10` doc line above the `#[trace("TC-035", "FR-024-AC-10")]` attribute. The `Trace:` doc-line form is deprecated in this ecosystem; Rust tests bind criteria through ix-trace-rs `#[trace]` only, and the attribute already carries the binding. Remove the line (the rest of the doc comment stays) | tests/it/layout.rs:153 |
| FND-006 | low | The qualified-path check reads only the last segment of the resolved path (`banned_name` uses `path.last()`). An associated-item path through a listed type therefore passes: `quire_contract_ir::kani::Witness::default()` and, after `use quire_contract_ir::kani as k;`, `k::Witness::default()` both survive. The doc comment, the matrix row and the FR-024 prose all say Contract IR qualified paths are checked, with no such exception. Match any segment after the IR root that is a listed name (or every prefix of the resolved path), or state the exception | tests/it/layout.rs:118-126, tests/it/layout.rs:271-282 |

## Dispositions

Round 1, reviewed at 2acbf09a9e1ecd76cbef094e44214bd53317ca08 (fix commit 63f6dea8 over 88af6e6). Mutants were re-run against the rebuilt `it` binary, each written as a new file under `src/replay/` and removed after its run. Every mutant that survived round 0 now dies with a `file:line: <action> <type>` violation: submodule alias, `extern crate` alias, `self::`/`super::` after a root alias, `Witness::{self}`, path-qualified use with no `use` item, a type alias to the IR path, a crate alias in one file used from another (the new test now kills it as well as L-2), a definition inside a `macro_rules!` body and one inside a macro invocation. Every mutant that died in round 0 still dies. New probes: a generic argument (`Vec<quire_contract_ir::kani::ReplaySource>`), a hiding rename (`as Fine`), a nested-module alias and a trait bound in `impl` position all die. An opaque macro path (`my_macro!(quire_contract_ir::kani::Witness)`) survives, which is a stated limit. Associated-item paths survive (FND-006). Gates at this head: `cargo fmt --check` passed, `make lint` passed, and `cargo test --locked --test it layout::` ran 7 tests, all passing.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `use_paths` and `resolve_path` now build a cross-file alias map keyed by module path, including submodule renames, `extern crate` renames, `self`/`super`/`crate` prefixes and `self` leaves; all four reported escapes die |
| FND-002 | fixed | `visit_macro` scans every macro's tokens with `macro_definitions` for `struct`/`enum`/`union`/`trait`/`type` followed by a listed name; both macro mutants die. Macro expansion is stated as outside the scan |
| FND-003 | fixed | `visit_path` resolves every path through the alias map; the fn-argument and type-alias mutants die. The associated-item residue is recorded as FND-006 |
| FND-004 | fixed | The test has a doc comment stating its scope and limits; the module header names FR-024-AC-10; violations read `file:line: <action> <type>` |
| FND-005 | still-open | the `/// Trace: FR-024-AC-10` line was added in this fix round and must be removed |
| FND-006 | still-open | associated-item paths through a listed IR type pass although the doc, the matrix row and the prose claim qualified paths are checked |
