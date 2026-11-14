//! The enum scalar comparison harness (FR-049): a bounded Kani proof that quire-exact's
//! `compare_native_enum` over two `NativeEnum` carriers agrees with the native operator over the
//! operands' ranks.
//!
//! The declaration comes from the owner route: the admitted context supplies the `EnumShape` and
//! the checked members, and this module takes them as supplied. Generation runs `from_member`
//! over that shape and each checked member, so a minted identity yields no operand. Inside the
//! proof an operand is `from_index(tag, ordered, index, count)` over one symbolic `i64`
//! constrained to `0..count`; the proof holds no variant list and no heap value.

use qsl_replay::AdmittedContext;
use quire_contract_model::{CheckedNodeId, CheckedPackageV2, CheckedTerm};
use quire_exact::{
    compare_native_enum, ComparisonOperator, EnumMember, EnumShape, IllTypedCause, Meter,
    NativeEnum, NativeEnumRefusal, Outcome,
};

use super::{
    outcome::UnsupportedObligation,
    record::{harness_path, record},
    render,
    spec::{Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement},
};
use crate::{
    core::artifact::Artifact,
    kani::abi::{KaniPrimitiveType, KaniSolver, SCALAR_HARNESS_LIMITS},
    kani::identity::{
        EnumScalarIdentity, KaniScalarObligationHarness, ProofCeilings,
        ScalarIntegerRepresentation, ScalarObligationArgument, ScalarObligationIdentity,
    },
    oracle::checked_graph::CheckedGraph,
    oracle::equality::admitted::{owner_enum, OwnerEnum, OwnerEnumRefusal},
    oracle::scalar::EXACT_SCALAR_CRATE_NAME,
};

/// One enum comparison to prove.
pub struct EnumScalarRequest<'a> {
    /// The original checked package the comparison node belongs to.
    pub source: &'a CheckedPackageV2,
    /// The owner route's admitted context for that package, when one accompanies the request.
    pub context: Option<&'a AdmittedContext>,
    /// The `quire.op.enum.*` application node.
    pub node_id: &'a CheckedNodeId,
    /// Resource ceilings the backend must run under.
    pub ceilings: ProofCeilings,
    /// The loop unwind bound of the harness.
    pub unwind: u32,
    /// The routed item's QSL occurrence; `None` for a direct request.
    pub requirement_occurrence: Option<&'a crate::core::requirement::RequirementOccurrence>,
    /// Whether the routed item carries a path condition, which this row does not lower.
    pub has_path_condition: bool,
    /// This harness's position among the enum harnesses of one request, which keeps names unique.
    pub ordinal: usize,
}

/// The settled disposition of one enum comparison.
#[derive(Debug)]
pub enum EnumScalarOutcome {
    /// A harness was generated.
    Supported(Box<EnumScalarHarness>),
    /// The item is refused with no harness.
    Refused(EnumScalarRefusal),
}

/// A generated enum scalar harness and the crate manifest it builds under.
#[derive(Debug)]
pub struct EnumScalarHarness {
    /// The harness, as every scalar harness execution reads it.
    pub harness: KaniScalarObligationHarness,
    /// The artifacts besides the harness a crate needs: its `Cargo.toml`.
    pub artifacts: Vec<Artifact>,
}

/// Why an enum comparison has no harness.
#[derive(Debug)]
pub enum EnumScalarRefusal {
    /// The node is not an enum comparison; its family keeps its own disposition.
    NotEnumComparison,
    /// The comparison is ill-typed; the cause is quire-exact's own.
    IllTyped {
        /// The ill-typed cause `compare_native_enum` returned.
        cause: IllTypedCause,
    },
    /// The owner route does not supply an exact finite encoding of the declaration.
    Unsupported {
        /// The capability the item needs.
        capability: EnumCapability,
        /// The enum the item names.
        enum_name: String,
        /// The owner's reason.
        detail: String,
    },
    /// A checked member is not the owner-admitted shape's member at its position.
    MemberNotInShape,
    /// The shape handed to the carrier builder is not the owner-admitted shape.
    ShapeNotOwnerAdmitted,
}

impl EnumScalarRefusal {
    /// The unsupported disposition of a refused enum comparison, retaining its typed cause.
    pub(crate) fn into_unsupported(self) -> UnsupportedObligation {
        let (cause, enum_name, detail) = match self {
            Self::NotEnumComparison => (
                "not_enum_comparison".to_owned(),
                String::new(),
                String::new(),
            ),
            Self::IllTyped { cause } => {
                (format!("ill_typed.{cause:?}"), String::new(), String::new())
            }
            Self::Unsupported {
                capability,
                enum_name,
                detail,
            } => (capability.name().to_owned(), enum_name, detail),
            Self::MemberNotInShape => (
                "member_not_in_shape".to_owned(),
                String::new(),
                String::new(),
            ),
            Self::ShapeNotOwnerAdmitted => (
                "shape_not_owner_admitted".to_owned(),
                String::new(),
                String::new(),
            ),
        };
        UnsupportedObligation::EnumComparisonNotRendered {
            cause,
            enum_name,
            detail,
        }
    }
}

/// The named capability an unsupported enum item lacks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnumCapability {
    /// The owner route supplied no admitted context for the declaration.
    OwnerAdmittedContext,
    /// The owner route did not admit the declaration's complete member list and ordered flag.
    OwnerAdmittedDeclaration,
    /// The member count does not fit the carrier's `u32` rank.
    CarrierMemberCount,
    /// The routed item has a path condition, which an enum harness does not lower.
    PathCondition,
}

impl EnumCapability {
    /// The stable capability name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::OwnerAdmittedContext => "enum.owner_admitted_context",
            Self::OwnerAdmittedDeclaration => "enum.owner_admitted_declaration",
            Self::CarrierMemberCount => "enum.carrier_member_count",
            Self::PathCondition => "enum.path_condition",
        }
    }
}

/// The comparison operators of the enum family, with the native Rust operator each is proved
/// against.
const OPERATORS: [(&str, ComparisonOperator, &str); 6] = [
    ("quire.op.enum.eq", ComparisonOperator::Equal, "=="),
    ("quire.op.enum.ne", ComparisonOperator::NotEqual, "!="),
    ("quire.op.enum.lt", ComparisonOperator::Less, "<"),
    ("quire.op.enum.le", ComparisonOperator::LessOrEqual, "<="),
    ("quire.op.enum.gt", ComparisonOperator::Greater, ">"),
    ("quire.op.enum.ge", ComparisonOperator::GreaterOrEqual, ">="),
];

/// Settle one enum comparison: refuse it, or generate its harness.
pub fn generate_enum_scalar_harness(request: &EnumScalarRequest<'_>) -> EnumScalarOutcome {
    match settle(request) {
        Ok(harness) => EnumScalarOutcome::Supported(Box::new(harness)),
        Err(refusal) => EnumScalarOutcome::Refused(refusal),
    }
}

fn settle(request: &EnumScalarRequest<'_>) -> Result<EnumScalarHarness, EnumScalarRefusal> {
    settle_with(request, |owner| {
        (owner.shape.clone(), owner.members.clone())
    })
}

/// The shape and members the generator hands to `from_member` for `owner`'s declaration. Production
/// passes the owner's own; a test passes a mint to show the comparison with the owner shape catches it.
#[cfg(test)]
pub(crate) fn generate_with_mint(
    request: &EnumScalarRequest<'_>,
    mint: impl Fn(&OwnerEnum) -> (EnumShape, Vec<EnumMember>),
) -> EnumScalarOutcome {
    match settle_with(request, mint) {
        Ok(harness) => EnumScalarOutcome::Supported(Box::new(harness)),
        Err(refusal) => EnumScalarOutcome::Refused(refusal),
    }
}

/// The carrier member count of a declaration of `members` members.
fn carrier_count(members: usize, enum_name: &str) -> Result<u32, EnumScalarRefusal> {
    match u32::try_from(members) {
        Ok(0) => Err(EnumScalarRefusal::Unsupported {
            capability: EnumCapability::OwnerAdmittedDeclaration,
            enum_name: enum_name.to_owned(),
            detail: "the owner supplied no member".to_owned(),
        }),
        Ok(count) => Ok(count),
        Err(_) => Err(EnumScalarRefusal::Unsupported {
            capability: EnumCapability::CarrierMemberCount,
            enum_name: enum_name.to_owned(),
            detail: "the member count does not fit u32".to_owned(),
        }),
    }
}

fn settle_with(
    request: &EnumScalarRequest<'_>,
    mint: impl Fn(&OwnerEnum) -> (EnumShape, Vec<EnumMember>),
) -> Result<EnumScalarHarness, EnumScalarRefusal> {
    let graph = CheckedGraph::from_checked(request.source);
    let Some(CheckedTerm::Application {
        operation,
        arguments,
        ..
    }) = graph.term(request.node_id)
    else {
        return Err(EnumScalarRefusal::NotEnumComparison);
    };
    let identity = operation
        .identity()
        .ok_or(EnumScalarRefusal::NotEnumComparison)?;
    let (_, operator, native) = OPERATORS
        .iter()
        .find(|(candidate, ..)| *candidate == identity)
        .ok_or(EnumScalarRefusal::NotEnumComparison)?;
    let [left, right] = arguments.as_slice() else {
        return Err(EnumScalarRefusal::NotEnumComparison);
    };
    let operand_type = |term: &CheckedTerm| match term {
        CheckedTerm::Reference { target } => {
            graph.node(target).map(|node| node.semantic_type.clone())
        }
        CheckedTerm::Literal { type_id, .. } => Some(type_id.clone()),
        _ => None,
    };
    let (Some(left_type), Some(right_type)) = (operand_type(left), operand_type(right)) else {
        return Err(EnumScalarRefusal::NotEnumComparison);
    };
    if request.has_path_condition {
        return Err(EnumScalarRefusal::Unsupported {
            capability: EnumCapability::PathCondition,
            enum_name: format!("{left_type:?}"),
            detail: "an enum harness does not lower a path condition".to_owned(),
        });
    }
    let context = authenticated(request)?;
    let declaration = owner_enum(context, &graph, &left_type)
        .map_err(|refusal| unsupported(&left_type, &refusal))?;
    // A second declaration is admitted by the owner route in its own right; operands of different
    // declarations carry different tags, which `compare_native_enum` refuses as ill-typed.
    let other = if right_type == left_type {
        None
    } else {
        Some(
            owner_enum(context, &graph, &right_type)
                .map_err(|refusal| unsupported(&right_type, &refusal))?,
        )
    };
    let count = carrier_count(declaration.members.len(), &declaration.qualified.join("."))?;
    let (passed_shape, passed_members) = mint(&declaration);
    admit_carriers(&passed_shape, &passed_members, &declaration.shape, 0)?;
    check_typed(*operator, &declaration, other.as_ref(), count)?;
    render_harness(request, identity, *operator, native, &declaration, count)
}

/// The admitted context, once it is the owner's context for this very package.
fn authenticated<'a>(
    request: &EnumScalarRequest<'a>,
) -> Result<Option<&'a AdmittedContext>, EnumScalarRefusal> {
    let Some(context) = request.context else {
        return Ok(None);
    };
    let mismatch = |detail: &str| EnumScalarRefusal::Unsupported {
        capability: EnumCapability::OwnerAdmittedContext,
        enum_name: String::new(),
        detail: detail.to_owned(),
    };
    let bytes = crate::oracle::equality::encode_original_package(request.source)
        .map_err(|_| mismatch("the original package did not encode"))?;
    if context.package_name().to_string() != request.source.name()
        || context.package_bytes() != bytes.as_bytes()
    {
        return Err(mismatch(
            "the admitted context is not the owner's context for this package",
        ));
    }
    Ok(Some(context))
}

fn unsupported(declaration: &CheckedNodeId, refusal: &OwnerEnumRefusal) -> EnumScalarRefusal {
    let (capability, detail) = match refusal {
        OwnerEnumRefusal::NoContext => (
            EnumCapability::OwnerAdmittedContext,
            "no admitted context accompanied the request".to_owned(),
        ),
        OwnerEnumRefusal::NotAdmitted(detail) => {
            (EnumCapability::OwnerAdmittedDeclaration, detail.clone())
        }
    };
    EnumScalarRefusal::Unsupported {
        capability,
        enum_name: format!("{declaration:?}"),
        detail,
    }
}

/// Run `from_member` over `shape` and each checked member, in declared order, after requiring that
/// `shape` is the owner-admitted shape. A shape or member list the generator derived, renamed or
/// rebuilt is not the owner's, so a minted identity yields no carrier.
pub(super) fn admit_carriers(
    shape: &EnumShape,
    members: &[EnumMember],
    owner: &EnumShape,
    declaration: u64,
) -> Result<Vec<NativeEnum>, EnumScalarRefusal> {
    if shape != owner {
        return Err(EnumScalarRefusal::ShapeNotOwnerAdmitted);
    }
    members
        .iter()
        .map(|member| {
            NativeEnum::from_member(declaration, shape, member).map_err(|refusal| match refusal {
                NativeEnumRefusal::MemberNotInShape => EnumScalarRefusal::MemberNotInShape,
                NativeEnumRefusal::IndexOutOfRange => EnumScalarRefusal::Unsupported {
                    capability: EnumCapability::CarrierMemberCount,
                    enum_name: String::new(),
                    detail: "the member count does not fit u32".to_owned(),
                },
            })
        })
        .collect()
}

/// Ask `compare_native_enum` whether the operands are well typed. Its ill-typed cause is the
/// refusal; the carriers of the first member of each declaration suffice because the cause depends
/// on the declaration and the operator only.
fn check_typed(
    operator: ComparisonOperator,
    left: &OwnerEnum,
    right: Option<&OwnerEnum>,
    count: u32,
) -> Result<(), EnumScalarRefusal> {
    let carrier = |declaration: &OwnerEnum, tag: u64, count: u32| {
        NativeEnum::from_index(tag, declaration.shape.is_ordered(), 0, count).map_err(|_| {
            EnumScalarRefusal::Unsupported {
                capability: EnumCapability::CarrierMemberCount,
                enum_name: declaration.qualified.join("."),
                detail: "the declaration has no member".to_owned(),
            }
        })
    };
    let left_carrier = carrier(left, 0, count)?;
    let right_carrier = match right {
        None => left_carrier,
        Some(other) => {
            let other_count = u32::try_from(other.members.len()).unwrap_or(u32::MAX);
            carrier(other, 1, other_count)?
        }
    };
    let mut meter = Meter::new(SCALAR_HARNESS_LIMITS);
    match compare_native_enum(operator, &left_carrier, &right_carrier, &mut meter) {
        Err(error) => Err(EnumScalarRefusal::IllTyped { cause: error.cause }),
        Ok(Outcome::Completed(_)) => Ok(()),
        Ok(_) => Err(EnumScalarRefusal::Unsupported {
            capability: EnumCapability::OwnerAdmittedDeclaration,
            enum_name: left.qualified.join("."),
            detail: "the comparison did not complete at generation".to_owned(),
        }),
    }
}

fn render_harness(
    request: &EnumScalarRequest<'_>,
    operation_identity: &str,
    operator: ComparisonOperator,
    native: &str,
    declaration: &OwnerEnum,
    count: u32,
) -> Result<EnumScalarHarness, EnumScalarRefusal> {
    render_text(
        request,
        operation_identity,
        operator,
        native,
        declaration,
        count,
    )
    .map_err(|error| EnumScalarRefusal::Unsupported {
        capability: EnumCapability::OwnerAdmittedDeclaration,
        enum_name: declaration.qualified.join("."),
        detail: format!("the harness did not render: {error:?}"),
    })
}

fn render_text(
    request: &EnumScalarRequest<'_>,
    operation_identity: &str,
    operator: ComparisonOperator,
    native: &str,
    declaration: &OwnerEnum,
    count: u32,
) -> Result<EnumScalarHarness, UnsupportedObligation> {
    let ordered = declaration.shape.is_ordered();
    let expr = |text: String| ExprText::new(text);
    let stem = format!(
        "kob_scalar_{}",
        crate::kani::abi::readable_component(
            operation_identity
                .strip_prefix("quire.op.")
                .unwrap_or(operation_identity)
        )
    );
    let stem = if request.ordinal == 0 {
        stem
    } else {
        format!("{stem}_{}", request.ordinal)
    };
    let module_symbol = format!("{stem}_module");
    let harness_symbol = format!("{stem}_proof");
    let mut body = Vec::new();
    for name in ["left", "right"] {
        body.push(Statement::Input {
            name: name.to_owned(),
            ty: KaniPrimitiveType::I64,
            domain: (0, i128::from(count) - 1),
            then: vec![LetBinding {
                name: format!("{name}_carrier"),
                ty: None,
                mutable: false,
                init: Init::Expression(expr(format!(
                    "rt::NativeEnum::from_index(0_u64, {ordered}, {name} as u32, {count}_u32).expect(\"the index is below the member count\")"
                ))?),
            }],
        });
    }
    body.push(Statement::Let(LetBinding {
        name: "meter".to_owned(),
        ty: None,
        mutable: true,
        init: Init::ScalarMeter(SCALAR_HARNESS_LIMITS),
    }));
    body.push(Statement::Let(LetBinding {
        name: "operator".to_owned(),
        ty: None,
        mutable: false,
        init: Init::Expression(expr(format!("rt::ComparisonOperator::{operator:?}"))?),
    }));
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: "compare_native_enum".to_owned(),
        args: vec![
            Argument::Value("operator".to_owned()),
            Argument::Borrow("left_carrier".to_owned()),
            Argument::Borrow("right_carrier".to_owned()),
            Argument::BorrowMut("meter".to_owned()),
        ],
    });
    body.push(Statement::Let(LetBinding {
        name: "native".to_owned(),
        ty: None,
        mutable: false,
        init: Init::Expression(expr(format!("left {native} right"))?),
    }));
    body.push(Statement::Let(LetBinding {
        name: "sound".to_owned(),
        ty: None,
        mutable: false,
        init: Init::Expression(expr(
            "match &outcome { Ok(rt::Outcome::Completed(value)) => *value == native, _ => false }"
                .to_owned(),
        )?),
    }));
    body.push(Statement::Assert {
        condition: expr("sound".to_owned())?,
        message: format!(
            "the native enum comparison must complete with the native {operation_identity} result over ranks"
        ),
    });
    body.push(Statement::Let(LetBinding {
        name: "completed".to_owned(),
        ty: None,
        mutable: false,
        init: Init::Expression(expr(
            "matches!(outcome, Ok(rt::Outcome::Completed(_)))".to_owned(),
        )?),
    }));
    body.push(Statement::Forget("meter".to_owned()));
    body.push(Statement::Forget("outcome".to_owned()));
    let oracle_source =
        "use quire_exact as rt;\nuse quire_exact::compare_native_enum;\n".to_owned();
    let spec = HarnessSpec::new(
        format!(
            "enum-scalar {} `{operation_identity}`",
            super::clause::v2::message_safe_node_label(request.node_id)
        ),
        harness_path(&module_symbol, &harness_symbol)?,
        oracle_source,
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expr("completed".to_owned())?,
            message: "the native enum comparison Completed branch is reachable".to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = ScalarObligationIdentity {
        integer_representation: ScalarIntegerRepresentation::General,
        native_integer_selection: None,
        ceilings: request.ceilings,
        node_id: request.node_id.clone(),
        operation_identity: operation_identity.to_owned(),
        oracle_symbol: "quire_exact::compare_native_enum".to_owned(),
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        arguments: ["left", "right"]
            .iter()
            .map(|name| ScalarObligationArgument {
                identifier: (*name).to_owned(),
                minimum: 0,
                maximum: i64::from(count) - 1,
            })
            .collect(),
        inferred_operands: Vec::new(),
        requirement_occurrence: request.requirement_occurrence.cloned(),
        path_condition: Vec::new(),
        guard_inputs: Vec::new(),
        enum_scalar: Some(EnumScalarIdentity {
            declaration: declaration.node.clone(),
            qualified_name: declaration.qualified.clone(),
            ordered,
            members: declaration.cases.clone(),
            operator: operation_identity.to_owned(),
            encoding: "i64 index into the declared member order, one per operand".to_owned(),
        }),
        solver: spec.solver(),
        unwind: spec.unwind(),
        options: spec.options().to_vec(),
        witness_options: spec.witness_options().to_vec(),
    };
    let record = record(&module_symbol, &identity, &rust)?;
    let harness = KaniScalarObligationHarness::generated_with_scalar_limits(
        identity,
        rust,
        record,
        SCALAR_HARNESS_LIMITS,
    );
    Ok(EnumScalarHarness {
        harness,
        artifacts: vec![Artifact::new(
            "Cargo.toml",
            crate::core::profile::oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME),
        )],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use qsl_replay::{
        compile_admitted, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
    };
    use quire_contract_model::{
        CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2ReadResult,
    };
    use quire_exact::VariantId;

    const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
ordered enum Level { LOW, MID, HIGH }\n\
enum Color { RED, GREEN }\n\
function lt using v(a: Level, b: Level): Boolean pure { a < b }\n\
function same using v(a: Color, b: Color): Boolean pure { a = b }\n";

    /// The checked package, the owner-admitted `Level`, and the owner-admitted `Color`.
    fn owners() -> (OwnerEnum, OwnerEnum) {
        let compiled = compile_admitted(
            CompileUnit {
                source: SourceIdentity::new("test", "enum-scalar.native"),
                package_name: "enum-scalar".to_owned(),
                path: "enum-scalar.native",
                bytes: SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("the source compiles");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) else {
            panic!("IR admits the source");
        };
        let graph = CheckedGraph::from_checked(&package);
        let context = compiled.context();
        let resolve = |name: &str| {
            let admitted = context
                .enums()
                .find(|item| item.qualified_name().iter().map(|p| p.as_str()).eq([name]))
                .expect("declared enum");
            let node = package
                .resolved_graph()
                .nodes
                .iter()
                .find(|node| {
                    context.wire_graph().key(&node.node_id).ok().as_ref() == Some(admitted.key())
                })
                .expect("IR retains the declaration")
                .node_id
                .clone();
            owner_enum(Some(context), &graph, &node).expect("the owner admits the enum")
        };
        (resolve("Level"), resolve("Color"))
    }

    fn renamed(shape: &EnumShape) -> Vec<VariantId> {
        shape
            .variants()
            .map(|variant| {
                VariantId::new(variant.owner().clone(), format!("{}_X", variant.case()))
                    .expect("a non-empty variant name")
            })
            .collect()
    }

    #[trace("TC-063", "FR-049-AC-3")]
    #[test]
    fn owner_shape_and_members_yield_one_carrier_per_member_in_declared_order() {
        let (level, _) = owners();
        let carriers = admit_carriers(&level.shape, &level.members, &level.shape, 0).unwrap();
        assert_eq!(
            carriers.iter().map(NativeEnum::index).collect::<Vec<_>>(),
            [0, 1, 2]
        );
        assert!(carriers
            .iter()
            .all(|c| c.is_ordered() && c.member_count() == 3));
    }

    #[trace("TC-063", "FR-049-AC-3")]
    #[test]
    fn a_minted_member_identity_at_the_same_position_yields_no_carrier() {
        let (level, _) = owners();
        let minted = renamed(&level.shape)
            .into_iter()
            .zip(0..)
            .map(|(variant, rank)| EnumMember::new(variant, rank))
            .collect::<Vec<_>>();
        assert!(matches!(
            admit_carriers(&level.shape, &minted, &level.shape, 0),
            Err(EnumScalarRefusal::MemberNotInShape)
        ));
    }

    #[trace("TC-063", "FR-049-AC-3")]
    #[test]
    fn a_generator_mint_in_both_shape_and_members_is_not_the_owner_admitted_shape() {
        let (level, _) = owners();
        let variants = renamed(&level.shape);
        let shape = EnumShape::new(level.shape.is_ordered(), variants.clone());
        let members = variants
            .into_iter()
            .zip(0..)
            .map(|(variant, rank)| EnumMember::new(variant, rank))
            .collect::<Vec<_>>();
        // The minted pair is self-consistent: `from_member` alone would accept it.
        assert!(admit_carriers(&shape, &members, &shape, 0).is_ok());
        assert!(matches!(
            admit_carriers(&shape, &members, &level.shape, 0),
            Err(EnumScalarRefusal::ShapeNotOwnerAdmitted)
        ));
    }

    #[trace("TC-063", "FR-049-AC-3")]
    #[test]
    fn a_member_list_reordered_against_the_declaration_yields_no_carrier() {
        let (level, _) = owners();
        let mut variants = level.shape.variants().cloned().collect::<Vec<_>>();
        variants.swap(0, 2);
        let reordered = variants
            .into_iter()
            .zip(0..)
            .map(|(variant, rank)| EnumMember::new(variant, rank))
            .collect::<Vec<_>>();
        assert!(matches!(
            admit_carriers(&level.shape, &reordered, &level.shape, 0),
            Err(EnumScalarRefusal::MemberNotInShape)
        ));
    }

    #[trace("TC-063", "FR-049-AC-8")]
    #[test]
    fn ordering_an_unordered_declaration_and_distinct_declarations_are_ill_typed() {
        let (level, color) = owners();
        let cause = |operator, left: &OwnerEnum, right: Option<&OwnerEnum>| {
            let count = u32::try_from(left.members.len()).unwrap();
            match check_typed(operator, left, right, count) {
                Err(EnumScalarRefusal::IllTyped { cause }) => Some(cause),
                other => {
                    assert!(other.is_ok(), "{other:?}");
                    None
                }
            }
        };
        assert_eq!(
            cause(ComparisonOperator::Less, &color, None),
            Some(IllTypedCause::UnorderedEnumOrdering)
        );
        assert_eq!(
            cause(ComparisonOperator::Equal, &level, Some(&color)),
            Some(IllTypedCause::DistinctEnumDeclarations)
        );
        assert_eq!(cause(ComparisonOperator::Less, &level, None), None);
        assert_eq!(cause(ComparisonOperator::Equal, &color, None), None);
    }

    /// The compiled corpus: its checked package and the admitted compilation.
    fn corpus() -> (CheckedPackageV2, qsl_replay::AdmittedCompilation) {
        compile(SOURCE, "enum-scalar")
    }

    fn compile(source: &str, name: &str) -> (CheckedPackageV2, qsl_replay::AdmittedCompilation) {
        let compiled = compile_admitted(
            CompileUnit {
                source: SourceIdentity::new("test", "enum-scalar.native"),
                package_name: name.to_owned(),
                path: "enum-scalar.native",
                bytes: source.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("the source compiles");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) else {
            panic!("IR admits the source");
        };
        (*package, compiled)
    }

    fn comparison(package: &CheckedPackageV2, identity: &str) -> CheckedNodeId {
        package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                matches!(package.term(&node.node_id),
                    Some(CheckedTerm::Application { operation, .. })
                        if operation.identity() == Some(identity))
            })
            .expect("the comparison node")
            .node_id
            .clone()
    }

    fn request<'a>(
        package: &'a CheckedPackageV2,
        context: Option<&'a AdmittedContext>,
        node_id: &'a CheckedNodeId,
    ) -> EnumScalarRequest<'a> {
        EnumScalarRequest {
            source: package,
            context,
            node_id,
            ceilings: ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(1 << 30).unwrap(),
                wall_clock: std::time::Duration::from_secs(60),
            },
            unwind: 1,
            requirement_occurrence: None,
            has_path_condition: false,
            ordinal: 0,
        }
    }

    /// A generator that mints a key in both the shape and the members it hands to `from_member`
    /// is refused by the comparison with the owner-admitted shape, through the whole generation.
    #[trace("TC-063", "FR-049-AC-3")]
    #[test]
    fn a_generator_that_mints_a_key_in_shape_and_members_is_refused_end_to_end() {
        let (package, compiled) = corpus();
        let node = comparison(&package, "quire.op.enum.lt");
        let request = request(&package, Some(compiled.context()), &node);
        assert!(matches!(
            generate_with_mint(&request, |owner| (
                owner.shape.clone(),
                owner.members.clone()
            )),
            EnumScalarOutcome::Supported(_)
        ));
        let mint_both = |owner: &OwnerEnum| {
            let variants = renamed(&owner.shape);
            let members = variants
                .iter()
                .cloned()
                .zip(0..)
                .map(|(variant, rank)| EnumMember::new(variant, rank))
                .collect();
            (EnumShape::new(owner.shape.is_ordered(), variants), members)
        };
        assert!(matches!(
            generate_with_mint(&request, mint_both),
            EnumScalarOutcome::Refused(EnumScalarRefusal::ShapeNotOwnerAdmitted)
        ));
        let mint_members = |owner: &OwnerEnum| {
            let members = renamed(&owner.shape)
                .into_iter()
                .zip(0..)
                .map(|(variant, rank)| EnumMember::new(variant, rank))
                .collect();
            (owner.shape.clone(), members)
        };
        assert!(matches!(
            generate_with_mint(&request, mint_members),
            EnumScalarOutcome::Refused(EnumScalarRefusal::MemberNotInShape)
        ));
    }

    #[trace("TC-063", "FR-049-AC-8")]
    #[test]
    fn zero_members_and_a_count_beyond_u32_have_named_capabilities() {
        let capability = |result: Result<u32, EnumScalarRefusal>| match result {
            Err(EnumScalarRefusal::Unsupported { capability, .. }) => capability,
            other => panic!("expected unsupported, got {other:?}"),
        };
        assert_eq!(
            capability(carrier_count(0, "E")),
            EnumCapability::OwnerAdmittedDeclaration
        );
        assert_eq!(
            capability(carrier_count(
                usize::try_from(u64::from(u32::MAX) + 1).unwrap(),
                "E"
            )),
            EnumCapability::CarrierMemberCount
        );
        assert_eq!(carrier_count(3, "E").unwrap(), 3);
    }

    /// A declaration the owner route does not admit, and a context that is not the owner's for
    /// this package, are unsupported with their named capabilities and no harness.
    #[trace("TC-063", "FR-049-AC-8")]
    #[test]
    fn a_declaration_the_owner_does_not_admit_is_unsupported_with_a_named_capability() {
        let (package, compiled) = corpus();
        let graph = CheckedGraph::from_checked(&package);
        let integer = package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| &*node.semantic_form == "integer")
            .expect("the integer scalar type")
            .node_id
            .clone();
        let refusal = owner_enum(Some(compiled.context()), &graph, &integer).unwrap_err();
        assert!(matches!(
            unsupported(&integer, &refusal),
            EnumScalarRefusal::Unsupported {
                capability: EnumCapability::OwnerAdmittedDeclaration,
                ..
            }
        ));
        let (_, foreign) = compile(SOURCE, "other");
        let node = comparison(&package, "quire.op.enum.lt");
        let EnumScalarOutcome::Refused(EnumScalarRefusal::Unsupported { capability, .. }) =
            generate_enum_scalar_harness(&request(&package, Some(foreign.context()), &node))
        else {
            panic!("a foreign context is refused");
        };
        assert_eq!(capability, EnumCapability::OwnerAdmittedContext);
    }

    /// QSL admits neither an ordering over an unordered enum nor operands of different enums, so
    /// no source reaches CG with those ill-typed causes.
    #[trace("TC-063", "FR-049-AC-8")]
    #[test]
    fn qsl_refuses_the_ill_typed_sources_before_cg() {
        for body in ["a < b", "a = b"] {
            let source = format!(
                "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nenum Color {{ RED, GREEN }}\nordered enum Level {{ LOW, MID }}\nfunction f using v(a: {}, b: {}): Boolean pure {{ {body} }}\n",
                if body == "a < b" { "Color" } else { "Level" },
                "Color",
            );
            let compiled = compile_admitted(
                CompileUnit {
                    source: SourceIdentity::new("test", "ill.native"),
                    package_name: "ill".to_owned(),
                    path: "ill.native",
                    bytes: source.as_bytes(),
                },
                [],
                &DependencyInput::default(),
                StageLimits::default(),
                ReplayLimits::default(),
            );
            eprintln!("QSLILL {body}: ok={}", compiled.is_ok());
            assert!(compiled.is_err(), "{body}");
        }
    }
}
