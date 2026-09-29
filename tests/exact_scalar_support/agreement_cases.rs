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
    decimal_multiply,
    integer_multiply,
    binary32_add,
    text_not_equal,
    quantity_at_least,
    integer_at_least,
    narrow_to_binary32,
    enum_at_least,
    quantity_add,
    integer_less,
    rational_divide,
    binary32_numeric_equal,
    admit_nfkd,
    enum_equal,
    integer_at_most,
    quantity_power,
    text_greater,
    quantity_greater,
    rational_multiply,
    modulo_bounded,
    binary64_bit_identical,
    quantity_equal,
    quantity_at_most,
    integer_subtract,
    convert_integer,
    decimal_subtract,
    integer_add,
    enum_greater,
    integer_negate,
    text_at_least,
    text_less,
    decimal_add,
    divide_euclidean,
    rational_add,
    decimal_round,
    text_equal,
    quantity_multiply,
    admit_nfkc,
    binary64_multiply,
    divide_bounded,
    text_at_most,
    binary64_divide,
    enum_at_most,
    binary64_total_order,
    enum_less,
    binary32_subtract,
    enum_not_equal,
    convert_decimal,
    quantity_less,
    divide_floor,
    admit_binary_utf8,
    quantity_subtract,
    rational_negate,
    quantity_divide,
    divide_truncating,
    admit_unicode_scalars,
    decimal_negate,
    rational_greater,
    admit_nfc,
    quantity_not_equal,
    admit_nfd,
    integer_divide,
    decimal_at_most,
    rational_subtract,
    decimal_divide,
    OracleStop,
};
use quire_contract_runtime::exact::{
    EnumValue, IllTyped, Integer, Meter, Outcome, Quantity, ScalarLimits, Text, TextPayload,
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
/// Generated `[0, 4]` admissions in `TextProfile::ALL` order.
type Admit = fn(&TextPayload, &mut Meter) -> Result<Outcome<Text>, OracleStop>;
const ADMISSIONS: [Admit; 6] = [
    admit_unicode_scalars,
    admit_nfc,
    admit_nfd,
    admit_nfkc,
    admit_nfkd,
    admit_binary_utf8,
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

// Deliberately untraced: this exists as `render_scalar`'s (`src/kani_obligations.rs`) F1
// mutation-testing vehicle, not to satisfy a numbered acceptance criterion. It runs the real
// generated oracle directly -- no Kani/CBMC involved -- to prove, by actual execution, that
// `quire.op.integer.add` over `[-1000,1000]` refuses an out-of-domain result rather than
// completing it: exactly the fact `render_scalar`'s rendered `sound` assertion now checks. The
// accompanying generator mutation (`integer_arithmetic_bound` in `src/exact_scalar.rs`, forcing
// its `Bounded` arm to also emit `None`) is not committed; it is a one-time proof, captured in
// this change's own review transcript, that removing the domain check makes this same call
// return `Completed(2000)` instead -- the input this test already knows must not complete.
#[test]
fn tc_026_integer_add_refuses_a_result_leaving_its_declared_domain() {
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

/// Trace: FR-014-AC-6, TC-024.
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
    assert_eq!(named, 11);
    println!("TC-192 three-way agreement: {vectors} generated vectors, {named} tabled tuples");
}

const COEFFICIENTS: [i64; 10] = [-25, -10, -7, -5, -1, 0, 1, 3, 8, 10];

/// Trace: FR-014-AC-6, TC-024.
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
    assert_eq!(named, 9);

    // Decimal ordering: runtime vs. generated oracle only (`agree2!`); see
    // this file's module doc for why the QSL authority leg is gone.
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
    assert_eq!(ordering, 30 * 30 + 2);
    println!(
        "TC-185 agreement: {vectors} generated and {named} tabled three-way, {ordering} ordering runtime-only"
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

/// Trace: FR-014-AC-6, TC-024.
#[test]
fn tc_024_tc186_text_and_enum_oracles_agree() {
    const T11: [u64; 10] = [0, 0, 0, 5, 3, 2, 0, 2, 6, 1];
    const T14: [u64; 10] = [0, 0, 0, 5, 3, 0, 0, 2, 3, 1];
    const T16: [u64; 10] = [0, 0, 0, 3, 2, 2, 0, 1, 5, 1];
    let mut admissions = 0_usize;
    for (p, admit) in ADMISSIONS.into_iter().enumerate() {
        for input in SEQUENCES {
            for tuple in [[u64::MAX; 10], T11, T14, T16] {
                agree2! {
                    limits: limits(tuple),
                    setup: { let input = payload(input); let target = text_type(0, 4, TextProfile::ALL[p]); },
                    direct: |m| admit_text(&input, &target, m),
                    generated: |m| done(admit(&input, m)),
                };
                admissions += 1;
            }
        }
    }
    assert_eq!(admissions, 6 * 12 * 4);

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
    assert_eq!(enums, 10 * 10 * 6 * 2);
    println!("TC-186 three-way agreement: {admissions} admissions, {comparisons} text comparisons, {enums} enum comparisons");
}

/// Trace: FR-014-AC-6, TC-024.
#[test]
fn tc_024_tc187_quantity_oracles_agree() {
    let families: [&[&str]; 2] = [
        &["m", "cm", "in", "rev", "m_alias"],
        &["K", "degC", "degF", "u1", "u2", "u3"],
    ];
    let values = [(0_i64, 1_i64), (1, 1), (-7, 3), (250, 1), (5463, 20)];
    let mut conversions = 0_usize;
    for family in families {
        for from in family {
            for to in family {
                for (n, d) in values {
                    agree2! {
                        limits: UNLIMITED,
                        setup: {
                            let f = fixture();
                            let (quantity, unit) = (f.q(ratio(n, d), from), f.unit(to));
                            let decimal = QuantityTarget::Decimal(decimal_type(-100_000, 100_000, 0, 2, RoundingMode::NearestEven));
                            let integer = QuantityTarget::Integer { domain: IntegerInterval::new(int(-1000), int(1000)).unwrap(), rounding: RoundingMode::TowardZero };
                        },
                        direct: |m| (
                            convert_quantity(&quantity, &unit, &decimal, m),
                            convert_quantity(&quantity, &unit, &integer, m),
                        ),
                        generated: |m| (
                            typed(convert_decimal(&quantity, &unit, m)),
                            typed(convert_integer(&quantity, &unit, m)),
                        ),
                    };
                    conversions += 2;
                }
            }
        }
    }
    assert_eq!(conversions, 2 * (5 * 5 + 6 * 6) * 5);

    let mut operations = 0_usize;
    for name in ["m", "cm", "in", "rev", "u2"] {
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
                operations += 4;
                for (o, compare) in QUANTITY_COMPARISONS.into_iter().enumerate() {
                    agree2! {
                        limits: UNLIMITED,
                        setup: { let f = fixture(); let (a, b) = (f.q(ratio(ln, ld), name), f.q(ratio(rn, rd), name)); },
                        direct: |m| compare_quantity(ComparisonOperator::ALL[o], &a, &b, m),
                        generated: |m| typed(compare(&a, &b, m)),
                    };
                    operations += 1;
                }
            }
            for exponent in ["-2", "-1", "0", "1", "2", "3"] {
                agree2! {
                    limits: UNLIMITED,
                    setup: { let f = fixture(); let (a, n) = (f.q(ratio(ln, ld), name), big(exponent)); },
                    direct: |m| evaluate_quantity(QuantityOperation::Power(&a, &n), m),
                    generated: |m| typed(quantity_power(&a, &n, m)),
                };
                operations += 1;
            }
        }
    }
    assert_eq!(operations, 5 * (5 * 5 * (4 + 6) + 5 * 6));

    // U19/U24 same-unit sums, U28 compound product, U13/U21 powers, U23
    // comparison tuples, and U03/U05 dimension mismatches.
    let mut named = 0_usize;
    for (a, b, tuple) in [
        ((2, "m"), (3, "m"), [3, 0, 0, 2, 5, 1]),
        ((2, "m"), (3, "m"), [3, 0, 0, 2, 4, 1]),
        ((1, "cm"), (2, "cm"), [2, 0, 0, 2, 5, 1]),
        ((1, "m"), (1, "s"), [0; 6]),
    ] {
        agree2! {
            limits: unit_tuple(tuple),
            setup: { let f = fixture(); let (a, b) = (f.qi(a.0, a.1), f.qi(b.0, b.1)); },
            direct: |m| evaluate_quantity(QuantityOperation::Add(&a, &b), m),
            generated: |m| typed(quantity_add(&a, &b, m)),
        };
        named += 1;
    }
    for tuple in [
        [19, 0, 2, 2, 11, 1],
        [12, 0, 2, 2, 11, 1],
        [18, 0, 2, 2, 11, 1],
    ] {
        agree2! {
            limits: unit_tuple(tuple),
            setup: { let f = fixture(); let (a, b) = (f.qi(1, "cm"), f.qi(1, "in")); },
            direct: |m| evaluate_quantity(QuantityOperation::Multiply(&a, &b), m),
            generated: |m| typed(quantity_multiply(&a, &b, m)),
        };
        named += 1;
    }
    for (value, exponent, tuple) in [
        (2, "18446744073709551616", [64, 0, 0, 1, 5, 1]),
        (0, "0", [1, 0, 0, 1, 4, 1]),
        (0, "-1", [1, 0, 0, 1, 1, 0]),
        (0, "-1", [1, 0, 0, 1, 0, 0]),
    ] {
        agree2! {
            limits: unit_tuple(tuple),
            setup: { let f = fixture(); let (a, n) = (f.qi(value, "m"), big(exponent)); },
            direct: |m| evaluate_quantity(QuantityOperation::Power(&a, &n), m),
            generated: |m| typed(quantity_power(&a, &n, m)),
        };
        named += 1;
    }
    for (a, b, tuple) in [
        ((1, "degC"), (2, "degC"), [13, 0, 2, 2, 9, 1]),
        ((1, "degC"), (2, "degC"), [13, 0, 2, 2, 8, 1]),
        ((0, "degC"), (32, "degF"), [0; 6]),
    ] {
        agree2! {
            limits: unit_tuple(tuple),
            setup: { let f = fixture(); let (a, b) = (f.qi(a.0, a.1), f.qi(b.0, b.1)); },
            direct: |m| compare_quantity(ComparisonOperator::Less, &a, &b, m),
            generated: |m| typed(quantity_less(&a, &b, m)),
        };
        named += 1;
    }
    assert_eq!(named, 14);
    println!("TC-187 three-way agreement: {conversions} conversions, {operations} operations, {named} tabled tuples");
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

/// Trace: FR-014-AC-6, TC-024.
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
    assert_eq!(named, 12);
    println!("TC-193 three-way agreement: {vectors} class-matrix vectors, {named} tabled tuples");
}

/// Trace: FR-014-AC-9, TC-024.
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

/// Trace: FR-014-AC-6, TC-024.
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
    assert_eq!(vectors, 41 + 7 * 41 * 41 + 39 + 5 * 39 * 39);
    println!("runtime-only agreement (no authority leg): {vectors} vectors");
}
