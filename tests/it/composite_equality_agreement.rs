//! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the
//! complementary-outcomes half): generated composite-equality oracles agree
//! with an independently assembled direct Contract Runtime call, and with
//! the pinned QSL authority, across record, tuple, option, collection and
//! recursive shapes; a malformed environment is refused with no charge; a
//! denied charge surfaces as `Outcome::Incomplete`, never a completed
//! Boolean; and the two operator variants over one expression node produce
//! complementary outcomes.
//!
//! The oracles under test are the committed golden `lib.rs`, which
//! `composite_equality_generation`'s AC-10 test proves is the generator's
//! current output, spliced in with `include!` exactly as
//! `exact_scalar_agreement.rs` does.

#[macro_use]
#[path = "../composite_equality_support/agreement.rs"]
mod support;

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `composite_equality_generation.rs` each pick independently,
// on the assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../composite_equality_support/package.rs"]
mod package;

#[allow(dead_code)] // not every generated helper is called by every vector.
mod generated {
    include!("../fixtures/composite_equality/lib.rs.golden");
}

use package::*;
use quire_contract_runtime::exact as rt;
use support::rt_side;

/// Trace: FR-018-AC-2, FR-018-AC-9, FR-018-AC-10, TC-029.
///
/// Every vector below is run under `UNLIMITED` limits, which lets `agree3!`
/// exercise a full, uncontested run against both the direct Contract Runtime
/// and the pinned QSL authority (AC-2) and, from the charges that run
/// actually admits, a denial of each one in turn (AC-9) in the same call:
/// `denials` discovers admitted charges dynamically rather than naming them.
///
/// Every one of the golden's 9 oracles is executed here (the `not_equal`
/// record, text and enum vectors close the gap AC-10's own defence had: a
/// mutated `EqualityOperatorKind::path()`, re-blessed, previously left every
/// AC-10 test green because 3 of 9 oracles were never run under AC-2).
#[test]
fn tc_029_ac2_and_ac9_record_tuple_option_collection_and_recursive_oracles_agree() {
    // Record (E_RECORD, `equal`): equal and unequal points.
    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_record();
                let left = record_point(&environment, lx, ly);
                let right = record_point(&environment, rx, ry);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(R_POINT)),
                operand_typed(composite_type(R_POINT)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_21af7ba1e91d21e90d1981308c350091506c953e7bc3a4dfa6a240083ad1a65b( // E_RECORD Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Tuple (E_TUPLE): equal, and unequal on each position.
    for (ln, lt, rn, rt_) in [
        (1, "abc", 1, "abc"),
        (1, "abc", 2, "abc"),
        (1, "abc", 1, "xyz"),
    ] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_tuple();
                let left = tuple_pair(&environment, ln, lt);
                let right = tuple_pair(&environment, rn, rt_);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(TUP_PAIR)),
                operand_typed(composite_type(TUP_PAIR)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_576f61c653038d5b860825b431f9128ca106118a261f1df432cbbc2a806a634a( // E_TUPLE Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Option (E_OPTION): both present equal, both present unequal, one absent.
    for (l, r) in [
        (Some(5), Some(5)),
        (Some(5), Some(6)),
        (Some(5), None),
        (None, None),
    ] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = option_int(l);
                let right = option_int(r);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(ValueType::option(ValueType::Integer)),
                operand_typed(ValueType::option(ValueType::Integer)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_89fc354672aa321d79f99516e767b60755c99fb1d86665198d4facbbe42e8009( // E_OPTION Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Collection (E_COLLECTION): equal, different length, same elements reordered.
    for (l, r) in [
        (vec![1_i64, 2, 3], vec![1, 2, 3]),
        (vec![1, 2, 3], vec![1, 2]),
        (vec![1, 2, 3], vec![3, 2, 1]),
    ] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_collection();
                let left = sequence_int(&l);
                let right = sequence_int(&r);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(ValueType::collection(sequence_type())),
                operand_typed(ValueType::collection(sequence_type())),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_b5507219bf292f5f3030b453e44db1cec20b0d45d02979b8e5d678d65872dc34( // E_COLLECTION Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Recursive (E_SELF): both leaves, and one leaf vs. one nested one level.
    for nested_right in [false, true] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_self();
                let leaf = record_self(&environment, FieldValue::Absent);
                let nested = record_self(&environment, FieldValue::Present(record_self(&environment, FieldValue::Absent)));
                let left = leaf.clone();
                let right = if nested_right { nested } else { leaf };
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(R_SELF)),
                operand_typed(composite_type(R_SELF)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_3152c7e10a22a6fda8cb49d358d1baadc2e7fd1c6542192baaee7c8aa8add24a( // E_SELF Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Nested composite (E_PAIR_OF_POINTS): equal and unequal pairs of records.
    for (la, lb, ra, rb) in [
        ((1, 2), (3, 4), (1, 2), (3, 4)),
        ((1, 2), (3, 4), (1, 2), (5, 6)),
    ] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_pair_of_points();
                let left = record_pair_of_points(&environment, record_point(&environment, la.0, la.1), record_point(&environment, lb.0, lb.1));
                let right = record_pair_of_points(&environment, record_point(&environment, ra.0, ra.1), record_point(&environment, rb.0, rb.1));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(R_PAIR_OF_POINTS)),
                operand_typed(composite_type(R_PAIR_OF_POINTS)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_d5c77d9a6fbf3c0c86bf60c5d285814883b8fa70fa2357e8d411699fae179b7c( // E_PAIR_OF_POINTS Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Record (E_RECORD, `not_equal`): the golden's second operator variant
    // over the same node id. Without this vector, mutating
    // `EqualityOperatorKind::path()` so `NotEqual` emits
    // `rt::EqualityOperator::Equal` and re-blessing the golden left every
    // AC-10 test green (FR-018-AC-10's own defect report): this oracle was
    // never executed under AC-2, only referenced by AC-11's complementary
    // check.
    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_record();
                let left = record_point(&environment, lx, ly);
                let right = record_point(&environment, rx, ry);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::NotEqual,
                operand_typed(composite_type(R_POINT)),
                operand_typed(composite_type(R_POINT)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_460db43031e0e8349e7b81afaab7700861e42478fa36bed7705e229477212aca( // E_RECORD NotEqual
                &environment, &left, &right, g,
            ),
        };
    }

    // Text (E_TEXT, `equal`): equal and unequal short strings within
    // BD_TEXT's bound. Previously referenced by nothing (module doc gap).
    for (lt, rt_) in [("abc", "abc"), ("abc", "xyz")] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = text_value(lt);
                let right = text_value(rt_);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(ValueType::Text(TextType::new(0, 16, TextProfile::Nfc).unwrap())),
                operand_typed(ValueType::Text(TextType::new(0, 16, TextProfile::Nfc).unwrap())),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_76d9cb71c7ab24969e6bb32d337ce648be691bb8172a036fa8344def2c9b7384( // E_TEXT Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Enum (E_ENUM, `equal`): equal and unequal members of the vendored
    // `Example.Status` declaration. Previously referenced by nothing (module
    // doc gap).
    for (l, r) in [("READY", "READY"), ("READY", "DONE")] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let declaration = enum_status();
                let left = declaration.value(l);
                let right = declaration.value(r);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(enum_status_type()),
                operand_typed(enum_status_type()),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_1605f3edda95d8c8720bd2effad6370ef6aac509862734b5720a75e4836d7328( // E_ENUM Equal
                &environment, &left, &right, g,
            ),
        };
    }
}

/// Trace: FR-018-AC-2, TC-029.
///
/// The one `converted` execution vector (FR-018-AC-2's disclosed gap):
/// E_CONV's left operand is `convert<Integer>(e)` for `e: Int[-100, 100]`
/// (the same pair FR-018-AC-3's generation-time test already uses), and the
/// right operand is untouched `Integer`. `direct_equality` builds this from
/// the request's own descriptor, independently of the generated source, so a
/// generator mutation that cross-wires which operand's conversion target
/// lands on which operand — FR-018-AC-2's "apply the right operand's
/// conversion before the left's" mutation row — makes the generated leg
/// disagree with the direct and authority legs rather than passing by
/// construction.
///
/// Full coverage of every `admits_equality_conversion` row (TC-029 step 1)
/// is out of scope for this vector; it demonstrates the harness can express
/// a `converted` operand at all, which it could not before this test. It
/// proves the emitted *typing* of a converted operand is right, not that
/// conversion metering is exercised: `Int[-100, 100] -> Integer` is a free
/// reinterpretation with no `Meter` call
/// (`operand_value`'s `(Integer | Int(_), Integer)` arm), so it admits no
/// conversion charge point at all. `tc_029_ac9_a_converted_operand_denies_its_own_conversion_charges`
/// below adds a vector that does.
#[test]
fn tc_029_ac2_a_converted_operand_agrees() {
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = Value::Integer(Integer::from(l));
                let right = Value::Integer(Integer::from(r));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Int(IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64)).unwrap()),
                    ValueType::Integer,
                ),
                operand_typed(ValueType::Integer),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_1693eb17de43a7d163afa716008703d09001b2a464938fa1755f11b0225b7a74( // E_CONV Equal
                &environment, &left, &right, g,
            ),
        };
    }
}

/// Trace: FR-018-AC-9, TC-029.
///
/// FR-018-AC-9 names "each conversion charge point in turn"; the vector
/// above admits none, so this one converts `Int[-100, 100]` to
/// `Decimal[-100, 100; 0, 0]`, which charges `DecimalOperands`,
/// `DecimalScaleExpansion`, `DecimalArithmetic` and `DecimalResultRetain`
/// (`operand_value`'s `Decimal` target arm). `agree3!` denies each of those
/// in turn, alongside the plan charges, across all three legs, comparing
/// every `LimitKind` counter of the run each denial stops — not only
/// `ResultUnits` — so a counter two of the three implementations get wrong
/// in the same denied run cannot pass silently.
#[test]
fn tc_029_ac9_a_converted_operand_denies_its_own_conversion_charges() {
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let target = DecimalType::new(
                    Integer::from(-100_i64), Integer::from(100_i64), 0, 0, RoundingMode::NearestEven,
                ).unwrap();
                let environment = environment_option();
                let left = Value::Integer(Integer::from(l));
                let right = Value::Decimal(Decimal::new(Integer::from(r), 0));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Int(IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64)).unwrap()),
                    ValueType::Decimal(target.clone()),
                ),
                operand_typed(ValueType::Decimal(target.clone())),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_0b3ddf56cea05ad8cd36a18946c412690addf723b7cc414727d10325e5166a94( // E_CONV_CHARGE Equal
                &environment, &left, &right, g,
            ),
        };
    }
}

/// Trace: FR-018-AC-2, TC-029, codegen#83.
///
/// One vector per remaining `admits_equality_conversion` row (`Int -> *` is
/// already covered above by E_CONV/E_CONV_CHARGE): `Rational -> Rational`,
/// `Rational -> Integer` (the `Rational -> {Integer, Int, Decimal}` row,
/// exercised against `Integer`), `Decimal -> Rational`, `Decimal ->
/// Decimal`, and `Decimal -> Integer` (the `Decimal -> {Integer, Int}` row,
/// exercised against `Integer`). Each runs through `agree3!`, so the
/// generated leg, the direct runtime leg and the QSL authority leg all agree
/// on it -- and each is a distinct oracle node, so AC-2's "apply the right
/// operand's conversion before the left's" mutation row has a genuine
/// converted operand to cross-wire on every row, not only E_CONV's.
#[test]
fn tc_029_ac2_every_remaining_admits_equality_conversion_row_agrees() {
    // Rational[-10, 10; 1, 1] -> Rational[-100, 100; 1, 5].
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = rational_value(l, 1);
                let right = rational_value(r, 1);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    rational_type(-10, 10, 1, 1),
                    rational_type(-100, 100, 1, 5),
                ),
                operand_typed(rational_type(-100, 100, 1, 5)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_7f1f4b680dd0e405ecf8900e17a85b0170e5083d634a561c2f3456c097666a8f( // E_CONV_RAT_RAT Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Rational[-50, 50; 1, 1] -> Integer.
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = rational_value(l, 1);
                let right = Value::Integer(Integer::from(r));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(rational_type(-50, 50, 1, 1), ValueType::Integer),
                operand_typed(ValueType::Integer),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_bd33625ec0cccc3d86ecb0ef526f4daff58640350e42523bd5452b3b1669845d( // E_CONV_RAT_INT Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Decimal[-100, 100; 0, 0] -> Rational[-100, 100; 1, 5].
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = Value::Decimal(Decimal::new(Integer::from(l), 0));
                let right = rational_value(r, 1);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Decimal(
                        DecimalType::new(
                            Integer::from(-100_i64), Integer::from(100_i64), 0, 0,
                            RoundingMode::NearestEven,
                        ).unwrap(),
                    ),
                    rational_type(-100, 100, 1, 5),
                ),
                operand_typed(rational_type(-100, 100, 1, 5)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_7b17b989a86f0178fda1b973617c3ed5e20afe5a8f6bf99058777d1aa1970ebd( // E_CONV_DEC_RAT Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Decimal[-100, 100; 0, 0] -> Decimal[-1000, 1000; 0, 2].
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let target = DecimalType::new(
                    Integer::from(-1000_i64), Integer::from(1000_i64), 0, 2,
                    RoundingMode::NearestEven,
                ).unwrap();
                let environment = environment_option();
                let left = Value::Decimal(Decimal::new(Integer::from(l), 0));
                let right = Value::Decimal(Decimal::new(Integer::from(r), 0));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Decimal(
                        DecimalType::new(
                            Integer::from(-100_i64), Integer::from(100_i64), 0, 0,
                            RoundingMode::NearestEven,
                        ).unwrap(),
                    ),
                    ValueType::Decimal(target.clone()),
                ),
                operand_typed(ValueType::Decimal(target.clone())),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_c1ba86b67ce3b9390a347abd17c61969054dc776695ced70b08fa8e63db1ce73( // E_CONV_DEC_DEC Equal
                &environment, &left, &right, g,
            ),
        };
    }

    // Decimal[-100, 100; 0, 0] -> Integer.
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree3! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = Value::Decimal(Decimal::new(Integer::from(l), 0));
                let right = Value::Integer(Integer::from(r));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Decimal(
                        DecimalType::new(
                            Integer::from(-100_i64), Integer::from(100_i64), 0, 0,
                            RoundingMode::NearestEven,
                        ).unwrap(),
                    ),
                    ValueType::Integer,
                ),
                operand_typed(ValueType::Integer),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_a60a24bc6662909a92fcfe0df5b382d0002c9a1f1cfc41b556ae986a27632ec4( // E_CONV_DEC_INT Equal
                &environment, &left, &right, g,
            ),
        };
    }
}

/// Trace: FR-018-AC-8, TC-029.
///
/// The other half of AC-8 (a refused declaration closure carrying its
/// `DeclarationCause`) is `tc_029_ac8_a_duplicate_field_is_refused_with_its_declaration_cause`
/// in `composite_equality_generation.rs`. This half is the oracle's own
/// `check_type` guard: called under an environment that does not admit the
/// operand's comparison type, the oracle refuses before any charge rather
/// than panicking or completing a Boolean.
#[test]
fn tc_029_ac8_check_type_guards_the_oracle() {
    let good = rt_side::environment_record();
    let left = rt_side::record_point(&good, 1, 2);
    let right = rt_side::record_point(&good, 1, 2);

    for empty in [
        // An environment declaring nothing at all.
        rt::TypeEnvironment::new(Vec::new(), core::iter::empty::<rt::ObjectTypeDeclaration>())
            .unwrap(),
        // The option shape's environment: it declares no composite at all
        // (an option payload needs none), so it omits R_POINT's key, which
        // the record operand's comparison type reaches.
        rt_side::environment_option(),
        // TC-029 step 6's third malformed environment: the identical record
        // shape, but declared under a different node key than the one the
        // operand's comparison type reaches (R_PAIR_OF_POINTS's key, not
        // R_POINT's), so the reached key is still absent.
        rt::TypeEnvironment::new(
            vec![rt::CompositeDeclaration::new(
                rt::NodeKey::from_hex(&key(R_PAIR_OF_POINTS)).unwrap(),
                "R_POINT-under-a-different-key",
                rt::CompositeShape::Record(vec![
                    rt::FieldDeclaration::new("x", rt::ValueType::Integer, rt::Presence::Required),
                    rt::FieldDeclaration::new("y", rt::ValueType::Integer, rt::Presence::Required),
                ]),
            )],
            core::iter::empty::<rt::ObjectTypeDeclaration>(),
        )
        .unwrap(),
    ] {
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let outcome =
            generated::oracle_21af7ba1e91d21e90d1981308c350091506c953e7bc3a4dfa6a240083ad1a65b(
                // E_RECORD Equal
                &empty, &left, &right, &mut meter,
            );
        assert_eq!(outcome, rt::Outcome::Refused(rt::Refusal::CheckedInvariant));
        assert!(
            meter.admitted_charges().is_empty(),
            "check_type guard must admit no charge before refusing"
        );
    }
}

/// Trace: FR-018-AC-9, TC-029.
///
/// A direct, explicit check (independent of `agree3!`'s Debug-equality
/// comparison above) that a denied charge surfaces as `Outcome::Incomplete`
/// and never as a completed Boolean.
#[test]
fn tc_029_ac9_a_denied_charge_is_incomplete_never_a_completed_boolean() {
    let environment = rt_side::environment_record();
    let left = rt_side::record_point(&environment, 1, 2);
    let right = rt_side::record_point(&environment, 1, 2);
    let denied = rt_side::denials(rt_side::UNLIMITED, |meter| {
        generated::oracle_21af7ba1e91d21e90d1981308c350091506c953e7bc3a4dfa6a240083ad1a65b(
            // E_RECORD Equal
            &environment,
            &left,
            &right,
            meter,
        )
    });
    assert!(
        !denied.is_empty(),
        "the record oracle must admit at least one charge to deny"
    );
    for (point, occurrence, outcome, _) in denied {
        assert!(
            matches!(outcome, rt::Outcome::Incomplete(_)),
            "denying {point:?}#{occurrence} did not yield Outcome::Incomplete: {outcome:?}"
        );
    }
}

/// Trace: FR-018-AC-11, TC-029.
///
/// The generation-level half (distinct symbols, `caller_declared`, the
/// blocked-item mark) is covered by
/// `tc_029_ac11_two_operators_over_one_node_get_distinct_symbols_and_are_caller_declared`
/// in `composite_equality_generation.rs`. This is the execution half: the
/// two operator variants over the same expression node produce
/// complementary outcomes on a vector whose operands differ.
#[test]
fn tc_029_ac11_operator_variants_produce_complementary_outcomes() {
    let equal_env =
        generated::environment_21af7ba1e91d21e90d1981308c350091506c953e7bc3a4dfa6a240083ad1a65b()
            .unwrap();
    let not_equal_env =
        generated::environment_460db43031e0e8349e7b81afaab7700861e42478fa36bed7705e229477212aca()
            .unwrap();

    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        let left = rt_side::record_point(&equal_env, lx, ly);
        let right = rt_side::record_point(&equal_env, rx, ry);
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let equal_outcome =
            generated::oracle_21af7ba1e91d21e90d1981308c350091506c953e7bc3a4dfa6a240083ad1a65b(
                // E_RECORD Equal
                &equal_env, &left, &right, &mut meter,
            );
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let not_equal_outcome =
            generated::oracle_460db43031e0e8349e7b81afaab7700861e42478fa36bed7705e229477212aca(
                // E_RECORD NotEqual
                &not_equal_env,
                &left,
                &right,
                &mut meter,
            );
        match (equal_outcome, not_equal_outcome) {
            (rt::Outcome::Completed(equal), rt::Outcome::Completed(not_equal)) => {
                assert_ne!(
                    equal, not_equal,
                    "equal and not_equal must be complementary for ({lx},{ly}) vs ({rx},{ry})"
                );
            }
            other => panic!("expected both oracles to complete a Boolean, got {other:?}"),
        }
    }
}
