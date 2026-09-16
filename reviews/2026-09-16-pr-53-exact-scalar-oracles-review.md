---
id: SR-030
title: "PR 53 exact scalar oracles review"
type: SpecReview
analysis: gap-analysis
scope: "PR #53 at f10587897f0d46727b6eae9b0670264646626ec4; FR-014 and TC-024 (implementation); FR-015 and FR-016 (spec text only)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---
# SR-030: PR 53 exact scalar oracles review

## Summary

One independent review of PR #53 (code review, Rust review and gap analysis for FR-014 and TC-024,
plus a check of whether the FR-015 and FR-016 spec text is enough to build from). The generated
code only calls runtime operators and the meter. It contains no charge amounts, panics or unsafe
code, and its output is deterministic. The problem is how the caller's descriptor is checked. The
generator checks only the node's form, operator class, arity and scalar result and operand types.
It never checks the operation law or its parameters. So a caller can label a node as the wrong
operation and still get a generated oracle.

## Verdict

**FAIL**: FND-001 is high. The caller's descriptor is the only authority on the operation law
(which division rule, which operator, rounding, text profile, decimal target, integer domain). The
V2 input does not constrain any of these. FR-014 says this gap is "recorded as a blocked item", but
no such item exists.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `check_item`/`Shape::of` (src/exact_scalar.rs:558-600, 742-823) check form, operator class, arity and scalar forms only. Nodes 1011, 1012 and 1013 in the corpus are byte-identical apart from the digest. Send a `Floor` descriptor for node 1011 and it generates `rt::divide(Floor, ..)` and reports `Generated`. The same happens for Add vs Multiply, Less vs Greater, IEEE NumericEqual vs TotalOrder vs BitIdentical, rounding mode, text profile, decimal target and integer domain. The oracle becomes a second semantic authority. Fix: (a) check every parameter the input can carry (integer/rational/decimal ranges, float rounding, text bounds) against the node's reachable `bounded_domain` nodes (`node.bounds`), and refuse when they are missing or disagree; (b) for the law itself, which V2 does not carry, add a typed upstream blocker (Contract IR operation identity) and refuse the item, or at least mark the claim as caller-declared so FR-015 cannot treat it as checked; (c) add tests that a mislabelled descriptor is refused or marked. | FR-014-AC-2, FR-014-AC-3, src/exact_scalar.rs |
| FND-002 | medium | `scalar_profile` sets `require_bounds: false` (src/exact_scalar.rs:551). That makes the `RequiresBound` refusal impossible, which contradicts FR-014 Behavior ("unbounded ... typed refusal"). Example: `T_INTEGER` has no bound, yet node 1002 generates with a domain of [-8,7] that only the descriptor supplies. Fix together with FND-001(a), or correct the FR text and delete the dead variant. | FR-014, src/exact_scalar.rs:316, 551 |
| FND-003 | medium | Test gap for AC-2. Many emitted cases are never compiled or run: integer subtract and multiply (both named in AC-2); rational and decimal subtract, multiply and negate; IEEE subtract and multiply; IEEE NumericEqual and BitIdentical; quantity subtract and divide; every comparison operator except Less; ordering LessOrEqual and GreaterOrEqual on integers; text profiles other than NFC. Add corpus entries so the golden file compiles them and the agreement tests execute them. | FR-014-AC-2, TC-024 |
| FND-004 | medium | Test gap for refusals. No test reaches `RequiresBound`, `InvalidBody`, `BodyIncomplete`, `LoweringWorkExhausted` (a closure over 65,536 work units), `SourceTooLarge`, the literal-operand branch of `operand_form` (every corpus operand is a reference), or `found: None`. | FR-014-AC-1, FR-014-AC-3, TC-024 |
| FND-005 | low | Operand classification (src/exact_scalar.rs:678-688) gives misleading reasons. An operand that is itself an application term is reported as `OperandTypeMismatch { found: None }` rather than as unsupported. A literal quantity operand is classified by its `value_kind` (`rational`), not its `unit` type. | src/exact_scalar.rs:678 |
| FND-006 | low | The generated `DecimalType::new(..)` maps failure to `OracleStop::IllTyped` (src/exact_scalar.rs:1114). Every other generated constant maps to `InvalidConstant`, which is what FR-014-CON-2 describes. | FR-014-CON-2 |
| FND-007 | low | Claim-map items are ordered by `CheckedNodeId`'s `Ord`, which compares domain before digest, but the code documents them as "ascending by node digest" (src/exact_scalar.rs:429, 459). A request with a different domain sorts out of digest order. | FR-014-AC-4 |
| FND-008 | low | The matrix row says TC-024 covers FR-014-CON-1 and CON-2, but `quire coverage` 0.32.0 reports those test tags as unmatched because CON ids are not minted trace targets. | spec/test-matrix.md, TC-024 |
| FND-009 | medium | FR-015 is not specific enough. It does not require bounds to come from the lowered IR `bounded_domain` nodes (rather than descriptors). It does not require refusing obligations over caller-declared operation identity (FND-001). The pin set lists only "version, solver and options": it is missing the executable digest, CBMC version, unwind, adapter profile, oracle crate digest and runtime revision. There is no acceptance criterion for vacuous (unsatisfiable) bounds. | FR-015-AC-2, FR-015-AC-4 |
| FND-010 | medium | FR-016 is not specific enough. Its Outputs omit `malformed`, although AC-1 and TC-026 use it. It does not bind a witness to the identity and pins of the harness that produced it. "Same typed outcome" does not say whether charges, consumed counters and limits must match. There is no resource bound on decoding a witness. | FR-016, TC-026 |

## Coverage

- Reconciliation: quire coverage (`quire 0.32.0`, engine 0.46.0@a874fb64), `--scope` set to the
  worktree root. All eight rows FR-014-AC-1 to AC-8 are backed by TC-024 tests. TC-024 is backed.
  FR-014-CON-1 and CON-2 tags are unmatched (FND-008).
- Tasks done: not applicable. PLAN-001 has no task for #48.
- Rows backed by a tagged test: 8 / 8 FR-014 criteria.
- Untraced behaviours and stubs: 0 stubs. The dead `RequiresBound` path is FND-002.
- Semantic review: ran for FR-014 (descriptor authority, refusal taxonomy, determinism, claim map,
  agreement independence). The agreement counts are real: the loops assert their counts, and the
  runtime-only rows are justified because QSL d9d5273 has no integer, rational or ordering operator.
  Agreement cannot catch FND-001, because the direct side restates the same descriptor parameters.
- Gates: `cargo +1.98.1 test --locked --test exact_scalar_generation --test exact_scalar_agreement
  --target-dir target-codex-backends` passed 13/13. `make lint` and `make fmt-check` are clean.
