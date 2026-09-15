//! Definedness-preserving checked arithmetic admission for generated backends.

use quire_contract_ir::kani::{
    lower_checked_arithmetic, ArithmeticLowering, CheckedArithmeticRequest, DispatchIndex,
    KaniOutcome, KaniProfile, ValidatedFiniteInput,
};

/// Produces the native reviewed arithmetic plan before code generation may render an artifact.
///
/// Division by zero, checked overflow, invalid ranges, profile refusal, and dispatch mismatch are
/// returned as Contract IR typed non-Boolean outcomes; no generator may replace them with an
/// assumption or a partial artifact.
pub fn prepare_checked_arithmetic(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: CheckedArithmeticRequest,
) -> Result<ArithmeticLowering, KaniOutcome> {
    lower_checked_arithmetic(profile, dispatch, input, request)
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::{
        kani::{
            CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, KaniOutcomeKind,
            KaniProfile, ModuleDescriptor, PopulationCompleteness, ProfileSelection,
            ResourceBounds, SemanticFamily,
        },
        NumericOperator,
    };

    use super::{prepare_checked_arithmetic, CheckedArithmeticRequest};

    /// Trace: FR-007-AC-1, FR-007-AC-3, TC-023.
    #[test]
    fn tc_023_division_by_zero_remains_a_typed_refusal() {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            executable_digest: "exe".to_owned(),
            options_digest: "opts".to_owned(),
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
        assert_eq!(outcome.kind, KaniOutcomeKind::Refused);
        assert_eq!(outcome.code, "kani_definedness_nonzero_divisor");
        assert_eq!(outcome.boolean_claim(), None);
    }

    /// Trace: FR-007-AC-1, FR-007-AC-2, FR-007-AC-3, TC-023.
    #[test]
    fn tc_023_checked_domain_admits_exact_values_and_refuses_outside_results() {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            executable_digest: "exe".to_owned(),
            options_digest: "opts".to_owned(),
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
        assert_eq!(refused.code, "kani_definedness_checked_range");
        assert_eq!(refused.boolean_claim(), None);
    }
}
