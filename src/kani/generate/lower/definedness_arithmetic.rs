//! CG-owned checked arithmetic admission over IR's validated finite input and typed outcome.

use quire_contract_ir::kani::{
    DispatchIndex, KaniOutcomeKind, KaniProfile, SemanticFamily, ValidatedFiniteInput,
};
use quire_contract_model::{std001_code, NumericOperator};

use super::{admit_family, refusal, FamilyLoweringError};

/// An exact arithmetic request with an inclusive result domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedArithmeticRequest {
    /// Source clause identity.
    pub source_id: &'static str,
    /// Checked operation.
    pub operator: NumericOperator,
    /// Left operand.
    pub left: i128,
    /// Right operand.
    pub right: i128,
    /// Inclusive result minimum.
    pub minimum: i128,
    /// Inclusive result maximum.
    pub maximum: i128,
}

/// The admitted request and its exact result; this is not a Kani verdict.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArithmeticLowering {
    /// Original request.
    pub request: CheckedArithmeticRequest,
    /// Checked value in the selected domain.
    pub value: i128,
}

/// Prepares one defined arithmetic operation for generated artifacts.
pub fn prepare_checked_arithmetic(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    _input: &ValidatedFiniteInput,
    request: CheckedArithmeticRequest,
) -> Result<ArithmeticLowering, FamilyLoweringError> {
    admit_family(
        profile,
        dispatch,
        "checked-arithmetic",
        SemanticFamily::DefinednessArithmetic,
        request.source_id,
    )?;
    if request.minimum > request.maximum {
        return Err(refusal(
            KaniOutcomeKind::InvalidInput,
            std001_code!("kani_arithmetic_range_invalid"),
            request.source_id,
            profile,
        ));
    }
    let value = match request.operator {
        NumericOperator::Add => request.left.checked_add(request.right),
        NumericOperator::Subtract => request.left.checked_sub(request.right),
        NumericOperator::Multiply => request.left.checked_mul(request.right),
        NumericOperator::Divide | NumericOperator::Remainder if request.right == 0 => {
            return Err(refusal(
                KaniOutcomeKind::Refused,
                std001_code!("kani_definedness_nonzero_divisor"),
                request.source_id,
                profile,
            ));
        }
        NumericOperator::Divide => request.left.checked_div(request.right),
        NumericOperator::Remainder => request.left.checked_rem(request.right),
    };
    let Some(value) = value else {
        return Err(refusal(
            KaniOutcomeKind::Refused,
            std001_code!("kani_definedness_checked_range"),
            request.source_id,
            profile,
        ));
    };
    if !(request.minimum..=request.maximum).contains(&value) {
        return Err(refusal(
            KaniOutcomeKind::Refused,
            std001_code!("kani_definedness_checked_range"),
            request.source_id,
            profile,
        ));
    }
    Ok(ArithmeticLowering { request, value })
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, KaniOutcomeKind,
        KaniProfile, ModuleDescriptor, PopulationCompleteness, ProfileSelection, ResourceBounds,
        SemanticFamily,
    };
    use quire_contract_model::NumericOperator;

    use super::{prepare_checked_arithmetic, CheckedArithmeticRequest};

    /// Trace: TC-023.
    #[test]
    fn tc_023_division_by_zero_remains_a_typed_refusal() {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        let profile = KaniProfile::new(
            selection.clone(),
            vec![CapabilityEntry {
                construct: "checked-arithmetic".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "arithmetic".to_owned(),
                },
            }],
        )
        .expect("valid fixture profile");
        let dispatch = DispatchIndex::new(vec![ModuleDescriptor {
            module_id: "arithmetic".to_owned(),
            family: SemanticFamily::DefinednessArithmetic,
            abi_revision: "abi".to_owned(),
            constructs: vec!["checked-arithmetic".to_owned()],
        }])
        .expect("valid fixture dispatch");
        let input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 1,
                max_references: 0,
                max_input_bytes: 1,
            },
            input_bytes: 0,
            objects: vec![],
            references: vec![],
        }
        .validate()
        .expect("valid empty population");
        let outcome = prepare_checked_arithmetic(
            &profile,
            &dispatch,
            &input,
            CheckedArithmeticRequest {
                source_id: "source",
                operator: NumericOperator::Divide,
                left: 1,
                right: 0,
                minimum: 0,
                maximum: 2,
            },
        )
        .expect_err("zero divisor must refuse");
        let outcome = outcome.outcome().expect("IR outcome was built");
        assert_eq!(outcome.kind, KaniOutcomeKind::Refused);
        assert_eq!(outcome.code.as_str(), "kani_definedness_nonzero_divisor");
        assert_eq!(outcome.boolean_claim(), None);
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_checked_domain_admits_exact_values_and_refuses_outside_results() {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        let profile = KaniProfile::new(
            selection.clone(),
            vec![CapabilityEntry {
                construct: "checked-arithmetic".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "arithmetic".to_owned(),
                },
            }],
        )
        .expect("valid fixture profile");
        let dispatch = DispatchIndex::new(vec![ModuleDescriptor {
            module_id: "arithmetic".to_owned(),
            family: SemanticFamily::DefinednessArithmetic,
            abi_revision: "abi".to_owned(),
            constructs: vec!["checked-arithmetic".to_owned()],
        }])
        .expect("valid fixture dispatch");
        let input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 1,
                max_references: 0,
                max_input_bytes: 1,
            },
            input_bytes: 0,
            objects: vec![],
            references: vec![],
        }
        .validate()
        .expect("valid empty population");
        let admitted = prepare_checked_arithmetic(
            &profile,
            &dispatch,
            &input,
            CheckedArithmeticRequest {
                source_id: "source",
                operator: NumericOperator::Add,
                left: 1,
                right: 2,
                minimum: 0,
                maximum: 3,
            },
        )
        .expect("in-range sum is admitted");
        assert_eq!(admitted.value, 3);
        let refused = prepare_checked_arithmetic(
            &profile,
            &dispatch,
            &input,
            CheckedArithmeticRequest {
                source_id: "source",
                operator: NumericOperator::Add,
                left: 2,
                right: 2,
                minimum: 0,
                maximum: 3,
            },
        )
        .expect_err("outside result must not lower");
        let refused = refused.outcome().expect("IR outcome was built");
        assert_eq!(refused.code.as_str(), "kani_definedness_checked_range");
        assert_eq!(refused.boolean_claim(), None);
    }
}
