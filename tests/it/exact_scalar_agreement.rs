//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime
//! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193
//! vectors, in outcome, every admitted charge, every consumed counter, and
//! the outcome of denying each admitted charge.
//!
//! The oracles are the committed golden `lib.rs`, which
//! `exact_scalar_generation` proves is the generator's current output. Each
//! oracle has a fixed operator and type, so a vector is an operand tuple and a
//! limit tuple taken from the TC's tabled or generated vectors.
//!
//! IR-254: every vector here runs `agree2!` (direct runtime vs. generated
//! oracle). A third, QSL value-authority leg (`agree3!`) used to run
//! alongside it; `../exact_scalar_support/agreement.rs`'s module doc records
//! why it was deleted rather than ported to the QSL revision this bumps to.

#[path = "../exact_scalar_support/agreement.rs"]
#[macro_use]
mod support;

#[allow(dead_code)] // `OracleStop` variants are matched only where they can occur.
mod generated {
    include!("../fixtures/exact_scalar/lib.rs.golden");
}

use generated::{
    oracle_08eb5d516b05b48856734299c7ff832ad4f68b2c9c5c8e28d5ae2725452a0851 as decimal_round, // code 1053
    oracle_0979b4cf4ab9a40079cb667ef5d273dc280a9e317b4cb875af62a17a098fe8a8 as quantity_at_least, // code 1135
    oracle_09800d0aad009e874e8f5dae4d53b631d09f50c9568a4aa4b71403487ef829ae as binary64_multiply, // code 1066
    oracle_19c4db98ef4e75a17e4911d2c10ffe15b213119d79da54e9da0382ecbba581db as enum_at_least, // code 1125
    oracle_1a528e2a6cbff097f4ebea8481b1b373906500144177935193b4cac8243367e8 as rational_subtract, // code 1034
    oracle_1d22a331e4928f0d5d8c67edbabbb308a39cd042c44b4020da97fbb1d8b4e937 as quantity_add, // code 1081
    oracle_1f3517af8733e49a567a9e074adb452949eb4c76f3f0d8118dcb3bd42d92ce4c as divide_floor, // code 1012
    oracle_2ae81bb9f99bc4020cb60eac561f0c6b51b0ecaa83269d7ac701dd2609d0b353 as binary32_numeric_equal, // code 1067
    oracle_2de8b4e02fef75d9ca9652abee57e420ff8150a6bd535a39c7aa07a4c0ef50c4 as modulo_bounded, // code 1021
    oracle_31ba8be5c3bdf1f42d890b944c5e77537aa9514d08bc582ea21f4248dc5aa545 as enum_equal, // code 1121
    oracle_3314c57c2dbf19fa312914ebb69732afc466912ff0a536da1d236350848046c1 as decimal_at_most, // code 1042
    oracle_339dad0b104be725d1554aaab15084d3ee7f9e392a202eff5094e81af29c3a86 as text_equal, // code 1111
    oracle_34e3167760b6256b159435437213ef99bb3008d3fcbaa3a62abeb06f2c232b38 as integer_divide, // code 1033
    oracle_3aeecc969a5cbe3efb7ca137ea0e94f98e19d97caa8eac550a2a22612409f099 as decimal_multiply, // code 1055
    oracle_40a2924f27a1a06be61c63fa7fc9fddfd1ef2caf34d829a80d22b6c64ccaa552 as integer_at_most, // code 1044
    oracle_47a3a2b2588b2df95bf54c2378e48b5cd9b08a5de8b49bfba2e57c01b4b653f8 as quantity_greater, // code 1134
    oracle_4881d33db12712b60c7fb95642827b1a5b0d3482383cc6c27b45dbc3a3ff8766 as rational_greater, // code 1043
    oracle_4bf5e05de79f7575a55fdff2704455a792bbaeba0b53f45b1e8d58d4fe741a02 as binary32_subtract, // code 1065
    oracle_4e52c120a34212736cec407ad35ebd053d45bb7bb01719d4eb23443998718218 as text_less, // code 1072
    oracle_4fd7d9c846f419fe3e571b8c5b3fbeaac7b2ecd1371c647579fd56c144f31564 as rational_divide, // code 1032
    oracle_52244a384ed3bbff2a449749cb5a4365b6e2302737adbedfdb531fd992d5f77d as binary64_bit_identical, // code 1068
    oracle_58e0beddd2a160e69f0107eeb47f336c57a6b85494717b5766c8ea3d08942b2b as quantity_equal, // code 1131
    oracle_58e715478854ee9d2aa1224cd7474d5df597245cc2d0928cdc8f7e6a7548f2c6 as quantity_at_most, // code 1133
    oracle_5937c7c8d41fb99522583d7f7713b7127de1025d18cb90427ffb470738658d5b as divide_truncating, // code 1011
    oracle_5d5501e82bc9abb8232c644c8dc6417e284df4de4905a366c5e3c335f644abe4 as decimal_subtract, // code 1054
    oracle_61934b838541031d77b98fca4f8ab58806ece25038715be3c65355b5c8aadfd3 as integer_at_least, // code 1045
    oracle_6b628cd4c20ad67f0c7921d53d18aa61bf40f9ed94cb40f686ef6bf59d54cb3b as integer_less, // code 1041
    oracle_6bf6f9dec50f69449bb65eee6a7aee2280985580471bd1e3e01bae3076dd4f90 as enum_greater, // code 1124
    oracle_6cc0e720ead1fff602d6cc9b92ecc0c34b80baf07563239645e83655872dc750 as rational_negate, // code 1036
    oracle_6d59babb09369322f471a07dafb1134b6175f9311392e7fc5883dfc5b7b10719 as rational_add, // code 1031
    oracle_6fa221ce7a3bd000c7c135e11fe32a4a3edefeff9802c9ffc9f7a3e726c51beb as text_greater, // code 1114
    oracle_7c4473ce8ba46c649cf546783a46291e1a9eccce3a8bf3735b19012e9c97aacd as binary32_add, // code 1061
    oracle_866cc8dd35af91e6f71645410137ecdb848c6d89c607ad1c74ba6f641ad54e51 as integer_subtract, // code 1003
    oracle_908706f2cdab4b62bc1d0265da8c292e3c383482a4944634f7998c7038207a33 as quantity_multiply, // code 1082
    oracle_983f74537b7d2dc7c879fecf913212294b55c86ba7ba7d32ed74f06c957c6ba4 as binary64_divide, // code 1062
    oracle_9dd795abc2b4ae969fcfc06143ff1dbb2b72a7ff5f30cbfa6f6b80340c8877e9 as decimal_divide, // code 1052
    oracle_9df4315fba041fb0207be64e79bd454f6dc0879caec000e28e5563d2c81b1bc7 as narrow_to_binary32, // code 1064
    oracle_a01d141e6af4478279c2d33de49bf6de559664401675d86ee667a9d5d6c25ca3 as text_at_least, // code 1115
    oracle_a0de80d9f4f37ad7a773d1ff7c6c65b88a7e4e394728821c909276d18c57756e as enum_at_most, // code 1123
    oracle_a2ba447bad58ca0b042a0b34e40d3a5b99ba5f2a3c213a900f9a1169309284e2 as binary64_total_order, // code 1063
    oracle_a377c02012a91a9f94843b4cc2166c44076bf22086cd71aa978bc2ae696f1b19 as enum_less, // code 1073
    oracle_ad54d638775c911ae73fa3a4d6c871ab96325153a2c7f75ec72db8ffa8afa545 as convert_decimal, // code 1086
    oracle_af3396f6babe7dfa143642de6ceb9de302088a253bf81bd6ae868a283b153880 as convert_integer, // code 1087
    oracle_b16cf657ba381db0997d24f7a5153a0497a237c532cc5c912a4388ff645ff3fd as enum_not_equal, // code 1122
    oracle_b1b9eb4b75c17615c3049e44e9d9ba411a76357d0f215ca61c0f6069c780e1e1 as text_at_most, // code 1113
    oracle_b6c4c488a6f6c69eb9547374c413f80f45d4d00f59d1a8e1446193f2fe82d437 as divide_euclidean, // code 1013
    oracle_bceb553b994a11bb19607a0934084b740a5d2b0a24635863fbe9a4e0160b0dd1 as quantity_less, // code 1084
    oracle_c9cd0b5a50b22e6782495be9100904d87d0c0aaf9436d85746a3440c2dfe3406 as quantity_subtract, // code 1088
    oracle_cae4029ee597fd4a07bd3b4b04e05d13c7e999d843db8ad0cdd16540dbbee76b as admit_nfkd, // code 1077
    oracle_cf05b13221bfc345b7c21c1995216a7ba6e5412bf12c193299b515e33781a5c6 as divide_bounded, // code 1014
    oracle_d463873f2d88e48f673860f11c3c06ef42aeb671de1ba58e5623de6ccb66db18 as admit_unicode_scalars, // code 1074
    oracle_d5bc5fbdaa1150e0cdaf5fd959392416c0b06ebcc671cffd721264de8c411047 as quantity_divide, // code 1089
    oracle_d9453619e02988b753e317579363a0d57f74a36953f18353fd4f0b28a9852014 as integer_add, // code 1001
    oracle_db0f20a30ed0a6687e6816b82404f5fbbe39ff543c406e55273faf73430d077a as decimal_negate, // code 1056
    oracle_e58152cf037217e004390e7ede00ed8ad29444867e10dedb11df9ac5351b81d4 as admit_nfkc, // code 1076
    oracle_edeb49d2e1ed26c7d0540389ad03c5e20049f11aaffd0a6752aed4e0ac4285e4 as decimal_add, // code 1051
    oracle_f11b7564500eb53d15b9ad310072266f100c53e0b452b665a4e25b304c2cd733 as admit_nfd, // code 1075
    oracle_f15287a740a285b180f83439d376a02972c44b417d164525076ae16bb146f4d9 as integer_negate, // code 1002
    oracle_f17752077cc598d1c29c1a24b80d9b7ca486ae389c56442514f27569308e8c51 as admit_nfc, // code 1071
    oracle_f1a09b02673ec9c1854843075872e6c71c557e59020825786c4f8fb9f4ab0ad7 as quantity_power, // code 1083
    oracle_f2927ffb5f00452e5ec2284106a9acb247d9aefa287c5c4a335225ea98828031 as quantity_not_equal, // code 1132
    oracle_f34d0bf1630a2cc21187d7cc591c0756fc4f1b1e5d8e4fbe82643f78b7f5c678 as text_not_equal, // code 1112
    oracle_f478e79124757f838c70dee68eaca3429710af7cdd631054585e8a089c499b29 as rational_multiply, // code 1035
    oracle_f637004aa7fbdfbd0a45002a43d541e0cf07321f98a58c22401a4d5284c0bb38 as admit_binary_utf8, // code 1078
    oracle_fa5c1e6a5ba44e1f69c8443c340badbe17dcccee1588d1db5494e0d55001236c as integer_multiply, // code 1004
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
