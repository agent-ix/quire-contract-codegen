//! Agreement support: direct owning kernel/QSV execution and generated
//! oracles.
//!
//! Adapted from the Contract Runtime agreement harness.
//! A vector body is written once. [`agree2!`] evaluates the direct call with
//! owning kernel/QSV APIs in scope and evaluates the generated
//! oracle with the runtime in scope. Every evaluation is run under the same
//! limits and then re-run with each admitted charge denied in turn. The two
//! `Debug` renderings, which include every value, typed refusal, incomplete
//! record, admitted charge and consumed counter, must be identical.
//!
//! This crate checks generated==direct owner execution only and claims no agreement with the QSL
//! value authority (FR-014-AC-47); that comparison for the quantity vectors is
//! owned by quire-integration IR-842.
//! QSL arch-lint T12-A confines this crate to `qsl_replay`'s public API.
//! The admitted source supplies nominal metadata; comparisons still execute
//! value-level operators directly, without replaying a QSL value witness.
//!
//! Named unit and enum fixtures come from the public QSL admitted facade.
//! Malformed graph controls separately use the public low-level QSV node boundary;
//! those arbitrary node labels make no claim to compiler provenance.

// Each helper is used by some vectors on one side only.
#![allow(dead_code, unused_imports)]

use std::collections::BTreeMap;

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
            (
                outcome,
                meter
                    .admitted_charges()
                    .iter()
                    .map(|charge| charge.point)
                    .collect(),
                consumed,
            )
        }

        /// One denied execution with its complete meter observation.
        #[derive(Debug, Eq, PartialEq)]
        pub struct DeniedRun<T> {
            /// The charge point whose occurrence was denied.
            pub point: ChargePoint,
            /// The one-based denied occurrence at that point.
            pub occurrence: u64,
            /// The original returned value or stop.
            pub outcome: T,
            /// Every admitted charge point in execution order.
            pub admitted_charges: Vec<ChargePoint>,
            /// Consumed counters in `LimitKind::ALL` order, including every kind.
            pub consumed: Vec<u64>,
        }

        /// Deny each admitted charge occurrence in turn, retaining the outcome,
        /// admitted charge order and every consumed counter of each denied run.
        pub fn denials<T>(
            limits: ScalarLimits,
            run: impl Fn(&mut Meter) -> T,
        ) -> Vec<DeniedRun<T>> {
            let mut meter = Meter::new(limits);
            let _ = run(&mut meter);
            let charges = meter
                .admitted_charges()
                .iter()
                .map(|charge| charge.point)
                .collect::<Vec<_>>();
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
                    DeniedRun {
                        point,
                        occurrence,
                        outcome,
                        admitted_charges: denied
                            .admitted_charges()
                            .iter()
                            .map(|charge| charge.point)
                            .collect(),
                        consumed: LimitKind::ALL
                            .iter()
                            .map(|kind| denied.consumed(*kind))
                            .collect(),
                    }
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
                self.keys[name].clone()
            }

            pub fn unit(&self, name: &str) -> IdentifiedUnit {
                self.named_units[name].clone()
            }

            pub fn resolve<'a>(&'a self, quantity: &'a Quantity) -> UnitQuantity<'a> {
                self.units
                    .resolve(quantity)
                    .expect("fixture quantity has its owner unit")
            }

            pub fn q(&self, value: Rational, name: &str) -> Quantity {
                Quantity::new(value, self.unit(name).id())
            }

            pub fn qi(&self, value: i64, name: &str) -> Quantity {
                self.q(Rational::from_integer(Integer::from(value)), name)
            }
        }

        pub fn fixture() -> Fixture {
            static FIXTURE: std::sync::OnceLock<Fixture> = std::sync::OnceLock::new();
            FIXTURE
                .get_or_init(|| named_fixture(&super::GraphSpec::tc187()))
                .clone()
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

pub const fn unit(
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

pub const fn root(name: &'static str, dimension: &'static str) -> UnitSpec {
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
                // Same declaration name and shape as `L`, different owner: a different node key.
                DimSpec {
                    name: "L_foreign",
                    owner: "other-model",
                    declaration: "Length",
                    terms: &[],
                },
            ],
            units: vec![
                root("m", "L"),
                root("m_foreign", "L_foreign"),
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

    /// TC-187 conversion dimensions, kept separate from the arithmetic fixture.
    pub fn conversions() -> Self {
        let mut spec = Self::tc187();
        for (name, terms) in [
            ("Area", &[("L", 2)][..]),
            ("Torque", &[("L", 2), ("M", 1), ("T", -2)][..]),
            ("Energy", &[("L", 2), ("M", 1), ("T", -2)][..]),
        ] {
            spec.dimensions.push(DimSpec {
                name,
                owner: "example-model",
                declaration: name,
                terms,
            });
        }
        spec.units.extend([
            root("m2", "Area"),
            unit("cm2", "Area", Some("m2"), (1, 10000), (0, 1)),
            root("N_m", "Torque"),
            root("J", "Energy"),
        ]);
        spec
    }
}

// ---- the runtime side --------------------------------------------------------

pub mod rt_side {
    use std::collections::BTreeMap;

    pub use quire_contract_runtime::scalar::{
        evaluate_boolean_short_circuit, ShortCircuitConnective,
    };
    pub use quire_exact::*;
    pub use quire_semantic_value::quantity::{
        compare_quantity, convert_quantity, evaluate_quantity_unit, Conversion, ConvertedValue,
        IdentifiedUnit, QuantityOperation, QuantityRefusal, QuantityTarget, QuantityUnit,
        UnitQuantity, UnitScope, UnitTable,
    };
    pub use quire_semantic_value::unit::{
        CompoundUnit, DimensionNode, NominalDeclaration, Unit, UnitGraph, UnitNode,
    };

    pub use quire_semantic_value::enumeration::{compare_enum, EnumDeclaration, EnumValue};
    pub use quire_semantic_value::semantic_node::{InvalidSemanticGraph, SemanticGraphCause};

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
        }
    }

    shared_helpers!();

    #[derive(Clone)]
    pub struct Fixture {
        pub units: UnitTable,
        keys: BTreeMap<&'static str, NodeKey>,
        named_units: BTreeMap<&'static str, IdentifiedUnit>,
    }

    pub struct LowLevelFixture {
        pub graph: UnitGraph,
    }

    /// Compile each original owner independently, then retain every returned unit
    /// and the complete owner tables. The foreign Length keeps its distinct owner.
    pub fn named_fixture(spec: &super::GraphSpec) -> Fixture {
        use std::fmt::Write;
        let mut units = UnitTable::default();
        let mut keys = BTreeMap::new();
        let mut named_units = BTreeMap::new();
        // `in` is a language keyword. The source declares `inch`; vector labels
        // and their exact 127/5000 scale remain unchanged.
        let source_unit = |name| if name == "in" { "inch" } else { name };
        for owner in ["example-model", "other-model"] {
            let dimensions = spec
                .dimensions
                .iter()
                .filter(|d| d.owner == owner)
                .collect::<Vec<_>>();
            let mut source = String::from("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n");
            for d in &dimensions {
                if d.terms.is_empty() {
                    writeln!(source, "dimension {};", d.declaration).unwrap();
                } else {
                    let terms = d
                        .terms
                        .iter()
                        .map(|(name, exponent)| {
                            let name = spec
                                .dimensions
                                .iter()
                                .find(|d| d.name == *name)
                                .unwrap()
                                .declaration;
                            format!("{name} ^ {exponent}")
                        })
                        .collect::<Vec<_>>()
                        .join(" * ");
                    writeln!(source, "dimension {} = {terms};", d.declaration).unwrap();
                }
            }
            let owned = spec
                .units
                .iter()
                .filter(|u| dimensions.iter().any(|d| d.name == u.dimension))
                .collect::<Vec<_>>();
            for u in &owned {
                let dimension = dimensions
                    .iter()
                    .find(|d| d.name == u.dimension)
                    .unwrap()
                    .declaration;
                write!(
                    source,
                    "unit {} : {dimension} = rational({}, {})",
                    source_unit(u.name),
                    u.scale.0,
                    u.scale.1
                )
                .unwrap();
                if let Some(target) = u.target {
                    write!(source, " * {}", source_unit(target)).unwrap();
                }
                if u.offset.0 != 0 {
                    write!(source, " + rational({}, {})", u.offset.0, u.offset.1).unwrap();
                }
                source.push_str(";\n");
            }
            source.push_str("function fixture using v(): Boolean pure { true }\n");
            let compiled = qsl_replay::compile_admitted(
                qsl_replay::CompileUnit {
                    source: qsl_replay::SourceIdentity::new("agent-ix", owner, "git", "1"),
                    package_name: owner.to_owned(),
                    path: "fixture.native",
                    bytes: source.as_bytes(),
                },
                [],
                &qsl_replay::DependencyInput::default(),
                qsl_replay::StageLimits::default(),
                qsl_replay::ReplayLimits::default(),
            )
            .unwrap_or_else(|error| panic!("named owner fixture failed: {error:?}\n{source}"));
            let context = compiled.context();
            for d in &dimensions {
                let admitted = context
                    .dimensions()
                    .find(|a| a.qualified_name().last().unwrap().as_str() == d.declaration)
                    .expect("owner dimension");
                keys.insert(d.name, admitted.node().clone());
            }
            for u in owned {
                let admitted = context
                    .units()
                    .find(|a| a.qualified_name().last().unwrap().as_str() == source_unit(u.name))
                    .expect("owner unit");
                let identified = IdentifiedUnit::new(admitted.quantity_unit().clone())
                    .expect("admitted unit identity");
                assert_eq!(identified.id(), admitted.id());
                keys.insert(u.name, admitted.node().clone());
                named_units.insert(u.name, identified);
            }
            units.append(context.unit_table().clone());
        }
        Fixture {
            units,
            keys,
            named_units,
        }
    }

    fn rational((numerator, denominator): (i64, i64)) -> Rational {
        Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap()
    }

    /// Low-level malformed-topology controls only: labels are arbitrary test bytes.
    pub fn admit_graph(spec: &super::GraphSpec) -> Result<LowLevelFixture, InvalidSemanticGraph> {
        let labels = spec
            .dimensions
            .iter()
            .map(|d| d.name)
            .chain(spec.units.iter().map(|u| u.name))
            .map(|name| {
                (
                    name,
                    NodeKey::new(
                        "agreement-fixture",
                        vec![PathSegment::Text(name.to_owned())],
                        0,
                    )
                    .unwrap(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let key = |name: &str| labels[name].clone();
        let dimensions = spec
            .dimensions
            .iter()
            .map(|d| {
                let mut terms = d
                    .terms
                    .iter()
                    .map(|(name, exponent)| (key(name), Integer::from(*exponent)))
                    .collect::<Vec<_>>();
                terms.sort_by(|(left, _), (right, _)| left.cmp(right));
                let node = DimensionNode::checked(terms, NominalDeclaration::default())
                    .map_err(|cause| InvalidSemanticGraph { cause })?;
                Ok((key(d.name), node))
            })
            .collect::<Result<BTreeMap<_, _>, InvalidSemanticGraph>>()?;
        let units = spec
            .units
            .iter()
            .map(|u| {
                let node = UnitNode::checked(
                    key(u.dimension),
                    u.target.map(key),
                    rational(u.scale),
                    rational(u.offset),
                    NominalDeclaration::default(),
                )
                .map_err(|cause| InvalidSemanticGraph { cause })?;
                Ok((key(u.name), node))
            })
            .collect::<Result<BTreeMap<_, _>, InvalidSemanticGraph>>()?;
        let graph = UnitGraph::from_checked_nodes(&dimensions, &units)?;
        Ok(LowLevelFixture { graph })
    }

    /// A genuine compiler-admitted enum, retaining its member identities.
    pub struct Enum {
        declaration: EnumDeclaration,
        cases: BTreeMap<String, NodeKey>,
    }

    pub fn enum_declaration(
        declaration: &str,
        ordered: bool,
        members: &[&str],
    ) -> Result<Enum, InvalidSemanticGraph> {
        use std::sync::{Mutex, OnceLock};
        static ENUMS: OnceLock<Mutex<BTreeMap<String, qsl_replay::AdmittedEnum>>> = OnceLock::new();
        let source = format!(
            "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n{}enum {declaration} {{ {} }}\n",
            if ordered { "ordered " } else { "" },
            members.join(", ")
        );
        let mut enums = ENUMS.get_or_init(Mutex::default).lock().unwrap();
        let admitted = enums.entry(source.clone()).or_insert_with(|| {
            let compiled = qsl_replay::compile_admitted(
                qsl_replay::CompileUnit {
                    source: qsl_replay::SourceIdentity::new("agent-ix", "example-model", "git", "1"),
                    package_name: "example-model".to_owned(),
                    path: "enum.native",
                    bytes: source.as_bytes(),
                },
                [],
                &qsl_replay::DependencyInput::default(),
                qsl_replay::StageLimits::default(),
                qsl_replay::ReplayLimits::default(),
            )
            .expect("genuine enum source");
            let admitted = compiled
                .context()
                .enums()
                .next()
                .expect("owner enum")
                .clone();
            admitted
        });
        let declaration = EnumDeclaration::new(
            admitted.key().clone(),
            admitted.is_ordered(),
            admitted
                .cases()
                .iter()
                .map(|case| case.case().as_str().to_owned())
                .collect(),
        )?;
        let cases = admitted
            .cases()
            .iter()
            .map(|case| (case.case().as_str().to_owned(), case.key().clone()))
            .collect();
        Ok(Enum { declaration, cases })
    }

    impl Enum {
        pub fn value(&self, case: &str) -> Result<EnumValue, InvalidSemanticGraph> {
            self.declaration.member(case, self.cases[case].clone())
        }
    }
}
