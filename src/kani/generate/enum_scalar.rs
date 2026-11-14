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
    oracle::equality::{owner_enum, OwnerEnum, OwnerEnumRefusal},
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

/// The named capability an unsupported enum item lacks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnumCapability {
    /// The owner route supplied no admitted context for the declaration.
    OwnerAdmittedContext,
    /// The owner route did not admit the declaration's complete member list and ordered flag.
    OwnerAdmittedDeclaration,
    /// The member count does not fit the carrier's `u32` rank.
    CarrierMemberCount,
}

impl EnumCapability {
    /// The stable capability name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::OwnerAdmittedContext => "enum.owner_admitted_context",
            Self::OwnerAdmittedDeclaration => "enum.owner_admitted_declaration",
            Self::CarrierMemberCount => "enum.carrier_member_count",
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
    let declaration = owner_enum(request.context, &graph, &left_type)
        .map_err(|refusal| unsupported(&left_type, &refusal))?;
    // A second declaration is admitted by the owner route in its own right; operands of different
    // declarations carry different tags, which `compare_native_enum` refuses as ill-typed.
    let other = if right_type == left_type {
        None
    } else {
        Some(
            owner_enum(request.context, &graph, &right_type)
                .map_err(|refusal| unsupported(&right_type, &refusal))?,
        )
    };
    let count =
        u32::try_from(declaration.members.len()).map_err(|_| EnumScalarRefusal::Unsupported {
            capability: EnumCapability::CarrierMemberCount,
            enum_name: declaration.qualified.join("."),
            detail: "the member count does not fit u32".to_owned(),
        })?;
    admit_carriers(
        &declaration.shape,
        &declaration.members,
        &declaration.shape,
        0,
    )?;
    check_typed(*operator, &declaration, other.as_ref(), count)?;
    render_harness(request, identity, *operator, native, &declaration, count)
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
                    "match rt::NativeEnum::from_index(0_u64, {ordered}, {name} as u32, {count}_u32) {{ Ok(carrier) => carrier, Err(_) => panic!(\"the index is below the member count\") }}"
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
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: "rt::compare_native_enum".to_owned(),
        args: vec![
            Argument::Value(format!("rt::ComparisonOperator::{operator:?}")),
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
    let oracle_source = "use quire_exact as rt;\n".to_owned();
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
        requirement_occurrence: None,
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
            crate::core::profile::oracle_crate_manifest(crate::EXACT_SCALAR_CRATE_NAME),
        )],
    })
}
