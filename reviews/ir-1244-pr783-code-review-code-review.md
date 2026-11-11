---
id: SR-17940486709564847047659
title: "IR-1244 PR #783 S1 pre-decoded Kani function harness code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #783 @ 2d3ebcf5 (code/ir-1244-predecoded-harness, slice S1); git diff origin/main...HEAD: src/lib.rs, src/oracle/claim.rs, src/oracle/function/{mod.rs,owner_path.rs,owner_render.rs,kani_harness.rs,kani_harness/tests.rs,kani_descriptor.rs,kani_descriptor/tests.rs}, tests/it/{kani_function_harness.rs,main.rs}"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: reviews
---

## Summary

Ticket: IR-1244 (slice S1 per the lead's amendment, Linear comment a0853d31: harness entry, the five AC-46 typed refusals, descriptor emitter, `cfg(kani)` rebuild, compile-only measurement; AC-45, AC-47 and TC-031 step 16 are S2 and stay PLANNED). Method: code-review with the rust-review lane folded in. Diff examined in full against the authoritative spec, FR-021 Behavior (about lines 377-455) and FR-021-AC-42, AC-44, AC-45, AC-46, AC-47 plus mutation rows 616-619, spec/oracle/matrix/tests.md:63, FR-047-AC-1/AC-8. Dependency sources read: quire-exact 5db2d5e (NodeKey, UnionMember, Debug impls), quire-semantic-value 3c4616d (TypeEnvironment derived Debug, PartialEq).

Executed in reviewer worktree /home/peter/dev/worktrees/IR-1244-review at 2d3ebcf5, sequentially under flock:
- Focused run: `cargo test --locked --test it -- kani_function_harness cover_last tc_031_ac19 tc_042_ac1` gave 12 passed, 0 failed (this includes both `kani_function_harness` tests, all `cover_last` tests, the tc_031_ac19 panic-free guard and the tc_042_ac1 src no-panic scan). `cargo test --locked --lib -- kani_` gave 51 passed, 0 failed, 1 ignored (installed backend); all 10 `kani_harness` and 3 `kani_descriptor` tests passed.
- Mutation control (one): the record-field presence in `declaration_descriptor` was forced to `Presence::Required`. `descriptors_cover_record_tuple_and_union_declarations` FAILED (`kani.contains("exact::Presence::Optional")`) and `kani_function_harness::rebuilt_composite_environment_matches_the_genuine_path` FAILED (Debug `left == right`). The mutation was reverted, and the tree was clean at 2d3ebcf5 before the measurement.
- Re-measurement at the final head (one run, compile-only, no retry): the emitted `same(left: Int[0,0], right: Int[0,0]): Boolean { left = right }` crate, taken from the integration test's own emission at 2d3ebcf5, plus the coder's hand-written `readiness::owner_path_codegen_only` proof. Command shape: `systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0 ... timeout 1800 flock ... cargo kani -j2 --only-codegen --unwind 2 --solver cadical`. Results: exit 0; wall 90 s (cargo 1m21s); scope memory peak 4.6G (journal "Consumed 2min 7.669s CPU time over 1min 30.006s wall clock time, 4.6G memory peak"); pretty-name map 131,733 entries, 26,516,704 bytes; symtab.out 51,008,804 bytes. Real Kani compiled the new `rebuilt` helper: the pretty-name map holds `rebuilt::<Integer, NonCanonicalInteger>`, `rebuilt::<Identifier, InvalidIdentifier>`, `rebuilt::<UnionMember, EmptyVariantName>` and one `kani::assert` symbol. No `CheckedPackageV2::read` or `acquire_checked_closure` symbol is in the harness map.

## Verdict

**CHANGES REQUIRED (no high finding).** These points are correct and were verified:
- (A) The `cfg(kani)` items hold no `CheckedPackageV2::read`, pre-built environment, membership verdict or `kani::assume`. The environment is built only through `TypeEnvironment::bounded_with_cancel` and `seal_union_verified`, with `UnionMember::from_admitted(declaration, identifier)` rebuilding the same `(NodeKey, identifier)` pair that the host view's `member.clone()` carries. `ready`, `finish` and `admits` are the unchanged shared HEADER code. A genuine environment failure stays `Err` in the struct, exactly as on the host. Membership, environment and guards are genuine: no fixture predicate, synthesized identity or widening.
- Descriptors cover all 20 `ValueType` variants by exhaustive match (no wildcard, so a new variant breaks the build) and NodeKey, including `PathSegment::Node` nesting, iteratively. The round trip is checked by Debug against a scratch crate. Debug is a sound proxy here: TypeEnvironment's Debug is derived over every field, and NodeKey's Debug prints its canonical Display, with quote and newline escapes exercised by the fixture keys.
- The `rebuilt` semantics are sound. `kani::assert(false, ..)` is a reachable CBMC property and is reported FAILED. The `loop {}` that follows is a self-loop, which CBMC symex replaces by `assume(false)` after the assertion, so it only cuts the already-failed path and cannot make a passing proof vacuous. The emitted proof is not vacuous on the rebuild path.
- The AC-46 mapping is: reader `Refused` gives `KaniReaderRefused`; `Incomplete` gives `KaniReaderIncomplete`; `CatalogUnavailable` gives `KaniReaderCatalogUnavailable` (exhaustive match, inspection-tested); any `CheckedFunctionError` from acquisition, selection, `checked_function_declarations` or `checked_composite_declarations` gives `KaniOwnerPathRefused{code}`; a nested-call body gives `KaniNestedCallBody{function}`. The three reader payloads are boxed.
- The entry takes no admitted package, declaring package or `ExactFunctionDeclaration`, and it classifies the body from the reader's own `Admitted` graph.
- The entry is own-unit only (imported owner is IR-1122).
- (D) The diff edits no spec or status text. It carries `#[trace("TC-031", ...)]` tags only (no `/// Trace:`), uses `#[cfg(test)] mod tests;` for both new test files, and adds no hashes or pins and no unsafe, unwrap or panic in new non-test code. Integer rendering is lossless (u64, usize and Integer through their canonical text).
- (E) The open PRs #583, #535, #474, #413 and #381 touch different `src/lib.rs` hunks (lines about 16, 70, 142, 164, 220 and 251). PR #783 edits only the `oracle::function` re-export block (about 106-116). No overlap.

The findings below are spec-gap or test-strength issues and documentation overclaims.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A selected body that is neither a supported scalar operator nor a supported equality, a multi-segment qualified name, or a missing source index is refused as the generic `InvalidPlanState`, which also signals renderer-internal inconsistency. AC-46 names only five causes, so this generation refusal has no typed cause. This is a spec gap for the lead (PR decisions a and d). | src/oracle/function/kani_harness.rs:83-110, src/oracle/function/kani_harness.rs:247-258 |
| FND-002 | medium | Behavior says the entry shall run `PreparedFunction::new` through the genuine owner path, and the entry's doc comment and the PR "What" section say it does. The entry actually runs only `checked_composite_declarations`; it never runs `bounded_with_cancel` or `seal_union_verified` on the host. An environment failure is therefore never observed at generation, and the claim overstates the code (PR decision e needs a lead ruling, or the wording must be corrected). | src/oracle/function/kani_harness.rs:218-245 |
| FND-003 | low | The `KaniNestedCallBody` refusal is returned only after the name, source-index and JSON lookups, each of which can return `InvalidPlanState` first and mask the nested-call cause. The name lookup also indexes `qualified_name[0]` instead of using a slice pattern. | src/oracle/function/kani_harness.rs:247-270 |
| FND-004 | low | `cancel_source` renders a fresh `Cancel::new()`, optionally pre-cancelled. A `Cancel::observing` link is silently dropped and the function has no doc comment stating the limitation (PR decision c). The proof models only the uncancelled or pre-cancelled state, which should be stated in code and noted for the spec's "Cancel sources" wording. | src/oracle/function/kani_harness.rs:188-200 |
| FND-005 | low | The PR body says "AC-42/44/46 flips left to the reviewer", and spec/oracle/matrix/tests.md:63 still reads "Planned (spec first, code follows)" after S1 code lands. The reviewer does not flip statuses. The row should be made honest by the coder as PARTIAL (S1 landed; AC-45, AC-47 and TC-031 step 16 PLANNED in S2), with AC-44 and AC-46 not claimed complete. | spec/oracle/matrix/tests.md:63 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-021-AC-42 | examined | The emitted function-oracle crate, compiled with cfg(kani), declares every body_{symbol} module pub(crate) with a pub(crate) Factory and omits every oracle_{symbol} host entry, and compiled without cfg(kani) is unchanged. |
| FR-021-AC-44 | examined | the cfg(kani) items of the emitted crate rebuild the prepared function from the literal descriptors Behavior lists, call TypeEnvironment::bounded_with_cancel and, for each union declaration, seal_union_verified, and run PreparedFunction::ready and finish with their admits guard. ... no CheckedPackageV2::read call |
| FR-021-AC-45 | context_only | For every harness the entry emits, the emitted crate embeds under cfg(not(kani)) ... every input of the genuine path ... (S2, PLANNED) |
| FR-021-AC-46 | examined | The Kani-harness entry refuses at generation time, with a typed refusal and no harness, const or partial bundle, each of five causes kept apart ... |
| FR-021-AC-47 | context_only | Over a tagged fixture with a record or union declaration ... AC-45's differential passes for the unmutated harness and fails for each of these adverse controls (S2, PLANNED) |
| FR-021 Behavior (Kani-harness entry) | examined | That entry shall run CheckedPackageV2::read over the retained bytes and proceed only on Admitted ..., then run acquire_checked_closure, select_checked_callable, checked_function_declarations and PreparedFunction::new through the genuine owner path. ... A descriptor rebuild that fails inside the proof is a failed assertion, never a skipped or assumed-away path. |
| spec/oracle/matrix/tests.md:63 | examined | FR-021-AC-44 through FR-021-AC-47 / TC-031 / Planned (IR-1244; spec first, code follows; IR-1121 stays blocked until that code lands) |
| FR-047-AC-1 | context_only | The harness drives the generated FR-021 body module directly ... over generation-time-decoded prepared data ... PLANNED (IR-1121) |
| FR-047-AC-8 | context_only | FR-047 gates its verdict on FR-021-AC-45's host differential (PLANNED, IR-1121) |
