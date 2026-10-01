---
id: "SR-770"
title: "CG PR 226 code review (with Rust review): tuple text position names its text_bounds node; lock to IR main"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@ea3a8985efa9e2381cadbde6075486ed3a76b951; src/composite_equality.rs, tests/composite_equality_support/package.rs, tests/it/composite_equality_generation.rs, Cargo.lock, PR body"
---

# SR-770: CG PR 226 code review (with Rust review)

## Summary

Ticket: IR-498. PR: agent-ix/quire-contract-codegen#226 at ea3a898, merge base 224ca6e
(origin/main is now ea07b79, three commits ahead; `git merge-tree` merges clean). This file
holds the code-review method with the rust-review lane folded in.

What I measured on a detached worktree at the reviewed sha:

- **Lock delta.** `git diff origin/main...HEAD -- Cargo.lock` moves only `quire-contract-ir`
  and `quire-contract-model`, from 0a889f9 to 968ba9b8. No qsl-* or quire-canonical entry
  moved. `make deny` (cargo-deny plus the one-copy awk check) exits 0. IR main has since
  moved to 4233b56 (IR-448). That does not matter here: 968ba9b8 is on main.
- **Corpus hunk.** tests/composite_equality_support/package.rs changes by 5 lines added and 2
  removed: the stale `0a889f9` doc mention, plus the TUP_PAIR position-1 reference
  (`T_TEXT` to `BD_TEXT`) with a 3-line comment. Line endings are LF, the trailing newline is
  kept and `git diff --check` is clean. There is no collateral change.
- **Generator arm.** `resolve_type` gains a `BoundedDomain` arm. It looks up the node's
  `semantic_type`, refuses a non-`scalar_type` base as `Unsupported{node_tag:
  "bounded_domain"}`, and otherwise calls `resolve_scalar` with only that node as its bounds.
  `resolve_scalar` now takes a bounds slice. The `ScalarType` arm passes the same per-type
  list as before, so its behaviour is unchanged. The new arm cannot recurse, because the base
  must be a scalar. `core::slice::from_ref(&node)` is idiomatic. It adds no panic, unsafe,
  index or cast.
- **Gates at head (CARGO_TARGET_DIR in the worktree).** fmt-check, spec, deny, audit-unsafe,
  lint (clippy -D warnings) and rustdoc (-Dwarnings) all exit 0. The test run covered the
  composite_equality and exact_function filters of the `it` binary: exit 0, 41 passed, 0
  failed, including `tc_029_a_cyclic_compared_type_is_refused_by_ir_today`. The rustdoc run shows no
  new unresolved link. The coder's kani log reads `head=ea3a898 exit=0`, 9 passed.
- **QSL emission.** qsl-semantics `check/lowering.rs` (`descend`, `ValueType::Text`) emits a
  text type as a `text_bounds` `bounded_domain` node over the `text` scalar, binding `min`,
  `max` and `text_profile`. That node is the type a member references. BD_TEXT has exactly
  this shape, so the TUP_PAIR change matches QSL for the text position.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | The new `bounded_domain` arm never checks that the bound's form fits its base scalar. Over an `integer` base, a bound of any form other than `integer_range` is dropped silently and the member reads as unbounded `Integer`. Over a `boolean`, `float32` or `float64` base the bound is ignored. Over an `enum` base, `resolve_scalar` keys `ValueType::Enum` on `node_key(type_id)`, where `type_id` is now the bound node and not the enum declaration. FR-018's new clause says such a member "is read from its own `binding` members", but in these cases nothing is read and nothing is refused. | src/composite_equality.rs:1023-1032; src/composite_equality.rs:1074-1080; src/composite_equality.rs:1129 |
| FND-002 | low | Process: the coder reports making the TUP_PAIR one-line change with a python string replace instead of the Edit tool. The resulting bytes are clean (see Summary), so the code is unaffected. The rule breach still stands. | tests/composite_equality_support/package.rs:1044-1047 |
| FND-003 | low | The PR body says qsl-replay "has no package emitter". That is inaccurate. `pub mod spine` exports `spine::compile`, which returns `Compiled { emitted: EmittedPackage, .. }`, the emitted checked-package/v2. The conclusion (no facade route) still holds for other reasons: qsl-replay's lib docs say CG may not name `spine` (ADR-011 FB-05, T-12 rule (a)), `compile` reads no lock, and `qsl-package` emit always writes `profile_selections: []`. The stated reason should be corrected. | PR body "Facade route" |

## Verdict

The fix is minimal and correct for the failing case. The corpus now references the text type
the way QSL emits it, the lock delta is the two IR crates only, and the gates are green.
FND-001 is a real edge, but no current producer emits it: QSL emits no bounded enum or
boolean, and no mismatched form. A reader would still trip on it, so it should refuse rather
than read as unbounded. FND-002 and FND-003 are low. Mergeable after FND-001 is fixed, or
after it is explicitly deferred with a ticket.

## New findings (disposition pass 1)

Reviewed at 9e564e7. The branch was rebased onto origin/main ea07b79. Its first commit,
3c51ecd, is range-diff-identical to ea3a898.

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-004 | medium | The fix refuses a `bounded_domain` over a `float32`/`float64` base as `Unsupported{bounded_domain}`. QSL emits every float type this way, as a `float_rounding` node over the float scalar (lowering.rs `ValueType::Float`). Measured with a scratch probe: a TUP_PAIR whose position 0 names a `float_rounding` node over `T_FLOAT64` is admitted by Contract IR 968ba9b8, and CG then refuses it as `Unsupported{bounded_domain}`. FR-018-AC-6 requires an operand type bearing an IEEE value at any depth to be refused as `OperatorIneligible`, and the corpus's own `E_NESTED_IEEE` shows that refusal is CG's `check_equality`, not IR admission. So a QSL-shaped float member gets the wrong refusal, and the reason given (the refusal is unreachable through admission) is false. Read `float_rounding` over `float32`/`float64` as `Float(width)`, so the IEEE refusal comes from `check_equality`, and test it. | src/composite_equality.rs:1032-1040; spec/oracle/functional/FR-018-composite-equality-oracles.md:145-151 |
| FND-005 | low | The `BoundedDomain` arm's comment still cites "FR-018 §Inputs". That rule now lives in the FR-018 Behavior clauses and FR-018-AC-16. | src/composite_equality.rs:1020-1022 |
| FND-006 | low | Process: the coder discloses a second breach in the fix round. A temporary probe test was appended by shell and removed with python, instead of the Edit tool. Verified: `git diff 3c51ecd 9e564e7` holds only the AC-16 tests, the codes, `set_body` and `tuple_members_package`. No probe residue, no CR, every touched file ends in a newline, `git diff --check` is clean. | tests/it/composite_equality_generation.rs:1090-1209 |

## Dispositions

| FND | outcome | sha/reason |
|-----|---------|------------|
| FND-001 | fixed | 9e564e7: the form check refuses a mismatched form as `MissingBound`, and boolean/enum/non-scalar bases as `Unsupported`. Mutation-tested: removing the check fails the wrong-form and base-refusal tests. The float choice in that same check is FND-004. |
| FND-002 | accepted-no-change | A process breach cannot be undone. It is disclosed in the PR body, and the bytes were verified clean. |
| FND-003 | fixed | 9e564e7 (PR body edited at this head): the facade section now names `spine::compile` and gives the real reasons (FB-05, no lock, empty `profile_selections`). |
| FND-004 | fixed | 82b669f: `"float32" \| "float64" => "float_rounding"` in the form table. A `float_rounding` member reads as `Float(width)`, and `check_equality` refuses it as `OperatorIneligible`. My own probe at head (float_rounding over T_FLOAT64 at TUP_PAIR position 0) refuses as `IllTyped{OperatorIneligible}`. Mutation: removing the arm fails `tc_029_ac16_a_float_rounding_member_is_refused_as_operator_ineligible`. |
| FND-005 | fixed | 82b669f: the comment now cites "FR-018 Behavior, FR-018-AC-16" and the float reading. |
| FND-006 | accepted-no-change | A process breach cannot be undone. It is disclosed in the PR body, and no residue was verified in round 1. This round's diff (9e564e7..82b669f) is clean: no CR, trailing newlines kept, `git diff --check` clean. |
