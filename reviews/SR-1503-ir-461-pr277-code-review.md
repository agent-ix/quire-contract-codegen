---
id: "SR-1503"
title: "CG PR 277 code review (with rust-review lane): the state and frame refusal-to-record mapping, the NotLowered and BoundNotResolved payloads and the MalformedClause reclassification"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@37a7e1e21162ed1af6031fa03dd0464ad4da62e7; src/kani/generate/outcome.rs, src/kani/generate/frame.rs, src/lib.rs, tests/it/kani_obligations_state_frame.rs (diff origin/main...HEAD, merge base e526390)"
---

# SR-1503: CG PR 277 code review

## Summary

Ticket: IR-461 (code PR 1 of 2). PR: agent-ix/quire-contract-codegen#277 at 37a7e1e, diffed
against origin/main e526390 (the merge base). This review covers `code-review` and its
`rust-review` lane. It also covers test-oracle strength, which I checked by hand mutation. Gap
analysis is SR-1504. The spec-review base checklist for the spec edits is SR-1505.

What I measured myself:

- `make ci` on the head with my own target dir: exit 0. fmt-check, `quire validate`, clippy
  `-D warnings`, msrv, deny (advisories, bans, licenses, sources ok), the unsafe audit, rustdoc
  and the tests all passed: lib 163, `it` 348 passed with 23 ignored, doc 1. Both new tests ran
  and passed. The NFR-005 panic-token scan passed. The only `quire validate` warnings are
  pre-existing EARS warnings at FR-017:152, a file this PR does not touch.
- The mapping `state_frame_disposition` (outcome.rs:749) checked row by row against the FR-015
  table (FR-015:323-339) and against the real enums. It has 17 `StateFrameRefusal` variants, 6
  `StateFrameLoweringRefusal` arms, 5 `BoundNotResolvedCause` grounds and 4
  `UnsupportedFrameEffect` values. Every row matches the table.
  `NotLowered(Unsupported)` becomes `NoFiniteEncoding { unsupported_node_id, family }` and
  `NotLowered(InvalidInput)` becomes `UnknownNode`, the same as negotiate.rs:603-618 does for
  `ExactScalarRefusal`. `FrameEffectUnsupported` and the condition shapes become
  `StateFrameRefused` carrying the refusal, never `NoFiniteEncoding`, as the table says
  (citing planned FR-015-AC-40).
- `lowering_refusal` (frame.rs) is an exhaustive `match` over `CompleteLoweringRecordV2` with
  no wildcard arm. The `Lowered` arm maps to `None`, so the payload type cannot hold a lowered
  node.
- Hand mutants. Each was applied in my own detached worktree, run against the AC-66 unit test,
  the `kani_obligations_state_frame::tc_025_*` tests and (for the lint mutants) clippy, and then
  reverted. Killed: MalformedClause to `invalid(UnknownNode)`; InvalidField to
  `InvalidStatePath`; dropping the `NotLowered(InvalidInput)` row into `refused`;
  `NoFiniteEncoding` naming `node_id` and not `unsupported_node_id`; a fixed family tag; the
  condition-read identifier check reverted to `InvalidField` (killed by the AC-67 test); a
  binding catch-all `other => refused(other)` with the `deny` kept (killed by clippy). Dropping
  a `NotLowered` arm fails to compile. Survived: see FND-001 and FND-002 below, and SR-1504
  FND-001.
- Public API: `StateFrameRefusal` and `UnsupportedFrameEffect` moved from `frame.rs` to
  `outcome.rs` and are still re-exported from the crate root under the same names.
  `BoundNotResolvedCause` and `StateFrameLoweringRefusal` are new re-exports. An org-wide
  `gh search code` for `StateFrameRefusal`, `UnsupportedFrameEffect` and
  `generate_state_frame_obligations` finds hits only in this repository. There is no
  downstream consumer and no compatibility shim, which is correct.
- Layout: `frame.rs` imports the refusal types from `outcome.rs`, and `outcome.rs` imports
  nothing from a family. This is consistent with AD-004 D-3 rule 1 (what the entry returns is
  the leaf, `outcome.rs`). It is also required, because `UnsupportedObligation` now holds a
  `StateFrameRefusal`. The L-2 acyclicity test passed.
- PR 2 content: grep for `state_frame_harnesses`, `generate_state_frame_role`, `StateFrameRole`
  and `ObligationItem::StateFrame` in `src/` finds nothing. `MixedStatePackages` is declared and
  has no producer yet. FR-015:360-362 assigns that reason to the first change, so this is not
  early PR 2 work.
- Harness text: `render`, `postcondition_body` and `frame_body` are unchanged, and
  `state_domains` returns the same domains for an admitted `bounded_domain`/`integer_range`
  bound. The generation tests in `make ci` passed. I found no reason to run `make kani`.

## Verdict

CONDITIONAL. The mapping is correct and total. The per-row oracle kills every disposition and
reason mutant I tried. The `Box<StateFrameLoweringRefusal>` keeps the refusal small. The new
non-test code holds no unwrap, expect, indexing or panic. The serialised form is snake_case with
no IR record, and there is no `Deserialize`, so there is no round-trip to check.

On the author's ambiguities:

- (3) The move to `outcome.rs` is required, as stated above.
- (4) The `cfg_attr(not(test), allow(dead_code, reason = ...))` is a correctly scoped, reasoned
  allowance that PR 2 must remove.
- (6) `NotAStateClause` gaining `node` is required, because table row 329 names the node.

Two medium findings and one low finding remain:

- The AC-66 inspection does not guard what it claims to guard (FND-001).
- The engine's ground classification is not asserted by any test in this PR (FND-002).
- The serialised `failed` arm carries unattributable numbers (FND-003).

The frame-grant reclassification that survives mutation is recorded in SR-1504 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AC-66 "no wildcard arm" inspection only checks that the text `_ =>` is absent. It does not check that the `#[deny(clippy::wildcard_enum_match_arm)]` attribute is present. Mutant M7b, a binding catch-all `other => refused(other)` that replaces the twelve explicit `refused` patterns with the `deny` removed, passes the AC-66 test, the AC-67 tests and `make lint`. Mutant M7c, which only drops the `deny`, also survives. The lint is the real gate, and nothing guards it. Assert the `deny` attribute in the inspected preamble, or parse the function with `syn` and refuse any arm whose pattern is `_` or a bare binding | src/kani/generate/outcome.rs:745-749, src/kani/generate/outcome.rs:1146-1154 |
| FND-002 | medium | No test asserts which ground the engine's new `field_range` classifies. Only `UnboundedType` is reached, by the integration test with `balance_bound: None`. Mutants M8 (swap `MemberAbsent` and `ValueNotReference`) and M9 (`EndpointOutsideI64` reported as `NotIntegerRange`) pass every test. All four grounds map to `StateFrameRefused`, but the record carries the cause, so a wrong cause ships in the serialised record. Planned FR-015-AC-63 (PR 2) names these four cases. It must assert the exact `cause`, not just the `state_frame_refused` code, or these mutants survive there too. Alternatively, add engine-level cases in this PR | src/kani/generate/frame.rs:666, src/kani/generate/frame.rs:671, src/kani/generate/frame.rs:687 |
| FND-003 | low | `StateFrameLoweringRefusal::Failed` serialises `limit` and `consumed` but skips `limit_kind`. The record carries two numbers and does not say whether they count work units or bytes. FR-015:345-347 says the reason carries "the node, field and effect it names (never the IR record)". `limit` and `consumed` are neither. Either serialise `limit_kind` as its wire name or skip all three, and pin the choice in the serialisation assertion | src/kani/generate/outcome.rs:493-503, src/kani/generate/outcome.rs:1125-1144 |

## New findings (disposition pass 1)

Disposition pass 1 reviewed head dbff533 (one fix commit on 37a7e1e; origin/main is still
e526390). `make ci` passed on dbff533 with my own target dir: lib 164, `it` 349 passed with 23
ignored, doc 1, and clippy, msrv, deny, the unsafe audit, rustdoc and `quire validate` all
passed. `quire coverage` is 303/442 with no status lies and 66 unbacked rows, unchanged from
37a7e1e. The diff from 37a7e1e to dbff533 touches only tests.md, outcome.rs and the state-frame
integration test.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The new ground test builds its `ValueNotReference` case as fixture variant 20. Variant 20 is already used by the ignored real-Kani test `tc_025_real_kani_a_violation_at_the_only_valuation_is_a_counterexample_not_the_covers_playback`, with bounds (0, 0). The fixture's own rule (kani_obligations_state_frame.rs:64-65) is that each variant owns its node codes, and the process-global registry asserts on a code bound to two bodies. I built both variant-20 fixtures in one test and it panicked: "code 70410 is already registered as 57959b0a..., cannot also register it as 9a8d2582... -- two distinct fixture nodes share one code". The default lane skips the ignored test, and `make kani` runs `--ignored` only, so neither current lane fails. Any `--include-ignored` run fails, and the registry mutex can cascade the failure to other tests. Use an unused variant, for example 21 | tests/it/kani_obligations_state_frame.rs:925, tests/it/kani_obligations_state_frame.rs:1190 |

## Dispositions

Round 1, reviewed at dbff533d8a3e7ce1f27608325af7f0f922b7899b.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | dbff533 |
| FND-002 | fixed | dbff533 |
| FND-003 | fixed | dbff533 |

Notes for round 1:

- FND-001. The new test `tc_025_the_state_frame_mapping_has_no_wildcard_or_binding_arm` parses
  outcome.rs with `syn`. It requires `deny(... wildcard_enum_match_arm ...)` on the function, and
  it refuses a `_` or bare-binding arm at any depth, with a floor of at least 14 arms. M7b,
  M7c and a new nested-wildcard mutant M13 are now killed by the lib tests.
- FND-002. The new test `tc_025_the_engine_names_the_ground_a_field_has_no_integer_range` asserts
  the exact cause for `NotIntegerRange`, `EndpointOutsideI64` and `ValueNotReference`, and
  `UnboundedType` was already asserted. M8 and M9 are now killed. I checked the claim that
  `MemberAbsent` is unreachable with a probe: I temporarily fixed the object's member to
  `balance` and had the clause read `ghost`. IR refused admission ("expected V2 admission"). So
  the mapping-table row is the only reachable check for that cause, and that is enough here.
- FND-003. `limit_kind` now serialises by snake_case name through an exhaustive match, and the
  serialisation assertion checks `"limit_kind": "work"`. M11 (wrong name) and M12 (skip
  restored) are killed.

Round 2, reviewed at a06d2c20c6255f13a6db6506bbdcd03df3386c84.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | a06d2c2 |

Notes for round 2:

- The diff from dbff533 to a06d2c2 touches only tests/it/kani_obligations_state_frame.rs.
- The ground test's `ValueNotReference` case now uses variant 21, which no other test uses. The
  ignored real-Kani test's shape moved into `single_valuation_shape()`, which owns variant 20.
- The new default-lane guard
  `tc_025_the_kani_lanes_fixture_variant_is_not_shared_with_a_default_lane_variant` builds that
  fixture in the default process.
- I ran a probe that builds every fixture the ignored tests build (variants 20, 1 and 0)
  alongside variant 21, together with the 17 default state-frame tests in one process. It passed
  with no registry panic. I did not run the real-Kani lane.
- The reverse mutant (ground case back to variant 20) is now killed in the default lane, with
  "code 70410 is already registered".
- `make ci` on a06d2c2 exited 0: lib 164, `it` 350 passed with 23 ignored, doc 1.
  `quire coverage` is 303/442, unchanged.
