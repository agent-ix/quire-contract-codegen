---
id: "SR-1351"
title: "IR-545 code review: one UnknownFunction claim per unknown function name on a call node"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@7dd5a391ddc801a6f9fa1c1e5376f4575ccbad48; src/oracle/function/mod.rs, tests/it/exact_function_generation.rs"
relationships: []
---

# SR-1351: IR-545 code review (PR #253)

## Summary

Ticket: IR-545. PR: agent-ix/quire-contract-codegen#253, head 7dd5a39, diffed against
origin/main 3033993 (the merge base). The rust-review lane is folded into this file.

The PR renames the last field of the private `ItemKey` from `refused_name` to `name` and sets it
in one more case. Before, it was set only when a duplicate-node declaration held the item's name
(FR-021-AC-22). Now it is also set when the name is absent from the declarations
(`function_node_id` is `None`). It adds six tests, `tc_031_ac24_case_i` to `case_vi`.

## Method

- Read the diff, `ItemKey::of`, `FunctionNames`, and the Stage 3 loop that builds `counts` and
  `by_key`.
- Checked that the new key matches the old one wherever the old one was used. The new `name` is
  `Some` exactly when the old `refused_name` was `Some`, or when `function_node_id` is `None`.
  `function_node_id_by_name` is built from every declaration, so the only items with a `None` id
  are unknown names. A uniquely declared name, and an ambiguous name (which resolves to the
  highest-sorted node id), still get `name: None`, so their keys do not change. Before this
  change, two different keys could never tie: the only collisions were distinct unknown names on
  one call node with equal arguments, and these collapsed into `DuplicateRequest`. Those are the
  only items whose output changes.
- Ran a byte comparison of emitted artifacts. A throwaway patch to `Artifact::new` dumped
  every artifact the non-ignored suite builds. At base 3033993, the suite built 7862 artifacts
  (5434 distinct). At the head with the six new tests skipped, it built the same 7862 (same
  multiset), and every distinct artifact's bytes were identical. The full head suite adds 424
  artifacts (8 distinct), all from the new tests. None of the 15 ignored Kani tests reaches the
  function oracle generator: only `src/oracle/function/mod.rs`, `src/lib.rs`,
  `tests/exact_function_support/package.rs`, `exact_function_generation.rs` and
  `exact_function_agreement.rs` mention it.
- Wrote my own probe for every TC-031 step 12 case (results in the gap-analysis review,
  SR-1352).
- Ran mutants in a throwaway copy, testing `exact_function`:
  1. Unknown names not keyed by name (the base behaviour): killed by case_ii and case_v.
  2. Key without the name at all: killed by case_ii, case_v, case_vi and ac22_fixture_vi.
  3. Name ahead of the declaring node id: killed by case_iv.
  4. Case-insensitive compare, keyed by `(lowercase, original)`: killed by case_v.
  5. Key by request position instead of the name: killed by case_ii, case_iii, case_v,
     case_vi and ac22_fixture_vi.
  6. Tie-break by the name's first request position, then the name: killed by case_ii, case_v
     and case_vi.

  Each test's expected values are hand-written literals (names, and node ids from `code_id`).
  None are derived from the implementation.
- Gates at the head, each run myself: `make fmt-check`, `make lint`, `make test` (130, 290 with 15
  ignored, and 1, all passing), `make deny`, `make audit-unsafe`, `make rustdoc`, `make msrv`,
  `cargo test --test it no_generation_panics` (4 passed). The PR adds no panic token, no
  `unwrap` and no `expect` to `src`. `ItemKey` is private, and no public type or signature
  changes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The doc comment on the new `entries` test helper says it returns entries "as `(call node, disposition)`", but it returns only the dispositions. | tests/it/exact_function_generation.rs:988-998 |

## Verdict

The change is correct for every case it targets. It is minimal, and it leaves every existing
fixture byte-identical. The keying is equivalent to the old `refused_name` behaviour for every
AC-22 case, and it does not change any known or ambiguous name. The same unknown name requested
twice still shares one key, so it still gives one `DuplicateRequest`. All six mutants are killed.
All gates pass. The one finding here is a doc nit in a test helper. One spec-ordering edge
case, where the order differs from the FR-021 Behavior bullet, is recorded as a medium finding in
SR-1352.

## Dispositions

Round 1, checked at 0e8dd504eec6295265a22de1d822b74854a75efe. `src/` is unchanged since 7dd5a39.
At this head, `make fmt-check` and `make lint` pass, the `exact_function` tests pass (36), and
`no_generation_panics` passes (4).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0e8dd504eec6295265a22de1d822b74854a75efe |
