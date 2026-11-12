//! Checked scalar plans. Execution and the QSL facade call belong to the driver.

use qsl_replay::{
    Domain, NativeOutcome, OperandIdentity, OperatorClaim, OperatorIdentity, OperatorParityReport,
    ParityArgument, ParityPreimage, ReplayLimits, ReplayRequestWire, ReplaySource,
    RequestTargetWire, ScalarOperand, ScalarOperation,
};
use quire_contract_model::{CheckedPackageReadLimits, CheckedPackageV2, CheckedScalarOperandChild};
use quire_exact::{Integer, IntegerInterval};

use super::{
    composite_builder::{origin, wire_node, CompositeBuildError, OriginalReplayContext},
    function::{LockedSource, ReplayInputs},
    scalar::decode_scalar_falsification,
    witness::DecodeFailure,
};
use crate::core::{
    artifact::Artifact,
    scalar_authority::{ScalarAuthorityError, ScalarReplayBody},
};
use crate::kani::identity::{KaniScalarObligationHarness, ScalarObligationIdentity};
use crate::kani::terminal::ScalarSettlement;

/// A typed pre-dispatch or report-binding refusal; this never fabricates a QSL settlement.
#[derive(Debug)]
pub enum ScalarBuildError {
    /// Generation cannot supply authentic complete positional metadata.
    Authority(ScalarAuthorityError),
    /// The supplied identity or full body changed under the original node identity.
    BodyMismatch,
    /// The independent original package/source/context does not match the proving authority.
    Context(Box<CompositeBuildError>),
    /// The request did not retain the exact generated artifact package.
    Artifacts,
    /// The original staging layout cannot be represented without a reserved/colliding path.
    Staging,
    /// Assertion playback does not match the selected scalar schema.
    Playback(DecodeFailure),
    /// A refusal on a generated family with no operator-parity route. It keeps the earlier
    /// refusal's cause and, like the absent capability itself, yields no terminal record.
    UnavailableFamily(Box<ScalarBuildError>),
    /// The QSL-owned structural mint refused the original claim.
    Identity(qsl_replay::IdentityEncodeError),
    /// This generated family has no integer operator-parity observation route.
    Operation,
    /// The driver's authenticated execution assertion differs from the original staged context.
    Observation,
    /// The genuine QSL report differs from the complete actual-sent claim.
    Report,
    /// Actual facade wire or replay budget differs from the independently retained context.
    SentContext,
}

/// Immutable original scalar source/package context, with no function-contract coercion.
#[derive(Debug)]
pub struct OriginalScalarContext {
    context: OriginalReplayContext,
}

impl OriginalScalarContext {
    /// Retain independently held source bytes, the admitted package and its same-emission raw document.
    /// Request construction requires the raw document to equal the source recompilation bytes.
    ///
    /// # Errors
    /// Refuses absent source/function or inconsistent original dependency context.
    pub fn new(
        original: CheckedPackageV2,
        raw_package: qsl_replay::ReplayPackage,
        original_source: &LockedSource,
        inputs: ReplayInputs,
        function: &str,
        read_limits: CheckedPackageReadLimits,
    ) -> Result<Self, ScalarBuildError> {
        OriginalReplayContext::new(
            original,
            raw_package,
            original_source,
            inputs,
            function,
            read_limits,
        )
        .map(|context| Self { context })
        .map_err(|cause| ScalarBuildError::Context(Box::new(cause)))
    }

    /// Decode an assertion playback against the original scalar identity, authenticate its
    /// complete body and artifact package, then prepare an operator-level request. Repeated
    /// parameter draws remain separate positions; no function argument map is constructed.
    ///
    /// # Errors
    /// Returns typed missing/stale authority, playback, context or artifact refusal before replay.
    pub fn plan(
        &self,
        harness: &KaniScalarObligationHarness,
        identity: &ScalarObligationIdentity,
        body: &ScalarReplayBody,
        artifacts: &[Artifact],
        playback: &str,
    ) -> Result<ScalarReplayPlan, ScalarBuildError> {
        // Capability availability is the last setup step: an absent capability never hides an
        // earlier context, byte-ceiling, adapter or domain refusal.
        let original = match harness.scalar_replay_body() {
            Ok(original) => Some(original),
            Err(ScalarAuthorityError::Unavailable) => None,
            Err(cause) => return Err(ScalarBuildError::Authority(cause)),
        };
        // Every refusal on a family without a parity route yields no terminal record.
        let available = original.is_some();
        let refuse = |cause: ScalarBuildError| {
            if available {
                cause
            } else {
                ScalarBuildError::UnavailableFamily(Box::new(cause))
            }
        };
        if identity != &harness.identity || original.is_some_and(|retained| retained != body) {
            return Err(refuse(ScalarBuildError::BodyMismatch));
        }
        if available {
            let original_package = harness
                .scalar_original_package()
                .map_err(ScalarBuildError::Authority)?;
            if original_package.as_ref() != self.context.original.as_ref() {
                return Err(ScalarBuildError::BodyMismatch);
            }
        }
        // A harness built outside routed generation retains no artifact package to compare.
        if (available || harness.retains_artifact_package())
            && !harness.matches_artifact_package(artifacts)
        {
            return Err(refuse(ScalarBuildError::Artifacts));
        }
        let parsed = crate::kani::output::playback::parse_assertion_playback(
            playback,
            identity.harness_symbol.as_str(),
            identity.module_symbol.as_str(),
            super::scalar::SCALAR_PLAYBACK_BYTES,
            identity.arguments.len() + identity.guard_inputs.len(),
        )
        .map_err(|cause| refuse(ScalarBuildError::Playback(DecodeFailure::from(cause))))?;
        let decoded = decode_scalar_falsification(identity, &parsed);
        if !available {
            return Err(match decoded {
                Err(failure) if failure.code != "cg_scalar_witness_operation_unsupported" => {
                    refuse(ScalarBuildError::Playback(failure))
                }
                _ => ScalarBuildError::Authority(ScalarAuthorityError::Unavailable),
            });
        }
        let values = decoded.map_err(ScalarBuildError::Playback)?;
        let original_package = harness
            .scalar_original_package()
            .map_err(ScalarBuildError::Authority)?;
        let context_error = |cause| ScalarBuildError::Context(Box::new(cause));
        let node = wire_node(original_package, &body.node).map_err(context_error)?;
        let occurrence = origin(&body.occurrence);
        let operands = body
            .operands
            .iter()
            .zip(&values)
            .map(|(operand, value)| {
                let range = IntegerInterval::spanning(
                    Integer::from(operand.range.lower),
                    Integer::from(operand.range.upper),
                );
                let identity = match &operand.child {
                    CheckedScalarOperandChild::GraphChild(child) => OperandIdentity::GraphChild(
                        wire_node(original_package, child).map_err(context_error)?,
                    ),
                    CheckedScalarOperandChild::InlineLiteral {
                        application,
                        occurrence,
                        ordinal,
                    } => {
                        if application != &body.node
                            || occurrence != &body.occurrence
                            || *ordinal != operand.ordinal
                        {
                            return Err(ScalarBuildError::BodyMismatch);
                        }
                        OperandIdentity::InlineLiteral
                    }
                };
                Ok(ScalarOperand {
                    value: *value,
                    range,
                    identity,
                })
            })
            .collect::<Result<Vec<_>, ScalarBuildError>>()?;
        let operation = match (identity.operation_identity.as_str(), operands.as_slice()) {
            ("quire.op.integer.add", [left, right]) => ScalarOperation::Add {
                left: left.clone(),
                right: right.clone(),
            },
            ("quire.op.integer.sub", [left, right]) => ScalarOperation::Subtract {
                left: left.clone(),
                right: right.clone(),
            },
            ("quire.op.integer.mul", [left, right]) => ScalarOperation::Multiply {
                left: left.clone(),
                right: right.clone(),
            },
            ("quire.op.integer.negate", [operand]) => ScalarOperation::Negate {
                operand: operand.clone(),
            },
            _ => return Err(ScalarBuildError::Operation),
        };
        let preimage = ParityPreimage {
            node: node.clone(),
            occurrence: occurrence.clone(),
            obligation_kind: body.kind.as_str().to_owned(),
            arguments: operands
                .iter()
                .map(|operand| ParityArgument {
                    identity: operand.identity.clone(),
                    domain: Domain::Range(operand.range.clone()),
                })
                .collect(),
        };
        let obligation =
            qsl_replay::parity_obligation(&preimage).map_err(ScalarBuildError::Identity)?;
        let wire = self.context.inputs.wire(
            self.context.raw_package.clone(),
            RequestTargetWire::function(
                self.context.selection.clone(),
                qsl_replay::OccurrenceKey::new(node.clone(), occurrence.clone()),
            ),
            ReplaySource::Input(Vec::new()),
            obligation,
            &[],
        );
        self.context
            .check_recompiled_context(&wire)
            .map_err(context_error)?;
        let staged_entries = crate::kani::run::scalar::scalar_staged_entries(harness, artifacts)
            .map_err(|_| ScalarBuildError::Staging)?;
        let limits = harness
            .original_scalar_limits()
            .map_err(|_| ScalarBuildError::Observation)?;
        if self.context.inputs.accounting_limits != limits {
            return Err(ScalarBuildError::Observation);
        }
        Ok(ScalarReplayPlan {
            wire,
            node,
            occurrence,
            kind: body.kind.as_str().to_owned(),
            operation,
            result_range: IntegerInterval::spanning(
                Integer::from(body.result_range.lower),
                Integer::from(body.result_range.upper),
            ),
            limits,
            replay_limits: self.context.inputs.replay_limits,
            staged_entries,
            identity: harness.identity.clone(),
            harness: harness.rust.clone(),
            record: harness.record.clone(),
            operands: values,
            original_inputs: std::sync::Arc::clone(&self.context.inputs),
            original_package: std::sync::Arc::clone(original_package),
        })
    }
}

/// Driver-owned execution assertion, supplied independently after executing the actual staged
/// artifact. The driver is trusted to execute honestly: these fields are checked ties, not
/// cryptographic or language-level proof of execution. This is not a Kani receipt or backend proof.
#[derive(Clone, Debug)]
pub struct ScalarDriverObservation {
    /// The complete staged artifact path/byte set actually executed, including harness and record.
    pub staged_entries: Vec<(String, Vec<u8>)>,
    /// Actual selected execution identity.
    pub identity: ScalarObligationIdentity,
    /// Actual persisted harness entry and record executed.
    pub harness: Artifact,
    /// Actual persisted proving record.
    pub record: Artifact,
    /// Actual operand vector in symbolic draw order.
    pub operands: Vec<i64>,
    /// Actual meter limits installed in that invocation.
    pub limits: quire_exact::ScalarLimits,
    /// Independently retained original source bytes for that invocation.
    pub source: LockedSource,
    /// Admitted original package for the invocation, including its full context.
    pub package: CheckedPackageV2,
    /// Actual typed native observation, not an expected or regenerated result.
    pub outcome: NativeOutcome,
}

/// Prepared scalar replay, before a driver has authenticated a native observation.
pub struct ScalarReplayPlan {
    wire: ReplayRequestWire,
    node: quire_exact::NodeKey,
    occurrence: qsl_replay::Origin,
    kind: String,
    operation: ScalarOperation,
    result_range: IntegerInterval,
    limits: quire_exact::ScalarLimits,
    replay_limits: ReplayLimits,
    staged_entries: Vec<(String, Vec<u8>)>,
    identity: ScalarObligationIdentity,
    harness: Artifact,
    record: Artifact,
    operands: Vec<i64>,
    original_inputs: std::sync::Arc<ReplayInputs>,
    original_package: std::sync::Arc<CheckedPackageV2>,
}

impl ScalarReplayPlan {
    /// The decoded vector to execute independently; repeated parameter draws stay independent.
    pub fn operands(&self) -> &[i64] {
        &self.operands
    }
    /// Exact original staged paths/bytes, including harness and record. A driver may stage
    /// these bytes, execute them, then independently capture the actual staged entries.
    pub fn staged_entries(&self) -> &[(String, Vec<u8>)] {
        &self.staged_entries
    }
    /// Original meter limits, supplied by the renderer rather than a replay budget.
    pub const fn scalar_limits(&self) -> quire_exact::ScalarLimits {
        self.limits
    }
    /// Check the driver's execution assertion against every original retained tie.
    /// The caller's honest execution assertion is an explicit trust boundary. This method
    /// executes nothing and makes no QSL call; a bare native outcome cannot complete a plan.
    ///
    /// # Errors
    /// Any changed/missing staged context, identity, harness, record, operand or limit refuses.
    pub fn complete(
        self,
        observed: ScalarDriverObservation,
    ) -> Result<CompletedScalarReplay, ScalarBuildError> {
        let mut actual_entries = observed.staged_entries;
        actual_entries.sort_by(|left, right| left.0.cmp(&right.0));
        let mut expected_entries = self.staged_entries;
        expected_entries.sort_by(|left, right| left.0.cmp(&right.0));
        if actual_entries != expected_entries
            || observed.identity != self.identity
            || observed.harness != self.harness
            || observed.record != self.record
            || observed.operands != self.operands
            || observed.limits != self.limits
            || observed.source != self.original_inputs.source
            || observed.package != *self.original_package
        {
            return Err(ScalarBuildError::Observation);
        }
        let claim = OperatorClaim {
            node: self.node,
            occurrence: self.occurrence,
            obligation_kind: self.kind,
            operation: self.operation,
            result_range: self.result_range,
            limits: self.limits,
            generated: observed.outcome,
        };
        let sent = claim.identity(self.wire.obligation_identity.clone());
        Ok(CompletedScalarReplay {
            wire: self.wire,
            claim,
            sent,
            replay_limits: self.replay_limits,
        })
    }
}

/// Completed plan carrying a checked driver observation. The driver calls QSL, then returns
/// its genuine report for full actual-sent claim binding on every outcome.
pub struct CompletedScalarReplay {
    wire: ReplayRequestWire,
    claim: OperatorClaim,
    sent: OperatorIdentity,
    replay_limits: ReplayLimits,
}
impl CompletedScalarReplay {
    /// Exact operator-level facade request; no call is performed inside CG.
    pub fn request(&self) -> (ReplayRequestWire, OperatorClaim, ReplayLimits) {
        (self.wire.clone(), self.claim.clone(), self.replay_limits)
    }
    /// Compare the entire genuine report claim before reading any terminal outcome, then return
    /// the report's result as the scalar settlement (FR-032-AC-20).
    /// # Errors
    /// Another actual-sent claim, including different native evidence or context, refuses.
    pub fn settlement<'r>(
        &self,
        actual_sent: &ScalarSentRequest,
        report: Option<&'r OperatorParityReport>,
    ) -> Result<ScalarSettlement<'r>, ScalarBuildError> {
        if actual_sent.replay_limits != self.replay_limits
            || !same_wire(&actual_sent.wire, &self.wire)
        {
            return Err(ScalarBuildError::SentContext);
        }
        let actual_claim = actual_sent
            .claim
            .identity(actual_sent.wire.obligation_identity.clone());
        let report = report.ok_or(ScalarBuildError::Report)?;
        if actual_claim != self.sent || report.claim() != &actual_claim {
            return Err(ScalarBuildError::Report);
        }
        Ok(ScalarSettlement::Parity(report.result()))
    }
}

impl ScalarBuildError {
    /// The settlement of a falsified run whose replay setup failed (FR-032-AC-24): `CgDefect`
    /// only for a failure CG raised on a supported scalar family; `None` for a named unavailable
    /// capability and for a binding mismatch, which must not write a record for another obligation.
    #[deny(
        clippy::wildcard_enum_match_arm,
        clippy::match_wildcard_for_single_variants
    )]
    pub fn setup_settlement(&self) -> Option<ScalarSettlement<'static>> {
        match self {
            Self::Authority(cause) => match cause {
                ScalarAuthorityError::Operation
                | ScalarAuthorityError::Ranges
                | ScalarAuthorityError::Record => Some(ScalarSettlement::CgDefect),
                ScalarAuthorityError::Unavailable
                | ScalarAuthorityError::Occurrence
                | ScalarAuthorityError::Operands(_)
                | ScalarAuthorityError::GeneratedArtifactChanged => None,
            },
            Self::UnavailableFamily(_) => None,
            Self::Playback(_) | Self::Staging | Self::Identity(_) => {
                Some(ScalarSettlement::CgDefect)
            }
            Self::BodyMismatch
            | Self::Context(_)
            | Self::Artifacts
            | Self::Operation
            | Self::Observation
            | Self::Report
            | Self::SentContext => None,
        }
    }
}

impl std::fmt::Debug for ScalarReplayPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScalarReplayPlan")
            .field("operands", &self.operands)
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl std::fmt::Debug for CompletedScalarReplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompletedScalarReplay")
            .field("sent", &self.sent)
            .finish_non_exhaustive()
    }
}
impl std::fmt::Display for ScalarBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scalar replay refusal: {self:?}")
    }
}
impl std::error::Error for ScalarBuildError {}

/// The driver's actual facade arguments, captured at dispatch, independently of the returned
/// report. The driver asserts these were the actual arguments sent; CG checks every wire member
/// and replay budget against its original context, then full actual-sent/report claim identity.
/// This record is an honest-driver assertion, not an unforgeable execution receipt.
pub struct ScalarSentRequest {
    /// Actual wire passed to the public operator facade.
    pub wire: ReplayRequestWire,
    /// Actual scalar claim passed to the facade.
    pub claim: OperatorClaim,
    /// Actual final facade replay budget (not part of OperatorIdentity).
    pub replay_limits: ReplayLimits,
}

pub(crate) fn same_wire(left: &ReplayRequestWire, right: &ReplayRequestWire) -> bool {
    // Exhaustive destructuring deliberately makes an upstream wire extension a compile error,
    // requiring its original-context tie to be considered rather than silently omitted.
    let ReplayRequestWire {
        package,
        profile_selections,
        dependencies,
        replay_target,
        source,
        obligation_identity,
        backend,
        state_environment,
        accounting_limits,
        stage_limits,
        supplied_limits,
        object_environment,
        declared_domains,
        byte_provision,
        documents,
    } = left;
    package == &right.package
        && profile_selections == &right.profile_selections
        && dependencies == &right.dependencies
        && replay_target == &right.replay_target
        && source == &right.source
        && obligation_identity == &right.obligation_identity
        && backend == &right.backend
        && state_environment == &right.state_environment
        && accounting_limits == &right.accounting_limits
        && stage_limits == &right.stage_limits
        && supplied_limits == &right.supplied_limits
        && object_environment == &right.object_environment
        && declared_domains == &right.declared_domains
        && byte_provision == &right.byte_provision
        && documents == &right.documents
}
