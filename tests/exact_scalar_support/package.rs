//! Admitted CheckedPackage V2 fixtures for exact scalar generation.
//!
//! The base is QSpec's `positive-nominal-identities.json` I04 vector, vendored
//! unchanged from Contract IR, which supplies an enum declaration, one of its
//! members, a dimension and a declared unit under their honest nominal keys.
//! Scalar types, values and expressions are appended under readable
//! zero-padded keys, and the package identity is re-derived exactly as the
//! Contract IR fixture support does.

#![allow(dead_code)] // Each test binary uses a different subset.

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

fn literal(kind: &str, value: &str) -> Value {
    json!({"term": "literal", "value_kind": kind, "value": value})
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// A V2 package under construction.
pub struct PackageBuilder {
    value: Value,
}

impl Default for PackageBuilder {
    fn default() -> Self {
        let text = include_str!("../fixtures/exact_scalar/positive-nominal-identities.json");
        Self {
            value: serde_json::from_str(text).expect("vendored fixture is JSON"),
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
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        nodes.push(json!({
            "node_id": node_ref(digest),
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": tag,
            "semantic_form": form,
            "semantic_type": node_ref(semantic_type),
            "dependencies": [],
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
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        match CheckedPackageV2::read(
            &bytes,
            CheckedPackageReadLimits::bounded(),
            &evidence(&wire),
        ) {
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

// ---- the scalar corpus -------------------------------------------------------

pub const T_BOOLEAN: u32 = 1;
pub const T_INTEGER: u32 = 2;
pub const T_RATIONAL: u32 = 3;
pub const T_DECIMAL: u32 = 4;
pub const T_FLOAT32: u32 = 5;
pub const T_FLOAT64: u32 = 6;
pub const T_TEXT: u32 = 7;
pub const V_INTEGER: u32 = 102;
pub const V_RATIONAL: u32 = 103;
pub const V_DECIMAL: u32 = 104;
pub const V_FLOAT32: u32 = 105;
pub const V_FLOAT64: u32 = 106;
pub const V_TEXT: u32 = 107;
pub const V_QUANTITY: u32 = 108;

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
pub const MISSING: u32 = 9999;

fn integer(value: i64) -> Integer {
    Integer::from(value)
}

fn interval(lower: i64, upper: i64) -> IntegerInterval {
    IntegerInterval::new(integer(lower), integer(upper)).expect("interval")
}

fn decimal_type(lower: i64, upper: i64, min: u64, max: u64, rounding: RoundingMode) -> DecimalType {
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
}

fn expression(
    code: u32,
    shape: (&'static str, &'static str),
    operands: &'static [&'static str],
    result: &'static str,
    operation: ExactScalarOperation,
) -> Expression {
    Expression {
        code,
        form: shape.0,
        operator: shape.1,
        operands,
        result,
        operation,
    }
}

const BINARY: (&str, &str) = ("binary", "binary");
const UNARY: (&str, &str) = ("unary", "unary");
const CONVERSION: (&str, &str) = ("conversion", "convert");

const INT2: &[&str] = &["integer", "integer"];
const INT1: &[&str] = &["integer"];
const RAT2: &[&str] = &["rational", "rational"];
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

/// Every generated family, one expression each (division profiles and
/// conversion targets once per law).
pub fn corpus() -> Vec<Expression> {
    use ExactScalarOperation as Op;
    let bounded5 = || IntegerDomain::Bounded(interval(-5, 5));
    vec![
        expression(
            1001,
            BINARY,
            INT2,
            "integer",
            Op::IntegerArithmetic {
                operator: IntegerOperator::Add,
                domain: IntegerDomain::Mathematical,
            },
        ),
        expression(
            1002,
            UNARY,
            INT1,
            "integer",
            Op::IntegerArithmetic {
                operator: IntegerOperator::Negate,
                domain: IntegerDomain::Bounded(interval(-8, 7)),
            },
        ),
        expression(
            1011,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile: DivisionProfile::Truncating,
                domain: IntegerDomain::Mathematical,
            },
        ),
        expression(
            1012,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile: DivisionProfile::Floor,
                domain: IntegerDomain::Mathematical,
            },
        ),
        expression(
            1013,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile: DivisionProfile::Euclidean,
                domain: IntegerDomain::Mathematical,
            },
        ),
        expression(
            1014,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile: DivisionProfile::Truncating,
                domain: bounded5(),
            },
        ),
        expression(
            1021,
            BINARY,
            INT2,
            "integer",
            Op::IntegerModulo { domain: bounded5() },
        ),
        expression(
            1031,
            BINARY,
            RAT2,
            "rational",
            Op::RationalArithmetic {
                operator: RationalOperator::Add,
                domain: None,
            },
        ),
        expression(
            1032,
            BINARY,
            RAT2,
            "rational",
            Op::RationalArithmetic {
                operator: RationalOperator::Divide,
                domain: Some(
                    RationalDomain::new(interval(-10, 10), interval(1, 4)).expect("domain"),
                ),
            },
        ),
        expression(
            1033,
            BINARY,
            INT2,
            "rational",
            Op::RationalArithmetic {
                operator: RationalOperator::IntegerDivide,
                domain: None,
            },
        ),
        expression(
            1041,
            BINARY,
            INT2,
            "boolean",
            Op::Ordering {
                operator: OrderingOperator::Less,
                operands: OrderingOperandKind::Integer,
            },
        ),
        expression(
            1042,
            BINARY,
            DEC2,
            "boolean",
            Op::Ordering {
                operator: OrderingOperator::LessOrEqual,
                operands: OrderingOperandKind::Decimal,
            },
        ),
        expression(
            1043,
            BINARY,
            RAT2,
            "boolean",
            Op::Ordering {
                operator: OrderingOperator::Greater,
                operands: OrderingOperandKind::Rational,
            },
        ),
        expression(
            1051,
            BINARY,
            DEC2,
            "decimal",
            Op::DecimalArithmetic {
                operator: DecimalOperator::Add,
                target: decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven),
            },
        ),
        expression(
            1052,
            BINARY,
            DEC2,
            "decimal",
            Op::DecimalArithmetic {
                operator: DecimalOperator::Divide,
                target: decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero),
            },
        ),
        expression(
            1053,
            CONVERSION,
            DEC1,
            "decimal",
            Op::DecimalArithmetic {
                operator: DecimalOperator::Round,
                target: decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway),
            },
        ),
        expression(
            1061,
            BINARY,
            F32_2,
            "float32",
            Op::IeeeArithmetic {
                operator: IeeeArithmeticOperator::Add,
                width: IeeeWidth::Binary32,
                rounding: RoundingMode::NearestEven,
            },
        ),
        expression(
            1062,
            BINARY,
            F64_2,
            "float64",
            Op::IeeeArithmetic {
                operator: IeeeArithmeticOperator::Divide,
                width: IeeeWidth::Binary64,
                rounding: RoundingMode::TowardZero,
            },
        ),
        expression(
            1063,
            BINARY,
            F64_2,
            "boolean",
            Op::IeeeComparison {
                comparison: IeeeComparison::TotalOrder,
                width: IeeeWidth::Binary64,
            },
        ),
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
        ),
        expression(
            1071,
            CONVERSION,
            TEXT1,
            "text",
            Op::TextAdmission {
                text_type: TextType::new(0, 4, TextProfile::Nfc).expect("text type"),
            },
        ),
        expression(
            1072,
            BINARY,
            TEXT2,
            "boolean",
            Op::TextComparison {
                operator: ComparisonOperator::Less,
            },
        ),
        expression(
            1073,
            BINARY,
            ENUM2,
            "boolean",
            Op::EnumComparison {
                operator: ComparisonOperator::Less,
            },
        ),
        expression(
            1081,
            BINARY,
            QTY2,
            "unit",
            Op::QuantityArithmetic {
                operator: QuantityOperator::Add,
            },
        ),
        expression(
            1082,
            BINARY,
            QTY2,
            "unit",
            Op::QuantityArithmetic {
                operator: QuantityOperator::Multiply,
            },
        ),
        expression(
            1083,
            BINARY,
            QTY_INT,
            "unit",
            Op::QuantityArithmetic {
                operator: QuantityOperator::Power,
            },
        ),
        expression(
            1084,
            BINARY,
            QTY2,
            "boolean",
            Op::QuantityComparison {
                operator: ComparisonOperator::Less,
            },
        ),
        expression(
            1085,
            CONVERSION,
            QTY1,
            "rational",
            Op::QuantityConversion {
                target: QuantityTarget::Exact,
            },
        ),
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
        ),
    ]
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

/// The type node of a scalar result form.
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
    for expression in corpus() {
        let arguments = expression
            .operands
            .iter()
            .map(|form| reference(&operand(form)))
            .collect();
        builder.code(
            expression.code,
            "expression",
            expression.form,
            &result_type(expression.result),
            application(expression.operator, arguments),
        );
    }
    let boolean = key(T_BOOLEAN);
    for code in [DUPLICATED, WRONG_RESULT, WRONG_ARITY] {
        let arguments = vec![reference(&key(V_INTEGER)), reference(&key(V_INTEGER))];
        builder.code(
            code,
            "expression",
            "binary",
            &key(T_INTEGER),
            application("binary", arguments),
        );
    }
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
        .code(
            CALLS_FUNCTION,
            "expression",
            "binary",
            &key(T_INTEGER),
            application(
                "binary",
                vec![reference(&key(FUNCTION)), reference(&key(V_INTEGER))],
            ),
        )
        .code(
            WRONG_BODY,
            "expression",
            "binary",
            &key(T_INTEGER),
            application("unary", vec![reference(&key(V_INTEGER))]),
        )
        .code(
            WRONG_OPERAND,
            "expression",
            "binary",
            &key(T_INTEGER),
            application(
                "binary",
                vec![reference(&key(V_INTEGER)), reference(&key(V_DECIMAL))],
            ),
        );
    builder
}

fn integer_add() -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: IntegerDomain::Mathematical,
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
        V_INTEGER,
        DUPLICATED,
        DUPLICATED,
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
            domain: IntegerDomain::Mathematical,
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
