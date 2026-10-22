//! Public source-to-clause agreement against canonical-parse owner operands.
//!
//! Source minus remains an arithmetic node: in particular, i64::MIN is a
//! negate of the positive, out-of-i64 child, never a negative literal leaf.
//! QSL arithmetic has mathematical Integer results. Bounded-result refusals
//! and reader-admitted literal variants belong to the separate boundary controls.

use std::{fmt::Write as _, fs, process::Command};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_v2_clause_oracle, EXACT_DEPENDENCY_SOURCE, SEMANTIC_VALUE_DEPENDENCY_SOURCE,
};

use crate::{kani_obligations_state_frame::native_twin::Twin, scratch_crate};

struct Case {
    source: String,
    // Authored independently of the generated Rust and its literal constructor.
    reference: String,
    expected: &'static str,
    skipped_arithmetic: bool,
}

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (source, owner, expected) in [
        ("=", "eq", "x == 5"),
        ("!=", "ne", "x != 5"),
        ("<", "lt", "x < 5"),
        ("<=", "le", "x <= 5"),
        (">", "gt", "x > 5"),
        (">=", "ge", "x >= 5"),
    ] {
        // Equality against a bounded field narrows the inline literal upstream;
        // use a genuine mathematical-Integer result instead. Its Add charges
        // must also occur in the independently authored reference tree.
        let (source, reference) = if matches!(owner, "eq" | "ne") {
            (
                format!("(self.balance + 0) {source} 5"),
                format!("c(\"{owner}\", a(\"add\", input(), lit(\"0\")), lit(\"5\"))"),
            )
        } else {
            (
                format!("self.balance {source} 5"),
                format!("c(\"{owner}\", input(), lit(\"5\"))"),
            )
        };
        cases.push(Case {
            source,
            reference,
            expected,
            skipped_arithmetic: false,
        });
    }
    for (source, reference, expected) in [
        ("(self.balance + 5) > 5", "c(\"gt\", a(\"add\", input(), lit(\"5\")), lit(\"5\"))", "x > 0"),
        ("(self.balance - 5) < 0", "c(\"lt\", a(\"sub\", input(), lit(\"5\")), lit(\"0\"))", "x < 5"),
        ("(self.balance * 5) >= 25", "c(\"ge\", a(\"mul\", input(), lit(\"5\")), lit(\"25\"))", "x >= 5"),
        ("(-self.balance) < 0", "c(\"lt\", neg(input()), lit(\"0\"))", "x > 0"),
        ("((self.balance + 5) * (self.balance - 5)) < 0", "c(\"lt\", a(\"mul\", a(\"add\", input(), lit(\"5\")), a(\"sub\", input(), lit(\"5\"))), lit(\"0\"))", "x < 5"),
        ("(170141183460469231731687303715884105727 + self.balance) > 170141183460469231731687303715884105727", "c(\"gt\", a(\"add\", lit(\"170141183460469231731687303715884105727\"), input()), lit(\"170141183460469231731687303715884105727\"))", "x > 0"),
    ] {
        cases.push(Case { source: source.into(), reference: reference.into(), expected, skipped_arithmetic: false });
    }
    // The endpoint neighbours are source expressions, with explicit negation
    // on the negative side. Positive 2^127 is outside this QSL fixture's intake.
    for (spelling, negative) in [
        ("0", false),
        ("1", false),
        ("1", true),
        ("9223372036854775806", false),
        ("9223372036854775807", false),
        ("9223372036854775808", false),
        ("9223372036854775807", true),
        ("9223372036854775808", true),
        ("9223372036854775809", true),
        ("170141183460469231731687303715884105727", false),
    ] {
        let (source, reference, expected) = if negative {
            (
                format!("(-{spelling}) < self.balance"),
                format!("c(\"lt\", neg(lit({spelling:?})), input())"),
                "true",
            )
        } else {
            let expected = match spelling {
                "0" => "x > 0",
                "1" => "x > 1",
                _ => "false",
            };
            (
                format!("{spelling} < self.balance"),
                format!("c(\"lt\", lit({spelling:?}), input())"),
                expected,
            )
        };
        cases.push(Case {
            source,
            reference,
            expected,
            skipped_arithmetic: false,
        });
    }
    for (source, reference, expected, skipped_arithmetic) in [
        (
            "(self.balance < 0) and ((self.balance + 5) > 0)",
            "b(\"and\", c(\"lt\", input(), lit(\"0\")), c(\"gt\", a(\"add\", input(), lit(\"5\")), lit(\"0\")))",
            "false",
            true,
        ),
        (
            "(self.balance >= 0) or ((self.balance * 5) > 0)",
            "b(\"or\", c(\"ge\", input(), lit(\"0\")), c(\"gt\", a(\"mul\", input(), lit(\"5\")), lit(\"0\")))",
            "true",
            true,
        ),
        (
            "(self.balance >= 0) and ((self.balance + 5) > 5)",
            "b(\"and\", c(\"ge\", input(), lit(\"0\")), c(\"gt\", a(\"add\", input(), lit(\"5\")), lit(\"5\")))",
            "x > 0",
            false,
        ),
        (
            "(self.balance < 0) or ((-self.balance) < 0)",
            "b(\"or\", c(\"lt\", input(), lit(\"0\")), c(\"lt\", neg(input()), lit(\"0\")))",
            "x > 0",
            false,
        ),
    ] {
        cases.push(Case {
            source: source.into(),
            reference: reference.into(),
            expected,
            skipped_arithmetic,
        });
    }
    cases
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_v2_integer_literals_preserve_owner_semantics_and_every_denial_prefix() {
    let scratch = scratch_crate::TemporaryDirectory::new("v2-clause-integer-literal-agreement");
    let mut source = String::from(NATIVE_SUPPORT);
    let cases = cases();
    for (index, case) in cases.iter().enumerate() {
        let (package, node) =
            Twin::with_balance_condition(&case.source).emitted_package("BalanceNeverDrops");
        let oracle = generate_v2_clause_oracle(&package, &node)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.source));
        let mut arguments = String::new();
        for input in &oracle.descriptor.inputs {
            assert_eq!(input.name, "current_balance", "{}", case.source);
            assert_eq!((input.lower, input.upper), (0, 1000));
            arguments.push_str("&input, ");
        }
        writeln!(source, "mod generated_{index} {{\n{}\n}}", oracle.source).unwrap();
        // Map every public typed stop without Debug/string erasure. A newly
        // introduced variant must force this agreement consumer to be updated.
        writeln!(source, r#"
#[ix_trace_rs::trace("TC-024", "FR-014-AC-53")]
#[test]
fn case_{index}() {{
    let expression = {reference};
    exercise({label:?}, &expression, |x| {{ let _ = x; {expected} }}, {skipped}, |x, meter| {{
        let input: rt::Integer = x.to_string().parse().expect("canonical input");
        match generated_{index}::oracle_clause({arguments}meter) {{
            Ok(outcome) | Err(generated_{index}::ClauseStop::Runtime(outcome)) => Observation::Runtime(outcome),
            Err(generated_{index}::ClauseStop::InvalidConstant) => Observation::InvalidConstant,
            Err(generated_{index}::ClauseStop::Oracle(stop)) => match stop {{
                generated_{index}::OracleStop::IllTyped(cause) => Observation::IllTyped(cause),
                generated_{index}::OracleStop::InvalidConstant => Observation::InvalidConstant,
                generated_{index}::OracleStop::OperandWidth => Observation::OperandWidth,
            }}
        }}
    }});
}}
"#, reference = case.reference, label = case.source, expected = case.expected, skipped = case.skipped_arithmetic).unwrap();
    }
    let manifest = format!(
        "[package]\nname = \"v2-literal-native-agreement\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}\nquire-semantic-value = {{ {SEMANTIC_VALUE_DEPENDENCY_SOURCE} }}\n{}\nix-trace-rs = {{ git = \"https://github.com/agent-ix/ix-trace-rs\", branch = \"main\" }}\n[workspace]\n",
        scratch_crate::runtime_dependency(&["exact"]),
    );
    scratch_crate::write_manifest(&scratch.path, &manifest);
    fs::write(scratch.path.join("src/lib.rs"), source).expect("write public native agreement");
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let output = Command::new(env!("CARGO"))
        .args([
            "test",
            "--locked",
            "--offline",
            "-j2",
            "--lib",
            "--",
            "--test-threads=1",
            "--nocapture",
        ])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&scratch.path)
        .output()
        .expect("run native agreement");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stdout}\n{stderr}");
    assert!(
        output.status.success(),
        "native agreement failed: {stdout}\n{stderr}"
    );
    for index in 0..cases.len() {
        assert!(
            stdout
                .lines()
                .any(|line| line == format!("test case_{index} ... ok")),
            "missing native case_{index}: {stdout}"
        );
    }
}

const NATIVE_SUPPORT: &str = r#"
use std::num::NonZeroU64;
use quire_exact as rt;
use quire_semantic_value::declaration as sv;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Observation {
    Runtime(rt::Outcome<bool>),
    IllTyped(rt::IllTyped),
    InvalidConstant,
    OperandWidth,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Snapshot {
    outcome: Observation,
    charges: Vec<rt::AdmittedCharge>,
    counters: Vec<u64>,
}
fn snapshot(outcome: Observation, meter: &rt::Meter) -> Snapshot {
    Snapshot { outcome, charges: meter.admitted_charges().to_vec(),
        counters: rt::LimitKind::ALL.iter().map(|kind| meter.consumed(*kind)).collect() }
}
fn completed<T>(outcome: rt::Outcome<T>) -> Result<T, Observation> {
    match outcome {
        rt::Outcome::Completed(value) => Ok(value),
        rt::Outcome::Undefined(reason) => Err(Observation::Runtime(rt::Outcome::Undefined(reason))),
        rt::Outcome::Refused(reason) => Err(Observation::Runtime(rt::Outcome::Refused(reason))),
        rt::Outcome::Incomplete(record) => Err(Observation::Runtime(rt::Outcome::Incomplete(record))),
    }
}
#[derive(Debug)]
enum Expr {
    Literal(&'static str), Input,
    Arithmetic(&'static str, Box<Expr>, Option<Box<Expr>>),
    Compare(&'static str, Box<Expr>, Box<Expr>),
    BooleanOp(&'static str, Box<Expr>, Box<Expr>),
}
fn lit(s: &'static str) -> Expr { Expr::Literal(s) }
fn input() -> Expr { Expr::Input }
fn a(op: &'static str, l: Expr, r: Expr) -> Expr { Expr::Arithmetic(op, Box::new(l), Some(Box::new(r))) }
fn neg(v: Expr) -> Expr { Expr::Arithmetic("neg", Box::new(v), None) }
fn c(op: &'static str, l: Expr, r: Expr) -> Expr { Expr::Compare(op, Box::new(l), Box::new(r)) }
fn b(op: &'static str, l: Expr, r: Expr) -> Expr { Expr::BooleanOp(op, Box::new(l), Box::new(r)) }
impl Expr {
    fn integer(&self, x: u64, meter: &mut rt::Meter) -> Result<rt::Integer, Observation> {
        match self {
            // Every operand, including source-negative children and field inputs,
            // is prepared by the canonical parser, never the emitted fast path.
            Self::Literal(spelling) => Ok(spelling.parse().expect("canonical literal")),
            Self::Input => Ok(x.to_string().parse().expect("canonical field")),
            Self::Arithmetic(op, l, r) => {
                let left = l.integer(x, meter)?;
                let right = match r { Some(r) => Some(r.integer(x, meter)?), None => None };
                let operation = match *op {
                    "add" => rt::IntegerArithmetic::Add(&left, right.as_ref().unwrap()),
                    "sub" => rt::IntegerArithmetic::Subtract(&left, right.as_ref().unwrap()),
                    "mul" => rt::IntegerArithmetic::Multiply(&left, right.as_ref().unwrap()),
                    "neg" => rt::IntegerArithmetic::Negate(&left),
                    other => panic!("unknown reference arithmetic {other}"),
                };
                completed(rt::evaluate_integer_arithmetic(operation, None, meter))
            }
            other => panic!("noninteger reference {other:?}"),
        }
    }
    fn boolean(&self, x: u64, meter: &mut rt::Meter) -> Result<bool, Observation> {
        match self {
            Self::Compare(op, l, r) => {
                let left = l.integer(x, meter)?;
                let right = r.integer(x, meter)?;
                let outcome = match *op {
                    "eq" | "ne" => {
                        let environment = sv::TypeEnvironment::default();
                        let operator = if *op == "eq" { sv::EqualityOperator::Equal } else { sv::EqualityOperator::NotEqual };
                        let checked = environment.check_equality(operator,
                            sv::EqualityOperand::typed(rt::ValueType::Integer),
                            sv::EqualityOperand::typed(rt::ValueType::Integer),
                            &quire_semantic_value::enumeration::EnumMemberIndex::default())
                            .map_err(Observation::IllTyped)?;
                        checked.evaluate(&environment, &rt::Value::Integer(left), &rt::Value::Integer(right), meter)
                    }
                    _ => {
                        let operator = match *op {
                            "lt" => rt::OrderingOperator::Less,
                            "le" => rt::OrderingOperator::LessOrEqual,
                            "gt" => rt::OrderingOperator::Greater,
                            "ge" => rt::OrderingOperator::GreaterOrEqual,
                            other => panic!("unknown reference comparison {other}"),
                        };
                        rt::order_numbers(operator, rt::OrderedOperands::Integers(&left, &right), meter)
                    }
                };
                completed(outcome)
            }
            Self::BooleanOp(op, l, r) => {
                let left = l.boolean(x, meter)?;
                // Independently decide laziness before the connective's retention.
                let right = match (*op, left) {
                    ("and", false) | ("or", true) => false,
                    _ => r.boolean(x, meter)?,
                };
                let connective = match *op {
                    "and" => rt::BooleanConnective::And(left, right),
                    "or" => rt::BooleanConnective::Or(left, right),
                    other => panic!("unknown reference Boolean {other}"),
                };
                completed(rt::evaluate_boolean(connective, meter))
            }
            other => panic!("nonboolean reference {other:?}"),
        }
    }
    fn outcome(&self, x: u64, meter: &mut rt::Meter) -> Observation {
        match self.boolean(x, meter) {
            Ok(value) => Observation::Runtime(rt::Outcome::Completed(value)),
            Err(stop) => stop,
        }
    }
}
fn generous() -> rt::ScalarLimits {
    rt::ScalarLimits { integer_bits: u64::MAX, decimal_digits: u64::MAX,
        scale_expansion: u64::MAX, text_input_bytes: u64::MAX,
        text_scalars: u64::MAX, normalized_scalars: u64::MAX,
        unit_edges: u64::MAX, value_occurrences: u64::MAX,
        work_units: u64::MAX, result_units: u64::MAX }
}
fn set_limit(limits: &mut rt::ScalarLimits, kind: rt::LimitKind, value: u64) {
    match kind {
        rt::LimitKind::IntegerBits => limits.integer_bits = value,
        rt::LimitKind::DecimalDigits => limits.decimal_digits = value,
        rt::LimitKind::ScaleExpansion => limits.scale_expansion = value,
        rt::LimitKind::TextInputBytes => limits.text_input_bytes = value,
        rt::LimitKind::TextScalars => limits.text_scalars = value,
        rt::LimitKind::NormalizedScalars => limits.normalized_scalars = value,
        rt::LimitKind::UnitEdges => limits.unit_edges = value,
        rt::LimitKind::ValueOccurrences => limits.value_occurrences = value,
        rt::LimitKind::WorkUnits => limits.work_units = value,
        rt::LimitKind::ResultUnits => limits.result_units = value,
    }
}
fn agree(label: &str, expression: &Expr, x: u64, generated: &impl Fn(u64, &mut rt::Meter) -> Observation,
    limits: rt::ScalarLimits, denial: Option<rt::InjectedDenial>) -> Snapshot {
    let make_meter = || match denial {
        Some(denial) => rt::Meter::new(limits).with_injected_denial(denial),
        None => rt::Meter::new(limits),
    };
    let mut expected_meter = make_meter();
    let expected = snapshot(expression.outcome(x, &mut expected_meter), &expected_meter);
    let mut actual_meter = make_meter();
    let actual = snapshot(generated(x, &mut actual_meter), &actual_meter);
    assert_eq!(actual, expected, "{label}, x={x}, limits={limits:?}, denial={denial:?}");
    actual
}
fn exercise(label: &str, expression: &Expr, expected_value: impl Fn(u64) -> bool,
    skipped_arithmetic: bool, generated: impl Fn(u64, &mut rt::Meter) -> Observation) {
    for x in [0, 1, 4, 5, 6, 1000] {
        let baseline = agree(label, expression, x, &generated, generous(), None);
        assert_eq!(baseline.outcome, Observation::Runtime(rt::Outcome::Completed(expected_value(x))), "mathematics: {label}, x={x}");
        if skipped_arithmetic {
            let Expr::BooleanOp(operator, left, _) = expression else {
                panic!("skipped control must be a Boolean operation");
            };
            let mut prefix_meter = rt::Meter::new(generous());
            let left_value = left.boolean(x, &mut prefix_meter).expect("left comparison completes");
            assert!(matches!((*operator, left_value), ("and", false) | ("or", true)));
            let prefix = prefix_meter.admitted_charges().to_vec();
            assert_eq!(&baseline.charges[..prefix.len()], prefix.as_slice(), "left comparison prefix: {label}");
            assert_eq!(rt::retain_boolean(expected_value(x), &mut prefix_meter), Ok(expected_value(x)));
            assert_eq!(baseline, snapshot(Observation::Runtime(rt::Outcome::Completed(expected_value(x))), &prefix_meter), "comparison then connective retention only: {label}");
            for point in [rt::ChargePoint::IntegerArithmeticOperands,
                rt::ChargePoint::IntegerArithmeticArithmetic,
                rt::ChargePoint::IntegerArithmeticResultRetain] {
                assert!(!baseline.charges.iter().any(|charge| charge.point == point), "skipped arithmetic charged: {label}, {point:?}");
                // A denial located solely in the skipped subtree cannot stop it.
                let skip = rt::InjectedDenial { point, occurrence: NonZeroU64::new(1).unwrap() };
                assert_eq!(agree(label, expression, x, &generated, generous(), Some(skip)), baseline);
            }
        }
        // Deny EVERY ordered admitted occurrence, not only each distinct point.
        for (index, charge) in baseline.charges.iter().enumerate() {
            let occurrence = baseline.charges[..=index].iter().filter(|previous| previous.point == charge.point).count() as u64;
            let denial = rt::InjectedDenial { point: charge.point, occurrence: NonZeroU64::new(occurrence).unwrap() };
            let denied = agree(label, expression, x, &generated, generous(), Some(denial));
            assert_eq!(denied.charges, baseline.charges[..index], "non-prefix/continued child: {label}, x={x}, index={index}");
            match denied.outcome {
                Observation::Runtime(rt::Outcome::Incomplete(record)) => assert_eq!(record.charge_point, charge.point),
                other => panic!("denial did not stop at {index}: {other:?}"),
            }
        }
        for (kind, consumed) in rt::LimitKind::ALL.into_iter().zip(&baseline.counters) {
            for limit in [0, *consumed, consumed.saturating_sub(1)] {
                let mut limits = generous();
                set_limit(&mut limits, kind, limit);
                let limited = agree(label, expression, x, &generated, limits, None);
                if limit >= *consumed { assert_eq!(limited, baseline); }
                else { assert!(matches!(limited.outcome, Observation::Runtime(rt::Outcome::Incomplete(_))), "shortage did not stop: {label}, {kind:?}"); }
            }
        }
    }
}
"#;
