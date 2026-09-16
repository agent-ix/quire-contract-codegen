//! Three-way agreement support: the QSL value authority, direct Contract
//! Runtime execution, and generated oracles.
//!
//! Adapted from Contract Runtime a04bd47
//! `conformance/qsl-agreement/tests/support/mod.rs` (same QSL pin d9d5273).
//! A vector body is written once. [`agree3!`] evaluates the direct call with
//! `quire_spec_language::value` in scope and again with
//! `quire_contract_runtime::exact` in scope, and evaluates the generated
//! oracle with the runtime in scope. Every evaluation is run under the same
//! limits and then re-run with each admitted charge denied in turn. The three
//! `Debug` renderings, which include every value, typed refusal, incomplete
//! record, admitted charge and consumed counter, must be identical.
//!
//! Compiler-owned work (definition-lock admission, node-key hashing, owner
//! selection) is done once, by the authority: node keys come from the
//! authority's preimage hashing and both sides consume the same keys.

// Each helper is used by some vectors on one side only, and the side modules
// glob-import both value APIs for the vector bodies.
#![allow(dead_code, unused_imports)]

use std::collections::BTreeMap;

use quire_spec_language::value as authority;
use serde_json::{json, Value};

/// Authority, direct runtime and generated oracle agree, including every
/// charge and every single-charge denial.
macro_rules! agree3 {
    (
        limits: $limits:expr,
        setup: { $($setup:tt)* },
        direct: |$m:ident| $direct:expr,
        generated: |$g:ident| $generated:expr $(,)?
    ) => {{
        let authority = {
            #[allow(unused_imports)]
            use crate::support::qsl_side::*;
            $($setup)*
            let limits = $limits;
            format!(
                "{:?}",
                (metered(limits, |$m: &mut Meter| $direct), denials(limits, |$m: &mut Meter| $direct))
            )
        };
        let (runtime, generated) = {
            #[allow(unused_imports)]
            use crate::support::rt_side::*;
            $($setup)*
            let limits = $limits;
            (
                format!(
                    "{:?}",
                    (metered(limits, |$m: &mut Meter| $direct), denials(limits, |$m: &mut Meter| $direct))
                ),
                format!(
                    "{:?}",
                    (
                        metered(limits, |$g: &mut Meter| $generated),
                        denials(limits, |$g: &mut Meter| $generated),
                    )
                ),
            )
        };
        assert_eq!(runtime, authority, "direct runtime disagrees with the QSL authority");
        assert_eq!(generated, runtime, "generated oracle disagrees with direct runtime");
    }};
}

/// Direct runtime and generated oracle agree, including every charge and every
/// single-charge denial, for operators the pinned authority does not expose.
macro_rules! agree2 {
    (
        limits: $limits:expr,
        setup: { $($setup:tt)* },
        direct: |$m:ident| $direct:expr,
        generated: |$g:ident| $generated:expr $(,)?
    ) => {{
        #[allow(unused_imports)]
        use crate::support::rt_side::*;
        $($setup)*
        let limits = $limits;
        let runtime = format!(
            "{:?}",
            (metered(limits, |$m: &mut Meter| $direct), denials(limits, |$m: &mut Meter| $direct))
        );
        let generated = format!(
            "{:?}",
            (metered(limits, |$g: &mut Meter| $generated), denials(limits, |$g: &mut Meter| $generated))
        );
        assert_eq!(generated, runtime, "generated oracle disagrees with direct runtime");
    }};
}

/// Helpers whose source is identical on both sides; only the types differ.
macro_rules! shared_helpers {
    () => {
        pub const UNLIMITED: ScalarLimits = ScalarLimits {
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

        /// `ScalarLimitsV1` in field order.
        pub fn limits(v: [u64; 10]) -> ScalarLimits {
            ScalarLimits {
                integer_bits: v[0],
                decimal_digits: v[1],
                scale_expansion: v[2],
                text_input_bytes: v[3],
                text_scalars: v[4],
                normalized_scalars: v[5],
                unit_edges: v[6],
                value_occurrences: v[7],
                work_units: v[8],
                result_units: v[9],
            }
        }

        /// TC-187 `L(i,d,e,o,w,r)` from one array.
        pub fn unit_tuple(v: [u64; 6]) -> ScalarLimits {
            limits([v[0], v[1], 0, 0, 0, 0, v[2], v[3], v[4], v[5]])
        }

        /// TC-193 `I(i,o,w,r)`.
        pub fn ieee_limits(i: u64, o: u64, w: u64, r: u64) -> ScalarLimits {
            limits([i, 0, 0, 0, 0, 0, 0, o, w, r])
        }

        pub fn int(value: i128) -> Integer {
            Integer::from(value)
        }

        pub fn big(spelling: &str) -> Integer {
            spelling.parse().unwrap()
        }

        pub fn dec(coefficient: i64, scale: u32) -> Decimal {
            Decimal::new(Integer::from(coefficient), scale)
        }

        pub fn ratio(numerator: i64, denominator: i64) -> Rational {
            Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
        }

        pub fn decimal_type(
            lower: i64,
            upper: i64,
            min: u64,
            max: u64,
            mode: RoundingMode,
        ) -> DecimalType {
            DecimalType::new(Integer::from(lower), Integer::from(upper), min, max, mode).unwrap()
        }

        pub fn text_type(min: u64, max: u64, profile: TextProfile) -> TextType {
            TextType::new(min, max, profile).unwrap()
        }

        pub fn payload(text: &str) -> TextPayload {
            TextPayload::from_utf8(text.as_bytes()).unwrap()
        }

        pub fn text(text: &str, profile: TextProfile) -> Text {
            admit_text(
                &payload(text),
                &text_type(0, 64, profile),
                &mut Meter::new(UNLIMITED),
            )
            .completed()
            .unwrap()
        }

        /// An outcome with every admitted charge and every consumed counter.
        pub fn metered<T>(
            limits: ScalarLimits,
            run: impl FnOnce(&mut Meter) -> T,
        ) -> (T, Vec<ChargePoint>, Vec<u64>) {
            let mut meter = Meter::new(limits);
            let outcome = run(&mut meter);
            let consumed = LimitKind::ALL
                .iter()
                .map(|kind| meter.consumed(*kind))
                .collect();
            (outcome, meter.admitted_charges().to_vec(), consumed)
        }

        /// Deny each admitted charge occurrence in turn:
        /// `(point, occurrence, outcome, result units consumed)`.
        pub fn denials<T>(
            limits: ScalarLimits,
            run: impl Fn(&mut Meter) -> T,
        ) -> Vec<(ChargePoint, u64, T, u64)> {
            let mut meter = Meter::new(limits);
            let _ = run(&mut meter);
            let charges = meter.admitted_charges().to_vec();
            let mut seen: Vec<ChargePoint> = Vec::new();
            charges
                .into_iter()
                .map(|point| {
                    seen.push(point);
                    let occurrence = seen.iter().filter(|p| **p == point).count() as u64;
                    let mut denied = Meter::new(limits)
                        .with_injected_denial(InjectedDenial { point, occurrence });
                    let outcome = run(&mut denied);
                    (
                        point,
                        occurrence,
                        outcome,
                        denied.consumed(LimitKind::ResultUnits),
                    )
                })
                .collect()
        }

        pub fn f32v(bits: u32) -> IeeeValue {
            IeeeValue::binary32(bits)
        }

        pub fn f64v(bits: u64) -> IeeeValue {
            IeeeValue::binary64(bits)
        }

        /// A pattern of `IeeeWidth::ALL[width]`.
        pub fn ieee(width: usize, bits: u64) -> IeeeValue {
            match IeeeWidth::ALL[width] {
                IeeeWidth::Binary32 => IeeeValue::binary32(u32::try_from(bits).unwrap()),
                IeeeWidth::Binary64 => IeeeValue::binary64(bits),
            }
        }

        impl Fixture {
            pub fn key(&self, name: &str) -> NodeKey {
                self.keys[name]
            }

            pub fn unit(&self, name: &str) -> QuantityUnit {
                QuantityUnit::Declared(Box::new(self.graph.unit(self.key(name)).unwrap().clone()))
            }

            pub fn q(&self, value: Rational, name: &str) -> Quantity {
                Quantity::new(value, self.unit(name))
            }

            pub fn qi(&self, value: i64, name: &str) -> Quantity {
                self.q(Rational::from_integer(Integer::from(value)), name)
            }
        }

        pub fn fixture() -> Fixture {
            admit_graph(&crate::support::GraphSpec::tc187()).unwrap()
        }
    };
}

// ---- side-neutral graph and enum descriptions -----------------------------

/// A dimension node: `terms` empty for a base dimension.
#[derive(Clone, Copy, Debug)]
pub struct DimSpec {
    pub name: &'static str,
    pub owner: &'static str,
    pub declaration: &'static str,
    pub terms: &'static [(&'static str, i64)],
}

/// A unit node.
#[derive(Clone, Copy, Debug)]
pub struct UnitSpec {
    pub name: &'static str,
    pub dimension: &'static str,
    pub target: Option<&'static str>,
    pub scale: (i64, i64),
    pub offset: (i64, i64),
}

/// A closed node set.
#[derive(Clone, Debug)]
pub struct GraphSpec {
    pub dimensions: Vec<DimSpec>,
    pub units: Vec<UnitSpec>,
}

const fn base(name: &'static str, declaration: &'static str) -> DimSpec {
    DimSpec {
        name,
        owner: "example-model",
        declaration,
        terms: &[],
    }
}

const fn unit(
    name: &'static str,
    dimension: &'static str,
    target: Option<&'static str>,
    scale: (i64, i64),
    offset: (i64, i64),
) -> UnitSpec {
    UnitSpec {
        name,
        dimension,
        target,
        scale,
        offset,
    }
}

const fn root(name: &'static str, dimension: &'static str) -> UnitSpec {
    unit(name, dimension, None, (1, 1), (0, 1))
}

impl GraphSpec {
    /// The QSpec TC-187 unit fixture.
    pub fn tc187() -> Self {
        Self {
            dimensions: vec![
                base("L", "Length"),
                base("T", "Time"),
                base("Theta", "Temperature"),
                base("M", "Mass"),
            ],
            units: vec![
                root("m", "L"),
                root("s", "T"),
                root("K", "Theta"),
                unit("cm", "L", Some("m"), (1, 100), (0, 1)),
                unit("in", "L", Some("m"), (127, 5000), (0, 1)),
                unit("degC", "Theta", Some("K"), (1, 1), (5463, 20)),
                unit("degF", "Theta", Some("K"), (5, 9), (45967, 180)),
                root("kg", "M"),
                unit("u1", "Theta", Some("K"), (1, 1), (10, 1)),
                unit("u2", "Theta", Some("u1"), (1, 1), (-10, 1)),
                unit("u3", "Theta", Some("degC"), (1, 1), (0, 1)),
                unit("rev", "L", Some("m"), (-1, 1), (0, 1)),
                unit("m_alias", "L", Some("m"), (1, 1), (0, 1)),
            ],
        }
    }

    fn node_id(key: &str) -> Value {
        json!({"domain": authority::NODE_KEY_DOMAIN, "digest": key})
    }

    fn owner(identity: &str) -> Value {
        json!({"kind": "definition", "authority": "agent-ix", "identity": identity})
    }

    pub fn dimension_json(dimension: &DimSpec, keys: &BTreeMap<&'static str, String>) -> Value {
        let mut terms: Vec<(&String, i64)> = dimension
            .terms
            .iter()
            .map(|(name, exponent)| (&keys[name], *exponent))
            .collect();
        terms.sort();
        let terms: Vec<Value> = terms
            .into_iter()
            .map(|(key, exponent)| {
                json!({"dimension_node_id": Self::node_id(key), "exponent": exponent.to_string()})
            })
            .collect();
        json!({
            "version": "quire.dimension-node/v1",
            "owner": Self::owner(dimension.owner),
            "qualified_declaration": ["Example", dimension.declaration],
            "terms": terms,
        })
    }

    pub fn unit_json(unit: &UnitSpec, keys: &BTreeMap<&'static str, String>) -> Value {
        let rational =
            |(n, d): (i64, i64)| json!({"numerator": n.to_string(), "denominator": d.to_string()});
        json!({
            "version": "quire.unit-node/v1",
            "owner": Self::owner("example-model"),
            "qualified_declaration": ["Example", unit.name],
            "dimension_node_id": Self::node_id(&keys[unit.dimension]),
            "target_unit_node_id": unit.target.map_or(Value::Null, |t| Self::node_id(&keys[t])),
            "scale": rational(unit.scale),
            "offset": rational(unit.offset),
        })
    }

    /// Honest keys, computed by the authority in declaration order.
    pub fn keys(&self) -> BTreeMap<&'static str, String> {
        let mut keys = BTreeMap::new();
        for dimension in &self.dimensions {
            let preimage =
                authority::DimensionPreimage::from_json(Self::dimension_json(dimension, &keys))
                    .unwrap();
            keys.insert(dimension.name, preimage.node_key().unwrap().to_string());
        }
        for unit in &self.units {
            let preimage =
                authority::UnitPreimage::from_json(Self::unit_json(unit, &keys)).unwrap();
            assert!(
                keys.insert(unit.name, preimage.node_key().unwrap().to_string())
                    .is_none(),
                "duplicate fixture name {}",
                unit.name
            );
        }
        keys
    }
}

pub fn enum_declaration_json(declaration: &str, ordered: bool, members: &[&str]) -> Value {
    json!({
        "version": "quire.enum-declaration-node/v1",
        "owner": {"kind": "definition", "authority": "agent-ix", "identity": "example-model"},
        "qualified_declaration": ["Example", declaration],
        "ordered": ordered,
        "members": members,
    })
}

pub fn enum_member_json(declaration_key: &str, case: &str) -> Value {
    json!({
        "version": "quire.enum-member-node/v1",
        "declaration_node_id": {"domain": authority::NODE_KEY_DOMAIN, "digest": declaration_key},
        "case": case,
    })
}

/// Authority-computed enum declaration key.
pub fn enum_declaration_key(declaration: &str, ordered: bool, members: &[&str]) -> String {
    let json = enum_declaration_json(declaration, ordered, members);
    let preimage = authority::EnumDeclarationPreimage::from_json(json).unwrap();
    preimage.node_key().unwrap().to_string()
}

pub fn enum_member_key(declaration_key: &str, case: &str) -> String {
    let preimage =
        authority::EnumMemberPreimage::from_json(enum_member_json(declaration_key, case)).unwrap();
    preimage.node_key().unwrap().to_string()
}

// ---- the authority side ------------------------------------------------------

pub mod qsl_side {
    use std::collections::BTreeMap;
    use std::sync::OnceLock;

    use quire_spec_language::value as authority;
    pub use quire_spec_language::value::*;

    shared_helpers!();

    fn lock() -> &'static DefinitionLock {
        DefinitionLock::pinned().unwrap()
    }

    /// The pinned lock's admitted division law, then the authority operator.
    pub fn divide(
        profile: DivisionProfile,
        a: &Integer,
        b: &Integer,
        domain: &IntegerDomain,
        meter: &mut Meter,
    ) -> Outcome<QuotientRemainder> {
        let role = match profile {
            DivisionProfile::Truncating => CatalogRole::IntegerDivisionTruncating,
            DivisionProfile::Floor => CatalogRole::IntegerDivisionFloor,
            DivisionProfile::Euclidean => CatalogRole::IntegerDivisionEuclidean,
        };
        let reference = lock().entry(role).unwrap().definition.clone();
        let admitted = lock().admit_integer_division(&[reference], None).unwrap();
        authority::divide(&admitted, a, b, domain, meter)
    }

    fn profile() -> &'static AdmittedIeeeProfile {
        static PROFILE: OnceLock<AdmittedIeeeProfile> = OnceLock::new();
        PROFILE.get_or_init(|| {
            let reference = lock()
                .entry(CatalogRole::IeeeProfile)
                .unwrap()
                .definition
                .clone();
            lock().admit_ieee_profile(&[reference], &[]).unwrap()
        })
    }

    pub fn evaluate_ieee<'a, O: Into<IeeeOperand<'a>>>(
        operation: IeeeOperation<O>,
        rounding: RoundingMode,
        meter: &mut Meter,
    ) -> Result<Outcome<IeeeResult>, IllTyped> {
        authority::evaluate_ieee(profile(), operation, rounding, meter)
    }

    pub fn compare_ieee<'a>(
        comparison: IeeeComparison,
        left: impl Into<IeeeOperand<'a>>,
        right: impl Into<IeeeOperand<'a>>,
        meter: &mut Meter,
    ) -> Result<Outcome<bool>, IllTyped> {
        authority::compare_ieee(profile(), comparison, left, right, meter)
    }

    pub fn convert_ieee_width(
        value: IeeeValue,
        target: IeeeWidth,
        rounding: RoundingMode,
        meter: &mut Meter,
    ) -> Outcome<IeeeResult> {
        authority::convert_ieee_width(profile(), value, target, rounding, meter)
    }

    pub struct Fixture {
        pub graph: UnitGraph,
        keys: BTreeMap<&'static str, NodeKey>,
    }

    fn owners() -> OwnerSelection {
        let owner = |identity: &str| {
            NodeOwner::Definition(OwnerSubject {
                authority: "agent-ix".into(),
                identity: identity.into(),
            })
        };
        OwnerSelection::new([owner("example-model")])
    }

    pub fn admit_graph(spec: &crate::support::GraphSpec) -> Result<Fixture, InvalidSemanticGraph> {
        use crate::support::GraphSpec;
        let hex = spec.keys();
        let key = |digest: &str| NodeKey::from_hex(digest).unwrap();
        let dimensions: Vec<_> = spec
            .dimensions
            .iter()
            .map(|d| {
                let preimage =
                    DimensionPreimage::from_json(GraphSpec::dimension_json(d, &hex)).unwrap();
                (preimage, key(&hex[d.name]))
            })
            .collect();
        let units: Vec<_> = spec
            .units
            .iter()
            .map(|u| {
                let preimage = UnitPreimage::from_json(GraphSpec::unit_json(u, &hex)).unwrap();
                (preimage, key(&hex[u.name]))
            })
            .collect();
        let graph = UnitGraph::admit(dimensions, units, &owners())?;
        let keys = hex
            .iter()
            .map(|(name, digest)| (*name, key(digest)))
            .collect();
        Ok(Fixture { graph, keys })
    }

    /// An admitted enum declaration under its honest key.
    pub struct Enum(EnumDeclaration);

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        let key = crate::support::enum_declaration_key(declaration, ordered, members);
        let json = crate::support::enum_declaration_json(declaration, ordered, members);
        let preimage = EnumDeclarationPreimage::from_json(json)?;
        EnumDeclaration::admit(preimage, NodeKey::from_hex(&key).unwrap(), &owners()).map(Enum)
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            let declaration = self.0.key().to_string();
            let key = crate::support::enum_member_key(&declaration, case);
            let preimage = EnumMemberPreimage::from_json(crate::support::enum_member_json(
                &declaration,
                case,
            ))?;
            self.0
                .admit_member(&preimage, NodeKey::from_hex(&key).unwrap())
        }
    }
}

// ---- the runtime side --------------------------------------------------------

pub mod rt_side {
    use std::collections::BTreeMap;

    pub use quire_contract_runtime::exact::*;

    shared_helpers!();

    pub struct Fixture {
        pub graph: UnitGraph,
        keys: BTreeMap<&'static str, NodeKey>,
    }

    fn rational((numerator, denominator): (i64, i64)) -> Rational {
        Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
    }

    pub fn admit_graph(spec: &crate::support::GraphSpec) -> Result<Fixture, InvalidSemanticGraph> {
        let hex = spec.keys();
        let key = |digest: &str| NodeKey::from_hex(digest).unwrap();
        let dimensions: Vec<_> = spec
            .dimensions
            .iter()
            .map(|d| {
                let mut terms: Vec<(NodeKey, Integer)> = d
                    .terms
                    .iter()
                    .map(|(name, exponent)| (key(&hex[name]), Integer::from(*exponent)))
                    .collect();
                terms.sort_by_key(|(term, _)| *term);
                (key(&hex[d.name]), terms)
            })
            .collect();
        let units: Vec<_> = spec
            .units
            .iter()
            .map(|u| {
                let declaration = UnitDeclaration {
                    dimension: key(&hex[u.dimension]),
                    target: u.target.map(|target| key(&hex[target])),
                    scale: rational(u.scale),
                    offset: rational(u.offset),
                };
                (key(&hex[u.name]), declaration)
            })
            .collect();
        let graph = UnitGraph::admit(dimensions, units)?;
        let keys = hex
            .iter()
            .map(|(name, digest)| (*name, key(digest)))
            .collect();
        Ok(Fixture { graph, keys })
    }

    /// A compiler-admitted enum declaration under its authority-computed key.
    pub struct Enum(EnumDeclaration);

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        let key = crate::support::enum_declaration_key(declaration, ordered, members);
        EnumDeclaration::new(NodeKey::from_hex(&key).unwrap(), ordered, members).map(Enum)
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            let declaration = self.0.key().to_string();
            let key = crate::support::enum_member_key(&declaration, case);
            self.0.member(case, NodeKey::from_hex(&key).unwrap())
        }
    }
}
