//! The IEEE float scalar harness (FR-053): a bounded Kani proof that quire-exact's
//! `compare_native_float` over two symbolic bit patterns agrees with an independent expression of
//! the same comparison, for the three comparisons `quire.op.ieee.numeric_equal`, `.total_order`
//! and `.bit_identical` at both widths.
//!
//! The supported set is the combinations whose quire-exact harnesses carry an executed Verified
//! verdict (FR-405 Behavior 15): the three comparisons. Every arithmetic identity is refused with a
//! named capability and no harness, because none of them has all three of its quire-exact
//! harnesses Verified. A float operand is one `i64` bit pattern; the carrier admits every
//! pattern, so the draw has no admission step and no bound is encoded into it.

use quire_contract_model::{CheckedNodeId, CheckedPackageV2, CheckedTerm};

use super::{
    outcome::{ObligationDisposition, UnsupportedObligation},
    record::{harness_path, record},
    render,
    spec::{Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement},
};
use crate::{
    core::artifact::Artifact,
    kani::abi::{KaniPrimitiveType, KaniSolver, SCALAR_HARNESS_LIMITS},
    kani::identity::{
        FloatScalarIdentity, KaniScalarObligationHarness, ProofCeilings,
        ScalarIntegerRepresentation, ScalarObligationArgument, ScalarObligationIdentity,
    },
    oracle::checked_graph::CheckedGraph,
    oracle::scalar::{
        lower::{operand_form, operand_own_bound},
        EXACT_SCALAR_CRATE_NAME,
    },
};

/// One float operation to prove.
pub struct FloatScalarRequest<'a> {
    /// The original checked package the operation node belongs to.
    pub source: &'a CheckedPackageV2,
    /// The `quire.op.ieee.*` application node.
    pub node_id: &'a CheckedNodeId,
    /// Resource ceilings the backend must run under.
    pub ceilings: ProofCeilings,
    /// The loop unwind bound of the harness.
    pub unwind: u32,
    /// The routed item's QSL occurrence; `None` for a direct request.
    pub requirement_occurrence: Option<&'a crate::core::requirement::RequirementOccurrence>,
    /// Whether the routed item carries a path condition, which this row does not lower.
    pub has_path_condition: bool,
    /// This harness's position among the float harnesses of one request, which keeps names unique.
    pub ordinal: usize,
}

/// The settled disposition of one float operation.
#[derive(Debug)]
pub enum FloatScalarOutcome {
    /// A harness was generated.
    Supported(Box<FloatScalarHarness>),
    /// The item is refused with no harness.
    Refused(FloatScalarRefusal),
}

/// A generated float scalar harness and the crate manifest it builds under.
#[derive(Debug)]
pub struct FloatScalarHarness {
    /// The harness, as every scalar harness execution reads it.
    pub harness: KaniScalarObligationHarness,
    /// The artifacts besides the harness a crate needs: its `Cargo.toml`.
    pub artifacts: Vec<Artifact>,
}

/// Why a float operation has no harness.
#[derive(Debug, Eq, PartialEq)]
pub enum FloatScalarRefusal {
    /// The node is not a float operation of this row; its family keeps its own disposition.
    NotFloatOperation,
    /// An operand's type reaches no `float_rounding` bound (FR-053-AC-3): `requires-bound`,
    /// naming the operand's type node, with no harness.
    RequiresBound {
        /// The operand's type node, which no `float_rounding` bound types.
        unbounded_type: CheckedNodeId,
    },
    /// The operation has no harness for the named capability.
    Unsupported {
        /// The capability the item lacks.
        capability: FloatCapability,
        /// The operation identity the item names.
        operation: String,
        /// The reason, naming the measured row where there is one.
        detail: String,
    },
}

/// The named capability an unsupported float item lacks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloatCapability {
    /// `add` has no exact finite Kani-tractable encoding: only its `nearest-even` harnesses
    /// returned a verdict (FR-405 Behavior 15).
    AddNotTractable,
    /// `sub`, as `add`.
    SubNotTractable,
    /// `mul`, as `add` at `binary32`; no `binary64` harness returned a verdict.
    MulNotTractable,
    /// `div`: no harness of any spelling returned a verdict, and Kani's own symbolic `f32`
    /// division is bit-wrong at `0x800000A9 / 0x9C873333`.
    DivNotTractable,
    /// `sqrt`: no harness of any spelling returned a verdict.
    SqrtNotTractable,
    /// `fma`: no harness of any spelling returned a verdict.
    FmaNotTractable,
    /// A float literal operand (FR-053-AC-1 `FloatLiteralOperand`): the row draws parameters only.
    LiteralOperand,
    /// An operand that is neither a parameter nor a literal (FR-015 `OperandNotSupported`).
    OperandNotParameter,
    /// The two operands are the same parameter: replay needs one value per named parameter.
    SameParameterOperand,
    /// An operand's `float_rounding` bound is present but unreadable (FR-014-AC-16).
    RoundingUnreadable,
    /// The operand forms are not one float width.
    OperandForm,
    /// The routed item has a path condition, which a float harness does not lower.
    PathCondition,
}

impl FloatCapability {
    /// The stable capability name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::AddNotTractable => "float.add.not_kani_tractable",
            Self::SubNotTractable => "float.sub.not_kani_tractable",
            Self::MulNotTractable => "float.mul.not_kani_tractable",
            Self::DivNotTractable => "float.div.not_kani_tractable",
            Self::SqrtNotTractable => "float.sqrt.not_kani_tractable",
            Self::FmaNotTractable => "float.fma.not_kani_tractable",
            Self::LiteralOperand => "float.literal_operand",
            Self::OperandNotParameter => "float.operand_not_parameter",
            Self::SameParameterOperand => "float.same_parameter_operand",
            Self::RoundingUnreadable => "float.rounding_unreadable",
            Self::OperandForm => "float.operand_form",
            Self::PathCondition => "float.path_condition",
        }
    }
}

impl FloatScalarRefusal {
    /// The unsupported disposition of a refused float operation, retaining its named capability.
    pub(crate) fn into_disposition(self) -> ObligationDisposition {
        match self {
            Self::RequiresBound { unbounded_type } => {
                ObligationDisposition::RequiresBound { unbounded_type }
            }
            other => ObligationDisposition::Unsupported {
                reason: other.into_unsupported(),
            },
        }
    }

    fn into_unsupported(self) -> UnsupportedObligation {
        let (cause, operation, detail) = match self {
            Self::RequiresBound { .. } => {
                ("requires_bound".to_owned(), String::new(), String::new())
            }
            Self::NotFloatOperation => (
                "not_float_operation".to_owned(),
                String::new(),
                String::new(),
            ),
            Self::Unsupported {
                capability,
                operation,
                detail,
            } => (capability.name().to_owned(), operation, detail),
        };
        UnsupportedObligation::FloatOperationNotRendered {
            cause,
            operation,
            detail,
        }
    }
}

/// The comparisons of the row and the `IeeeComparison` variant each is proved as.
const COMPARISONS: [(&str, &str); 3] = [
    ("quire.op.ieee.numeric_equal", "NumericEqual"),
    ("quire.op.ieee.total_order", "TotalOrder"),
    ("quire.op.ieee.bit_identical", "BitIdentical"),
];

/// The arithmetic identities and the capability each is refused with.
const ARITHMETIC: [(&str, FloatCapability); 12] = [
    (
        "quire.op.ieee.float32.add",
        FloatCapability::AddNotTractable,
    ),
    (
        "quire.op.ieee.float64.add",
        FloatCapability::AddNotTractable,
    ),
    (
        "quire.op.ieee.float32.sub",
        FloatCapability::SubNotTractable,
    ),
    (
        "quire.op.ieee.float64.sub",
        FloatCapability::SubNotTractable,
    ),
    (
        "quire.op.ieee.float32.mul",
        FloatCapability::MulNotTractable,
    ),
    (
        "quire.op.ieee.float64.mul",
        FloatCapability::MulNotTractable,
    ),
    (
        "quire.op.ieee.float32.div",
        FloatCapability::DivNotTractable,
    ),
    (
        "quire.op.ieee.float64.div",
        FloatCapability::DivNotTractable,
    ),
    (
        "quire.op.ieee.float32.sqrt",
        FloatCapability::SqrtNotTractable,
    ),
    (
        "quire.op.ieee.float64.sqrt",
        FloatCapability::SqrtNotTractable,
    ),
    (
        "quire.op.ieee.float32.fma",
        FloatCapability::FmaNotTractable,
    ),
    (
        "quire.op.ieee.float64.fma",
        FloatCapability::FmaNotTractable,
    ),
];

/// Whether `identity` is an operation this row settles, as a harness or as a named refusal.
pub(crate) fn is_float_row_identity(identity: &str) -> bool {
    COMPARISONS
        .iter()
        .any(|(candidate, _)| *candidate == identity)
        || ARITHMETIC
            .iter()
            .any(|(candidate, _)| *candidate == identity)
}

/// The measured disposition of an arithmetic identity, recorded in the refusal.
fn arithmetic_detail(capability: FloatCapability, rounding: Option<&str>) -> String {
    let spelling = rounding.map_or_else(
        || "the node's rounding spelling is not readable".to_owned(),
        |rounding| format!("the node's rounding spelling is {rounding}"),
    );
    let measured = match capability {
        FloatCapability::AddNotTractable | FloatCapability::SubNotTractable => {
            "the nearest-even harnesses are Verified at both widths, but the exact and directed \
             spellings reached no verdict within 600 s"
        }
        FloatCapability::MulNotTractable => {
            "the binary32 nearest-even harness is Verified; the binary64 nearest-even harness and \
             the exact and directed spellings reached no verdict within 600 s"
        }
        FloatCapability::DivNotTractable => {
            "no harness of any spelling reached a verdict within 600 s, and Kani's symbolic f32 \
             division is bit-wrong at 0x800000A9 / 0x9C873333 (correct result 0x1BA00000)"
        }
        _ => "no harness of any spelling reached a verdict within 600 s",
    };
    format!(
        "an arithmetic operation is a supported row only when all three of its quire-exact \
         harnesses are Verified (FR-405 Behavior 15); {measured}; {spelling}"
    )
}

/// Settle one float operation: refuse it, or generate its harness.
pub fn generate_float_scalar_harness(request: &FloatScalarRequest<'_>) -> FloatScalarOutcome {
    match settle(request) {
        Ok(harness) => FloatScalarOutcome::Supported(Box::new(harness)),
        Err(refusal) => FloatScalarOutcome::Refused(refusal),
    }
}

fn unsupported(
    capability: FloatCapability,
    operation: &str,
    detail: impl Into<String>,
) -> FloatScalarRefusal {
    FloatScalarRefusal::Unsupported {
        capability,
        operation: operation.to_owned(),
        detail: detail.into(),
    }
}

/// One operand of the comparison: the parameter's name and node, and its float form.
struct Operand {
    name: String,
    node: CheckedNodeId,
    form: String,
}

fn settle(request: &FloatScalarRequest<'_>) -> Result<FloatScalarHarness, FloatScalarRefusal> {
    let graph = CheckedGraph::from_checked(request.source);
    let Some(
        term @ CheckedTerm::Application {
            operation,
            arguments,
            ..
        },
    ) = graph.term(request.node_id)
    else {
        return Err(FloatScalarRefusal::NotFloatOperation);
    };
    let identity = operation
        .identity()
        .ok_or(FloatScalarRefusal::NotFloatOperation)?;
    if let Some((_, capability)) = ARITHMETIC
        .iter()
        .find(|(candidate, _)| *candidate == identity)
    {
        let rounding = crate::core::ir::scalar::mode(Some(term), "rounding");
        return Err(unsupported(
            *capability,
            identity,
            arithmetic_detail(*capability, rounding),
        ));
    }
    let (_, comparison) = COMPARISONS
        .iter()
        .find(|(candidate, _)| *candidate == identity)
        .ok_or(FloatScalarRefusal::NotFloatOperation)?;
    let [left, right] = arguments.as_slice() else {
        return Err(FloatScalarRefusal::NotFloatOperation);
    };
    if request.has_path_condition {
        return Err(unsupported(
            FloatCapability::PathCondition,
            identity,
            "a float harness does not lower a path condition",
        ));
    }
    let operand = |term: &CheckedTerm| -> Result<Operand, FloatScalarRefusal> {
        let literal = unsupported(
            FloatCapability::LiteralOperand,
            identity,
            "the row draws parameters only; a float literal is not a drawn operand",
        );
        let not_parameter = || {
            unsupported(
                FloatCapability::OperandNotParameter,
                identity,
                "an operand that is neither a parameter of the function nor a literal is not drawn",
            )
        };
        let CheckedTerm::Reference { target } = term else {
            return Err(if matches!(term, CheckedTerm::Literal { .. }) {
                literal
            } else {
                not_parameter()
            });
        };
        let Ok(declaration) = request.source.parameter_declaration(target) else {
            // A float literal arrives as a reference to a bit-constructor value node.
            let is_literal = graph
                .term(target)
                .is_some_and(|body| matches!(body, CheckedTerm::Literal { .. }));
            return Err(if is_literal { literal } else { not_parameter() });
        };
        // FR-053-AC-3, the operand half: CG's own check until the Contract IR ruling (IR-69)
        // covers floats. The operand's type must reach a `float_rounding` bound.
        match operand_own_bound(&graph, term) {
            Some(bound) => {
                let readable = matches!(&bound.body, CheckedTerm::Aggregate { members }
                    if crate::core::ir::scalar_bounds::read_float_rounding(members).is_some());
                if !readable {
                    return Err(unsupported(
                        FloatCapability::RoundingUnreadable,
                        identity,
                        "the operand's float_rounding bound is present but unreadable",
                    ));
                }
            }
            None => {
                let typed = graph
                    .node(target)
                    .map_or_else(|| target.clone(), |node| node.semantic_type.clone());
                return Err(FloatScalarRefusal::RequiresBound {
                    unbounded_type: typed,
                });
            }
        }
        let form = operand_form(&graph, term).unwrap_or_default();
        Ok(Operand {
            name: declaration.name.to_string(),
            node: target.clone(),
            form,
        })
    };
    let (left, right) = (operand(left)?, operand(right)?);
    if left.node == right.node {
        return Err(unsupported(
            FloatCapability::SameParameterOperand,
            identity,
            "one parameter in both operand slots is not drawn twice or replayed by name",
        ));
    }
    let wide = match (left.form.as_str(), right.form.as_str()) {
        ("float32", "float32") => false,
        ("float64", "float64") => true,
        _ => {
            return Err(unsupported(
                FloatCapability::OperandForm,
                identity,
                format!(
                    "the operand forms are `{}` and `{}`, not one float width",
                    left.form, right.form
                ),
            ))
        }
    };
    render_harness(request, identity, comparison, wide, [left, right]).map_err(|error| {
        unsupported(
            FloatCapability::OperandForm,
            identity,
            format!("the harness did not render: {error:?}"),
        )
    })
}

/// The Rust text of the expected side: an expression over the operand bit patterns `a` and `b`
/// that shares no function with the native comparison, not even its decode.
fn expected_side(comparison: &str, wide: bool) -> String {
    match comparison {
        "NumericEqual" => "!a_nan && !b_nan && (((a | b) << 1) == 0 || a == b)".to_owned(),
        "BitIdentical" => "a == b".to_owned(),
        _ => {
            let float = if wide { "f64" } else { "f32" };
            format!("{float}::from_bits(a).total_cmp(&{float}::from_bits(b)).is_le()")
        }
    }
}

fn render_harness(
    request: &FloatScalarRequest<'_>,
    operation_identity: &str,
    comparison: &str,
    wide: bool,
    operands: [Operand; 2],
) -> Result<FloatScalarHarness, UnsupportedObligation> {
    let expr = |text: String| ExprText::new(text);
    let (bits, shift, exponent, fraction, carrier) = if wide {
        ("u64", 52, "0x7FF_u64", "0xF_FFFF_FFFF_FFFF_u64", "binary64")
    } else {
        ("u32", 23, "0xFF_u32", "0x7F_FFFF_u32", "binary32")
    };
    let domain = if wide {
        (i128::from(i64::MIN), i128::from(i64::MAX))
    } else {
        (0, i128::from(u32::MAX))
    };
    let stem = super::scalar::scalar_stem(operation_identity);
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
            domain,
            then: vec![LetBinding {
                name: format!("{name}_carrier"),
                ty: None,
                mutable: false,
                init: Init::Expression(expr(format!(
                    "rt::NativeFloat::{carrier}({name} as {bits})"
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
        name: "comparison".to_owned(),
        ty: None,
        mutable: false,
        init: Init::Expression(expr(format!("rt::IeeeComparison::{comparison}"))?),
    }));
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: "compare_native_float".to_owned(),
        args: vec![
            Argument::Value("comparison".to_owned()),
            Argument::Value("left_carrier".to_owned()),
            Argument::Value("right_carrier".to_owned()),
            Argument::BorrowMut("meter".to_owned()),
        ],
    });
    // The expected side reads the operand bit patterns directly: a classification by exponent and
    // fraction fields, bit equality, or the host's own `total_cmp`.
    for (name, init) in [
        ("a", format!("left as {bits}")),
        ("b", format!("right as {bits}")),
        (
            "a_nan",
            format!("((a >> {shift}) & {exponent}) == {exponent} && (a & {fraction}) != 0"),
        ),
        (
            "b_nan",
            format!("((b >> {shift}) & {exponent}) == {exponent} && (b & {fraction}) != 0"),
        ),
        ("native", expected_side(comparison, wide)),
    ] {
        body.push(Statement::Let(LetBinding {
            name: name.to_owned(),
            ty: None,
            mutable: false,
            init: Init::Expression(expr(init)?),
        }));
    }
    // Each way the comparison can stop short of `Completed` gets its own assertion, so the
    // failing check names the typed outcome instead of folding it into a disagreement.
    for (name, pattern, message) in [
        (
            "incomplete",
            "Ok(rt::Outcome::Incomplete(_))",
            "the comparison stopped Incomplete, which is never a verified agreement",
        ),
        (
            "ill_typed",
            "Err(rt::IllTyped { cause: rt::IllTypedCause::DistinctIeeeWidths })",
            "the comparison was ill-typed: DistinctIeeeWidths",
        ),
    ] {
        body.push(Statement::Let(LetBinding {
            name: name.to_owned(),
            ty: None,
            mutable: false,
            init: Init::Expression(expr(format!("matches!(&outcome, {pattern})"))?),
        }));
        body.push(Statement::Assert {
            condition: expr(format!("!{name}"))?,
            message: message.to_owned(),
        });
    }
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
            "the native float comparison must complete with the expected {operation_identity} result over the bit patterns"
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
        "use quire_exact as rt;\nuse quire_exact::compare_native_float;\n".to_owned();
    let spec = HarnessSpec::new(
        format!(
            "float-scalar {} `{operation_identity}`",
            super::clause::v2::message_safe_node_label(request.node_id)
        ),
        harness_path(&module_symbol, &harness_symbol)?,
        oracle_source,
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expr("completed".to_owned())?,
            message: "the native float comparison Completed branch is reachable".to_owned(),
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
        oracle_symbol: "quire_exact::compare_native_float".to_owned(),
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        arguments: operands
            .iter()
            .map(|operand| ScalarObligationArgument {
                identifier: operand.name.clone(),
                minimum: i64::try_from(domain.0).unwrap_or(i64::MIN),
                maximum: i64::try_from(domain.1).unwrap_or(i64::MAX),
            })
            .collect(),
        inferred_operands: Vec::new(),
        requirement_occurrence: request.requirement_occurrence.cloned(),
        path_condition: Vec::new(),
        guard_inputs: Vec::new(),
        enum_scalar: None,
        float_scalar: Some(FloatScalarIdentity {
            operator: operation_identity.to_owned(),
            width: if wide { "float64" } else { "float32" }.to_owned(),
            operands: operands
                .iter()
                .map(|operand| operand.node.clone())
                .collect(),
            encoding: format!(
                "one i64 bit pattern per operand, {}",
                if wide {
                    "every i64 read as a u64 binary64 pattern"
                } else {
                    "0..=0xFFFF_FFFF read as a u32 binary32 pattern"
                }
            ),
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
    Ok(FloatScalarHarness {
        harness,
        artifacts: vec![Artifact::new(
            "Cargo.toml",
            crate::core::profile::oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME),
        )],
    })
}
