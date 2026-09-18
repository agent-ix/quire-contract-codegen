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
#[path = "composite_equality_support/agreement.rs"]
mod support;

#[path = "composite_equality_support/package.rs"]
mod package;

#[allow(dead_code)] // not every generated helper is called by every vector.
mod generated {
    include!("fixtures/composite_equality/lib.rs.golden");
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
            generated: |g| crate::generated::oracle_15f4bacf0908fb59db407fd5451621cf30ff18f498af4c10999d959dffa2d9a6(
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
            generated: |g| crate::generated::oracle_c917767e5a48d3f6ba8f8804a76cdee231ca38dd3a2f340202f8bea2437e6021(
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
            generated: |g| crate::generated::oracle_19fe98bf2b61e3d287b91d96038ca3cfba585e6aa9d9fe396023e97f58252af7(
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
            generated: |g| crate::generated::oracle_de0bc9911bbaf25c8346ed1416cc325826cf3440884090ff975156bd744704a5(
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
            generated: |g| crate::generated::oracle_295074cf411f93dea95fe591f3e87ab4360e53bd096eeffe2e6999eae57f81e9(
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
            generated: |g| crate::generated::oracle_960fbf99330aa3db769521829ca3626509908d5088735094a70b5d09f94ab04a(
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
            generated: |g| crate::generated::oracle_d710d3e47cbf0de46401ae4200ee16178c4b09b5c42dde2a7932a5ab12eafc9f(
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
            generated: |g| crate::generated::oracle_6c3907cac3cc8b8def849dd9a7ccc97bd2e4965d6088a8d1e66073cfed623edf(
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
            generated: |g| crate::generated::oracle_e88ebbb852f2c82a1bd2db38e1b7341e7314f5dfa10716a9567dc7ae7e05420b(
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
/// a `converted` operand at all, which it could not before this test.
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
            generated: |g| crate::generated::oracle_ded0d4f3fd07be4748b614b6744395c6686310e7059138ab338bb6b798683f7c(
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
            generated::oracle_15f4bacf0908fb59db407fd5451621cf30ff18f498af4c10999d959dffa2d9a6(
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
        generated::oracle_15f4bacf0908fb59db407fd5451621cf30ff18f498af4c10999d959dffa2d9a6(
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
        generated::environment_15f4bacf0908fb59db407fd5451621cf30ff18f498af4c10999d959dffa2d9a6()
            .unwrap();
    let not_equal_env =
        generated::environment_d710d3e47cbf0de46401ae4200ee16178c4b09b5c42dde2a7932a5ab12eafc9f()
            .unwrap();

    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        let left = rt_side::record_point(&equal_env, lx, ly);
        let right = rt_side::record_point(&equal_env, rx, ry);
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let equal_outcome =
            generated::oracle_15f4bacf0908fb59db407fd5451621cf30ff18f498af4c10999d959dffa2d9a6(
                &equal_env, &left, &right, &mut meter,
            );
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let not_equal_outcome =
            generated::oracle_d710d3e47cbf0de46401ae4200ee16178c4b09b5c42dde2a7932a5ab12eafc9f(
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
