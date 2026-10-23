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
