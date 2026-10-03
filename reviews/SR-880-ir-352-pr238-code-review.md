---
id: "SR-880"
title: "CG PR 238 code review (with rust-review lane): generated function oracles never panic on an unknown runtime variant"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@5cb264db4df4ba16290d19236369e530d5debb62; src/oracle/function/mod.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs (diff origin/main...HEAD, base 80f4785, current)"
---

# SR-880: CG PR 238 code review

## Summary

Ticket: IR-352. PR: agent-ix/quire-contract-codegen#238 at 5cb264d, one commit on base 80f4785,
which is the current `origin/main`. The `rust-review` lane is folded into this file.

What I checked:

- Emitted templates. Both body templates in `render_body` (scalar and equality) now end their
  `match` over `Result<Outcome<_>, Refusal>` with `Ok(_) => rt::Outcome::Refused(rt::Refusal::CheckedInvariant)`.
  The five arms before it are unchanged: `Ok(Completed)` rewrapped as `Value::Integer` or
  `Value::Boolean`, `Ok(Undefined)`, `Ok(Refused)`, `Ok(Incomplete)`, then
  `Err(refusal) => Outcome::Refused(refusal)`. The emitted `checked_package()` builds its empty
  environment with `rt::TypeEnvironment::default()`.
- `Default` at the locked runtime. `Cargo.lock` (unchanged by this PR) pins
  quire-contract-runtime at ccc722bd56c64b264070a30a14020efd0bf4a0f4. There `TypeEnvironment`
  derives `Default` (`src/exact/composite.rs:938`). Its fields are two `BTreeMap`s.
  `TypeEnvironment::new` starts from `Self::default()`, so with two empty inputs it returns
  `Ok` of that same default value. `default()` and the old `new(empty, empty).expect(..)` give
  the same environment.
- Emitted corpus. I dumped the emitted `src/lib.rs` of `main_oracles()` and `chain_oracles()`
  using a temporary test that I reverted afterwards. Grepping them found no `unwrap`, `expect`,
  `unreachable`, `panic`, `todo`, `unimplemented` or indexing. The main corpus holds a call body
  (`call_fn`), two scalar bodies (`add_fn` and `unrelated_fn`), an equality body (`eq_fn`), and
  five item functions that only `.map` the `CheckedPackage::call` result. Inside the equality
  body, `TypeEnvironment::new(..)` is matched, and its `Err(_)` returns
  `Refused(CheckedInvariant)`, with no `expect`.
- Byte identity. I dumped the same corpora with `src/oracle/function/mod.rs` at base 80f4785 and
  diffed them against head. Only the lines this PR intends to change differ: in each corpus the
  `checked_package()` environment (2 lines become 1), plus 3 `Ok(_)` arms in main and 1 in chain.
  `Cargo.toml` is identical.
- Negate. `BinaryIntegerOperator::of` is the single predicate that both `classify_body_shape`
  (stage 1 refusal) and `ClassifiedBody::of` (stage 2) use. In stage 1, a scalar body reaches
  `resolved = Ok` only through `own_shape = Ok`, and `own_shape` runs `classify_body_shape`. A
  `Call` body resolves through its callee, but its own body is `Call`, and `ClassifiedBody::of`
  always returns `Some` for that. So the stage 2 `let Some(body) = .. else { continue }` cannot
  fire for a Negate body, or for any other body. `survivors` now comes from `classified`, in the
  same order and with the same members as before. `item_disposition` looks up survivors with
  `find`, which cannot panic, and `function_index[..]` is keyed from that same `classified`
  set. So the restructure adds no new panic path. `src/oracle/function/mod.rs` now holds zero
  panicking-macro invocations. The generator-side `.unwrap()`/`.expect(..)` calls (lines 903,
  961, 963, 1002, 1096) are untouched. The spec leaves them to the separate ticket, so they are
  not findings here.
- Rust idiom. `BinaryIntegerOperator` and `ClassifiedBody` are private, documented, and carry
  no derives beyond `Clone, Copy` on the operator enum. `render_body` now takes the classified
  body, so the renderer cannot represent `Negate`. That is a sound "make the bad state
  unrepresentable" step and smaller than making `render_body` return `Result`. No `unsafe`,
  async, integer-conversion or public-API surface changed.
- Mutation probes. I ran eight probes on my own worktree and reverted each one. Every probe was
  killed:
  - `unreachable!` catch-all put back in the scalar template, and separately in the equality
    template: the AC-19, AC-20 and generator-guard tests failed.
  - `.expect(` put back in `checked_package()`: the AC-19 test failed.
  - A defensive `Negate => unreachable!` arm added: the guard test failed.
  - The `UnsupportedOperator` refusal dropped: the AC-21 test failed.
  - The scalar catch-all dropped: the AC-20 test failed.
  - The catch-all moved before `Err(refusal)`: the AC-20 test failed.
  - The catch-all given a different value: the AC-20 test failed.
- Tests not weakened. The guard test's `EXCUSED` list goes from 4 to 2. Only the three
  `function/mod.rs` lines are dropped, because those lines no longer exist. The per-entry
  exact-count check and the offending-line check are unchanged. The renamed Negate test keeps
  its `UnsupportedOperator` assertion and adds two absence assertions.

## Verdict

The change is correct and the smallest honest one. The emitted function-oracle source is free
of panicking paths, and the generator file holds no panicking macro. Generated bytes change only
where the spec says they should. All the new tests can fail, and they did fail under mutation.
Two low findings, neither blocking.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Stage 2 classifies a scalar operator a second time (`ClassifiedBody::of`) after stage 1 already did (`classify_body_shape`), and covers the gap with a dead `let .. else { continue }` that would silently drop a function if the two ever diverged. Returning the `ClassifiedBody` from stage 1's `own_shape` would remove the branch and the second classification | src/oracle/function/mod.rs:951-955, src/oracle/function/mod.rs:716-720 |
| FND-002 | low | `invokes_macro` in `exact_function_generation.rs` is a second copy of `invokes_panicking_macro` in `exact_scalar_generation.rs`: the same name/`!`/identifier-boundary logic. If one copy's matching rule is fixed, the other drifts | tests/it/exact_function_generation.rs:576-584, tests/it/exact_scalar_generation.rs:2145-2156 |

## New findings (disposition pass 1)

Round 1, reviewed at e44365d0d552844ca2aeb5bca9ab43e705638b3c: the fix commit e44365d on top of
5cb264d, base still 80f4785 (current `origin/main`).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | Stage 2 now takes each body with `own_shape.remove(&declaration.node_id)`, but `own_shape` is keyed by node id and nothing rejects two declarations that share one. With two such declarations, the second `insert` overwrites the first. The first declaration then removes and renders the second one's body under its own name. The second gets `None` and is silently dropped. An item naming the dropped function still passes `item_disposition`, which finds a survivor by node id, and then the caller's lookup by name hits `.expect("item_disposition only returns Ok for a surviving function")` and panics. Reproduced through the public `generate_exact_function_oracles` with `function_add("add_first")` plus `function_eq("eq_same_node")` moved onto node `FN_ADD`. Head e44365d panics at mod.rs:1085. Base 80f4785 and 5cb264d return `Ok` on the same input and render each function with its own body. This is a new reachable generator panic, introduced by the fix round | src/oracle/function/mod.rs:937-944, src/oracle/function/mod.rs:1082-1085, src/oracle/function/mod.rs:854-883 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e44365d |
| FND-002 | fixed | e44365d |
| FND-003 | fixed | b94fe6b |
| FND-004 | deferred | IR-540 (refuse duplicate declaring node ids in generate_exact_function_oracles, spec first). The misleading UnknownFunction reason arises only from duplicate node ids, and the crossed claims and oracle symbols for surviving duplicates are byte-identical at base 80f4785. #238 did not introduce the duplicate-node-id handling; it only removed the panic |

## New findings (disposition pass 2)

Round 2, reviewed at b94fe6bf82c977ae40473d7306311945a0c7ad42: the fix commit b94fe6b on top of
e44365d, base still 80f4785 (current `origin/main`).

Round 2 checks:

- My round-1 reproduction (`add_first` plus `eq_same_node` on node `FN_ADD`) now returns `Ok`.
  Its `src/lib.rs`, claim map and location map are byte-identical to base 80f4785, apart from the
  stated `checked_package()` and `Ok(_)` lines.
- The main and chain corpora are byte-identical to base, apart from the same stated lines.
- The new test fails at e44365d's `mod.rs` (it panics at line 1085). It also fails on a body-swap
  mutant where each declaration takes the last body on its node id.
- I traced every remaining generator-side panic site. None is reachable from duplicate node ids
  or from anything the restructure newly allows:
  - The `name_counts[..]` and `unique_name_node_id[..]` lookups are always keyed by present keys.
  - `own_shape.get(..).unwrap()` is unchanged from base.
  - The two stage-2 `resolve_operand_type` `.expect`s now run only on a declaration whose own
    stage-1 result was `Ok`, and that result already resolved the same types. This also closes a
    base path where a refused duplicate could reach them.
  - The `TypeEnvironment::new(empty)` `.expect` cannot fail.
  - Both `function_index[..]` lookups are keyed from survivors.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new `None` arm in the item loop (`ClaimDisposition::Refused { UnknownFunction }`) is reachable when two declarations share a node id, the named one is refused in stage 1, and its sibling survives. Reproduced with `neg_first` (`Negate` on node `FN_ADD`) plus `eq_same_node` on the same node: the `neg_first` item is refused as `UnknownFunction { name: "neg_first" }`, not with its own `UnsupportedOperator`. `item_disposition`'s comment reserves `UnknownFunction` for "a name absent from the request's own declarations entirely", so the reason is misleading. The arm is unreachable for requests with unique node ids. Base 80f4785 panicked on this input (in `render_body`'s `Negate => unreachable!`), so the new behavior is strictly better than base | src/oracle/function/mod.rs:1089-1103, src/oracle/function/mod.rs:1186-1192 |
