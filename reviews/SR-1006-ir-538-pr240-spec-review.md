---
id: "SR-1006"
title: "CG PR 240 spec review: FR-018-AC-17 and AC-18 ban panic sites in emitted and generator source"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@daa93aedc7a5e88c9a43b4114cb6010a4ede7d38; spec/oracle/functional/FR-018-composite-equality-oracles.md (two new Behavior bullets, FR-018-AC-17, FR-018-AC-18 and their mutation rows), spec/oracle/matrix/TC-029-composite-equality-oracles.md (step 9, Expected Results), spec/oracle/matrix/tests.md (FR-018 row and TC-029 traces); diff origin/main...HEAD, base 85b8114"
---

# SR-1006: CG PR 240 spec review

## Summary

Ticket: IR-538. PR: agent-ix/quire-contract-codegen#240 at daa93ae, spec-only (3 files, +30/-1).
This review covers requirement quality (EARS form, integrity, atomicity), consistency with the
rest of FR-018 and with the code the criteria constrain, and how strong the two new mutation rows
are as test oracles. I measured every claim against the code at the PR head and did not rely on
the PR body or the ticket text.

What I measured:

- The six emitted `.expect` sites exist exactly where the PR says. They are in
  `src/oracle/equality/mod.rs` at 1414 (the `integer` helper), 1559 (`RationalDomain`), 1577
  (`TextType`), 1588 (`CardinalityBound`), 1603 (`IntegerInterval`) and 1611 (`DecimalType`).
  The two `unreachable!` arms are at 1574 (`ValueType::Quantity`) and 1595
  (`ValueType::Reference`). AC-17's mutation row names all six reconstructions, and AC-18's
  names both arms.
- `src/oracle/scalar/mod.rs` has no `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!` or
  `unimplemented!` before its `#[cfg(test)]` module at line 3343. Every hit is a test site.
- Each AC is a direct assertion. The new ACs are 🚧 in both the TC-029 step and the tests.md row.
  The TC-029 traces list grows to AC-18. Numbering runs 1 to 18 with no gap, and every AC has
  exactly one mutation row. No id cell carries an emoji.
- `quire validate` (spec, plan, reviews) exits 0. `quire coverage --strict` reports 66 unbacked
  rows both at base and at head: 323 rows at base and 325 at head, 214 backed in both. The two
  new planned rows do not add unbacked rows. `make spec` was not used, because the Makefile
  hardcodes `$(TRUSTED_HOME)/.npm-global/bin/quire`. I ran `quire` 0.33.0 from PATH.

## Verdict

Changes requested. The scan half of both ACs is sound: AC-18 counts the string literals the
generator emits, so the AC-17 and AC-18 mutants (re-adding an `.expect` template or an
`unreachable!` arm) are killed by the generator-source scan even where the corpus does not
render a given template. The behavioural halves are the problem:

- AC-18 names the wrong error type for a per-item refusal (high).
- AC-18 describes a request that no public input can produce, so its behavioural test is either
  impossible or vacuous.
- AC-17 promises a typed `Err(InvalidDeclaration)` whose cause the runtime cannot express.
- AC-17's failure path has no test that could fail.
- The ACs reach into FR-014's generator, although the requirement statement does not.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-018-AC-18 (and the second new Behavior bullet) refuses "the item" with an `OracleGenerationError` and says "that item does not appear in the emitted crate". But `OracleGenerationError` is the whole-call failure type. The entry point's doc says it "Fails as a whole only with `SourceTooLarge`, `ClaimMapSerialization` or `UnknownRuntimeVariant`; every per-item problem is a refusal in the claim map". Rendering runs in `source.item(..)?` after the claim is recorded, so an `Err` there returns no crate at all and drops every sibling, which contradicts AC-1 (siblings unchanged). The per-item type is `CompositeEqualityRefusal`, which already has an `Unsupported` variant. Name the per-item refusal and variant, and say that rendering, or a pre-render check, must happen before the item is committed as Generated. Or, if a whole-call error is intended, say so and drop "that item does not appear" | spec/oracle/functional/FR-018-composite-equality-oracles.md:240, spec/oracle/functional/FR-018-composite-equality-oracles.md:212-215, src/oracle/equality/mod.rs:541-548, src/oracle/equality/mod.rs:644, src/oracle/claim.rs:74-91 |
| FND-002 | medium | The behavioural half of FR-018-AC-18 cannot be reached through the public entry point. No resolution path produces `ValueType::Quantity` or `ValueType::Reference`: quantity `scalar_type` leaves are refused earlier as unsupported, and a `Reference` operand is refused by Contract IR (`tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today`). TC-029 step 9 says "request an equality whose operand type is rendered with a `ValueType::Quantity`". No such request exists. A public-API test would hit the earlier refusal and pass today with both `unreachable!` arms still present. The step should call `render_value_type` directly from the module's `#[cfg(test)]` tests and assert the named typed refusal. Otherwise, drop the behavioural clause and rely on the scan | spec/oracle/matrix/TC-029-composite-equality-oracles.md:107-115, spec/oracle/functional/FR-018-composite-equality-oracles.md:240, src/oracle/equality/mod.rs:1197, src/oracle/equality/mod.rs:1573-1596 |
| FND-003 | medium | The second clause of FR-018-AC-17 has no test that could fail. That clause says a literal or bound that fails to reconstruct yields `Err(InvalidDeclaration)` or `Outcome::Refused(CheckedInvariant)`. No fixture can make a reconstruction fail under the locked runtime, because every bound is validated at generation, and TC-029 step 9 only scans for tokens. Mutants that pass the scan but break the property: a silent fallback such as `.ok()`/`unwrap_or(..)` to an unbounded type, which widens the type, or `assert!`/`assert_eq!`/`debug_assert!`, `std::process::abort()`, `unwrap_unchecked` or slice indexing. The token list omits all of these. Strengthen it in one of these ways. (a) A structural assertion on the emitted source: every reconstruction helper returns `Result`, and its `Err` reaches only `environment_*`'s `Err` or the oracle's `Refused(CheckedInvariant)`. (b) Widen the token list. (c) Have the emitted crate deny the clippy panic lints, since AC-2 already compiles it. Add the silent-fallback mutant to the AC-17 mutation row | spec/oracle/functional/FR-018-composite-equality-oracles.md:239, spec/oracle/functional/FR-018-composite-equality-oracles.md:272, spec/oracle/matrix/TC-029-composite-equality-oracles.md:107-115 |
| FND-004 | medium | FR-018-AC-17 makes a failed reconstruction in the environment constructor yield `Err(InvalidDeclaration)`, but does not say which `declaration` or `cause`. The runtime's `InvalidDeclaration { declaration: String, cause: DeclarationCause }` has a `#[non_exhaustive]` cause with only `DuplicateKey`, `DuplicateMember`, `UnknownDeclaration`, `Type(IllTypedCause)` and `Recursion`. None of these describes an empty interval, a zero denominator or an empty cardinality bound, so emitted code would have to fabricate a misleading cause. Name the cause the emitted code reports, or choose a different typed outcome. If neither fits, raise the runtime need | spec/oracle/functional/FR-018-composite-equality-oracles.md:239, spec/oracle/functional/FR-018-composite-equality-oracles.md:207-211 |
| FND-005 | medium | The ACs claim more than the requirement statement. The new bullet says "The generator shall emit no ...", and in FR-018 that means the composite equality generator. AC-17 also scans FR-014's TC-024 scalar crate, and AC-18 scans `src/oracle/scalar/mod.rs`. Both are FR-014's subject, and FR-014 has no such AC, and both trace only to TC-029. Both are already clean (measured: zero non-test sites in the scalar generator), so the scalar half is a regression guard filed under the wrong owner. Move it to an FR-014 AC traced to TC-024, or name the scalar generator in FR-018's statement | spec/oracle/functional/FR-018-composite-equality-oracles.md:205-211, spec/oracle/functional/FR-018-composite-equality-oracles.md:239-240 |
| FND-006 | low | The new first bullet overlaps the existing Behavior bullet. That bullet says each emitted oracle function "shall contain no `unwrap`, `expect`, index or arithmetic that can panic". The new bullet bans tokens but drops index and arithmetic panics, so FR-018 now states the panic rule twice, with two different scopes. The bullets ban the bare word `unwrap`, while the AC bans `.unwrap(`. The emitted oracle already uses `.unwrap_or_else(left_source_*)`, which a word-level reading would ban. Merge the two bullets into one rule that keeps index and arithmetic, and spell the token the same way as the AC | spec/oracle/functional/FR-018-composite-equality-oracles.md:133-136, spec/oracle/functional/FR-018-composite-equality-oracles.md:205-211, src/oracle/equality/mod.rs:1473 |
| FND-007 | low | EARS and atomicity. The first new bullet joins three obligations: no panic tokens in emitted source, none in generation code, and how a failed reconstruction maps to a typed value. The last is a separate behaviour, not part of a ban. The second bullet carries rationale in the requirement ("rather than asserting that an earlier stage already refused it"). Split the first bullet into three, and move the rationale into prose | spec/oracle/functional/FR-018-composite-equality-oracles.md:205-215 |
| FND-008 | low | Two scoping words are imprecise. "The emitted `src/lib.rs` of the TC-029 corpus crate" is ambiguous, because TC-029 generates many crates across its tests; name the agreement-corpus generation. And "its own generation code shall contain none" is wider than AC-18's two-file scan. The equality generator also runs `core::naming`, `core::artifact` and `oracle::claim`, all clean today. Either state that the file list is the definition, or widen the scan | spec/oracle/functional/FR-018-composite-equality-oracles.md:206-207, spec/oracle/functional/FR-018-composite-equality-oracles.md:239-240 |

## New findings (disposition pass 1)

Reviewed at f2f2786a5f17533700ba370beb088b5d35cf47bc. Only the fix-round delta, daa93ae..f2f2786, is in scope.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | FR-018-AC-18 and its Behavior bullet require `Err(OracleGenerationError)` but name no variant. None of the existing variants fits a known but unsupported `ValueType`: `SourceTooLarge`, `ClaimMapSerialization`, `LocationMapSerialization` and `UnknownRuntimeVariant`. Reusing `UnknownRuntimeVariant { enum_name: "ValueType" }` would misreport Quantity and Reference as unknown variants. Adding a variant changes the entry point's documented set of whole-call failures. Name the variant | spec/oracle/functional/FR-018-composite-equality-oracles.md:220-221, spec/oracle/functional/FR-018-composite-equality-oracles.md:246, src/oracle/claim.rs:74-91 |
| FND-010 | low | FR-018-AC-17's structural clause checks only calls of reconstruction helpers. Suppose a fallible runtime constructor (`IntegerInterval::new`, `RationalDomain::new`, `TextType::new`, `CardinalityBound::new`, `DecimalType::new`, or `.parse()` of an integer) is called inline, outside any helper, with a `match` whose `Err` arm falls back to an unbounded type. That passes every AC-17 check: no banned token and no helper call. The Behavior bullet ("shall do so through helpers") is not asserted. Add: each such constructor call appears only inside a reconstruction helper | spec/oracle/functional/FR-018-composite-equality-oracles.md:245, spec/oracle/matrix/TC-029-composite-equality-oracles.md:107-117 |
| FND-011 | low | FR-014-AC-39 has no owning FR-014 Behavior statement: FR-014's Behavior section has no panic-ban bullet. Its text opens with a status prefix, "PLANNED (IR-538).". That prefix duplicates the 🚧 matrix row, is not part of a direct assertion, and goes stale when the test lands | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351, spec/oracle/functional/FR-014-exact-scalar-oracles.md:199-308 |
| FND-012 | low | TC-024 step 3 (the FR-014-AC-39 scan) is appended as step 3 of the "Boolean connectives, comparisons and source maps" subsection. There, "every crate this corpus generates" reads as only that subsection's Boolean corpus, not every crate TC-024 generates. Give the scan its own section, or name the corpora it covers | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:157-162 |
| FND-013 | low | Two problems in the new Behavior bullets. The first bullet defines its token list only by reference to FR-018-AC-19, so the requirement depends on its own acceptance criterion. The reconstruction bullet carries a rationale sentence ("The runtime's `DeclarationCause` has no cause for ... so the constructor does not report one as `InvalidDeclaration`"). State the list, or the prohibition, in the statement, and move the rationale into prose | spec/oracle/functional/FR-018-composite-equality-oracles.md:211-219 |
| FND-014 | low | `ReconstructionError` is used by the Outputs bullet, the Behavior bullet, AC-17 and TC-029 step 9, but nothing says who defines it or what it carries. The runtime at the locked revision (ccc722b) has no such type. The Outputs bullet says the generated crate defines `EnvironmentError`, but is silent on `ReconstructionError`. Say the generated crate defines it, and what it holds: the runtime's `EmptyInterval`, `ZeroDenominator` and `EmptyCardinalityBound`, a parse error, or a unit marker | spec/oracle/functional/FR-018-composite-equality-oracles.md:112-116, spec/oracle/functional/FR-018-composite-equality-oracles.md:213-216 |

## New findings (disposition pass 2)

Reviewed at 7ada4ff4f3f07004555df987eb5050973d340971. Only the fix-round delta, f2f2786..7ada4ff, is in scope.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | medium | The new whole-call variant `OracleGenerationError::UnsupportedValueType { family }` is a breaking change to a public API, for a path the spec itself calls unreachable. `OracleGenerationError` is re-exported at the crate root (src/lib.rs:52), shared by the scalar, function and equality generators, and wrapped by `RoutedGenerationError::Oracle`. It is not `#[non_exhaustive]`. quire-driver matches it exhaustively with no wildcard in `oracle_code` and `oracle_message` (quire-driver src/refusal.rs:241-247, 327-341), so the driver stops compiling (E0004) on its next CG bump, and it needs a catalog `Code` for the new variant. The spec records no downstream impact and no code mapping. Either keep the error internal: a private render error mapped at the item boundary, or a check before the item is committed that yields `CompositeEqualityRefusal::Unsupported`, so the public enum is unchanged. Or keep the variant, record the driver change (its `Code`, for example `RuntimeInvariant`) in FR-018, and coordinate the driver bump | spec/oracle/functional/FR-018-composite-equality-oracles.md:222-234, spec/oracle/functional/FR-018-composite-equality-oracles.md:259, src/oracle/claim.rs:73-91, src/lib.rs:52, src/routed/generate.rs:157 |
| FND-016 | low | The new rationale paragraph sits inside the Behavior bullet list. "If the generated source exceeds its size ceiling" (line 235) follows the paragraph with no blank line, so the list renders as two lists with prose between them. Move the paragraph after the last Behavior bullet | spec/oracle/functional/FR-018-composite-equality-oracles.md:226-236 |

## New findings (disposition pass 3)

Reviewed at 4f179aaf718802c85bf058629274de4015bc5b5e. Only the fix-round delta, 7ada4ff..4f179aa, is in scope.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-017 | medium | `render_value_type` now returns `Err(RenderError::UnsupportedValueType { family })`, and the item-boundary bullet turns any `RenderError` into a per-item `CompositeEqualityRefusal::Unsupported` with `node_tag = family`. But the same function also yields `OracleGenerationError::UnknownRuntimeVariant`: its own `&_ => unknown_variant("ValueType")` arm, plus `collection_kind_path`, `rounding_path` and `profile_path`, which it calls. claim.rs documents that error as a whole-call failure, because a stale runtime "puts every item's result in doubt". The spec does not say how `RenderError` carries that case, and it has no `family` to map. As written, an implementer may fold it into `RenderError` and downgrade it to a per-item refusal, silently changing the documented whole-call policy. State that `RenderError` carries the unknown-variant case separately, for example as `Generation(OracleGenerationError)`, and that only `UnsupportedValueType` maps per item while the other case still fails the whole call | spec/oracle/functional/FR-018-composite-equality-oracles.md:222-228, src/oracle/equality/mod.rs:1597, src/oracle/equality/mod.rs:1636-1660, src/oracle/claim.rs:60-72 |
| FND-018 | low | `CompositeEqualityRefusal::Unsupported` also requires `unsupported_node_id` ("First unsupported reachable node"). `RenderError` carries only `family`, and neither the bullet, nor AC-18, nor TC-029 step 10 says which node id the mapping reports: the item's node, or the operand type node. Two implementers would differ, and the test asserts only `node_tag`. Name the id | spec/oracle/functional/FR-018-composite-equality-oracles.md:227-229, spec/oracle/functional/FR-018-composite-equality-oracles.md:263, src/oracle/equality/mod.rs:374-379 |

## New findings (disposition pass 4)

Reviewed at 1915018a93ca410f12ddf5005101844cf60351f1. Only the fix-round delta, 4f179aa..1915018, is in scope.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-019 | low | The new node-id rule, "`unsupported_node_id` equal to the type node of the operand being rendered (as the existing quantity refusal reports it)", matches the code only for a top-level operand. The existing quantity refusal reports the member's own type node: `resolve_scalar`'s `type_id` is "the member's type node", passed down from `resolve_type` at every nesting level (equality/mod.rs:1062-1064, 1107-1110, 1197-1201). So for a quantity nested inside a record field, option or collection, the existing refusal names that inner node, not the operand's type node. `render_value_type` is also called from `render_composite_declaration` (1508), for the field types of the `composites_*` constructor, where no operand is being rendered. Say which id applies there, or say "the outermost type node the failing render was called for", and drop the parenthetical. The path is unreachable today, so this is wording only | spec/oracle/functional/FR-018-composite-equality-oracles.md:230-234, spec/oracle/functional/FR-018-composite-equality-oracles.md:272, src/oracle/equality/mod.rs:1107-1110, src/oracle/equality/mod.rs:1508 |

## Dispositions

Round 1, reviewed at f2f2786a5f17533700ba370beb088b5d35cf47bc (fix commit f2f2786, the
fix-round delta over daa93ae). `quire validate` (spec, plan, reviews) exits 0. `quire coverage
--strict` reports 66 unbacked rows, the same as the baseline: 327 rows, 214 backed.

Design checks:

- `EnvironmentError { Declaration(InvalidDeclaration), Reconstruction(ReconstructionError) }`
  is sound against the runtime. `InvalidDeclaration` is a public struct with public fields
  (composite.rs:873), so wrapping it in an emitted enum needs nothing from the runtime.
- The oracle's mapping of a failed reconstruction to `Outcome::Refused(Refusal::CheckedInvariant)`
  matches FR-021:58 ("`Outcome::Refused(Refusal::CheckedInvariant)`, never an `unwrap` or
  `expect`").
- It is not a compatibility layer. The constructor's return type is replaced outright, and no
  old `InvalidDeclaration` signature is kept. The emitted signature change reaches the
  repository's agreement tests: tests/composite_equality_support/agreement.rs and
  agreement_cases.rs, and tests/it/composite_equality_generation.rs.
- TC-029 step 10 is reachable and not vacuous. `render_value_type` is a private function in the
  same module, so `use super::*` reaches it. `ValueType::Quantity(QuantityUnit::Compound(CompoundUnit::dimensionless()))`
  and `ValueType::Reference(NodeKey::from_hex(..)?)` can both be built through public runtime
  API. With today's `unreachable!` arms the test panics, so it fails.
- The widened token lists (AC-19, FR-014-AC-39) can be met today. The non-test parts of
  equality/mod.rs and scalar/mod.rs have no `assert!`, `debug_assert!`, `process::abort` or
  `unwrap_unchecked`.
- The emitted-source index heuristic in AC-17 matches no current emitted template.
- Numbering and counts are consistent. FR-014-AC-39 follows AC-38, and FR-018-AC-19 follows
  AC-18. TC-024's and TC-029's trace lists and the tests.md rows are extended, with no gap and
  no emoji in an id cell.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f2f2786 |
| FND-002 | fixed | f2f2786 |
| FND-003 | fixed | f2f2786 |
| FND-004 | fixed | f2f2786 |
| FND-005 | fixed | f2f2786 |
| FND-006 | fixed | f2f2786 |
| FND-007 | fixed | f2f2786 |
| FND-008 | fixed | f2f2786 |

Round 2 dispositions, reviewed at 7ada4ff4f3f07004555df987eb5050973d340971 (fix commit 7ada4ff).
`quire validate` (spec, plan, reviews) exits 0. `quire coverage --strict` reports 66 unbacked
rows, the same as the baseline (327 rows, 214 backed). The committed copies
reviews/SR-1006-ir-538-pr240-spec-review.md and reviews/SR-1007-ir-538-pr240-gap-analysis.md
were byte-identical to the round-1 scratchpad files. With this round's additions, SR-1006 now
differs, so it must be re-copied. SR-1007 is unchanged.

Checks on this round's changes:

- `ReconstructionError`'s six unit variants (`Integer`, `Interval`, `Rational`, `Decimal`, `Text`,
  `Cardinality`) map one to one onto the six reconstruction sites: the `integer` helper,
  `IntegerInterval`, `RationalDomain`, `DecimalType`, `TextType` and `CardinalityBound`. Both
  enums are defined in the generated crate as public types, so the emitted crate's
  `-D warnings` build reports no dead code.
- `UnsupportedValueType` cannot be reached through `generate_composite_equality_oracles`.
  `resolve_type` never yields `ValueType::Quantity` or `ValueType::Reference`: quantity leaves
  are `Unsupported` and a `composite_type.reference` is `BlockedOnUpstream`. So the variant is
  consistent with AC-1. Its cost is to public API (FND-015).
- The FR-014 Behavior bullet now owns FR-014-AC-39, and the AC's "PLANNED" prefix is gone.
  TC-024 has its own "Panic-free source scan" section, which covers every crate from all of
  TC-024's sections.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-009 | fixed | 7ada4ff |
| FND-010 | fixed | 7ada4ff |
| FND-011 | fixed | 7ada4ff |
| FND-012 | fixed | 7ada4ff |
| FND-013 | fixed | 7ada4ff |
| FND-014 | fixed | 7ada4ff |

Round 3 dispositions, reviewed at 4f179aaf718802c85bf058629274de4015bc5b5e (fix commit 4f179aa).
`quire validate` (spec, plan, reviews) exits 0. `quire coverage --strict` reports 66 unbacked
rows, the same as the baseline (327 rows, 214 backed). At 4f179aa the committed copies in
`reviews/` were byte-identical to the round-2 scratchpad files. With this round's additions,
SR-1006 differs again and must be re-copied. SR-1007 is unchanged.

FND-015 is fixed. The public `OracleGenerationError` is no longer changed, `RenderError` is
private to `src/oracle/equality/mod.rs`, and the render failure reaches callers only as a
per-item `CompositeEqualityRefusal::Unsupported`. That is consistent with AC-1: the item is
refused, and its siblings are unchanged. `node_tag` is documented as "Its family or form", and
`"quantity"` is already used there, so `family` as the `node_tag` fits the code. FND-016 is
fixed: the rationale paragraph now follows the last Behavior bullet, so the list renders as
one.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-015 | fixed | 4f179aa |
| FND-016 | fixed | 4f179aa |

Round 4 dispositions, reviewed at 1915018a93ca410f12ddf5005101844cf60351f1 (fix commit 1915018).
`quire validate` (spec, plan, reviews) exits 0. `quire coverage --strict` reports 66 unbacked
rows, the same as the baseline (327 rows, 214 backed). At 1915018 the committed copies in
`reviews/` were byte-identical to the round-3 scratchpad files. With this round's additions,
SR-1006 differs again and must be re-copied. SR-1007 is unchanged.

FND-017 is fixed. `RenderError::Generation(OracleGenerationError)` carries the whole-call
`UnknownRuntimeVariant` from each site, measured: `render_value_type`'s own `&_` arm (1597),
`collection_kind_path` (1636), `rounding_path` (1648) and `profile_path` (1660). Only
`UnsupportedValueType` maps per item, `Generation` fails the whole call as it does today, and
AC-18 and TC-029 step 10 assert both branches. FND-018 is fixed: the id is now named. Its
precise wording is FND-019.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-017 | fixed | 1915018 |
| FND-018 | fixed | 1915018 |

Round 5 dispositions, reviewed at 58f7f8f851b90bf892deec6159b96f122a3b6d1e (fix commit 58f7f8f).
`quire validate` (spec, plan, reviews) exits 0. `quire coverage --strict` reports 66 unbacked
rows, the same as the baseline (327 rows, 214 backed). At 58f7f8f the committed copies in
`reviews/` were byte-identical to the round-4 scratchpad files. With this round's row, SR-1006
differs and must be re-copied. SR-1007 is unchanged. This round adds no new finding.

FND-019 is fixed. I measured every `render_value_type` call site:

- the direct calls for the operand source and target types (equality/mod.rs:1445-1452);
- the calls inside `render_composite_declaration` (1520, 1530), which is called at 1436;
- the nested recursion (1584, 1590).

All of them run inside `SourceBuilder::item(&mut self, symbol, item: &CompositeEqualityItem,
generated)` (1424). That function holds `item.node_id`, the checked expression node, so "the
item's expression node id" is known at every call site. `render_value_type(value_type:
&ValueType)` (1550) takes no node id, as the bullet says. The Behavior bullet, FR-018-AC-18 and
TC-029 step 10 all name the same id, and the step covers both the operand-type case and the
composite-declaration case.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-019 | fixed | 58f7f8f |
