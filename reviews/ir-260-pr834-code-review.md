---
id: SR-17940486709564847047985
title: "IR-260 PR #834 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #834 at 55457462, git diff origin/main...HEAD; IR-260 and IR-262; src/kani/generate/float_scalar.rs, src/routed/generate.rs, src/replay/scalar.rs, src/replay/function.rs, src/kani/identity.rs, docs/qualification/ir1530-general-route-scalar-table.json, docs/qualification/ir260-float-scalar-kani-measurements.md, tests/it/float_scalar_harness.rs, tests/it/enum_scalar_harness.rs, tests/it/routed_generation.rs, Makefile, tests/it/kani_gate.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-260 (three commits) and IR-262 (two commits, 5ba16a56 and 309f8966). Code-review with
the rust-review lane folded in.

Checked clean. No synthetic IR: `tests/it/float_scalar_harness.rs` compiles a genuine V1-draft QSL
source through `qsl_replay::compile_admitted`, reads it with strict `CheckedPackageV2::read`, and the
installed-Kani tests take the harness from `generate_routed` (`routed_harness`), not from the direct
generator. The harness calls quire-exact's `compare_native_float` over `NativeFloat::binary32/64`
carriers and never the owner's `evaluate_ieee`/`compare_ieee` BigInt path. The expected side reads
the bit patterns directly (field classification, bit equality, `f{32,64}::from_bits(..).total_cmp`)
and shares no function with the native comparison. Draws: `float64` every `i64` reinterpreted as
`u64`, `float32` `0..=u32::MAX`; the only `kani::assume`s are the two domain assumes. One final
cover. `settle_float_items` only rewrites records negotiation left as `OperationNotRendered` for a
float-row identity, so other families are untouched. Decode reinterprets `i64` to `u64` bitwise
(no checked conversion, no host float), and schema validation refuses a width/bound/operator
mismatch. Mutants: each asserts the replaced text occurs in the routed harness and that the
replacement changed it, so none is a no-op. Planner ruling: every arithmetic identity is refused
with `float.<op>.not_kani_tractable` and no harness; this matches quire-exact TC-953's evidence
(no arithmetic operation and width has all three spelling-group harnesses Verified; only
`NearestEven` add/sub at both widths and mul at Binary32 verify), so refusing add/sub/mul at every
spelling is what "fully verified spelling sets" requires. The `div` refusal detail carries the Kani
f32 division finding. The Makefile `KANI_FILTERS` and `EXPECTED_KANI_FILTERS` both gain
`float_scalar_harness`. IR-262: the same-generation refused enum sibling test is real (one routed
request, the guarded node refused with `enum.path_condition`, siblings equal to their solo
generation), and the AC-9 byte-identity now includes a float comparison and a float arithmetic node.

Tractability table tool-version fields: `TractabilityRow` is `deny_unknown_fields` with
`kani_version`, `cbmc_version` and `solver_version` required and no serde default, so a row cannot
load without them. The values copied (Kani 0.68.0, CBMC 6.11.0, cadical-3.0.0) agree with quire-exact
TC-953's provenance paragraph. That does not breach the QSL-406/EA-49 ruling for FR-053 run evidence,
which records no versions; the conflict sits in IR-1526's FR-056 row schema (FND-003).

Runs on this head (own target dir, under `flock /tmp/cargo-slot.lock`, one job at a time): see Verdict.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Operand refusal causes are swapped against FR-053-AC-1. Any operand that is not a `CheckedTerm::Reference` is refused as `float.literal_operand` with the detail "a float literal is not a drawn operand", so a nested expression operand such as `numericEqual(a + b, b)` is labelled a literal. A real float literal (per the FR-053 Status it arrives as a reference to a bit-constructor value node) gets `float.operand_not_parameter`. AC-1 requires `FloatLiteralOperand` for a literal and FR-015's `OperandNotSupported` for an operand that is neither parameter nor literal. Neither case has a test. Classify by what the reference targets (a literal value node versus another node) and add one routed control for each. | src/kani/generate/float_scalar.rs:317-337 |
| FND-002 | low | `replay_float_witness` maps every width other than `"float32"` to `WitnessValue::Float64`. Decode rejects an unknown width earlier, but this is a public function that takes a hand-built `FloatWitness`, so an unknown width replays as binary64. Match `"float64"` explicitly and refuse anything else (or carry the width as an enum, not a `String`). | src/replay/function.rs:1748-1755 |
| FND-003 | low | The five new tractability rows are a record only. The decider does not select them (no integer width on a float clause, as the doc says), `peak_bytes` is `null` although TC-953 records a peak for every row, and `max_operand_bits: 32` ("the width from which the probe did not finish") does not describe add/sub/mul at Binary32, whose `NearestEven` harnesses finished. The required tool-version fields are a churn source that conflicts with QSL-406/EA-49 and belong to IR-1526's FR-056 schema. Record the TC-953 peaks, say in the table doc that `max_operand_bits` is nominal for float rows, and have the lead file an IR ticket against FR-056 to make the version fields optional or drop them. | docs/qualification/ir1530-general-route-scalar-table.json:34-121, src/kani/tractability.rs:181-186 |
| FND-004 | low | The `routed_guarded` test helper finds the guard parameter by matching the `Debug` text of the node body (`format!("{:?}", node.body).contains("value: \"flag\"")`), which breaks silently if the `Debug` derive of `CheckedTerm` changes. Match on the parameter declaration name, as `parameter_declaration` already exposes. | tests/it/enum_scalar_harness.rs:96-106 |

## Verdict

Runs on head 55457462, own target dir, under `flock /tmp/cargo-slot.lock`, one job at a time: whole `cargo test --locked --test it` gave 1064 passed, 0 failed, 97 ignored. This is the first whole run after the Makefile/kani_gate fix. Installed Kani, each test in its own MemoryMax=24G scope: `kani_numeric_equal_float64_verifies` Verified (6.5 s), `kani_total_order_float32_verifies` Verified (2.8 s), `kani_bit_identical_asked_as_numeric_equal_is_falsified` Falsified with a counterexample (26.8 s), `kani_nan_reflexive_numeric_equal_expectation_is_falsified` Falsified (27.4 s), and `kani_a_falsifying_counterexample_replays_through_the_owner_route` Falsified, decoded and replayed through `qsl_replay` (18.4 s). All five agree with the measurement doc's verdict table. The argv was the identity options with unwind 2 and cadical.

Not mergeable as is, only because of the gap-analysis and spec-review findings on this PR. The code itself is sound and follows the planner ruling. FND-001 (operand cause labels) should be fixed in the same PR. FND-002 to FND-004 are small; FND-003's version-field schema question goes to an IR ticket against IR-1526's FR-056 and does not block this PR.

## Dispositions

Round 1, checked at f11eef79. The rendered harness text is unchanged: `render_harness` is untouched and only `settle` and the refusal mapping changed. Whole `--lib` 584 passed; whole `--test it` 1074 passed, 0 failed (QUIRE_CVC5_BIN set, own target dir, flock). New installed-Kani tests run on this head: forced cover `CoverUnsatisfied` (0 of 1); run-evidence test Verified with every option and no version key; ten remaining comparison mutants each Falsified (234 s).

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | A `CheckedTerm::Literal`, or a reference to a literal value node, is now `float.literal_operand`; any other non-parameter operand is `float.operand_not_parameter`. Direct and routed controls (`lit64`, `nested64`) are tested. |
| FND-002 | fixed | `"float64"` is matched explicitly and any other width is `UnsupportedWitnessValue`, with a test. |
| FND-003 | fixed | Rows carry TC-953's peaks (checked against TC-953: the maximum non-Verified peak per row is correct). `max_operand_bits` is documented as nominal. Tool versions are made optional in `TractabilityRow` and FR-056, done in this PR instead of a separate ticket, and the float rows drop them. |
| FND-004 | fixed | The guard parameter is now found through `parameter_declaration(..).name`. |
