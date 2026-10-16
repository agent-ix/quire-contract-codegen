---
id: SR-11801
title: "IR-268 PR #396 gap analysis at head 85c0e97"
type: SpecReview
scope: "agent-ix/quire-contract-codegen#396 @85c0e9747ad5ca0dad5411087c93895a9ee99050; FR-014-AC-2, FR-014-AC-6, FR-014-AC-18, FR-015-AC-96..102 (collection half); tests/it, tests/exact_scalar_support, src/kani/generate/collection.rs, src/oracle/scalar/mod.rs"
relationships: []
---

# SR-11801: IR-268 PR #396 gap analysis at head 85c0e97

## Summary

Ticket: IR-268. This review is planless; plan completion was not assessed. I read the computed
matrix (`quire matrix --scope . --format tsv`). FR-015-AC-96, -97, -99, -100, -101 and -102 and
FR-014-AC-2 and -18 are all tagged. FR-015-AC-98, the graph half owned by IR-805, is untagged, as
expected. For each criterion I checked whether the tagged tests can fail on the defect the
criterion guards. Where it was cheap I ran a mutant (see SR-11800: the quantity-conversion and
nested-quantifier mutants go red).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-014-AC-6 still requires the TC-187 vectors to agree, and QSpec TC-187 is "Quantity and unit conversion". The test traced to AC-6 no longer runs any conversion vector, and AC-6 was not amended | tests/exact_scalar_support/agreement_cases.rs:464-572; spec/oracle/functional/FR-014-exact-scalar-oracles.md:427 |
| FND-002 | low | TC-024 says "derive every `quantity.convert` node at each rounding mode", but all three conversion fixtures use mode `exact` only | tests/exact_scalar_support/package.rs:2669-2700; tests/it/exact_scalar_generation.rs:3489 |
| FND-003 | low | The AC-101 "missing or malformed selected declaration" refusal (`FieldNotAdmitted`) is tested only on the helper with an empty iterator. No negotiation-level test reaches it | src/kani/generate/collection.rs:951-956, 433-438 |

### FND-001

FR-014-AC-6 says: "Executing generated oracles on the QSpec TC-185, TC-186, TC-187, TC-192 and
TC-193 vectors yields outcomes, charges and consumed counters equal to direct runtime execution
and to the QSL value authority." QSpec TC-187 is titled "Quantity and unit conversion". Rows U01,
U04, U07, U08, U09b and U10 are conversions. At main, `tc_024_tc187_quantity_oracles_agree` ran
`2 * (5*5 + 6*6) * 5` conversion agreements. At head it runs none and still carries
`Trace: FR-014-AC-6`. AC-6 now reads as satisfied by a test that skips the conversion half of
TC-187. Fix: amend AC-6 (and TC-024) to exclude the TC-187 conversion rows as refused under
`QuantityConversionUnsupported`, or assert that refusal for them. Spec wording is for the spec
reviewer. The gap here is that a passing test claims a criterion it no longer exercises.

### FND-002

TC-024 step text: "derive every `quantity.convert` node at each rounding mode and check it is
refused as `QuantityConversionUnsupported`". The three fixtures, 1086, 1087 and 2028, are built
in one loop with `mode_kv("rounding", "exact")`. The guard is identity-only, so the verdict would
be the same for other modes. Still, the test does not do what its TC says. Either the TC drops
"at each rounding mode" (if IR pins `exact` for a quantity-typed conversion), or the fixtures add
the other modes.

### FND-003

`find_admitted_declaration` is covered only by `unresolved_projection_declaration_is_field_not_admitted`,
which calls it on `std::iter::empty()`. Two paths have no test: `model_object_fields(..)` failing,
and `fields.field(&field_name)` missing (collection.rs:433-438). The stronger SR-11681 FND-003
request, a negotiation-level `FieldNotAdmitted` record, is still open. This is low because a
QSL-admitted package resolves these by construction.

## Clean units

- FR-014-AC-2 and FR-014-AC-18 (conversion refusal):
  `tc_024_quantity_conversion_is_refused_with_its_named_capability` and
  `tc_033_a_quantity_conversion_is_unsupported_with_its_named_capability_and_no_harness` assert the
  exact variant, identity and capability, and that no harness is emitted. `QUANTITY_CONVERSIONS`
  is a non-empty `[u32; 3]`, so the loop is not vacuous. The mutant run shows both tests go red.
- FR-015-AC-96: symbolic length and elements, both endpoints inclusive, and no `max_items` input.
  Assertions at tests/it/kani_obligations_symbolic_domains.rs:975 and 1346 pin the bounds.
- FR-015-AC-97: there are requires-bound and unsupported controls for an ordered set, an
  unsupported predicate, a missing upper bound, Reference elements and String/Uuid/Timestamp
  elements.
- FR-015-AC-99: the cover comes after the assert. The native `qsl_replay::replay` control runs in
  the ordinary suite. The real-Kani control is `#[ignore]` and runs in the Kani lane through the
  Makefile `KANI_FILTERS`. It was not run here.
- FR-015-AC-100: regeneration is byte-identical, and the subject and unwind change the identity
  (FND-004 of SR-11800 aside).
- FR-015-AC-101: refusals for unwind, memory ceiling and nested negation/disjunction. The nested
  test is shown discriminating by a mutant.
- FR-015-AC-102: one disposition per item, and a refused neighbour does not reject a valid item.

## Verdict

Coverage of the collection half is substantive, and the new refusal tests have strong oracles.
FND-001 is a medium trace and coverage gap: AC-6 claims the TC-187 vectors that the PR stopped
running. It should be fixed by a spec amendment or a refusal assertion, in this PR or in a
tracked follow-up.

## Dispositions

Disposition pass 1 at fix-round commit 7e16de1. The checks and the mutant run are as recorded in
SR-11800's Dispositions. The mutant removed the `check_item` guard:
`tc_024_caller_declared_quantity_conversion_is_refused_and_generates_nothing`,
`tc_024_tc187_conversion_rows_are_refused_with_the_named_capability` and
`tc_025_a_quantity_conversion_claim_is_unsupported_with_its_named_capability_and_no_harness` went red.
The derivation-path tests (`tc_033_...`, `a_selected_quantity_conversion_settles_...`) stayed green
because the derivation guard still holds. So each of the two guards has at least one test that
fails without it. A mutant import of `crate::oracle::claim` into collection.rs failed
`l_2_every_import_follows_the_dependency_direction` with the allowlist message, so the AD-004
allowlist is enforced.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-014-AC-6 now excludes the TC-187 conversion rows and names FR-014-AC-2 as their owner. `assert_rows_agreed` fails any AC-6 row with 0 agreements, and every agreement test calls it. |
| FND-002 | fixed | TC-024 step text now reads "derive every `quantity.convert` node of the corpus (each carries the `exact` mode its unit type pins)", which matches the fixtures. |
| FND-003 | fixed | `fn declared_field` (src/kani/generate/collection.rs:339) is module-private: not pub, not pub(crate), not re-exported. It is used at :449 in place of the old `map_err`/`ok_or`, and it maps both an `Err` listing and a missing field to `FieldNotAdmitted`, exactly as before, so production behaviour is unchanged. The lib test `a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted` drives both error branches through the combinator the production call uses. Every supported integration case exercises the Ok path. That is adequate for branches admitted packages cannot reach. |
| FND-004 | fixed | Round 2 (45f72ee). The label-loop test is deleted. AC-2 now says the tests exercise the three corpus conversion nodes, not individual vectors. AC-6 excludes exactly U01, U04, U07, U08, U09b, U10, U15-U18, U22, U25, U26 and U29. That matches every "Convert" row of QSpec TC-187. U09 (ill_typed dimension key) is back in AC-6 scope. tests.md claims no per-row count. |
| FND-005 | fixed | Round 2. `tc_025_a_quantity_conversion_claim_is_unsupported_...` destructures `KaniObligationOutcome::Emitted` and asserts `harnesses`, `scalar_harnesses`, `state_frame_harnesses` and `collection_harnesses` are all empty. A control over node 1001 yields one scalar harness, so the absence is measured. |
| FND-006 | fixed | Round 2. FR-014 Behavior and AC-2 now state that the function-application and composite-equality generators are exempt: they refuse a conversion as `FormMismatch { found: "conversion" }` before any artifact. Tests: the lib test `a_quantity_conversion_body_is_a_form_mismatch_for_every_body_kind`, plus integration tests for both generators. The equality test asserts the exact `FormMismatch`. A probe mutant that panics in `classify_body_shape` for a non-fixture conversion node turned `tc_024_function_generator_refuses_...` red, so the integration test does reach the form check on the production path. |
| FND-007 | fixed | Round 3 (2d3cca8, 1cce284). `generate_function_schedules(_with_limits)` refuse a body that reaches a conversion as `ScheduleRefusal::QuantityConversionUnsupported { body, capability }` on `refused_functions`. The check runs before the parameter and result-kind lookups, so neither result type gives an `Err(MissingNode)` any more. `validate_schedule_item` refuses a call that reaches one in the same way. Both tests assert the exact variant, node and capability. Mutants disabling either check turn their test red (logs/mutation.log, exit 101). FR-014 Behavior and AC-2 now list the schedules as entry points. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The per-row TC-187 refusal test only labels rows. Every "row" re-asserts the same three corpus nodes, and U09, which is not a conversion row, is in the list | tests/it/exact_scalar_generation.rs:3514-3558; spec/oracle/functional/FR-014-exact-scalar-oracles.md:433 |
| FND-005 | low | The Kani negotiation entry-point test asserts only the V1 `harnesses` vector is empty, not `scalar_harnesses`, `state_frame_harnesses` or `collection_harnesses`. S2's empty-artifact-set check is not met there | tests/it/kani_obligations.rs:663-665, 1190 |
| FND-006 | low | The FR-014 entry-point list for the conversion refusal omits `generate_exact_function_oracles` and the composite-equality generator, and gives no stated reason they are exempt | spec/oracle/functional/FR-014-exact-scalar-oracles.md:273 |

### FND-004

`CONVERSION_ROWS` is an array of 11 label strings. For each label the loop runs the same
`derive_one` and `generate_exact_scalar_oracles` over the same `QUANTITY_CONVERSIONS` nodes (1086,
1087, 2028). None of those nodes carries a row's vector, unit or target. Adding, removing or
renaming a row changes only the final count assertion, so the test is no per-row evidence: it is
the three-node refusal repeated 11 times. QSpec TC-187 U09 ("Use a base-dimension name/shape equal
to `L` but a different I04 semantic-node key", expected `ill_typed`) is not a conversion. Still,
the list includes it, and AC-6 excludes it ("U07 to U10"). The test cannot catch this
misclassification because it never consults the row. Fix one of two ways: tie each row to a
fixture node built from that row's operands and target, or state in AC-2 that the conversion rows
share one refusal and assert it once. Either way, take U09 out of the conversion list and keep it
in AC-6.

### FND-005

`scalar_records` destructures the outcome with `emitted(..)`, which returns only `harnesses` (the
V1 `KaniObligationHarness` list), and asserts that is empty. A `ScalarClaim` item that rendered
would land in `scalar_harnesses`, which the test never reads. The `Unsupported` record assertion
would still catch a supported disposition, so no defect ships. However, the "no harness" half of
this test checks a vector the item can never populate, contrary to the S2 steer.

### FND-006

The S1 steer requires a stated reason for any entry point that is exempt. FR-014's Behavior
statement lists derive, generate/plan, `generate_routed`, `negotiate_kani_obligations` and
capability settlement. `generate_exact_function_oracles` accepts only `integer.add/sub/mul`
bodies (src/oracle/function/mod.rs:1687-1691), and composite equality refuses quantity values
(src/oracle/equality/mod.rs:1945-1946). Both refuse a conversion, but under their own reasons and
without the named capability, and AC-2 does not say why they are exempt. This is low with medium
confidence: no emission is possible, so this is a statement gap and not a behaviour gap.

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | `generate_function_schedules` and `generate_function_schedules_with_limits` are public and take function bodies, but they are neither in FR-014's quantity-conversion entry-point list nor stated as exempt, and no test drives a conversion body through them | src/oracle/function/mod.rs:282-330, 386-420; src/oracle/function/schedule.rs:61-72 |

### FND-007

Planner steer S1 says every entry point that can receive `quire.op.quantity.convert` must either
refuse it with `QUANTITY_CONVERSION_CAPABILITY` or be shown exempt by an earlier typed refusal.
Either way the case must be stated in AC-2 and tested. The schedule generator is public
(re-exported at src/lib.rs:84-85) and accepts `FunctionScheduleDeclaration`s naming any admitted
function body.

By reading the code, a conversion body is refused before any artifact, but with a generic reason:
- Its unit result type is not in `type_kinds`, which holds only boolean and integer scalar types
  (mod.rs:318-329), so `result_kind` fails with `ScheduleRefusal::MissingNode`.
- Otherwise `Operation::from_identity` refuses any identity outside its seven-entry set as
  `UnsupportedOperation` (schedule.rs:61-72).
- The refusal is a whole-request `Err`, not a per-item disposition.

Nothing tests this. FR-014's Behavior statement and AC-2 do not name the schedule generator. This
is low with medium confidence: there is no emission, and the gap is in the statement and the
test. The exemption was established by reading the code, not by executing it.

I swept the other generators. Each has a closed operation whitelist that a conversion identity
cannot match: Kani frame comparisons (frame.rs:1271-1276), collection quantifier predicates
(collection.rs `resolve_predicate`), replay composite equality (`quire.op.structural.eq/ne`,
composite_builder.rs:270-271) and the Kani scalar renderer (`ScalarOperation::of`, integer
arithmetic only). All of them refuse before rendering. None is a V2 per-node entry point for an
arbitrary application, so I raise no further finding. A one-line AC-2 note covering
"closed-whitelist families" would make S1 complete.

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | No test pins `generate_exact_function_oracles` refusing a conversion reached from a declared function body with `QuantityConversionUnsupported`, although AC-2 claims it. With the new `admitted_binary_body` guard disabled, the body test stays green | tests/it/exact_scalar_generation.rs:3561-3625, 3737; src/oracle/function/mod.rs:1971-1988 |

### FND-008

AC-2 now says: "The function-application generator and the function schedules refuse a
conversion reached from a body, a call or a call argument with `QuantityConversionUnsupported`".
Coverage of that clause is uneven across the three paths:
- Call argument: `tc_024_function_generator_refuses_a_conversion_reached_as_an_argument_or_a_body`
  asserts the exact refusal. Despite its name and doc ("and so is the declared function over a
  conversion body"), it only covers the argument path.
- Body: the kept `tc_024_function_generator_refuses_a_quantity_conversion_body_before_any_artifact`
  asserts only that every claim is some `Refused { .. }` (which also holds when the claim map is
  empty) and that the location map is empty.
- Its doc still says the typed refusal is "`FormMismatch` with the form `conversion`, ... asserted
  on the classifier in the generator's unit tests". That unit test was deleted in 1cce284, and
  the refusal is no longer `FormMismatch`.

Mutation run (logs/mutation.log, exit 101, reverted): I disabled the conversion check in
`admitted_binary_body` (mod.rs:1973). The argument test went red, but the body test stayed green.
So the named-capability refusal on the body path has no test that fails without it.

No emission ships: with the guard off, the body falls through to the classifier and is still
refused. But an AC clause is unbacked, and the test name and doc misstate what is covered.

Fix: in the body test, assert
`ExactFunctionRefusal::QuantityConversionUnsupported { node_id, capability }` on a claim whose
item calls the conversion-bodied function, and assert the claim map is non-empty. Also update
both docs.

Also checked this round, with no finding:
- `first_conversion_reached` terminates. It keeps a `seen` set, visits each node at most once
  over the admitted graph (bounded by `CheckedPackageReadLimits`), and silently skips ids that
  are not present.
- The `admitted_binary_body` scan iterates the already-computed reached closure.
- Making `QUANTITY_CONVERT_IDENTITY` `pub(crate)` and importing it from `oracle::function` adds
  an intra-`oracle` edge. That matches the existing `oracle::equality -> oracle::scalar` import,
  and AD-004 L-2 (layout tests) passes.
- Deleting the classifier lib test lost no non-conversion coverage. FR-021-AC-21 Negate is still
  tested at tests/it/exact_function_generation.rs:2634, and a non-binary `FormMismatch` at
  :1851.
- I found no other entry point that can receive a conversion node. Replay function harnesses take
  witness values, not graph nodes. The routed path is the derivation guard. The Kani
  frame, collection, scalar and replay composite families are closed whitelists, as AC-2 now
  states.

