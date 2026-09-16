//! Admitted CheckedPackage V2 fixtures for exact scalar generation.
//!
//! The base is QSpec's `positive-nominal-identities.json` I04 vector, vendored
//! unchanged from Contract IR, which supplies an enum declaration, one of its
//! members, a dimension and a declared unit under their honest nominal keys.
//! Scalar types, values and expressions are appended under readable
//! zero-padded keys, and the package identity is re-derived exactly as the
//! Contract IR fixture support does.
//!
//! Bounds are `bounded_domain` nodes keyed by the digest of their content and
//! listed in the dependencies of each expression that uses them, so each
//! expression reaches exactly the bounds it declares. Their bodies use the
//! encoding `quire_contract_codegen::exact_scalar` documents.

#![allow(dead_code)] // Each test binary uses a different subset.

use std::collections::BTreeSet;

use quire_contract_codegen::{
    DecimalOperator, ExactScalarItem, ExactScalarOperation, IeeeArithmeticOperator,
    IntegerOperator, OrderingOperandKind, QuantityOperator, RationalOperator,
};
use quire_contract_ir::{
    CheckedArtifactLocator, CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult,
};
use quire_contract_runtime::exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, Integer,
    IntegerDomain, IntegerInterval, OrderingOperator, QuantityTarget, RationalDomain, RoundingMode,
    TextProfile, TextType,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";
pub const ENUM_TYPE: &str = "7928f1e1b570335b404c8d21c66da8a3b8e37e434b0ebc622f80285488811562";
pub const ENUM_MEMBER: &str = "42ba51e7e622d99f292a5e6dcc196bd216efb98b33755910e54af4d65078d032";
pub const UNIT_TYPE: &str = "79637623a46d29e884b62c6fa292aeb29d41e4ecc4e800b4d7ee910a3eaf23a4";

/// A readable node key: the code, zero-padded to a 64-digit digest.
pub fn key(code: u32) -> String {
    format!("{code:0>64}")
}

pub fn id(digest: &str) -> CheckedNodeId {
    serde_json::from_value(node_ref(digest)).expect("node id")
}

pub fn code_id(code: u32) -> CheckedNodeId {
    id(&key(code))
}

fn node_ref(digest: &str) -> Value {
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

pub fn reference(digest: &str) -> Value {
    json!({"term": "reference", "target": node_ref(digest)})
}

pub fn application(operator: &str, arguments: Vec<Value>) -> Value {
    json!({"term": "application", "operator": operator, "arguments": arguments})
}

fn aggregate() -> Value {
    json!({"term": "aggregate", "members": []})
}

pub fn literal(kind: &str, value: &str) -> Value {
    json!({"term": "literal", "value_kind": kind, "value": value})
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// A V2 package under construction.
pub struct PackageBuilder {
    value: Value,
    bounds: BTreeSet<String>,
}

impl Default for PackageBuilder {
    fn default() -> Self {
        let text = include_str!("../fixtures/exact_scalar/positive-nominal-identities.json");
        Self {
            value: serde_json::from_str(text).expect("vendored fixture is JSON"),
            bounds: BTreeSet::new(),
        }
    }
}

impl PackageBuilder {
    pub fn node(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.node_with(digest, tag, form, semantic_type, body, &[])
    }

    pub fn node_with(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        dependencies: &[String],
    ) -> &mut Self {
        let dependencies = dependencies
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|digest| node_ref(digest))
            .collect::<Vec<_>>();
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        nodes.push(json!({
            "node_id": node_ref(digest),
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": tag,
            "semantic_form": form,
            "semantic_type": node_ref(semantic_type),
            "dependencies": dependencies,
            "occurrences": [{"role": "declaration", "ordinal": 0}],
            "body": body,
        }));
        let source = self.value["lock"]["sources"][0].clone();
        let map = self.value["source_map"].as_array_mut().expect("source map");
        let start = map.len();
        map.push(json!({
            "node_id": node_ref(digest),
            "role": "declaration",
            "ordinal": 0,
            "regions": [{
                "source": source,
                "start": start,
                "end": start + 1,
            }],
        }));
        self
    }

    pub fn code(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.node(&key(code), tag, form, semantic_type, body)
    }

    /// A node that depends on `bounds`, adding each bound node once.
    pub fn bounded(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        bounds: &[Bound],
    ) -> &mut Self {
        let keys = bounds
            .iter()
            .map(|bound| self.bound(bound))
            .collect::<Vec<_>>();
        self.node_with(&key(code), tag, form, semantic_type, body, &keys)
    }

    /// Add `bound` once, returning its key.
    pub fn bound(&mut self, bound: &Bound) -> String {
        let digest = bound.key();
        if self.bounds.insert(digest.clone()) {
            self.node(
                &digest,
                "bounded_domain",
                bound.form(),
                &bound.bounded_type(),
                bound.body(),
            );
        }
        digest
    }

    /// The wire document with its identity projection and package id refreshed.
    pub fn wire(&self) -> Value {
        let mut package = self.value.clone();
        let projection = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .cloned()
            .map(|mut node| {
                node.as_object_mut().expect("node").remove("occurrences");
                node
            })
            .collect::<Vec<_>>();
        package["identity_preimage"]["identity_projection"] = Value::Array(projection);
        let preimage = serde_json::to_vec(&package["identity_preimage"]).expect("preimage");
        package["package_id"]["digest"] = json!(sha256_hex(&preimage));
        package
    }

    pub fn admit(&self) -> CheckedPackageV2 {
        self.admit_with(CheckedPackageReadLimits::bounded())
    }

    pub fn admit_with(&self, limits: CheckedPackageReadLimits) -> CheckedPackageV2 {
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        match CheckedPackageV2::read(&bytes, limits, &evidence(&wire)) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("expected V2 admission, got {other:?}"),
        }
    }
}

fn locator(artifact: &Value) -> CheckedArtifactLocator {
    let text = |value: &Value| -> Box<str> { value.as_str().expect("locator member").into() };
    CheckedArtifactLocator {
        authority: text(&artifact["authority"]),
        identity: text(&artifact["identity"]),
        revision_namespace: text(&artifact["revision"]["namespace"]),
        revision_value: text(&artifact["revision"]["value"]),
        domain: text(&artifact["digest_domain"]),
    }
}

fn evidence(package: &Value) -> CheckedPackageEvidence {
    let lock = &package["lock"];
    let mut artifacts = lock["sources"].as_array().cloned().unwrap_or_default();
    artifacts.push(lock["edition"]["definition"].clone());
    artifacts.extend(
        lock["definition_selections"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    artifacts.push(package["diagnostics"]["catalog"].clone());
    let mut evidence = CheckedPackageEvidence::new();
    for artifact in &artifacts {
        evidence.insert_artifact_digest(
            locator(artifact),
            artifact["digest"].as_str().expect("digest"),
        );
    }
    evidence.support_feature("quire.value.complete/v1");
    evidence
}

// ---- bounds ------------------------------------------------------------------

/// One `bounded_domain` node on a corpus scalar type.
#[derive(Clone, Debug)]
pub enum Bound {
    /// `integer_range` on `T_INTEGER`.
    Integer(i64, i64),
    /// `rational_range` on `T_RATIONAL`: numerator and denominator intervals.
    Rational(i64, i64, i64, i64),
    /// `decimal_range` on `T_DECIMAL`.
    Decimal(i64, i64, u64, u64, &'static str),
    /// `float_rounding` on the named float type.
    Rounding(&'static str, &'static str),
    /// `text_bounds` on `T_TEXT`.
    Text(u64, u64, &'static str),
    /// Any form on any corpus type with any body.
    Raw {
        form: &'static str,
        bounded: &'static str,
        body: Value,
    },
}

fn integer_literal(value: impl ToString) -> Value {
    literal("integer", &value.to_string())
}

impl Bound {
    pub fn form(&self) -> &'static str {
        match self {
            Self::Integer(..) => "integer_range",
            Self::Rational(..) => "rational_range",
            Self::Decimal(..) => "decimal_range",
            Self::Rounding(..) => "float_rounding",
            Self::Text(..) => "text_bounds",
            Self::Raw { form, .. } => form,
        }
    }

    fn bounded_form(&self) -> &'static str {
        match self {
            Self::Integer(..) => "integer",
            Self::Rational(..) => "rational",
            Self::Decimal(..) => "decimal",
            Self::Rounding(float, _) => float,
            Self::Text(..) => "text",
            Self::Raw { bounded, .. } => bounded,
        }
    }

    pub fn bounded_type(&self) -> String {
        result_type(self.bounded_form())
    }

    pub fn body(&self) -> Value {
        let members = match self {
            Self::Integer(lower, upper) => vec![integer_literal(lower), integer_literal(upper)],
            Self::Rational(nl, nu, dl, du) => vec![
                integer_literal(nl),
                integer_literal(nu),
                integer_literal(dl),
                integer_literal(du),
            ],
            Self::Decimal(lower, upper, min, max, rounding) => vec![
                integer_literal(lower),
                integer_literal(upper),
                integer_literal(min),
                integer_literal(max),
                literal("text", rounding),
            ],
            Self::Rounding(_, rounding) => vec![literal("text", rounding)],
            Self::Text(min, max, profile) => vec![
                integer_literal(min),
                integer_literal(max),
                literal("text", profile),
            ],
            Self::Raw { body, .. } => return body.clone(),
        };
        json!({"term": "aggregate", "members": members})
    }

    /// The node key: a digest of the form, the bounded type and the body.
    pub fn key(&self) -> String {
        let content = json!([self.form(), self.bounded_type(), self.body()]);
        sha256_hex(&serde_json::to_vec(&content).expect("bound content"))
    }
}

/// An integer range whose upper bound is spelled non-canonically.
pub fn unreadable_bound() -> Bound {
    Bound::Raw {
        form: "integer_range",
        bounded: "integer",
        body: json!({
            "term": "aggregate",
            "members": [literal("integer", "-5"), literal("integer", "05")],
        }),
    }
}

pub const INT: Bound = Bound::Integer(-1000, 1000);
pub const INT5: Bound = Bound::Integer(-5, 5);
pub const RAT: Bound = Bound::Rational(-1000, 1000, 1, 1000);
pub const DEC: Bound = Bound::Decimal(-1000, 1000, 0, 2, "nearest-even");
pub const TEXT: Bound = Bound::Text(0, 64, "nfc");

// ---- the scalar corpus -------------------------------------------------------

pub const T_BOOLEAN: u32 = 1;
pub const T_INTEGER: u32 = 2;
pub const T_RATIONAL: u32 = 3;
pub const T_DECIMAL: u32 = 4;
pub const T_FLOAT32: u32 = 5;
pub const T_FLOAT64: u32 = 6;
pub const T_TEXT: u32 = 7;
pub const V_BOOLEAN: u32 = 101;
pub const V_INTEGER: u32 = 102;
pub const V_RATIONAL: u32 = 103;
pub const V_DECIMAL: u32 = 104;
pub const V_FLOAT32: u32 = 105;
pub const V_FLOAT64: u32 = 106;
pub const V_TEXT: u32 = 107;
pub const V_QUANTITY: u32 = 108;
/// A value whose semantic type is a bound, not a scalar type.
pub const V_UNTYPED: u32 = 109;

/// The corpus integer subtraction, whose right operand is an inline literal.
pub const LITERAL_OPERAND: u32 = 1003;

/// Non-generated nodes, each with the reason it is refused.
pub const COMPOSITE: u32 = 2001;
pub const FUNCTION: u32 = 2002;
pub const MODEL: u32 = 2003;
pub const RELATION: u32 = 2004;
pub const STATE: u32 = 2005;
pub const TEMPORAL: u32 = 2006;
pub const PROTOCOL: u32 = 2007;
pub const CALLS_FUNCTION: u32 = 2011;
pub const WRONG_BODY: u32 = 2013;
pub const WRONG_OPERAND: u32 = 2014;
/// Well-formed integer additions requested with a refused request shape.
pub const DUPLICATED: u32 = 2015;
pub const WRONG_RESULT: u32 = 2016;
pub const WRONG_ARITY: u32 = 2017;
/// An integer addition reaching an unbounded integer type.
pub const UNBOUNDED: u32 = 2021;
/// An integer addition bounded in the IR, requested as mathematical.
pub const MATHEMATICAL: u32 = 2022;
/// A binary32 addition with no rounding bound.
pub const MISSING_ROUNDING: u32 = 2023;
/// An integer addition reaching two integer ranges.
pub const AMBIGUOUS: u32 = 2024;
/// An integer addition whose range spells a bound non-canonically.
pub const UNREADABLE: u32 = 2025;
/// An integer addition whose only integer bound has the text form.
pub const WRONG_BOUND_FORM: u32 = 2026;
/// A floor division bounded `[-5, 5]`, requested over `[-1000, 1000]`.
pub const DOMAIN_MISMATCH: u32 = 2027;
/// A quantity conversion to a bounded rational, requested as exact.
pub const QUANTITY_EXACT: u32 = 2028;
/// An integer addition whose left operand is itself an application.
pub const EXPRESSION_OPERAND: u32 = 2029;
/// A quantity addition whose right operand is a literal.
pub const LITERAL_QUANTITY: u32 = 2030;
/// An integer addition whose right operand has no scalar type.
pub const UNTYPED_OPERAND: u32 = 2031;
pub const MISSING: u32 = 9999;

fn integer(value: i64) -> Integer {
    Integer::from(value)
}

pub fn interval(lower: i64, upper: i64) -> IntegerInterval {
    IntegerInterval::new(integer(lower), integer(upper)).expect("interval")
}

pub fn bounded(lower: i64, upper: i64) -> IntegerDomain {
    IntegerDomain::Bounded(interval(lower, upper))
}

pub fn rational_domain(nl: i64, nu: i64, dl: i64, du: i64) -> RationalDomain {
    RationalDomain::new(interval(nl, nu), interval(dl, du)).expect("domain")
}

pub fn decimal_type(
    lower: i64,
    upper: i64,
    min: u64,
    max: u64,
    rounding: RoundingMode,
) -> DecimalType {
    DecimalType::new(integer(lower), integer(upper), min, max, rounding).expect("decimal type")
}

/// One generated expression of the corpus.
pub struct Expression {
    pub code: u32,
    pub form: &'static str,
    pub operator: &'static str,
    pub operands: &'static [&'static str],
    pub result: &'static str,
    pub operation: ExactScalarOperation,
    pub bounds: Vec<Bound>,
}

fn expression(
    code: u32,
    shape: (&'static str, &'static str),
    operands: &'static [&'static str],
    result: &'static str,
    operation: ExactScalarOperation,
    bounds: Vec<Bound>,
) -> Expression {
    Expression {
        code,
        form: shape.0,
        operator: shape.1,
        operands,
        result,
        operation,
        bounds,
    }
}

const BINARY: (&str, &str) = ("binary", "binary");
const UNARY: (&str, &str) = ("unary", "unary");
const CONVERSION: (&str, &str) = ("conversion", "convert");

const INT2: &[&str] = &["integer", "integer"];
const INT1: &[&str] = &["integer"];
const RAT2: &[&str] = &["rational", "rational"];
const RAT1: &[&str] = &["rational"];
const DEC2: &[&str] = &["decimal", "decimal"];
const DEC1: &[&str] = &["decimal"];
const F32_2: &[&str] = &["float32", "float32"];
const F64_2: &[&str] = &["float64", "float64"];
const F64_1: &[&str] = &["float64"];
const TEXT1: &[&str] = &["text"];
const TEXT2: &[&str] = &["text", "text"];
const ENUM2: &[&str] = &["enum", "enum"];
const QTY2: &[&str] = &["unit", "unit"];
const QTY_INT: &[&str] = &["unit", "integer"];
const QTY1: &[&str] = &["unit"];

/// Codes of the text, enum and quantity comparisons, in `ComparisonOperator::ALL` order.
pub const TEXT_COMPARISONS: [u32; 6] = [1111, 1112, 1072, 1113, 1114, 1115];
pub const ENUM_COMPARISONS: [u32; 6] = [1121, 1122, 1073, 1123, 1124, 1125];
pub const QUANTITY_COMPARISONS: [u32; 6] = [1131, 1132, 1084, 1133, 1134, 1135];
/// Codes of the text admissions, in `TextProfile::ALL` order, all `[0, 4]`.
pub const TEXT_ADMISSIONS: [u32; 6] = [1074, 1071, 1075, 1076, 1077, 1078];

/// Every generated family: each integer, rational and decimal operator, each
/// division law, each IEEE operator and comparison exercised, each text
/// profile, each comparison operator, and each bounded conversion target.
pub fn corpus() -> Vec<Expression> {
    use ExactScalarOperation as Op;
    let integer = |code, shape, operands, operator, domain: IntegerDomain, bound: Bound| {
        expression(
            code,
            shape,
            operands,
            "integer",
            Op::IntegerArithmetic { operator, domain },
            vec![bound],
        )
    };
    let division = |code, profile, lower, upper, bound: Bound| {
        expression(
            code,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile,
                domain: bounded(lower, upper),
            },
            vec![bound],
        )
    };
    let rational = |code, shape, operands, operator, domain, bounds| {
        expression(
            code,
            shape,
            operands,
            "rational",
            Op::RationalArithmetic {
                operator,
                domain: Some(domain),
            },
            bounds,
        )
    };
    let ordering = |code, operands, operator, kind, bound: Bound| {
        expression(
            code,
            BINARY,
            operands,
            "boolean",
            Op::Ordering {
                operator,
                operands: kind,
            },
            vec![bound],
        )
    };
    let decimal = |code, shape, operands, operator, target, bound: Bound| {
        expression(
            code,
            shape,
            operands,
            "decimal",
            Op::DecimalArithmetic { operator, target },
            vec![bound],
        )
    };
    let ieee = |code, operands, result, operator, width, rounding: RoundingMode| {
        expression(
            code,
            BINARY,
            operands,
            result,
            Op::IeeeArithmetic {
                operator,
                width,
                rounding,
            },
            vec![Bound::Rounding(result, rounding.as_str())],
        )
    };
    let ieee_comparison = |code, operands, comparison, width| {
        expression(
            code,
            BINARY,
            operands,
            "boolean",
            Op::IeeeComparison { comparison, width },
            Vec::new(),
        )
    };
    let wide_decimal = || decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven);
    let quantity = |code, operands, operator| {
        expression(
            code,
            BINARY,
            operands,
            "unit",
            Op::QuantityArithmetic { operator },
            if operands == QTY_INT {
                vec![INT]
            } else {
                Vec::new()
            },
        )
    };

    let mut corpus = vec![
        integer(
            1001,
            BINARY,
            INT2,
            IntegerOperator::Add,
            bounded(-1000, 1000),
            INT,
        ),
        integer(
            1002,
            UNARY,
            INT1,
            IntegerOperator::Negate,
            bounded(-8, 7),
            Bound::Integer(-8, 7),
        ),
        integer(
            1003,
            BINARY,
            INT2,
            IntegerOperator::Subtract,
            bounded(-1000, 1000),
            INT,
        ),
        integer(
            1004,
            BINARY,
            INT2,
            IntegerOperator::Multiply,
            bounded(-1000, 1000),
            INT,
        ),
        division(1011, DivisionProfile::Truncating, -1000, 1000, INT),
        division(1012, DivisionProfile::Floor, -1000, 1000, INT),
        division(1013, DivisionProfile::Euclidean, -1000, 1000, INT),
        division(1014, DivisionProfile::Truncating, -5, 5, INT5),
        expression(
            1021,
            BINARY,
            INT2,
            "integer",
            Op::IntegerModulo {
                domain: bounded(-5, 5),
            },
            vec![INT5],
        ),
        rational(
            1031,
            BINARY,
            RAT2,
            RationalOperator::Add,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1032,
            BINARY,
            RAT2,
            RationalOperator::Divide,
            rational_domain(-10, 10, 1, 4),
            vec![Bound::Rational(-10, 10, 1, 4)],
        ),
        rational(
            1033,
            BINARY,
            INT2,
            RationalOperator::IntegerDivide,
            rational_domain(-1000, 1000, 1, 1000),
            vec![INT, RAT],
        ),
        rational(
            1034,
            BINARY,
            RAT2,
            RationalOperator::Subtract,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1035,
            BINARY,
            RAT2,
            RationalOperator::Multiply,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1036,
            UNARY,
            RAT1,
            RationalOperator::Negate,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        ordering(
            1041,
            INT2,
            OrderingOperator::Less,
            OrderingOperandKind::Integer,
            INT,
        ),
        ordering(
            1042,
            DEC2,
            OrderingOperator::LessOrEqual,
            OrderingOperandKind::Decimal,
            DEC,
        ),
        ordering(
            1043,
            RAT2,
            OrderingOperator::Greater,
            OrderingOperandKind::Rational,
            RAT,
        ),
        ordering(
            1044,
            INT2,
            OrderingOperator::LessOrEqual,
            OrderingOperandKind::Integer,
            INT,
        ),
        ordering(
            1045,
            INT2,
            OrderingOperator::GreaterOrEqual,
            OrderingOperandKind::Integer,
            INT,
        ),
        decimal(
            1051,
            BINARY,
            DEC2,
            DecimalOperator::Add,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1052,
            BINARY,
            DEC2,
            DecimalOperator::Divide,
            decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero),
            Bound::Decimal(-1000, 1000, 0, 2, "toward-zero"),
        ),
        decimal(
            1053,
            CONVERSION,
            DEC1,
            DecimalOperator::Round,
            decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway),
            Bound::Decimal(-100, 100, 0, 0, "nearest-away"),
        ),
        decimal(
            1054,
            BINARY,
            DEC2,
            DecimalOperator::Subtract,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1055,
            BINARY,
            DEC2,
            DecimalOperator::Multiply,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1056,
            UNARY,
            DEC1,
            DecimalOperator::Negate,
            wide_decimal(),
            DEC,
        ),
        ieee(
            1061,
            F32_2,
            "float32",
            IeeeArithmeticOperator::Add,
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
        ),
        ieee(
            1062,
            F64_2,
            "float64",
            IeeeArithmeticOperator::Divide,
            IeeeWidth::Binary64,
            RoundingMode::TowardZero,
        ),
        ieee_comparison(1063, F64_2, IeeeComparison::TotalOrder, IeeeWidth::Binary64),
        expression(
            1064,
            CONVERSION,
            F64_1,
            "float32",
            Op::IeeeWidthConversion {
                source: IeeeWidth::Binary64,
                target: IeeeWidth::Binary32,
                rounding: RoundingMode::NearestEven,
            },
            vec![Bound::Rounding("float32", "nearest-even")],
        ),
        ieee(
            1065,
            F32_2,
            "float32",
            IeeeArithmeticOperator::Subtract,
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
        ),
        ieee(
            1066,
            F64_2,
            "float64",
            IeeeArithmeticOperator::Multiply,
            IeeeWidth::Binary64,
            RoundingMode::TowardPositive,
        ),
        ieee_comparison(
            1067,
            F32_2,
            IeeeComparison::NumericEqual,
            IeeeWidth::Binary32,
        ),
        ieee_comparison(
            1068,
            F64_2,
            IeeeComparison::BitIdentical,
            IeeeWidth::Binary64,
        ),
        quantity(1081, QTY2, QuantityOperator::Add),
        quantity(1082, QTY2, QuantityOperator::Multiply),
        quantity(1083, QTY_INT, QuantityOperator::Power),
        quantity(1088, QTY2, QuantityOperator::Subtract),
        quantity(1089, QTY2, QuantityOperator::Divide),
        expression(
            1086,
            CONVERSION,
            QTY1,
            "decimal",
            Op::QuantityConversion {
                target: QuantityTarget::Decimal(decimal_type(
                    -100_000,
                    100_000,
                    0,
                    2,
                    RoundingMode::NearestEven,
                )),
            },
            vec![Bound::Decimal(-100_000, 100_000, 0, 2, "nearest-even")],
        ),
        expression(
            1087,
            CONVERSION,
            QTY1,
            "integer",
            Op::QuantityConversion {
                target: QuantityTarget::Integer {
                    domain: interval(-1000, 1000),
                    rounding: RoundingMode::TowardZero,
                },
            },
            vec![INT],
        ),
    ];
    for (code, profile) in TEXT_ADMISSIONS.into_iter().zip(TextProfile::ALL) {
        corpus.push(expression(
            code,
            CONVERSION,
            TEXT1,
            "text",
            Op::TextAdmission {
                text_type: TextType::new(0, 4, profile).expect("text type"),
            },
            vec![Bound::Text(0, 4, profile.as_str())],
        ));
    }
    for (index, operator) in ComparisonOperator::ALL.into_iter().enumerate() {
        corpus.push(expression(
            TEXT_COMPARISONS[index],
            BINARY,
            TEXT2,
            "boolean",
            Op::TextComparison { operator },
            vec![TEXT],
        ));
        corpus.push(expression(
            ENUM_COMPARISONS[index],
            BINARY,
            ENUM2,
            "boolean",
            Op::EnumComparison { operator },
            Vec::new(),
        ));
        corpus.push(expression(
            QUANTITY_COMPARISONS[index],
            BINARY,
            QTY2,
            "boolean",
            Op::QuantityComparison { operator },
            Vec::new(),
        ));
    }
    corpus.sort_by_key(|expression| expression.code);
    corpus
}

/// The operand value node of a scalar form.
fn operand(form: &str) -> String {
    match form {
        "integer" => key(V_INTEGER),
        "rational" => key(V_RATIONAL),
        "decimal" => key(V_DECIMAL),
        "float32" => key(V_FLOAT32),
        "float64" => key(V_FLOAT64),
        "text" => key(V_TEXT),
        "enum" => ENUM_MEMBER.to_owned(),
        "unit" => key(V_QUANTITY),
        other => panic!("no operand node for {other}"),
    }
}

/// The type node of a scalar form.
fn result_type(form: &str) -> String {
    match form {
        "boolean" => key(T_BOOLEAN),
        "integer" => key(T_INTEGER),
        "rational" => key(T_RATIONAL),
        "decimal" => key(T_DECIMAL),
        "float32" => key(T_FLOAT32),
        "float64" => key(T_FLOAT64),
        "text" => key(T_TEXT),
        "unit" => UNIT_TYPE.to_owned(),
        other => panic!("no type node for {other}"),
    }
}

fn integer_pair() -> Value {
    application(
        "binary",
        vec![reference(&key(V_INTEGER)), reference(&key(V_INTEGER))],
    )
}

/// The complete package: types, values, the corpus, and refused nodes.
pub fn corpus_package() -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    for (code, form) in [
        (T_BOOLEAN, "boolean"),
        (T_INTEGER, "integer"),
        (T_RATIONAL, "rational"),
        (T_DECIMAL, "decimal"),
        (T_FLOAT32, "float32"),
        (T_FLOAT64, "float64"),
        (T_TEXT, "text"),
    ] {
        builder.code(code, "scalar_type", form, &key(code), aggregate());
    }
    for (code, ty, kind, value) in [
        (V_INTEGER, key(T_INTEGER), "integer", "3"),
        (V_RATIONAL, key(T_RATIONAL), "rational", "1/2"),
        (V_DECIMAL, key(T_DECIMAL), "decimal", "1.5"),
        (V_FLOAT32, key(T_FLOAT32), "float32_bits", "0"),
        (V_FLOAT64, key(T_FLOAT64), "float64_bits", "0"),
        (V_TEXT, key(T_TEXT), "text", "a"),
        (V_QUANTITY, UNIT_TYPE.to_owned(), "rational", "1"),
    ] {
        builder.code(code, "value", "literal", &ty, literal(kind, value));
    }
    builder.code(
        V_BOOLEAN,
        "value",
        "literal",
        &key(T_BOOLEAN),
        json!({"term": "literal", "value_kind": "boolean", "value": true}),
    );
    let int_bound = builder.bound(&INT);
    builder.code(
        V_UNTYPED,
        "value",
        "literal",
        &int_bound,
        literal("integer", "3"),
    );
    for expression in corpus() {
        let mut arguments = expression
            .operands
            .iter()
            .map(|form| reference(&operand(form)))
            .collect::<Vec<_>>();
        if expression.code == LITERAL_OPERAND {
            arguments[1] = literal("integer", "3");
        }
        builder.bounded(
            expression.code,
            "expression",
            expression.form,
            &result_type(expression.result),
            application(expression.operator, arguments),
            &expression.bounds,
        );
    }
    let integer_type = key(T_INTEGER);
    for (code, bounds) in [
        (DUPLICATED, vec![INT]),
        (WRONG_RESULT, vec![INT]),
        (WRONG_ARITY, vec![INT]),
        (UNBOUNDED, Vec::new()),
        (MATHEMATICAL, vec![INT]),
        (AMBIGUOUS, vec![INT, INT5]),
        (UNREADABLE, vec![unreadable_bound()]),
        (
            WRONG_BOUND_FORM,
            vec![Bound::Raw {
                form: "text_bounds",
                bounded: "integer",
                body: Bound::Text(0, 4, "nfc").body(),
            }],
        ),
        (DOMAIN_MISMATCH, vec![INT5]),
    ] {
        builder.bounded(
            code,
            "expression",
            "binary",
            &integer_type,
            integer_pair(),
            &bounds,
        );
    }
    builder
        .bounded(
            MISSING_ROUNDING,
            "expression",
            "binary",
            &key(T_FLOAT32),
            application(
                "binary",
                vec![reference(&key(V_FLOAT32)), reference(&key(V_FLOAT32))],
            ),
            &[],
        )
        .bounded(
            QUANTITY_EXACT,
            "expression",
            "conversion",
            &key(T_RATIONAL),
            application("convert", vec![reference(&key(V_QUANTITY))]),
            &[RAT],
        )
        .bounded(
            EXPRESSION_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application("binary", vec![integer_pair(), reference(&key(V_INTEGER))]),
            &[INT],
        )
        .bounded(
            LITERAL_QUANTITY,
            "expression",
            "binary",
            UNIT_TYPE,
            application(
                "binary",
                vec![reference(&key(V_QUANTITY)), literal("rational", "1")],
            ),
            &[],
        )
        .bounded(
            UNTYPED_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                vec![reference(&key(V_INTEGER)), reference(&key(V_UNTYPED))],
            ),
            &[INT],
        );
    let boolean = key(T_BOOLEAN);
    builder
        .code(COMPOSITE, "composite_type", "record", &boolean, aggregate())
        .code(
            FUNCTION,
            "function",
            "pure_function",
            &boolean,
            application("call", vec![]),
        )
        .code(MODEL, "model", "model_import", &boolean, aggregate())
        .code(RELATION, "relation", "relationship", &boolean, aggregate())
        .code(STATE, "state", "state_clause", &boolean, aggregate())
        .code(
            TEMPORAL,
            "temporal",
            "temporal_clause",
            &boolean,
            application("temporal", vec![]),
        )
        .code(
            PROTOCOL,
            "protocol",
            "protocol_clause",
            &boolean,
            application("protocol_control", vec![]),
        )
        .bounded(
            CALLS_FUNCTION,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                vec![reference(&key(FUNCTION)), reference(&key(V_INTEGER))],
            ),
            &[INT],
        )
        .bounded(
            WRONG_BODY,
            "expression",
            "binary",
            &integer_type,
            application("unary", vec![reference(&key(V_INTEGER))]),
            &[INT],
        )
        .bounded(
            WRONG_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                vec![reference(&key(V_INTEGER)), reference(&key(V_DECIMAL))],
            ),
            &[INT, DEC],
        );
    builder
}

pub fn integer_add() -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(-1000, 1000),
    }
}

/// Refused items of the golden request, each with an integer-add descriptor
/// unless the refusal is about the descriptor.
pub fn refused_items() -> Vec<ExactScalarItem> {
    let item = |code, operation| ExactScalarItem {
        node_id: code_id(code),
        operation,
    };
    let mut items: Vec<ExactScalarItem> = [
        COMPOSITE,
        FUNCTION,
        MODEL,
        RELATION,
        STATE,
        TEMPORAL,
        PROTOCOL,
        CALLS_FUNCTION,
        WRONG_BODY,
        WRONG_OPERAND,
        MISSING,
        V_BOOLEAN,
        DUPLICATED,
        DUPLICATED,
        UNBOUNDED,
        AMBIGUOUS,
        UNREADABLE,
        WRONG_BOUND_FORM,
        EXPRESSION_OPERAND,
        UNTYPED_OPERAND,
    ]
    .into_iter()
    .map(|code| item(code, integer_add()))
    .collect();
    items.push(item(
        WRONG_RESULT,
        ExactScalarOperation::DecimalArithmetic {
            operator: DecimalOperator::Add,
            target: decimal_type(-1, 1, 0, 0, RoundingMode::NearestEven),
        },
    ));
    items.push(item(
        WRONG_ARITY,
        ExactScalarOperation::IntegerArithmetic {
            operator: IntegerOperator::Negate,
            domain: bounded(-1000, 1000),
        },
    ));
    items.push(item(
        MATHEMATICAL,
        ExactScalarOperation::IntegerArithmetic {
            operator: IntegerOperator::Add,
            domain: IntegerDomain::Mathematical,
        },
    ));
    items.push(item(
        MISSING_ROUNDING,
        ExactScalarOperation::IeeeArithmetic {
            operator: IeeeArithmeticOperator::Add,
            width: IeeeWidth::Binary32,
            rounding: RoundingMode::NearestEven,
        },
    ));
    items.push(item(
        DOMAIN_MISMATCH,
        ExactScalarOperation::IntegerDivision {
            profile: DivisionProfile::Floor,
            domain: bounded(-1000, 1000),
        },
    ));
    items.push(item(
        QUANTITY_EXACT,
        ExactScalarOperation::QuantityConversion {
            target: QuantityTarget::Exact,
        },
    ));
    items.push(item(
        LITERAL_QUANTITY,
        ExactScalarOperation::QuantityArithmetic {
            operator: QuantityOperator::Add,
        },
    ));
    items
}

/// The golden request: every corpus expression plus every refused item.
pub fn golden_items() -> Vec<ExactScalarItem> {
    corpus()
        .into_iter()
        .map(|expression| ExactScalarItem {
            node_id: code_id(expression.code),
            operation: expression.operation,
        })
        .chain(refused_items())
        .collect()
}
