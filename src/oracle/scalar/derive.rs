// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

use super::lower::{
    lowered, operand_form, read_decimal_range, read_float_rounding, read_integer_range,
    read_rational_range, read_text_bounds, scalar_type_form, BoundView, Bounds,
};
use super::{
    scalar_profile, BooleanOperator, BoundForm, CatalogOperator, CheckedNodeId, CheckedNodeTag,
    CheckedPackageV2, CheckedSemanticNodeV2, CheckedTerm, ClaimDerivationRefusal,
    ComparisonOperator, CompleteLoweringRecordV2, DecimalOperator, DecimalType, DivisionProfile,
    EqualityOperandKind, EqualityOperator, ExactScalarItem, ExactScalarOperation,
    ExactScalarRefusal, Graph, GraphBody, IeeeArithmeticOperator, IeeeComparison, IeeeWidth,
    IntegerDomain, IntegerOperator, OrderingOperandKind, OrderingOperator, QuantityOperator,
    RationalOperator, RoundingMode, ScalarForm, Value, QUANTITY_CONVERT_IDENTITY,
};

pub(super) fn conversion_target<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> Option<CheckedNodeId> {
    let payload = graph.operation(&node.node_id)?;
    (payload["identity"].as_str() == Some(QUANTITY_CONVERT_IDENTITY))
        .then(|| serde_json::from_value(payload["member"]["declaration"].clone()).ok())
        .flatten()
}

pub(super) fn quantity_conversion_target_is_unit<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> bool {
    conversion_target(graph, node)
        .and_then(|id| graph.node(&id))
        .is_some_and(|target| {
            target.node_tag.as_ref() == "scalar_type" && target.semantic_form.as_ref() == "unit"
        })
}

/// An operation identity with the parameters that are not read from the node's bounds, laws,
/// mode or operand forms already resolved. It is the inverse of [`catalogued_operation`]:
/// `family_of_identity` is the one match over identity strings, and a round-trip test walks
/// every descriptor through both.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Family {
    Boolean(BooleanOperator),
    Equality(EqualityOperator, EqualityOperandKind),
    IntegerArithmetic(IntegerOperator),
    IntegerDivision,
    IntegerModulo,
    RationalArithmetic(RationalOperator),
    /// `rational.div`: `IntegerDivide` or `Divide` by the operand forms.
    RationalDivision,
    Ordering(OrderingOperator, OrderingOperandKind),
    DecimalArithmetic(DecimalOperator),
    IeeeArithmetic(IeeeArithmeticOperator, IeeeWidth),
    /// `ieee.numeric_equal`, `total_order` and `bit_identical`: the width comes from the operands.
    IeeeComparison(IeeeComparison),
    IeeeToFloat32,
    IeeeToFloat64,
    /// `numeric.convert`: text admission when the result is text.
    NumericConvert,
    TextComparison(ComparisonOperator),
    EnumComparison(ComparisonOperator),
    QuantityConversion,
    QuantityArithmetic(QuantityOperator),
    QuantityComparison(ComparisonOperator),
}

/// The inverse of [`catalogued_operation`] over identities. Identities outside the derivable set,
/// which is every identity this match does not name, are `None`.
pub(super) fn family_of_identity(identity: &str) -> Option<Family> {
    use ComparisonOperator as C;
    use OrderingOperandKind as K;
    use OrderingOperator as O;
    if let Some(catalog_operator) = CatalogOperator::from_identity(identity) {
        return Some(match catalog_operator {
            CatalogOperator::Add
            | CatalogOperator::Subtract
            | CatalogOperator::Multiply
            | CatalogOperator::Negate => {
                Family::IntegerArithmetic(catalog_operator.entry().scalar_operator?)
            }
            CatalogOperator::Less
            | CatalogOperator::LessOrEqual
            | CatalogOperator::Greater
            | CatalogOperator::GreaterOrEqual => Family::Ordering(
                catalog_operator.entry().ordering_operator?,
                OrderingOperandKind::Integer,
            ),
            CatalogOperator::Equal => {
                Family::Equality(EqualityOperator::Equal, EqualityOperandKind::Integer)
            }
            CatalogOperator::NotEqual => {
                Family::Equality(EqualityOperator::NotEqual, EqualityOperandKind::Integer)
            }
        });
    }
    Some(match identity {
        "quire.op.boolean.and" => Family::Boolean(BooleanOperator::And),
        "quire.op.boolean.or" => Family::Boolean(BooleanOperator::Or),
        "quire.op.boolean.not" => Family::Boolean(BooleanOperator::Not),
        "quire.op.boolean.implies" => Family::Boolean(BooleanOperator::Implies),
        "quire.op.boolean.eq" => {
            Family::Equality(EqualityOperator::Equal, EqualityOperandKind::Boolean)
        }
        "quire.op.boolean.ne" => {
            Family::Equality(EqualityOperator::NotEqual, EqualityOperandKind::Boolean)
        }
        "quire.op.integer.div" => Family::IntegerDivision,
        "quire.op.integer.mod" => Family::IntegerModulo,
        "quire.op.rational.add" => Family::RationalArithmetic(RationalOperator::Add),
        "quire.op.rational.sub" => Family::RationalArithmetic(RationalOperator::Subtract),
        "quire.op.rational.mul" => Family::RationalArithmetic(RationalOperator::Multiply),
        "quire.op.rational.negate" => Family::RationalArithmetic(RationalOperator::Negate),
        "quire.op.rational.div" => Family::RationalDivision,
        "quire.op.rational.lt" => Family::Ordering(O::Less, K::Rational),
        "quire.op.rational.le" => Family::Ordering(O::LessOrEqual, K::Rational),
        "quire.op.rational.gt" => Family::Ordering(O::Greater, K::Rational),
        "quire.op.rational.ge" => Family::Ordering(O::GreaterOrEqual, K::Rational),
        "quire.op.decimal.lt" => Family::Ordering(O::Less, K::Decimal),
        "quire.op.decimal.le" => Family::Ordering(O::LessOrEqual, K::Decimal),
        "quire.op.decimal.gt" => Family::Ordering(O::Greater, K::Decimal),
        "quire.op.decimal.ge" => Family::Ordering(O::GreaterOrEqual, K::Decimal),
        "quire.op.decimal.add" => Family::DecimalArithmetic(DecimalOperator::Add),
        "quire.op.decimal.sub" => Family::DecimalArithmetic(DecimalOperator::Subtract),
        "quire.op.decimal.mul" => Family::DecimalArithmetic(DecimalOperator::Multiply),
        "quire.op.decimal.div" => Family::DecimalArithmetic(DecimalOperator::Divide),
        "quire.op.decimal.negate" => Family::DecimalArithmetic(DecimalOperator::Negate),
        "quire.op.numeric.convert_rounding" => Family::DecimalArithmetic(DecimalOperator::Round),
        "quire.op.ieee.float32.add" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Add, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.sub" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Subtract, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.mul" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Multiply, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.div" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Divide, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float64.add" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Add, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.sub" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Subtract, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.mul" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Multiply, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.div" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Divide, IeeeWidth::Binary64)
        }
        "quire.op.ieee.numeric_equal" => Family::IeeeComparison(IeeeComparison::NumericEqual),
        "quire.op.ieee.total_order" => Family::IeeeComparison(IeeeComparison::TotalOrder),
        "quire.op.ieee.bit_identical" => Family::IeeeComparison(IeeeComparison::BitIdentical),
        "quire.op.ieee.to_float32" => Family::IeeeToFloat32,
        "quire.op.ieee.to_float64" => Family::IeeeToFloat64,
        "quire.op.numeric.convert" => Family::NumericConvert,
        "quire.op.text.eq" => Family::TextComparison(C::Equal),
        "quire.op.text.ne" => Family::TextComparison(C::NotEqual),
        "quire.op.text.lt" => Family::TextComparison(C::Less),
        "quire.op.text.le" => Family::TextComparison(C::LessOrEqual),
        "quire.op.text.gt" => Family::TextComparison(C::Greater),
        "quire.op.text.ge" => Family::TextComparison(C::GreaterOrEqual),
        "quire.op.enum.eq" => Family::EnumComparison(C::Equal),
        "quire.op.enum.ne" => Family::EnumComparison(C::NotEqual),
        "quire.op.enum.lt" => Family::EnumComparison(C::Less),
        "quire.op.enum.le" => Family::EnumComparison(C::LessOrEqual),
        "quire.op.enum.gt" => Family::EnumComparison(C::Greater),
        "quire.op.enum.ge" => Family::EnumComparison(C::GreaterOrEqual),
        QUANTITY_CONVERT_IDENTITY => Family::QuantityConversion,
        "quire.op.quantity.add" => Family::QuantityArithmetic(QuantityOperator::Add),
        "quire.op.quantity.sub" => Family::QuantityArithmetic(QuantityOperator::Subtract),
        "quire.op.quantity.mul" => Family::QuantityArithmetic(QuantityOperator::Multiply),
        "quire.op.quantity.div" => Family::QuantityArithmetic(QuantityOperator::Divide),
        "quire.op.quantity.pow" => Family::QuantityArithmetic(QuantityOperator::Power),
        "quire.op.quantity.eq" => Family::QuantityComparison(C::Equal),
        "quire.op.quantity.ne" => Family::QuantityComparison(C::NotEqual),
        "quire.op.quantity.lt" => Family::QuantityComparison(C::Less),
        "quire.op.quantity.le" => Family::QuantityComparison(C::LessOrEqual),
        "quire.op.quantity.gt" => Family::QuantityComparison(C::Greater),
        "quire.op.quantity.ge" => Family::QuantityComparison(C::GreaterOrEqual),
        _ => return None,
    })
}

/// Derive descriptors from the checked lowering records.
pub(super) fn derive_exact_scalar_items(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    let lowering = package.lower(node_ids, &scalar_profile());
    derive_exact_scalar_items_from_records(package, node_ids, &lowering.records)
}

/// Classify each item under its original one-key lowering semantics while
/// sharing Contract IR's graph and closure context across the request.
pub(crate) fn derive_exact_scalar_items_independent(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    let records = package.lower_independent_records(node_ids, &scalar_profile());
    derive_exact_scalar_items_from_records(package, node_ids, &records)
}

fn derive_exact_scalar_items_from_records(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
    records: &[CompleteLoweringRecordV2],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    let graph = Graph::from_checked_with_node_visit(package, || {
        #[cfg(test)]
        SCALAR_DERIVE_GRAPH_VISITS.with(|count| count.set(count.get() + 1));
    });
    node_ids
        .iter()
        .zip(records)
        .map(|(node_id, record)| {
            derive_operation(&graph, node_id, record).map(|operation| ExactScalarItem {
                node_id: node_id.clone(),
                operation,
            })
        })
        .collect()
}

#[cfg(test)]
thread_local! {
    static SCALAR_DERIVE_GRAPH_VISITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn take_scalar_derive_graph_visits() -> usize {
    SCALAR_DERIVE_GRAPH_VISITS.with(|count| count.replace(0))
}

/// What the derivation reads about one node.
pub(super) struct Derivation<
    'a,
    'g,
    'n,
    Body: GraphBody = Value,
    NodeBody = CheckedTerm,
    BoundBody = CheckedTerm,
> {
    pub(super) graph: &'a Graph<'g, Body>,
    pub(super) node: &'g CheckedSemanticNodeV2<NodeBody>,
    pub(super) bounds: BoundView<'n, BoundBody>,
    pub(super) identity: &'a str,
}

impl<Body: GraphBody, NodeBody, BoundBody> Derivation<'_, '_, '_, Body, NodeBody, BoundBody> {
    pub(super) fn right_may_stop(&self) -> bool {
        let Some(right) = self
            .graph
            .arguments(&self.node.node_id)
            .and_then(|args| args.get(1))
        else {
            return false;
        };
        match right {
            CheckedTerm::Application { .. } => true,
            CheckedTerm::Reference { target } => self.graph.node(target).is_some_and(|node| {
                &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                    && matches!(
                        self.graph.term(&node.node_id),
                        Some(CheckedTerm::Application { .. })
                    )
            }),
            _ => false,
        }
    }

    pub(super) fn operand_forms_not_derivable(&self) -> ExactScalarRefusal {
        ClaimDerivationRefusal::OperandFormsNotDerivable {
            operation_identity: self.identity.to_owned(),
        }
        .refusal()
    }

    fn mode_not_derivable(&self) -> ExactScalarRefusal {
        ClaimDerivationRefusal::ModeNotDerivable {
            operation_identity: self.identity.to_owned(),
        }
        .refusal()
    }

    /// The node's result bound of `form`.
    fn read<T>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
    ) -> Result<T, ExactScalarRefusal> {
        let bounds = Bounds {
            graph: self.graph,
            node: self.bounds.clone(),
        };
        bounds.read(form, read).map(|(_, value)| value)
    }

    /// The scalar forms of the application's arguments.
    fn operand_forms(&self) -> Vec<Option<String>> {
        self.graph
            .arguments(&self.node.node_id)
            .into_iter()
            .flatten()
            .map(|argument| operand_form(self.graph, argument))
            .collect()
    }

    /// The scalar form of the node's result type.
    fn result_form(&self) -> Option<String> {
        scalar_type_form(self.graph, &self.node.semantic_type)
    }

    /// The `operation.mode` value when its kind is `rounding`.
    fn mode_value(&self) -> Option<&str> {
        let mode = &self.graph.operation(&self.node.node_id)?["mode"];
        (mode["kind"].as_str() == Some("rounding"))
            .then(|| mode["value"].as_str())
            .flatten()
    }

    /// Requires `operation.mode` to carry `rounding`.
    pub(super) fn require_mode(&self, rounding: RoundingMode) -> Result<(), ExactScalarRefusal> {
        if self.mode_value() == Some(rounding.as_str()) {
            Ok(())
        } else {
            Err(self.mode_not_derivable())
        }
    }

    /// The one division profile `operation.laws` names.
    pub(super) fn division_profile(&self) -> Result<DivisionProfile, ExactScalarRefusal> {
        let named = DivisionProfile::ALL
            .into_iter()
            .filter(|profile| {
                self.graph
                    .operation(&self.node.node_id)
                    .and_then(|operation| operation.get("laws"))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .any(|law| {
                        law["definition"]["identity"].as_str()
                            == Some(profile.definition_identity())
                    })
            })
            .collect::<Vec<_>>();
        match named.as_slice() {
            [profile] => Ok(*profile),
            _ => Err(ClaimDerivationRefusal::LawNotDerivable {
                operation_identity: self.identity.to_owned(),
            }
            .refusal()),
        }
    }

    fn integer_domain(&self) -> Result<IntegerDomain, ExactScalarRefusal> {
        let bounds = Bounds {
            graph: self.graph,
            node: self.bounds.clone(),
        };
        // A reached bounded operand beside a plain reference is refused for
        // operand ownership before the selected own result can lack a bound.
        if self.bounds.selected.is_some() {
            bounds.integer_operand_bounds()?;
        }
        let domain = self.read(BoundForm::IntegerRange, read_integer_range)?;
        // Derivation refuses what generation refuses, so the two never disagree on a node.
        bounds.integer_operand_bounds()?;
        Ok(IntegerDomain::Bounded(domain))
    }

    fn decimal_target(&self) -> Result<DecimalType, ExactScalarRefusal> {
        self.read(BoundForm::DecimalRange, read_decimal_range)
    }

    fn float_rounding(&self) -> Result<RoundingMode, ExactScalarRefusal> {
        self.read(BoundForm::FloatRounding, read_float_rounding)
    }

    /// Both operands share one scalar form, of the `candidates`.
    fn shared_operand_form<T: Copy>(
        &self,
        candidates: &[(&str, T)],
    ) -> Result<T, ExactScalarRefusal> {
        let forms = self.operand_forms();
        let [Some(first), Some(second)] = forms.as_slice() else {
            return Err(self.operand_forms_not_derivable());
        };
        candidates
            .iter()
            .find(|(form, _)| first == form && second == form)
            .map(|(_, value)| *value)
            .ok_or_else(|| self.operand_forms_not_derivable())
    }
}

pub(super) fn derive_operation<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node_id: &CheckedNodeId,
    record: &CompleteLoweringRecordV2,
) -> Result<ExactScalarOperation, ExactScalarRefusal> {
    // The graph and the lowering come first, so a node absent from the graph, or one that does not
    // lower, keeps the refusal `generate_exact_scalar_oracles` gives it and FR-015 classifies.
    let node = graph
        .node(node_id)
        .ok_or(ExactScalarRefusal::InvalidInput)?;
    let lowered = lowered(record)?;
    derive_operation_from_view(graph, node, BoundView::complete(lowered))
}

pub(super) fn derive_operation_from_view<'g, Body: GraphBody, NodeBody, BoundBody>(
    graph: &Graph<'g, Body>,
    node: &'g CheckedSemanticNodeV2<NodeBody>,
    bounds: BoundView<'_, BoundBody>,
) -> Result<ExactScalarOperation, ExactScalarRefusal> {
    use ExactScalarOperation as Op;
    if &*node.node_tag != CheckedNodeTag::Expression.as_wire()
        || !matches!(
            graph.term(&node.node_id),
            Some(CheckedTerm::Application { .. })
        )
    {
        return Err(ClaimDerivationRefusal::NotApplication {
            node_tag: node.node_tag.to_string(),
        }
        .refusal());
    }
    let identity = graph
        .operation(&node.node_id)
        .and_then(|operation| operation.get("identity"))
        .and_then(Value::as_str)
        .ok_or_else(|| ClaimDerivationRefusal::MissingOperationIdentity.refusal())?;
    let derivation = Derivation {
        graph,
        node,
        bounds,
        identity,
    };
    let family = family_of_identity(identity).ok_or_else(|| {
        ClaimDerivationRefusal::OperationNotDerivable {
            operation_identity: identity.to_owned(),
        }
        .refusal()
    })?;
    Ok(match family {
        Family::Boolean(operator) => Op::Boolean {
            operator,
            right_may_stop: matches!(
                operator,
                BooleanOperator::And | BooleanOperator::Or | BooleanOperator::Implies
            ) && derivation.right_may_stop(),
        },
        Family::Equality(operator, operands) => Op::Equality { operator, operands },
        Family::IntegerArithmetic(operator) => Op::IntegerArithmetic {
            operator,
            domain: derivation.integer_domain()?,
        },
        Family::IntegerDivision => Op::IntegerDivision {
            profile: derivation.division_profile()?,
            domain: derivation.integer_domain()?,
        },
        Family::IntegerModulo => Op::IntegerModulo {
            domain: derivation.integer_domain()?,
        },
        Family::RationalArithmetic(operator) => Op::RationalArithmetic {
            operator,
            domain: Some(derivation.read(BoundForm::RationalRange, read_rational_range)?),
        },
        Family::RationalDivision => Op::RationalArithmetic {
            operator: derivation.shared_operand_form(&[
                ("integer", RationalOperator::IntegerDivide),
                ("rational", RationalOperator::Divide),
            ])?,
            domain: Some(derivation.read(BoundForm::RationalRange, read_rational_range)?),
        },
        Family::Ordering(operator, operands) => Op::Ordering { operator, operands },
        Family::DecimalArithmetic(operator) => {
            if operator == DecimalOperator::Round
                && derivation.result_form().as_deref() != Some(ScalarForm::Decimal.wire())
            {
                return Err(derivation.operand_forms_not_derivable());
            }
            let target = derivation.decimal_target()?;
            if operator != DecimalOperator::Negate {
                derivation.require_mode(target.rounding())?;
            }
            Op::DecimalArithmetic { operator, target }
        }
        Family::IeeeArithmetic(operator, width) => {
            let rounding = derivation.float_rounding()?;
            derivation.require_mode(rounding)?;
            Op::IeeeArithmetic {
                operator,
                width,
                rounding,
            }
        }
        Family::IeeeComparison(comparison) => Op::IeeeComparison {
            comparison,
            width: derivation.shared_operand_form(&[
                ("float32", IeeeWidth::Binary32),
                ("float64", IeeeWidth::Binary64),
            ])?,
        },
        Family::IeeeToFloat32 => {
            let rounding = derivation.float_rounding()?;
            derivation.require_mode(rounding)?;
            Op::IeeeWidthConversion {
                source: IeeeWidth::Binary64,
                target: IeeeWidth::Binary32,
                rounding,
            }
        }
        Family::IeeeToFloat64 => Op::IeeeWidthConversion {
            source: IeeeWidth::Binary32,
            target: IeeeWidth::Binary64,
            rounding: derivation.float_rounding()?,
        },
        Family::NumericConvert => {
            if derivation.result_form().as_deref() != Some(ScalarForm::Text.wire()) {
                return Err(derivation.operand_forms_not_derivable());
            }
            Op::TextAdmission {
                text_type: derivation.read(BoundForm::TextBounds, read_text_bounds)?,
            }
        }
        Family::TextComparison(operator) => Op::TextComparison { operator },
        Family::EnumComparison(operator) => Op::EnumComparison { operator },
        Family::QuantityConversion => {
            derivation.require_mode(RoundingMode::Exact)?;
            if !quantity_conversion_target_is_unit(graph, node) {
                return Err(derivation.operand_forms_not_derivable());
            }
            Op::QuantityConversion
        }
        Family::QuantityArithmetic(operator) => Op::QuantityArithmetic { operator },
        Family::QuantityComparison(operator) => Op::QuantityComparison { operator },
    })
}
