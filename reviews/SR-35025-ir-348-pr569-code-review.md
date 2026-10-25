---
id: SR-35025
title: "IR-348 CG569 code-review and rust-review (AD-004 L-8 state-frame slice)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-348; PR569; exact 34424f6115361c2bcf3b7afdacafc758b8012a00; base dd59aa14b8af9d8f34638669125736997817f419; src/core/ir/frame.rs, src/core/ir/mod.rs, src/kani/generate/frame.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
---

## Summary

Ticket: IR-348 (AD-004 L-8, `state_frame` typed-node-access slice). This is an independent Opus review of draft CG #569 at the frozen head `34424f61` against base `dd59aa14` (= `origin/main`), covering `git diff origin/main...HEAD` only. The Rust lane (rust-review with rust-style) is folded into this artifact.

Behavior preservation was traced path by path. `Graph::shape` replaces the `node_tag`/`semantic_form` reads with the same string equality. `Graph::scope` is `read_scope` with the same anchor, member, reference and frame-shape checks, mapped field by field onto `StateFrameScope`. `Located::referenced_node` carries the id of the same node that `node` held. `clause_kind`, `application_identity` and `projected_field` return `None` exactly where the old inline `Value` reads failed. Each `None` maps to the same refusal (`MalformedClause`, `ConditionNotSupported` or `NotAStateClause` with the same strings), and the order of checks is unchanged. No `unwrap`, `expect`, `panic!` or indexing was added outside tests. No test was removed or weakened. `kani/generate/frame.rs` no longer names `serde_json::Value`, `.get("`, `.node_tag`, `.semantic_form` or `.nodes`.

## Verdict

FAIL / not mergeable at `34424f61`. The new `pub(crate) use crate::core::ir::frame::Graph;` re-export fails the existing layout test `layout::l_2_no_file_but_the_crate_root_re_exports_an_item` (AD-004 L-2), so `make test` is red (FND-001). The author's focused checks did not run `layout::`. The fix is small: import `Graph` from `core::ir::frame` in `replay/state_clause.rs` and delete the re-export. Production behavior is otherwise preserved, and the PR body truthfully keeps L-8 open: it does not touch AD-004, and the scalar, equality and function slices are not claimed. FND-002 (stringly node kind instead of the model's typed kind) is a medium departure from AD-004's typed-tag design. The rest are low-severity duplication and hygiene items.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `pub(crate) use crate::core::ir::frame::Graph;` keeps the old `kani::generate::frame::Graph` path alive as a forwarding re-export, which `replay/state_clause.rs:45` still imports. That breaks AD-004 L-2 and step 3 ("no old-path forwarding module"). The existing test `layout::l_2_no_file_but_the_crate_root_re_exports_an_item` fails at this head: `a re-export in a directory: ["kani/generate/frame.rs: pub(crate) use crate::core::ir::frame::Graph;"]`. Import from `core::ir::frame` in `replay/state_clause.rs` and drop the re-export. Note that codex2's L-6 work also edits `replay/state_clause.rs`, so expect a one-line import conflict. | src/kani/generate/frame.rs:56; src/replay/state_clause.rs:45 |
| FND-002 | medium | `NodeShape` erases the node kind to `&str` (`tag`, `form`), and callers still match string literals (`"state"`/`"state_clause"`, `"operation_anchor"`, `"frame"`, `"value"`/`"parameter"`). The model exposes typed kinds: `CheckedPackageV2::node_kinds()` yields `CheckedNodeKind` (for example `State(StateForm::OperationAnchor)`), and `CheckedNodeTag` is already used in this file. AD-004's `core/ir` layer "exposes a node view with a typed tag". This slice moves the stringly lookup instead of typing it. Raw strings are needed only for the `NotAStateClause` refusal text. | src/core/ir/frame.rs:17-26; src/core/ir/frame.rs:55-60; src/core/ir/frame.rs:86; src/core/ir/frame.rs:94; src/kani/generate/frame.rs:621; src/kani/generate/frame.rs:824-826 |
| FND-003 | low | `Graph` keeps the `pub(crate) package` field and also adds a `package()` accessor, so there are two ways to reach the package. `field_range` uses the accessor (`kani/generate/frame.rs:848`), while `replay/state_clause.rs:1032` uses the field. Make the field private and use the accessor. | src/core/ir/frame.rs:30; src/core/ir/frame.rs:47-49; src/replay/state_clause.rs:1032 |
| FND-004 | low | `core/ir` now has two parallel graph views and duplicated helpers. `frame::Graph` (package plus a `BTreeMap` node index, last-wins on a duplicate id) duplicates `clause::ClauseGraphView`/`ClauseNodeIndex` (first-wins). `frame::{application, node_id, reference}` duplicate `clause::{arguments, node_id, reference}`. AD-004 calls for "one typed node-access layer". Put the shared helpers in `checked_term.rs` and keep one graph view. | src/core/ir/frame.rs:29-45; src/core/ir/frame.rs:121-135; src/core/ir/frame.rs:168-181; src/core/ir/clause.rs:46-73; src/core/ir/clause.rs:76-119 |
| FND-005 | low | The new boundary test is marked with a `/// Trace: AD-004 L-8.` doc line. That convention is deprecated: the repo binds tests with ix-trace-rs `#[trace(...)]`, and new `Trace:` lines should not be added. Drop the line, or bind with `#[trace]` if an AC owns L-8. | src/core/ir/frame.rs:185 |
| FND-006 | low | The PR body says reference following moved to `core/ir`, but `ClauseShape::read` still destructures the parameters `CheckedTerm::Aggregate` and matches `CheckedTerm::Reference { target }` inline. That duplicates the new `core::ir::frame::reference` helper. | src/kani/generate/frame.rs:642-652; src/core/ir/frame.rs:175-180 |

## Coverage

The reviewer ran these at the frozen head in this worktree's own `target/`, with `RUST_TEST_THREADS=4`:

- `cargo fmt --check`: exit 0.
- `cargo clippy --locked --lib --tests -- -D warnings`: exit 0.
- `cargo test --locked --test it -- kani_obligations_state_frame:: replay --skip real_kani`: 114 passed, 0 failed, 3 ignored. This covers the author's 75 state-frame tests plus the replay tests that consume `Graph`/`ClauseShape`.
- `cargo test --locked --lib -- core::ir:: kani::generate::frame:: replay::state_clause::`: 7 passed, 0 failed.
- `cargo test --locked --test it -- layout::`: 7 passed, 1 FAILED (`l_2_no_file_but_the_crate_root_re_exports_an_item`, FND-001).
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --no-deps --document-private-items` (`make rustdoc`): exit 0.
- `QSPEC_DIR` was not needed for these filters. The aggregate gate and Kani were not run; the code lead runs them on the final head.

Overlap check: open PR #567 (L-12) edits only `spec/assurance/AD-004-cg-crate-layout.md`. The L-6 worktree (`ir348-l6-output-reader`, `a2e85a90` plus uncommitted edits) edits `kani/{classify,output,run}`, `replay/{frame,function,scalar,state_clause,witness}.rs`, `lib.rs` and tests. Neither PR touches this PR's three files, so there is no file overlap. FND-001's fix would touch `replay/state_clause.rs`.

Plan completion: not assessed

## Examined scope and bindings

```yaml
clean: false
scope:
  - {id: AD-004-L-8, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "L-8. `serde_json::Value` and `.get(\"` appear for body terms only in `core/ir`. Test: a grep gate."}
  - {id: AD-004-step-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "this step introduces no old-path forwarding module or other compatibility layer. L-8 lands with the last typed-node-access PR."}
  - {id: AD-004-core-ir, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "One typed node-access layer `core/ir/`: the only code in CG that opens a `CheckedPackageV2` body term. It exposes a node view with a typed tag, typed member accessors and typed errors"}
  - {id: src/core/ir/frame.rs, path: src/core/ir/frame.rs, role: examined, excerpt: "pub(crate) struct Graph<'g> { pub(crate) package: &'g CheckedPackageV2, nodes: BTreeMap<..> }"}
  - {id: src/kani/generate/frame.rs, path: src/kani/generate/frame.rs, role: examined, excerpt: "pub(crate) use crate::core::ir::frame::Graph;"}
  - {id: src/core/ir/mod.rs, path: src/core/ir/mod.rs, role: examined, excerpt: "pub(crate) mod frame;"}
bindings: []
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The repair moves the frame path from the removed `Graph` (a last-wins `BTreeMap` collect, the same as `CheckedPackageV2`'s own `node_index`, which `package.term(id)` reads) to `ClauseGraphView` (first-wins, for both nodes and kinds). If an admitted graph ever repeated a `node_id`, `shape`/`kind`/`follow` could describe the first node while `term()` returns the last node's body. The reviewer could not confirm that admission refuses duplicate node ids. The behavior is unchanged when ids are unique. The pre-existing `clause.rs` reader already has this property. | src/core/ir/clause.rs:57-62; src/core/ir/clause.rs:77-86; src/core/ir/frame.rs:19-31; src/core/ir/frame.rs:45-50 |

## Dispositions

### Round 1 — exact `69983f712143387126266e77982ce4a872a5b63a` (fix diff `34424f61..69983f71`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The `pub(crate) use crate::core::ir::frame::Graph;` re-export is deleted. `replay/state_clause.rs:41` imports `core::ir::clause::ClauseGraphView` directly. No `pub use`/`pub(crate) use` remains outside `lib.rs`. The reviewer ran `cargo test --locked --test it -- layout:: ...` and `l_2_no_file_but_the_crate_root_re_exports_an_item` now passes. |
| FND-002 | fixed | `NodeShape` carries `kind: CheckedNodeKind` from `CheckedPackageV2::node_kinds()`. Every check is now typed: `CheckedNodeKind::State(StateForm::StateClause)` (`kani/generate/frame.rs:622`), `OperationAnchor` and `Frame` (`core/ir/frame.rs:55`, `:63`), and `Value(ValueForm::Parameter)` (`kani/generate/frame.rs:820`). `tag`/`form` strings remain only for the `NotAStateClause` refusal text. |
| FND-003 | fixed | `Graph` is gone. `ClauseGraphView.package` is private, and both `field_range` and `replay/state_clause.rs` `state_fields` use `package()`. |
| FND-004 | fixed | There is now one graph view: the frame accessors are inherent methods on `clause::ClauseGraphView` (`core/ir/frame.rs:18-73`). `node_id`, `reference` and `application` live once in `core/ir/checked_term.rs:9-40`, and `clause.rs` uses them. |
| FND-005 | fixed | The `/// Trace: AD-004 L-8.` line is removed, and the test carries no trace tag. |
| FND-006 | fixed | Parameter reference decoding is `core::ir::frame::parameter_references`. `ClauseShape::read` calls it with the same `malformed` refusal. |

FND-007 is new in this round. It is low and does not block merge: it is unreachable unless admission accepts a repeated `node_id`. It is routed to the code lead to accept or defer.

Gate evidence at `69983f71`, reviewer-run in the worktree's own `target/`, `RUST_TEST_THREADS=4`:
- `cargo fmt --check`: 0.
- `cargo clippy --locked --lib --tests -- -D warnings`: 0.
- `cargo test --locked --test it -- layout:: kani_obligations_state_frame:: replay --skip real_kani`: 121 passed, 0 failed, 3 ignored.
- `cargo test --locked --lib -- core::ir:: kani::generate::frame:: replay::state_clause::`: 7/7 passed.

The aggregate gate and Kani were not run.
