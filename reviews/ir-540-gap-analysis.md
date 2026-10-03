---
id: SR-1005
title: "CG PR 241 gap analysis: FR-021-AC-22 against src/oracle/function/mod.rs"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@b4309b85673304ed2ba9b580847af46e8fce338e; FR-021-AC-22, spec/oracle/matrix/tests.md FR-021-AC-22 row, src/oracle/function/mod.rs, src/kani/generate/negotiate.rs, tests/it/layout.rs, schemas/, quire-contract-model checked_package/v2/lower.rs at 968ba9b (git diff origin/main...HEAD, base 85b8114)"
---
# SR-1005: CG PR 241 gap analysis

## Summary

Ticket: IR-540. Spec-only PR. AC-22 is 🚧 Planned, so I checked it against today's code to see
whether the Planned status and the tests.md claim are true. I did not assess plan completion.

These claims come from reading `src/oracle/function/mod.rs` at b4309b8:

- **Duplicate declaring node ids are not checked today.** Stage 1 refuses only a name that more
  than one declaration uses (`name_counts`, mod.rs:835-866). No code counts node ids.
- **Claims and symbols cross.** Several tables are keyed by node id:
  - `own_shape` (857) and `resolved` (894) are `BTreeMap<&CheckedNodeId, _>`. The later insert
    overwrites the earlier one, and the fixed-point loop skips the second declaration through
    `contains_key` (898).
  - `function_index` (992) is also keyed by node id, so both survivors get the last index.
  - `bodies` is positional. If both bodies pass Stage 1, both declarations enter `classified` and
    `checked_package()`.

  In `item_disposition`, `survivors.find(node_id == ...)` (1210) returns the first survivor for
  either name. So an item naming the second function records the first function's name and the
  shared, overwritten `Origin::Body` index. The renderer then picks the declaration by name
  (1090), so the claim and the emitted oracle disagree.
- **The second symptom.** One declaration is refused in Stage 1 and its sibling with the same node
  id survives. If the surviving sibling's `own_shape` entry is written last, the item naming the
  refused function resolves to the sibling, fails the name lookup at 1090, and is refused as
  `UnknownFunction`. This is SR-880 FND-004. If the refused one is written last, both items carry
  the refused function's reason instead.
- **No misalignment from lowering.** `CheckedPackageV2::lower` (quire-contract-model lower.rs:255)
  returns one record per requested id, duplicates included. The `zip` with `ordered_functions`
  therefore stays aligned, and declarations with distinct node ids are not shifted.
- **Variant placement.** No existing `ExactFunctionRefusal` variant fits:
  - `DuplicateRequest` is item-key level.
  - `AmbiguousFunctionName` is name based.
  - `InvalidInput` means a node absent from the graph.

  The refusal is per function, and siblings with distinct ids must still generate (AC-1 and AC-12).
  `OracleGenerationError` is whole-call and fails only for size and serialization, so it is the
  wrong place.
- **Effect of adding the variant.** The enum is public and re-exported at the crate root
  (src/lib.rs:71-73). It is not `#[non_exhaustive]`, so adding a variant is a breaking change for
  downstream exhaustive matches. That is acceptable for prerelease. In this repo:
  - No exhaustive match over `ExactFunctionRefusal` exists. `negotiate.rs:600` matches
    `ExactScalarRefusal`.
  - No layout or rustdoc test pins the variant names.
  - No schema lists its serde `code` values.

  So the spec needs no API or layout note. AD-004's pure-motion rules do not apply.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The tests.md FR-021-AC-13 row reads ✅ Covered, but AC-13's permutation-determinism property does not hold today for duplicate node ids. `sort_by` is stable, so declarations with equal ids keep the caller's order. Which declaration wins `own_shape`, and which one `survivors.find` returns, therefore depends on the order of the request. AC-22's refusal-of-all fixes this as a side effect. TC-031 step 9 should also permute the two same-node declarations, so a fix that refuses only the later-sorted copy is caught regardless of input order | src/oracle/function/mod.rs:825 |
| FND-002 | low | AC-22 says nothing about a declared function whose own nested `call` body names a duplicate-node function. AC-12 covers only a callee "absent from the request", and a duplicate-node callee is present. Today such a caller would get `UnknownCallee` through `resolved` (mod.rs:909-913). State whether the caller gets `UnknownCallee` or `DuplicateDeclaringNode`, or say that AC-12 governs it | spec/oracle/functional/FR-021-function-application-oracles.md:248 |

## Verdict

The gap is real and the spec describes it accurately. The ticket's two symptoms are confirmed by
tracing the code. The 🚧 Planned status and the tests.md wording ("only duplicate names are
[checked], so claims and oracle symbols cross") are true. A new variant on `ExactFunctionRefusal`
is the right place, and it touches no exhaustive match, pinned name set or schema. Two lows remain.
Note for the coder: the comments at mod.rs:77-82 and mod.rs:854-856 claim that keying by node id
prevents collapse and body swaps. Those comments must change with the fix.

## Dispositions

Reviewed at agent-ix/quire-contract-codegen@08a7f0366888bad18e710b6b9287b543fff0db61 (fix-round delta b4309b8..08a7f03).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 08a7f03 |
| FND-002 | fixed | 08a7f03 |
