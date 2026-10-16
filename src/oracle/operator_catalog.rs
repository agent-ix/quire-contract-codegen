//! Closed typed identities consumed by admitted V2 generators.
//!
//! This is a CG-side projection of identities already admitted by Contract IR. It does not
//! participate in admission and deliberately contains only operations with current V2 consumers.

use quire_exact::OrderingOperator;

/// Integer `+`, `-`, `*` and unary `-` shared by the catalog and scalar oracle API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerOperator {
    /// `a + b`.
    Add,
    /// `a - b`.
    Subtract,
    /// `a * b`.
    Multiply,
    /// `-a`.
    Negate,
}

/// Typed identity for an operation shared by scalar oracle and Kani consumers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Operator {
    Add,
    Subtract,
    Multiply,
    Negate,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

/// Complete metadata that differs between consumers for one catalog entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Entry {
    pub(crate) identity: &'static str,
    pub(crate) family: Family,
    pub(crate) operands: OperandOrder,
    pub(crate) result: ResultKind,
    pub(crate) scalar_operator: Option<IntegerOperator>,
    pub(crate) scalar_kani: Option<ScalarKaniOperator>,
    pub(crate) ordering_operator: Option<OrderingOperator>,
    pub(crate) frame_symbol: Option<FrameSymbol>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Family {
    IntegerArithmetic,
    IntegerOrdering,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OperandOrder {
    Unary,
    LeftThenRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResultKind {
    Integer,
    Boolean,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameSymbol {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

impl FrameSymbol {
    pub(crate) const fn rust(self) -> &'static str {
        match self {
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
            Self::Equal => "==",
            Self::NotEqual => "!=",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ScalarKaniOperator {
    Add,
    Subtract,
    Multiply,
    Negate,
}

impl Operator {
    pub(crate) const ALL: [Self; 10] = [
        Self::Add,
        Self::Subtract,
        Self::Multiply,
        Self::Negate,
        Self::Less,
        Self::LessOrEqual,
        Self::Greater,
        Self::GreaterOrEqual,
        Self::Equal,
        Self::NotEqual,
    ];

    pub(crate) const fn entry(self) -> Entry {
        use Operator as O;
        let (
            identity,
            family,
            operands,
            result,
            scalar_operator,
            scalar_kani,
            ordering_operator,
            frame_symbol,
        ) = match self {
            O::Add => (
                "quire.op.integer.add",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(IntegerOperator::Add),
                Some(ScalarKaniOperator::Add),
                None,
                None,
            ),
            O::Subtract => (
                "quire.op.integer.sub",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(IntegerOperator::Subtract),
                Some(ScalarKaniOperator::Subtract),
                None,
                None,
            ),
            O::Multiply => (
                "quire.op.integer.mul",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(IntegerOperator::Multiply),
                Some(ScalarKaniOperator::Multiply),
                None,
                None,
            ),
            O::Negate => (
                "quire.op.integer.negate",
                Family::IntegerArithmetic,
                OperandOrder::Unary,
                ResultKind::Integer,
                Some(IntegerOperator::Negate),
                Some(ScalarKaniOperator::Negate),
                None,
                None,
            ),
            O::Less => (
                "quire.op.integer.lt",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                Some(OrderingOperator::Less),
                Some(FrameSymbol::Less),
            ),
            O::LessOrEqual => (
                "quire.op.integer.le",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                Some(OrderingOperator::LessOrEqual),
                Some(FrameSymbol::LessOrEqual),
            ),
            O::Greater => (
                "quire.op.integer.gt",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                Some(OrderingOperator::Greater),
                Some(FrameSymbol::Greater),
            ),
            O::GreaterOrEqual => (
                "quire.op.integer.ge",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                Some(OrderingOperator::GreaterOrEqual),
                Some(FrameSymbol::GreaterOrEqual),
            ),
            O::Equal => (
                "quire.op.integer.eq",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                None,
                Some(FrameSymbol::Equal),
            ),
            O::NotEqual => (
                "quire.op.integer.ne",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                None,
                None,
                Some(FrameSymbol::NotEqual),
            ),
        };
        Entry {
            identity,
            family,
            operands,
            result,
            scalar_operator,
            scalar_kani,
            ordering_operator,
            frame_symbol,
        }
    }

    pub(crate) fn from_identity(identity: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|operation| operation.entry().identity == identity)
    }

    pub(crate) fn from_ordering_operator(operator: OrderingOperator) -> Option<Self> {
        match operator {
            OrderingOperator::Less => Some(Self::Less),
            OrderingOperator::LessOrEqual => Some(Self::LessOrEqual),
            OrderingOperator::Greater => Some(Self::Greater),
            OrderingOperator::GreaterOrEqual => Some(Self::GreaterOrEqual),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    #[trace("FR-038-AC-1", "FR-038-AC-2", "FR-038-AC-3")]
    #[test]
    fn catalog_entries_preserve_identity_family_arity_result_and_consumer_projection() {
        let expected = [
            (
                "quire.op.integer.add",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(ScalarKaniOperator::Add),
                None,
            ),
            (
                "quire.op.integer.sub",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(ScalarKaniOperator::Subtract),
                None,
            ),
            (
                "quire.op.integer.mul",
                Family::IntegerArithmetic,
                OperandOrder::LeftThenRight,
                ResultKind::Integer,
                Some(ScalarKaniOperator::Multiply),
                None,
            ),
            (
                "quire.op.integer.negate",
                Family::IntegerArithmetic,
                OperandOrder::Unary,
                ResultKind::Integer,
                Some(ScalarKaniOperator::Negate),
                None,
            ),
            (
                "quire.op.integer.lt",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::Less),
            ),
            (
                "quire.op.integer.le",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::LessOrEqual),
            ),
            (
                "quire.op.integer.gt",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::Greater),
            ),
            (
                "quire.op.integer.ge",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::GreaterOrEqual),
            ),
            (
                "quire.op.integer.eq",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::Equal),
            ),
            (
                "quire.op.integer.ne",
                Family::IntegerOrdering,
                OperandOrder::LeftThenRight,
                ResultKind::Boolean,
                None,
                Some(FrameSymbol::NotEqual),
            ),
        ];
        assert_eq!(Operator::ALL.len(), expected.len());
        for (operation, expected) in Operator::ALL.into_iter().zip(expected) {
            let entry = operation.entry();
            assert_eq!(
                (
                    entry.identity,
                    entry.family,
                    entry.operands,
                    entry.result,
                    entry.scalar_kani,
                    entry.frame_symbol
                ),
                (expected.0, expected.1, expected.2, expected.3, expected.4, expected.5)
            );
            assert_eq!(Operator::from_identity(entry.identity), Some(operation));
        }
        assert_eq!(
            Operator::from_identity("quire.op.integer.eq")
                .unwrap()
                .entry()
                .scalar_operator,
            None
        );
        assert_eq!(Operator::from_identity("quire.op.boolean.and"), None);
    }
}
