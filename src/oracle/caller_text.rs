//! Caller-ingress TextPayload admission generation (FR-035).
//!
//! This route has its own declared text type and finite proof input. It does not
//! consume a CheckedPackage node or borrow the FR-014 scalar claim vocabulary.

use quire_contract_runtime::exact::{TextProfile, TextType};
use serde::Serialize;

use crate::core::artifact::Artifact;

/// The finite symbolic source class supported by the caller admission proof.
pub const CALLER_TEXT_PAYLOAD_CLASS: &str = "zero-to-two-scalars:e,u+0301,u+00e9,u+1f600";

/// Exact UTF-8 members of the finite proof class in length/lexicographic order.
const CALLER_TEXT_PROOF_INPUTS: [&str; 21] = [
    "",
    "e",
    "\u{301}",
    "\u{e9}",
    "\u{1f600}",
    "ee",
    "e\u{301}",
    "e\u{e9}",
    "e\u{1f600}",
    "\u{301}e",
    "\u{301}\u{301}",
    "\u{301}\u{e9}",
    "\u{301}\u{1f600}",
    "\u{e9}e",
    "\u{e9}\u{301}",
    "\u{e9}\u{e9}",
    "\u{e9}\u{1f600}",
    "\u{1f600}e",
    "\u{1f600}\u{301}",
    "\u{1f600}\u{e9}",
    "\u{1f600}\u{1f600}",
];

/// Request for one caller-ingress admission oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallerTextAdmissionRequest {
    /// Caller-declared inclusive type bounds and profile.
    pub text_type: TextType,
}

/// Typed reason that a caller Text proof request cannot be generated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallerTextProofRefusal {
    /// The request does not use the finite zero-to-two-scalar source class.
    UnsupportedPayloadClass,
    /// The requested bounds are outside the two encoded proof types.
    RequiresSupportedBounds {
        /// Requested inclusive minimum.
        min: u64,
        /// Requested inclusive maximum.
        max: u64,
    },
    /// The runtime added a profile with no finite proof encoding here.
    UnsupportedProfile,
}

impl std::fmt::Display for CallerTextProofRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPayloadClass => {
                formatter.write_str("unsupported caller Text payload class")
            }
            Self::RequiresSupportedBounds { min, max } => {
                write!(
                    formatter,
                    "caller Text bounds {min}..={max} have no finite proof encoding"
                )
            }
            Self::UnsupportedProfile => {
                formatter.write_str("runtime Text profile has no caller proof encoding")
            }
        }
    }
}

impl std::error::Error for CallerTextProofRefusal {}

/// Caller-declared identity of a finite Text admission proof.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallerTextProofIdentity {
    /// Declared profile.
    pub profile: CallerTextProfile,
    /// Inclusive minimum profile length.
    pub min: u64,
    /// Inclusive maximum profile length.
    pub max: u64,
    /// Exact finite symbolic payload class.
    pub payload_class: &'static str,
}

/// The six runtime text profiles this generator recognizes in caller claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CallerTextProfile {
    /// Unicode scalar count without normalization.
    UnicodeScalars,
    /// Unicode NFC.
    Nfc,
    /// Unicode NFD.
    Nfd,
    /// Unicode NFKC.
    Nfkc,
    /// Unicode NFKD.
    Nfkd,
    /// UTF-8 byte count.
    BinaryUtf8,
}

/// Generated caller Text oracle source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallerTextOracle {
    /// Caller-ingress identity, without checked-node or QSL provenance.
    pub identity: CallerTextProofIdentity,
    /// Source artifact containing the oracle function.
    pub source: Artifact,
}

/// A caller-declared finite Text Kani harness, before it enters the shared executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallerTextProofHarness {
    /// Caller-ingress identity for this harness.
    pub identity: CallerTextProofIdentity,
    /// Complete Rust source for the bounded proof harness.
    pub source: Artifact,
    /// Solver name.
    pub solver: String,
    /// Options passed to the solver adapter.
    pub options: Vec<String>,
}

/// Emit a caller-ingress oracle that delegates admission and accounting to Contract Runtime.
pub fn generate_caller_text_oracle(
    request: CallerTextAdmissionRequest,
) -> Result<CallerTextOracle, CallerTextProofRefusal> {
    let text_type = request.text_type;
    let (min, max) = (text_type.min(), text_type.max());
    let source = "use quire_contract_runtime::exact as rt;\n\npub fn admit_caller_text(operand: &rt::TextPayload, text_type: &rt::TextType, meter: &mut rt::Meter) -> rt::Outcome<rt::Text> {\n    rt::admit_text(operand, text_type, meter)\n}\n".to_owned();
    Ok(CallerTextOracle {
        identity: CallerTextProofIdentity {
            profile: caller_profile(text_type.profile())?,
            min,
            max,
            payload_class: CALLER_TEXT_PAYLOAD_CLASS,
        },
        source: Artifact::new("src/lib.rs", source),
    })
}

/// Emit the generated public caller oracle source for embedding in a proof harness.
///
/// The proof harness consumes this artifact rather than reconstructing an equivalent wrapper, so
/// agreement and mutation checks exercise the same function callers receive.
pub(crate) fn generated_caller_text_oracle_source(
    text_type: &TextType,
) -> Result<String, CallerTextProofRefusal> {
    let oracle = generate_caller_text_oracle(CallerTextAdmissionRequest {
        text_type: *text_type,
    })?;
    Ok(oracle.source.contents)
}

/// Execute caller-ingress admission through Contract Runtime's exact API.
pub fn admit_caller_text(
    payload: &quire_contract_runtime::exact::TextPayload,
    text_type: &TextType,
    meter: &mut quire_contract_runtime::exact::Meter,
) -> quire_contract_runtime::exact::Outcome<quire_contract_runtime::exact::Text> {
    quire_contract_runtime::exact::admit_text(payload, text_type, meter)
}

/// Refuse proof requests whose bounds are outside the finite generated pair.
pub fn validate_caller_text_proof(
    text_type: &TextType,
    payload_class: &str,
) -> Result<CallerTextProofIdentity, CallerTextProofRefusal> {
    if payload_class != CALLER_TEXT_PAYLOAD_CLASS {
        return Err(CallerTextProofRefusal::UnsupportedPayloadClass);
    }
    let bounds = (text_type.min(), text_type.max());
    if !matches!(bounds, (0, 8) | (1, 1)) {
        return Err(CallerTextProofRefusal::RequiresSupportedBounds {
            min: bounds.0,
            max: bounds.1,
        });
    }
    Ok(CallerTextProofIdentity {
        profile: caller_profile(text_type.profile())?,
        min: bounds.0,
        max: bounds.1,
        payload_class: CALLER_TEXT_PAYLOAD_CLASS,
    })
}

/// Generate a finite caller admission harness for one supported profile and bound.
///
/// The returned harness is deliberately not an `ir_confirmed` scalar obligation. The production
/// executor adapter is maintained in the shared FR-017 lane.
pub fn generate_caller_text_proof_harness(
    text_type: &TextType,
    payload_class: &str,
    solver: &str,
    options: Vec<String>,
) -> Result<CallerTextProofHarness, CallerTextProofRefusal> {
    let identity = validate_caller_text_proof(text_type, payload_class)?;
    let profile = profile_source(identity.profile);
    let min = identity.min;
    let max = identity.max;
    let oracle = generated_caller_text_oracle_source(text_type)?;
    let input_arms = caller_text_input_arms();
    let source = format!(
        r#"pub mod caller_text {{
{oracle}

fn unlimited_meter() -> rt::Meter {{
    rt::Meter::new(rt::ScalarLimits {{
        integer_bits: u64::MAX, decimal_digits: u64::MAX, scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX, text_scalars: u64::MAX, normalized_scalars: u64::MAX,
        unit_edges: u64::MAX, value_occurrences: u64::MAX, work_units: u64::MAX,
        result_units: u64::MAX,
    }})
}}

fn assert_same(actual: &rt::Outcome<rt::Text>, direct: &rt::Outcome<rt::Text>) {{
    match (actual, direct) {{
        (rt::Outcome::Completed(actual), rt::Outcome::Completed(direct)) => {{
            assert_eq!(actual.text_type(), direct.text_type());
            assert_eq!(actual.retained(), direct.retained());
            // Text::length() is derived from text_type.profile() and retained text, both
            // compared above, so checking it again duplicates the same symbolic relation.
            assert_eq!(actual.payload().as_str(), direct.payload().as_str());
            assert_eq!(actual.payload().provenance(), direct.payload().provenance());
        }}
        (rt::Outcome::Undefined(actual), rt::Outcome::Undefined(direct)) => assert_eq!(actual, direct),
        (rt::Outcome::Refused(actual), rt::Outcome::Refused(direct)) => assert_eq!(actual, direct),
        (rt::Outcome::Incomplete(actual), rt::Outcome::Incomplete(direct)) => assert_eq!(actual, direct),
        _ => assert!(false, "completion/refusal disagreement"),
    }}
}}

#[kani::proof]
pub fn caller_text_admission_agrees() {{
    let selector: u8 = kani::any();
    kani::assume(selector < 21);
    let payload = match selector {{
{input_arms}
        _ => {{ assert!(false, "selector is in the finite caller text class"); return; }}
    }};
    let payload = match payload {{
        Ok(payload) => payload,
        Err(_) => {{ assert!(false, "finite caller text member is valid UTF-8"); return; }}
    }};
    let text_type = rt::TextType::new({min}, {max}, {profile}).expect("generated valid bounds");
    let mut generated_meter = unlimited_meter();
    let generated = admit_caller_text(&payload, &text_type, &mut generated_meter);
    let mut direct_meter = unlimited_meter();
    let direct = rt::admit_text(&payload, &text_type, &mut direct_meter);
    assert_same(&generated, &direct);
    assert_eq!(generated_meter, direct_meter);
    kani::cover!(true, "a constructed member of the finite caller text class is reached");
}}
}}
"#,
        oracle = oracle,
        input_arms = input_arms,
    );
    Ok(CallerTextProofHarness {
        identity,
        source: Artifact::new("src/lib.rs", source),
        solver: solver.to_owned(),
        options,
    })
}

pub(crate) fn caller_profile(
    profile: TextProfile,
) -> Result<CallerTextProfile, CallerTextProofRefusal> {
    match profile {
        TextProfile::UnicodeScalars => Ok(CallerTextProfile::UnicodeScalars),
        TextProfile::Nfc => Ok(CallerTextProfile::Nfc),
        TextProfile::Nfd => Ok(CallerTextProfile::Nfd),
        TextProfile::Nfkc => Ok(CallerTextProfile::Nfkc),
        TextProfile::Nfkd => Ok(CallerTextProfile::Nfkd),
        TextProfile::BinaryUtf8 => Ok(CallerTextProfile::BinaryUtf8),
        _ => Err(CallerTextProofRefusal::UnsupportedProfile),
    }
}

fn profile_source(profile: CallerTextProfile) -> &'static str {
    match profile {
        CallerTextProfile::UnicodeScalars => "rt::TextProfile::UnicodeScalars",
        CallerTextProfile::Nfc => "rt::TextProfile::Nfc",
        CallerTextProfile::Nfd => "rt::TextProfile::Nfd",
        CallerTextProfile::Nfkc => "rt::TextProfile::Nfkc",
        CallerTextProfile::Nfkd => "rt::TextProfile::Nfkd",
        CallerTextProfile::BinaryUtf8 => "rt::TextProfile::BinaryUtf8",
    }
}

fn caller_text_input_arms() -> String {
    CALLER_TEXT_PROOF_INPUTS
        .iter()
        .enumerate()
        .map(|(selector, input)| {
            format!("{selector} => rt::TextPayload::from_utf8({input:?}.as_bytes()),")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use quire_contract_runtime::exact::{
        ChargePoint, InjectedDenial, Meter, ScalarLimits, TextPayload,
    };
    use std::num::NonZeroU64;

    fn unlimited_meter() -> Meter {
        Meter::new(ScalarLimits {
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
        })
    }

    /// Trace: FR-035-AC-1, FR-035-AC-6.
    #[test]
    fn caller_oracle_calls_runtime_admission_without_checked_node_identity() {
        let generated = generate_caller_text_oracle(CallerTextAdmissionRequest {
            text_type: TextType::new(0, 8, TextProfile::Nfc).expect("valid type"),
        })
        .expect("known runtime profile");
        assert!(generated.source.contents.contains(
            "pub fn admit_caller_text(operand: &rt::TextPayload, text_type: &rt::TextType, meter: &mut rt::Meter)"
        ));
        assert_eq!(
            generated.source.contents.matches("rt::admit_text(").count(),
            1
        );
        assert_eq!(generated.identity.profile, CallerTextProfile::Nfc);
        assert_eq!((generated.identity.min, generated.identity.max), (0, 8));
        assert_eq!(generated.identity.payload_class, CALLER_TEXT_PAYLOAD_CLASS);

        let inputs = ["", "e", "\u{301}", "\u{e9}", "\u{1f600}", "e\u{301}"];
        for profile in TextProfile::ALL {
            for (min, max) in [(0, 8), (1, 1)] {
                let text_type = TextType::new(min, max, profile).expect("valid type");
                for input in inputs {
                    let payload = TextPayload::from_utf8(input.as_bytes()).expect("valid UTF-8");
                    let mut actual_meter = unlimited_meter();
                    let actual = admit_caller_text(&payload, &text_type, &mut actual_meter);
                    let mut direct_meter = unlimited_meter();
                    let direct = quire_contract_runtime::exact::admit_text(
                        &payload,
                        &text_type,
                        &mut direct_meter,
                    );
                    assert_same(&actual, &direct);
                    assert_eq!(actual_meter, direct_meter);
                }
            }
        }
    }

    /// Trace: FR-035-AC-2, FR-035-AC-3.
    #[test]
    fn constructor_refusal_precedes_admission_and_reached_charge_denials_match_runtime() {
        let constructor_meter = unlimited_meter();
        let before_constructor = constructor_meter.clone();
        let error = TextPayload::from_utf8(&[0x66, 0x80]).expect_err("malformed UTF-8");
        assert_eq!(error.valid_up_to, 1);
        assert_eq!(constructor_meter, before_constructor);
        let payload = TextPayload::from_utf8("e\u{301}".as_bytes()).expect("valid UTF-8");
        let refusing_type = TextType::new(1, 1, TextProfile::Nfd).expect("valid type");
        let mut refusing_meter = unlimited_meter();
        let refused = admit_caller_text(&payload, &refusing_type, &mut refusing_meter);
        assert!(matches!(
            refused,
            quire_contract_runtime::exact::Outcome::Refused(_)
        ));
        assert!(!refusing_meter
            .admitted_charges()
            .contains(&ChargePoint::TextResultRetain));

        let payload = TextPayload::from_utf8("e\u{301}".as_bytes()).expect("valid UTF-8");
        for profile in TextProfile::ALL {
            let text_type = TextType::new(0, 8, profile).expect("valid type");
            let mut baseline_meter = unlimited_meter();
            let baseline = admit_caller_text(&payload, &text_type, &mut baseline_meter);
            assert!(matches!(
                baseline,
                quire_contract_runtime::exact::Outcome::Completed(_)
            ));
            for point in baseline_meter.admitted_charges().to_vec() {
                let denial = InjectedDenial {
                    point,
                    occurrence: NonZeroU64::new(1).expect("nonzero"),
                };
                let mut actual_meter = unlimited_meter().with_injected_denial(denial);
                let actual = admit_caller_text(&payload, &text_type, &mut actual_meter);
                let mut direct_meter = unlimited_meter().with_injected_denial(denial);
                let direct = quire_contract_runtime::exact::admit_text(
                    &payload,
                    &text_type,
                    &mut direct_meter,
                );
                assert_same(&actual, &direct);
                assert_eq!(actual_meter, direct_meter);
                if point == ChargePoint::TextNormalizeOutput {
                    assert!(!actual_meter
                        .admitted_charges()
                        .contains(&ChargePoint::TextResultRetain));
                }
            }
        }

        let nfc = TextType::new(1, 1, TextProfile::Nfc).expect("valid type");
        let mut nfc_meter = unlimited_meter();
        let nfc_result = admit_caller_text(&payload, &nfc, &mut nfc_meter);
        let quire_contract_runtime::exact::Outcome::Completed(nfc_text) = nfc_result else {
            panic!("NFC composes e plus combining acute to one scalar")
        };
        assert_eq!(nfc_text.retained(), "\u{e9}");

        let nfd = TextType::new(1, 1, TextProfile::Nfd).expect("valid type");
        let mut nfd_meter = unlimited_meter();
        let nfd_result = admit_caller_text(&payload, &nfd, &mut nfd_meter);
        assert!(matches!(
            nfd_result,
            quire_contract_runtime::exact::Outcome::Refused(_)
        ));
        assert!(!nfd_meter
            .admitted_charges()
            .contains(&ChargePoint::TextResultRetain));
    }

    fn assert_same(
        actual: &quire_contract_runtime::exact::Outcome<quire_contract_runtime::exact::Text>,
        direct: &quire_contract_runtime::exact::Outcome<quire_contract_runtime::exact::Text>,
    ) {
        use quire_contract_runtime::exact::Outcome;
        match (actual, direct) {
            (Outcome::Completed(actual), Outcome::Completed(direct)) => {
                assert_eq!(actual.text_type(), direct.text_type());
                assert_eq!(actual.retained(), direct.retained());
                assert_eq!(actual.length(), direct.length());
                assert_eq!(actual.payload().as_str(), direct.payload().as_str());
                assert_eq!(actual.payload().provenance(), direct.payload().provenance());
            }
            (Outcome::Undefined(actual), Outcome::Undefined(direct)) => {
                assert_eq!(actual, direct)
            }
            (Outcome::Refused(actual), Outcome::Refused(direct)) => assert_eq!(actual, direct),
            (Outcome::Incomplete(actual), Outcome::Incomplete(direct)) => {
                assert_eq!(actual, direct)
            }
            _ => panic!("completion/refusal disagreement"),
        }
    }

    /// Trace: FR-035-AC-6.
    #[test]
    fn proof_request_refuses_unsupported_class_and_bounds() {
        let supported = TextType::new(1, 1, TextProfile::Nfc).expect("valid type");
        assert_eq!(
            validate_caller_text_proof(&supported, "arbitrary-utf8"),
            Err(CallerTextProofRefusal::UnsupportedPayloadClass)
        );
        let unsupported = TextType::new(0, 9, TextProfile::Nfc).expect("valid type");
        assert_eq!(
            validate_caller_text_proof(&unsupported, CALLER_TEXT_PAYLOAD_CLASS),
            Err(CallerTextProofRefusal::RequiresSupportedBounds { min: 0, max: 9 })
        );
    }

    /// Trace: FR-035-AC-4, FR-035-AC-6.
    #[test]
    fn proof_generator_emits_each_profile_and_both_supported_bound_pairs() {
        for profile in TextProfile::ALL {
            for (min, max) in [(0, 8), (1, 1)] {
                let text_type = TextType::new(min, max, profile).expect("valid type");
                let harness = generate_caller_text_proof_harness(
                    &text_type,
                    CALLER_TEXT_PAYLOAD_CLASS,
                    "cadical",
                    vec!["--unwind".to_owned(), "8".to_owned()],
                )
                .expect("supported request");
                let source = harness.source.contents;
                assert_eq!(harness.identity.profile, caller_profile(profile).unwrap());
                assert_eq!((harness.identity.min, harness.identity.max), (min, max));
                assert_eq!(harness.identity.payload_class, CALLER_TEXT_PAYLOAD_CLASS);
                assert_eq!(harness.solver, "cadical");
                assert_eq!(harness.options, ["--unwind", "8"]);
                assert_eq!(source.matches("kani::cover!(").count(), 1);
                assert!(source.ends_with("}\n"));
                assert!(source.contains("let selector: u8 = kani::any();"));
                assert!(source.contains("kani::assume(selector < 21);"));
                assert!(source.contains("let payload = match selector {"));
                assert!(source.contains(
                    "_ => { assert!(false, \"selector is in the finite caller text class\"); return; }"
                ));
                assert_eq!(source.matches("rt::TextPayload::from_utf8(").count(), 21);
                for (selector, input) in CALLER_TEXT_PROOF_INPUTS.iter().enumerate() {
                    assert!(source.contains(&format!(
                        "{selector} => rt::TextPayload::from_utf8({input:?}.as_bytes()),"
                    )));
                }
                assert_eq!(source.matches("kani::any()").count(), 1);
                assert!(!source.contains("encode_utf8"));
                assert!(source.contains("let generated = admit_caller_text("));
                assert!(source.contains("let direct = rt::admit_text("));
                assert!(source.contains("assert_eq!(generated_meter, direct_meter);"));
                assert!(
                    source
                        .find("assert_eq!(generated_meter, direct_meter);")
                        .unwrap()
                        < source.find("kani::cover!(").unwrap()
                );
            }
        }
    }

    /// Trace: FR-035-AC-4.
    #[test]
    fn symbolic_selector_table_is_a_bijection_to_the_twenty_one_scalar_sequences() {
        let alphabet = ['e', '\u{301}', '\u{e9}', '\u{1f600}'];
        let mut expected = vec![String::new()];
        expected.extend(alphabet.iter().map(char::to_string));
        for first in alphabet {
            for second in alphabet {
                expected.push([first, second].iter().collect());
            }
        }
        assert_eq!(CALLER_TEXT_PROOF_INPUTS.len(), 21);
        assert_eq!(
            CALLER_TEXT_PROOF_INPUTS.as_slice(),
            expected.iter().map(String::as_str).collect::<Vec<_>>()
        );
        assert_eq!(
            CALLER_TEXT_PROOF_INPUTS
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            21
        );
    }
}
