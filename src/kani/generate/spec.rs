//! Validated, private harness descriptions (FR-015).

use super::outcome::{UnsupportedObligation, MAX_OBLIGATION_UNWIND};
use crate::{
    core::identity::HarnessPath,
    kani::abi::{adapter_options, KaniPrimitiveType, KaniSolver},
};
use std::collections::BTreeSet;
use syn::{
    ext::IdentExt,
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
            forbidden: text.contains("#["),
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
        if path.segments.pairs().any(|pair| {
            let segment = pair.value();
            identifier_identity(&segment.ident) == "kani"
                && (pair.punct().is_some()
                    || matches!(&segment.arguments, syn::PathArguments::AngleBracketed(arguments)
                        if arguments.colon2_token.is_some()))
        }) {
            self.forbidden = true;
        }
        visit::visit_path(self, path);
    }
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) {
        self.forbidden = true;
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if !mac
            .path
            .get_ident()
            .is_some_and(|name| identifier_identity(name) == "matches")
        {
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
    ScalarMeter(quire_contract_runtime::exact::ScalarLimits),
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
    Value(String),
    /// A fixed lazy Boolean outcome passed to the generated oracle.
    BooleanOutcomeClosure(String),
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
    /// Fixed compiled runtime subject for the V2 Boolean implication control.
    SubjectImplicationCall,
    /// Customer subject for one V2 clause parameter, returning its Boolean condition.
    ClauseSubjectCall {
        callee: String,
        argument: String,
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
fn identifier_identity(identifier: &syn::Ident) -> String {
    identifier.unraw().to_string()
}
fn identifier(name: &str) -> Result<String, HarnessSpecError> {
    syn::parse_str::<syn::Ident>(name)
        .map(|identifier| identifier_identity(&identifier))
        .map_err(|_| HarnessSpecError::InvalidIdentifier(name.to_owned()))
}
fn message(text: &str) -> Result<(), HarnessSpecError> {
    if text.contains(['"', '\\', '{', '}', '\n', '\r']) {
        Err(HarnessSpecError::InvalidMessage)
    } else {
        Ok(())
    }
}
fn bind(names: &mut BTreeSet<String>, name: &str) -> Result<String, HarnessSpecError> {
    let identity = identifier(name)?;
    if !names.insert(identity.clone()) {
        return Err(HarnessSpecError::DuplicateBinding(name.to_owned()));
    }
    Ok(identity)
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
                    let input_identity = bind(&mut names, name)?;
                    let mut shadowed = false;
                    for binding in then {
                        if identifier(&binding.name)? == input_identity && !shadowed {
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
                Statement::Let(binding) => {
                    bind(&mut names, &binding.name)?;
                }
                Statement::Call {
                    result,
                    callee,
                    args,
                } => {
                    bind(&mut names, result)?;
                    identifier(callee)?;
                    for argument in args {
                        let name = match argument {
                            Argument::Value(name)
                            | Argument::BooleanOutcomeClosure(name)
                            | Argument::Borrow(name)
                            | Argument::BorrowMut(name) => name,
                        };
                        identifier(name)?;
                    }
                    called = true;
                }
                Statement::SubjectImplicationCall if !called => {
                    return Err(HarnessSpecError::AssertBeforeCall);
                }
                Statement::SubjectImplicationCall => {}
                Statement::ClauseSubjectCall { callee, argument } => {
                    if !called {
                        return Err(HarnessSpecError::AssertBeforeCall);
                    }
                    syn::parse_str::<syn::Path>(callee)
                        .map_err(|_| HarnessSpecError::InvalidExpression)?;
                    identifier(argument)?;
                    bind(&mut names, "subject")?;
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

    #[trace("FR-015-AC-112", "TC-025")]
    #[test]
    fn expression_namespace_checks_use_identifier_identity_and_path_position() {
        for text in [
            "kani",
            "r#kani",
            "rt::kani",
            "rt::r#kani",
            "rt /* gap */ :: r#kani",
            "not_kani::any::<i64>()",
            "Kani::value",
            "matches!(r#kani, true)",
            "matches!(value, rt::r#kani)",
            "r#matches!(rt::r#kani, _)",
        ] {
            assert_eq!(
                ExprText::new(text).map(|expression| expression.as_str().to_owned()),
                Ok(text.to_owned()),
                "legal expression {text}"
            );
        }
        for text in [
            "kani :: any::<i64>()",
            "kani /* gap */ :: assume(false)",
            "r#kani :: any::<i64>()",
            "r#kani /* gap */ :: assume(false)",
            ":: r#kani :: any::<bool>()",
            "rt :: r#kani :: value",
            "kani :: <i64>()",
            "r#kani /* gap */ :: <i64>()",
            "rt :: r#kani :: <i64>()",
            "matches!(r#kani :: any::<bool>(), true)",
            "matches!(matches!(r#kani /* gap */ :: any::<bool>(), true), true)",
            "r#matches!(r#kani :: any::<bool>(), true)",
            "matches!(value, r#kani /* gap */ :: Some(_))",
        ] {
            assert_eq!(
                ExprText::new(text),
                Err(HarnessSpecError::ForbiddenExpression),
                "Kani namespace or turbofish spelling {text}"
            );
        }
    }

    #[trace("FR-015-AC-112", "TC-025")]
    #[test]
    fn raw_identifiers_cannot_evade_duplicate_binding_checks() {
        for (first, second) in [("x", "r#x"), ("r#x", "x")] {
            for body in [
                vec![input(first), input(second), call()],
                vec![input(first), Statement::Let(binding(second)), call()],
                vec![
                    Statement::Let(binding(first)),
                    Statement::Let(binding(second)),
                    call(),
                ],
                vec![
                    input(first),
                    Statement::Call {
                        result: second.into(),
                        callee: "probe".into(),
                        args: vec![],
                    },
                ],
                vec![
                    Statement::Let(binding(first)),
                    Statement::Call {
                        result: second.into(),
                        callee: "probe".into(),
                        args: vec![],
                    },
                ],
                vec![
                    Statement::Input {
                        name: first.into(),
                        ty: KaniPrimitiveType::I64,
                        domain: (-1, 1),
                        then: vec![binding(first), binding(second)],
                    },
                    call(),
                ],
                vec![
                    Statement::Input {
                        name: "other".into(),
                        ty: KaniPrimitiveType::I64,
                        domain: (-1, 1),
                        then: vec![binding(first), binding(second)],
                    },
                    call(),
                ],
            ] {
                assert_eq!(
                    error(body, vec![cover()], 1),
                    HarnessSpecError::DuplicateBinding(second.into())
                );
            }
        }
        for (result, later) in [("answer", "r#answer"), ("r#answer", "answer")] {
            assert_eq!(
                error(
                    vec![
                        Statement::Call {
                            result: result.into(),
                            callee: "probe".into(),
                            args: vec![]
                        },
                        Statement::Let(binding(later))
                    ],
                    vec![cover()],
                    1
                ),
                HarnessSpecError::DuplicateBinding(later.into())
            );
        }
    }

    #[trace("FR-015-AC-112", "FR-015-AC-116", "TC-025")]
    #[test]
    fn own_input_shadow_accepts_raw_alias_once_and_preserves_source_spelling() {
        for (name, shadow) in [("x", "r#x"), ("r#x", "x"), ("r#x", "r#x")] {
            let spec = construct(
                vec![
                    Statement::Input {
                        name: name.into(),
                        ty: KaniPrimitiveType::I64,
                        domain: (-1, 1),
                        then: vec![binding("native"), binding(shadow)],
                    },
                    Statement::Let(binding("r#distinct")),
                    Statement::Call {
                        result: "r#answer".into(),
                        callee: "r#probe".into(),
                        args: vec![Argument::Borrow("r#x".into())],
                    },
                ],
                vec![cover()],
                1,
            )
            .unwrap();
            let source = super::super::render::render(&spec).unwrap().contents;
            assert!(source.contains(&format!("let {name}: i64 =")));
            assert!(source.contains(&format!("let {shadow} = x;")));
            assert!(source.contains("let r#answer = r#probe(&r#x);"));
            assert_eq!(
                error(
                    vec![
                        Statement::Input {
                            name: name.into(),
                            ty: KaniPrimitiveType::I64,
                            domain: (-1, 1),
                            then: vec![binding(shadow), binding(name)]
                        },
                        call()
                    ],
                    vec![cover()],
                    1
                ),
                HarnessSpecError::DuplicateBinding(name.into())
            );
        }
    }
}
