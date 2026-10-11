---
id: SR-8302
title: integrity review of IR-306 PR 362
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@235a416299edd2ed6b0bbb6e84db79f62d5d6827; spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md,
  spec/routed/matrix/TC-033-routed-generation.md
review_set: subset
---

## Summary

Ticket: IR-306. This integrity pass examined the four-file spec diff at 235a416299edd2ed6b0bbb6e84db79f62d5d6827. It found 1 defect(s).

## Reviewed units

- `FR-014-AC-25` (examined; `spec/oracle/functional/FR-014-exact-scalar-oracles.md:441`): A `reference` operand to a plain scalar parameter or a non-literal `value` node is refused as `RequiresBound` naming its type, in generation and in derivation, where another operand owns a bound or the result is typed by one (`x + y`, `y + y`, and `x + y` over a scalar-typed result); a literal operand is never refused as `RequiresBound`. An eligible application reference follows FR-014-AC-45 instead.
- `FR-014-AC-45` (examined; `spec/oracle/functional/FR-014-exact-scalar-oracles.md:461`): For a QSL-shaped outer integer operation narrowed to an `integer_range`, a direct reference to a plain-Integer inner application over independently bounded operands supplies a conservative inclusive operand interval: `(x + 1) + 1` for `x: Int[0, 9]` supplies `[1, 10]` for its inner operand, and `-(x + 1)` supplies `[1, 10]` to the outer negate; `x * -1` for `x: Int[1, 9]` supplies `[-1, -1]` from a negate application over the canonical literal `1`. Each outer node derives and generates an `ir_confirmed` oracle, records the inferred range by argument position without inventing a checked bound i
- `FR-014-AC-46` (examined; `spec/oracle/functional/FR-014-exact-scalar-oracles.md:462`): An inferred interval uses only the four named integer operations and canonical literal or `integer_range`-typed leaves. A direct reference to a raw application consumed by a separate narrowing conversion never takes that conversion's domain as its operand interval; where the raw application has another consumer, requesting that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer parameter at any depth remains `RequiresBound` naming Integer. A missing, cyclic or unsupported application, wrong-kind or noncanonical literal, and an inference depth, work or endpoint-bit
- `FR-015-AC-11` (examined; `spec/kani/functional/FR-015-bounded-kani-obligations.md:629`): Every symbolic argument carries an inclusive assumption equal to its IR `bounded_domain`, its FR-014 inferred application interval (FR-015-AC-103), or a literal operand's own value (FR-015-AC-16), and a bounded-integer post-state result is required to lie in the same domain; no generated source carries a `#[kani::unwind]`.
- `FR-015-AC-103` (examined; `spec/kani/functional/FR-015-bounded-kani-obligations.md:722`): Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0, 20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x: Int[0, 9]` into `Int[-10, -1]` each settle `Supported` with an IR-confirmed outer oracle and a harness. Their respective argument ranges are `[1, 10]` and `[1, 1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the outer declared result bounds. Each inferred range carries its source application id in the harness identity.
- `FR-015-AC-104` (examined; `spec/kani/functional/FR-015-bounded-kani-obligations.md:723`): A plain-Integer parameter as any nested leaf keeps `requires_bound` with the Integer `unbounded_type` and no harness; a direct reference to a raw application never borrows a separate narrowing conversion's domain, and an inferred endpoint outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch` and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact scalar refusal, including its node, position or limit fields, and no harness. Refused items leave a suppor
- `TC-024` (examined; `spec/oracle/matrix/TC-024-exact-scalar-oracles.md:96`): 20a. From admitted QSL-shaped nodes, derive and generate the outer operations
- `TC-033` (examined; `spec/routed/matrix/TC-033-routed-generation.md:99`): 19. **Nested bounded operands (FR-014-AC-45, FR-015-AC-103).** Route the outer
- `FR-015` (examined; `spec/kani/functional/FR-015-bounded-kani-obligations.md:148-151`):   bound comes from the IR `bounded_domain` nodes the closure reaches, a   canonical literal, or the independently inferred application interval of   FR-014-AC-45. Each operand is a parameter, literal or eligible referenced   integer application. The request also carries the refinement case cap of

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The composite-equality input contract admits inferred integer application operands from FR-014, but the FR-015 shadow-shape table still refuses every operand other than a parameter or literal as OperandNotSupported. Limit this new input rule to V2 scalar claims or define composite-equality behavior consistently. | spec/kani/functional/FR-015-bounded-kani-obligations.md:148-151 |

## Verdict

FAIL
