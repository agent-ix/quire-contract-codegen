---
id: SR-15351
title: "IR-954 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-954; Cargo.lock, src/kani/generate/frame.rs, tests/it/main.rs, tests/it/scalar_literal_operands.rs"
review_set: subset
---

## Summary

Frozen PR509 head 59f879f3; authentic QSL source compilation, checked V2 admission, lowering, contract admission and public routed generation. Planner-scoped cases: plain add/equality/negate literals, plus bounded-peer equality numeric.narrow. No edited wire, patched node, local producer copy or replay-success stand-in. No applicable AssuranceProfile found.

## Verdict

PASS: no diff defects found. Code-review includes the Rust lane. Full gate remains lead-owned and unrun here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Rust checks: no CI/lint/coverage weakening; no unsafe, production panic, boundary cast, async/locking change, vendoring, host change, new public API or protected IR-948 file. Quantity explicitly follows the existing typed NonRangeType refusal. Arithmetic controls assert literal type, singleton operand range, authentic InlineLiteral node/occurrence/position and actual generated harness draws. Equality checks emitted inline/reference shape and successful oracle generation; narrowed equality checks actual generated checked_bounds empty. This does not inspect QSL private parity Domain::Bounds, mint O-09 identity, execute generated oracle or run scalar replay. Existing scalar_playback helper returns an arithmetic harness and requires a result conversion, so does not provide the new checked-package/equality fixture API. No duplicate public helper was added.

Supplied logs inspected: old producer red control (3 intended reference/literal failures), current scalar/playback (14 pass), frame (3 pass), focused Clippy and fmt pass. No reviewer Cargo/Kani/CBMC execution; one aggregate gate remains required on final reviewed head.

## Examined scope

```yaml
clean: true
scope:
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16.
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention). Independent canonical-preimage checks include only the claimed application node, authentic expression occurrence, authoritative scalar kind and one argument per operand position in actual draw order, each with position ordinal, typed `OperandIdentity` and harness operand range. Actual graph references use `GraphChild { node_id }`; inline literals use `InlineLiteral { node_id, occurrence_key, position }` from the authentic application/occurrence and position, with their value in the singleton range. QSL emits no ext
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: name = "qsl-replay"
- id: src/kani/generate/frame.rs
  path: src/kani/generate/frame.rs
  role: examined
  excerpt: '            | CheckedMemberType::Quantity(_)'
- id: tests/it/main.rs
  path: tests/it/main.rs
  role: examined
  excerpt: mod scalar_literal_operands;
- id: tests/it/scalar_literal_operands.rs
  path: tests/it/scalar_literal_operands.rs
  role: examined
  excerpt: fn equality_with_a_bounded_peer_keeps_the_narrowed_literal_as_a_graph_child() {
findings: []
bindings:
- test_id: add_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: add_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-9
  trace: correct
- test_id: equality_with_a_plain_integer_peer_keeps_the_literal_inline
  ac_id: FR-032-AC-9
  trace: correct
- test_id: negate_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-3
  trace: correct
- test_id: negate_keeps_the_plain_literal_inline_through_scalar_generation
  ac_id: FR-032-AC-9
  trace: correct
- test_id: equality_with_a_bounded_peer_keeps_the_narrowed_literal_as_a_graph_child
  ac_id: FR-032-AC-9
  trace: correct
```
