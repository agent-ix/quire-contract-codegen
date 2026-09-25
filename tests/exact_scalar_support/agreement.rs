//! Agreement support: direct Contract Runtime execution and generated
//! oracles.
//!
//! Adapted from Contract Runtime 4e33052
//! `conformance/qsl-agreement/tests/support/mod.rs`.
//! A vector body is written once. [`agree2!`] evaluates the direct call with
//! `quire_contract_runtime::exact` in scope and evaluates the generated
//! oracle with the runtime in scope. Every evaluation is run under the same
//! limits and then re-run with each admitted charge denied in turn. The two
//! `Debug` renderings, which include every value, typed refusal, incomplete
//! record, admitted charge and consumed counter, must be identical.
//!
//! IR-254: this file used to run a third, authority leg through
//! `quire_spec_language::value`, comparing the direct runtime call against
//! QSL's own value-level implementation of the same operators before
//! comparing either to the generated oracle (`agree3!`, since removed). That
//! leg was already red (AGE-1989) when this pin bump landed, and the QSL
//! revision this bumps to withdraws `quire_spec_language::value` entirely:
//! QSL arch-lint T12-A confines this crate to `qsl_replay`'s public API, one
//! source-recompiling proof-witness replay executor (`qsl_replay::replay`,
//! taking a `ReplayRequestWire` built from digest-addressed compiled QSL
//! source and a witness arm). None of this file's vectors are shaped as
//! compiled source plus a witness -- they call value-level operators
//! directly (`Meter`, `Integer`, `IeeeValue`, unit conversion, division
//! profiles, ...) -- so none can be re-expressed through that facade without
//! building a new source-level test harness from nothing, which is out of
//! scope for a pin bump. The authority leg is deleted outright rather than
//! ported; every vector below still runs the direct-runtime-vs-generated-
//! oracle comparison it always did.
//!
//! Node keys in this file no longer need to be QSL's own honest
//! preimage-hash values, because nothing here compares them to QSL's
//! computation anymore: [`GraphSpec::keys`] only has to be internally
//! self-consistent within one process (the direct call and the generated
//! oracle call always share one constructed [`Fixture`]), so it now hashes
//! its own JSON directly with `sha2` instead of calling into QSL.

// Each helper is used by some vectors on one side only.
#![allow(dead_code, unused_imports)]

use std::collections::BTreeMap;

use quire_contract_runtime::exact::NODE_KEY_DOMAIN;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// A stable, process-internal digest for a fixture node's JSON. Not QSL's
/// preimage hash (see the module doc): nothing here compares against QSL's
/// computation anymore, so any deterministic function that gives every
/// distinct fixture node a distinct key is sufficient.
fn preimage_digest(document: &Value) -> String {
    format!("{:x}", Sha256::digest(document.to_string().as_bytes()))
}

/// Direct runtime and generated oracle agree, including every charge and every
/// single-charge denial.
macro_rules! agree2 {
    (
        limits: $limits:expr,
        setup: { $($setup:tt)* },
        direct: |$m:ident| $direct:expr,
        generated: |$g:ident| $generated:expr $(,)?
    ) => {{
        #[allow(unused_imports)]
        use self::support::rt_side::*;
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

        /// The `[-1000, 1000]` integer domain the corpus IR bounds integers to.
        pub fn wide() -> IntegerDomain {
            IntegerDomain::Bounded(IntegerInterval::new(int(-1000), int(1000)).unwrap())
        }

        /// The corpus rational range: numerators `[-1000, 1000]`, denominators `[1, 1000]`.
        pub fn wide_rational() -> RationalDomain {
            RationalDomain::new(
                IntegerInterval::new(int(-1000), int(1000)).unwrap(),
                IntegerInterval::new(int(1), int(1000)).unwrap(),
            )
            .unwrap()
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
                    let mut denied = Meter::new(limits).with_injected_denial(InjectedDenial {
                        point,
                        occurrence: to_occurrence(occurrence),
                    });
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
                _ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
            admit_graph(&super::GraphSpec::tc187()).unwrap()
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
        json!({"domain": NODE_KEY_DOMAIN, "digest": key})
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

    /// Keys, self-consistently hashed within this process (see the module
    /// doc: nothing compares these to QSL's own computation anymore).
    pub fn keys(&self) -> BTreeMap<&'static str, String> {
        let mut keys = BTreeMap::new();
        for dimension in &self.dimensions {
            let digest = preimage_digest(&Self::dimension_json(dimension, &keys));
            keys.insert(dimension.name, digest);
        }
        for unit in &self.units {
            let digest = preimage_digest(&Self::unit_json(unit, &keys));
            assert!(
                keys.insert(unit.name, digest).is_none(),
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
        "declaration_node_id": {"domain": NODE_KEY_DOMAIN, "digest": declaration_key},
        "case": case,
    })
}

/// Enum declaration key (see the module doc: self-consistent within this
/// process, not QSL's own computation).
pub fn enum_declaration_key(declaration: &str, ordered: bool, members: &[&str]) -> String {
    preimage_digest(&enum_declaration_json(declaration, ordered, members))
}

pub fn enum_member_key(declaration_key: &str, case: &str) -> String {
    preimage_digest(&enum_member_json(declaration_key, case))
}

// ---- the runtime side --------------------------------------------------------

pub mod rt_side {
    use std::collections::BTreeMap;

    pub use quire_contract_runtime::exact::*;

    /// The runtime's `InjectedDenial::occurrence` is `NonZeroU64`
    /// (`agent-ix/quire-contract-runtime#20`). Every caller in this crate
    /// passes a literal or a derived count that is always at least one, so
    /// this never panics.
    pub fn to_occurrence(n: u64) -> std::num::NonZeroU64 {
        std::num::NonZeroU64::new(n).unwrap()
    }

    /// `evaluate_integer_arithmetic`'s bound is `Option<&IntegerInterval>`, the same two states
    /// `IntegerDomain` already carried before the runtime dropped the wrapper
    /// (`Mathematical` -> `None`, `Bounded(interval)` -> `Some(&interval)`).
    pub fn as_bound(domain: &IntegerDomain) -> Option<&IntegerInterval> {
        match domain {
            IntegerDomain::Mathematical => None,
            IntegerDomain::Bounded(interval) => Some(interval),
            &_ => unreachable!("IntegerDomain gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
        }
    }

    shared_helpers!();

    pub struct Fixture {
        pub graph: UnitGraph,
        keys: BTreeMap<&'static str, NodeKey>,
    }

    fn rational((numerator, denominator): (i64, i64)) -> Rational {
        Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
    }

    pub fn admit_graph(spec: &super::GraphSpec) -> Result<Fixture, InvalidSemanticGraph> {
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

    /// A compiler-admitted enum declaration under its self-consistently
    /// hashed key (see the module doc).
    pub struct Enum(EnumDeclaration);

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        let key = super::enum_declaration_key(declaration, ordered, members);
        EnumDeclaration::new(NodeKey::from_hex(&key).unwrap(), ordered, members).map(Enum)
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            let declaration = self.0.key().to_string();
            let key = super::enum_member_key(&declaration, case);
            self.0.member(case, NodeKey::from_hex(&key).unwrap())
        }
    }
}
