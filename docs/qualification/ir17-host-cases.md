# IR17 portable host cases

This handoff shares genuine source fixtures and exact host commands with the QSL owner. It does not claim IR17 acceptance, generated native agreement, or Kani qualification. Read the selected dependency cohort from [Cargo.toml](../../Cargo.toml) and [Cargo.lock](../../Cargo.lock); a run against QSL main is a separate owner sweep.

## Commands and execution boundary

[ir17-host-commands.json](ir17-host-commands.json) contains the exact 24 argv arrays selecting 33 declared host functions. Run from the repository root, sequentially, retaining each exit and output. Some filters select multiple functions. The private resolver and raw-reader controls deliberately have different admission boundaries from genuine public generation.

Use an isolated checkout with its own `CARGO_TARGET_DIR`, one Cargo job and one test thread. The raw QSpec control additionally needs `QSPEC_REPO` (or `QSPEC_DIR`) pointing to a real quire-specification Git checkout containing its referenced fixtures. An unset path reports UNAVAILABLE; do not report that as conformance or use a dummy path. These commands remain subject to the host operator's existing resource reservation and memory controls.

The focused, fail-fast reproduction is:

```sh
cargo test --locked --test it composite_equality_generation::tc_029_ac17_emitted_source_has_no_panic_site_fallback_or_index_expression -- --exact --nocapture --test-threads=1
```

This test reaches the unchanged `conv_charge` producer failure. It is **not** an independent sweep of all source cases. Do not stop a QSL owner sweep at that first failure or replace its domain.

The initial 33-function host batch reported 26 passes and seven failures. A genuine retained-dependency fixture omission was subsequently corrected and its focused host control passed. Six shared conversion-related failures remain pending the QSL producer fix; the complete 33-function selection has not been rerun as a green matrix. Later compiler/lint success does not change those host outcomes.

The host commands exclude generated-crate builds, native agreement runners, Kani/backend execution, ignored controls, and aggregate gates. The 33 host functions do not cover the six IR258–263 scalar proof rows or the original IR705 proofs.

## Existing source corpus: all 15 cases

The single source authority is [authentic_native.rs](../../tests/composite_equality_support/authentic_native.rs): `CASES` enumerates the corpus and `source(case)` returns each complete genuine QSL program. Do not duplicate or reconstruct these literals. Original numeric domains, bounds, rounding modes and aliases stay in that file. Native value/charge vectors remain in [cases/](../../tests/composite_equality_support/cases/).

| Case | Source shape or conversion |
|---|---|
| `record` | Point and OtherPoint with Integer members |
| `tuple` | Pair with bounded Integer and NFC text |
| `option` | Optional bounded Integer |
| `collection` | Bounded Sequence of bounded Integer |
| `recursive` | SelfRecord with optional recursive field |
| `nested` | PairOfPoints containing Point records |
| `text` | Bounded NFC text equality |
| `enum` | Ordered Phase, including its complete case table |
| `conv` | Bounded Integer to canonical Integer |
| `conv_charge` | Bounded Integer to Decimal |
| `conv_rat_rat` | Rational to Rational |
| `conv_rat_int` | Rational to Integer |
| `conv_dec_rat` | Decimal to Rational |
| `conv_dec_dec` | Decimal to Decimal |
| `conv_dec_int` | Decimal to Integer |

For an independent QSL-main sweep, the owner runner must iterate **every** `CASES` entry, call `source(case)`, then call the public `compile_admitted` facade and the strict IR reader, matching each `Result` and recording its typed outcome before continuing. Reuse the actual source identity, path, dependencies and limits shown by `compile_source`; do not reuse its fail-fast `expect`. Record source admission, strict IR admission and any later consumer generation separately. No existing single CG host command provides this independent all-case result collection; this handoff adds no new executable or public configuration API.

The separate `mixed` public host case comes from [admitted composite fixture](../../tests/admitted_composite_support/fixture.rs), not a sixteenth entry in `CASES`. It retains authentic tuple/record names, dependencies and whole-package/context binding. [admitted enum fixture](../../tests/admitted_enum_support/fixture.rs) retains authentic ordered/unordered enums, unused cases, and direct/transitive foreign declarations.

## Consumer and control pointers

- [composite_equality_generation.rs](../../tests/it/composite_equality_generation.rs): `genuine_host_case`, its source selection, and host public-generation/claim/refusal checks. Its matrix helpers are fail-fast.
- [admitted_composite_equality.rs](../../tests/it/admitted_composite_equality.rs): genuine host closure/capability ordering and name-distinctness controls. `genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys` builds and executes generated native crates; exclude that function from a host-only sweep.
- [admitted_enum_equality.rs](../../tests/it/admitted_enum_equality.rs): genuine public wrong-context, optional capability, nested traversal, repeatability and text/option/sequence host controls. `genuine_enum_generated_native_abi_agrees_with_owner` and `genuine_bounded_parameter_keeps_other_integer_and_target_unbounded` execute native crates; they are separate qualification.
- [composite_equality_agreement.rs](../../tests/it/composite_equality_agreement.rs): original native corpus runner, including all conversion cases. It is not a host-only test.
- [scalar_playback.rs](../../tests/it/scalar_playback.rs): genuine Integer-source routing, decoding and retained limits controls; not six-family scalar proof coverage.
- [exact scalar package builder](../../tests/exact_scalar_support/package.rs), [agreement vectors](../../tests/exact_scalar_support/agreement_cases.rs), and [Kani scalar fixture](../../tests/it/kani_scalar_native_fixture.rs): synthetic checked-IR or direct-kernel/QSpec fixtures. Their useful assertions do not establish genuine QSL-source proof rows.

Owner-issued composite `key()` is the kernel declaration handle; `node()` is the wire lookup identity. Preserve their distinction, typed Unknown/Foreign/WrongKind refusals, authentic context/package binding, and original no-charge/ordered-charge controls. Do not mint keys, substitute wire IDs for kernel handles, or bypass the strict IR reader.

## IR258–263 and owner-supplied gaps

Each scalar row requires its own genuine QSL-to-IR-to-CG verified Kani case, mutation counterexample, exact encoding, bounded capability behavior, and tool/options evidence. The sources below are reusable input evidence, not completion of those rows.

| Ticket | Genuine source in this corpus | Limit or separate dependency |
|---|---|---|
| IR258 rational | `conv_rat_rat`, `conv_rat_int`, `conv_dec_rat` | Conversion/equality inputs, not rational arithmetic Kani qualification; QSL944 remains relevant. |
| IR259 decimal | `conv_charge`, `conv_dec_rat`, `conv_dec_dec`, `conv_dec_int` | Conversion/equality inputs; retain the producer failures and all original domains. |
| IR260 IEEE float | None identified among these 15 source cases | Synthetic IR/runtime float fixtures are distinct; QSL948 needs a source-profile specification first. |
| IR261 text | `text`, `tuple`, genuine text/option/sequence public host controls | Not full text-operation/admission or Kani qualification. |
| IR262 enum | `enum` and genuine admitted-enum fixtures | Complete case/index/foreign-context host checks; native and proof acceptance remain separate. |
| IR263 quantity | No successful quantity source among these 15 cases | Existing quantity conversion/equality source probes refused the unit-name-as-type route. Synthetic quantity FormMismatch/Unsupported controls remain precise; QSL946 is a grammar/specification dependency. |

The QSL owner reports a public-main sweep including the merged composite facade. The following owner-supplied gaps distinguish delivered defects, missing tables and missing language contracts; they are not new CG specifications:

| Owner ticket | Reported gap | Ownership / boundary |
|---|---|---|
| [QSL944](https://linear.app/agent-ix/issue/QSL-944) | Checked conversion is dropped in cross-family `=` / `!=` lowering | Draco QSL code, in progress. Real `numeric.convert` child and the converted-family equality are needed; renaming only the parent operator is insufficient. Strict IR refusal is correct. |
| [QSL945](https://linear.app/agent-ix/issue/QSL-945) | No union key / VariantIds | Terra owner continuation; missing owner identity table, not permission for CG to construct IDs. |
| [QSL947](https://linear.app/agent-ix/issue/QSL-947) | No unit table / UnitId | Terra owner continuation; missing owner table, not a CG decoder workaround. |
| [QSL946](https://linear.app/agent-ix/issue/QSL-946) | `Quantity<u>` is outside S1/S2 grammar | Draco, specification first; blocks IR263. No invented source syntax or domain substitution. |
| [QSL948](https://linear.app/agent-ix/issue/QSL-948) | No source route admits the IEEE profile | Draco, specification first; blocks IR260. Synthetic float coverage does not resolve that source contract. |

All five are owner-reported urgent Spine work. Ticket state and owner-main contents can advance independently of this handoff; consult the owning tickets for current disposition. No host or compiler result here claims these missing capabilities are delivered.
