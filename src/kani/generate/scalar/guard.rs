//! Path-guard lowering for the scalar family (FR-022, IR-304).
//!
//! A guard restricts which inputs the claim is about, so it becomes a `Statement::Assume` in the
//! harness. The same assumptions narrow each symbolic input's range: interval analysis removes
//! only values that falsify an assumed guard, so no execution the claim covers is dropped.

use quire_contract_model::CheckedNodeId;

use crate::{
    core::requirement::{RequirementGuard, RequirementOccurrence},
    kani::{
        abi::{i64_literal, KaniPrimitiveType},
        generate::outcome::PathGuardRefusal,
        identity::{ScalarGuardDomain, ScalarGuardInput},
    },
};

/// The six IR-confirmed integer comparisons a guard may be.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Comparison {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl Comparison {
    /// The comparison a catalogued identity names.
    pub(crate) fn from_identity(identity: &str) -> Option<Self> {
        Some(match identity {
            "quire.op.integer.lt" => Self::Lt,
            "quire.op.integer.le" => Self::Le,
            "quire.op.integer.gt" => Self::Gt,
            "quire.op.integer.ge" => Self::Ge,
            "quire.op.integer.eq" => Self::Eq,
            "quire.op.integer.ne" => Self::Ne,
            _ => return None,
        })
    }

    /// `c <op> x` read as `x <mirrored> c`.
    fn mirrored(self) -> Self {
        match self {
            Self::Lt => Self::Gt,
            Self::Le => Self::Ge,
            Self::Gt => Self::Lt,
            Self::Ge => Self::Le,
            Self::Eq => Self::Eq,
            Self::Ne => Self::Ne,
        }
    }

    /// The operator a `holds = false` guard requires.
    fn negated(self) -> Self {
        match self {
            Self::Lt => Self::Ge,
            Self::Le => Self::Gt,
            Self::Gt => Self::Le,
            Self::Ge => Self::Lt,
            Self::Eq => Self::Ne,
            Self::Ne => Self::Eq,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
        }
    }
}

/// One side of a guard comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GuardSide {
    /// The parameter at this index of [`GuardPlan::parameters`].
    Parameter(usize),
    /// An integer literal that fits `i64`.
    Literal(i64),
}

/// The lowerable shape of a guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GuardForm {
    /// A direct reference to a Boolean parameter.
    Boolean(usize),
    /// An integer comparison with at least one parameter side.
    Compare {
        operator: Comparison,
        left: GuardSide,
        right: GuardSide,
    },
}

/// One lowered guard of the path condition.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedGuard {
    pub(crate) occurrence: RequirementOccurrence,
    pub(crate) holds: bool,
    pub(crate) form: GuardForm,
}

/// A parameter's declared domain as the package spells it; the `i64` check is a later step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DeclaredDomain {
    Boolean,
    Integer { lower: String, upper: String },
}

/// A parameter a guard reads, in order of first read.
#[derive(Clone, Debug)]
pub(crate) struct GuardParameter {
    /// The parameter node, in the authenticated V2 source frame.
    pub(crate) node: CheckedNodeId,
    pub(crate) declared: DeclaredDomain,
    /// The operand positions (call order) that directly reference this parameter.
    pub(crate) operand_positions: Vec<usize>,
}

/// A path condition whose every guard is lowerable.
#[derive(Clone, Debug, Default)]
pub(crate) struct GuardPlan {
    pub(crate) path_condition: Vec<RequirementGuard>,
    pub(crate) parameters: Vec<GuardParameter>,
    pub(crate) guards: Vec<ResolvedGuard>,
}

/// The first guard that is not lowered, and why.
pub(crate) type GuardRefusal = Box<(RequirementOccurrence, PathGuardRefusal)>;

/// Why the guards cannot narrow the claim's inputs.
pub(crate) enum NarrowingRefusal {
    /// A guard parameter's declared domain does not fit `i64`.
    BoundNotI64 { lower: String, upper: String },
    /// The guards empty a parameter's domain.
    Unsatisfiable(Box<UnsatisfiedPath>),
}

/// The guard at which a parameter's domain became empty.
#[derive(Clone, Debug)]
pub(crate) struct UnsatisfiedPath {
    pub(crate) guard: RequirementOccurrence,
    pub(crate) parameter: CheckedNodeId,
    pub(crate) declared: ScalarGuardDomain,
}

/// A parameter's narrowed domain.
#[derive(Clone, Copy, Debug)]
enum Narrowed {
    Boolean {
        can_be_false: bool,
        can_be_true: bool,
    },
    Integer {
        lower: i64,
        upper: i64,
    },
}

/// The narrowed domain of each guard parameter.
pub(crate) struct NarrowedGuards(Vec<Narrowed>);

/// What the renderer needs of the guards: the extra inputs and the assumptions.
pub(crate) struct RenderedGuards {
    pub(crate) inputs: Vec<GuardInput>,
    /// One assumption per guard, outermost first.
    pub(crate) assumptions: Vec<String>,
}

/// A symbolic input drawn for a parameter no operand reads.
pub(crate) struct GuardInput {
    pub(crate) identity: ScalarGuardInput,
    pub(crate) primitive: KaniPrimitiveType,
    pub(crate) domain: (i128, i128),
}

impl GuardPlan {
    /// Whether the path condition is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.guards.is_empty()
    }

    /// Intersect each parameter's declared domain with what the guards admit, in order.
    pub(crate) fn narrow(&self) -> Result<NarrowedGuards, NarrowingRefusal> {
        let mut domains = Vec::with_capacity(self.parameters.len());
        for parameter in &self.parameters {
            domains.push(match &parameter.declared {
                DeclaredDomain::Boolean => Narrowed::Boolean {
                    can_be_false: true,
                    can_be_true: true,
                },
                DeclaredDomain::Integer { lower, upper } => match (lower.parse(), upper.parse()) {
                    (Ok(lower), Ok(upper)) => Narrowed::Integer { lower, upper },
                    _ => {
                        return Err(NarrowingRefusal::BoundNotI64 {
                            lower: lower.clone(),
                            upper: upper.clone(),
                        })
                    }
                },
            });
        }
        let declared = domains.clone();
        for guard in &self.guards {
            let emptied = match guard.form {
                GuardForm::Boolean(parameter) => {
                    let Narrowed::Boolean {
                        can_be_false,
                        can_be_true,
                    } = &mut domains[parameter]
                    else {
                        continue;
                    };
                    if guard.holds {
                        *can_be_false = false;
                    } else {
                        *can_be_true = false;
                    }
                    (!*can_be_false && !*can_be_true).then_some(parameter)
                }
                GuardForm::Compare {
                    operator,
                    left,
                    right,
                } => {
                    let operator = if guard.holds {
                        operator
                    } else {
                        operator.negated()
                    };
                    let (parameter, operator, literal) = match (left, right) {
                        (GuardSide::Parameter(parameter), GuardSide::Literal(literal)) => {
                            (parameter, operator, literal)
                        }
                        (GuardSide::Literal(literal), GuardSide::Parameter(parameter)) => {
                            (parameter, operator.mirrored(), literal)
                        }
                        _ => continue,
                    };
                    let Narrowed::Integer { lower, upper } = &mut domains[parameter] else {
                        continue;
                    };
                    let (mut low, mut high) = (i128::from(*lower), i128::from(*upper));
                    let c = i128::from(literal);
                    match operator {
                        Comparison::Lt => high = high.min(c - 1),
                        Comparison::Le => high = high.min(c),
                        Comparison::Gt => low = low.max(c + 1),
                        Comparison::Ge => low = low.max(c),
                        Comparison::Eq => {
                            low = low.max(c);
                            high = high.min(c);
                        }
                        Comparison::Ne => {
                            if c == low {
                                low += 1;
                            } else if c == high {
                                high -= 1;
                            }
                        }
                    }
                    if low > high {
                        Some(parameter)
                    } else {
                        // Both endpoints came from `i64` values, so they still fit.
                        *lower = i64::try_from(low).unwrap_or(*lower);
                        *upper = i64::try_from(high).unwrap_or(*upper);
                        None
                    }
                }
            };
            if let Some(parameter) = emptied {
                return Err(NarrowingRefusal::Unsatisfiable(Box::new(UnsatisfiedPath {
                    guard: guard.occurrence.clone(),
                    parameter: self.parameters[parameter].node.clone(),
                    declared: match declared[parameter] {
                        Narrowed::Boolean { .. } => ScalarGuardDomain::Boolean,
                        Narrowed::Integer { lower, upper } => ScalarGuardDomain::Integer {
                            minimum: lower,
                            maximum: upper,
                        },
                    },
                })));
            }
        }
        Ok(NarrowedGuards(domains))
    }

    /// Narrow each operand range bound to a guard parameter.
    pub(crate) fn narrow_operands(&self, narrowed: &NarrowedGuards, ranges: &mut [(i64, i64)]) {
        for (parameter, domain) in self.parameters.iter().zip(&narrowed.0) {
            let Narrowed::Integer { lower, upper } = domain else {
                continue;
            };
            for position in &parameter.operand_positions {
                if let Some(range) = ranges.get_mut(*position) {
                    range.0 = range.0.max(*lower);
                    range.1 = range.1.min(*upper);
                }
            }
        }
    }

    /// The guard inputs and assumptions of the harness. `operand_names` are the harness's operand
    /// variables, in call order; an operand-read parameter is read through `<name>_native`.
    pub(crate) fn render(
        &self,
        narrowed: &NarrowedGuards,
        operand_names: &[&str],
    ) -> RenderedGuards {
        let mut inputs = Vec::new();
        let mut readers: Vec<Vec<String>> = Vec::with_capacity(self.parameters.len());
        for (parameter, domain) in self.parameters.iter().zip(&narrowed.0) {
            let mut names = parameter
                .operand_positions
                .iter()
                .filter_map(|position| operand_names.get(*position))
                .map(|name| format!("{name}_native"))
                .collect::<Vec<_>>();
            if names.is_empty() {
                let identifier = format!("guard_{}", inputs.len());
                let (primitive, domain, identity) = match *domain {
                    Narrowed::Boolean {
                        can_be_false,
                        can_be_true,
                    } => (
                        KaniPrimitiveType::Boolean,
                        (i128::from(!can_be_false), i128::from(can_be_true)),
                        ScalarGuardDomain::Boolean,
                    ),
                    Narrowed::Integer { lower, upper } => (
                        KaniPrimitiveType::I64,
                        (i128::from(lower), i128::from(upper)),
                        ScalarGuardDomain::Integer {
                            minimum: lower,
                            maximum: upper,
                        },
                    ),
                };
                inputs.push(GuardInput {
                    identity: ScalarGuardInput {
                        identifier: identifier.clone(),
                        parameter: parameter.node.clone(),
                        domain: identity,
                    },
                    primitive,
                    domain,
                });
                names.push(identifier);
            }
            readers.push(names);
        }
        let assumptions = self
            .guards
            .iter()
            .map(|guard| assumption(guard, &readers))
            .collect();
        RenderedGuards {
            inputs,
            assumptions,
        }
    }
}

/// The one assumption of a guard: its comparison (or its negation), required of every reader.
fn assumption(guard: &ResolvedGuard, readers: &[Vec<String>]) -> String {
    let terms: Vec<String> = match guard.form {
        GuardForm::Boolean(parameter) => readers[parameter]
            .iter()
            .map(|reader| {
                if guard.holds {
                    reader.clone()
                } else {
                    format!("!{reader}")
                }
            })
            .collect(),
        GuardForm::Compare {
            operator,
            left,
            right,
        } => {
            let sides = |side: GuardSide| match side {
                GuardSide::Parameter(parameter) => readers[parameter].clone(),
                GuardSide::Literal(value) => vec![i64_literal(value)],
            };
            let (left, right) = (sides(left), sides(right));
            left.iter()
                .flat_map(|left| {
                    right.iter().map(move |right| {
                        let comparison = format!("{left} {} {right}", operator.symbol());
                        if guard.holds {
                            comparison
                        } else {
                            format!("!({comparison})")
                        }
                    })
                })
                .collect()
        }
    };
    terms.join(" && ")
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use serde_json::json;

    use super::*;

    fn node(digit: &str) -> CheckedNodeId {
        serde_json::from_value(json!({"package": "guard-test", "path": [digit], "ordinal": 0}))
            .expect("node id")
    }

    /// One guard `x <operator> literal` over `x: Int[0, 9]`.
    fn plan(operator: Comparison, literal: i64, holds: bool) -> GuardPlan {
        GuardPlan {
            path_condition: Vec::new(),
            parameters: vec![GuardParameter {
                node: node("x"),
                declared: DeclaredDomain::Integer {
                    lower: "0".to_owned(),
                    upper: "9".to_owned(),
                },
                operand_positions: vec![0],
            }],
            guards: vec![ResolvedGuard {
                occurrence: RequirementOccurrence {
                    node_id: node("g"),
                    role: quire_contract_model::CheckedOccurrenceRole::Expression,
                    ordinal: 0,
                },
                holds,
                form: GuardForm::Compare {
                    operator,
                    left: GuardSide::Parameter(0),
                    right: GuardSide::Literal(literal),
                },
            }],
        }
    }

    fn narrowed_range(plan: &GuardPlan) -> Option<(i64, i64)> {
        let narrowed = plan.narrow().ok()?;
        let mut ranges = [(0, 9)];
        plan.narrow_operands(&narrowed, &mut ranges);
        Some(ranges[0])
    }

    /// `=` keeps the value it names; `!=` removes it only from an endpoint, and a negated `=` or
    /// `!=` is the opposite. These forms need a plan of their own because QSL converts an `=`
    /// literal, so no authored guard reaches them.
    #[trace("FR-022-AC-33", "TC-033")]
    #[test]
    fn equality_guards_narrow_by_the_value_they_name() {
        for (operator, literal, holds, expected) in [
            (Comparison::Eq, 5, true, Some((5, 5))),
            (Comparison::Eq, 5, false, Some((0, 9))),
            (Comparison::Eq, 0, false, Some((1, 9))),
            (Comparison::Eq, 9, false, Some((0, 8))),
            (Comparison::Eq, 20, true, None),
            (Comparison::Ne, 0, true, Some((1, 9))),
            (Comparison::Ne, 9, true, Some((0, 8))),
            (Comparison::Ne, 5, true, Some((0, 9))),
            (Comparison::Ne, 9, false, Some((9, 9))),
            (Comparison::Ne, 20, false, None),
        ] {
            assert_eq!(
                narrowed_range(&plan(operator, literal, holds)),
                expected,
                "{operator:?} {literal} holds={holds}"
            );
        }
    }
}
