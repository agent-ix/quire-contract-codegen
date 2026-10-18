//! Validated, private harness descriptions (FR-015).

use super::outcome::{UnsupportedObligation, MAX_OBLIGATION_UNWIND};
use crate::{
    core::identity::HarnessPath,
    kani::abi::{adapter_options, KaniPrimitiveType, KaniSolver},
};
use std::collections::BTreeSet;
use syn::{
    parse::{Parse, ParseStream},
    visit::{self, Visit},
    Expr, Pat, Token,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum HarnessSpecError {
    CoverCount(usize),
    LiteralTrueCover,
    CallCount(usize),
    InputAfterCall,
    AssumeAfterCall,
    AssertBeforeCall,
    DuplicateBinding(String),
    InvalidIdentifier(String),
    InvalidDomain,
    InvalidUnwind(u32),
    InvalidMessage,
    InvalidExpression,
    ForbiddenExpression,
}
impl std::fmt::Display for HarnessSpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for HarnessSpecError {}
impl From<HarnessSpecError> for UnsupportedObligation {
    fn from(error: HarnessSpecError) -> Self {
        Self::InvalidGeneratedSyntax {
            error: error.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExprText(String);
impl ExprText {
    pub(super) fn new(text: impl Into<String>) -> Result<Self, HarnessSpecError> {
        let text = text.into();
        let expression =
            syn::parse_str::<Expr>(&text).map_err(|_| HarnessSpecError::InvalidExpression)?;
        let mut check = ExpressionCheck {
            forbidden: text.contains("kani::") || text.contains("#["),
        };
        check.visit_expr(&expression);
        if check.forbidden {
            return Err(HarnessSpecError::ForbiddenExpression);
        }
        Ok(Self(text))
    }
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}
struct Matches {
    expression: Expr,
    pattern: Pat,
}
impl Parse for Matches {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let expression = input.parse()?;
        input.parse::<Token![,]>()?;
        let pattern = Pat::parse_multi_with_leading_vert(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        if !input.is_empty() {
            return Err(input.error("unguarded pattern required"));
        }
        Ok(Self {
            expression,
            pattern,
        })
    }
}
struct ExpressionCheck {
    forbidden: bool,
}
impl<'ast> Visit<'ast> for ExpressionCheck {
    fn visit_expr(&mut self, expression: &'ast Expr) {
        if matches!(
            expression,
            Expr::Block(_)
                | Expr::Unsafe(_)
                | Expr::Async(_)
                | Expr::Closure(_)
                | Expr::Loop(_)
                | Expr::While(_)
                | Expr::ForLoop(_)
                | Expr::Let(_)
                | Expr::Const(_)
                | Expr::TryBlock(_)
        ) {
            self.forbidden = true;
        }
        visit::visit_expr(self, expression);
    }
    fn visit_block(&mut self, _: &'ast syn::Block) {
        self.forbidden = true;
    }
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path.segments.iter().any(|segment| segment.ident == "kani") {
            self.forbidden = true;
        }
        visit::visit_path(self, path);
    }
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) {
        self.forbidden = true;
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if !mac.path.is_ident("matches") {
            self.forbidden = true;
            return;
        }
        match syn::parse2::<Matches>(mac.tokens.clone()) {
            Ok(arguments) => {
                let mut nested = ExpressionCheck { forbidden: false };
                nested.visit_expr(&arguments.expression);
                nested.visit_pat(&arguments.pattern);
                self.forbidden |= nested.forbidden;
            }
            Err(_) => self.forbidden = true,
        }
    }
    fn visit_pat(&mut self, pattern: &'ast Pat) {
        if matches!(pattern, Pat::Macro(_)) {
            self.forbidden = true;
            return;
        }
        visit::visit_pat(self, pattern);
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ProofKind {
    Proof,
}
#[derive(Clone, Copy, Debug)]
pub(super) enum LetType {
    I128,
}
#[derive(Clone, Debug)]
pub(super) enum Init {
    Expression(ExprText),
    UnlimitedMeter,
}
#[derive(Clone, Debug)]
pub(super) struct LetBinding {
    pub(super) name: String,
    pub(super) ty: Option<LetType>,
    pub(super) mutable: bool,
    pub(super) init: Init,
}
#[derive(Clone, Debug)]
pub(super) enum Argument {
    Borrow(String),
    BorrowMut(String),
}
#[derive(Clone, Debug)]
pub(super) enum Statement {
    Input {
        name: String,
        ty: KaniPrimitiveType,
        domain: (i128, i128),
        then: Vec<LetBinding>,
    },
    // Only the non-vacuity backend control needs an extra assumption at this stage.
    #[cfg_attr(not(test), allow(dead_code))]
    Assume {
        condition: ExprText,
    },
    Let(LetBinding),
    Call {
        result: String,
        callee: String,
        args: Vec<Argument>,
    },
    Assert {
        condition: ExprText,
        message: String,
    },
}
#[derive(Clone, Debug)]
pub(super) struct Cover {
    pub(super) condition: ExprText,
    pub(super) message: String,
}

#[derive(Clone, Debug)]
pub(super) struct HarnessSpec {
    label: String,
    path: HarnessPath,
    oracle_source: String,
    proof: ProofKind,
    body: Vec<Statement>,
    cover: Cover,
    unwind: u32,
    solver: KaniSolver,
    options: Vec<String>,
}
fn identifier(name: &str) -> Result<(), HarnessSpecError> {
    syn::parse_str::<syn::Ident>(name)
        .map(|_| ())
        .map_err(|_| HarnessSpecError::InvalidIdentifier(name.to_owned()))
}
fn message(text: &str) -> Result<(), HarnessSpecError> {
    if text.contains(['"', '\\', '{', '}', '\n', '\r']) {
        Err(HarnessSpecError::InvalidMessage)
    } else {
        Ok(())
    }
}
fn bind(names: &mut BTreeSet<String>, name: &str) -> Result<(), HarnessSpecError> {
    identifier(name)?;
    if !names.insert(name.to_owned()) {
        return Err(HarnessSpecError::DuplicateBinding(name.to_owned()));
    }
    Ok(())
}
fn literal_true(expression: &Expr) -> bool {
    let mut expression = expression;
    loop {
        expression = match expression {
            Expr::Paren(parenthesized) => &parenthesized.expr,
            Expr::Group(group) => &group.expr,
            Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Bool(value),
                ..
            }) => return value.value,
            _ => return false,
        };
    }
}
impl HarnessSpec {
    pub(super) fn new(
        label: String,
        path: HarnessPath,
        oracle_source: String,
        proof: ProofKind,
        body: Vec<Statement>,
        covers: Vec<Cover>,
        unwind: u32,
        solver: KaniSolver,
    ) -> Result<Self, HarnessSpecError> {
        if covers.len() != 1 {
            return Err(HarnessSpecError::CoverCount(covers.len()));
        }
        let mut covers = covers.into_iter();
        let cover = covers.next().ok_or(HarnessSpecError::CoverCount(0))?;
        message(&label)?;
        message(&cover.message)?;
        if syn::parse_str::<Expr>(cover.condition.as_str())
            .is_ok_and(|expression| literal_true(&expression))
        {
            return Err(HarnessSpecError::LiteralTrueCover);
        }
        if !(1..=MAX_OBLIGATION_UNWIND).contains(&unwind) {
            return Err(HarnessSpecError::InvalidUnwind(unwind));
        }
        let calls = body
            .iter()
            .filter(|s| matches!(s, Statement::Call { .. }))
            .count();
        if calls != 1 {
            return Err(HarnessSpecError::CallCount(calls));
        }
        let mut called = false;
        let mut names = BTreeSet::new();
        for statement in &body {
            match statement {
                Statement::Input {
                    name,
                    ty,
                    domain: (minimum, maximum),
                    then,
                } => {
                    if called {
                        return Err(HarnessSpecError::InputAfterCall);
                    }
                    let (low, high) = match ty {
                        KaniPrimitiveType::I64 => (i128::from(i64::MIN), i128::from(i64::MAX)),
                        KaniPrimitiveType::Boolean => (0, 1),
                    };
                    if minimum > maximum || *minimum < low || *maximum > high {
                        return Err(HarnessSpecError::InvalidDomain);
                    }
                    bind(&mut names, name)?;
                    let mut shadowed = false;
                    for binding in then {
                        if binding.name == *name && !shadowed {
                            shadowed = true;
                        } else {
                            bind(&mut names, &binding.name)?;
                        }
                    }
                }
                Statement::Assume { .. } if called => {
                    return Err(HarnessSpecError::AssumeAfterCall)
                }
                Statement::Assume { .. } => {}
                Statement::Let(binding) => bind(&mut names, &binding.name)?,
                Statement::Call {
                    result,
                    callee,
                    args,
                } => {
                    bind(&mut names, result)?;
                    identifier(callee)?;
                    for argument in args {
                        let (Argument::Borrow(name) | Argument::BorrowMut(name)) = argument;
                        identifier(name)?;
                    }
                    called = true;
                }
                Statement::Assert { message: text, .. } => {
                    if !called {
                        return Err(HarnessSpecError::AssertBeforeCall);
                    }
                    message(text)?;
                }
            }
        }
        let options = adapter_options(&path.to_string(), unwind, solver, false);
        Ok(Self {
            label,
            path,
            oracle_source,
            proof,
            body,
            cover,
            unwind,
            solver,
            options,
        })
    }
    pub(super) fn label(&self) -> &str {
        &self.label
    }
    pub(super) fn path(&self) -> &HarnessPath {
        &self.path
    }
    pub(super) fn oracle_source(&self) -> &str {
        &self.oracle_source
    }
    pub(super) fn proof(&self) -> ProofKind {
        self.proof
    }
    pub(super) fn body(&self) -> &[Statement] {
        &self.body
    }
    pub(super) fn cover(&self) -> &Cover {
        &self.cover
    }
    pub(super) fn unwind(&self) -> u32 {
        self.unwind
    }
    pub(super) fn solver(&self) -> KaniSolver {
        self.solver
    }
    pub(super) fn options(&self) -> &[String] {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::identity::{HarnessSymbol, ModuleSymbol};
    use ix_trace_rs::trace;

    fn expression(text: &str) -> ExprText {
        ExprText::new(text).unwrap()
    }
    fn binding(name: &str) -> LetBinding {
        LetBinding {
            name: name.into(),
            ty: None,
            mutable: false,
            init: Init::Expression(expression("x")),
        }
    }
    fn input(name: &str) -> Statement {
        Statement::Input {
            name: name.into(),
            ty: KaniPrimitiveType::I64,
            domain: (-1, 1),
            then: vec![],
        }
    }
    fn call() -> Statement {
        Statement::Call {
            result: "answer".into(),
            callee: "probe".into(),
            args: vec![Argument::Borrow("x".into())],
        }
    }
    fn cover() -> Cover {
        Cover {
            condition: expression("answer"),
            message: "reachable".into(),
        }
    }
    fn construct(
        body: Vec<Statement>,
        covers: Vec<Cover>,
        unwind: u32,
    ) -> Result<HarnessSpec, HarnessSpecError> {
        HarnessSpec::new(
            "control".into(),
            HarnessPath {
                module: ModuleSymbol::try_from("control").unwrap(),
                harness: HarnessSymbol::try_from("check").unwrap(),
            },
            "fn probe(_: &i64) -> bool { true }".into(),
            ProofKind::Proof,
            body,
            covers,
            unwind,
            KaniSolver::Cadical,
        )
    }
    fn error(body: Vec<Statement>, covers: Vec<Cover>, unwind: u32) -> HarnessSpecError {
        construct(body, covers, unwind).unwrap_err()
    }

    #[trace("FR-015-AC-112", "TC-025")]
    #[test]
    fn constructor_refuses_cover_call_and_statement_order_violations() {
        assert_eq!(
            error(vec![call()], vec![], 1),
            HarnessSpecError::CoverCount(0)
        );
        assert_eq!(
            error(vec![call()], vec![cover(), cover()], 1),
            HarnessSpecError::CoverCount(2)
        );
        for text in ["true", " true ", "(true)", "((true))"] {
            assert_eq!(
                error(
                    vec![call()],
                    vec![Cover {
                        condition: expression(text),
                        ..cover()
                    }],
                    1
                ),
                HarnessSpecError::LiteralTrueCover
            );
        }
        assert_eq!(
            error(vec![], vec![cover()], 1),
            HarnessSpecError::CallCount(0)
        );
        assert_eq!(
            error(vec![call(), call()], vec![cover()], 1),
            HarnessSpecError::CallCount(2)
        );
        assert_eq!(
            error(vec![call(), input("x")], vec![cover()], 1),
            HarnessSpecError::InputAfterCall
        );
        assert_eq!(
            error(
                vec![
                    call(),
                    Statement::Assume {
                        condition: expression("false")
                    }
                ],
                vec![cover()],
                1
            ),
            HarnessSpecError::AssumeAfterCall
        );
        assert_eq!(
            error(
                vec![
                    Statement::Assert {
                        condition: expression("true"),
                        message: "check".into()
                    },
                    call()
                ],
                vec![cover()],
                1
            ),
            HarnessSpecError::AssertBeforeCall
        );
    }

    #[trace("FR-015-AC-112", "TC-025")]
    #[test]
    fn constructor_checks_names_domains_messages_and_unwind_boundaries() {
        for unwind in [0, MAX_OBLIGATION_UNWIND + 1] {
            assert_eq!(
                error(vec![call()], vec![cover()], unwind),
                HarnessSpecError::InvalidUnwind(unwind)
            );
        }
        for unwind in [1, MAX_OBLIGATION_UNWIND] {
            let spec = construct(
                vec![
                    Statement::Input {
                        name: "x".into(),
                        ty: KaniPrimitiveType::I64,
                        domain: (i128::from(i64::MIN), i128::from(i64::MAX)),
                        then: vec![binding("native"), binding("x")],
                    },
                    call(),
                ],
                vec![cover()],
                unwind,
            )
            .unwrap();
            assert_eq!(
                spec.options(),
                adapter_options("control::check", unwind, KaniSolver::Cadical, false)
            );
        }
        for body in [
            vec![input("x"), input("x"), call()],
            vec![input("x"), Statement::Let(binding("x")), call()],
            vec![input("answer"), call()],
            vec![
                Statement::Input {
                    name: "x".into(),
                    ty: KaniPrimitiveType::I64,
                    domain: (0, 1),
                    then: vec![binding("x"), binding("x")],
                },
                call(),
            ],
        ] {
            assert!(matches!(
                error(body, vec![cover()], 1),
                HarnessSpecError::DuplicateBinding(_)
            ));
        }
        for name in ["", "x-y", "fn", "a::b", "_"] {
            for body in [
                vec![input(name), call()],
                vec![Statement::Let(binding(name)), call()],
                vec![
                    Statement::Input {
                        name: "x".into(),
                        ty: KaniPrimitiveType::I64,
                        domain: (0, 1),
                        then: vec![binding(name)],
                    },
                    call(),
                ],
                vec![Statement::Call {
                    result: name.into(),
                    callee: "probe".into(),
                    args: vec![],
                }],
                vec![Statement::Call {
                    result: "answer".into(),
                    callee: name.into(),
                    args: vec![],
                }],
            ] {
                assert_eq!(
                    error(body, vec![cover()], 1),
                    HarnessSpecError::InvalidIdentifier(name.into())
                );
            }
        }
        for (ty, domain) in [
            (KaniPrimitiveType::I64, (1, 0)),
            (KaniPrimitiveType::I64, (i128::from(i64::MIN) - 1, 0)),
            (KaniPrimitiveType::I64, (0, i128::from(i64::MAX) + 1)),
            (KaniPrimitiveType::Boolean, (-1, 1)),
            (KaniPrimitiveType::Boolean, (0, 2)),
        ] {
            assert_eq!(
                error(
                    vec![
                        Statement::Input {
                            name: "x".into(),
                            ty,
                            domain,
                            then: vec![]
                        },
                        call()
                    ],
                    vec![cover()],
                    1
                ),
                HarnessSpecError::InvalidDomain
            );
        }
        for forbidden in ['"', '\\', '{', '}', '\n', '\r'] {
            let text = format!("bad{forbidden}message");
            assert_eq!(
                error(
                    vec![call()],
                    vec![Cover {
                        message: text.clone(),
                        ..cover()
                    }],
                    1
                ),
                HarnessSpecError::InvalidMessage
            );
            assert_eq!(
                error(
                    vec![
                        call(),
                        Statement::Assert {
                            condition: expression("answer"),
                            message: text.clone()
                        }
                    ],
                    vec![cover()],
                    1
                ),
                HarnessSpecError::InvalidMessage
            );
            let path = construct(vec![call()], vec![cover()], 1)
                .unwrap()
                .path()
                .clone();
            assert_eq!(
                HarnessSpec::new(
                    text,
                    path,
                    String::new(),
                    ProofKind::Proof,
                    vec![call()],
                    vec![cover()],
                    1,
                    KaniSolver::Cadical
                )
                .unwrap_err(),
                HarnessSpecError::InvalidMessage
            );
        }
        construct(
            vec![
                Statement::Input {
                    name: "x".into(),
                    ty: KaniPrimitiveType::Boolean,
                    domain: (0, 1),
                    then: vec![],
                },
                call(),
            ],
            vec![cover()],
            1,
        )
        .unwrap();
    }

    #[trace("FR-015-AC-112", "TC-025")]
    #[test]
    fn expression_validation_descends_into_matches_arguments_and_patterns() {
        assert_eq!(
            ExprText::new("x +"),
            Err(HarnessSpecError::InvalidExpression)
        );
        for text in [
            "kani::any()",
            "kani :: any()",
            "#[allow(unused)] x",
            "{ x }",
            "unsafe { x }",
            "async { x }",
            "|x| x",
            "loop {}",
            "while false {}",
            "for x in y {}",
            "let x = y",
            "panic!()",
            "assert!(x)",
            "matches!(x, Some(y) if y > 0)",
            "matches!(panic!(), _)",
            "matches!({ x }, _)",
            "matches!(|x| x, _)",
            "matches!(x, m!())",
            "matches!(x, Some(m!()))",
            "matches!(matches!({ x }, _), true)",
            "match x { _ => { true } }",
            "if x { true } else { false }",
            "matches!(if x { true } else { false }, true)",
        ] {
            assert!(ExprText::new(text).is_err(), "must refuse {text}");
        }
        for text in [
            "match x { Some(value) => value > 0, _ => false }",
            "matches!(x, Some(_))",
            "matches!(x, A | B,)",
            "matches!(matches!(x, A), true)",
            "i128::from(x) + 1",
        ] {
            assert_eq!(expression(text).as_str(), text);
        }
    }
}
