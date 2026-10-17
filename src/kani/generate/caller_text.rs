//! Caller-ingress Text admission Kani harness generation (FR-035).

use quire_exact::TextType;

use crate::{
    core::{
        artifact::Artifact,
        identity::{HarnessSymbol, ModuleSymbol, SymbolError},
    },
    kani::{
        abi::{adapter_options, KaniSolver},
        identity::{CallerTextObligationIdentity, KaniCallerTextObligationHarness, ProofCeilings},
    },
    oracle::caller_text::{generate_caller_text_proof_harness, CallerTextProofRefusal},
};

/// Inputs for generating one caller Text admission harness.
#[derive(Clone, Debug)]
pub struct CallerTextKaniRequest {
    /// Caller-declared text type.
    pub text_type: TextType,
    /// Requested finite source class. Only the declared zero-to-two-scalar class is supported.
    pub payload_class: String,
    /// Resource ceilings applied at execution.
    pub ceilings: ProofCeilings,
    /// Kani solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
}

/// Why a caller Text Kani harness could not be generated.
#[derive(Debug)]
pub enum CallerTextKaniGenerationError {
    /// The request has an unsupported finite source, type bound or profile.
    Proof(CallerTextProofRefusal),
    /// A fixed generated module or harness name was rejected as a Rust identifier.
    Symbol(SymbolError),
    /// The generated source did not parse as Rust.
    InvalidSource,
    /// The identity record could not be serialized.
    RecordSerialization,
}

impl std::fmt::Display for CallerTextKaniGenerationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proof(refusal) => std::fmt::Display::fmt(refusal, formatter),
            Self::Symbol(error) => write!(
                formatter,
                "generated caller Text harness symbol is invalid: {error}"
            ),
            Self::InvalidSource => {
                formatter.write_str("generated caller Text Kani source did not parse")
            }
            Self::RecordSerialization => {
                formatter.write_str("caller Text Kani identity could not be serialized")
            }
        }
    }
}

impl std::error::Error for CallerTextKaniGenerationError {}

impl From<CallerTextProofRefusal> for CallerTextKaniGenerationError {
    fn from(refusal: CallerTextProofRefusal) -> Self {
        Self::Proof(refusal)
    }
}

impl From<SymbolError> for CallerTextKaniGenerationError {
    fn from(error: SymbolError) -> Self {
        Self::Symbol(error)
    }
}

/// Generate a caller-ingress proof harness and its persisted execution identity.
pub fn generate_caller_text_kani_harness(
    request: CallerTextKaniRequest,
) -> Result<KaniCallerTextObligationHarness, CallerTextKaniGenerationError> {
    let profile = request.text_type.profile();
    let bounds = (request.text_type.min(), request.text_type.max());
    let module = format!(
        "caller_text_{}_{}_{}",
        profile.as_str().replace('-', "_"),
        bounds.0,
        bounds.1
    );
    let stem = "caller_text_admission_agrees";
    let harness_symbol = HarnessSymbol::try_from(stem)?;
    let module_symbol = ModuleSymbol::try_from(module.clone())?;
    let path = format!("{module_symbol}::{harness_symbol}");
    let options = adapter_options(&path, request.unwind, request.solver, false);
    let mut proof = generate_caller_text_proof_harness(
        &request.text_type,
        &request.payload_class,
        request.solver.as_str(),
        options.clone(),
    )?;
    proof.source.contents = proof
        .source
        .contents
        .replace("pub mod caller_text {", &format!("pub mod {module} {{"));
    syn::parse_file(&proof.source.contents)
        .map_err(|_| CallerTextKaniGenerationError::InvalidSource)?;

    let caller_claim = proof.identity;
    let identity = CallerTextObligationIdentity {
        caller_claim,
        ceilings: request.ceilings,
        module_symbol,
        harness_symbol,
        solver: request.solver,
        unwind: request.unwind,
        options,
    };
    let record = caller_text_record(&identity, &proof.source)?;
    Ok(KaniCallerTextObligationHarness {
        identity,
        rust: proof.source,
        record,
    })
}

fn caller_text_record(
    identity: &CallerTextObligationIdentity,
    source: &Artifact,
) -> Result<Artifact, CallerTextKaniGenerationError> {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Record<'a> {
        identity: &'a CallerTextObligationIdentity,
        rust_path: &'a str,
    }
    let mut contents = serde_json::to_string(&Record {
        identity,
        rust_path: &source.path,
    })
    .map_err(|_| CallerTextKaniGenerationError::RecordSerialization)?;
    contents.push('\n');
    Ok(Artifact::new(
        format!("kani-obligations/{}.json", identity.harness_symbol),
        contents,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_exact::TextProfile;
    use std::{num::NonZeroU64, time::Duration};

    use crate::oracle::caller_text::{generate_caller_text_oracle, CallerTextAdmissionRequest};

    fn request(profile: TextProfile, min: u64, max: u64) -> CallerTextKaniRequest {
        CallerTextKaniRequest {
            text_type: TextType::new(min, max, profile).expect("valid type"),
            payload_class: crate::oracle::caller_text::CALLER_TEXT_PAYLOAD_CLASS.to_owned(),
            ceilings: ProofCeilings {
                memory_bytes: NonZeroU64::new(1_073_741_824).expect("nonzero"),
                wall_clock: Duration::from_secs(120),
            },
            solver: KaniSolver::Cadical,
            unwind: 8,
        }
    }

    #[trace("FR-035-AC-4", "FR-017-AC-26")]
    #[test]
    fn caller_harness_identity_is_profile_bound_and_executor_ready() {
        let harness = generate_caller_text_kani_harness(request(TextProfile::Nfc, 1, 1))
            .expect("supported caller proof");
        assert_eq!(
            harness.identity.caller_claim.profile,
            crate::oracle::caller_text::CallerTextProfile::Nfc
        );
        assert_eq!(
            (
                harness.identity.caller_claim.min,
                harness.identity.caller_claim.max
            ),
            (1, 1)
        );
        assert_eq!(
            harness.identity.caller_claim.payload_class,
            crate::oracle::caller_text::CALLER_TEXT_PAYLOAD_CLASS
        );
        assert_eq!(
            harness.identity.harness_path().to_string(),
            "caller_text_nfc_1_1::caller_text_admission_agrees"
        );
        assert!(harness.identity.options.windows(2).any(|pair| pair
            == [
                "--harness",
                "caller_text_nfc_1_1::caller_text_admission_agrees"
            ]));
        assert_eq!(
            harness.record.path,
            "kani-obligations/caller_text_admission_agrees.json"
        );
        assert!(harness
            .record
            .contents
            .contains("caller_text_nfc_1_1::caller_text_admission_agrees"));
    }

    #[trace("FR-035-AC-1", "FR-035-AC-4", "FR-035-AC-5", "TC-050")]
    #[test]
    fn proof_harness_embeds_the_public_generated_oracle_artifact() {
        let text_type = TextType::new(1, 1, TextProfile::Nfc).expect("valid type");
        let oracle = generate_caller_text_oracle(CallerTextAdmissionRequest { text_type });
        let harness = generate_caller_text_kani_harness(request(TextProfile::Nfc, 1, 1))
            .expect("generated proof harness");
        let embedded = oracle.source.contents.trim();
        assert!(harness.rust.contents.contains(embedded));
        assert_eq!(
            harness
                .rust
                .contents
                .matches("pub fn admit_caller_text(")
                .count(),
            1
        );
    }

    #[trace("FR-035-AC-4", "FR-017-AC-26")]
    #[test]
    fn twelve_profiles_and_bounds_have_distinct_harnesses_and_caller_claims() {
        let mut names = std::collections::BTreeSet::new();
        for profile in TextProfile::ALL {
            for (min, max) in [(0, 8), (1, 1)] {
                let harness = generate_caller_text_kani_harness(request(profile, min, max))
                    .expect("supported profile and bounds");
                assert!(names.insert(harness.identity.harness_path().to_string()));
                assert_eq!(
                    harness.identity.caller_claim.profile,
                    crate::oracle::caller_text::caller_profile(profile)
                );
                assert_eq!(
                    (
                        harness.identity.caller_claim.min,
                        harness.identity.caller_claim.max
                    ),
                    (min, max)
                );
                assert_eq!(harness.rust.contents.matches("kani::cover!(").count(), 1);
                assert!(harness.rust.contents.ends_with("}\n}\n"));
                assert_eq!(harness.identity.solver.as_str(), "cadical");
                assert!(harness.identity.options.contains(&"--harness".to_owned()));
            }
        }
        assert_eq!(names.len(), 12);
    }

    #[trace("FR-035-AC-6")]
    #[test]
    fn unsupported_payload_class_returns_typed_refusal_without_harness() {
        let mut request = request(TextProfile::Nfc, 1, 1);
        request.payload_class = "arbitrary-utf8".to_owned();
        assert!(matches!(
            generate_caller_text_kani_harness(request),
            Err(CallerTextKaniGenerationError::Proof(
                CallerTextProofRefusal::UnsupportedPayloadClass
            ))
        ));
    }
}
