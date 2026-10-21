// SPDX-License-Identifier: AGPL-3.0-or-later
//! Original numeric domains and adverse vectors, with independently admitted source identities.
#[macro_use]
#[path = "../agreement.rs"]
mod support;
#[path = "../codes.rs"]
mod package;
#[path = "../authentic_native.rs"]
mod fixture;
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
    /// Private AC8 fault changes only the prepared environment, retaining its original context/index.
    pub(super) fn record_with_replaced_environment(
        environment: sv::TypeEnvironment, left: &exact::Value, right: &exact::Value, meter: &mut exact::Meter,
    ) -> exact::Outcome<bool> {
        let mut prepared = environment_e_record_equal(Some(super::support::context())).expect("original prepared capability");
        prepared.environment = environment;
        oracle_e_record_equal(&prepared, left, right, meter)
    }

}
use ix_trace_rs::trace;
use package::*;
use support::rt_side as rt;
use support::rt_side;

#[trace("FR-018-AC-2", "FR-018-AC-9", "FR-018-AC-10", "TC-029")]
#[test]
fn original_shape_vectors_record_0() {
    // Record (E_RECORD, `equal`): equal and unequal points.
    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        agree2! {
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
            generated: |g| generated::oracle_e_record_equal( // E_RECORD Equal
                &generated::environment_e_record_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

#[trace("FR-018-AC-2", "FR-018-AC-9", "FR-018-AC-10", "TC-029")]
#[test]
fn original_shape_vectors_record_6() {
    // Record (E_RECORD, `not_equal`): the second operator variant over the
    // same node id. Without this vector, mutating
    // `EqualityOperatorKind::path()` so `NotEqual` emits
    // `rt::EqualityOperator::Equal` leaves every generation-time test green
    // (FR-018-AC-10's own defect report): this oracle would be executed only
    // by AC-11's complementary check.
    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        agree2! {
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
            generated: |g| generated::oracle_e_record_not_equal( // E_RECORD NotEqual
                &generated::environment_e_record_not_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

#[trace("FR-018-AC-8", "TC-029")]
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
        // operand's comparison type reaches (the source's OtherPoint key, not
        // Point's), so the reached key is still absent.
        rt::TypeEnvironment::new(
            vec![rt::CompositeDeclaration::new(
                support::context().composites().find(|declaration| declaration.qualified_name().iter().map(|part| part.as_str()).eq(["OtherPoint"])).expect("genuine distinct admitted record").key(),
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
            generated::record_with_replaced_environment(
                empty, &left, &right, &mut meter,
            );
        assert!(matches!(
            outcome,
            rt::Outcome::Refused(rt::Refusal::CheckedInvariant {
                cause: quire_exact::CheckedInvariantCause::GeneratedTypeCheckRejected { .. },
            })
        ));
        assert!(
            meter.admitted_charges().is_empty(),
            "check_type guard must admit no charge before refusing"
        );
    }
}

#[trace("FR-018-AC-9", "TC-029")]
#[test]
fn tc_029_ac9_a_denied_charge_is_incomplete_never_a_completed_boolean() {
    let environment = rt_side::environment_record();
    let left = rt_side::record_point(&environment, 1, 2);
    let right = rt_side::record_point(&environment, 1, 2);
    let denied = rt_side::denials(rt_side::UNLIMITED, |meter| {
        generated::oracle_e_record_equal(
            // E_RECORD Equal
            &generated::environment_e_record_equal(Some(support::context())).expect("same original admitted context"),
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

#[trace("FR-018-AC-11", "TC-029")]
#[test]
fn tc_029_ac11_operator_variants_produce_complementary_outcomes() {
    let equal_env = generated::environment_e_record_equal(Some(support::context())).unwrap();
    let not_equal_env = generated::environment_e_record_not_equal(Some(support::context())).unwrap();

    for (lx, ly, rx, ry) in [(1, 2, 1, 2), (1, 2, 3, 4)] {
        let left = rt_side::record_point(&rt_side::environment_record(), lx, ly);
        let right = rt_side::record_point(&rt_side::environment_record(), rx, ry);
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let equal_outcome = generated::oracle_e_record_equal(
            // E_RECORD Equal
            &equal_env, &left, &right, &mut meter,
        );
        let mut meter = rt::Meter::new(rt_side::UNLIMITED);
        let not_equal_outcome = generated::oracle_e_record_not_equal(
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
