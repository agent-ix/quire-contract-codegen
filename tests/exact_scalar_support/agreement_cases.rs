//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime
//! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193
//! vectors, in outcome, every admitted charge, every consumed counter, and
//! the outcome of denying each admitted charge.
//!
//! This file is not part of the `it` binary. It is the `agreement` integration
//! test of a scratch crate that `tests/it/exact_scalar_agreement.rs` writes at
//! test time from the generator's current output for the corpus request, so the
//! oracles executed here are always the ones the generator emits now. Each
//! oracle has a fixed operator and type, so a vector is an operand tuple and a
//! limit tuple taken from the TC's tabled or generated vectors.
//!
//! IR-254: every vector here runs `agree2!` (direct runtime vs. generated
//! oracle). A third, QSL value-authority leg (`agree3!`) used to run
//! alongside it; `agreement.rs`'s module doc records why it was deleted
//! rather than ported to the QSL revision this bumps to.

#[path = "agreement.rs"]
#[macro_use]
mod support;

/// The generated crate under test. `names.rs`, written beside the scratch crate's manifest at test
/// time, re-exports each generated oracle under the operation name these cases call it by.
mod generated {
    pub use quire_exact_scalar_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use generated::{
    binary32_add, binary32_numeric_equal, binary32_subtract, binary64_bit_identical,
    binary64_divide, binary64_multiply, binary64_total_order, decimal_add, decimal_at_most,
    decimal_divide, decimal_multiply, decimal_negate, decimal_round, decimal_subtract,
    divide_bounded, divide_euclidean, divide_floor, divide_truncating, enum_at_least, enum_at_most,
    enum_equal, enum_greater, enum_less, enum_not_equal, integer_add, integer_at_least,
    integer_at_most, integer_divide, integer_less, integer_multiply, integer_negate,
    integer_subtract, modulo_bounded, narrow_to_binary32, quantity_add, quantity_at_least,
    quantity_at_most, quantity_convert, quantity_divide, quantity_equal, quantity_greater,
    quantity_less, quantity_multiply, quantity_not_equal, quantity_power, quantity_subtract,
    rational_add, rational_divide, rational_greater, rational_multiply, rational_negate,
    rational_subtract, text_at_least, text_at_most, text_equal, text_greater, text_less,
    text_not_equal, OracleStop,
};
use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    EnumValue, IllTyped, Integer, Meter, Outcome, Quantity, ScalarLimits, Text,
};

/// Generated oracles in `ComparisonOperator::ALL` order.
type Compare<T> = fn(&T, &T, &mut Meter) -> Result<Outcome<bool>, OracleStop>;
const TEXT_COMPARISONS: [Compare<Text>; 6] = [
    text_equal,
    text_not_equal,
    text_less,
    text_at_most,
    text_greater,
    text_at_least,
];
const ENUM_COMPARISONS: [Compare<EnumValue>; 6] = [
    enum_equal,
    enum_not_equal,
    enum_less,
    enum_at_most,
    enum_greater,
    enum_at_least,
];
const QUANTITY_COMPARISONS: [Compare<Quantity>; 6] = [
    quantity_equal,
    quantity_not_equal,
    quantity_less,
    quantity_at_most,
    quantity_greater,
    quantity_at_least,
];

/// An oracle over an infallible runtime operator. Its generated constants are
/// valid, so a stop is a generator defect.
fn done<T>(result: Result<Outcome<T>, OracleStop>) -> Outcome<T> {
    result.unwrap_or_else(|stop| panic!("generated oracle stopped: {stop:?}"))
}

/// An oracle over a runtime operator that can refuse its operands as ill-typed.
fn typed<T>(result: Result<Outcome<T>, OracleStop>) -> Result<Outcome<T>, IllTyped> {
    result.map_err(|stop| match stop {
        OracleStop::IllTyped(ill) => ill,
        other => panic!("generated oracle stopped: {other:?}"),
    })
}

/// Fails when any named row of an agreement test compared nothing, so a row whose loop never runs
/// cannot pass as agreement.
fn assert_rows_agreed(rows: &[(&str, usize)]) {
    for (row, agreements) in rows {
        assert!(
            *agreements > 0,
            "row {row} yields 0 agreements; an agreement row that compares nothing is not agreement"
        );
    }
}

// Deliberately untraced: this exists as `render_scalar`'s (`src/kani/generate/scalar.rs`) F1
// mutation-testing vehicle, not to satisfy a numbered acceptance criterion. It runs the real
// generated oracle directly -- no Kani/CBMC involved -- to prove, by actual execution, that
// `quire.op.integer.add` over `[-1000,1000]` refuses an out-of-domain result rather than
// completing it: exactly the fact `render_scalar`'s rendered `sound` assertion now checks. The
// accompanying generator mutation (`integer_arithmetic_bound` in `src/oracle/scalar/mod.rs`, forcing
// its `Bounded` arm to also emit `None`) is not committed; it is a one-time proof, captured in
// this change's own review transcript, that removing the domain check makes this same call
// return `Completed(2000)` instead -- the input this test already knows must not complete.
#[test]
fn integer_add_refuses_a_result_leaving_its_declared_domain() {
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    });
    let outcome = integer_add(
        &Integer::from(1000_i64),
        &Integer::from(1000_i64),
        &mut meter,
    )
    .expect("the generated oracle never stops for well-typed operands");
    assert!(
        !matches!(outcome, Outcome::Completed(_)),
        "1000 + 1000 leaves [-1000,1000] and must never be reported Completed: {outcome:?}"
    );
    assert!(
        matches!(outcome, Outcome::Refused(_)),
        "an out-of-domain integer arithmetic result is Refused, not Undefined or Incomplete: {outcome:?}"
    );
}

#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc192_division_and_modulus_oracles_agree() {
    const DIV_08: [u64; 10] = [3, 0, 0, 0, 0, 0, 0, 2, 4, 2];
    const DIV_10: [u64; 10] = [3, 0, 0, 0, 0, 0, 0, 2, 4, 1];
    let with = |mut base: [u64; 10], index: usize, value: u64| {
        base[index] = value;
        base
    };
    let mut vectors = 0_usize;
    for a in -20_i128..=20 {
        for b in -20_i128..=20 {
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); },
                direct: |m| (
                    divide(DivisionProfile::Truncating, &a, &b, &wide(), m),
                    divide(DivisionProfile::Floor, &a, &b, &wide(), m),
                    divide(DivisionProfile::Euclidean, &a, &b, &wide(), m),
                ),
                generated: |m| (
                    done(divide_truncating(&a, &b, m)),
                    done(divide_floor(&a, &b, m)),
                    done(divide_euclidean(&a, &b, m)),
                ),
            };
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(-5), int(5)).unwrap()); },
                direct: |m| divide(DivisionProfile::Truncating, &a, &b, &bounded, m),
                generated: |m| done(divide_bounded(&a, &b, m)),
            };
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(-5), int(5)).unwrap()); },
                direct: |m| modulo(&a, &b, &bounded, m),
                generated: |m| done(modulo_bounded(&a, &b, m)),
            };
            vectors += 5;
        }
    }
    assert_rows_agreed(&[("TC-192 generated vectors", vectors)]);
    assert_eq!(vectors, 5 * 41 * 41);

    // DIV-08 exact tuple, short work, short result; DIV-11 and DIV-13.
    let mut named = 0_usize;
    for (a, b, tuple) in [
        (7, 3, DIV_08),
        (7, 3, with(DIV_08, 8, 3)),
        (7, 3, with(DIV_08, 9, 1)),
        (7, 3, with(with(DIV_08, 0, 2), 8, 0)),
        (7, 0, with(DIV_08, 8, 1)),
        (7, 0, with(DIV_08, 8, 0)),
    ] {
        agree2! {
            limits: limits(tuple),
            setup: { let (a, b) = (int(a), int(b)); },
            direct: |m| divide(DivisionProfile::Truncating, &a, &b, &wide(), m),
            generated: |m| done(divide_truncating(&a, &b, m)),
        };
        named += 1;
    }
    // DIV-10, DIV-11 and DIV-12 (the modulus refused before retention).
    for (a, b, tuple) in [
        (-7, 3, DIV_10),
        (7, 0, with(DIV_10, 8, 1)),
        (7, 0, with(DIV_10, 8, 0)),
        (-7, 3, with(DIV_10, 9, 0)),
        (-9, 1, with(with(DIV_10, 9, 0), 8, 2)),
    ] {
        agree2! {
            limits: limits(tuple),
            setup: { let (a, b) = (int(a), int(b)); let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(-5), int(5)).unwrap()); },
            direct: |m| modulo(&a, &b, &bounded, m),
            generated: |m| done(modulo_bounded(&a, &b, m)),
        };
        named += 1;
    }
    assert_rows_agreed(&[("TC-192 tabled tuples", named)]);
    assert_eq!(named, 11);
    println!("TC-192 agreement: {vectors} generated vectors, {named} tabled tuples");
}

const COEFFICIENTS: [i64; 10] = [-25, -10, -7, -5, -1, 0, 1, 3, 8, 10];

#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc185_decimal_oracles_agree() {
    const D09: [u64; 10] = [7, 3, 2, 0, 0, 0, 0, 2, 5, 1];
    let operands = COEFFICIENTS
        .iter()
        .flat_map(|c| (0..3).map(move |s| (*c, s)))
        .collect::<Vec<_>>();
    let mut vectors = 0_usize;
    for (ca, sa) in &operands {
        agree2! {
            limits: UNLIMITED,
            setup: { let a = dec(*ca, *sa); let target = decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway); },
            direct: |m| evaluate_decimal(DecimalOperation::Round(&a), &target, m),
            generated: |m| done(decimal_round(&a, m)),
        };
        agree2! {
            limits: UNLIMITED,
            setup: { let a = dec(*ca, *sa); let target = decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven); },
            direct: |m| evaluate_decimal(DecimalOperation::Negate(&a), &target, m),
            generated: |m| done(decimal_negate(&a, m)),
        };
        vectors += 2;
        for (cb, sb) in &operands {
            agree2! {
                limits: UNLIMITED,
                setup: {
                    let (a, b) = (dec(*ca, *sa), dec(*cb, *sb));
                    let add = decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven);
                    let divide = decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero);
                },
                direct: |m| (
                    evaluate_decimal(DecimalOperation::Add(&a, &b), &add, m),
                    evaluate_decimal(DecimalOperation::Subtract(&a, &b), &add, m),
                    evaluate_decimal(DecimalOperation::Multiply(&a, &b), &add, m),
                    evaluate_decimal(DecimalOperation::Divide(&a, &b), &divide, m),
                ),
                generated: |m| (
                    done(decimal_add(&a, &b, m)),
                    done(decimal_subtract(&a, &b, m)),
                    done(decimal_multiply(&a, &b, m)),
                    done(decimal_divide(&a, &b, m)),
                ),
            };
            vectors += 4;
        }
    }
    assert_rows_agreed(&[("TC-185 generated vectors", vectors)]);
    assert_eq!(vectors, 2 * 30 + 4 * 30 * 30);

    // D03 (both signed halves), D05, D06, D09 exact and short, D15.
    let mut named = 0_usize;
    for coefficient in [25_i64, -25] {
        agree2! {
            limits: UNLIMITED,
            setup: { let a = dec(coefficient, 1); let target = decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway); },
            direct: |m| evaluate_decimal(DecimalOperation::Round(&a), &target, m),
            generated: |m| done(decimal_round(&a, m)),
        };
        named += 1;
    }
    let with_work = |work: u64| {
        let mut tuple = D09;
        tuple[8] = work;
        tuple
    };
    for ((ca, sa), (cb, sb), tuple) in [
        ((1, 0), (3, 0), [u64::MAX; 10]),
        ((1, 0), (0, 0), [u64::MAX; 10]),
        ((1, 0), (0, 3), [u64::MAX; 10]),
        ((1, 0), (3, 0), D09),
        ((1, 0), (3, 0), with_work(4)),
        ((1, 0), (0, 3), with_work(1)),
        ((1, 0), (0, 3), with_work(0)),
    ] {
        agree2! {
            limits: limits(tuple),
            setup: { let (a, b) = (dec(ca, sa), dec(cb, sb)); let target = decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero); },
            direct: |m| evaluate_decimal(DecimalOperation::Divide(&a, &b), &target, m),
            generated: |m| done(decimal_divide(&a, &b, m)),
        };
        named += 1;
    }
    assert_rows_agreed(&[("TC-185 tabled tuples", named)]);
    assert_eq!(named, 9);

    // Decimal ordering: runtime vs. generated oracle only (`agree2!`); see
    // FR-014-AC-47 for why this file has no QSL authority leg.
    let mut ordering = 0_usize;
    for (ca, sa) in &operands {
        for (cb, sb) in &operands {
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (dec(*ca, *sa), dec(*cb, *sb)); },
                direct: |m| order_numbers(OrderingOperator::LessOrEqual, OrderedOperands::Decimals(&a, &b), m),
                generated: |m| done(decimal_at_most(&a, &b, m)),
            };
            ordering += 1;
        }
    }
    for tuple in [
        [5, 2, 1, 0, 0, 0, 0, 2, 3, 1],
        [4, 2, 1, 0, 0, 0, 0, 2, 3, 1],
    ] {
        agree2! {
            limits: limits(tuple),
            setup: { let (a, b) = (dec(15, 1), dec(2, 0)); },
            direct: |m| order_numbers(OrderingOperator::LessOrEqual, OrderedOperands::Decimals(&a, &b), m),
            generated: |m| done(decimal_at_most(&a, &b, m)),
        };
        ordering += 1;
    }
    assert_rows_agreed(&[("TC-185 ordering", ordering)]);
    assert_eq!(ordering, 30 * 30 + 2);
    println!(
        "TC-185 agreement: {vectors} generated and {named} tabled, {ordering} ordering runtime-only"
    );
}

const E_ACUTE: &str = "\u{e9}";
const E_COMBINING: &str = "e\u{301}";
const SEQUENCES: [&str; 12] = [
    "",
    "a",
    "b",
    "ab",
    E_ACUTE,
    E_COMBINING,
    "\u{fb00}",
    "ff",
    "\u{7f}",
    "\u{80}",
    "\u{1e0a}\u{323}",
    "\u{1e0c}\u{307}",
];

#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc186_text_and_enum_oracles_agree() {
    const T11: [u64; 10] = [0, 0, 0, 5, 3, 2, 0, 2, 6, 1];
    const T14: [u64; 10] = [0, 0, 0, 5, 3, 0, 0, 2, 3, 1];
    // The six `[0, 4]` text admissions are not generated today: no catalogued `convert`
    // identity admits a text operand, so Contract IR refuses their nodes
    // (`tc_024_text_admission_corpus_is_refused_by_ir_today`).
    let mut comparisons = 0_usize;
    for (o, compare) in TEXT_COMPARISONS.into_iter().enumerate() {
        for p in 0..6_usize {
            for left in SEQUENCES {
                for right in SEQUENCES {
                    agree2! {
                        limits: UNLIMITED,
                        setup: { let (l, r) = (text(left, TextProfile::ALL[p]), text(right, TextProfile::ALL[p])); },
                        direct: |m| compare_text(ComparisonOperator::ALL[o], &l, &r, m),
                        generated: |m| typed(compare(&l, &r, m)),
                    };
                    comparisons += 1;
                }
            }
        }
    }
    // T10 distinct profiles, T11 NFC tuple, T14 non-normalizing tuples.
    for (left, lp, right, rp, tuple) in [
        ("abc", 1, "abc", 5, [0; 10]),
        (E_ACUTE, 1, E_COMBINING, 1, T11),
        (E_ACUTE, 0, E_COMBINING, 0, T14),
        (E_ACUTE, 5, E_COMBINING, 5, T14),
    ] {
        agree2! {
            limits: limits(tuple),
            setup: { let (l, r) = (text(left, TextProfile::ALL[lp]), text(right, TextProfile::ALL[rp])); },
            direct: |m| compare_text(ComparisonOperator::Less, &l, &r, m),
            generated: |m| typed(text_less(&l, &r, m)),
        };
        comparisons += 1;
    }
    assert_rows_agreed(&[("TC-186 text comparisons", comparisons)]);
    assert_eq!(comparisons, 6 * 6 * 12 * 12 + 4);

    // T07, T08, T09 (recomputed keys) and T15 over every declaration pair.
    let declarations: [(&str, bool, &[&str]); 4] = [
        ("One", true, &["ONLY"]),
        ("Ordered", true, &["C", "A", "B"]),
        ("Unordered", false, &["A", "B", "C"]),
        ("Other", true, &["A", "B", "C"]),
    ];
    let mut enums = 0_usize;
    for (ln, lo, lm) in declarations {
        for (rn, ro, rm) in declarations {
            for lc in lm {
                for rc in rm {
                    for (o, compare) in ENUM_COMPARISONS.into_iter().enumerate() {
                        for tuple in [[u64::MAX; 10], [0; 10]] {
                            agree2! {
                                limits: limits(tuple),
                                setup: {
                                    let left = enum_declaration(ln, lo, lm).unwrap().value(lc).unwrap();
                                    let right = enum_declaration(rn, ro, rm).unwrap().value(rc).unwrap();
                                },
                                direct: |m| compare_enum(ComparisonOperator::ALL[o], &left, &right, m),
                                generated: |m| typed(compare(&left, &right, m)),
                            };
                            enums += 1;
                        }
                    }
                }
            }
        }
    }
    assert_rows_agreed(&[("TC-186 enum comparisons", enums)]);
    assert_eq!(enums, 10 * 10 * 6 * 2);
    println!("TC-186 agreement: {comparisons} text comparisons, {enums} enum comparisons");
}

/// The unit families the generated TC-187 grid runs every same-unit operation over.
const UNIT_GRID: [&str; 6] = ["m", "cm", "in", "rev", "u2", "s"];

/// The TC-187 arithmetic, comparison, power and tabled-limit vectors. Exact conversion
/// vectors run separately below; scalar and compound targets remain refusal-only cases.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_quantity_oracles_agree() {
    let values = [(0_i64, 1_i64), (1, 1), (-7, 3), (250, 1), (5463, 20)];

    let (mut arithmetic, mut comparisons, mut powers) = (0_usize, 0_usize, 0_usize);
    for name in UNIT_GRID {
        for (ln, ld) in values {
            for (rn, rd) in values {
                agree2! {
                    limits: UNLIMITED,
                    setup: { let f = fixture(); let (a, b) = (f.q(ratio(ln, ld), name), f.q(ratio(rn, rd), name)); },
                    direct: |m| (
                        evaluate_quantity(QuantityOperation::Add(&a, &b), m),
                        evaluate_quantity(QuantityOperation::Subtract(&a, &b), m),
                        evaluate_quantity(QuantityOperation::Multiply(&a, &b), m),
                        evaluate_quantity(QuantityOperation::Divide(&a, &b), m),
                    ),
                    generated: |m| (
                        typed(quantity_add(&a, &b, m)),
                        typed(quantity_subtract(&a, &b, m)),
                        typed(quantity_multiply(&a, &b, m)),
                        typed(quantity_divide(&a, &b, m)),
                    ),
                };
                arithmetic += 4;
                for (o, compare) in QUANTITY_COMPARISONS.into_iter().enumerate() {
                    agree2! {
                        limits: UNLIMITED,
                        setup: { let f = fixture(); let (a, b) = (f.q(ratio(ln, ld), name), f.q(ratio(rn, rd), name)); },
                        direct: |m| compare_quantity(ComparisonOperator::ALL[o], &a, &b, m),
                        generated: |m| typed(compare(&a, &b, m)),
                    };
                    comparisons += 1;
                }
            }
            for exponent in ["-2", "-1", "0", "1", "2", "3"] {
                agree2! {
                    limits: UNLIMITED,
                    setup: { let f = fixture(); let (a, n) = (f.q(ratio(ln, ld), name), big(exponent)); },
                    direct: |m| evaluate_quantity(QuantityOperation::Power(&a, &n), m),
                    generated: |m| typed(quantity_power(&a, &n, m)),
                };
                powers += 1;
            }
        }
    }
    assert_rows_agreed(&[
        ("TC-187 same-unit arithmetic", arithmetic),
        ("TC-187 comparisons", comparisons),
        ("TC-187 powers", powers),
    ]);
    let operations = arithmetic + comparisons + powers;
    assert_eq!(operations, UNIT_GRID.len() * (5 * 5 * (4 + 6) + 5 * 6));

    // The tabled TC-187 vectors run QSpec's own limit tuples and assert QSpec's disposition,
    // including the whole incomplete record: U03 and U19/U24 sums, U28 compound product,
    // U13/U21 powers, U23 comparison tuples.
    let add = Arithmetic::Add;
    let mark = vectors_run();

    let run = agree_arithmetic(rt::unit_tuple([0; 6]), add, (1, "m"), (1, "s"));
    assert_ill_typed_without_charge("U03", &run);
    let run = agree_arithmetic(rt::unit_tuple([4, 0, 0, 2, 5, 1]), add, (2, "m"), (3, "m"));
    assert_eq!(*completed(&run, "U19").value(), rt::ratio(5, 1));
    let run = agree_arithmetic(rt::unit_tuple([4, 0, 0, 2, 4, 1]), add, (2, "m"), (3, "m"));
    assert_incomplete(
        &run.0,
        rt::LimitKind::WorkUnits,
        4,
        4,
        "1",
        rt::ChargePoint::UnitResultRetain,
    );
    let run = agree_arithmetic(
        rt::unit_tuple([4, 0, 0, 2, 5, 1]),
        add,
        (1, "cm"),
        (2, "cm"),
    );
    assert_eq!(*completed(&run, "U24").value(), rt::ratio(3, 1));
    assert!(
        !run.1.contains(&rt::ChargePoint::UnitEdge),
        "U24 charges no edge"
    );
    assert_vectors_since("TC-187 U03/U19/U24 sums", mark, 4);

    let mark = vectors_run();

    let multiply = Arithmetic::Multiply;
    let run = agree_arithmetic(
        rt::unit_tuple([20, 0, 2, 2, 11, 1]),
        multiply,
        (1, "cm"),
        (1, "in"),
    );
    assert_eq!(*completed(&run, "U28").value(), rt::ratio(127, 500_000));
    for (bits, consumed, next) in [(13, 9, "14"), (19, 15, "20")] {
        let run = agree_arithmetic(
            rt::unit_tuple([bits, 0, 2, 2, 11, 1]),
            multiply,
            (1, "cm"),
            (1, "in"),
        );
        assert_incomplete(
            &run.0,
            rt::LimitKind::IntegerBits,
            bits,
            consumed,
            next,
            rt::ChargePoint::UnitRationalArithmetic,
        );
    }
    assert_vectors_since("TC-187 U28 products", mark, 3);

    let mark = vectors_run();

    let run = agree_power(
        rt::unit_tuple([64, 0, 0, 1, 5, 1]),
        (2, "m"),
        "18446744073709551616",
    );
    assert_incomplete(
        &run.0,
        rt::LimitKind::IntegerBits,
        64,
        2,
        "36893488147419103232",
        rt::ChargePoint::UnitRationalArithmetic,
    );
    let run = agree_power(rt::unit_tuple([1, 0, 0, 1, 4, 1]), (0, "m"), "0");
    let one = completed(&run, "U21 0^0");
    assert_eq!(*one.value(), rt::ratio(1, 1));
    assert!(one.unit().dimension().is_dimensionless());
    let run = agree_power(rt::unit_tuple([1, 0, 0, 1, 1, 0]), (0, "m"), "-1");
    assert!(matches!(
        run.0,
        Ok(Outcome::Undefined(rt::Undefined::DivisionByZero))
    ));
    assert_eq!(run.1, vec![rt::ChargePoint::UnitIdentityRead]);
    let run = agree_power(rt::unit_tuple([1, 0, 0, 1, 0, 0]), (0, "m"), "-1");
    assert_incomplete(
        &run.0,
        rt::LimitKind::WorkUnits,
        0,
        0,
        "1",
        rt::ChargePoint::UnitIdentityRead,
    );
    assert_vectors_since("TC-187 U13/U21 powers", mark, 4);

    let mark = vectors_run();

    let run = agree_comparison(
        rt::unit_tuple([15, 0, 2, 2, 9, 1]),
        rt::ComparisonOperator::Less,
        (1, "degC"),
        (2, "degC"),
    );
    assert!(matches!(run.0, Ok(Outcome::Completed(true))));
    let run = agree_comparison(
        rt::unit_tuple([15, 0, 2, 2, 8, 1]),
        rt::ComparisonOperator::Less,
        (1, "degC"),
        (2, "degC"),
    );
    assert_incomplete(
        &run.0,
        rt::LimitKind::WorkUnits,
        8,
        8,
        "1",
        rt::ChargePoint::UnitResultRetain,
    );
    let run = agree_comparison(
        rt::UNLIMITED,
        rt::ComparisonOperator::Less,
        (1, "rev"),
        (2, "rev"),
    );
    assert!(matches!(run.0, Ok(Outcome::Completed(false))));
    let run = agree_comparison(
        rt::UNLIMITED,
        rt::ComparisonOperator::Equal,
        (1, "degC"),
        (1, "degC"),
    );
    assert!(matches!(run.0, Ok(Outcome::Completed(true))));
    for operator in [rt::ComparisonOperator::Less, rt::ComparisonOperator::Equal] {
        let run = agree_comparison(rt::unit_tuple([0; 6]), operator, (0, "degC"), (32, "degF"));
        assert!(matches!(run.0, Err(IllTyped { .. })) && run.1.is_empty());
    }
    assert_vectors_since("TC-187 U23 comparison tuples", mark, 6);
    println!("TC-187 runtime and generated-oracle agreement: {operations} grid operations, 17 tabled tuples");
}

/// Runs one comparison vector on both sides (see [`agree_arithmetic`]).
fn agree_comparison(
    bounds: ScalarLimits,
    operator: rt::ComparisonOperator,
    left: (i64, &str),
    right: (i64, &str),
) -> QuantityRun<bool> {
    note_vector();
    let index = rt::ComparisonOperator::ALL
        .iter()
        .position(|candidate| *candidate == operator)
        .expect("a comparison operator");
    let compare = QUANTITY_COMPARISONS[index];
    agree2! {
        limits: bounds,
        setup: { let f = fixture(); let (a, b) = (f.qi(left.0, left.1), f.qi(right.0, right.1)); },
        direct: |m| compare_quantity(operator, &a, &b, m),
        generated: |m| typed(compare(&a, &b, m)),
    };
    let f = rt::fixture();
    let (a, b) = (f.qi(left.0, left.1), f.qi(right.0, right.1));
    rt::metered(bounds, |m| rt::compare_quantity(operator, &a, &b, m))
}

/// The run stopped incomplete with exactly QSpec's record.
fn assert_incomplete<T: std::fmt::Debug>(
    result: &Result<Outcome<T>, IllTyped>,
    limit_kind: rt::LimitKind,
    limit: u64,
    consumed: u64,
    next_charge: &str,
    charge_point: rt::ChargePoint,
) {
    let Ok(Outcome::Incomplete(record)) = result else {
        panic!("expected an incomplete record, got {result:?}");
    };
    assert_eq!(record.limit_kind, limit_kind);
    assert_eq!((record.limit, record.consumed), (limit, consumed));
    assert_eq!(record.next_charge, rt::big(next_charge));
    assert_eq!(record.charge_point, charge_point);
}

use support::rt_side as rt;

thread_local! {
    /// Vectors the agreement helpers have run on this test's thread. A test measures the vectors it
    /// ran against the count its row list names, so deleting a vector call fails the test.
    static VECTORS_RUN: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn note_vector() {
    VECTORS_RUN.with(|ran| ran.set(ran.get() + 1));
}

fn vectors_run() -> usize {
    VECTORS_RUN.with(std::cell::Cell::get)
}

/// The vectors run since `mark`, which must equal the rows the criterion names for `row`.
fn assert_vectors_since(row: &str, mark: usize, expected: usize) {
    let ran = vectors_run() - mark;
    assert_rows_agreed(&[(row, ran)]);
    assert_eq!(ran, expected, "{row}: every named vector must run");
}

type QuantityRun<T> = (Result<Outcome<T>, IllTyped>, Vec<rt::ChargePoint>, Vec<u64>);

#[derive(Clone, Copy)]
enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Arithmetic {
    const ALL: [Self; 4] = [Self::Add, Self::Subtract, Self::Multiply, Self::Divide];

    fn direct(
        self,
        a: &Quantity,
        b: &Quantity,
        meter: &mut Meter,
    ) -> Result<Outcome<Quantity>, IllTyped> {
        rt::evaluate_quantity(
            match self {
                Self::Add => rt::QuantityOperation::Add(a, b),
                Self::Subtract => rt::QuantityOperation::Subtract(a, b),
                Self::Multiply => rt::QuantityOperation::Multiply(a, b),
                Self::Divide => rt::QuantityOperation::Divide(a, b),
            },
            meter,
        )
    }

    fn generated(
        self,
        a: &Quantity,
        b: &Quantity,
        meter: &mut Meter,
    ) -> Result<Outcome<Quantity>, IllTyped> {
        typed(match self {
            Self::Add => quantity_add(a, b, meter),
            Self::Subtract => quantity_subtract(a, b, meter),
            Self::Multiply => quantity_multiply(a, b, meter),
            Self::Divide => quantity_divide(a, b, meter),
        })
    }
}

/// Runs one binary quantity vector on the direct runtime and the generated oracle, requires them
/// to agree in outcome, charges, counters and every single-charge denial, and returns the runtime
/// run for the vector's own expected-disposition assertions.
fn agree_arithmetic(
    bounds: ScalarLimits,
    operation: Arithmetic,
    left: (i64, &str),
    right: (i64, &str),
) -> QuantityRun<Quantity> {
    note_vector();
    agree2! {
        limits: bounds,
        setup: { let f = fixture(); let (a, b) = (f.qi(left.0, left.1), f.qi(right.0, right.1)); },
        direct: |m| operation.direct(&a, &b, m),
        generated: |m| operation.generated(&a, &b, m),
    };
    let f = rt::fixture();
    let (a, b) = (f.qi(left.0, left.1), f.qi(right.0, right.1));
    let (outcome, charges, consumed) = rt::metered(bounds, |m| operation.direct(&a, &b, m));
    (outcome, charges, consumed)
}

/// The power counterpart of [`agree_arithmetic`].
fn agree_power(bounds: ScalarLimits, base: (i64, &str), exponent: &str) -> QuantityRun<Quantity> {
    note_vector();
    agree2! {
        limits: bounds,
        setup: { let f = fixture(); let (a, n) = (f.qi(base.0, base.1), big(exponent)); },
        direct: |m| evaluate_quantity(QuantityOperation::Power(&a, &n), m),
        generated: |m| typed(quantity_power(&a, &n, m)),
    };
    let f = rt::fixture();
    let (a, n) = (f.qi(base.0, base.1), rt::big(exponent));
    let (outcome, charges, consumed) = rt::metered(bounds, |m| {
        rt::evaluate_quantity(rt::QuantityOperation::Power(&a, &n), m)
    });
    (outcome, charges, consumed)
}

/// The vector refused `ill_typed` and admitted no charge.
fn assert_ill_typed_without_charge(row: &str, run: &QuantityRun<Quantity>) {
    assert!(
        matches!(run.0, Err(IllTyped { .. })),
        "{row}: expected refused ill_typed, got {:?}",
        run.0
    );
    assert!(
        run.1.is_empty(),
        "{row}: a refusal before arithmetic admits no charge"
    );
    assert!(
        run.2.iter().all(|consumed| *consumed == 0),
        "{row}: nothing is consumed"
    );
}

/// The ascending compound-unit terms of a quantity, by base-unit name.
fn compound_terms(quantity: &Quantity) -> Vec<(rt::NodeKey, Integer)> {
    match quantity.unit() {
        rt::QuantityUnit::Compound(unit) => unit
            .terms()
            .map(|(key, exponent)| (key, exponent.clone()))
            .collect(),
        other => panic!("expected a compound unit, got {other:?}"),
    }
}

fn expected_terms(f: &rt::Fixture, terms: &[(&str, i64)]) -> Vec<(rt::NodeKey, Integer)> {
    let mut expected: Vec<_> = terms
        .iter()
        .map(|(name, exponent)| (f.key(name), Integer::from(*exponent)))
        .collect();
    expected.sort_by_key(|(key, _)| *key);
    expected
}

fn completed(run: &QuantityRun<Quantity>, row: &str) -> Quantity {
    match &run.0 {
        Ok(Outcome::Completed(quantity)) => quantity.clone(),
        other => panic!("{row}: expected a completed quantity, got {other:?}"),
    }
}

/// TC-187 U12: quantity multiplication, division and power under canonical compound units, with
/// the `s` family the grid needs for a mixed-unit product.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u12_compound_unit_products_agree() {
    let f = rt::fixture();
    let unlimited = rt::UNLIMITED;
    let mark = vectors_run();

    let product = agree_arithmetic(unlimited, Arithmetic::Multiply, (2, "m"), (3, "s"));
    let quantity = completed(&product, "2 m * 3 s");
    assert_eq!(*quantity.value(), rt::ratio(6, 1));
    assert_eq!(
        compound_terms(&quantity),
        expected_terms(&f, &[("m", 1), ("s", 1)])
    );

    let quotient = agree_arithmetic(unlimited, Arithmetic::Divide, (6, "m"), (2, "s"));
    let quantity = completed(&quotient, "6 m / 2 s");
    assert_eq!(*quantity.value(), rt::ratio(3, 1));
    assert_eq!(
        compound_terms(&quantity),
        expected_terms(&f, &[("m", 1), ("s", -1)])
    );

    let square = agree_power(unlimited, (2, "m"), "2");
    let quantity = completed(&square, "(2 m)^2");
    assert_eq!(*quantity.value(), rt::ratio(4, 1));
    assert_eq!(compound_terms(&quantity), expected_terms(&f, &[("m", 2)]));

    let cancelled = agree_arithmetic(unlimited, Arithmetic::Divide, (5, "m"), (5, "m"));
    let quantity = completed(&cancelled, "5 m / 5 m");
    assert_eq!(*quantity.value(), rt::ratio(1, 1));
    assert!(quantity.unit().dimension().is_dimensionless());
    assert!(compound_terms(&quantity).is_empty());
    assert_vectors_since("TC-187 U12 products", mark, 4);
}

/// TC-187 U02: the normalized dimension map drops zero exponents; `(L*T)/T` is `L` and `L^0` is
/// the empty dimension, both in the retained sorted maps and through the quantity operators.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u02_dimension_normalization_agrees() {
    let f = rt::fixture();
    let (l, t) = (
        f.graph.dimension(f.key("L")).expect("length dimension"),
        f.graph.dimension(f.key("T")).expect("time dimension"),
    );
    let normalized = l.multiply(t).divide(t);
    assert_eq!(&normalized, l);
    assert_eq!(
        normalized
            .exponents()
            .map(|(k, e)| (k, e.clone()))
            .collect::<Vec<_>>(),
        vec![(f.key("L"), Integer::from(1_i64))]
    );
    let zeroth = l.power(&rt::int(0));
    assert!(zeroth.is_dimensionless());
    assert_eq!(zeroth.exponents().count(), 0);

    // The same normalization through the operators both sides execute: (1 m * 1 s) / 1 s.
    let mark = vectors_run();
    note_vector();
    agree2! {
        limits: UNLIMITED,
        setup: { let f = fixture(); let (a, b) = (f.qi(1, "m"), f.qi(1, "s")); },
        direct: |m| {
            let product = evaluate_quantity(QuantityOperation::Multiply(&a, &b), m);
            let quotient = match &product {
                Ok(Outcome::Completed(p)) => Some(evaluate_quantity(QuantityOperation::Divide(p, &b), m)),
                _ => None,
            };
            (product, quotient)
        },
        generated: |m| {
            let product = typed(quantity_multiply(&a, &b, m));
            let quotient = match &product {
                Ok(Outcome::Completed(p)) => Some(typed(quantity_divide(p, &b, m))),
                _ => None,
            };
            (product, quotient)
        },
    };
    let (a, b) = (f.qi(1, "m"), f.qi(1, "s"));
    let ((product, quotient), _, _) = rt::metered(rt::UNLIMITED, |m| {
        let product = rt::evaluate_quantity(rt::QuantityOperation::Multiply(&a, &b), m);
        let quotient = match &product {
            Ok(Outcome::Completed(p)) => Some(rt::evaluate_quantity(
                rt::QuantityOperation::Divide(p, &b),
                m,
            )),
            _ => None,
        };
        (product, quotient)
    });
    assert!(matches!(product, Ok(Outcome::Completed(_))));
    let Some(Ok(Outcome::Completed(quantity))) = quotient else {
        panic!("(1 m * 1 s) / 1 s must complete");
    };
    assert_eq!(quantity.unit().dimension(), l);
    assert_eq!(compound_terms(&quantity), expected_terms(&f, &[("m", 1)]));

    let power = agree_power(rt::UNLIMITED, (1, "m"), "0");
    let quantity = completed(&power, "(1 m)^0");
    assert!(quantity.unit().dimension().is_dimensionless());
    assert_vectors_since("TC-187 U02 normalization", mark, 2);
}

/// TC-187 U05: arithmetic, comparison-free operators and a power on a nonzero-offset point
/// (`degC`, `degF`) are refused `ill_typed` before any charge.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u05_affine_point_arithmetic_is_refused_alike() {
    let mark = vectors_run();
    for (left, right) in [
        ((1, "degC"), (2, "degC")),
        ((1, "degF"), (2, "degF")),
        ((1, "degC"), (2, "degF")),
    ] {
        for operation in Arithmetic::ALL {
            let run = agree_arithmetic(rt::UNLIMITED, operation, left, right);
            assert_ill_typed_without_charge("U05 binary", &run);
        }
    }
    for unit in ["degC", "degF"] {
        for exponent in ["-1", "0", "2"] {
            let run = agree_power(rt::UNLIMITED, (3, unit), exponent);
            assert_ill_typed_without_charge("U05 power", &run);
        }
    }
    assert_vectors_since("TC-187 U05 affine points", mark, 3 * 4 + 2 * 3);
}

/// TC-187 U09: a base dimension with the same declaration name and shape as `L` but a different
/// owner, hence a different node key, is an incompatible dimension despite the resemblance.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u09_a_foreign_dimension_key_is_ill_typed() {
    let mark = vectors_run();
    let run = agree_arithmetic(rt::UNLIMITED, Arithmetic::Add, (1, "m"), (1, "m_foreign"));
    assert_ill_typed_without_charge("U09", &run);
    let Err(refusal) = run.0 else {
        unreachable!("asserted above")
    };
    assert_eq!(refusal.cause, rt::IllTypedCause::IncompatibleDimensions);
    assert_eq!(IllTyped::CODE, "ill_typed");
    // The control: the same shape under the one `L` key is accepted.
    let control = agree_arithmetic(rt::UNLIMITED, Arithmetic::Add, (1, "m"), (1, "m"));
    assert!(matches!(control.0, Ok(Outcome::Completed(_))));
    assert_vectors_since("TC-187 U09 foreign dimension key", mark, 2);
}

/// TC-187 U14: under `L(0,0,0,0,0,0)` each incompatible, affine or distinct-unit operation is
/// refused `ill_typed` with no charge.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u14_static_refusals_charge_nothing() {
    let zero = rt::unit_tuple([0; 6]);
    let mark = vectors_run();
    for (operation, left, right) in [
        (Arithmetic::Add, (1, "m"), (1, "s")),
        (Arithmetic::Add, (1, "degC"), (1, "s")),
        (Arithmetic::Add, (1, "degC"), (1, "degC")),
        (Arithmetic::Divide, (1, "degC"), (0, "degC")),
        (Arithmetic::Add, (1, "cm"), (1, "m")),
    ] {
        let run = agree_arithmetic(zero, operation, left, right);
        assert_ill_typed_without_charge("U14", &run);
    }
    assert_vectors_since("TC-187 U14 static refusals", mark, 5);
}

/// TC-187 U20: the affine-by-composed-offset units `u1` and `u3` refuse arithmetic with no
/// charge; `u2`'s offsets compose to zero, so its sum is exact with no edge.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u20_composed_offsets_decide_affinity() {
    let zero = rt::unit_tuple([0; 6]);
    let mark = vectors_run();
    let sum = agree_arithmetic(zero, Arithmetic::Add, (1, "u1"), (2, "u1"));
    assert_ill_typed_without_charge("U20 u1 sum", &sum);
    let product = agree_arithmetic(zero, Arithmetic::Multiply, (1, "u3"), (1, "m"));
    assert_ill_typed_without_charge("U20 u3 product", &product);

    let exact = agree_arithmetic(
        rt::unit_tuple([4, 0, 0, 2, 5, 1]),
        Arithmetic::Add,
        (1, "u2"),
        (2, "u2"),
    );
    let quantity = completed(&exact, "U20 u2 sum");
    assert_eq!(*quantity.value(), rt::ratio(3, 1));
    let f = rt::fixture();
    assert_eq!(quantity.unit(), &f.unit("u2"));
    assert!(
        !exact.1.contains(&rt::ChargePoint::UnitEdge),
        "u2 is not affine: no edge is charged"
    );
    assert_vectors_since("TC-187 U20 affinity", mark, 3);
}

/// TC-187 U27: `1 cm / 0 cm` is undefined after both identity reads and before any edge; one work
/// unit fewer stops incomplete at the second read.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u27_division_by_zero_precedes_every_edge() {
    let mark = vectors_run();
    let undefined = agree_arithmetic(
        rt::unit_tuple([1, 0, 0, 2, 2, 0]),
        Arithmetic::Divide,
        (1, "cm"),
        (0, "cm"),
    );
    assert!(matches!(
        undefined.0,
        Ok(Outcome::Undefined(rt::Undefined::DivisionByZero))
    ));
    assert_eq!(
        undefined.1,
        vec![
            rt::ChargePoint::UnitIdentityRead,
            rt::ChargePoint::UnitIdentityRead
        ]
    );

    let stopped = agree_arithmetic(
        rt::unit_tuple([1, 0, 0, 2, 1, 0]),
        Arithmetic::Divide,
        (1, "cm"),
        (0, "cm"),
    );
    let Ok(Outcome::Incomplete(incomplete)) = &stopped.0 else {
        panic!(
            "U27 under one work unit must be incomplete, got {:?}",
            stopped.0
        );
    };
    assert_eq!(incomplete.limit_kind, rt::LimitKind::WorkUnits);
    assert_eq!((incomplete.limit, incomplete.consumed), (1, 1));
    assert_eq!(incomplete.next_charge, Integer::from(1_i64));
    assert_eq!(incomplete.charge_point, rt::ChargePoint::UnitIdentityRead);
    assert_vectors_since("TC-187 U27 division by zero", mark, 2);
}

/// The refusal cause of a graph admission that must fail.
fn admission_refusal(
    result: Result<rt::UnitGraph, rt::InvalidSemanticGraph>,
) -> rt::SemanticGraphCause {
    match result {
        Ok(_) => panic!("the unit graph was admitted"),
        Err(refusal) => {
            assert_eq!(rt::InvalidSemanticGraph::CODE, "invalid_semantic_graph");
            refusal.cause
        }
    }
}

fn node(n: u8) -> rt::NodeKey {
    rt::NodeKey::from_hex(&format!("{n:064x}")).expect("a 64-digit hex key")
}

fn declaration(
    dimension: rt::NodeKey,
    target: Option<rt::NodeKey>,
    scale: rt::Rational,
) -> rt::UnitDeclaration {
    rt::UnitDeclaration {
        dimension,
        target,
        scale,
        offset: rt::ratio(0, 1),
    }
}

/// TC-187 U06 and U11: the generated oracles take an already admitted unit graph, so these rows
/// have no generated oracle; they run against the same graph admission every agreement vector
/// builds its fixture through. A zero scale and each broken topology of the `L` graph is refused
/// `invalid_semantic_graph`, with its typed cause, before a quantity can exist.
#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc187_u06_u11_unit_graph_admission_refuses_each_malformed_graph() {
    // The corpus fixture itself admits.
    assert!(rt::admit_graph(&support::GraphSpec::tc187()).is_ok());

    // U06: a unit with zero scale.
    let mut zero_scale = support::GraphSpec::tc187();
    zero_scale
        .units
        .push(support::unit("zero", "L", Some("m"), (0, 1), (0, 1)));
    let cause = admission_refusal(rt::admit_graph(&zero_scale).map(|fixture| fixture.graph));
    assert_eq!(cause, rt::SemanticGraphCause::ZeroScale);

    // U11, built through the fixture: a second targetless root under `L`.
    let mut two_roots = support::GraphSpec::tc187();
    two_roots.units.push(support::root("m_second_root", "L"));
    let cause = admission_refusal(rt::admit_graph(&two_roots).map(|fixture| fixture.graph));
    assert_eq!(cause, rt::SemanticGraphCause::DuplicateRoot);

    // U11, built from raw node keys, because content-hashed fixture keys cannot form a cycle,
    // omit a root that a target needs, or name a target that does not exist.
    let (length, time) = (node(1), node(2));
    let (a, b, root) = (node(10), node(11), node(12));
    let dimensions = || [(length, vec![]), (time, vec![])];
    let one = || rt::ratio(1, 1);

    let cause = admission_refusal(rt::UnitGraph::admit(
        dimensions(),
        [
            (a, declaration(length, Some(b), one())),
            (b, declaration(length, Some(a), one())),
        ],
    ));
    assert_eq!(cause, rt::SemanticGraphCause::MissingRoot);

    let cause = admission_refusal(rt::UnitGraph::admit(
        dimensions(),
        [
            (root, declaration(length, None, one())),
            (a, declaration(length, Some(b), one())),
        ],
    ));
    assert_eq!(cause, rt::SemanticGraphCause::UnknownTarget);

    let cause = admission_refusal(rt::UnitGraph::admit(
        dimensions(),
        [
            (root, declaration(time, None, one())),
            (b, declaration(length, None, one())),
            (a, declaration(length, Some(root), one())),
        ],
    ));
    assert_eq!(cause, rt::SemanticGraphCause::CrossDimensionTarget);

    let cause = admission_refusal(rt::UnitGraph::admit(
        dimensions(),
        [
            (root, declaration(length, None, one())),
            (a, declaration(length, Some(a), one())),
        ],
    ));
    assert_eq!(cause, rt::SemanticGraphCause::TargetCycle);
}

/// The TC-193 class matrix patterns of `IeeeWidth::ALL[width]`.
fn classes(width: usize) -> Vec<u64> {
    let (exponent_bits, fraction_bits): (u32, u32) = if width == 0 { (8, 23) } else { (11, 52) };
    let sign = 1_u64 << (exponent_bits + fraction_bits);
    let exponent = ((1_u64 << exponent_bits) - 1) << fraction_bits;
    let fraction = (1_u64 << fraction_bits) - 1;
    let quiet = 1_u64 << (fraction_bits - 1);
    let one = ((1_u64 << (exponent_bits - 1)) - 1) << fraction_bits;
    let three = one + (1 << fraction_bits) + quiet;
    let max = exponent - (1 << fraction_bits) + fraction;
    [
        0,
        1,
        fraction,
        1 << fraction_bits,
        max,
        one,
        three,
        exponent,
    ]
    .into_iter()
    .flat_map(|bits| [bits, bits | sign])
    .chain([
        exponent | 1,
        sign | exponent | 2,
        exponent | quiet | 3,
        sign | exponent | quiet,
    ])
    .collect()
}

#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_tc193_ieee_oracles_agree() {
    let (narrow, wide) = (classes(0), classes(1));
    let mut vectors = 0_usize;
    for a in &narrow {
        for b in &narrow {
            agree2! {
                limits: UNLIMITED,
                setup: { let (x, y) = (ieee(0, *a), ieee(0, *b)); },
                direct: |m| (
                    evaluate_ieee(IeeeOperation::Add(x, y), RoundingMode::NearestEven, m),
                    evaluate_ieee(IeeeOperation::Subtract(x, y), RoundingMode::NearestEven, m),
                    compare_ieee(IeeeComparison::NumericEqual, x, y, m),
                ),
                generated: |m| (
                    typed(binary32_add(x, y, m)),
                    typed(binary32_subtract(x, y, m)),
                    typed(binary32_numeric_equal(x, y, m)),
                ),
            };
            vectors += 3;
        }
    }
    for a in &wide {
        for b in &wide {
            agree2! {
                limits: UNLIMITED,
                setup: { let (x, y) = (ieee(1, *a), ieee(1, *b)); },
                direct: |m| (
                    evaluate_ieee(IeeeOperation::Divide(x, y), RoundingMode::TowardZero, m),
                    evaluate_ieee(IeeeOperation::Multiply(x, y), RoundingMode::TowardPositive, m),
                    compare_ieee(IeeeComparison::TotalOrder, x, y, m),
                    compare_ieee(IeeeComparison::BitIdentical, x, y, m),
                ),
                generated: |m| (
                    typed(binary64_divide(x, y, m)),
                    typed(binary64_multiply(x, y, m)),
                    typed(binary64_total_order(x, y, m)),
                    typed(binary64_bit_identical(x, y, m)),
                ),
            };
            vectors += 4;
        }
        agree2! {
            limits: UNLIMITED,
            setup: { let x = ieee(1, *a); },
            direct: |m| convert_ieee_width(x, IeeeWidth::Binary32, RoundingMode::NearestEven, m),
            generated: |m| done(narrow_to_binary32(x, m)),
        };
        vectors += 1;
    }
    assert_rows_agreed(&[("TC-193 class-matrix vectors", vectors)]);
    assert_eq!(vectors, 3 * 20 * 20 + 4 * 20 * 20 + 20);

    // F21 zero sums and F09 under NearestEven, then the F31 narrowing tuples.
    let mut named = 0_usize;
    for (a, b, (i, o, w, r)) in [
        (0x0000_0000, 0x0000_0000, (32, 2, 4, 1)),
        (0x0000_0000, 0x0000_0000, (32, 2, 3, 1)),
        (0x3f80_0000, 0x3380_0000, (32, 2, 3, 1)),
        (0x3f80_0000, 0x3380_0000, (31, 2, 4, 1)),
    ] {
        agree2! {
            limits: ieee_limits(i, o, w, r),
            setup: { let (x, y) = (f32v(a), f32v(b)); },
            direct: |m| evaluate_ieee(IeeeOperation::Add(x, y), RoundingMode::NearestEven, m),
            generated: |m| typed(binary32_add(x, y, m)),
        };
        named += 1;
    }
    for (bits, work) in [
        (0xfff0_0000_0000_0000, 2),
        (0xfff0_0000_0000_0000, 1),
        (0x3fb9_9999_9999_999a, 4),
        (0x47ef_ffff_e000_0000, 4),
        (0x47ef_ffff_f000_0000, 4),
        (0x47ef_ffff_f000_0000, 3),
        (1, 4),
        (1, 3),
    ] {
        agree2! {
            limits: ieee_limits(64, 1, work, 1),
            setup: { let x = f64v(bits); },
            direct: |m| convert_ieee_width(x, IeeeWidth::Binary32, RoundingMode::NearestEven, m),
            generated: |m| done(narrow_to_binary32(x, m)),
        };
        named += 1;
    }
    assert_rows_agreed(&[("TC-193 tabled tuples", named)]);
    assert_eq!(named, 12);
    println!("TC-193 agreement: {vectors} class-matrix vectors, {named} tabled tuples");
}

#[trace("FR-014-AC-9", "TC-024")]
#[test]
fn tc_024_operand_width_mismatch_is_a_typed_stop_before_any_charge() {
    use quire_contract_runtime::exact::{IeeeValue, Meter, ScalarLimits};
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    });
    let (narrow, wide) = (IeeeValue::binary32(0), IeeeValue::binary64(0));
    assert_eq!(
        binary32_add(narrow, wide, &mut meter),
        Err(OracleStop::OperandWidth)
    );
    assert_eq!(
        binary64_total_order(narrow, wide, &mut meter),
        Err(OracleStop::OperandWidth)
    );
    assert_eq!(
        narrow_to_binary32(narrow, &mut meter),
        Err(OracleStop::OperandWidth)
    );
    assert!(meter.admitted_charges().is_empty());
}

#[trace("FR-014-AC-6", "TC-024")]
#[test]
fn tc_024_no_vector_here_has_an_authority_leg_and_all_agree_with_direct_runtime() {
    let mut vectors = 0_usize;
    for a in -20_i128..=20 {
        agree2! {
            limits: UNLIMITED,
            setup: { let a = int(a); let domain = IntegerDomain::Bounded(IntegerInterval::new(int(-8), int(7)).unwrap()); },
            direct: |m| evaluate_integer_arithmetic(IntegerArithmetic::Negate(&a), as_bound(&domain), m),
            generated: |m| done(integer_negate(&a, m)),
        };
        vectors += 1;
        for b in -20_i128..=20 {
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); },
                direct: |m| (
                    evaluate_integer_arithmetic(IntegerArithmetic::Add(&a, &b), as_bound(&wide()), m),
                    evaluate_integer_arithmetic(IntegerArithmetic::Subtract(&a, &b), as_bound(&wide()), m),
                    evaluate_integer_arithmetic(IntegerArithmetic::Multiply(&a, &b), as_bound(&wide()), m),
                    {
                        let (da, db) = (Rational::from_integer(a.clone()), Rational::from_integer(b.clone()));
                        evaluate_rational_arithmetic(RationalArithmetic::Divide(&da, &db), Some(&wide_rational()), m)
                    },
                ),
                generated: |m| (
                    done(integer_add(&a, &b, m)),
                    done(integer_subtract(&a, &b, m)),
                    done(integer_multiply(&a, &b, m)),
                    done(integer_divide(&a, &b, m)),
                ),
            };
            agree2! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); },
                direct: |m| (
                    order_numbers(OrderingOperator::Less, OrderedOperands::Integers(&a, &b), m),
                    order_numbers(OrderingOperator::LessOrEqual, OrderedOperands::Integers(&a, &b), m),
                    order_numbers(OrderingOperator::GreaterOrEqual, OrderedOperands::Integers(&a, &b), m),
                ),
                generated: |m| (
                    done(integer_less(&a, &b, m)),
                    done(integer_at_most(&a, &b, m)),
                    done(integer_at_least(&a, &b, m)),
                ),
            };
            vectors += 7;
        }
    }
    let rationals = (-6_i64..=6)
        .flat_map(|n| (1_i64..=3).map(move |d| (n, d)))
        .collect::<Vec<_>>();
    for (ln, ld) in &rationals {
        agree2! {
            limits: UNLIMITED,
            setup: { let a = ratio(*ln, *ld); },
            direct: |m| evaluate_rational_arithmetic(RationalArithmetic::Negate(&a), Some(&wide_rational()), m),
            generated: |m| done(rational_negate(&a, m)),
        };
        vectors += 1;
        for (rn, rd) in &rationals {
            agree2! {
                limits: UNLIMITED,
                setup: {
                    let (a, b) = (ratio(*ln, *ld), ratio(*rn, *rd));
                    let domain = RationalDomain::new(IntegerInterval::new(int(-10), int(10)).unwrap(), IntegerInterval::new(int(1), int(4)).unwrap()).unwrap();
                },
                direct: |m| (
                    evaluate_rational_arithmetic(RationalArithmetic::Add(&a, &b), Some(&wide_rational()), m),
                    evaluate_rational_arithmetic(RationalArithmetic::Subtract(&a, &b), Some(&wide_rational()), m),
                    evaluate_rational_arithmetic(RationalArithmetic::Multiply(&a, &b), Some(&wide_rational()), m),
                    evaluate_rational_arithmetic(RationalArithmetic::Divide(&a, &b), Some(&domain), m),
                    order_numbers(OrderingOperator::Greater, OrderedOperands::Rationals(&a, &b), m),
                ),
                generated: |m| (
                    done(rational_add(&a, &b, m)),
                    done(rational_subtract(&a, &b, m)),
                    done(rational_multiply(&a, &b, m)),
                    done(rational_divide(&a, &b, m)),
                    done(rational_greater(&a, &b, m)),
                ),
            };
            vectors += 5;
        }
    }
    assert_rows_agreed(&[("runtime-only vectors", vectors)]);
    assert_eq!(vectors, 41 + 7 * 41 * 41 + 39 + 5 * 39 * 39);
    println!("runtime-only agreement (no authority leg): {vectors} vectors");
}

/// Adapt the direct runtime conversion outcome to the oracle's quantity result.
fn direct_conversion(
    source: &Quantity,
    unit: &rt::QuantityUnit,
    meter: &mut Meter,
) -> Result<Outcome<Quantity>, IllTyped> {
    rt::convert_quantity(source, unit, &rt::QuantityTarget::Exact, meter).map(|outcome| {
        match outcome {
            Outcome::Completed(converted) => {
                let rt::ConvertedValue::Exact(value) = converted.value() else {
                    panic!("exact target")
                };
                Outcome::Completed(Quantity::new(value.clone(), converted.unit().clone()))
            }
            Outcome::Undefined(reason) => Outcome::Undefined(reason),
            Outcome::Refused(reason) => Outcome::Refused(reason),
            Outcome::Incomplete(reason) => Outcome::Incomplete(reason),
            _ => panic!("unknown outcome"),
        }
    })
}

fn conversion_fixture() -> rt::Fixture {
    rt::admit_graph(&support::GraphSpec::conversions()).expect("conversion graph")
}

fn compound_source(f: &rt::Fixture, magnitude: i64, terms: &[(&str, i64)]) -> Quantity {
    let mut terms = terms
        .iter()
        .map(|(unit, exponent)| (f.key(unit), rt::int(i128::from(*exponent))))
        .collect::<Vec<_>>();
    terms.sort_by_key(|(key, _)| *key);
    Quantity::new(
        rt::ratio(magnitude, 1),
        rt::QuantityUnit::Compound(f.graph.compound_unit(&terms).unwrap()),
    )
}

fn agree_conversion(
    bounds: ScalarLimits,
    source: &Quantity,
    unit: &rt::QuantityUnit,
) -> QuantityRun<Quantity> {
    note_vector();
    agree2! {
        limits: bounds,
        setup: {},
        direct: |m| direct_conversion(source, unit, m),
        generated: |m| typed(quantity_convert(source, unit, m)),
    };
    rt::metered(bounds, |m| direct_conversion(source, unit, m))
}

/// QSpec TC-187 U01/U04/U09b/U10/U15/U22/U25(second)/U26, including each denied charge.
#[trace("FR-014-AC-6", "FR-014-AC-48", "FR-014-AC-50", "TC-024")]
#[test]
fn exact_quantity_conversion_vectors_agree() {
    let before = vectors_run();
    let f = conversion_fixture();
    for (row, source, target, bounds, expected) in [
        (
            "U01",
            f.qi(250, "cm"),
            "m",
            rt::UNLIMITED,
            Some(rt::ratio(5, 2)),
        ),
        (
            "U04 C",
            f.qi(0, "degC"),
            "K",
            rt::UNLIMITED,
            Some(rt::ratio(5463, 20)),
        ),
        (
            "U04 F",
            f.qi(32, "degF"),
            "K",
            rt::UNLIMITED,
            Some(rt::ratio(5463, 20)),
        ),
        (
            "U09b",
            f.qi(1, "m_alias"),
            "m",
            rt::UNLIMITED,
            Some(rt::ratio(1, 1)),
        ),
        (
            "U10",
            f.qi(100, "cm"),
            "m",
            rt::unit_tuple([8, 0, 1, 1, 6, 1]),
            Some(rt::ratio(1, 1)),
        ),
        (
            "U15",
            f.qi(1, "in"),
            "cm",
            rt::unit_tuple([15, 0, 2, 1, 9, 1]),
            Some(rt::ratio(127, 50)),
        ),
        (
            "U15 denied edge",
            f.qi(1, "in"),
            "cm",
            rt::unit_tuple([7, 0, 1, 1, 9, 1]),
            None,
        ),
        (
            "U22 m2",
            compound_source(&f, 4, &[("m", 2)]),
            "m2",
            rt::unit_tuple([3, 0, 0, 1, 3, 1]),
            Some(rt::ratio(4, 1)),
        ),
        (
            "U22 cm2",
            compound_source(&f, 4, &[("m", 2)]),
            "cm2",
            rt::unit_tuple([17, 0, 1, 1, 6, 1]),
            Some(rt::ratio(40000, 1)),
        ),
        (
            "U22 denied bits",
            compound_source(&f, 4, &[("m", 2)]),
            "cm2",
            rt::unit_tuple([16, 0, 1, 1, 6, 1]),
            None,
        ),
        (
            "U25 second",
            compound_source(&f, 1, &[("kg", 1), ("m", 2), ("s", -2)]),
            "J",
            rt::unit_tuple([1, 0, 0, 1, 3, 1]),
            Some(rt::ratio(1, 1)),
        ),
        (
            "U26",
            f.qi(1, "u2"),
            "u1",
            rt::unit_tuple([6, 0, 3, 1, 12, 1]),
            Some(rt::ratio(-9, 1)),
        ),
        (
            "U26 denied edge",
            f.qi(1, "u2"),
            "u1",
            rt::unit_tuple([6, 0, 2, 1, 12, 1]),
            None,
        ),
    ] {
        let unit = f.unit(target);
        let run = agree_conversion(bounds, &source, &unit);
        if let Some(value) = expected {
            assert_eq!(
                run.0,
                Ok(Outcome::Completed(Quantity::new(value, unit))),
                "{row}"
            );
        } else {
            match row {
                "U15 denied edge" => assert_incomplete(
                    &run.0,
                    rt::LimitKind::UnitEdges,
                    1,
                    1,
                    "2",
                    rt::ChargePoint::UnitEdge,
                ),
                "U22 denied bits" => assert_incomplete(
                    &run.0,
                    rt::LimitKind::IntegerBits,
                    16,
                    5,
                    "17",
                    rt::ChargePoint::UnitRationalArithmetic,
                ),
                "U26 denied edge" => assert_incomplete(
                    &run.0,
                    rt::LimitKind::UnitEdges,
                    2,
                    2,
                    "3",
                    rt::ChargePoint::UnitEdge,
                ),
                _ => panic!("unexpected incomplete row {row}"),
            }
        }
    }
    note_vector();
    let source = f.qi(250, "cm");
    let unit = f.unit("m");
    let one = f.qi(1, "m");
    agree2! {
        limits: rt::UNLIMITED,
        setup: {},
        direct: |m| match direct_conversion(&source, &unit, m) {
            Ok(Outcome::Completed(converted)) => rt::evaluate_quantity(rt::QuantityOperation::Add(&converted, &one), m),
            other => other,
        },
        generated: |m| match typed(quantity_convert(&source, &unit, m)) {
            Ok(Outcome::Completed(converted)) => typed(quantity_add(&converted, &one, m)),
            other => other,
        },
    };
    let converted = quantity_convert(&source, &unit, &mut Meter::new(rt::UNLIMITED))
        .unwrap()
        .completed()
        .unwrap();
    assert_eq!(
        quantity_add(&converted, &one, &mut Meter::new(rt::UNLIMITED)).unwrap(),
        Outcome::Completed(f.q(rt::ratio(7, 2), "m"))
    );
    assert_ne!(
        f.unit("m_alias"),
        f.unit("m"),
        "U09b preserves distinct source identity"
    );
    let final_denial = agree_conversion(
        rt::unit_tuple([8, 0, 1, 1, 6, 0]),
        &f.qi(100, "cm"),
        &f.unit("m"),
    );
    assert_incomplete(
        &final_denial.0,
        rt::LimitKind::ResultUnits,
        0,
        0,
        "1",
        rt::ChargePoint::UnitResultRetain,
    );
    let unit = f.unit("m");
    let source = f.qi(100, "cm");
    let denied = rt::denials(rt::unit_tuple([8, 0, 1, 1, 6, 1]), |m| {
        typed(quantity_convert(&source, &unit, m))
    });
    assert!(denied.iter().any(|(point, _, outcome, retained)| *point
        == rt::ChargePoint::UnitResultRetain
        && matches!(outcome, Ok(Outcome::Incomplete(_)))
        && *retained == 0));
    assert_eq!(vectors_run() - before, 15, "every conversion row ran");
}

/// Dimension identity mismatches refuse before any charge, including with a populated meter.
#[trace("FR-014-AC-49", "TC-024")]
#[test]
fn quantity_conversion_ill_typed_preserves_meter() {
    let before = vectors_run();
    let f = conversion_fixture();
    for (row, source, target) in [
        ("m to s", f.qi(1, "m"), "s"),
        ("U09", f.qi(1, "m_foreign"), "m"),
        ("U22 nominal", f.qi(1, "N_m"), "J"),
        ("compound to m", compound_source(&f, 4, &[("m", 2)]), "m"),
    ] {
        let unit = f.unit(target);
        let run = agree_conversion(rt::UNLIMITED, &source, &unit);
        assert_ill_typed_without_charge(row, &run);
        let mut meter = Meter::new(rt::UNLIMITED);
        let seed = f.qi(100, "cm");
        let _ = quantity_convert(&seed, &f.unit("m"), &mut meter).expect("seed");
        let baseline = (
            meter.admitted_charges().to_vec(),
            rt::LimitKind::ALL
                .iter()
                .map(|kind| meter.consumed(*kind))
                .collect::<Vec<_>>(),
        );
        assert!(matches!(
            quantity_convert(&source, &unit, &mut meter),
            Err(OracleStop::IllTyped(_))
        ));
        assert_eq!(
            baseline,
            (
                meter.admitted_charges().to_vec(),
                rt::LimitKind::ALL
                    .iter()
                    .map(|kind| meter.consumed(*kind))
                    .collect::<Vec<_>>()
            )
        );
    }
    assert_eq!(vectors_run() - before, 4);
}
