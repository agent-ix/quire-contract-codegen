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

## Disposition pass 2

Candidate 21ff3635 versus review-custody 3314af5b changes only tests/it/routed_generation.rs and tests/it/exact_scalar_agreement.rs. No new defect or oracle weakening found. The prior aggregate attempt failed two tests and never started Kani; its result is not passing evidence. Supplied fix-round focused logs show 15 scalar/playback/routed controls pass, both agreement fixtures pass concurrently, and focused Clippy/fmt pass. Reviewer ran no build, full gate or Kani.

The work fixture now authors 2049 genuine x-plus-literal leaves. Its balanced operand has 4097 distinct addition applications plus one shared parameter, counted from the admitted producer graph as 4098 referenced nodes. The independent sum 0 through 2048 is 2098176, matching the updated bounded result. Removing leaf-only parentheses preserves associative addition grouping while reducing syntax-node cost; internal parentheses retain balanced depth. Production inference limits are unchanged: 4096 work and endpoint bits, 64 depth. The test still demands typed Nodes refusal at consumed=4097 with no harness, and still compares the supported sibling against its isolated routing and generated claim. The new census measures the actual producer graph; it does not fabricate wire or replace the refusal oracle.

The agreement runner now gives each generated fixture root its own target directory. All four callers use distinct labels, including the two scalar corpora that share package name/version. Claim-map-derived aliases, generated source, independent runtime comparison cases, success status and exact passed-test count checks are unchanged. Isolation prevents sibling generated-library artifacts from being selected. The material cost is compiling dependencies per scratch fixture; no product behavior or gate threshold changes.

All round-1 outcomes remain closed: 303 deferred baseline owning-spec items and 17 rejected default-lane-only rationales. No finding is fixed by this patch and no new finding is added. The repository completeness verdict remains FAIL for baseline untagged work; scoped IR-954 review is acceptable pending custody and the lead-owned final clean-head gates. No new issue, execution waiver or replay-completion claim.

No disposition rows are required this round: the prior real findings all have final outcomes; the clean code-review placeholder is not a defect.

```yaml
dispositions: []
new_findings: []
scope:
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with
    the Integer `unbounded_type` and no harness; a direct reference to a raw application
    never borrows a separate narrowing conversion's domain, and an inferred endpoint
    outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming
    both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact
    scalar refusal, including its node, position or limit fields, and no harness.
    Refused items leave a suppor
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-9
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Generated oracle functions do not panic: an invalid generated constant,
    including a decimal target, stops as `InvalidConstant`, an operand of the wrong
    width stops before any charge, and generated source over its ceiling is a typed
    error with no output.'
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles
    against the runtime alone, and each oracle''s outcome, admitted charges and consumed
    counters equal direct runtime execution. This criterion claims no agreement with
    the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle
    against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming it.'
- id: FR-014-AC-46
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An inferred interval uses only the four named integer operations and canonical
    literal or `integer_range`-typed leaves. A direct reference to a raw application
    consumed by a separate narrowing conversion never takes that conversion's domain
    as its operand interval; where the raw application has another consumer, requesting
    that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer
    parameter at any depth remains `RequiresBound` naming Integer. A missing, cyclic
    or unsupported application, wrong-kind or noncanonical literal, and an inference
    depth, work or endpoint-bit
- id: tests/it/routed_generation.rs
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: '        4098,'
- id: tests/it/exact_scalar_agreement.rs
  path: tests/it/exact_scalar_agreement.rs
  role: examined
  excerpt: '        .env("CARGO_TARGET_DIR", root.join("target"))'
bindings:
- test_id: tc_033_qsl_nested_operand_depth_limit_is_typed_and_keeps_sibling_supported
  ac_id: FR-014-AC-46
  trace: correct
- test_id: tc_033_qsl_nested_operand_depth_limit_is_typed_and_keeps_sibling_supported
  ac_id: FR-015-AC-104
  trace: correct
- test_id: tc_024_generated_scalar_crate_agrees_with_direct_runtime
  ac_id: FR-014-AC-6
  trace: correct
- test_id: tc_024_generated_scalar_crate_agrees_with_direct_runtime
  ac_id: FR-014-AC-9
  trace: correct
- test_id: tc_024_v2_boolean_and_equality_oracles_agree_with_direct_runtime
  ac_id: FR-014-AC-37
  trace: correct
```
