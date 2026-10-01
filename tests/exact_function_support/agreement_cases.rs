//! FR-021-AC-2, AC-4, AC-5, AC-7, AC-9, AC-17: generated function-application
//! oracles agree with an independently assembled direct Contract Runtime
//! call; `InputRefusal` surfaces unchanged; the shared `MAX_CALL_DEPTH`
//! budget bounds a nested-call chain; a denial injected at `function.call`
//! yields `Outcome::Incomplete` with no charge applied; and the static
//! location map's `origin` half is confirmed against the runtime's own
//! `CheckRefusal`.
//!
//! This file is not part of the `it` binary. It is the `agreement` integration
//! test of a scratch crate that `tests/it/exact_function_agreement.rs` writes
//! at test time from the generator's current output: the crate itself is the
//! main FR-021 corpus, and `chain/lib.rs` beside it is the generator's output
//! for the >128-deep nested-call chain corpus. AC-17 needs the generator
//! itself and stays in `tests/it/exact_function_agreement.rs`.
//!
//! FR-021-AC-18 (three-way agreement with the QSL authority) is not
//! implemented here: this repository reaches QSL only through `qsl-replay`'s
//! public API, not the `quire_spec_language::value::expression` API AC-18
//! was written against (see `src/oracle/function/mod.rs`'s module doc). This
//! file's agreement legs are exactly two: the generated oracle and a direct
//! Contract Runtime call.

/// The generated main-corpus crate under test. `names.rs`, written beside the scratch crate's
/// manifest at test time, re-exports each executed oracle as `oracle_<function>`.
mod generated {
    pub use quire_exact_function_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

/// The generator's output for the nested-call chain corpus, written beside the
/// scratch crate's manifest, so AC-7 below executes the real generated `Call`
/// bodies rather than a hand-built parallel double.
#[allow(dead_code)] // the chain's own oracle is not called; AC-7 enters at `chain_0`.
mod chain_generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/chain/lib.rs"));
}

use quire_contract_runtime::exact as rt;
use std::num::NonZeroU64;

const UNLIMITED: rt::ScalarLimits = rt::ScalarLimits {
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
};

fn empty_objects() -> rt::ObjectEnvironment {
    let types =
        rt::TypeEnvironment::new(Vec::new(), core::iter::empty::<rt::ObjectTypeDeclaration>())
            .expect("empty declaration closure admits");
    rt::ObjectEnvironment::new(&types, core::iter::empty()).expect("no objects admits trivially")
}

/// A direct, independently assembled `PackageDeclarations` mirroring the
/// generated crate's own `add_fn`/`call_fn`/`eq_fn`/`unrelated_fn` -- built
/// fresh from real `Body` closures, never read back from the generated
/// source. Function insertion order here does not need to match the
/// generator's own `Origin::Body{index}` assignment: AC-2 compares
/// *outcomes and charges*, not indices (AC-3 covers indices, in
/// `exact_function_generation.rs`).
fn direct_package() -> rt::CheckedPackage {
    fn add_body() -> rt::Body {
        Box::new(
            |frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {
                let [rt::Value::Integer(left), rt::Value::Integer(right)] = args else {
                    return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);
                };
                match frame.meter(|meter| {
                    rt::evaluate_integer_arithmetic(
                        rt::IntegerArithmetic::Add(left, right),
                        None,
                        meter,
                    )
                }) {
                    Ok(rt::Outcome::Completed(value)) => {
                        rt::Outcome::Completed(rt::Value::Integer(value))
                    }
                    Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),
                    Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),
                    Ok(rt::Outcome::Incomplete(incomplete)) => rt::Outcome::Incomplete(incomplete),
                    Err(refusal) => rt::Outcome::Refused(refusal),
                    Ok(_) => unreachable!("Outcome gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                }
            },
        )
    }

    let functions = vec![
        rt::FunctionDeclaration {
            name: "add_fn".to_owned(),
            parameters: vec![
                ("a".to_owned(), rt::ValueType::Integer),
                ("b".to_owned(), rt::ValueType::Integer),
            ],
            result: rt::ValueType::Integer,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: add_body(),
        },
        rt::FunctionDeclaration {
            name: "unrelated_fn".to_owned(),
            parameters: vec![
                ("a".to_owned(), rt::ValueType::Integer),
                ("b".to_owned(), rt::ValueType::Integer),
            ],
            result: rt::ValueType::Integer,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: add_body(),
        },
        rt::FunctionDeclaration {
            name: "eq_fn".to_owned(),
            parameters: vec![
                ("a".to_owned(), rt::ValueType::Integer),
                ("b".to_owned(), rt::ValueType::Integer),
            ],
            result: rt::ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(
                |frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {
                    let [left, right] = args else {
                        return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);
                    };
                    let environment = rt::TypeEnvironment::new(
                        Vec::new(),
                        core::iter::empty::<rt::ObjectTypeDeclaration>(),
                    )
                    .expect("empty declaration closure admits");
                    let checked = environment
                        .check_equality(
                            rt::EqualityOperator::Equal,
                            rt::EqualityOperand::typed(rt::ValueType::Integer),
                            rt::EqualityOperand::typed(rt::ValueType::Integer),
                        )
                        .expect("Integer/Integer Equal admits");
                    match frame.meter(|meter| checked.evaluate(left, right, meter)) {
                        Ok(rt::Outcome::Completed(value)) => {
                            rt::Outcome::Completed(rt::Value::Boolean(value))
                        }
                        Ok(rt::Outcome::Undefined(undefined)) => rt::Outcome::Undefined(undefined),
                        Ok(rt::Outcome::Refused(refusal)) => rt::Outcome::Refused(refusal),
                        Ok(rt::Outcome::Incomplete(incomplete)) => {
                            rt::Outcome::Incomplete(incomplete)
                        }
                        Err(refusal) => rt::Outcome::Refused(refusal),
                        Ok(_) => unreachable!("Outcome gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                    }
                },
            ),
        },
        rt::FunctionDeclaration {
            name: "call_fn".to_owned(),
            parameters: vec![
                ("a".to_owned(), rt::ValueType::Integer),
                ("b".to_owned(), rt::ValueType::Integer),
            ],
            result: rt::ValueType::Integer,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(
                |frame: &rt::Frame, args: &[rt::Value]| -> rt::Outcome<rt::Value> {
                    frame.call("add_fn", args)
                },
            ),
        },
    ];
    rt::PackageDeclarations {
        types: rt::TypeEnvironment::new(
            Vec::new(),
            core::iter::empty::<rt::ObjectTypeDeclaration>(),
        )
        .expect("empty declaration closure admits"),
        functions,
    }
    .check(rt::CheckMode::Linked, rt::CheckingLimits::default())
    .expect("direct package admits")
}

/// Same comparison shape a generated oracle itself exposes: `package.call`
/// returns `Result<Evaluation, InputRefusal>`, and the generated oracle
/// maps that down to `Result<Outcome<Value>, InputRefusal>`
/// (`.map(|evaluation| evaluation.outcome)`, AC-16) before returning it --
/// so the direct leg here does the identical `.map` before formatting, or
/// the two Debug strings would differ on `Evaluation`'s `location`/`losses`
/// fields alone, which is not what AC-2 is asking to compare.
fn call_debug(package: &rt::CheckedPackage, function: &str, arguments: Vec<rt::Value>) -> String {
    let objects = empty_objects();
    let mut meter = rt::Meter::new(UNLIMITED);
    let outcome = package
        .call(function, arguments, &objects, &mut meter)
        .map(|evaluation| evaluation.outcome);
    format!("{outcome:?} consumed={:?}", meter.admitted_charges())
}

/// Trace: FR-021-AC-2, TC-031. The generated oracle's `Outcome<Value>` and
/// admitted charge sequence equal a direct `CheckedPackage::call` on an
/// independently assembled package, for the scalar (add_fn), equality
/// (eq_fn), and nested-call (call_fn) forms -- TC-031 step 1(a)'s
/// multi-form package.
#[test]
fn tc_031_ac2_generated_oracle_agrees_with_direct_runtime_call() {
    let direct = direct_package();
    let generated = generated::checked_package().expect("generated package admits");
    let objects = empty_objects();

    // add_fn(2, 3)
    {
        let args = vec![
            rt::Value::Integer(2i64.into()),
            rt::Value::Integer(3i64.into()),
        ];
        let direct_result = call_debug(&direct, "add_fn", args.clone());
        let mut meter = rt::Meter::new(UNLIMITED);
        let generated_result =
            generated::oracle_add_fn(
                &generated, args, &objects, &mut meter,
            );
        assert_eq!(
            format!(
                "{:?} consumed={:?}",
                generated_result,
                meter.admitted_charges()
            ),
            direct_result,
        );
    }

    // eq_fn(5, 5) and eq_fn(5, 6)
    for (a, b) in [(5i64, 5i64), (5i64, 6i64)] {
        let args = vec![rt::Value::Integer(a.into()), rt::Value::Integer(b.into())];
        let direct_result = call_debug(&direct, "eq_fn", args.clone());
        let mut meter = rt::Meter::new(UNLIMITED);
        let generated_result =
            generated::oracle_eq_fn(
                &generated, args, &objects, &mut meter,
            );
        assert_eq!(
            format!(
                "{:?} consumed={:?}",
                generated_result,
                meter.admitted_charges()
            ),
            direct_result,
        );
    }

    // call_fn(7, 8) -- nests into add_fn via Frame::call.
    {
        let args = vec![
            rt::Value::Integer(7i64.into()),
            rt::Value::Integer(8i64.into()),
        ];
        let direct_result = call_debug(&direct, "call_fn", args.clone());
        let mut meter = rt::Meter::new(UNLIMITED);
        let generated_result =
            generated::oracle_call_fn(
                &generated, args, &objects, &mut meter,
            );
        assert_eq!(
            format!(
                "{:?} consumed={:?}",
                generated_result,
                meter.admitted_charges()
            ),
            direct_result,
        );
    }
}

/// Trace: FR-021-AC-4, TC-031. `InputRefusal::Arity` and `::WrongValueKind`
/// are returned by the generated oracle unchanged, before any charge;
/// `Meter::admitted_charges()` is empty on both. `::UnknownFunction` has no
/// generated-oracle path at all (an item naming an undeclared function is
/// refused at generation time and emits no oracle -- see
/// `exact_function_generation.rs`'s AC-1 test), so it is driven here
/// directly against `CheckedPackage::call`, the same entry point every
/// generated oracle itself delegates to unchanged. `::DanglingReference` is
/// not exercised: the runtime's `validate_arguments`
/// (`quire-contract-runtime/src/exact/expression.rs`) checks
/// `value_type.admits(argument)` -- `WrongValueKind` -- before it ever
/// walks a value for a dangling reference, and a `Value::Reference`
/// supplied for this generator's Integer/Boolean-only parameter kinds
/// always fails that first check. `DanglingReference` is structurally
/// unreachable for any oracle this generator can produce (a
/// reference-typed parameter is refused at generation time, AC-10, blocked
/// on qsl#120), so no test here asserts it.
#[test]
fn tc_031_ac4_input_refusal_surfaces_unchanged_before_any_charge() {
    let generated = generated::checked_package().expect("generated package admits");
    let objects = empty_objects();

    // Arity: add_fn takes 2 arguments.
    {
        let mut meter = rt::Meter::new(UNLIMITED);
        let result =
            generated::oracle_add_fn(
                &generated,
                vec![rt::Value::Integer(1i64.into())],
                &objects,
                &mut meter,
            );
        assert!(matches!(
            result,
            Err(rt::InputRefusal::Arity {
                declared: 2,
                supplied: 1
            })
        ));
        assert!(meter.admitted_charges().is_empty());
    }

    // WrongValueKind: add_fn's first parameter is Integer, not Boolean.
    {
        let mut meter = rt::Meter::new(UNLIMITED);
        let result =
            generated::oracle_add_fn(
                &generated,
                vec![rt::Value::Boolean(true), rt::Value::Integer(1i64.into())],
                &objects,
                &mut meter,
            );
        assert!(matches!(
            result,
            Err(rt::InputRefusal::WrongValueKind { parameter: 0 })
        ));
        assert!(meter.admitted_charges().is_empty());
    }

    // A Reference value supplied for add_fn's Integer parameter fails
    // `value_type.admits(argument)` before the runtime ever walks it for a
    // dangling reference -- see this test's own doc comment for why
    // `DanglingReference` itself is untested here.
    {
        let mut meter = rt::Meter::new(UNLIMITED);
        let dangling = rt::ObjectReference::new(
            rt::UniverseIdentity::new(b"universe").expect("universe identity"),
            rt::NodeKey::from_bytes([7; 32]),
            rt::ObjectIdentity::new(b"object").expect("object identity"),
        );
        let result =
            generated::oracle_add_fn(
                &generated,
                vec![
                    rt::Value::Reference(dangling),
                    rt::Value::Integer(1i64.into()),
                ],
                &objects,
                &mut meter,
            );
        assert!(matches!(
            result,
            Err(rt::InputRefusal::WrongValueKind { parameter: 0 })
        ));
        assert!(meter.admitted_charges().is_empty());
    }

    // UnknownFunction: driven directly, since no generated oracle exists
    // for an undeclared name.
    {
        let mut meter = rt::Meter::new(UNLIMITED);
        let result = generated.call("does_not_exist_anywhere", Vec::new(), &objects, &mut meter);
        assert!(
            matches!(result, Err(rt::InputRefusal::UnknownFunction(name)) if name == "does_not_exist_anywhere")
        );
        assert!(meter.admitted_charges().is_empty());
    }
}

/// Trace: FR-021-AC-5, TC-031. Arity is decided before any per-argument
/// value-kind check: a call with both the wrong count *and* a wrong kind
/// among the supplied arguments reports `Arity`, never `WrongValueKind` --
/// matching FR-273-AC-2's own ordering, which the generated oracle inherits
/// unchanged by delegating wholesale to `CheckedPackage::call`.
#[test]
fn tc_031_ac5_arity_is_decided_before_value_kind() {
    let generated = generated::checked_package().expect("generated package admits");
    let objects = empty_objects();
    let mut meter = rt::Meter::new(UNLIMITED);
    let result =
        generated::oracle_add_fn(
            &generated,
            vec![rt::Value::Boolean(true)], // wrong arity AND wrong kind
            &objects,
            &mut meter,
        );
    assert!(matches!(
        result,
        Err(rt::InputRefusal::Arity {
            declared: 2,
            supplied: 1
        })
    ));
}

/// Trace: FR-021-AC-9, TC-031. A denial injected at the `function.call`
/// charge point yields `Outcome::Incomplete` naming that point, and the
/// denied charge is never applied.
#[test]
fn tc_031_ac9_denial_at_function_call_yields_incomplete_with_no_charge_applied() {
    let generated = generated::checked_package().expect("generated package admits");
    let objects = empty_objects();
    let denial = rt::InjectedDenial {
        point: rt::ChargePoint::FunctionCall,
        occurrence: NonZeroU64::new(1).unwrap(),
    };
    let mut meter = rt::Meter::new(UNLIMITED).with_injected_denial(denial);
    let result =
        generated::oracle_add_fn(
            &generated,
            vec![
                rt::Value::Integer(1i64.into()),
                rt::Value::Integer(2i64.into()),
            ],
            &objects,
            &mut meter,
        );
    match result {
        Ok(rt::Outcome::Incomplete(incomplete)) => {
            assert_eq!(incomplete.charge_point, rt::ChargePoint::FunctionCall);
        }
        other => panic!("expected Ok(Outcome::Incomplete(..)) naming function.call, got {other:?}"),
    }
    // The denied charge point is never applied: no charge was admitted.
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: FR-021-AC-7, TC-031. Re-entry through a chain of nested
/// `Frame::call`s is bounded by one `CheckingLimits::depth` budget
/// (`MAX_CALL_DEPTH` = 128) shared across the whole chain, refusing
/// `Refusal::CheckedInvariant` before any further charge once exceeded --
/// exercised against the generated `chain_generated` module above, so this
/// drives the *real* generated `Call` bodies (`frame.call(&next, args)`,
/// rendered by `render_body`) through `Frame::call`'s own depth enforcement,
/// not a hand-built parallel double standing in for them. The chain corpus's
/// shape (140 links, deeper than `MAX_CALL_DEPTH`) is asserted where it is
/// built, in `tests/it/exact_function_generation.rs`.
#[test]
fn tc_031_ac7_nested_call_chain_is_bounded_by_max_call_depth() {
    let package = chain_generated::checked_package().expect("generated chain package admits");
    let objects = empty_objects();
    let mut meter = rt::Meter::new(UNLIMITED);
    let result = package.call(
        "chain_0",
        vec![
            rt::Value::Integer(1i64.into()),
            rt::Value::Integer(2i64.into()),
        ],
        &objects,
        &mut meter,
    );
    match result {
        Ok(rt::Evaluation { outcome, .. }) => {
            assert!(
                matches!(outcome, rt::Outcome::Refused(rt::Refusal::CheckedInvariant)),
                "expected CheckedInvariant once MAX_CALL_DEPTH is exceeded, got {outcome:?}"
            );
        }
        Err(refusal) => panic!("expected a completed call reaching depth refusal, got {refusal:?}"),
    }
}
