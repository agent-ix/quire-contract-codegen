//! IR-independent model of one admitted integer comparison.
//!
//! A relation compares one primary read with either a literal or a partner read over one shared
//! inclusive `i64` domain, because the IR types both operands of a `Compare` with one
//! `IntegerType`.

/// Comparison operator of an admitted `Compare` node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComparisonOperator {
    /// `left == right`.
    Equal,
    /// `left != right`.
    NotEqual,
    /// `left < right`.
    Less,
    /// `left <= right`.
    LessEqual,
    /// `left > right`.
    Greater,
    /// `left >= right`.
    GreaterEqual,
}

impl ComparisonOperator {
    /// Every operator, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Equal,
        Self::NotEqual,
        Self::Less,
        Self::LessEqual,
        Self::Greater,
        Self::GreaterEqual,
    ];

    /// Evaluates `left <operator> right`.
    #[must_use]
    pub const fn evaluate(self, left: i64, right: i64) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::Less => left < right,
            Self::LessEqual => left <= right,
            Self::Greater => left > right,
            Self::GreaterEqual => left >= right,
        }
    }
}

/// Operand position the primary read occupies in the comparison as written.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperandPosition {
    /// The primary read is the left operand.
    Left,
    /// The primary read is the right operand, which happens only when the left operand is a
    /// literal.
    Right,
}

/// The operand compared with the primary read.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Partner {
    /// An integer literal `k`.
    Literal(i64),
    /// A second read sharing the primary read's domain.
    Read,
}

/// One admitted single-comparison relation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Relation {
    /// Comparison operator as written.
    pub operator: ComparisonOperator,
    /// Position of the primary read in the comparison as written.
    pub primary: OperandPosition,
    /// Literal or partner read compared with the primary read.
    pub partner: Partner,
}

impl Relation {
    /// Evaluates the relation for one primary value and, for a partner read, the partner value.
    ///
    /// Returns `None` when the valuation does not match the relation's shape: a partner value for a
    /// literal relation, or no partner value for a two-read relation.
    #[must_use]
    pub const fn evaluate(self, primary: i64, partner: Option<i64>) -> Option<bool> {
        let other = match (self.partner, partner) {
            (Partner::Literal(value), None) | (Partner::Read, Some(value)) => value,
            _ => return None,
        };
        Some(match self.primary {
            OperandPosition::Left => self.operator.evaluate(primary, other),
            OperandPosition::Right => self.operator.evaluate(other, primary),
        })
    }

    /// Returns whether the relation compares two reads.
    #[must_use]
    pub const fn has_partner_read(self) -> bool {
        matches!(self.partner, Partner::Read)
    }
}

/// Shared inclusive integer domain of every read in one relation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Domain {
    /// Inclusive minimum from the declaration's `IntegerType`.
    pub minimum: i64,
    /// Inclusive maximum from the declaration's `IntegerType`.
    pub maximum: i64,
}

impl Domain {
    /// Returns whether `value` lies in `minimum..=maximum`.
    #[must_use]
    pub fn contains(self, value: i128) -> bool {
        i128::from(self.minimum) <= value && value <= i128::from(self.maximum)
    }
}
