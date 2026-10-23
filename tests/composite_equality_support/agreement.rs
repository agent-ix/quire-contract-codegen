//! Agreement support for composite equality (FR-018-AC-2, AC-8, AC-9,
//! AC-11): direct Contract Runtime execution and generated oracles.
//!
//! Each original vector is run against an independent direct QSV environment and the
//! actual emitted prepared oracle. The fixture recompiles its retained original source
//! through the public replay facade solely to obtain genuine identities and complete enum
//! membership; it does not claim a third QSL value-oracle or proof leg. Outcomes, ordered
//! charges, every counter and every single-charge denial must agree.

#![allow(dead_code, unused_imports)]

/// The exact source case used by this scratch run, admitted independently through the public root.
pub fn context() -> &'static qsl_replay::AdmittedContext {
    static ORIGINAL: std::sync::OnceLock<qsl_replay::AdmittedCompilation> = std::sync::OnceLock::new();
    ORIGINAL.get_or_init(|| {
        let case = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/case-mode"))
            .expect("host retained original source case");
        crate::fixture::compile(&case)
    }).context()
}

/// Direct runtime and generated oracle agree, including every charge and
/// every single-charge denial.
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
            (
                metered(limits, |$g: &mut Meter| $generated),
                denials(limits, |$g: &mut Meter| $generated),
            )
        );
        assert_eq!(generated, runtime, "generated oracle disagrees with direct runtime");
    }};
}

/// Helpers whose source is identical on both sides; only the imported types
/// differ.
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
                meter.admitted_charges().iter().map(|charge| charge.point).collect(),
                consumed,
            )
        }

        /// Deny each admitted charge occurrence in turn, discovered
        /// dynamically from a full run: `(point, occurrence, outcome, every
        /// counter of the run stopped immediately before that occurrence —
        /// the denied charge is never applied, so this *is* that run's
        /// final state)`. All ten `LimitKind`s are carried, not only
        /// `ResultUnits`, so `agree2!`'s cross-leg `Debug` comparison checks
        /// every counter that the direct runtime call and the generated
        /// oracle each drive into a denied run, for every admitted charge
        /// point in turn, not just one of them.
        pub fn denials(
            limits: ScalarLimits,
            run: impl Fn(&mut Meter) -> Outcome<bool>,
        ) -> Vec<(ChargePoint, u64, Outcome<bool>, Vec<u64>)> {
            let mut meter = Meter::new(limits);
            let _ = run(&mut meter);
            let charges = meter.admitted_charges().iter().map(|charge| charge.point).collect::<Vec<_>>();
            let mut seen: Vec<ChargePoint> = Vec::new();
            charges
                .into_iter()
                .map(|point| {
                    seen.push(point);
                    let occurrence = seen.iter().filter(|p| **p == point).count() as u64;
                    let mut denied = Meter::new(limits).with_injected_denial(InjectedDenial {
                        point,
                        // `InjectedDenial::occurrence` is a `NonZeroU64`
                        // (deliberately, so a malformed 0-based request
                        // cannot compile); `occurrence` here is always at
                        // least 1 since `seen.push(point)` above runs before
                        // this count.
                        occurrence: core::num::NonZeroU64::new(occurrence).unwrap(),
                    });
                    let outcome = run(&mut denied);
                    let counters: Vec<u64> = LimitKind::ALL
                        .iter()
                        .map(|kind| denied.consumed(*kind))
                        .collect();
                    (point, occurrence, outcome, counters)
                })
                .collect()
        }

        pub fn node_key(code: u32) -> NodeKey {
            let name = match code {
                super::super::package::R_POINT => "Point",
                super::super::package::TUP_PAIR => "Pair",
                super::super::package::R_SELF => "SelfRecord",
                super::super::package::R_PAIR_OF_POINTS => "PairOfPoints",
                other => panic!("no declared agreement type for code {other}"),
            };
            super::context().composites().find(|declaration|
                declaration.qualified_name().iter().map(|part| part.as_str()).eq([name])
            ).expect("the selected original source declares this exact type").key()
        }

        pub fn composite_type(code: u32) -> ValueType {
            ValueType::Composite(node_key(code))
        }

        /// The corpus's admitted Integer range, also used by option and sequence elements.
        pub fn corpus_integer_type() -> ValueType {
            ValueType::Int(
                IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64)).unwrap(),
            )
        }

        /// `Rational[num_lo, num_hi; den_lo, den_hi]` -- codegen#83's
        /// `admits_equality_conversion` `Rational -> *` rows.
        pub fn rational_type(num_lo: i64, num_hi: i64, den_lo: i64, den_hi: i64) -> ValueType {
            ValueType::Rational(
                RationalDomain::new(
                    IntegerInterval::new(Integer::from(num_lo), Integer::from(num_hi)).unwrap(),
                    IntegerInterval::new(Integer::from(den_lo), Integer::from(den_hi)).unwrap(),
                )
                .unwrap(),
            )
        }

        pub fn rational_value(numerator: i64, denominator: i64) -> Value {
            Value::Rational(
                Rational::new(Integer::from(numerator), Integer::from(denominator)).unwrap(),
            )
        }

        // ---- environments, one per corpus shape --------------------------
        //
        // Declared directly against `CompositeDeclaration`, `FieldDeclaration`
        // and `NodeKey` rather than read back from the generated crate
        // (which only ever produces `quire_contract_runtime::exact::TypeEnvironment`),
        // so this expands under `rt_side` into a Contract Runtime environment
        // built the same way the direct-call leg builds its own.

        /// R_POINT: `{ x: Integer, y: Integer }`.
        pub fn environment_record() -> TypeEnvironment {
            TypeEnvironment::new(
                vec![CompositeDeclaration::new(
                    node_key(super::super::package::R_POINT),
                    "R_POINT",
                    CompositeShape::Record(vec![
                        FieldDeclaration::new("x", ValueType::Integer, Presence::Required),
                        FieldDeclaration::new("y", ValueType::Integer, Presence::Required),
                    ]),
                )],
                core::iter::empty::<ObjectTypeDeclaration>(),
            )
            .unwrap()
        }

        /// TUP_PAIR: `(Int[-100, 100], Text[0, 16; nfc])`.
        pub fn environment_tuple() -> TypeEnvironment {
            TypeEnvironment::new(
                vec![CompositeDeclaration::new(
                    node_key(super::super::package::TUP_PAIR),
                    "TUP_PAIR",
                    CompositeShape::Tuple(vec![
                        corpus_integer_type(),
                        ValueType::Text(TextType::new(0, 16, TextProfile::Nfc).unwrap()),
                    ]),
                )],
                core::iter::empty::<ObjectTypeDeclaration>(),
            )
            .unwrap()
        }

        /// `Option<Integer>`: no composite declaration needed.
        pub fn environment_option() -> TypeEnvironment {
            TypeEnvironment::new(Vec::new(), core::iter::empty::<ObjectTypeDeclaration>()).unwrap()
        }

        /// A sequence of `Integer`: no composite declaration needed.
        pub fn environment_collection() -> TypeEnvironment {
            TypeEnvironment::new(Vec::new(), core::iter::empty::<ObjectTypeDeclaration>()).unwrap()
        }

        /// R_SELF: `{ next: R_SELF? }`, a one-node self-recursion.
        pub fn environment_self() -> TypeEnvironment {
            TypeEnvironment::new(
                vec![CompositeDeclaration::new(
                    node_key(super::super::package::R_SELF),
                    "R_SELF",
                    CompositeShape::Record(vec![FieldDeclaration::new(
                        "next",
                        composite_type(super::super::package::R_SELF),
                        Presence::Optional,
                    )]),
                )],
                core::iter::empty::<ObjectTypeDeclaration>(),
            )
            .unwrap()
        }

        /// R_PAIR_OF_POINTS: `{ a: R_POINT, b: R_POINT }`, alongside R_POINT
        /// itself.
        pub fn environment_pair_of_points() -> TypeEnvironment {
            TypeEnvironment::new(
                vec![
                    CompositeDeclaration::new(
                        node_key(super::super::package::R_POINT),
                        "R_POINT",
                        CompositeShape::Record(vec![
                            FieldDeclaration::new("x", ValueType::Integer, Presence::Required),
                            FieldDeclaration::new("y", ValueType::Integer, Presence::Required),
                        ]),
                    ),
                    CompositeDeclaration::new(
                        node_key(super::super::package::R_PAIR_OF_POINTS),
                        "R_PAIR_OF_POINTS",
                        CompositeShape::Record(vec![
                            FieldDeclaration::new(
                                "a",
                                composite_type(super::super::package::R_POINT),
                                Presence::Required,
                            ),
                            FieldDeclaration::new(
                                "b",
                                composite_type(super::super::package::R_POINT),
                                Presence::Required,
                            ),
                        ]),
                    ),
                ],
                core::iter::empty::<ObjectTypeDeclaration>(),
            )
            .unwrap()
        }

        pub fn record_point(environment: &TypeEnvironment, x: i64, y: i64) -> Value {
            environment
                .record(
                    node_key(super::super::package::R_POINT),
                    vec![
                        ("x", FieldValue::Present(Value::Integer(Integer::from(x)))),
                        ("y", FieldValue::Present(Value::Integer(Integer::from(y)))),
                    ],
                )
                .unwrap()
        }

        pub fn record_pair_of_points(environment: &TypeEnvironment, a: Value, b: Value) -> Value {
            environment
                .record(
                    node_key(super::super::package::R_PAIR_OF_POINTS),
                    vec![("a", FieldValue::Present(a)), ("b", FieldValue::Present(b))],
                )
                .unwrap()
        }

        pub fn record_self(environment: &TypeEnvironment, next: FieldValue) -> Value {
            environment
                .record(
                    node_key(super::super::package::R_SELF),
                    vec![("next", next)],
                )
                .unwrap()
        }

        pub fn text_value(text: &str) -> Value {
            let payload = TextPayload::from_utf8(text.as_bytes()).unwrap();
            let text_type = TextType::new(0, 16, TextProfile::Nfc).unwrap();
            Value::Text(
                admit_text(&payload, &text_type, &mut Meter::new(UNLIMITED))
                    .completed()
                    .unwrap(),
            )
        }

        pub fn tuple_pair(environment: &TypeEnvironment, n: i64, text: &str) -> Value {
            environment
                .tuple(
                    node_key(super::super::package::TUP_PAIR),
                    vec![Value::Integer(Integer::from(n)), text_value(text)],
                )
                .unwrap()
        }

        pub fn option_int(value: Option<i64>) -> Value {
            match value {
                Some(n) => {
                    OptionValue::present(corpus_integer_type(), Value::Integer(Integer::from(n)))
                        .unwrap()
                }
                None => OptionValue::none(corpus_integer_type()),
            }
        }

        pub fn sequence_type() -> CollectionType {
            CollectionType::new(
                CollectionKind::Sequence,
                corpus_integer_type(),
                Some(CardinalityBound::new(0, 8).unwrap()),
            )
        }

        pub fn sequence_int(elements: &[i64]) -> Value {
            let occurrences = elements
                .iter()
                .map(|n| Value::Integer(Integer::from(*n)))
                .collect();
            form_collection(&sequence_type(), occurrences, &mut Meter::new(UNLIMITED))
                .unwrap()
                .completed()
                .unwrap()
        }

        /// The original source's complete admitted Phase shape.
        pub fn enum_status_type() -> ValueType {
            let declaration = enum_status();
            let actual = super::context().enums().find(|item| item.qualified_name().iter().map(|part| part.as_str()).eq(["Phase"]))
                .expect("the original enum source");
            ValueType::Enum(EnumShape::new(actual.is_ordered(), actual.cases().iter().map(|case| declaration.member(case.case().as_str()).variant())))
        }

        pub fn enum_members() -> quire_semantic_value::enumeration::EnumMemberIndex {
            let mut index = quire_semantic_value::enumeration::EnumMemberIndex::default();
            for actual in super::context().enums() {
                let declaration = EnumDeclaration::new(actual.key(), actual.is_ordered(), actual.cases().iter().map(|case| case.case().as_str().to_owned()).collect())
                    .expect("genuine full enum declaration");
                for case in actual.cases() { index.record(declaration.member(case.case().as_str(), case.key()).expect("actual admitted member")); }
            }
            index
        }

        /// An operand's static type for a direct `check_equality` call: a
        /// source type and, for a `convert<T>` operand, its conversion
        /// target — the same shape
        /// `quire_contract_codegen::composite_equality::EqualityOperandDescriptor`
        /// requests. [`direct_equality`] needs this, rather than a bare
        /// `ValueType`, to express a `converted` operand (FR-018-AC-2's
        /// conversion-ordering mutation has nothing to swap without one).
        #[derive(Clone)]
        pub struct DirectOperand {
            source: ValueType,
            target: Option<ValueType>,
        }

        /// An operand of static type `source`, applying no conversion.
        pub fn operand_typed(source: ValueType) -> DirectOperand {
            DirectOperand {
                source,
                target: None,
            }
        }

        /// An operand `convert<target>(e)` for `e` of static type `source`.
        pub fn operand_converted(source: ValueType, target: ValueType) -> DirectOperand {
            DirectOperand {
                source,
                target: Some(target),
            }
        }

        /// `TypeEnvironment::check_equality` plus `CheckedEquality::evaluate`,
        /// invoked directly and driven from the request's own operator and
        /// operand descriptors (FR-018-AC-2, AC-3): no `check_type` guard,
        /// since none of this file's requests are refused by the
        /// environment, so the corresponding oracle's guard never takes its
        /// early-return branch either.
        pub fn direct_equality(
            environment: &TypeEnvironment,
            operator: EqualityOperator,
            left: DirectOperand,
            right: DirectOperand,
            left_value: &Value,
            right_value: &Value,
            meter: &mut Meter,
        ) -> Outcome<bool> {
            let left_operand = match left.target {
                Some(target) => EqualityOperand::converted(left.source, target),
                None => EqualityOperand::typed(left.source),
            };
            let right_operand = match right.target {
                Some(target) => EqualityOperand::converted(right.source, target),
                None => EqualityOperand::typed(right.source),
            };
            let checked = match environment.check_equality(operator, left_operand, right_operand, &enum_members()) {
                Ok(checked) => checked,
                Err(error) => {
                    return Outcome::Refused(Refusal::CheckedInvariant {
                        cause: quire_exact::CheckedInvariantCause::GeneratedEqualityCheckRejected {
                            cause: error.cause,
                        },
                    })
                }
            };
            checked.evaluate(environment, left_value, right_value, meter)
        }
    };
}

pub mod rt_side {
    pub use quire_semantic_value::declaration::{
        CompositeDeclaration, CompositeShape, EqualityOperand, EqualityOperator, FieldDeclaration,
        ObjectTypeDeclaration, TypeEnvironment,
    };
    pub use quire_semantic_value::enumeration::{EnumDeclaration, EnumValue};
    pub use quire_exact::*;
    shared_helpers!();

    /// Genuine original Phase declaration, including every admitted case key and canonical rank.
    pub struct EnumStatus {
        declaration: EnumDeclaration,
        admitted: qsl_replay::AdmittedEnum,
    }

    pub fn enum_status() -> EnumStatus {
        let admitted = super::context().enums().find(|item| item.qualified_name().iter().map(|part| part.as_str()).eq(["Phase"]))
            .expect("the original source declared Phase").clone();
        let declaration = EnumDeclaration::new(admitted.key(), admitted.is_ordered(), admitted.cases().iter().map(|case| case.case().as_str().to_owned()).collect()).expect("real original enum declaration");
        EnumStatus { declaration, admitted }
    }

    impl EnumStatus {
        pub fn member(&self, case: &str) -> EnumValue {
            let wire = qsl_replay::WireNodeId::from_digest(*self.admitted.key().as_bytes());
            let name = Identifier::new(case).expect("case identifier");
            let actual = super::context().enum_case(wire, &name).expect("real admitted case");
            self.declaration.member(case, actual.key()).expect("member belongs to original declaration")
        }
        pub fn value(&self, case: &str) -> Value {
            let member = self.member(case);
            Value::Enum(EnumMember::new(member.variant(), u32::try_from(member.position()).expect("original rank fits u32")))
        }
    }
}
