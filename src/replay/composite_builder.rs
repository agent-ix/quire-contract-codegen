// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Original-package construction of verified structural equality requests (FR-033).
//!
//! This boundary authenticates the retained source and checked package and constructs positional
//! O-09 arguments. It binds the caller's retained proof-content identity; it does not authenticate
//! a generated artifact, run a native observation, or establish a backend proof strength.

use std::{collections::BTreeMap, fmt};

use qsl_replay::{
    compile_package, parity_obligation, settle_verified_shadow, BoundEntries, ByteDigest,
    CompositeEvidence, CompositeIdentity, CompositeParityClaim, DependencyInput, DigestDomain,
    DigestRecord, Domain, DomainKey, EqualityOperator, Identifier, IdentityEncodeError, NoPayload,
    OccurrenceKey, OperandIdentity, Origin, ParityArgument, ParityPreimage, ProofBound,
    QualifiedName, ReplayRefusal, ReplayRequest, ReplayRequestRefusal, ReplayRequestWire,
    ReplaySource, ReplayTarget, ReplayTargetMembers, RequestTargetWire, Role, VerifiedShadow,
    VerifiedShadowReport, WireNodeId, WitnessEnvelope, WitnessPacket, WitnessRefusal,
};
use quire_contract_model::{
    CheckedCompositeOperandDomain, CheckedCompositeOperandError, CheckedCompositeOperands,
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageEvidence,
    CheckedPackageIncomplete, CheckedPackageReadLimits, CheckedPackageRefusal, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedScalarOperandChild, CheckedSourceRef,
};

use super::{
    composite::{verified_shadow_terminal_value, CompositeReportError, VerifiedShadowSettlement},
    function::{DependencyLockError, ReplayInputs},
};

/// A pre-invocation construction refusal. No QSL report or terminal value exists on this path.
#[derive(Debug)]
pub enum CompositeBuildError {
    /// The retained unit source reference or bytes differ from the original package's lock.
    SourceMismatch,
    /// The supplied bytes recompile to a different semantic package.
    PackageMismatch,
    /// An admitted package identity cannot be represented in the replay identity type.
    PackageIdentity,
    /// Retained or recompiled context differs from the original lock, graph or source map.
    ContextMismatch,
    /// Original dependency packages are not retained by this constructor.
    ImportedContextUnsupported,
    /// The selected function identifier is invalid.
    InvalidFunction,
    /// The selected function is absent from the original package.
    UnknownFunction,
    /// An admitted node identity cannot be represented in the replay identity type.
    NodeIdentity,
    /// The lock's dependency inputs are not admitted.
    Dependencies(DependencyLockError),
    /// QSL refused recompilation of the retained source.
    Recompile(Box<ReplayRefusal>),
    /// QSL's public request decoder refused the original wire before recompilation.
    RequestDecode(Box<ReplayRequestRefusal>),
    /// IR refused the recompiled checked package.
    PackageRead(CheckedPackageRefusal),
    /// IR reached a checked-package read ceiling.
    PackageReadLimit(CheckedPackageIncomplete),
    /// The original application has no complete admitted structural equality operand projection.
    Operands(Box<CheckedCompositeOperandError>),
    /// The requested equality operator differs from the original application's operator.
    OperatorMismatch {
        /// The operator the caller requested.
        requested: EqualityOperator,
        /// The operator carried by the original node.
        actual: EqualityOperator,
    },
    /// The selected node has no supported structural equality operation.
    UnsupportedNodeOperation,
    /// The accessor returned a shape outside this graph-child constructor.
    OperandShape,
    /// The owning O-09 encoder refused the preimage.
    Identity(IdentityEncodeError),
    /// QSL refused the producer packet before request construction.
    PacketAdmission(WitnessRefusal),
    /// An admitted packet belongs to another checked parity claim.
    PacketMismatch(CompositePacketMember),
    /// A genuine QSL report did not bind to the request actually sent.
    Report(CompositeReportError),
}

/// The checked parity claim member that did not match an independently supplied packet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositePacketMember {
    /// The packet selected a different function.
    Target,
    /// The packet selected another application occurrence.
    Occurrence,
    /// The packet named another application node.
    Subject,
    /// The packet belongs to another checked package.
    Package,
    /// The packet's source references differ from the checked package context.
    SourceDigests,
    /// The packet's profile selections differ from the request context.
    Profiles,
    /// The packet's accounting limits differ from the checked claim context.
    Limits,
    /// The packet's declared domains differ from the checked claim context.
    Domains,
    /// The packet names a different claim producer.
    Backend,
    /// Parity requires an input-source packet.
    SourceArm,
    /// The packet identifies a different parity obligation.
    Obligation,
}

impl fmt::Display for CompositeBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceMismatch => f.write_str("original source does not match package lock"),
            Self::PackageMismatch => {
                f.write_str("recompiled package differs from original package")
            }
            Self::PackageIdentity => f.write_str("package identity is not representable"),
            Self::ContextMismatch => {
                f.write_str("recompiled context differs from original context")
            }
            Self::ImportedContextUnsupported => {
                f.write_str("original imported context is unsupported")
            }
            Self::InvalidFunction => f.write_str("invalid function identifier"),
            Self::UnknownFunction => f.write_str("function is absent from original package"),
            Self::NodeIdentity => f.write_str("node identity is not representable"),
            Self::Dependencies(cause) => cause.fmt(f),
            Self::Recompile(cause) => cause.fmt(f),
            Self::RequestDecode(cause) => cause.fmt(f),
            Self::PackageRead(cause) => {
                f.write_str("checked-package read refused")?;
                if let Some(path) = &cause.path {
                    write!(f, " at {path}")?;
                }
                Ok(())
            }
            Self::PackageReadLimit(cause) => write!(
                f,
                "checked-package read reached ceiling {} after consuming {} units",
                cause.limit, cause.consumed
            ),
            Self::Operands(cause) => cause.fmt(f),
            Self::OperatorMismatch { requested, actual } => {
                write!(
                    f,
                    "requested operator {requested:?} differs from original {actual:?}"
                )
            }
            Self::UnsupportedNodeOperation => f.write_str("unsupported original node operation"),
            Self::OperandShape => f.write_str("unsupported structural operand projection"),
            Self::Identity(cause) => cause.fmt(f),
            Self::PacketAdmission(cause) => cause.fmt(f),
            Self::PacketMismatch(member) => {
                write!(
                    f,
                    "parity packet does not match the checked claim: {member:?}"
                )
            }
            Self::Report(cause) => cause.fmt(f),
        }
    }
}

impl std::error::Error for CompositeBuildError {}

/// Immutable original proving context for admitted graph-child structural equality routes.
///
/// The caller supplies the original admitted package and retained source, dependencies, limits
/// and canonical proof-content identity. The content identity is a binding input, not independent
/// authentication of a generated artifact. No native/falsified production route is exposed here.
#[derive(Debug)]
pub struct OriginalCompositeEqContext {
    original: CheckedPackageV2,
    node_positions: BTreeMap<CheckedNodeId, usize>,
    inputs: ReplayInputs,
    dependencies: DependencyInput,
    selection: QualifiedName,
    package_id: DigestRecord,
    content_identity: DigestRecord,
    read_limits: CheckedPackageReadLimits,
}

struct SelectedParityClaim {
    operands: CheckedCompositeOperands,
    preimage: ParityPreimage,
    obligation: qsl_replay::ObligationIdentity,
    claim: CompositeParityClaim,
}

impl OriginalCompositeEqContext {
    /// Retain the original context and require its exact unit source reference and bytes.
    ///
    /// [`Self::request`] recompiles using the constructed request's admitted stage limits and
    /// requires the complete original package/context before returning a request for invocation.
    /// Imported contexts refuse until CG retains their admitted dependency packages from the
    /// original compile; this constructor does not compile or resolve libraries.
    ///
    /// # Errors
    /// Returns a typed source, package identity, dependency or function-selection refusal. These
    /// prechecks create neither a parity report nor a terminal value. Recompile/read/context
    /// refusals are returned later by [`Self::request`].
    // The 192-byte refusal is retained inline to preserve the public PackageRead payload.
    #[allow(clippy::result_large_err)]
    pub fn new(
        original: CheckedPackageV2,
        inputs: ReplayInputs,
        function: &str,
        content_identity: DigestRecord,
        read_limits: CheckedPackageReadLimits,
    ) -> Result<Self, CompositeBuildError> {
        let identifier =
            Identifier::new(function).map_err(|_| CompositeBuildError::InvalidFunction)?;
        let selection = QualifiedName::new(vec![identifier])
            .map_err(|_| CompositeBuildError::InvalidFunction)?;
        let source = &inputs.source;
        let source_ref = CheckedSourceRef {
            authority: source.authority.clone().into_boxed_str(),
            identity: source.identity.clone().into_boxed_str(),
            digest_domain: source.digest().domain().as_str().into(),
            digest: source.digest().hex().into_boxed_str(),
        };
        if !original.lock().sources.contains(&source_ref) {
            return Err(CompositeBuildError::SourceMismatch);
        }
        let (inputs, dependencies) = inputs.admit().map_err(CompositeBuildError::Dependencies)?;
        let selections = &original.lock().dependency_selections;
        if selections.len() != inputs.dependencies.len()
            || selections
                .iter()
                .zip(&inputs.dependencies)
                .any(|(original, retained)| {
                    original.identity.as_ref() != retained.identity
                        || original.package_id.domain.as_ref()
                            != retained.package_id.domain().as_str()
                        || original.package_id.digest.as_ref() != retained.package_id.hex()
                })
        {
            return Err(CompositeBuildError::ContextMismatch);
        }
        if !selections.is_empty() {
            return Err(CompositeBuildError::ImportedContextUnsupported);
        }
        if original.package_id().domain.as_ref() != DigestDomain::PackageSemanticV2.as_str() {
            return Err(CompositeBuildError::PackageIdentity);
        }
        let package_id = DigestRecord::mint(
            DigestDomain::PackageSemanticV2,
            ByteDigest::from_hex(&original.package_id().digest)
                .map_err(|_| CompositeBuildError::PackageIdentity)?
                .as_bytes(),
        );
        original
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "function"
                    && node.declaration.as_ref().is_some_and(|declaration| {
                        declaration.qualified_name.len() == 1
                            && declaration.qualified_name[0].as_ref() == function
                    })
            })
            .ok_or(CompositeBuildError::UnknownFunction)?;
        let mut node_positions = BTreeMap::new();
        for (position, node) in original.graph().nodes.iter().enumerate() {
            node_positions
                .entry(node.node_id.clone())
                .or_insert(position);
        }
        Ok(Self {
            original,
            node_positions,
            inputs,
            dependencies,
            selection,
            package_id,
            content_identity,
            read_limits,
        })
    }

    /// Build O-09 from the original Eq or Ne application's two authentic positional operands.
    ///
    /// Parameter positions retain only their Node bounds. Population metadata is excluded from
    /// this request. Literal graph children retain empty Bounds and hence singleton source values;
    /// QSL derives their values and domains from the original source and owns selected-function
    /// body and occurrence membership at invocation, returning a binding-checked refusal report.
    /// Projection/encoder refusals precede recompilation: public QSL stage-limit decoding requires
    /// the genuine O-09 wire, so this boundary does not authenticate context before projection.
    /// The supplied work ceiling
    /// bounds the IR operand projection. A requested operator absent from the original node,
    /// unsupported operations and inline operands refuse.
    ///
    /// # Errors
    /// Returns a typed original node/occurrence, operand, encoder, request decode, recompile or
    /// original-context refusal, with no report.
    // The 192-byte refusal is retained inline to preserve the public PackageRead payload.
    #[allow(clippy::result_large_err)]
    fn selected_claim(
        &self,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        operator: EqualityOperator,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<SelectedParityClaim, CompositeBuildError> {
        let operands = self
            .original
            .composite_application_operands(node, occurrence, work_limit)
            .map_err(|cause| CompositeBuildError::Operands(Box::new(cause)))?;
        // IR's operand projection validates Eq/Ne but does not return a typed operator yet.
        // Read the selected node identity until the IR API carries that operation (IR-803).
        let actual = self
            .node_positions
            .get(node)
            .and_then(|position| self.original.graph().nodes.get(*position))
            .and_then(|entry| entry.body.pointer("/operation/identity"))
            .and_then(serde_json::Value::as_str);
        let actual = node_equality_operator(actual)?;
        if actual != operator {
            return Err(CompositeBuildError::OperatorMismatch {
                requested: operator,
                actual,
            });
        }
        // Keep the public PackageRead error inline; its refusal payload is 192 bytes.
        #[allow(clippy::result_large_err)]
        let arguments = operands.operands.iter().map(|operand| {
            let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                return Err(CompositeBuildError::OperandShape);
            };
            let child = wire_node(child)?;
            let bounds = match &operand.domain {
                CheckedCompositeOperandDomain::Literal => Vec::new(),
                CheckedCompositeOperandDomain::Parameter { .. } => harness_bounds.iter()
                    .filter(|bound| matches!(bound.domain(), DomainKey::Node { node, .. } if *node == child))
                    .cloned().collect(),
            };
            Ok(ParityArgument {
                identity: OperandIdentity::GraphChild(child),
                domain: Domain::Bounds(BoundEntries::new(bounds).map_err(CompositeBuildError::Identity)?),
            })
        }).collect::<Result<Vec<_>, _>>()?;
        if arguments.len() != 2 {
            return Err(CompositeBuildError::OperandShape);
        }
        let node = wire_node(node)?;
        let occurrence = origin(occurrence);
        let preimage = ParityPreimage {
            node,
            occurrence: occurrence.clone(),
            obligation_kind: "bounded_shadow".to_owned(),
            arguments,
        };
        let obligation = parity_obligation(&preimage).map_err(CompositeBuildError::Identity)?;
        let claim = CompositeParityClaim {
            node,
            occurrence,
            operator,
            obligation_kind: preimage.obligation_kind.clone(),
            harness_bounds: harness_bounds
                .iter()
                .filter(|bound| matches!(bound.domain(), DomainKey::Node { .. }))
                .cloned()
                .collect(),
            limits: self.inputs.accounting_limits,
            content_identity: self.content_identity,
        };
        Ok(SelectedParityClaim {
            operands,
            preimage,
            obligation,
            claim,
        })
    }

    /// Complete an input-arm packet from an original checked parity claim, before request intake.
    ///
    /// # Errors
    /// Returns the original checked-claim projection or identity refusal.
    #[allow(clippy::result_large_err)]
    pub fn parity_packet(
        &self,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        operator: EqualityOperator,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<WitnessPacket<NoPayload>, CompositeBuildError> {
        let selected =
            self.selected_claim(node, occurrence, operator, harness_bounds, work_limit)?;
        Ok(WitnessPacket {
            obligation_identity: Some(*selected.obligation.as_bytes()),
            occurrence_key: Some(OccurrenceKey::new(
                selected.preimage.node,
                selected.preimage.occurrence.clone(),
            )),
            clause_node: Some(selected.preimage.node),
            replay_target: ReplayTargetMembers::function(self.selection.clone()),
            package_id: Some((
                Some(self.package_id.domain().as_str().to_owned()),
                self.package_id.hex(),
            )),
            source_digests: Some(vec![self.inputs.source.wire()]),
            profile_selections: Some(Vec::new()),
            run_limits: Some(self.inputs.accounting_limits),
            declared_domains: Some(self.inputs.declared_domains.clone()),
            backend: Some("quire-contract-codegen".to_owned()),
            trace_position: Some(None),
            source: Some(ReplaySource::Input(Vec::new())),
            family_payload: Some(NoPayload),
        })
    }

    /// Build a parity request only from an admitted packet bound to this checked claim.
    ///
    /// # Errors
    /// Returns a typed packet admission, claim-binding, original projection or replay setup
    /// refusal before any report or terminal value exists.
    #[allow(clippy::result_large_err)]
    pub fn request_from_packet(
        &self,
        packet: WitnessPacket<NoPayload>,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        operator: EqualityOperator,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<OriginalCompositeEqRequest, CompositeBuildError> {
        let selected =
            self.selected_claim(node, occurrence, operator, harness_bounds, work_limit)?;
        let envelope = WitnessEnvelope::reconstruct(packet, self.inputs.replay_limits)
            .map_err(CompositeBuildError::PacketAdmission)?;
        if envelope.replay_target() != &ReplayTarget::function(self.selection.clone(), None) {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Target,
            ));
        }
        if envelope.occurrence_key()
            != &OccurrenceKey::new(selected.preimage.node, selected.preimage.occurrence.clone())
        {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Occurrence,
            ));
        }
        if envelope.clause_node() != selected.preimage.node {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Subject,
            ));
        }
        if envelope.package_id() != self.package_id {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Package,
            ));
        }
        if envelope.to_packet().source_digests != Some(vec![self.inputs.source.wire()]) {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::SourceDigests,
            ));
        }
        if !envelope.profile_selections().is_empty() {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Profiles,
            ));
        }
        if envelope.run_limits() != self.inputs.accounting_limits {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Limits,
            ));
        }
        if envelope.declared_domains() != self.inputs.declared_domains.as_slice() {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Domains,
            ));
        }
        if envelope.backend().as_str() != "quire-contract-codegen" {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Backend,
            ));
        }
        if !matches!(envelope.source(), ReplaySource::Input(_)) {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::SourceArm,
            ));
        }
        if envelope.obligation_identity() != selected.obligation {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Obligation,
            ));
        }
        let ReplayTarget::Declaration { name, .. } = envelope.replay_target() else {
            return Err(CompositeBuildError::PacketMismatch(
                CompositePacketMember::Target,
            ));
        };
        let wire = self.inputs.wire(
            envelope.package_id(),
            RequestTargetWire::function(name.clone(), envelope.occurrence_key().clone()),
            envelope.source().clone(),
            *envelope.obligation_identity().as_bytes(),
            &[],
        );
        self.check_recompiled_context(&wire)?;
        Ok(OriginalCompositeEqRequest {
            wire,
            claim: selected.claim,
            operands: selected.operands,
            preimage: selected.preimage,
            replay_limits: self.inputs.replay_limits,
        })
    }

    // The 192-byte refusal is retained inline to preserve the public PackageRead payload.
    #[allow(clippy::result_large_err)]
    fn check_recompiled_context(
        &self,
        wire: &ReplayRequestWire,
    ) -> Result<(), CompositeBuildError> {
        let request = ReplayRequest::decode(wire.clone(), self.inputs.replay_limits)
            .map_err(|cause| CompositeBuildError::RequestDecode(Box::new(cause)))?;
        let compiled = compile_package(
            self.inputs.source.source_identity(),
            &self.inputs.source.identity,
            &self.inputs.source.bytes,
            [],
            &self.dependencies,
            request.stage_limits().clone(),
            self.inputs.replay_limits,
        )
        .map_err(|cause| CompositeBuildError::Recompile(Box::new(cause)))?;
        if self.package_id != compiled.package_id() {
            return Err(CompositeBuildError::PackageMismatch);
        }
        let mut evidence = CheckedPackageEvidence::new();
        for feature in &self.original.lock().required_features {
            evidence.support_feature(feature.as_ref());
        }
        let recompiled = match CheckedPackageV2::read(compiled.bytes(), self.read_limits, &evidence)
        {
            CheckedPackageV2ReadResult::Admitted(package) => package,
            CheckedPackageV2ReadResult::Refused(cause) => {
                return Err(CompositeBuildError::PackageRead(cause))
            }
            CheckedPackageV2ReadResult::Incomplete(cause) => {
                return Err(CompositeBuildError::PackageReadLimit(cause))
            }
        };
        if self.original != *recompiled {
            return Err(CompositeBuildError::ContextMismatch);
        }
        Ok(())
    }
}

// The 192-byte refusal is retained inline to preserve the public PackageRead payload.
#[allow(clippy::result_large_err)]
fn wire_node(node: &CheckedNodeId) -> Result<WireNodeId, CompositeBuildError> {
    WireNodeId::from_hex(&node.digest).ok_or(CompositeBuildError::NodeIdentity)
}

fn origin(occurrence: &CheckedOccurrence) -> Origin {
    let role = match occurrence.role {
        CheckedOccurrenceRole::Declaration => "declaration",
        CheckedOccurrenceRole::Type => "type",
        CheckedOccurrenceRole::Expression => "expression",
        CheckedOccurrenceRole::Anchor => "anchor",
        CheckedOccurrenceRole::Claim => "claim",
        CheckedOccurrenceRole::Generated => "generated",
    };
    Origin::new(Role::new(role), occurrence.ordinal)
}

/// A constructed original equality request. Its claim and wire cannot be mutated after construction.
pub struct OriginalCompositeEqRequest {
    wire: ReplayRequestWire,
    claim: CompositeParityClaim,
    operands: CheckedCompositeOperands,
    preimage: ParityPreimage,
    replay_limits: qsl_replay::ReplayLimits,
}

impl fmt::Debug for OriginalCompositeEqRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OriginalCompositeEqRequest")
            .field("claim", &self.claim)
            .field("operands", &self.operands)
            .field("preimage", &self.preimage)
            .field("replay_limits", &self.replay_limits)
            .finish_non_exhaustive()
    }
}

impl OriginalCompositeEqRequest {
    /// The typed request that carries the admitted packet's replay selection and occurrence.
    pub fn wire(&self) -> &ReplayRequestWire {
        &self.wire
    }

    /// The retained original operands and their package-authored domains, in positional order.
    pub fn operands(&self) -> &CheckedCompositeOperands {
        &self.operands
    }

    /// The actual typed positional O-09 preimage passed to the owning encoder.
    pub fn preimage(&self) -> &ParityPreimage {
        &self.preimage
    }

    /// Invoke QSL's public verified-shadow facade with retained verified-run evidence.
    ///
    /// This consumes caller-retained evidence; it neither runs Kani nor infers a proof strength.
    /// The genuine returned report passes the existing full-identity converter before returning.
    ///
    /// # Errors
    /// Returns a binding refusal without a terminal value if QSL's report does not match what was sent.
    // The 192-byte refusal is retained inline to preserve the public PackageRead payload.
    #[allow(clippy::result_large_err)]
    pub fn settle_verified(
        self,
        verified: VerifiedShadow,
    ) -> Result<OriginalCompositeEqReport, CompositeBuildError> {
        let sent = CompositeIdentity::new(
            qsl_replay::ObligationIdentity::from_digest(self.wire.obligation_identity),
            &self.claim,
            CompositeEvidence::Verified(verified),
        );
        let report = settle_verified_shadow(self.wire, self.claim, verified, self.replay_limits);
        verified_shadow_terminal_value(&sent, Some(&report))
            .map_err(CompositeBuildError::Report)?;
        Ok(OriginalCompositeEqReport { sent, report })
    }
}

/// A genuine verified-shadow QSL report and the immutable sent identity the converter checked.
#[derive(Debug)]
pub struct OriginalCompositeEqReport {
    sent: CompositeIdentity,
    report: VerifiedShadowReport,
}

impl OriginalCompositeEqReport {
    /// The complete claim retained before invoking QSL.
    pub fn sent(&self) -> &CompositeIdentity {
        &self.sent
    }

    /// QSL's genuine public facade report.
    pub fn report(&self) -> &VerifiedShadowReport {
        &self.report
    }

    /// Read the report through the binding-checked IR-666 converter.
    ///
    /// # Errors
    /// Returns the converter's typed full-identity refusal, without a value.
    pub fn settlement(&self) -> Result<VerifiedShadowSettlement<'_>, CompositeReportError> {
        verified_shadow_terminal_value(&self.sent, Some(&self.report))
    }
}

/// The equality operator a selected node's operation identity names. Only the two structural
/// equality identities name one; every other identity, a quantity conversion included, and an
/// absent identity are an unsupported node operation.
// The 192-byte refusal is retained inline to preserve the public PackageRead payload.
#[allow(clippy::result_large_err)]
fn node_equality_operator(identity: Option<&str>) -> Result<EqualityOperator, CompositeBuildError> {
    match identity {
        Some("quire.op.structural.eq") => Ok(EqualityOperator::Equal),
        Some("quire.op.structural.ne") => Ok(EqualityOperator::NotEqual),
        Some(_) | None => Err(CompositeBuildError::UnsupportedNodeOperation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    /// A quantity conversion is not an equality operation: the builder refuses it as an
    /// unsupported node operation, with no request.
    #[trace("FR-014-AC-2", "TC-024")]
    #[test]
    fn a_quantity_conversion_is_an_unsupported_node_operation() {
        assert!(matches!(
            node_equality_operator(Some("quire.op.quantity.convert")),
            Err(CompositeBuildError::UnsupportedNodeOperation)
        ));
        assert!(matches!(
            node_equality_operator(Some("quire.op.structural.eq")),
            Ok(EqualityOperator::Equal)
        ));
    }
}
