---
id: "SR-653"
title: "CG PR 212 code review: codegen corpus for current Contract IR operator typing and lock bump"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@58dd43d829f4ee742c87775455e73dc98adde05e; src/composite_equality.rs, tests/composite_equality_support/*, tests/exact_scalar_support/*, tests/it/{composite_equality_generation,exact_scalar_generation,exact_scalar_agreement,exact_function_generation}.rs, Cargo.lock"
---

# SR-653: CG PR 212 code review

## Summary

Ticket: IR-480. PR: agent-ix/quire-contract-codegen#212, head 58dd43d, branch
`fix/ir-480-corpus-operator-typing`. It is stacked on #211 (`fix/ir-477-drop-artifact-digest-calls`).
This review covers the PR's own diff (`origin/fix/ir-477-drop-artifact-digest-calls...HEAD`, 13
files, +664/-374) and the net `origin/main...HEAD` diff for `Cargo.lock`. It is code-review with
the rust-review lane folded in. The QSL emission rules were read from
quire-spec-language@a28a5578 (`qsl-semantics/src/check/lowering.rs`). The IR rules were read from
quire-contract-ir@0a889f9 (`checked_package/v2/operations.rs`).

## Method

- I read the full diff. I compared the test names, vectors and oracles before and after. Dropped:
  the six `admit_*` oracles and their 288 agreement vectors, the `E_SELF` golden item, its
  agreement loop (2 vectors) and its `agreement_names` entry, and the TextAdmission selector
  assertion in `tc_024_each_overloaded_identity_derives_by_its_own_selector`. Restored: the four
  ordered-enum nodes, which now use reference operands; the literal workaround is gone. Added:
  AC-15, the IR-refusal pins for the recursive type, the direct reference and the six text
  admissions, and `R_WITH_REF`.
- I mutated `operand_type_id` four ways in a throwaway copy, then restored it:
  - read `result_type`: killed by AC-15, AC-3 and agreement
  - drop the convert arm: killed by the same three
  - read through any application: SURVIVES
  - single step instead of the loop: SURVIVES
- I probed the read-through with descriptors that disagree with the body's conversion. These were
  scratch tests, since removed.
- Gate at 58dd43d, no patch config, private scratchpad TRUSTED_HOME: `make ci` exits 0.
  - lib: 101/101
  - `it`: 242 passed, 0 failed, 8 ignored (both the msrv and test runs)
  - `deny`: advisories, bans, licenses and sources ok; `check_one_copy.awk` ok
  - `make kani`: 8 passed, 0 failed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The corpus and the new convert read-through do not match QSL emission. QSL a28a5578 keys every operand, literal or `convert`, as its own node and puts a `reference` term to it in the equality's `arguments` (`typed_application`/`value_node` return `SemanticTerm::reference`). It builds no text, decimal or composite literal (`UnbuiltLiteral`). `operand_type_id` reads only an inline `literal` or an inline `convert` application and returns `None` for a `reference`. So every composite-equality item over a real QSL package refuses `OperandTypeMismatch{found: None}`. The read-through serves only the hand-built inline shape. The literal-only read predates this PR; the convert extension and the claim that the corpus is "what real QSL emits" are new | src/composite_equality.rs:906-917; tests/composite_equality_support/package.rs:269-271, 313-332 |
| FND-002 | medium | The hand-built `numeric.convert` member is `{"kind":"type_argument"}` with no `declaration`. QSL always emits `{"kind":"type_argument","declaration":<target type node>}` (lowering.rs `type_argument`). IR happens to admit the short form, so the corpus diverges from QSL silently | tests/composite_equality_support/package.rs:320 |
| FND-003 | medium | The E_SELF drop rests on a false premise. `R_SELF` is `record{next: Option<R_SELF>}` and reaches no text. QSL's leaf walk does not enter a composite that reaches no text (lowering.rs rules 3/4, around 941-948), so it emits `leaves: []`, not a `recursion:<n>` leaf. `[]` is exactly what the corpus builds. IR 0a889f9 `check_leaf_count` refuses any compared type that reaches itself (`LeafWalkEnd::Cycle` -> ill_typed/operator-ineligible), whether or not text is reachable. So IR refuses QSL's real emission. The doc comments and the matrix say "QSL emits a recursion leaf" and "undecidable", which misstates the evidence the open research run is deciding on. Coverage lost: generation, golden item and generated-crate agreement for any recursive composite | tests/composite_equality_support/package.rs:619-628; tests/it/composite_equality_generation.rs:549-556; spec/test-matrix.md:94 |
| FND-004 | low | Two `operand_type_id` mutants survive every test: dropping the `operator == "convert"` guard (any application is read through) and replacing the loop with a single step. A nested convert reads the innermost literal. The clause instead says "the type of the operand it converts", which for a nested convert is the inner application's `result_type`. No test covers either | src/composite_equality.rs:906-917 |
| FND-005 | low | The read-through discards the conversion itself. The body's `convert` `result_type` is never compared with the descriptor's `conversion_target`, and a `typed(source)` descriptor passes the operand check over a converting body. Probes on E_CONV, E_CONV_DEC_DEC, E_CONV_DEC_RAT, E_CONV_RAT_RAT and E_CONV_CHARGE show that runtime `check_equality` refuses every such combination (TypeMismatch), so this corpus cannot exploit it. FR-018's "disagrees with its descriptor's operand types" is still only half enforced | src/composite_equality.rs:921-937 |
| FND-006 | low | Two corpus-size floors add the constant `refused_corpus().len()` (6) to the count they bound: `generated + refused_corpus().len() > 60` and `corpus.len() + refused_corpus().len() > 60`. The message reads "the corpus generates every family", but the six are not generated. No coverage is lost, since the exact `corpus().len() == calls.len()` check at :396 stands, but the floor overstates what it measures. Lower it to the generated count's real floor or drop it | tests/it/exact_scalar_generation.rs:176-179, 1926-1929 |
| FND-007 | low | `assert_eq!(claim.function, "add_fn")` is now tautological, because the `find_map` above it already selects on `function == "add_fn"`. The agreement test name `..._collection_and_recursive_oracles_agree` still says "recursive" although no recursive vector remains | tests/it/exact_function_generation.rs:143-157; tests/composite_equality_support/agreement_cases.rs:54 |
| FND-008 | low | The WRONG_OPERAND, 3002 and LITERAL_QUANTITY fixtures use literals typed by one node but carrying another `value_kind` (integer-typed decimal or text; unit-typed rational). They rely on IR not cross-checking `literal.type` against `value_kind`, and on shapes QSL never emits (no decimal, text or quantity literal). They are fine as negative fixtures, but their comments do not say they are non-QSL shapes | tests/exact_scalar_support/package.rs:2830-2844, 2586-2596; tests/it/exact_scalar_generation.rs:1329-1337 |

## Verdict

Sound, and checked:

- The lock bump. Only the listed crates moved: quire-contract-ir and quire-contract-model go from
  54f9a48 to 0a889f9, and qsl-attrs, qsl-cst, qsl-eval, qsl-forms, qsl-foundation, qsl-package,
  qsl-replay, qsl-semantics and quire-exact go from 7d0497b to a28a5578. Nothing else in
  `Cargo.lock` changed. There is one `quire-contract-ir` package entry, there is no
  `.cargo/config.toml`, and deny plus the one-copy check pass.
- The three IR-refusal pins each assert the exact code, cause, JSON pointer and locus. Each panics
  with "IR now admits ..." if IR starts admitting the case. None is `#[ignore]`d, and the six text
  admissions are kept (not deleted) in `refused_corpus()`.
- AC-15's test kills the result-type mutant that AC-15's counterexample names.
- The text-profile law and mode on `text.eq` match QSL. So does the single `position:1` structural
  leaf for `Tuple<Int, Text>` (QSL's leaf path, laws and mode), with no leaves for types that reach
  no text.
- `git merge-tree` against #210 (IR-277), #209 (IR-459) and `fix/ir-464-nonvacuity-cover` is
  clean. Their only shared file is `spec/test-matrix.md`, in different hunks, and none touches
  `Cargo.lock` or the corpus.

FND-001 is the root cause the planner named, and this PR deepens it in spec and code. FND-002 and
FND-003 are wording or shape fixes that belong in this PR. FND-001 can be deferred to a ticket only
if FR-018's clause stops claiming QSL emits this shape (see SR-655 FND-001).

## Dispositions

Round 1, reviewed at bdbad7af90728ce194bb685dae2cb247923c0582 (lock commit 58dd43d unchanged, #211 branch
unmoved at 9ddf5a9, main 113b624). Gate run by the reviewer at bdbad7a, no patch config, with a private
TRUSTED_HOME:

- `make ci` exits 0: lib 101/101, `it` 244 passed, 0 failed, 8 ignored (msrv and test runs).
- `deny` passes, as does the one-copy check.
- `make kani`: 8 passed, 0 failed.

`operand_type_id` mutants:

- killed: read through any application (by the non-conversion test), single step (by the nested
  test), read the conversion's own `semantic_type`, iteration bound 1, and no read-through.
- surviving, both equivalent: dropping the `expression`-tag check (only expression nodes carry an
  application body) and `last()` in place of `first()` (a convert has one argument).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed bdbad7a | `operand_type_id` now resolves a `reference` operand through the graph. A convert `expression` node is read through to the first non-conversion node, any other node reads as its `semantic_type`, and inline terms read as no type. The loop is bounded by `graph.len()+1` and every step uses `?`, so a missing node, a cycle or a non-convert node cannot loop or panic. The corpus now keys operands as QSL a28a5578 does: `value`/`parameter` nodes with name/level bindings and the `expression` occurrence role (lowering.rs `parameter`), and the convert as its own `expression`/`conversion` node with operator `convert` and a `type_argument` member carrying `declaration`, referenced from the equality, plus the FR-322 dependency join. IR 0a889f9 admits the corpus |
| FND-002 | fixed bdbad7a | The convert member is now `{"kind":"type_argument","declaration":<target type node>}`, matching QSL's `Member::TypeArgument` |
| FND-003 | fixed bdbad7a | The comments and matrix now say QSL emits `leaves: []` for `R_SELF` and that IR refuses any compared type that reaches itself (the reference reader rule), pending STD-129. The pin is renamed `tc_029_a_cyclic_compared_type_is_refused_by_ir_today`, and the coverage loss is recorded on a split FR-018-AC-2 row |
| FND-004 | fixed bdbad7a | Both surviving mutants (guard removal, single step) are now killed by `tc_029_ac15_a_non_conversion_application_operand_is_read_as_its_own_type` and `tc_029_ac15_nested_conversions_are_read_through_to_the_innermost_operand` |
| FND-005 | accepted-no-change | FR-018's clause now states deliberately that the conversion's `result_type` is not compared, because `check_equality`'s verdict on the descriptor governs. Round-0 probes showed the runtime refuses every mismatched conversion in the corpus |
| FND-006 | fixed bdbad7a | The refused constant is removed from both sums, and the floor is `>= 59`. It is tight: raising either floor to `>= 60` fails both tests, so exactly 59 expressions are admitted (65 less the six text admissions; the ordered-enum codes are back). The lost six are recorded under FR-014-AC-2, and the exact `corpus().len() == calls.len()` check stands |
| FND-007 | fixed bdbad7a | The tautological `add_fn` assert is removed, and the agreement test is renamed `tc_029_ac2_and_ac9_record_tuple_option_and_collection_oracles_agree` |
| FND-008 | fixed bdbad7a | The LITERAL_QUANTITY, WRONG_OPERAND and 3002 fixtures each carry a comment saying they are deliberate negative fixtures in a shape QSL never emits |
