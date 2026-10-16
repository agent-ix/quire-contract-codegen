---
id: TC-017
title: "Verify checked V2 clause strategy admission and refusal"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: verifies
---
# TC-017: Verify checked V2 clause strategy admission and refusal

## Description

Verify selected-claim V2 strategy admission through IR's public clause-context comparison accessor,
exact domain checks, first-defect diagnostics and atomic bundle output. This case is planned until
CG's V2 strategy implementation, V2 comparison oracle and a CG-side real QSL ConfigVersion V2
producer fixture exist; IR-703's public accessor is implemented in Contract IR.

## Test Procedure

1. Produce a fresh ConfigVersion V2 package in QSL from the direct postcondition
   `self.versionNumber = pre(self.versionNumber)`, admit it through Contract IR and select its
   authentic clause id and claim occurrence. Request a strategy and inspect the census, relation,
   output and diagnostic fields. Independently request the let-bound
   `let s=pre(self) in s.versionNumber` form and assert typed operand ineligibility. Label any
   hand-built topology fixtures synthetic; they do not establish QSL producer behavior.
2. Through public IR admission, construct an invariant with a direct typed `amount < 7` comparison,
   `amount` bounded 0..=1000, and its reversed-order counterpart. Inspect authored operand order,
   operator, provenance, observation and literal position. Assert QSL lowers ConfigVersion's `=`
   to `quire.op.integer.eq` on the admitted package.
3. Put a valid requested claim beside a comparison-oracle-refused sibling in one package. Request
   each claim separately. Exercise a selected-claim oracle-construction failure, each of the six
   operators through the planned V2 comparison-oracle path, arithmetic and Boolean operands,
   an alias, a literal-only comparison, same-read and mixed-observation read pairs.
4. Exercise unknown clause, absent claim, unsupported clause kind and malformed comparison, alone
   and in combinations that establish FR-008's first-defect order. Compare each IR accessor cause,
   clause/claim, structural path and reached child id with CG's diagnostic. Assert no unique
   expression source occurrence is attributed to a child id.
5. Vary each exact read endpoint and literal across i64::MIN - 1, i64::MAX + 1, i128::MIN and
   i128::MAX. Exercise two reads whose ranges are separately i64-representable but unequal, then
   equal. Check all refusal paths and the complete-bundle/zero-artifact boundary.

6. For each of integer.eq, ne, lt, le, gt and ge, build a selected claim over a 1- to 4-member
   domain (read-versus-read, and read-versus-literal with the literal at each member and in either
   authored position), evaluate the V2 comparison oracle for every valuation, and compare each
   result with an independently evaluated ordered relation. Inspect that the eq and ne oracles come
   from the V2 comparison-oracle path and not from `generate_exact_scalar_oracles` or the V1 bound
   oracle.
7. In a CG integration test, obtain the comparison from IR's public accessor through its public API
   only, for each closed operator, provenance and observation variant and each refusal, and compare
   the operator, ordinals, provenance, observation, bounds and refusal fields with CG's census,
   relation and diagnostic. Assert no `CheckedSemanticNodeV2.body` read.

## Expected Results

- The real ConfigVersion direct form admits two `State` entries in authored `Post`, `Pre` order,
  each 0..=1000, with shared project id and selected claim; its let-bound alias refuses.
- The invariant admits `Less`, `amount` and literal 7; reversing operands preserves authored order.
- The good selected claim succeeds beside a refused sibling. The refused selected claim and a
  selected-claim oracle-construction failure emit no artifact. All six operators produce
  executable oracles whose results agree with relation tags. Other ineligible shapes refuse at
  their own locus.
- Refusals preserve typed first cause and contextual structural location; no child id is presented
  as a unique source occurrence. No refusal emits part of a bundle.
- Out-of-i64 endpoints and literals never narrow, wrap or clamp. Unequal read ranges refuse at
  operand one even when both fit i64; equal ranges may admit.
- Every operator's V2 oracle result equals the independent evaluation for every valuation and
  literal position (FR-008-AC-7); eq and ne come from the V2 comparison-oracle path (FR-008-AC-8).
- Every CG census, relation and diagnostic field equals the accessor's value, and no body JSON is
  read (FR-008-AC-9).
