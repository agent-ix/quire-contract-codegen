---
id: SR-15384
title: "IR-17 admitted unit metadata — spec-review/dependency"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-018-composite-equality-oracles.md; spec/oracle/matrix/TC-029-composite-equality-oracles.md; IR-17 PR515"
review_set: subset
---

## Summary

Independent spec-review/dependency of the frozen two-file unit-metadata specification delta. No actionable findings.

## Verdict

**PASS** — Logical order: QSL FR-398 authoritative admitted unit table (enablement) → CG FR-018-AC-45 metadata retention (enablement) → TC-029 planned host verification. Positive Quantity equality is outside this chain and remains refused. The new references edge points upstream to the table owner; no new backward edge or dependency cycle is introduced. The facade contract is already delivered, so no fallback or copied table is needed.

## Examined scope

- `FR-018-AC-45` (examined), spec/oracle/functional/FR-018-composite-equality-oracles.md: After the existing setting/accounting and original-package context guards succeed, generation-time construction and emitted preparation supplied Some(context) attach a clone of that context's complete authoritative unit_table() to the constructed TypeEnvironment, retaining declared but unused units without rebuilding a table, deriving UnitId/NodeKey or substituting another package's table. Valid context-free None behavior remains unchanged. Wrong-package refusal retains its whole-request precedence; direct and nested Quantity items retain their existing typed Unsupported behavior and

- `TC-029-unit-table-metadata` (examined), spec/oracle/matrix/TC-029-composite-equality-oracles.md: Planned host verification for FR-018-AC-45 (IR-17): compile genuine source
through root `compile_admitted` declaring dimension `Length`, units `m` and
`km` with `unit km : Length = rational(1000, 1) * m;`, an additional declared
but unused unit, direct Quantity parameters, a nested Quantity member, and an
independent supported Boolean or Union equality. Preserve the original source,
package and genuine context. Read expected unit entries and their owner-issued
identities independently from that original context's public facade; construct
no replacement IDs or table. Include a unit ab

- `FR-018-AC-18` (context_only), spec/oracle/functional/FR-018-composite-equality-oracles.md: `render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference` returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported` with `node_tag` equal to `family` and `unsupported_node_id` equal to the item's expression node id, emitting no code for the item and leaving its siblings unchanged, while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVarian

- `FR-018-AC-38` (context_only), spec/oracle/functional/FR-018-composite-equality-oracles.md: With `None`, standalone generation and emitted preparation accept successfully reconstructed non-nominal primitive and nested Option/Collection closures; their index is empty only after complete zero-enum resolution. Existing admitted-IR conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion") and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext, no symbols for the refused item and unchanged independent siblings. This does not claim matching QSL source emission or source authentication for those IR

- `FR-018-AC-40` (context_only), spec/oracle/functional/FR-018-composite-equality-oracles.md: Some(wrong-package context) fails the whole standalone call after existing setting/accounting checks and before item classification, for primitive-only, nominal and empty requests. Emitted context-free and nominal constructors both reject the same supplied mismatch before reconstruction. None and Some(wrong) have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper.

- `QSL FR-398` (context_only), quire-spec-language/spec/functional/FR-398-expose-the-admitted-unit-table-through-the-replay-facade.md: unit_table returns the package’s kernel unit table: each admitted unit by its kernel id. The context lists every declared unit, used or not.

## Review limits

The requested scope is the new unit metadata contract and TC-029 procedure, not the entire inherited IR-17 migration. No applicable AssuranceProfile or required review selection exists. SpecReview template and installed schema were read. Quire spec validation succeeded; existing unrelated grammar warnings were preserved. No Cargo, build, tests, backend, Kani, namespace execution, workflow change or frozen-branch edit occurred. Rust review and production gap analysis do not apply to this Markdown-only delta; object review adds no separate lens because no domain schema changes. Jev criterion strength and optional whole-repository semantic review were not selected. Planned test evidence is not current qualification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
