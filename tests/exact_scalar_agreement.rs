//! FR-014-AC-6: generated exact scalar oracles, direct Contract Runtime
//! execution and the QSL value authority agree on the QSpec TC-185, TC-186,
//! TC-187, TC-192 and TC-193 vectors, in outcome, every admitted charge, every
//! consumed counter, and the outcome of denying each admitted charge.
//!
//! The oracles are the committed golden `lib.rs`, which
//! `exact_scalar_generation` proves is the generator's current output. Each
//! oracle has a fixed operator and type, so a vector is an operand tuple and a
//! limit tuple taken from the TC's tabled or generated vectors.
//!
//! Integer arithmetic, rational arithmetic and ordering are checked against
//! direct runtime execution only (`agree2!`), and are counted separately.
//! The pinned authority is QSL 21c507e, which does now export these
//! operators (`order_numbers`, `evaluate_integer_arithmetic`,
//! `evaluate_rational_arithmetic`, QSL #119); these vectors stay
//! runtime-only because this crate has not yet added the `agree3!`
//! authority leg for them, not because the authority lacks the operator.

#[path = "exact_scalar_support/agreement.rs"]
#[macro_use]
mod support;

#[allow(dead_code)] // `OracleStop` variants are matched only where they can occur.
mod generated {
    include!("fixtures/exact_scalar/lib.rs.golden");
}

use generated::{
    oracle_039f121e76c968abdb029639896a532c898fc1652be1e4ccad6b07e0a2ff13ec as divide_euclidean, // code 1013
    oracle_03dfa554855dad3f941fe32721b324ce40c5c0d49d18da2fb788d213f0a7297e as rational_divide, // code 1032
    oracle_0588c927fc1e3cd76ad4936610a80aa865b40a993815a34e3c0637db077ee8fa as convert_decimal, // code 1086
    oracle_0979b4cf4ab9a40079cb667ef5d273dc280a9e317b4cb875af62a17a098fe8a8 as quantity_at_least, // code 1135
    oracle_0aa253cfbd1497e0597a14acc421fbef316a49ef47b4b2865da61392b128deb9 as integer_subtract, // code 1003
    oracle_0b285623dfb8c23cf8bce67a6c0288a986f49d1aa17d4f4be4e69bb5d573a88b as integer_less, // code 1041
    oracle_0c6911ff6e061c6e594476c169dd2badf9375d73b15064548fccd1b6c4d44047 as binary64_divide, // code 1062
    oracle_1416d80c94e64ba3e99b788fcb4e6e2cb3a4f68e2494859fc1d39bcd1106d5df as integer_at_least, // code 1045
    oracle_19c4db98ef4e75a17e4911d2c10ffe15b213119d79da54e9da0382ecbba581db as enum_at_least, // code 1125
    oracle_1d22a331e4928f0d5d8c67edbabbb308a39cd042c44b4020da97fbb1d8b4e937 as quantity_add, // code 1081
    oracle_2ae81bb9f99bc4020cb60eac561f0c6b51b0ecaa83269d7ac701dd2609d0b353 as binary32_numeric_equal, // code 1067
    oracle_2f1a2419ef0d6681996df514cb5542498596e232ad8bcccd8844d22b932b7867 as binary32_add, // code 1061
    oracle_31ba8be5c3bdf1f42d890b944c5e77537aa9514d08bc582ea21f4248dc5aa545 as enum_equal, // code 1121
    oracle_39b33a7e73ba0a00de7ff05c2179d87a8a585b9c8abca5bd379bf7e28836184e as admit_unicode_scalars, // code 1074
    oracle_3efc56a077e1cba5cb6242c15c9a3a3c15ffb1641ea1e2929731fb5155691a31 as admit_nfd, // code 1075
    oracle_42db23b4bcccb9302f9942237b74a3f08b9e876b055ef8de8e70f31e1826a3ca as admit_nfkc, // code 1076
    oracle_42dd93eff86aa8cfccaa5168679e1f116044cc14953959605133e851a3bc7ed8 as narrow_to_binary32, // code 1064
    oracle_4326980bf1f5e4329073989b87f08cd905c45b93baa74d0fc9c848abbefd7d65 as quantity_power, // code 1083
    oracle_47a3a2b2588b2df95bf54c2378e48b5cd9b08a5de8b49bfba2e57c01b4b653f8 as quantity_greater, // code 1134
    oracle_47b376074fc662359f371f2d293bda19f0d12deced9395c06ea8497ba29d2b5f as rational_add, // code 1031
    oracle_4ed07981c04dfb0373fcbef2ebe130f0bdb401bf65095add0438e0b93d9cd9cd as decimal_multiply, // code 1055
    oracle_52244a384ed3bbff2a449749cb5a4365b6e2302737adbedfdb531fd992d5f77d as binary64_bit_identical, // code 1068
    oracle_5352c057e3511ef4f051a97c7ae28502c07428fbfcbf9c07e70cb915fb02b452 as rational_negate, // code 1036
    oracle_571e5fabe2814590ee9404dbe9dc0649d5431e385ee08491f1dd58681d87b200 as rational_greater, // code 1043
    oracle_58a51a57ee8881d0a138b773828ed59a41034cbf1dabda6d3dbef4f7006d3b96 as binary32_subtract, // code 1065
    oracle_58e0beddd2a160e69f0107eeb47f336c57a6b85494717b5766c8ea3d08942b2b as quantity_equal, // code 1131
    oracle_58e715478854ee9d2aa1224cd7474d5df597245cc2d0928cdc8f7e6a7548f2c6 as quantity_at_most, // code 1133
    oracle_5b2826b130b1ca7b051ecf949158e4affdddc1cd9662966830fbcc1c49a4f6d4 as text_at_least, // code 1115
    oracle_5cdb8c024674c1df54ec48032898678b8060b35ebd7afa0b921a2e1305a0519d as rational_multiply, // code 1035
    oracle_5f61d8d0aa46097c2e48a1371659eb05f6c06d8cd236d41a59f59835d999cbe5 as divide_floor, // code 1012
    oracle_6bf6f9dec50f69449bb65eee6a7aee2280985580471bd1e3e01bae3076dd4f90 as enum_greater, // code 1124
    oracle_7116975042026662f32053f132326f99ac77c9f19fc821f643d399946a95b7ba as admit_nfkd, // code 1077
    oracle_743448b1d946d62a4dd71ba9a207108a5bd0c1c0296d73d89fdf2e6b4f7364a5 as text_not_equal, // code 1112
    oracle_8bb2ae8f3616df1355d6413dbaeb50a432eb8381dd0592a587247b3c4e819e33 as decimal_negate, // code 1056
    oracle_908706f2cdab4b62bc1d0265da8c292e3c383482a4944634f7998c7038207a33 as quantity_multiply, // code 1082
    oracle_9ab21858d0370ca58d201ca99820a62d763edf0759fe450266793539fadff7e1 as decimal_add, // code 1051
    oracle_9fdf3985c0b43f785cfefbbf6165e5f4ea1b352ace1255945312ba5c425cf3eb as decimal_subtract, // code 1054
    oracle_9ff019124f48ce642629f0407397e7105b21a789f3f7640bfe0a4574c5493088 as admit_binary_utf8, // code 1078
    oracle_a0de80d9f4f37ad7a773d1ff7c6c65b88a7e4e394728821c909276d18c57756e as enum_at_most, // code 1123
    oracle_a2ba447bad58ca0b042a0b34e40d3a5b99ba5f2a3c213a900f9a1169309284e2 as binary64_total_order, // code 1063
    oracle_a329df781d5237142015bd701f97883d5ed63c0611159e02a6926c370cc764b9 as binary64_multiply, // code 1066
    oracle_a330d3174b8007d333a6b6ef9ffb64fb1d1ca661e68bfa16bc4a97a22f02a4fd as decimal_divide, // code 1052
    oracle_a377c02012a91a9f94843b4cc2166c44076bf22086cd71aa978bc2ae696f1b19 as enum_less, // code 1073
    oracle_a4a9fc633e99b26af80e9a3ebd57b2466cb2b129b91e2d7a0a52b1a354aa9537 as admit_nfc, // code 1071
    oracle_a7c07e21cdd73648144d3aa814141edd22701b3d30945d960b12f43b34701e6b as integer_negate, // code 1002
    oracle_b16cf657ba381db0997d24f7a5153a0497a237c532cc5c912a4388ff645ff3fd as enum_not_equal, // code 1122
    oracle_ba0be80c1454160f54fdca2e457d511c63a7c90be68e62af06e0a47c0a94ae09 as text_less, // code 1072
    oracle_bceb553b994a11bb19607a0934084b740a5d2b0a24635863fbe9a4e0160b0dd1 as quantity_less, // code 1084
    oracle_bf1fba69e7bd5e8333c01e86706e924f267bd71b430acc8c7a76acff55486c37 as divide_bounded, // code 1014
    oracle_c0b9bb76f7c9fd1dbe32548b62594463029a344683d3e6d80d6ff81d46559521 as text_equal, // code 1111
    oracle_c79672f55bb623b60c68120b04ef09c3a9620c2a07a79906cce1ccd4cb6f861c as integer_at_most, // code 1044
    oracle_c9c225d70e630e9f1608f025e9a477aea350b69be50a071c4071e8de184575f4 as decimal_round, // code 1053
    oracle_c9cd0b5a50b22e6782495be9100904d87d0c0aaf9436d85746a3440c2dfe3406 as quantity_subtract, // code 1088
    oracle_c9f548caf1fa9f246ae373328eef7076c7057b32b3c29d1a510aecfa40e15870 as modulo_bounded, // code 1021
    oracle_cb6d89d9ccaeb994f56e9d6449948829dbc9aebc38dd9affaee93322016f6b78 as text_greater, // code 1114
    oracle_ccc188803f119d4957f6f77b6cd39518a78cf7598eec9ed711fc89f3960a83b1 as integer_divide, // code 1033
    oracle_d374553fd79167cd2c2c86a5dcf9517ea9546c82df0919477ef0e0de0cbf5b4b as divide_truncating, // code 1011
    oracle_d5bc5fbdaa1150e0cdaf5fd959392416c0b06ebcc671cffd721264de8c411047 as quantity_divide, // code 1089
    oracle_d62afb79208475a4cb4154dea2d2589032ddee90ff48e423fcfa735cac0b8381 as convert_integer, // code 1087
    oracle_dccfeaa6b01d129207d9720a411676ac18d973bd5dc5836b5214905651b14bd1 as text_at_most, // code 1113
    oracle_dead56fc7f2b2e0997fb0fec90105766fcd758d284f45e28a77ee6d98b2f8315 as integer_add, // code 1001
    oracle_ea7af6c97225cce45d8a07f5d97bf0ee2b894ae1c353ea9b53975bea93d3d15a as integer_multiply, // code 1004
    oracle_f2927ffb5f00452e5ec2284106a9acb247d9aefa287c5c4a335225ea98828031 as quantity_not_equal, // code 1132
    oracle_f66b4082cd53c8a163da6918dd73b6c8126ab4662cb8fa989b0902604aea13fb as rational_subtract, // code 1034
    oracle_fe900e8ad5d452ab6468556b17ca5077619df0bf2ee450c26d51b55c1602d5f2 as decimal_at_most, // code 1042
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
            agree3! {
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
            agree3! {
                limits: UNLIMITED,
                setup: { let (a, b) = (int(a), int(b)); let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(-5), int(5)).unwrap()); },
                direct: |m| divide(DivisionProfile::Truncating, &a, &b, &bounded, m),
                generated: |m| done(divide_bounded(&a, &b, m)),
            };
            agree3! {
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
        agree3! {
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
        agree3! {
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
        agree3! {
            limits: UNLIMITED,
            setup: { let a = dec(*ca, *sa); let target = decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway); },
            direct: |m| evaluate_decimal(DecimalOperation::Round(&a), &target, m),
            generated: |m| done(decimal_round(&a, m)),
        };
        agree3! {
            limits: UNLIMITED,
            setup: { let a = dec(*ca, *sa); let target = decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven); },
            direct: |m| evaluate_decimal(DecimalOperation::Negate(&a), &target, m),
            generated: |m| done(decimal_negate(&a, m)),
        };
        vectors += 2;
        for (cb, sb) in &operands {
            agree3! {
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
        agree3! {
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
        agree3! {
            limits: limits(tuple),
            setup: { let (a, b) = (dec(ca, sa), dec(cb, sb)); let target = decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero); },
            direct: |m| evaluate_decimal(DecimalOperation::Divide(&a, &b), &target, m),
            generated: |m| done(decimal_divide(&a, &b, m)),
        };
        named += 1;
    }
    assert_eq!(named, 9);

    // Decimal ordering: the pinned authority (QSL 21c507e) now exports
    // `OrderedOperands::Decimals` (QSL #119), so this is no longer a case of
    // no authority operator existing. It stays runtime-only (`agree2!`)
    // because this crate has not yet added the `agree3!` authority leg.
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
                agree3! {
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
                    agree3! {
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
        agree3! {
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
                            agree3! {
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
                    agree3! {
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
                agree3! {
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
                    agree3! {
                        limits: UNLIMITED,
                        setup: { let f = fixture(); let (a, b) = (f.q(ratio(ln, ld), name), f.q(ratio(rn, rd), name)); },
                        direct: |m| compare_quantity(ComparisonOperator::ALL[o], &a, &b, m),
                        generated: |m| typed(compare(&a, &b, m)),
                    };
                    operations += 1;
                }
            }
            for exponent in ["-2", "-1", "0", "1", "2", "3"] {
                agree3! {
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
        agree3! {
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
        agree3! {
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
        agree3! {
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
        agree3! {
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
            agree3! {
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
            agree3! {
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
        agree3! {
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
        agree3! {
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
        agree3! {
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
fn tc_024_operators_without_an_authority_agree_with_direct_runtime() {
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
    println!("runtime-only agreement (no agree3! authority leg yet): {vectors} vectors");
}
