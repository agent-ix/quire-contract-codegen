---
id: SR-16302
title: "IR-988 CG #555 function source index code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; CG #555 139b2815; src/oracle/function/mod.rs, tests/it/exact_function_generation.rs; FR-021-AC-13, FR-021-AC-22, FR-021-AC-27"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Ticket: IR-988. Reviewed the complete CG #555 diff at 139b2815 against 33eca811, the caller and focused integration evidence. The Rust review covered source-reference lookup, duplicate source identities, surviving declaration selection, refusal order, output order, ownership, and the test counter. No code defect was found. `git diff --check` and `cargo fmt --all -- --check` passed. The author's focused test and clippy results were read from the PR description; this reviewer did not rerun builds or aggregate gates. Final-head gates remain due after the #513 rebase.

## Examined criteria

- FR-021-AC-13 (examined): "Schedule source and claim-map bytes are identical across runs and request permutations; entries order by call-node id, declaring function-node id, then source argument-node ids, with the name tie-break AC-24 specifies."
- FR-021-AC-22 (examined): "When two or more declarations share one declaring node id, none of them appears in the instruction table or in `location-map.json`, and every item naming one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }` ... before Stage 1 classification."
- FR-021-AC-27 (examined): "An admitted nested body consists of separate V2 expression nodes connected only by `reference` leaves under QSpec FR-322 ... callee argument 0 binds an exact V2 function node ... and remaining arguments preserve source order."

## Verdict

**PASS** for the code/Rust review at this exact head. The index retains the graph's prior key resolution and maps duplicate `(name, body)` source matches to `None`, the same `BodyMismatch` path as the previous uniqueness scan. `FunctionNames::resolve` still precedes survivor selection and package refusal. The indexed declaration returned to the pending list is the same resolved declaration, preserving output order and bytes on the examined paths. Evidence completion is assessed separately in SR-16303.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## New findings (disposition pass 1)

Narrow review of the 139b2815..7783fabd evidence-fix delta. The admitted fixture and public-path K/R measurement are sound on inspection, but the new output assertion violates CG's own committed `AGENTS.md` and `CLAUDE.md` rule against recording file hashes and checksum catalogs. This changes the exact-head code-review verdict to **CONDITIONAL** until the assertion is represented without a committed digest catalog.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The nine literal SHA-256 values form a checksum catalog for generated artifact and report bytes; CG's `AGENTS.md` explicitly forbids introducing hash records that track files or tools, permitting only canonical proof-identity digests | src/oracle/function/mod.rs:1999-2016; AGENTS.md:3-9 |

The test computes SHA-256 over `artifact.path` and `artifact.contents`, followed by serialized claim and location maps. These are generated files and reports, not canonical identity digests that bind a proof to its exact content. A direct structured output comparison or a focused non-digest baseline report would satisfy the behavioral check without adding this catalog. No production-code defect was found in the delta.

## Dispositions

Disposition pass 2, against CG #555 head dbae5e0c14539ef263479138972583751ed0c975. Narrowly reviewed only the 7783fabd..dbae5e0c test delta. The code/Rust verdict is **PASS** at this exact head; the #513 rebase and final-head gate remain outstanding.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The committed SHA-256 import, digest computation and all nine literal output checksums are removed. The public admitted test compares complete `ExactFunctionOracles` values across repeated generation and retains K generated plus one typed `BodyMismatch` assertions. A separate, uncommitted full-output export from the pre-change main and indexed branch was compared bytewise across all nine K/R cells; every corresponding file was identical. The export control is test-only and opt-in through `IR988_OUTPUT_DIR`. |

### After excerpt

`let repeated = generate_exact_function_oracles(&contract, &positions, &function_positions, &declarations, &item_positions, &items).expect("repeat generation from the same admitted package"); assert_eq!(output, repeated, "complete output changed across repeat runs");` (`src/oracle/function/mod.rs:1989-2001`). The reviewer's direct comparison of `/home/peter/dev/qualification-logs/ir988-cg555/direct-before/` with `direct-after/` found 9/9 same-named, byte-identical full output files.

## Incremental combined-slice review

Reviewed CG #555 draft head 63740402c0a33791fc02ffd44b16e7dd40a223e7 against its PR base bba613ef. The five earlier function commits are patch-identical by `git range-diff` to the reviewed 717b3bd8 series; `src/oracle/function/mod.rs`, `src/lib.rs` and the two custody reviews have no edits after their rebased carry at 670f3867. The incremental production diff is `src/oracle/scalar/mod.rs:1485-1507`; the rest adds two routed scalar tests and one composite retained-context test. The bound strategy and negotiation files are absent from this PR's active diff and are excluded from this verdict.

Additional examined scope:

- `src/oracle/scalar/mod.rs:1485-1507`, `src/routed/contract_package_tests.rs:2509-2720` (examined): source occurrence selection, typed `IncompleteContext` refusals, generated sibling and package-local selected result bounds; FR-039-AC-2 and FR-039-AC-7.
- `tests/it/composite_original_eq.rs:954-1058` and retained `src/replay/composite_builder.rs:208-225,255-370` (examined): one context constructed before repeated Eq/Ne requests, typed operator mismatch, original package identity, positional operand/preimage and consumed-work observation; FR-033-AC-1, with FR-018-AC-33/35 as context.
- IR `ContractPackageV1::source_occurrences` at owner revision 1208489 (context only): its retained `source_index` stores the first entry by node id, matching the former CG `.iter().find` first-match rule; it returns the original occurrence slice without reordering.
- CG #542 head 1af1e8a7 (context only): it rewrites much of `src/oracle/scalar/mod.rs` and still contains the old `selected_scalar_key` scan; the added accessor hunk must be carried and rechecked after #542 merges.

**Current code/Rust verdict: PASS by diff inspection at 63740402.** The scalar replacement preserves the application/conversion role and ordinal filters, context-root check and `result_context` refusal order. The composite addition changes tests only, and it explicitly uses one retained context per K/R cell. No new code defect or rule violation was found in the incremental slice. The draft's locked IR model revision predates `source_occurrences`, so compilation and aggregate gate evidence apply only after the actual #546/#542 base carry. This is not a final-head gate claim.

## #546 base carry

Narrow review of CG #555 head 3063ce875f1fcad0c2ceace999131e0d14a46a55 on actual main a3aad3b757bdd3b9028094c0b562c26970aaa3d3. `git range-diff bba613ef..63740402 a3aad3b7..3063ce87` matched eight commits exactly; the third commit differs only because #546 added the independent `checked_package_evidence` test registration immediately before this PR's unchanged `ir988_function_fixture` registration in `src/lib.rs`. The five-line registration addition is identical on both bases.

The full binary diff across the function, routed-test, composite-test and two custody-review files is byte-identical between old-base and new-base patches (SHA-256 `94c80d93ab8b30be702d6a4cb660eb67aa0b1648c036b3475b01c12b0c4ee176`). The scalar patch ID is identical (`b74dbb71a82945f3e5214a451cb9901f0c45e6e1`); the resulting scalar file differs only because its new base includes #546 changes. The current selected-scalar hunk still calls the retained `source_occurrences` accessor before the same role/ordinal, context-root and `result_context` checks. `Cargo.lock` now inherits owner `quire-contract-model` revision 120848913526dc57a07de089010df392813ea050 from #546 main, and #555 changes no lock entry. That owner accessor indexes the first source-map node by key and returns the original occurrence slice, preserving first-match order. `cargo fmt --all -- --check`, `git diff --check` and Quire validation passed for this carry.

**Carry verdict: PASS by narrow inspection at 3063ce87.** No product or test behavior was authored by the rebase. `src/strategy/bound/generation.rs`, negotiation, and `src/replay/composite_builder.rs` remain outside the active PR diff. The composite test blob is unchanged and still documents one retained context per K/R cell. CG #542 remains unmerged and rewrites scalar internals, so its actual merge/rebase requires another narrow overlap review and the final-head gate.

## #542/#557 base and scalar evidence carry

Narrow same-reviewer inspection of CG #555 exact head `e6257910e279aa39104a51ae7550e3549aa02773` on merged main `464f216cd32582422c5da527d1332ef9580e01a1`. The nine prior commits carry from `3063ce87` in order; `git range-diff` shows commits 4-9 exact. Commits 1-3 adapt the prior source index and admitted test to #542's `CheckedGraph` and typed `CheckedTerm`. In `FunctionSourceIndex::new`, `graph.nodes()` retains complete-key ordering; `graph.term(&node.node_id)` supplies the typed body, `binding_value` still chooses the first `body` binding, and `reference_in_graph` still requires a represented target. The map still marks duplicate `(name, body)` keys ambiguous, yielding the prior `BodyMismatch`. `FunctionNames::resolve`, stage refusal and survivor lookup remain in the same order. Commit `84484377` changes borrow/layout only; `e6257910` moves the existing admitted test fixture registration into its oracle module, without changing the public fixture. The post-#542 typed function test still checks one source-index graph visit per admitted node, nine K/R cells, complete repeated output equality and one typed refused sibling. Fresh typed-cohort before/after full-output measurement was reported as in progress by the author; no claim of such a comparison is made here.

The new scalar test in `source_selection_tests.rs` builds genuine QSL packages, admits the checked and lowered packages through the IR readers, and calls public `generate_contract_scalar_oracles` for K=2/4/8 selected additions and R=4/8/16 unrelated function roots. The only scalar production change against current main is the previously reviewed replacement of two whole-source-map `find` calls by owner-retained `source_occurrences`, preserving first-match and occurrence order. New `cfg(test)` counters instrument those two keyed lookups and local occurrence predicates; no instrumentation enters release behavior. The test asserts generated claim identity, bounds, source map and oracle source, then enforces zero whole-map selection visits and exactly `2*K` indexed calls. The #542 scalar overlap did not change the role/ordinal, context-root, or result-context refusal sequence. The nine-cell before/after result is assessed in SR-16303.

`git diff --check 464f216c..e6257910` passed. No aggregate test or gate was run by this reviewer. **Code/Rust verdict: PASS at e6257910**, subject to the author's final-head gates. Bound strategy/negotiation product code and composite builder remain outside this PR diff; the composite test still constructs one retained context per K/R cell.
