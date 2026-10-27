//! FR-031: the Boolean oracle's integer arithmetic and comparison take the runtime's meaning.
//!
//! The constructions below are the verified ones of TC-044: each is admitted by
//! `DeclarationEnvironment::check_expression`, and each operand vector that overflows does so
//! through a declared bound the vector violates while the guards still pass.

use ix_trace_rs::trace;
use std::{cell::Cell, fmt::Write as _, fs, path::PathBuf, process::Command};

use crate::scratch_crate::{runtime_dependency, write_manifest, TemporaryDirectory};
use quire_contract_codegen::EXACT_DEPENDENCY_SOURCE;
use quire_contract_codegen::{
    generate_boolean_oracle, GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
    OracleRequest,
};
use quire_contract_model::{
    AnchorName, BooleanOperator, ClauseId, ComparisonOperator, DeclarationEnvironment,
    ExecutionPoint, Expression, ExpressionKind, IntegerDomain, IntegerType, NumericOperator,
    OverflowPolicy, PackageId, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, TypedExpression, ValueDeclaration, ValueDeclarationKind, ValueType,
};
use syn::visit::Visit;

// ---------------------------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------------------------

fn name(value: &str) -> SymbolName {
    SymbolName::new(value).unwrap()
}

fn span(start: u64, end: u64) -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("arithmetic-test").unwrap(),
        SourceRevision::new(1).unwrap(),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, start as u32 + 1, start).unwrap(),
        SourceLocation::new(source, 1, end as u32 + 1, end).unwrap(),
    )
    .unwrap()
}

fn requirement() -> RequirementRef {
    RequirementRef::new(
        PackageId::new("agent-ix/arithmetic-test").unwrap(),
        RequirementId::new("FR-031").unwrap(),
        RequirementRevision::new(3).unwrap(),
    )
}

fn handler() -> ExecutionPoint {
    ExecutionPoint::Handler {
        name: AnchorName::new("generate").unwrap(),
    }
}

fn integer_type(minimum: i64, maximum: i64, overflow: OverflowPolicy) -> IntegerType {
    IntegerType::new(
        IntegerDomain::Signed,
        i128::from(minimum),
        i128::from(maximum),
        overflow,
    )
    .unwrap()
}

/// Builds the nodes of one clause over one integer type, giving every node its own span.
struct Clause {
    value_type: IntegerType,
    next: Cell<u64>,
}

impl Clause {
    fn new(minimum: i64, maximum: i64, overflow: OverflowPolicy) -> Self {
        Self {
            value_type: integer_type(minimum, maximum, overflow),
            next: Cell::new(100),
        }
    }

    fn at(&self) -> SourceSpan {
        let start = self.next.get();
        self.next.set(start + 2);
        span(start, start + 1)
    }

    fn variable(&self, value: &str) -> Expression {
        Expression::new(
            ExpressionKind::ValueReference {
                name: name(value),
                observation: StateObservation::Current,
            },
            self.at(),
        )
    }

    fn literal(&self, value: i64) -> Expression {
        Expression::new(
            ExpressionKind::IntegerLiteral {
                value: i128::from(value),
                value_type: self.value_type.clone(),
            },
            self.at(),
        )
    }

    fn numeric(
        &self,
        operator: NumericOperator,
        left: Expression,
        right: Expression,
    ) -> Expression {
        Expression::new(
            ExpressionKind::Numeric {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            self.at(),
        )
    }

    fn negate(&self, operand: Expression) -> Expression {
        Expression::new(
            ExpressionKind::NumericNegate {
                operand: Box::new(operand),
            },
            self.at(),
        )
    }

    fn compare(
        &self,
        operator: ComparisonOperator,
        left: Expression,
        right: Expression,
    ) -> Expression {
        Expression::new(
            ExpressionKind::Compare {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            self.at(),
        )
    }

    fn connective(
        &self,
        operator: BooleanOperator,
        left: Expression,
        right: Expression,
    ) -> Expression {
        Expression::new(
            ExpressionKind::Boolean {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            self.at(),
        )
    }

    fn and(&self, left: Expression, right: Expression) -> Expression {
        self.connective(BooleanOperator::ShortCircuitAnd, left, right)
    }

    /// A Boolean input read.
    fn flag(&self, value: &str) -> Expression {
        Expression::new(
            ExpressionKind::ValueReference {
                name: name(value),
                observation: StateObservation::Current,
            },
            self.at(),
        )
    }

    /// The environment declaring each of `inputs` as this clause's integer type and each of
    /// `flags` as a Boolean, in the order given.
    fn environment(&self, inputs: &[&str], flags: &[&str]) -> DeclarationEnvironment {
        let mut declarations = Vec::new();
        for (index, value) in inputs.iter().enumerate() {
            declarations.push(ValueDeclaration::new(
                name(value),
                ValueDeclarationKind::Input,
                ValueType::integer(self.value_type.clone()),
                span(index as u64, index as u64 + 1),
            ));
        }
        for (index, value) in flags.iter().enumerate() {
            declarations.push(ValueDeclaration::new(
                name(value),
                ValueDeclarationKind::Input,
                ValueType::Boolean,
                span(50 + index as u64, 51 + index as u64),
            ));
        }
        DeclarationEnvironment::new(requirement(), vec![], declarations, vec![]).unwrap()
    }
}

/// One admitted construction: its environment, its expression, and the span of the node a
/// refusal must name.
pub(crate) struct Built {
    environment: DeclarationEnvironment,
    expression: Expression,
    node: SourceSpan,
}

impl Built {
    fn typed(&self) -> TypedExpression {
        self.environment
            .check_expression(&self.expression, &ValueType::Boolean, &handler(), true)
            .expect("IR admits the construction")
    }

    fn generate(
        &self,
    ) -> Result<quire_contract_codegen::OracleArtifactBundle, Vec<GenerationDiagnostic>> {
        let typed = self.typed();
        let clause = ClauseId::new("clause").unwrap();
        generate_boolean_oracle(&OracleRequest {
            requirement: self.environment.owner(),
            clause: &clause,
            expression: &typed,
        })
    }

    fn source(&self) -> String {
        self.generate()
            .expect("the construction generates")
            .rust
            .contents
    }

    fn refusal(&self) -> GenerationDiagnostic {
        self.generate()
            .expect_err("the construction is refused")
            .remove(0)
    }
}

/// The guarded add, subtract or multiply of O-1, O-2 and O-4: a guard on `x`, then either a
/// second guarded variable `y` or a literal second operand, then the arithmetic node compared
/// with a bound. The guards bound each operand on one side only, so the declared bound on the
/// other side is what makes IR admit the construction, and an operand vector outside it still
/// passes the guards.
#[derive(Clone, Copy)]
struct Pair {
    operator: NumericOperator,
    minimum: i64,
    maximum: i64,
    x_guard: (ComparisonOperator, i64),
    second: Second,
}

#[derive(Clone, Copy)]
enum Second {
    Variable((ComparisonOperator, i64)),
    Literal(i64),
}

/// O-1: `x >= -5 && (y >= -5 && x + y <= 0)` over `-10..=0`, vector `x = i64::MAX`, `y = 1`.
const O1: Pair = Pair {
    operator: NumericOperator::Add,
    minimum: -10,
    maximum: 0,
    x_guard: (ComparisonOperator::GreaterEqual, -5),
    second: Second::Variable((ComparisonOperator::GreaterEqual, -5)),
};

/// O-2: `x <= -5 && (y >= -5 && x - y <= 0)` over `-10..=0`, vector `x = i64::MIN`, `y = 1`.
const O2: Pair = Pair {
    operator: NumericOperator::Subtract,
    minimum: -10,
    maximum: 0,
    x_guard: (ComparisonOperator::LessEqual, -5),
    second: Second::Variable((ComparisonOperator::GreaterEqual, -5)),
};

/// O-4: `x < 5 && x * 2 <= 10` over `0..=10`, vector `x = i64::MIN`.
const O4: Pair = Pair {
    operator: NumericOperator::Multiply,
    minimum: 0,
    maximum: 10,
    x_guard: (ComparisonOperator::Less, 5),
    second: Second::Literal(2),
};

impl Pair {
    fn build(&self, comparison: ComparisonOperator, bound: i64) -> Built {
        let clause = Clause::new(self.minimum, self.maximum, OverflowPolicy::Reject);
        let variables: &[&str] = match self.second {
            Second::Variable(_) => &["x", "y"],
            Second::Literal(_) => &["x"],
        };
        let environment = clause.environment(variables, &[]);
        let x_guard = clause.compare(
            self.x_guard.0,
            clause.variable("x"),
            clause.literal(self.x_guard.1),
        );
        let (second_operand, y_guard) = match self.second {
            Second::Variable((operator, limit)) => (
                clause.variable("y"),
                Some(clause.compare(operator, clause.variable("y"), clause.literal(limit))),
            ),
            Second::Literal(value) => (clause.literal(value), None),
        };
        let node = clause.numeric(self.operator, clause.variable("x"), second_operand);
        let node_span = node.source().clone();
        let relation = clause.compare(comparison, node, clause.literal(bound));
        let expression = match y_guard {
            Some(y_guard) => clause.and(x_guard, clause.and(y_guard, relation)),
            None => clause.and(x_guard, relation),
        };
        Built {
            environment,
            expression,
            node: node_span,
        }
    }

    /// The outcome of the clause on `x` and `y`, from plain integers: the guards, then the exact
    /// result in `i128` against the declared interval. It calls no runtime function and reads
    /// nothing from the emitter.
    fn model(&self, comparison: ComparisonOperator, bound: i64, x: i64, y: i64) -> Expect {
        let Some(second) = self.guarded_second(x, y) else {
            return Expect::Completed(false);
        };
        let (left, right) = (i128::from(x), second);
        let result = match self.operator {
            NumericOperator::Add => left + right,
            NumericOperator::Subtract => left - right,
            NumericOperator::Multiply => left * right,
            NumericOperator::Divide | NumericOperator::Remainder => {
                unreachable!("only add, subtract and multiply are modelled")
            }
        };
        if !(i128::from(self.minimum)..=i128::from(self.maximum)).contains(&result) {
            return Expect::Refused;
        }
        Expect::Completed(holds(comparison, result, i128::from(bound)))
    }

    /// The second operand when both guards pass, `None` when a guard is false.
    fn guarded_second(&self, x: i64, y: i64) -> Option<i128> {
        if !holds(self.x_guard.0, i128::from(x), i128::from(self.x_guard.1)) {
            return None;
        }
        match self.second {
            Second::Variable((operator, limit)) => {
                holds(operator, i128::from(y), i128::from(limit)).then_some(i128::from(y))
            }
            Second::Literal(value) => Some(i128::from(value)),
        }
    }
}

/// What a clause evaluates to on one vector.
#[derive(Clone, Copy, PartialEq)]
enum Expect {
    Completed(bool),
    Refused,
}

fn holds(operator: ComparisonOperator, left: i128, right: i128) -> bool {
    match operator {
        ComparisonOperator::Equal => left == right,
        ComparisonOperator::NotEqual => left != right,
        ComparisonOperator::Less => left < right,
        ComparisonOperator::LessEqual => left <= right,
        ComparisonOperator::Greater => left > right,
        ComparisonOperator::GreaterEqual => left >= right,
    }
}

fn o1_add(comparison: ComparisonOperator, bound: i64) -> Built {
    O1.build(comparison, bound)
}

fn o2_subtract(comparison: ComparisonOperator, bound: i64) -> Built {
    O2.build(comparison, bound)
}

pub(crate) fn o4_multiply(comparison: ComparisonOperator, bound: i64) -> Built {
    O4.build(comparison, bound)
}

/// O-3 (divide) and O-5 (remainder): `y >= -1 && (y <= -1 && x OP y <= 5)` over `-5..=5`,
/// vector `x = i64::MIN`, `y = -1`.
fn o3_o5(operator: NumericOperator) -> Built {
    let clause = Clause::new(-5, 5, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &[]);
    let lower = clause.compare(
        ComparisonOperator::GreaterEqual,
        clause.variable("y"),
        clause.literal(-1),
    );
    let upper = clause.compare(
        ComparisonOperator::LessEqual,
        clause.variable("y"),
        clause.literal(-1),
    );
    let node = clause.numeric(operator, clause.variable("x"), clause.variable("y"));
    let node_span = node.source().clone();
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(5));
    let expression = clause.and(lower, clause.and(upper, relation));
    Built {
        environment,
        expression,
        node: node_span,
    }
}

/// Z-1 and Z-2: `y <= 1 && x / y <= 10` over `1..=10`, vectors `(5, 0)` and `(0, 0)`.
fn z1_z2() -> Built {
    let clause = Clause::new(1, 10, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &[]);
    let guard = clause.compare(
        ComparisonOperator::LessEqual,
        clause.variable("y"),
        clause.literal(1),
    );
    let node = clause.numeric(
        NumericOperator::Divide,
        clause.variable("x"),
        clause.variable("y"),
    );
    let node_span = node.source().clone();
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(10));
    Built {
        environment,
        expression: clause.and(guard, relation),
        node: node_span,
    }
}

/// Z-3 and Z-4: `y <= 0 && x % (y + 11) <= 10` over `-10..=11`, vectors `(5, -11)` and `(0, -11)`.
fn z3_z4() -> Built {
    let clause = Clause::new(-10, 11, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &[]);
    let guard = clause.compare(
        ComparisonOperator::LessEqual,
        clause.variable("y"),
        clause.literal(0),
    );
    let divisor = clause.numeric(
        NumericOperator::Add,
        clause.variable("y"),
        clause.literal(11),
    );
    let node = clause.numeric(NumericOperator::Remainder, clause.variable("x"), divisor);
    let node_span = node.source().clone();
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(10));
    Built {
        environment,
        expression: clause.and(guard, relation),
        node: node_span,
    }
}

/// A guarded full-domain division used for the independent truncating-law grid.
fn division_grid(operator: NumericOperator) -> Built {
    let clause = Clause::new(-10, 10, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &[]);
    let nonzero = clause.compare(
        ComparisonOperator::NotEqual,
        clause.variable("y"),
        clause.literal(0),
    );
    let node = clause.numeric(operator, clause.variable("x"), clause.variable("y"));
    let node_span = node.source().clone();
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(-4));
    Built {
        environment,
        expression: clause.and(nonzero, relation),
        node: node_span,
    }
}

/// R-1 and R-2 ensure the unselected member is never domain-checked.
fn member_only_reproduction(operator: NumericOperator) -> Built {
    let (minimum, maximum) = if operator == NumericOperator::Divide {
        (1, 10)
    } else {
        (-10, 5)
    };
    let clause = Clause::new(minimum, maximum, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &[]);
    let left = if operator == NumericOperator::Divide {
        clause.literal(10)
    } else {
        clause.variable("x")
    };
    let right = if operator == NumericOperator::Divide {
        clause.variable("y")
    } else {
        clause.literal(-1)
    };
    let node = clause.numeric(operator, left, right);
    let node_span = node.source().clone();
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(maximum));
    Built {
        environment,
        expression: relation,
        node: node_span,
    }
}

/// S-1 to S-4: a divide or remainder over a `saturate` type.
fn saturate_division(operator: NumericOperator, guarded: bool) -> Built {
    let (minimum, maximum) = if guarded {
        (i64::MIN, i64::MAX)
    } else {
        (1, 10)
    };
    let clause = Clause::new(minimum, maximum, OverflowPolicy::Saturate);
    let environment = clause.environment(&["x", "y"], &[]);
    let node = clause.numeric(operator, clause.variable("x"), clause.variable("y"));
    let node_span = node.source().clone();
    let bound = if guarded { 0 } else { 10 };
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(bound));
    let expression = if guarded {
        let nonzero = clause.compare(
            ComparisonOperator::NotEqual,
            clause.variable("y"),
            clause.literal(0),
        );
        clause.and(nonzero, relation)
    } else {
        relation
    };
    Built {
        environment,
        expression,
        node: node_span,
    }
}

/// A `saturate` add, subtract or multiply compared with a literal.
fn saturate_arithmetic(
    operator: NumericOperator,
    minimum: i64,
    maximum: i64,
    operand: i64,
    bound: i64,
) -> Built {
    let clause = Clause::new(minimum, maximum, OverflowPolicy::Saturate);
    let environment = clause.environment(&["x"], &[]);
    let node = clause.numeric(operator, clause.variable("x"), clause.literal(operand));
    let node_span = node.source().clone();
    let expression = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(bound));
    Built {
        environment,
        expression,
        node: node_span,
    }
}

// ---------------------------------------------------------------------------------------------
// Source scanning
// ---------------------------------------------------------------------------------------------

/// What a generated source holds: binary operators, method names, macro names and called paths,
/// read from its syntax tree so text in a comment or a literal is not mistaken for code.
#[derive(Default)]
pub(crate) struct Scan {
    binary: Vec<&'static str>,
    methods: Vec<String>,
    macros: Vec<String>,
    calls: Vec<String>,
}

fn binary_spelling(operator: &syn::BinOp) -> &'static str {
    match operator {
        syn::BinOp::Add(_) => "+",
        syn::BinOp::Sub(_) => "-",
        syn::BinOp::Mul(_) => "*",
        syn::BinOp::Div(_) => "/",
        syn::BinOp::Rem(_) => "%",
        syn::BinOp::And(_) => "&&",
        syn::BinOp::Or(_) => "||",
        syn::BinOp::BitXor(_) => "^",
        syn::BinOp::BitAnd(_) => "&",
        syn::BinOp::BitOr(_) => "|",
        syn::BinOp::Shl(_) => "<<",
        syn::BinOp::Shr(_) => ">>",
        syn::BinOp::Eq(_) => "==",
        syn::BinOp::Lt(_) => "<",
        syn::BinOp::Le(_) => "<=",
        syn::BinOp::Ne(_) => "!=",
        syn::BinOp::Ge(_) => ">=",
        syn::BinOp::Gt(_) => ">",
        _ => "compound assignment",
    }
}

fn path_text(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        self.binary.push(binary_spelling(&node.op));
        syn::visit::visit_expr_binary(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.methods.push(node.method.to_string());
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func {
            self.calls.push(path_text(&path.path));
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.macros.push(path_text(&node.path));
        syn::visit::visit_macro(self, node);
    }
}

pub(crate) fn scan(source: &str) -> Scan {
    let file = syn::parse_file(source).expect("the generated source parses");
    let mut scan = Scan::default();
    scan.visit_file(&file);
    scan
}

impl Scan {
    pub(crate) fn count(&self, operator: &str) -> usize {
        self.binary
            .iter()
            .filter(|found| **found == operator)
            .count()
    }

    fn calls_to(&self, suffix: &str) -> usize {
        self.calls
            .iter()
            .filter(|call| call.ends_with(suffix))
            .count()
    }
}

/// The spellings of an arithmetic operator the native oracle must never emit between operands.
const ARITHMETIC: [&str; 5] = ["+", "-", "*", "/", "%"];

/// The method families the native oracle must never emit for add, subtract or multiply.
const WRAPPING_FAMILIES: [&str; 4] = ["checked_", "wrapping_", "saturating_", "overflowing_"];

fn assert_no_raw_arithmetic(source: &str) {
    let scanned = scan(source);
    for operator in ARITHMETIC {
        assert_eq!(
            scanned.count(operator),
            0,
            "no Rust `{operator}` between operands:\n{source}"
        );
    }
    for method in &scanned.methods {
        assert!(
            WRAPPING_FAMILIES
                .iter()
                .all(|family| !method.starts_with(family)),
            "no `{method}` call:\n{source}"
        );
    }
}

/// The first `pub fn` signature line of a generated oracle.
fn signature(source: &str) -> &str {
    source
        .lines()
        .find(|line| line.starts_with("pub fn "))
        .expect("a generated oracle has one public function")
}

fn symbol(source: &str) -> &str {
    signature(source)
        .strip_prefix("pub fn ")
        .and_then(|rest| rest.split('(').next())
        .expect("a symbol")
}

/// The parameter names of the oracle, in signature order, without the meter.
fn parameters(source: &str) -> Vec<String> {
    let line = signature(source);
    let inner = line
        .split_once('(')
        .and_then(|(_, rest)| rest.rsplit_once(')'))
        .map(|(inner, _)| inner)
        .expect("a parameter list");
    inner
        .split(", ")
        .filter(|parameter| !parameter.is_empty() && !parameter.starts_with("meter"))
        .map(|parameter| {
            parameter
                .split(':')
                .next()
                .expect("a parameter name")
                .to_owned()
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Generation: the shape of the native oracle (AC-1, AC-3, AC-12 to AC-15)
// ---------------------------------------------------------------------------------------------

const COMPARISONS: [ComparisonOperator; 6] = [
    ComparisonOperator::Equal,
    ComparisonOperator::NotEqual,
    ComparisonOperator::Less,
    ComparisonOperator::LessEqual,
    ComparisonOperator::Greater,
    ComparisonOperator::GreaterEqual,
];

/// Literal-only arithmetic compared with a literal over `0..=10`: IR admits it with no guard.
fn literal_arithmetic(comparison: ComparisonOperator) -> Built {
    let clause = Clause::new(0, 10, OverflowPolicy::Reject);
    let environment = clause.environment(&[], &[]);
    let node = clause.numeric(NumericOperator::Add, clause.literal(1), clause.literal(2));
    let node_span = node.source().clone();
    let expression = clause.compare(comparison, node, clause.literal(3));
    Built {
        environment,
        expression,
        node: node_span,
    }
}

/// A clause with no arithmetic node: `flag implies (x < 3 or not (x = y))` over `-10..=10`.
fn arithmetic_free() -> Built {
    let clause = Clause::new(-10, 10, OverflowPolicy::Reject);
    let environment = clause.environment(&["x", "y"], &["flag"]);
    let less = clause.compare(
        ComparisonOperator::Less,
        clause.variable("x"),
        clause.literal(3),
    );
    let equal = clause.compare(
        ComparisonOperator::Equal,
        clause.variable("x"),
        clause.variable("y"),
    );
    let negated = Expression::new(
        ExpressionKind::BooleanNot {
            operand: Box::new(equal),
        },
        clause.at(),
    );
    let either = clause.connective(BooleanOperator::TotalOr, less, negated);
    let node = either.source().clone();
    let expression = clause.connective(BooleanOperator::Implication, clause.flag("flag"), either);
    Built {
        environment,
        expression,
        node,
    }
}

/// FR-031-AC-1: add, subtract and multiply over a `reject` type are the runtime's
/// `evaluate_integer_arithmetic` with their variant and the type's interval, and the body holds
/// no Rust arithmetic operator and no wrapping, checked, saturating or overflowing method.
#[trace("TC-044", "FR-031-AC-1")]
#[test]
fn tc_044_native_arithmetic_is_the_runtime_exact_operation_over_the_types_interval() {
    for (built, variant, interval) in [
        (
            o1_add(ComparisonOperator::LessEqual, 0),
            "Add",
            "bound(-10_i64, 0_i64)",
        ),
        (
            o2_subtract(ComparisonOperator::LessEqual, 0),
            "Subtract",
            "bound(-10_i64, 0_i64)",
        ),
        (
            o4_multiply(ComparisonOperator::LessEqual, 10),
            "Multiply",
            "bound(0_i64, 10_i64)",
        ),
    ] {
        let source = built.source();
        assert_no_raw_arithmetic(&source);
        let scanned = scan(&source);
        assert_eq!(
            scanned.calls_to("evaluate_integer_arithmetic"),
            1,
            "one runtime call per arithmetic node:\n{source}"
        );
        assert!(
            source.contains(&format!("rt::IntegerArithmetic::{variant}(")),
            "{variant}:\n{source}"
        );
        assert!(source.contains(interval), "{interval}:\n{source}");
        assert!(
            !source.contains("Charge"),
            "no charge amount appears in the source:\n{source}"
        );
    }
}

/// FR-031-AC-3: an oracle with an arithmetic node returns `Outcome<bool>` and takes a trailing
/// `&mut Meter`; one with none returns `bool` and takes no meter.
#[trace("TC-044", "FR-031-AC-3")]
#[test]
fn tc_044_an_arithmetic_oracle_returns_an_outcome_and_takes_a_meter() {
    let arithmetic = o4_multiply(ComparisonOperator::LessEqual, 10).source();
    let line = signature(&arithmetic);
    assert!(
        line.ends_with("(x_current: i64, meter: &mut rt::Meter) -> rt::Outcome<bool> {"),
        "{line}"
    );
    let literal_only = literal_arithmetic(ComparisonOperator::Equal).source();
    assert!(
        signature(&literal_only).ends_with("(meter: &mut rt::Meter) -> rt::Outcome<bool> {"),
        "{}",
        signature(&literal_only)
    );
    let free = arithmetic_free().source();
    let line = signature(&free);
    assert!(line.ends_with(") -> bool {"), "{line}");
    assert!(!line.contains("meter"), "{line}");
    assert!(
        !free.contains("rt::"),
        "an arithmetic-free oracle names no runtime path"
    );
}

/// The arithmetic-free oracle's expected bytes, including its required NFR-002 header.
/// They remain verbatim so a source change is a visible diff.
const ARITHMETIC_FREE_ORACLE: &str = "// SPDX-License-Identifier: MIT OR Apache-2.0
// @generated by quire-contract-codegen. Do not edit.
// Requirement: FR-031@3; Clause: clause

/// Generated contract identity for `FR-031@3`.
pub const ORACLE_FR_031_3_CLAUSE_IDENTITY: quire_contract_runtime::ContractIdentity<'static> =
quire_contract_runtime::ContractIdentity::new(
quire_contract_runtime::RequirementId::new(\"FR-031\"),
quire_contract_runtime::RevisionId::new(\"3\"),
);
/// Generated clause identity for `clause`.
pub const ORACLE_FR_031_3_CLAUSE_CLAUSE: quire_contract_runtime::ClauseId<'static> =
quire_contract_runtime::ClauseId::new(\"clause\");
/// Evaluates generated oracle `FR-031@3/clause`.
#[must_use]
pub fn oracle_fr_031_3_clause(flag_current: bool, x_current: i64, y_current: i64) -> bool {
quire_contract_runtime::operators::implies_short_circuit(
flag_current
,
|| {
quire_contract_runtime::operators::or_total(
|| {
(
x_current
)
<
(
3_i64
)
},
|| {
!(
(
x_current
)
==
(
y_current
)
)
},
)
},
)
}
";

/// FR-031-AC-12: an arithmetic-free oracle retains its expected bytes apart from the required
/// NFR-002 header change.
#[trace("TC-044", "FR-031-AC-12")]
#[test]
fn tc_044_an_arithmetic_free_oracle_is_byte_identical_to_the_one_before() {
    assert_eq!(arithmetic_free().source(), ARITHMETIC_FREE_ORACLE);
}

/// FR-031-AC-13: generating an arithmetic oracle twice, and from a request whose declarations are
/// permuted, yields identical bytes.
#[trace("TC-044", "FR-031-AC-13")]
#[test]
fn tc_044_an_arithmetic_oracle_is_deterministic_and_independent_of_declaration_order() {
    let first = o1_add(ComparisonOperator::LessEqual, 0);
    let bytes = first.generate().unwrap();
    assert_eq!(bytes, first.generate().unwrap());
    let permuted = {
        let mut built = o1_add(ComparisonOperator::LessEqual, 0);
        let clause = Clause::new(-10, 0, OverflowPolicy::Reject);
        built.environment = clause.environment(&["y", "x"], &[]);
        built
    };
    assert_eq!(bytes, permuted.generate().unwrap());
}

/// FR-031-AC-14: the generated source of an arithmetic oracle holds no `unwrap`, `expect` or
/// panic macro, for every connective and negation the arithmetic can sit under.
#[trace("TC-044", "FR-031-AC-14")]
#[test]
fn tc_044_an_arithmetic_oracle_holds_no_unwrap_expect_or_panic_macro() {
    let mut sources = vec![
        o1_add(ComparisonOperator::LessEqual, 0).source(),
        o2_subtract(ComparisonOperator::Equal, 0).source(),
        o4_multiply(ComparisonOperator::LessEqual, 10).source(),
    ];
    sources.extend(
        propagation_clauses()
            .iter()
            .map(|(_, built)| built.source()),
    );
    for source in sources {
        let scanned = scan(&source);
        for method in &scanned.methods {
            assert!(
                !method.starts_with("unwrap") && method != "expect",
                "`{method}`:\n{source}"
            );
        }
        for found in &scanned.macros {
            assert!(
                !matches!(
                    found.as_str(),
                    "panic" | "unreachable" | "todo" | "unimplemented" | "assert" | "assert_eq"
                ),
                "`{found}!`:\n{source}"
            );
        }
        assert!(scanned.macros.is_empty(), "{:?}", scanned.macros);
    }
}

/// FR-031-AC-15: each of the six comparisons with an arithmetic operand is the runtime call the
/// table names, with no Rust comparison operator between its operands.
#[trace("TC-044", "FR-031-AC-15")]
#[test]
fn tc_044_a_comparison_with_an_arithmetic_operand_is_the_runtime_call() {
    for comparison in COMPARISONS {
        let source = literal_arithmetic(comparison).source();
        let scanned = scan(&source);
        for operator in ["==", "!=", "<", "<=", ">", ">="] {
            assert_eq!(scanned.count(operator), 0, "`{operator}`:\n{source}");
        }
        let expected = match comparison {
            ComparisonOperator::Equal => "equality(true,",
            ComparisonOperator::NotEqual => "equality(false,",
            ComparisonOperator::Less => "rt::order_numbers(rt::OrderingOperator::Less,",
            ComparisonOperator::LessEqual => "rt::order_numbers(rt::OrderingOperator::LessOrEqual,",
            ComparisonOperator::Greater => "rt::order_numbers(rt::OrderingOperator::Greater,",
            ComparisonOperator::GreaterEqual => {
                "rt::order_numbers(rt::OrderingOperator::GreaterOrEqual,"
            }
        };
        assert!(source.contains(expected), "{comparison:?}:\n{source}");
        if matches!(
            comparison,
            ComparisonOperator::Equal | ComparisonOperator::NotEqual
        ) {
            assert!(source.contains("planned_equality("), "{source}");
        }
    }
}

/// `left OP right` over two arithmetic-free integers of `i64::MIN..=i64::MAX`.
fn plain_comparison(comparison: ComparisonOperator) -> Built {
    let clause = Clause::new(i64::MIN, i64::MAX, OverflowPolicy::Reject);
    let environment = clause.environment(&["left", "right"], &[]);
    let expression = clause.compare(
        comparison,
        clause.variable("left"),
        clause.variable("right"),
    );
    let node = expression.source().clone();
    Built {
        environment,
        expression,
        node,
    }
}

/// FR-031-AC-4, generation half: an arithmetic-free comparison is the one native operator the
/// table names and returns a plain `bool`.
#[trace("TC-044", "FR-031-AC-4")]
#[test]
fn tc_044_an_arithmetic_free_comparison_keeps_the_native_operator() {
    for comparison in COMPARISONS {
        let source = plain_comparison(comparison).source();
        let expected = match comparison {
            ComparisonOperator::Equal => "==",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::Less => "<",
            ComparisonOperator::LessEqual => "<=",
            ComparisonOperator::Greater => ">",
            ComparisonOperator::GreaterEqual => ">=",
        };
        let scanned = scan(&source);
        assert_eq!(scanned.binary, [expected], "{comparison:?}:\n{source}");
        assert!(signature(&source).ends_with(") -> bool {"));
        assert!(!source.contains("rt::"));
    }
}

// ---------------------------------------------------------------------------------------------
// Generation: refusals (AC-7, AC-18)
// ---------------------------------------------------------------------------------------------

/// FR-031-AC-7: an add, subtract or multiply over a `saturate` type is refused with
/// `UnsupportedSaturatingArithmetic` at the node's span, naming the missing runtime operation,
/// with no artifact. On the tree before the change each of these generated.
#[trace("TC-044", "FR-031-AC-7")]
#[test]
fn tc_044_saturating_arithmetic_is_refused_at_its_node() {
    for built in [
        saturate_arithmetic(NumericOperator::Add, i64::MIN, i64::MAX, 1, 0),
        saturate_arithmetic(NumericOperator::Subtract, i64::MIN, i64::MAX, 1, 0),
        saturate_arithmetic(NumericOperator::Multiply, 0, 10, 2, 10),
    ] {
        let refusal = built.refusal();
        assert_eq!(
            refusal.code,
            GenerationErrorCode::UnsupportedSaturatingArithmetic
        );
        assert_eq!(refusal.terminal_state, GenerationTerminalState::Unsupported);
        assert_eq!(refusal.source_span.as_ref(), Some(&built.node));
        assert!(
            refusal
                .message
                .contains("saturating integer operation in the exact kernel"),
            "{}",
            refusal.message
        );
    }
}

/// FR-031-AC-7: the refusal locus is the first unsupported node in authored preorder, so a
/// `saturate` addition followed by a negation is refused at the addition, and a negation
/// followed by one is refused at the negation.
#[trace("TC-044", "FR-031-AC-7")]
#[test]
fn tc_044_the_refusal_locus_is_the_first_unsupported_node_in_preorder() {
    let clause = Clause::new(-10, 10, OverflowPolicy::Saturate);
    let environment = clause.environment(&[], &[]);
    let addition = clause.numeric(NumericOperator::Add, clause.literal(1), clause.literal(1));
    let addition_span = addition.source().clone();
    let first = clause.compare(ComparisonOperator::Equal, addition, clause.literal(2));
    let negation = clause.negate(clause.literal(1));
    let negation_span = negation.source().clone();
    let second = clause.compare(ComparisonOperator::Equal, negation, clause.literal(-1));
    let forward = Built {
        environment: environment.clone(),
        expression: clause.connective(BooleanOperator::TotalAnd, first, second),
        node: addition_span.clone(),
    };
    let refusal = forward.refusal();
    assert_eq!(
        refusal.code,
        GenerationErrorCode::UnsupportedSaturatingArithmetic
    );
    assert_eq!(refusal.source_span.as_ref(), Some(&addition_span));

    let addition = clause.numeric(NumericOperator::Add, clause.literal(1), clause.literal(1));
    let first = clause.compare(ComparisonOperator::Equal, addition, clause.literal(2));
    let negation = clause.negate(clause.literal(1));
    let negation_span_first = negation.source().clone();
    let second = clause.compare(ComparisonOperator::Equal, negation, clause.literal(-1));
    let backward = Built {
        environment,
        expression: clause.connective(BooleanOperator::TotalAnd, second, first),
        node: negation_span_first.clone(),
    };
    let refusal = backward.refusal();
    assert_eq!(refusal.code, GenerationErrorCode::UnsupportedExpression);
    assert_eq!(refusal.source_span.as_ref(), Some(&negation_span_first));
    assert_ne!(negation_span, negation_span_first);
}

/// The four saturating division constructions of TC-044 step 11.
fn saturating_division_constructions() -> Vec<(&'static str, Built)> {
    vec![
        ("S-1", saturate_division(NumericOperator::Divide, false)),
        ("S-2", saturate_division(NumericOperator::Remainder, false)),
        ("S-3", saturate_division(NumericOperator::Divide, true)),
        ("S-4", saturate_division(NumericOperator::Remainder, true)),
    ]
}

/// FR-031-AC-18: saturating division is refused at its own node with the typed code.
#[trace("TC-044", "FR-031-AC-18")]
#[test]
fn tc_044_saturating_divide_and_remainder_are_refused_by_the_native_oracle() {
    for (case, built) in saturating_division_constructions() {
        let refusal = built.refusal();
        assert_eq!(
            refusal.code,
            GenerationErrorCode::UnsupportedSaturatingArithmetic,
            "{case}"
        );
        assert_eq!(
            refusal.terminal_state,
            GenerationTerminalState::Unsupported,
            "{case}"
        );
        assert_eq!(refusal.source_span.as_ref(), Some(&built.node), "{case}");
        assert!(
            refusal.message.contains("saturating integer operation"),
            "{case}: {}",
            refusal.message
        );
        assert!(built.generate().is_err(), "{case}");
    }
}

/// The source selects one exact division member and bounds only that member.
#[trace("TC-044", "FR-031-AC-2", "FR-031-AC-22")]
#[test]
fn tc_044_native_division_uses_one_exact_member_and_domain() {
    for (built, member, interval) in [
        (z1_z2(), "Quotient", "bound(1_i64, 10_i64)"),
        (
            o3_o5(NumericOperator::Remainder),
            "Remainder",
            "bound(-5_i64, 5_i64)",
        ),
    ] {
        let source = built.source();
        let scanned = scan(&source);
        assert_no_raw_arithmetic(&source);
        assert_eq!(scanned.calls_to("rt::divide"), 1, "{source}");
        assert_eq!(scanned.calls_to("rt::modulo"), 0, "{source}");
        assert!(
            source.contains("rt::DivisionProfile::Truncating"),
            "{source}"
        );
        assert!(
            source.contains(&format!("rt::DivisionMember::{member}")),
            "{source}"
        );
        assert!(source.contains(interval), "{source}");
        assert!(
            source.contains("rt::IntegerDomain::Bounded(interval)"),
            "{source}"
        );
        assert!(source.contains("use quire_exact as rt;"), "{source}");
        assert!(signature(&source).ends_with("meter: &mut rt::Meter) -> rt::Outcome<bool> {"));
        assert!(
            !source.contains("quire_contract_runtime::exact"),
            "{source}"
        );
        assert!(
            !source.contains("quire_contract_runtime::operators"),
            "{source}"
        );
        assert!(!source.contains("evaluate_boolean"), "{source}");
    }
}

/// Zero, overflow and compound-divisor paths run in generated Rust without a panic.
#[trace(
    "TC-044",
    "FR-031-AC-6",
    "FR-031-AC-16",
    "FR-031-AC-22",
    "FR-031-AC-26"
)]
#[test]
fn tc_044_native_division_zero_and_minimum_vectors() {
    let cases = [
        ("z1", z1_z2(), "5", "0", "zero"),
        ("z2", z1_z2(), "0", "0", "zero"),
        ("z3", z3_z4(), "5", "-11", "zero"),
        ("z4", z3_z4(), "0", "-11", "zero"),
        (
            "o3",
            o3_o5(NumericOperator::Divide),
            "i64::MIN",
            "-1",
            "quotient",
        ),
        (
            "o5",
            o3_o5(NumericOperator::Remainder),
            "i64::MIN",
            "-1",
            "true",
        ),
    ];
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, (label, built, x, y, expected)) in cases.into_iter().enumerate() {
        let source = built.source();
        let module = module_of(index, "division_vector");
        let invocation = call(
            &format!("crate::{module}"),
            &source,
            &[("x_current", x), ("y_current", y)],
            "&mut meter()",
        );
        let assertion = match expected {
            "zero" => "assert_eq!(outcome, rt::Outcome::Undefined(rt::Undefined::DivisionByZero));",
            "quotient" => "match outcome { rt::Outcome::Refused(reason @ rt::Refusal::DivisionOutOfDomain { member: rt::DivisionMember::Quotient, .. }) => { assert_eq!(reason.code(), Some(\"division_out_of_domain\")); assert_eq!(reason.cause(), Some(\"quotient-outside-domain\")); if let rt::Refusal::DivisionOutOfDomain { domain, .. } = &reason { assert_eq!(domain.lower(), &rt::Integer::from(-5)); assert_eq!(domain.upper(), &rt::Integer::from(5)); } }, other => panic!(\"{other:?}\") }",
            _ => "assert_eq!(outcome, rt::Outcome::Completed(true));",
        };
        writeln!(
            checks,
            "#[test]\nfn {label}() {{ let outcome = {invocation}; {assertion} }}"
        )
        .unwrap();
        modules.push((module, source));
    }
    run_crate("division-vectors", &modules, &checks);
}

/// Compare generated outcomes with plain i128 selected-member arithmetic, including both
/// member-only reproductions and the negative quotient that separates truncation from floor.
#[trace("TC-044", "FR-031-AC-17")]
#[test]
fn tc_044_native_division_matches_independent_member_model() {
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, operator) in [NumericOperator::Divide, NumericOperator::Remainder]
        .into_iter()
        .enumerate()
    {
        let source = division_grid(operator).source();
        let module = module_of(index, "division_grid");
        let operation = if operator == NumericOperator::Divide {
            "/"
        } else {
            "%"
        };
        let invocation = call(
            &format!("crate::{module}"),
            &source,
            &[("x_current", "x"), ("y_current", "y")],
            "&mut meter()",
        );
        writeln!(checks, "#[test]\nfn grid_{index}() {{
            let values: &[i64] = &[-12, -10, -7, -1, 0, 1, 2, 5, 10, 12];
            for &x in values {{ for &y in values {{
                let outcome = {invocation};
                let expected = if y == 0 {{ rt::Outcome::Completed(false) }} else {{
                    let member = i128::from(x) {operation} i128::from(y);
                    if !(-10..=10).contains(&member) {{
                        rt::Outcome::Refused(rt::Refusal::DivisionOutOfDomain {{
                            domain: Box::new(rt::IntegerInterval::new(rt::Integer::from(-10), rt::Integer::from(10)).unwrap()),
                            member: rt::DivisionMember::{},
                        }})
                    }} else {{ rt::Outcome::Completed(member <= -4) }}
                }};
                assert_eq!(outcome, expected, \"x={{x}}, y={{y}}\");
            }} }}
        }}", if operator == NumericOperator::Divide { "Quotient" } else { "Remainder" }).unwrap();
        modules.push((module, source));
    }
    for (index, (operator, x, y)) in [
        (NumericOperator::Divide, "0", "5"),
        (NumericOperator::Remainder, "-10", "0"),
    ]
    .into_iter()
    .enumerate()
    {
        let source = member_only_reproduction(operator).source();
        let module = module_of(index, "member_only");
        let invocation = call(
            &format!("crate::{module}"),
            &source,
            &[("x_current", x), ("y_current", y)],
            "&mut meter()",
        );
        let (dividend, divisor, minimum, maximum) = if operator == NumericOperator::Divide {
            ("10", "5", "1", "10")
        } else {
            ("-10", "-1", "-10", "5")
        };
        let selected = if operator == NumericOperator::Divide {
            "quotient"
        } else {
            "remainder"
        };
        let rejected = if operator == NumericOperator::Divide {
            "remainder"
        } else {
            "quotient"
        };
        writeln!(
            checks,
            "#[test]\nfn reproduction_{index}() {{
            let dividend = {dividend}_i128;
            let divisor = {divisor}_i128;
            let quotient = dividend / divisor;
            let remainder = dividend % divisor;
            assert!(({minimum}..={maximum}).contains(&{selected}));
            assert!(!({minimum}..={maximum}).contains(&{rejected}));
            assert_eq!({invocation}, rt::Outcome::Completed({selected} <= {maximum}));
        }}"
        )
        .unwrap();
        modules.push((module, source));
    }
    run_crate("division-model", &modules, &checks);
}

/// The mixed add/remainder clause retains its completed connective after both integer results;
/// denial of that final point leaves the two admitted result charges intact.
#[trace("TC-044", "FR-031-AC-22", "FR-031-AC-26")]
#[test]
fn tc_044_mixed_division_connective_retain_denial_is_typed() {
    let source = z3_z4().source();
    let module = module_of(0, "division_retain");
    let completed = call(
        &format!("crate::{module}"),
        &source,
        &[("x_current", "5"), ("y_current", "-10")],
        "&mut meter()",
    );
    let denied = call(
        &format!("crate::{module}"),
        &source,
        &[("x_current", "5"), ("y_current", "-10")],
        "&mut limited",
    );
    let checks = format!(
        "#[test]\nfn completed_and_denied() {{
            assert_eq!({completed}, rt::Outcome::Completed(true));
            let mut limited = rt::Meter::new(rt::ScalarLimits {{ result_units: 2, ..UNLIMITED }});
            match {denied} {{
                rt::Outcome::Incomplete(stop) => assert_eq!(stop.charge_point, rt::ChargePoint::BooleanResultRetain),
                other => panic!(\"expected denied Boolean retain, got {{other:?}}\"),
            }}
            assert_eq!(limited.consumed(rt::LimitKind::ResultUnits), 2);
            assert_eq!(limited.admitted_charges().iter().filter(|charge| charge.point == rt::ChargePoint::BooleanResultRetain).count(), 0);
        }}"
    );
    run_crate("division-retain", &[(module, source)], &checks);
}

// ---------------------------------------------------------------------------------------------
// Generated crates
// ---------------------------------------------------------------------------------------------

/// The unlimited meter the generated checks run under unless a test sets a limit.
const CHECK_PRELUDE: &str = "use quire_exact as rt;

const UNLIMITED: rt::ScalarLimits = rt::ScalarLimits {
    integer_bits: u64::MAX,
    decimal_digits: u64::MAX,
    scale_expansion: u64::MAX,
    text_input_bytes: u64::MAX,
    text_scalars: u64::MAX,
    normalized_scalars: u64::MAX,
    unit_edges: u64::MAX,
    value_occurrences: u64::MAX,
    work_units: u64::MAX,
    result_units: u64::MAX,
};

fn meter() -> rt::Meter {
    rt::Meter::new(UNLIMITED)
}

/// 0 false, 1 true, 2 `Refused(IntegerOutOfDomain)`, 3 incomplete, 4 anything else.
fn kind(outcome: rt::Outcome<bool>) -> u8 {
    match outcome {
        rt::Outcome::Completed(false) => 0,
        rt::Outcome::Completed(true) => 1,
        rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain { .. }) => 2,
        rt::Outcome::Incomplete(_) => 3,
        _ => 4,
    }
}
";

/// Writes `modules` (name and source) and `checks` into one scratch crate, compiles it warning-
/// clean and runs its tests against `quire-contract-runtime`, so generated code is executed.
/// Every crate shares one target directory, so the runtime builds once.
fn run_crate(label: &str, modules: &[(String, String)], checks: &str) {
    let directory = TemporaryDirectory::new(&format!("quire-arithmetic-{label}"));
    fs::create_dir_all(directory.path.join("src/generated")).unwrap();
    let mut root = String::from("#![deny(warnings)]\n#![allow(unexpected_cfgs)]\n\n");
    for (module, source) in modules {
        fs::write(
            directory.path.join(format!("src/generated/{module}.rs")),
            source,
        )
        .unwrap();
        writeln!(
            root,
            "#[path = \"generated/{module}.rs\"]\npub mod {module};"
        )
        .unwrap();
    }
    write!(
        root,
        // The prelude's helpers are shared by every crate; each crate uses some of them.
        "\n#[cfg(test)]\nmod checks {{\n#![allow(dead_code)]\n{CHECK_PRELUDE}\n{checks}\n}}\n"
    )
    .unwrap();
    fs::write(directory.path.join("src/lib.rs"), root).unwrap();
    write_manifest(
        &directory.path,
        &format!(
            "[package]\nname = \"arithmetic-{label}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("oracle-arithmetic"),
        )
        .current_dir(&directory.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated crate `{label}` failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The call of `module`'s oracle with `arguments` (parameter name, Rust value) in signature
/// order, under `meter`.
fn call(module: &str, source: &str, arguments: &[(&str, &str)], meter: &str) -> String {
    let mut values = parameters(source)
        .iter()
        .map(|parameter| {
            arguments
                .iter()
                .find(|(name, _)| name == parameter)
                .map(|(_, value)| (*value).to_owned())
                .unwrap_or_else(|| panic!("no value for `{parameter}`"))
        })
        .collect::<Vec<_>>();
    values.push(meter.to_owned());
    format!("{module}::{}({})", symbol(source), values.join(", "))
}

fn module_of(index: usize, label: &str) -> String {
    format!("{label}_{index}")
}

// ---------------------------------------------------------------------------------------------
// Overflow, the differential and agreement with the runtime (AC-4, AC-5, AC-8)
// ---------------------------------------------------------------------------------------------

/// FR-031-AC-5: under `reject`, an add, subtract or multiply result outside the type's interval
/// is `Refused(IntegerOutOfDomain)`, never a panic or a wrapped value: O-1, O-2 and O-4.
#[trace("TC-044", "FR-031-AC-5")]
#[test]
fn tc_044_overflow_under_reject_is_the_runtimes_refusal() {
    let cases = [
        (
            o1_add(ComparisonOperator::LessEqual, 0),
            [("x_current", "i64::MAX"), ("y_current", "1")],
        ),
        (
            o2_subtract(ComparisonOperator::LessEqual, 0),
            [("x_current", "i64::MIN"), ("y_current", "1")],
        ),
        (
            o4_multiply(ComparisonOperator::LessEqual, 10),
            [("x_current", "i64::MIN"), ("y_current", "")],
        ),
    ];
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, (built, vector)) in cases.iter().enumerate() {
        let source = built.source();
        let module = module_of(index, "overflow");
        let arguments = vector
            .iter()
            .filter(|(_, value)| !value.is_empty())
            .copied()
            .collect::<Vec<_>>();
        writeln!(
            checks,
            "#[test]\nfn case_{index}() {{\n    let outcome = {};\n    assert!(matches!(outcome, rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain {{ .. }})), \"{{outcome:?}}\");\n}}\n",
            call(&format!("crate::{module}"), &source, &arguments, "&mut meter()")
        )
        .unwrap();
        modules.push((module, source));
    }
    run_crate("overflow", &modules, &checks);
}

/// The operand values the differential grid covers: inside the declared domains, at their edges
/// and outside them, up to the extremes of `i64`.
const GRID: [i64; 17] = [
    i64::MIN,
    i64::MIN + 1,
    -11,
    -10,
    -6,
    -5,
    -4,
    -1,
    0,
    1,
    4,
    5,
    6,
    10,
    11,
    i64::MAX - 1,
    i64::MAX,
];

/// FR-031-AC-8: in a generated crate run against the runtime, the outcome of an add, subtract or
/// multiply oracle equals a plain-integer model: `Completed` with the comparison of the exact
/// `i128` result when it lies in the type's interval, `Refused(IntegerOutOfDomain)` when it does
/// not, with the outcome kind compared on every vector of the grid.
#[trace("TC-044", "FR-031-AC-8")]
#[test]
fn tc_044_arithmetic_outcomes_equal_an_independent_plain_integer_model() {
    let mut modules = Vec::new();
    let mut checks = String::new();
    let mut refusals = 0;
    let mut completions = 0;
    for (index, (pair, comparison, bound)) in [
        (O1, ComparisonOperator::LessEqual, -3),
        (O1, ComparisonOperator::Equal, -3),
        (O2, ComparisonOperator::LessEqual, -3),
        (O2, ComparisonOperator::NotEqual, -3),
        (O4, ComparisonOperator::GreaterEqual, 4),
        (O4, ComparisonOperator::Equal, 4),
    ]
    .into_iter()
    .enumerate()
    {
        let source = pair.build(comparison, bound).source();
        let module = module_of(index, "grid");
        let mut rows = String::new();
        for x in GRID {
            for y in GRID {
                let expected = match pair.model(comparison, bound, x, y) {
                    Expect::Completed(false) => 0,
                    Expect::Completed(true) => 1,
                    Expect::Refused => 2,
                };
                refusals += usize::from(expected == 2);
                completions += usize::from(expected == 1);
                writeln!(rows, "        ({x}_i64, {y}_i64, {expected}),").unwrap();
            }
        }
        let arguments = [("x_current", "x"), ("y_current", "y")];
        let arguments = arguments
            .iter()
            .filter(|(name, _)| {
                parameters(&source)
                    .iter()
                    .any(|parameter| parameter == name)
            })
            .copied()
            .collect::<Vec<_>>();
        writeln!(
            checks,
            "#[test]\nfn grid_{index}() {{\n    let rows: &[(i64, i64, u8)] = &[\n{rows}    ];\n    for &(x, y, expected) in rows {{\n        let _ = (x, y);\n        let outcome = {};\n        assert_eq!(kind(outcome), expected, \"x = {{x}}, y = {{y}}\");\n    }}\n}}\n",
            call(&format!("crate::{module}"), &source, &arguments, "&mut meter()")
        )
        .unwrap();
        modules.push((module, source));
    }
    assert!(refusals > 100, "the grid must reach refusals: {refusals}");
    assert!(
        completions > 5,
        "the grid must reach true results: {completions}"
    );
    run_crate("grid", &modules, &checks);
}

/// FR-031-AC-4, agreement half: over the seven boundary values and two interior values, taken
/// pairwise in both orders, an arithmetic-free comparison oracle agrees with the runtime:
/// `order_numbers` for the four orderings, `check_equality` then `evaluate` for equal and not
/// equal. Held agreement: it passes before and after the change.
#[trace("TC-044", "FR-031-AC-4")]
#[test]
fn tc_044_arithmetic_free_comparisons_agree_with_the_runtime() {
    let values = [
        i64::MIN,
        i64::MIN + 1,
        -1,
        0,
        1,
        i64::MAX - 1,
        i64::MAX,
        -300,
        7,
    ];
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, comparison) in COMPARISONS.into_iter().enumerate() {
        let source = plain_comparison(comparison).source();
        let module = module_of(index, "compare");
        let runtime = match comparison {
            ComparisonOperator::Equal | ComparisonOperator::NotEqual => {
                let verdict = if comparison == ComparisonOperator::Equal {
                    "equal"
                } else {
                    "!equal"
                };
                format!(
                    "{{\n            match rt::planned_equality(&rt::Value::Integer(rt::Integer::from(left)), &rt::Value::Integer(rt::Integer::from(right)), &mut meter()) {{\n                rt::Outcome::Completed(equal) => rt::Outcome::Completed({verdict}),\n                other => other,\n            }}\n        }}"
                )
            }
            _ => {
                let operator = match comparison {
                    ComparisonOperator::Less => "Less",
                    ComparisonOperator::LessEqual => "LessOrEqual",
                    ComparisonOperator::Greater => "Greater",
                    _ => "GreaterOrEqual",
                };
                format!(
                    "rt::order_numbers(rt::OrderingOperator::{operator}, rt::OrderedOperands::Integers(&rt::Integer::from(left), &rt::Integer::from(right)), &mut meter())"
                )
            }
        };
        let oracle = format!(
            "{}::{}(left, right)",
            format_args!("crate::{module}"),
            symbol(&source)
        );
        let list = values
            .iter()
            .map(|value| format!("{value}_i64"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            checks,
            "#[test]\nfn compare_{index}() {{\n    let values: &[i64] = &[{list}];\n    for &left in values {{\n        for &right in values {{\n            let expected = {runtime};\n            assert!(matches!(expected, rt::Outcome::Completed(_)), \"{{expected:?}}\");\n            assert_eq!(rt::Outcome::Completed({oracle}), expected, \"{{left}} vs {{right}}\");\n        }}\n    }}\n}}\n"
        )
        .unwrap();
        modules.push((module, source));
    }
    run_crate("compare", &modules, &checks);
}

// ---------------------------------------------------------------------------------------------
// Propagation (AC-9)
// ---------------------------------------------------------------------------------------------

/// `x < 5 && x * 2 <= 10` over `0..=10`: O-4's guarded multiply, as an operand of a connective.
fn guarded_multiply(clause: &Clause, variable: &str) -> Expression {
    let guard = clause.compare(
        ComparisonOperator::Less,
        clause.variable(variable),
        clause.literal(5),
    );
    let node = clause.numeric(
        NumericOperator::Multiply,
        clause.variable(variable),
        clause.literal(2),
    );
    let relation = clause.compare(ComparisonOperator::LessEqual, node, clause.literal(10));
    clause.and(guard, relation)
}

/// The connectives FR-031-AC-9 names, each with a guarded multiply that stops at `i64::MIN`
/// (`Refused(IntegerOutOfDomain)`, the guard passing and the declared lower bound violated) or
/// at an operand wider than a small meter allows (`Incomplete`).
fn propagation_clauses() -> Vec<(&'static str, Built)> {
    let mut built = Vec::new();
    let mut add = |label: &'static str, shape: &dyn Fn(&Clause) -> Expression| {
        let clause = Clause::new(0, 10, OverflowPolicy::Reject);
        let environment = clause.environment(&["x", "y"], &["flag"]);
        let expression = shape(&clause);
        let node = expression.source().clone();
        built.push((
            label,
            Built {
                environment,
                expression,
                node,
            },
        ));
    };
    add("left_stops", &|clause| {
        clause.and(guarded_multiply(clause, "x"), clause.flag("flag"))
    });
    add("right_unreached", &|clause| {
        clause.and(clause.flag("flag"), guarded_multiply(clause, "x"))
    });
    add("implication", &|clause| {
        clause.connective(
            BooleanOperator::Implication,
            clause.flag("flag"),
            guarded_multiply(clause, "x"),
        )
    });
    add("or", &|clause| {
        clause.connective(
            BooleanOperator::ShortCircuitOr,
            clause.flag("flag"),
            guarded_multiply(clause, "x"),
        )
    });
    add("not", &|clause| {
        Expression::new(
            ExpressionKind::BooleanNot {
                operand: Box::new(guarded_multiply(clause, "x")),
            },
            clause.at(),
        )
    });
    add("total_and", &|clause| {
        clause.connective(
            BooleanOperator::TotalAnd,
            guarded_multiply(clause, "x"),
            guarded_multiply(clause, "y"),
        )
    });
    add("total_or", &|clause| {
        clause.connective(
            BooleanOperator::TotalOr,
            guarded_multiply(clause, "x"),
            guarded_multiply(clause, "y"),
        )
    });
    add("total_with_flag", &|clause| {
        clause.connective(
            BooleanOperator::TotalAnd,
            clause.flag("flag"),
            guarded_multiply(clause, "y"),
        )
    });
    built
}

/// FR-031-AC-9: the first non-completed outcome in evaluation order is the oracle's outcome. A
/// stop in the left operand of a short-circuit connective is returned; a stop in a right operand
/// the left decided is never reached; a total connective returns the first stop of its two
/// operands; and a stop is never `Completed(true)` or `Completed(false)`.
#[trace("TC-044", "FR-031-AC-9")]
#[test]
fn tc_044_the_first_stop_in_evaluation_order_is_the_outcome() {
    const REFUSED: u8 = 2;
    const INCOMPLETE: u8 = 3;
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, (label, built)) in propagation_clauses().into_iter().enumerate() {
        let source = built.source();
        let module = module_of(index, "stop");
        // (flag, x, y, integer-bits limit, expected kind): `x = i64::MIN` passes `x < 5` and
        // leaves the declared `0..=10`, so the multiply is refused; `-100` is a narrow operand.
        let vectors: &[(&str, &str, &str, u64, u8)] = match label {
            "left_stops" => &[
                ("false", "i64::MIN", "0", u64::MAX, REFUSED),
                ("true", "i64::MIN", "0", u64::MAX, REFUSED),
                ("true", "3", "0", u64::MAX, 1),
            ],
            "right_unreached" => &[
                ("false", "i64::MIN", "0", u64::MAX, 0),
                ("true", "i64::MIN", "0", u64::MAX, REFUSED),
                ("true", "3", "0", u64::MAX, 1),
            ],
            "implication" => &[
                ("false", "i64::MIN", "0", u64::MAX, 1),
                ("true", "i64::MIN", "0", u64::MAX, REFUSED),
            ],
            "or" => &[
                ("true", "i64::MIN", "0", u64::MAX, 1),
                ("false", "i64::MIN", "0", u64::MAX, REFUSED),
            ],
            "not" => &[
                ("false", "i64::MIN", "0", u64::MAX, REFUSED),
                ("false", "3", "0", u64::MAX, 0),
                ("false", "7", "0", u64::MAX, 1),
            ],
            // The left stop is a refusal of a narrow operand; the right is too wide for the
            // meter. Swapping them swaps which stop is first.
            "total_and" | "total_or" => &[
                ("false", "-3", "i64::MIN", 10, REFUSED),
                ("false", "i64::MIN", "-3", 10, INCOMPLETE),
                ("false", "3", "i64::MIN", u64::MAX, REFUSED),
            ],
            "total_with_flag" => &[
                ("false", "0", "i64::MIN", u64::MAX, REFUSED),
                ("true", "0", "i64::MIN", u64::MAX, REFUSED),
            ],
            other => panic!("no vectors for {other}"),
        };
        for (row, (flag, x, y, bits, expected)) in vectors.iter().enumerate() {
            let arguments = [
                ("flag_current", *flag),
                ("x_current", *x),
                ("y_current", *y),
            ];
            let arguments = arguments
                .iter()
                .filter(|(name, _)| {
                    parameters(&source)
                        .iter()
                        .any(|parameter| parameter == name)
                })
                .copied()
                .collect::<Vec<_>>();
            let limits = if *bits == u64::MAX {
                "meter()".to_owned()
            } else {
                format!("rt::Meter::new(rt::ScalarLimits {{ integer_bits: {bits}, ..UNLIMITED }})")
            };
            writeln!(
                checks,
                "#[test]\nfn {label}_{row}() {{\n    let mut meter = {limits};\n    let outcome = {};\n    assert_eq!(kind(outcome), {expected}, \"{label} flag={flag} x={x} y={y}\");\n}}\n",
                call(&format!("crate::{module}"), &source, &arguments, "&mut meter")
            )
            .unwrap();
        }
        modules.push((module, source));
    }
    run_crate("stop", &modules, &checks);
}

/// A completed native connective retains its result exactly once, including a
/// left-decided short circuit; an operand stop contributes no outer retention.
///
#[trace("TC-044", "FR-031-AC-26")]
#[test]
fn tc_044_native_connectives_retain_only_completed_results() {
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, (label, built)) in propagation_clauses().into_iter().enumerate() {
        let (flag, x, y, expected_kind, expected_retains) = match label {
            "left_stops" => ("false", "i64::MIN", "0", 2, 0),
            "right_unreached" => ("false", "i64::MIN", "0", 0, 1),
            "implication" => ("false", "i64::MIN", "0", 1, 1),
            "or" => ("true", "i64::MIN", "0", 1, 1),
            "not" => ("false", "3", "0", 0, 2),
            "total_and" | "total_or" => ("false", "3", "3", 1, 3),
            "total_with_flag" => ("false", "0", "3", 0, 2),
            other => panic!("missing retention case for {other}"),
        };
        let source = built.source();
        let module = module_of(index, "retain");
        let arguments = [("flag_current", flag), ("x_current", x), ("y_current", y)];
        let arguments = arguments
            .iter()
            .filter(|(name, _)| {
                parameters(&source)
                    .iter()
                    .any(|parameter| parameter == name)
            })
            .copied()
            .collect::<Vec<_>>();
        writeln!(
            checks,
            "#[test]\nfn retain_{index}() {{\n    let mut meter = meter();\n    let outcome = {};\n    assert_eq!(kind(outcome), {expected_kind});\n    let retains = meter.admitted_charges().iter().filter(|charge| charge.point == rt::ChargePoint::BooleanResultRetain).count();\n    assert_eq!(retains, {expected_retains}, \"{label}\");\n}}\n",
            call(&format!("crate::{module}"), &source, &arguments, "&mut meter")
        )
        .unwrap();
        if label == "right_unreached" {
            writeln!(
                checks,
                "#[test]\nfn retain_denied() {{\n    let mut meter = rt::Meter::new(rt::ScalarLimits {{ result_units: 0, ..UNLIMITED }});\n    let outcome = {};\n    match outcome {{\n        rt::Outcome::Incomplete(incomplete) => assert_eq!(incomplete.charge_point, rt::ChargePoint::BooleanResultRetain),\n        other => panic!(\"expected a denied retain charge: {{other:?}}\"),\n    }}\n    assert_eq!(meter.consumed(rt::LimitKind::ResultUnits), 0);\n    assert!(meter.admitted_charges().is_empty());\n}}\n",
                call(&format!("crate::{module}"), &source, &arguments, "&mut meter")
            )
            .unwrap();
        }
        modules.push((module, source));
    }
    run_crate("retain", &modules, &checks);
}

/// Pure connective subtrees still retain when their enclosing oracle is metered.
/// The arithmetic on the right is syntactically present but never evaluated.
#[trace("TC-044", "FR-031-AC-26", "FR-031-AC-9")]
#[test]
fn tc_044_pure_connective_subtrees_retain_within_a_metered_root() {
    let mut modules = Vec::new();
    let mut checks = String::new();
    for (index, (label, flag, expected_retains)) in [
        ("not", true, 2_u64),
        ("short_and", false, 2),
        ("short_or", false, 2),
        ("total_and", true, 3),
        ("total_or", true, 4),
        ("implication", true, 3),
        ("comparison_leaf", true, 1),
    ]
    .into_iter()
    .enumerate()
    {
        let clause = Clause::new(0, 10, OverflowPolicy::Reject);
        let not = || {
            Expression::new(
                ExpressionKind::BooleanNot {
                    operand: Box::new(clause.flag("flag")),
                },
                clause.at(),
            )
        };
        let left = match label {
            "not" => not(),
            "short_and" => clause.and(clause.flag("flag"), not()),
            "short_or" => clause.connective(
                BooleanOperator::ShortCircuitOr,
                clause.flag("flag"),
                clause.flag("flag"),
            ),
            "total_and" => clause.connective(BooleanOperator::TotalAnd, not(), clause.flag("flag")),
            "total_or" => clause.connective(BooleanOperator::TotalOr, not(), not()),
            "implication" => {
                clause.connective(BooleanOperator::Implication, clause.flag("flag"), not())
            }
            "comparison_leaf" => clause.compare(
                ComparisonOperator::Less,
                clause.variable("x"),
                clause.variable("x"),
            ),
            _ => unreachable!(),
        };
        let arithmetic = clause.numeric(
            NumericOperator::Add,
            clause.variable("x"),
            clause.literal(0),
        );
        let right = clause.compare(ComparisonOperator::Equal, arithmetic, clause.variable("x"));
        let expression = clause.and(left, right);
        let built = Built {
            environment: clause.environment(&["x"], &["flag"]),
            node: expression.source().clone(),
            expression,
        };
        let source = built.source();
        let module = module_of(index, "pure_retain");
        let arguments = [
            ("flag_current", if flag { "true" } else { "false" }),
            ("x_current", "i64::MIN"),
        ];
        let arguments = arguments
            .iter()
            .filter(|(name, _)| {
                parameters(&source)
                    .iter()
                    .any(|parameter| parameter == name)
            })
            .copied()
            .collect::<Vec<_>>();
        let invocation = call(
            &format!("crate::{module}"),
            &source,
            &arguments,
            "&mut meter",
        );
        writeln!(checks, "#[test]\nfn pure_{label}() {{\n    for budget in 0..={expected_retains} {{\n        let mut meter = rt::Meter::new(rt::ScalarLimits {{ result_units: budget, ..UNLIMITED }});\n        let outcome = {invocation};\n        if budget == {expected_retains} {{\n            assert_eq!(outcome, rt::Outcome::Completed(false), \"{label}: all completed connectives retain\");\n        }} else {{\n            match outcome {{\n                rt::Outcome::Incomplete(stop) => assert_eq!(stop.charge_point, rt::ChargePoint::BooleanResultRetain),\n                other => panic!(\"{label}: budget {{budget}} must deny a connective retain: {{other:?}}\"),\n            }}\n        }}\n        assert_eq!(meter.consumed(rt::LimitKind::ResultUnits), budget, \"{label}: no retention after a child stop\");\n        assert_eq!(meter.admitted_charges().len() as u64, budget);\n        assert!(meter.admitted_charges().iter().all(|charge| charge.point == rt::ChargePoint::BooleanResultRetain), \"{label}: unreachable right arithmetic must charge nothing\");\n    }}\n}}\n").unwrap();
        modules.push((module, source));
    }
    run_crate("pure-retain", &modules, &checks);
}

// ---------------------------------------------------------------------------------------------
// Consumers that need a plain bool (AC-10)
// ---------------------------------------------------------------------------------------------

/// A one-clause bound package whose precondition is `amount OP 0 < 7` over `0..=1000` with the
/// given overflow policy, so IR admits it and the oracle's own refusal is the one under test.
fn arithmetic_projection(overflow: &str, operator: &str) -> serde_json::Value {
    let span = |line: u64| {
        let source = serde_json::json!({"document":"arithmetic-package", "revision":1});
        serde_json::json!({"start":{"source":source,"line":line,"column":1,"byte_offset":line - 1},
            "end":{"source":source,"line":line,"column":2,"byte_offset":line}})
    };
    let integer = serde_json::json!({
        "kind":"integer", "domain":"signed", "minimum":"0", "maximum":"1000", "overflow":overflow
    });
    let owner =
        serde_json::json!({"package":"test/arithmetic","requirement":"FR-100","revision":3});
    let anchor = serde_json::json!({"kind":"pre","operation":"checkAmount"});
    let read = serde_json::json!({
        "node":"value_reference", "name":"amount", "observation":"current", "source":span(4)
    });
    let literal = |value: i64, line: u64| serde_json::json!({"node":"integer_literal","value":value.to_string(),"value_type":integer,"source":span(line)});
    serde_json::json!({
        "format": quire_contract_model::EXECUTABLE_PROJECTION_FORMAT,
        "package": {
            "id": "test/arithmetic",
            "schema_version": {"major":1,"minor":1},
            "source": {"document":"arithmetic-package","revision":1},
            "requirements": [{"id":"FR-100","revision":3,"source":span(1),"clauses":[{
                "id":"amount-check", "kind":"precondition", "anchor":anchor, "source":span(2),
                "body":{"node":"reference","identity":{
                    "requirement":owner, "kind":"input", "observation":"current", "path":["amount"]}}
            }]}]
        },
        "bindings": [{
            "clause": {"requirement":owner, "clause":"amount-check"},
            "expression": {
                "owner": owner, "types": [],
                "values": [{"name":"amount","kind":"input","value_type":integer,"source":span(3)}],
                "functions": [],
                "expression": {"node":"compare","operator":"less",
                    "left":{"node":"numeric","operator":operator,"left":read,"right":literal(if operator == "divide" { 1 } else { 0 }, 6),"source":span(4)},
                    "right":literal(7, 5), "source":span(3)},
                "expected_type": {"kind":"boolean"}, "execution_point": anchor, "clause_root": true
            }
        }]
    })
}

fn bound_package(value: &serde_json::Value) -> quire_contract_model::BoundPackage {
    quire_contract_model::BoundPackage::from_json_bytes(&serde_json::to_vec(value).unwrap())
        .unwrap()
}

/// Bound oracle generation is all-or-nothing over a package, so a strategy requested for a healthy
/// clause is refused when a sibling clause of the same package holds a refused node, and the
/// refusal names the sibling (FR-031 Consumers, FR-008-AC-3).
#[trace("TC-044", "FR-031-AC-10", "FR-008-AC-3")]
#[test]
fn tc_044_a_refused_sibling_clause_refuses_the_strategy_of_every_clause_in_its_package() {
    use quire_contract_codegen::{
        generate_bound_strategy, BoundStrategyPopulation, BoundStrategyRequest, StrategyErrorCode,
    };
    let mut value = arithmetic_projection("saturate", "add");
    let refused = quire_contract_model::ClauseRef::new(
        quire_contract_model::RequirementRef::parse("test/arithmetic", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    // A healthy sibling: the same read compared with the same literal, with no arithmetic.
    let mut clause = value["package"]["requirements"][0]["clauses"][0].clone();
    clause["id"] = serde_json::json!("healthy-check");
    value["package"]["requirements"][0]["clauses"]
        .as_array_mut()
        .unwrap()
        .push(clause);
    let mut binding = value["bindings"][0].clone();
    binding["clause"]["clause"] = serde_json::json!("healthy-check");
    let read = binding["expression"]["expression"]["left"]["left"].clone();
    binding["expression"]["expression"]["left"] = read;
    value["bindings"].as_array_mut().unwrap().push(binding);
    let package = bound_package(&value);
    let healthy = quire_contract_model::ClauseRef::new(
        quire_contract_model::RequirementRef::parse("test/arithmetic", "FR-100", 3).unwrap(),
        ClauseId::new("healthy-check").unwrap(),
    );
    let refusal = generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause: &healthy,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
    })
    .expect_err("the refused sibling refuses the whole package");
    assert_eq!(refusal.code, StrategyErrorCode::UnsupportedClause);
    assert_eq!(
        refusal.generation_code,
        Some(GenerationErrorCode::UnsupportedSaturatingArithmetic)
    );
    assert_eq!(refusal.clause.as_deref(), Some(&refused));
}

/// The tri-state harness and bound strategies refuse arithmetic when their Boolean sink
/// cannot carry an outcome.
#[trace("TC-044", "FR-031-AC-10")]
#[test]
fn tc_044_boolean_sinks_refuse_arithmetic() {
    use quire_contract_codegen::{
        generate_bound_strategy, generate_tristate_harness, BoundStrategyPopulation,
        BoundStrategyRequest, HarnessErrorCode, HarnessRequest, StrategyErrorCode,
    };

    // The tri-state harness: UnsupportedExpression at the first arithmetic node for a `reject`
    // add, and the oracle's own code for a `saturate` add and for a divide.
    for (built, expected) in [
        (
            o1_add(ComparisonOperator::LessEqual, 0),
            GenerationErrorCode::UnsupportedExpression,
        ),
        (
            saturate_arithmetic(NumericOperator::Add, i64::MIN, i64::MAX, 1, 0),
            GenerationErrorCode::UnsupportedSaturatingArithmetic,
        ),
        (
            o3_o5(NumericOperator::Divide),
            GenerationErrorCode::UnsupportedExpression,
        ),
    ] {
        let precondition =
            Expression::new(ExpressionKind::BooleanLiteral { value: true }, span(40, 41));
        let precondition = built
            .environment
            .check_expression(&precondition, &ValueType::Boolean, &handler(), true)
            .unwrap();
        let postcondition = built.typed();
        let pre = ClauseId::new("pre").unwrap();
        let post = ClauseId::new("post").unwrap();
        let refusals = generate_tristate_harness(&HarnessRequest {
            requirement: built.environment.owner(),
            precondition_clause: &pre,
            postcondition_clause: &post,
            precondition: &precondition,
            postcondition: &postcondition,
            execution_point: "handler:generate",
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
        })
        .expect_err("the harness refuses arithmetic");
        assert_eq!(refusals.len(), 1);
        assert_eq!(refusals[0].code, HarnessErrorCode::ClauseGenerationFailed);
        assert_eq!(refusals[0].generation_code, Some(expected));
        assert_eq!(
            refusals[0].terminal_state,
            GenerationTerminalState::Unsupported
        );
        assert!(
            refusals[0].path.starts_with("postcondition."),
            "{}",
            refusals[0].path
        );
    }

    let clause = quire_contract_model::ClauseRef::new(
        quire_contract_model::RequirementRef::parse("test/arithmetic", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );

    // The bound strategies: a clause whose oracle generates, a `reject` add used as a comparison
    // operand, keeps UnsupportedRelation; a clause the oracle refuses is UnsupportedClause
    // carrying the oracle's code and span.
    let strategy = |package: &quire_contract_model::BoundPackage| {
        generate_bound_strategy(&BoundStrategyRequest {
            package,
            clause: &clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
        })
        .expect_err("the strategy refuses arithmetic")
    };
    let reject = bound_package(&arithmetic_projection("reject", "add"));
    let refusal = strategy(&reject);
    assert_eq!(refusal.code, StrategyErrorCode::UnsupportedRelation);
    assert_eq!(refusal.generation_code, None);
    let saturate = bound_package(&arithmetic_projection("saturate", "add"));
    let refusal = strategy(&saturate);
    assert_eq!(refusal.code, StrategyErrorCode::UnsupportedClause);
    assert_eq!(
        refusal.generation_code,
        Some(GenerationErrorCode::UnsupportedSaturatingArithmetic)
    );
    assert_eq!(refusal.terminal_state, GenerationTerminalState::Unsupported);
    assert_eq!(refusal.source_span.as_ref().unwrap().start().line(), 4);
    let division = bound_package(&arithmetic_projection("saturate", "divide"));
    let refusal = strategy(&division);
    assert_eq!(refusal.code, StrategyErrorCode::UnsupportedClause);
    assert_eq!(
        refusal.generation_code,
        Some(GenerationErrorCode::UnsupportedSaturatingArithmetic)
    );
}
