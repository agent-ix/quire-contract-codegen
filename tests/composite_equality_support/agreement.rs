//! Three-way agreement support for composite equality (FR-018-AC-2, AC-8,
//! AC-9, AC-11): the pinned QSL authority (`quire_spec_language::value`),
//! direct Contract Runtime execution, and generated oracles.
//!
//! `quire_spec_language::value` re-exports the identical FR-149 equality
//! surface (`TypeEnvironment::check_equality`, `CheckedEquality::evaluate`,
//! `EqualityOperand`, `EqualitySchedule`, the same `equality.*` charge
//! points) that `quire_contract_runtime::exact` pins, so all three legs are
//! reachable here — unlike `exact_scalar_support/agreement.rs`'s runtime-only
//! vectors for integer arithmetic and ordering, this file has no `agree2!`
//! gap to disclose for FR-018-AC-2 itself.
//!
//! Adapted from `exact_scalar_support/agreement.rs`: a vector body is
//! written once as unqualified calls; [`agree3!`] evaluates it with
//! `quire_spec_language::value` in scope, again with
//! `quire_contract_runtime::exact` in scope, and evaluates the generated
//! oracle with the runtime in scope. Every evaluation is run under the same
//! limits and then re-run with each admitted charge denied in turn. The
//! three `Debug` renderings must be identical.

#![allow(dead_code, unused_imports)]

use quire_spec_language::value as authority;
use serde_json::{json, Value as JsonValue};

/// The vendored `Example.Status` enum declaration (`quire_type_id`
/// `package::ENUM_TYPE_DIGEST`, already verified nominal identity):
/// `ordered`, members `READY`/`DONE` in declaration order.
fn enum_status_declaration_json() -> JsonValue {
    json!({
        "version": "quire.enum-declaration-node/v1",
        "owner": {"kind": "definition", "authority": "agent-ix", "identity": "example-model"},
        "qualified_declaration": ["Example", "Status"],
        "ordered": true,
        "members": ["READY", "DONE"],
    })
}

fn enum_status_member_json(case: &str) -> JsonValue {
    json!({
        "version": "quire.enum-member-node/v1",
        "declaration_node_id": {
            "domain": authority::NODE_KEY_DOMAIN,
            "digest": crate::package::ENUM_TYPE_DIGEST,
        },
        "case": case,
    })
}

/// Authority-computed member key for `case` of `Example.Status`.
fn enum_status_member_key(case: &str) -> String {
    let preimage = authority::EnumMemberPreimage::from_json(enum_status_member_json(case)).unwrap();
    preimage.node_key().unwrap().to_string()
}

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
            (outcome, meter.admitted_charges().to_vec(), consumed)
        }

        /// The QSL authority's `InjectedDenial::occurrence` is a plain `u64`;
        /// the Contract Runtime's is a `NonZeroU64` (deliberately, so a
        /// malformed 0-based request cannot compile). This trait lets
        /// [`denials`] build the field's value generically across both
        /// module expansions of [`shared_helpers`].
        trait DenialOccurrence {
            fn denial_occurrence(n: u64) -> Self;
        }
        impl DenialOccurrence for u64 {
            fn denial_occurrence(n: u64) -> Self {
                n
            }
        }
        impl DenialOccurrence for core::num::NonZeroU64 {
            fn denial_occurrence(n: u64) -> Self {
                core::num::NonZeroU64::new(n).unwrap()
            }
        }

        /// Deny each admitted charge occurrence in turn, discovered
        /// dynamically from a full run: `(point, occurrence, outcome, every
        /// counter of the run stopped immediately before that occurrence —
        /// the denied charge is never applied, so this *is* that run's
        /// final state)`. All ten `LimitKind`s are carried, not only
        /// `ResultUnits`, so `agree3!`'s cross-leg `Debug` comparison checks
        /// every counter the three independent legs compute for each denial,
        /// not just one of them.
        pub fn denials<T>(
            limits: ScalarLimits,
            run: impl Fn(&mut Meter) -> T,
        ) -> Vec<(ChargePoint, u64, T, Vec<u64>)> {
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
                        occurrence: DenialOccurrence::denial_occurrence(occurrence),
                    });
                    let outcome = run(&mut denied);
                    let counters = LimitKind::ALL
                        .iter()
                        .map(|kind| denied.consumed(*kind))
                        .collect();
                    (point, occurrence, outcome, counters)
                })
                .collect()
        }

        pub fn node_key(code: u32) -> NodeKey {
            NodeKey::from_hex(&crate::package::key(code)).unwrap()
        }

        pub fn composite_type(code: u32) -> ValueType {
            ValueType::Composite(node_key(code))
        }

        // ---- environments, one per corpus shape --------------------------
        //
        // Declared directly against each side's own `CompositeDeclaration`,
        // `FieldDeclaration` and `NodeKey` rather than read back from the
        // generated golden crate (which only ever produces
        // `quire_contract_runtime::exact::TypeEnvironment`), so the identical
        // text below expands under `qsl_side` into a QSL authority
        // environment and under `rt_side` into a Contract Runtime one.

        /// R_POINT: `{ x: Integer, y: Integer }`.
        pub fn environment_record() -> TypeEnvironment {
            TypeEnvironment::new(
                vec![CompositeDeclaration::new(
                    node_key(crate::package::R_POINT),
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
                    node_key(crate::package::TUP_PAIR),
                    "TUP_PAIR",
                    CompositeShape::Tuple(vec![
                        ValueType::Int(
                            IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64))
                                .unwrap(),
                        ),
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
                    node_key(crate::package::R_SELF),
                    "R_SELF",
                    CompositeShape::Record(vec![FieldDeclaration::new(
                        "next",
                        composite_type(crate::package::R_SELF),
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
                        node_key(crate::package::R_POINT),
                        "R_POINT",
                        CompositeShape::Record(vec![
                            FieldDeclaration::new("x", ValueType::Integer, Presence::Required),
                            FieldDeclaration::new("y", ValueType::Integer, Presence::Required),
                        ]),
                    ),
                    CompositeDeclaration::new(
                        node_key(crate::package::R_PAIR_OF_POINTS),
                        "R_PAIR_OF_POINTS",
                        CompositeShape::Record(vec![
                            FieldDeclaration::new(
                                "a",
                                composite_type(crate::package::R_POINT),
                                Presence::Required,
                            ),
                            FieldDeclaration::new(
                                "b",
                                composite_type(crate::package::R_POINT),
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
                    node_key(crate::package::R_POINT),
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
                    node_key(crate::package::R_PAIR_OF_POINTS),
                    vec![("a", FieldValue::Present(a)), ("b", FieldValue::Present(b))],
                )
                .unwrap()
        }

        pub fn record_self(environment: &TypeEnvironment, next: FieldValue) -> Value {
            environment
                .record(node_key(crate::package::R_SELF), vec![("next", next)])
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
                    node_key(crate::package::TUP_PAIR),
                    vec![Value::Integer(Integer::from(n)), text_value(text)],
                )
                .unwrap()
        }

        pub fn option_int(value: Option<i64>) -> Value {
            match value {
                Some(n) => {
                    OptionValue::present(ValueType::Integer, Value::Integer(Integer::from(n)))
                        .unwrap()
                }
                None => OptionValue::none(ValueType::Integer),
            }
        }

        pub fn sequence_type() -> CollectionType {
            CollectionType::new(
                CollectionKind::Sequence,
                ValueType::Integer,
                CardinalityBound::new(0, 8).unwrap(),
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

        /// The vendored `Example.Status` enum's `ValueType`, matching
        /// `package::ENUM_TYPE_DIGEST`.
        pub fn enum_status_type() -> ValueType {
            ValueType::Enum(node_key_from_digest(crate::package::ENUM_TYPE_DIGEST))
        }

        fn node_key_from_digest(digest: &str) -> NodeKey {
            NodeKey::from_hex(digest).unwrap()
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
            let checked = match environment.check_equality(operator, left_operand, right_operand) {
                Ok(checked) => checked,
                Err(_) => return Outcome::Refused(Refusal::CheckedInvariant),
            };
            checked.evaluate(left_value, right_value, meter)
        }
    };
}

pub mod qsl_side {
    pub use quire_spec_language::value::*;
    shared_helpers!();

    /// The vendored `Example.Status` declaration, admitted under its
    /// verified node id (`crate::package::ENUM_TYPE_DIGEST`).
    pub struct EnumStatus(EnumDeclaration);

    fn owners() -> OwnerSelection {
        OwnerSelection::new([NodeOwner::Definition(OwnerSubject {
            authority: "agent-ix".into(),
            identity: "example-model".into(),
        })])
    }

    pub fn enum_status() -> EnumStatus {
        let preimage =
            EnumDeclarationPreimage::from_json(crate::support::enum_status_declaration_json())
                .unwrap();
        let key = NodeKey::from_hex(crate::package::ENUM_TYPE_DIGEST).unwrap();
        EnumStatus(EnumDeclaration::admit(preimage, key, &owners()).unwrap())
    }

    impl EnumStatus {
        pub fn value(&self, case: &str) -> Value {
            let key = crate::support::enum_status_member_key(case);
            let preimage =
                EnumMemberPreimage::from_json(crate::support::enum_status_member_json(case))
                    .unwrap();
            let member = NodeKey::from_hex(&key).unwrap();
            Value::Enum(self.0.admit_member(&preimage, member).unwrap())
        }
    }
}

pub mod rt_side {
    pub use quire_contract_runtime::exact::*;
    shared_helpers!();

    /// The vendored `Example.Status` declaration, under its authority-verified
    /// node id (`crate::package::ENUM_TYPE_DIGEST`) and authority-computed
    /// member keys, so the same identities are in play as `qsl_side`'s.
    pub struct EnumStatus(EnumDeclaration);

    pub fn enum_status() -> EnumStatus {
        let key = NodeKey::from_hex(crate::package::ENUM_TYPE_DIGEST).unwrap();
        EnumStatus(EnumDeclaration::new(key, true, &["READY", "DONE"]).unwrap())
    }

    impl EnumStatus {
        pub fn value(&self, case: &str) -> Value {
            let key = crate::support::enum_status_member_key(case);
            let member = NodeKey::from_hex(&key).unwrap();
            Value::Enum(self.0.member(case, member).unwrap())
        }
    }
}
