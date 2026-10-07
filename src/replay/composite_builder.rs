// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Original-package construction of verified structural equality requests (FR-033).
//!
//! This boundary authenticates the retained source and checked package and constructs positional
//! O-09 arguments. It binds the caller's retained proof-content identity; it does not authenticate
//! a generated artifact, run a native observation, or establish a backend proof strength.

use std::fmt;

use qsl_replay::{
    compile_package, parity_obligation, settle_verified_shadow, BoundEntries, ByteDigest,
    CompositeEvidence, CompositeIdentity, CompositeParityClaim, DependencyInput, DigestDomain,
    DigestRecord, Domain, DomainKey, EqualityOperator, Identifier, IdentityEncodeError,
    OperandIdentity, Origin, ParityArgument, ParityPreimage, ProofBound, QualifiedName,
    ReplayRefusal, ReplayRequest, ReplayRequestRefusal, ReplayRequestWire, ReplaySource, Role,
    VerifiedShadow, VerifiedShadowReport, WireNodeId,
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
    /// The original application has no complete admitted Eq operand projection.
    Operands(Box<CheckedCompositeOperandError>),
    /// The accessor returned a shape outside this graph-child Eq constructor.
    OperandShape,
    /// The owning O-09 encoder refused the preimage.
    Identity(IdentityEncodeError),
    /// A genuine QSL report did not bind to the request actually sent.
    Report(CompositeReportError),
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
            Self::OperandShape => f.write_str("unsupported Eq operand projection"),
            Self::Identity(cause) => cause.fmt(f),
            Self::Report(cause) => cause.fmt(f),
        }
    }
}

impl std::error::Error for CompositeBuildError {}

/// Immutable original proving context for the admitted graph-child `structural.eq` route.
///
/// The caller supplies the original admitted package and retained source, dependencies, limits
/// and canonical proof-content identity. The content identity is a binding input, not independent
/// authentication of a generated artifact. No native/falsified production route is exposed here.
#[derive(Debug)]
pub struct OriginalCompositeEqContext {
    original: CheckedPackageV2,
    inputs: ReplayInputs,
    dependencies: DependencyInput,
    selection: QualifiedName,
    package_id: DigestRecord,
    content_identity: DigestRecord,
    read_limits: CheckedPackageReadLimits,
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
        Ok(Self {
            original,
            inputs,
            dependencies,
            selection,
            package_id,
            content_identity,
            read_limits,
        })
    }

    /// Build O-09 from the original Eq application's two authentic positional operands.
    ///
    /// Parameter positions retain only their Node bounds. Population metadata is excluded from
    /// this request. Literal graph children retain empty Bounds and hence singleton source values;
    /// QSL derives their values and domains from the original source and owns selected-function
    /// body and occurrence membership at invocation, returning a binding-checked refusal report.
    /// Projection/encoder refusals precede recompilation: public QSL stage-limit decoding requires
    /// the genuine O-09 wire, so this boundary does not authenticate context before projection.
    /// The supplied work ceiling
    /// bounds the IR operand projection. Unsupported operations and inline operands refuse.
    ///
    /// # Errors
    /// Returns a typed original node/occurrence, operand, encoder, request decode, recompile or
    /// original-context refusal, with no report.
    pub fn request(
        &self,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<OriginalCompositeEqRequest, CompositeBuildError> {
        let operands = self
            .original
            .composite_application_operands(node, occurrence, work_limit)
            .map_err(|cause| CompositeBuildError::Operands(Box::new(cause)))?;
        let arguments = operands.operands.iter().map(|operand| {
            let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                return Err(CompositeBuildError::OperandShape);
            };
            let child = wire_node(child)?;
            let bounds = match &operand.domain {
                CheckedCompositeOperandDomain::Literal => Vec::new(),
                CheckedCompositeOperandDomain::Parameter { .. } => harness_bounds.iter()
                    .filter(|bound| matches!(&bound.domain, DomainKey::Node { node, .. } if *node == child))
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
            operator: EqualityOperator::Equal,
            obligation_kind: preimage.obligation_kind.clone(),
            harness_bounds: harness_bounds
                .iter()
                .filter(|bound| matches!(bound.domain, DomainKey::Node { .. }))
                .cloned()
                .collect(),
            limits: self.inputs.accounting_limits,
            content_identity: self.content_identity,
        };
        let wire = self.inputs.wire(
            self.package_id,
            self.selection.clone(),
            ReplaySource::Input(Vec::new()),
            *obligation.as_bytes(),
            &[],
        );
        self.check_recompiled_context(&wire)?;
        Ok(OriginalCompositeEqRequest {
            wire,
            claim,
            operands,
            preimage,
            replay_limits: self.inputs.replay_limits,
        })
    }

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

/// A constructed original Eq request. Its claim and wire cannot be mutated after construction.
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
