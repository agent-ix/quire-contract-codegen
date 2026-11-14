---
id: SR-17940486709564847047982
title: "IR-262 PR #833 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #833 at 36cb1232, git diff origin/main...HEAD; IR-262; src/kani/generate/enum_scalar.rs, src/routed/generate.rs, src/replay/scalar.rs, src/oracle/equality/admitted.rs, src/oracle/equality/mod.rs, src/kani/identity.rs, src/kani/generate/outcome.rs, src/lib.rs, tests/it/enum_scalar_harness.rs, tests/it/kani_gate.rs, Makefile, Cargo.lock, docs/qualification/ir262-enum-scalar-kani-measurements.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-262. This is a code review of PR #833 at 36cb1232, with the rust-review lane folded in.
I read the whole diff. Every cargo run used a scratch copy of the head, my own target dir and
`flock /tmp/cargo-slot.lock`, one job at a time.

**Focused tests:** `--lib enum_scalar` gave 9 passed; `--test it enum_scalar_harness` gave 14
passed and 20 ignored.

**Installed Kani (0.68.0, CBMC 6.11.0), run through the routed `generate_routed_admitted` path:**

- **Verified:** `kani_lt_verifies` (3.3 s) and `kani_unordered_ne_verifies` (3.1 s).
- **Falsified:** `kani_lt_rendered_as_le_is_falsified`, `kani_reversed_ordering_is_falsified_for_ge`,
  `kani_an_incomplete_comparison_is_not_verified` and
  `kani_a_falsifying_counterexample_decodes_and_replays_against_the_declared_operator`.

The argv was `-Z function-contracts --harness <module>::<proof> --exact --unwind 1 --solver cadical
--output-format regular -Z unstable-options --export-json ...`. The verdicts, options and wall
times match `docs/qualification/ir262-enum-scalar-kani-measurements.md` and the harness text
shipped at the head.

**Mint-seam mutation check:** I deleted the `if shape != owner { return Err(ShapeNotOwnerAdmitted) }`
guard in `admit_carriers`. **KILLED** by `a_generator_mint_in_both_shape_and_members_is_not_the_owner_admitted_shape`
and `a_generator_that_mints_a_key_in_shape_and_members_is_refused_end_to_end`.

## Verdict

**FAIL: one high finding and one medium.**

What is sound:

- **Routed wiring:**
  - `settle_enum_items` rewrites only `OperationNotRendered` records whose identity starts with
    `quire.op.enum.`, so `generate_routed` and `generate_routed_native` pass `None` and are
    unchanged.
  - The integer sibling's output is compared byte for byte between the legacy and admitted calls.
  - Harness names are `kob_scalar_enum_*`, plus an ordinal, so they cannot collide with integer
    stems, and `index_harnesses` still refuses duplicates.
- **Owner shape:**
  - The admitted context is authenticated against the canonical package bytes before
    `owner_enum`.
  - The shape and members come from `AdmittedContext::enum_declaration` and `enum_case`, and
    nothing is minted.
- **Proof shape:**
  - The proof holds no variant list: `from_index` over one `i64` per operand, with exactly one
    final cover.
- **Decode:** the decode reuses the scalar I64 width, comment and arity checks, and a schema whose
  bounds disagree with the member list is refused.
- **Tests:**
  - No test scans repository source.
  - The harness-shape tests assert substrings of the generated harness text. That is an output
    check, and the Kani verdicts back it.
  - Every new test has `#[trace("TC-063", ...)]`.
- **Lock:** the `Cargo.lock` quire-exact bump comes from the normal resolver.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Enum items settle `Supported` while the owner replay route FR-049 requires is not wired. FR-049 Behavior: "CG SHALL reconstruct the member ... and submit it through the existing owner replay route ... If that route is unavailable the item is `unsupported` with no harness." The PR says the route is unavailable, yet emits harnesses. Its reason (`qsl_replay::ScalarOperation` has only Add/Subtract/Multiply/Negate) names the node-level scalar route. But QSL's function replay route (`qsl_replay::replay`, which CG already drives in `src/replay/function.rs`) does accept `WitnessValue::Enum { declaration, member }` for an enum parameter: `qsl-replay/src/execute/argument.rs:773-796`, exercised by QSL `tc_906_leaf_families_replay_and_refuse_by_kind_and_domain` (FR-098-AC-10). The corpus claims are function-shaped (`function lt(a: Level, b: Level): Boolean pure { a < b }`). What blocks replay here is CG's own adapter, which refuses `WitnessValue::Enum` at `src/replay/function.rs:256-260`. So the gap is in this repo, not upstream. Fix: replay the decoded `EnumWitness` through `qsl_replay::replay` with `WitnessValue::Enum` (declaration key and member name from the owner context) in this PR. Or, if node-level replay is truly required and absent upstream, follow the SHALL (settle `unsupported` with a named capability), or get FR-049 amended by its owner before merge. | src/routed/generate.rs:880-937; src/replay/scalar.rs:192-268; src/replay/function.rs:256-260; spec/kani/functional/FR-049-enum-scalar-harnesses.md (Behavior, witness roundtrip) |
| FND-002 | medium | `kani_a_falsifying_counterexample_decodes_and_replays_against_the_declared_operator` replays nothing. After decoding it asserts `left_index == right_index` and `!(left_index < right_index)` in test code. No CG function evaluates the declared operator over the decoded members, and nothing reaches an owner route. So the Status claim "the counterexample is decoded to members and replayed against the declared operator in CG only" and the test's AC-5 tag overstate it. Fix: make the replay real (see FND-001) and assert its reproduced-disagreement verdict. At minimum, rename the test and drop the "replayed" claim. | tests/it/enum_scalar_harness.rs:734-760 |
| FND-003 | low | Leftovers: debug `eprintln!("QSLILL {body}: ok={}", ...)` in a unit test; the no-op `cases[4].3 = true;` (every ordered case is already `true`); and the stem builder in `render_text` duplicates `scalar::scalar_stem`. Fix: remove the first two and call `scalar_stem`. | src/kani/generate/enum_scalar.rs:414-421,919; tests/it/enum_scalar_harness.rs:223 |
| FND-004 | low | `qsl_refuses_the_ill_typed_sources_before_cg` asserts only `compiled.is_err()`. Any compile failure passes it: a grammar typo, a profile change or an unrelated admission refusal. So it does not show that QSL refuses the ordering-over-unordered and distinct-declaration sources, which the FR-049 Status relies on to keep the CG refusal off the source path. Fix: assert the specific QSL diagnostic code for each source. | src/kani/generate/enum_scalar.rs:898-922 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| settle_with / authenticated / admit_carriers / check_typed / render_text | examined | `admit_carriers(&passed_shape, &passed_members, &declaration.shape, 0)?; check_typed(*operator, &declaration, other.as_ref(), count)?;` |
| settle_enum_items / generate_routed_admitted | examined | rewrites only `OperationNotRendered` with `quire.op.enum.` identity |
| owner_enum | examined | shape from `resolve_enum`, members from `context.enum_case` |
| validate_schema / decode_enum_falsification | examined | enum schema bounds `0..count-1`, operator equality |
| EnumScalarIdentity / EnumComparisonNotRendered | examined | new identity and disposition fields |
| tests/it/enum_scalar_harness.rs | examined | 14 host tests, 20 installed-Kani tests |
| qsl-replay 434869e execute/argument.rs Enum arm | context_only | `(ValueType::Enum(shape), WitnessValue::Enum { declaration, member }) => { ... values.push(Value::Enum(EnumMember::new(variant, rank))) }` |
| qsl-replay 434869e scalar.rs ScalarOperation | context_only | `Add`, `Subtract`, `Multiply`, `Negate` only |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | `an_enum_witness_value_is_rendered_into_the_transcript` builds `WitnessValue::Enum` with `declaration: package.site.site.function.clone()`, which is the function's key, not the enum declaration's. The executor stub returns `WrongArm` before QSL ever sees the request, so the test passes only because the witness-arm enum path is never executed. The witness-arm route (`replay_falsification` with an enum argument) therefore has no end-to-end test; only the Input arm (`replay_enum_witness` / `evaluate_boolean_over`) is exercised against QSL. Fix: use the owner's enum declaration key (from the admitted context) and let the request reach `qsl_replay::replay`, asserting a reproduced or evidence verdict. | src/replay/function.rs:1849-1916 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `replay_enum_witness` evaluates the selected QSL function through `qsl_replay::replay` (Input arm) over `WitnessValue::Enum` members; the adapter now renders Enum entries as QSL value text; Status no longer claims the route is unavailable. |
| FND-002 | fixed | `kani_a_falsifying_counterexample_replays_through_the_owner_route` now calls `replay_enum_witness` and asserts the owner result is false where the mutant claimed true; host test `a_decoded_witness_replays_through_the_owner_route_as_the_declared_members` covers all six operators. |
| FND-003 | fixed | `QSLILL` eprintln and the no-op `cases[4].3 = true` removed; stem now `super::scalar::scalar_stem`. |
| FND-004 | fixed | The test now asserts `Package(Check` and `IllTyped(OperatorIneligible)` / `IllTyped(TypeMismatch)` per source. |
| FND-005 | accepted-no-change | Round 2 (7776d34d): test renamed `an_enum_witness_value_is_rendered_into_the_transcript_text`. Its doc comment now says it checks rendering and refusal only, with a stand-in declaration key. No QSL fixture builds an enum function Kani-identity request, so the witness-arm enum path cannot be driven end to end yet. The integration test `a_decoded_witness_replays_through_the_owner_route_as_the_declared_members` covers QSL reading a real declaration: it sends the owner's own key through `qsl_replay::replay`. Narrowing is honest; low risk. |
