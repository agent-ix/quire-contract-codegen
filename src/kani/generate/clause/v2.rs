//! Bounded harnesses for admitted V2 preconditions and state invariants (FR-015).

use quire_contract_model::{CheckedNodeId, CheckedPackageV2, CheckedSemanticId};

use crate::{
    kani::{
        abi::{KaniPrimitiveType, KaniSolver},
        generate::{
            outcome::{KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
            render,
            spec::{
                Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement,
            },
        },
        identity::{
            KaniV2ClauseObligationHarness, ObligationKind, ScalarObligationArgument,
            V2ClauseObligationIdentity,
        },
    },
    oracle::clause::{generate_v2_clause_oracle, V2ClauseOracle},
};

pub(in crate::kani::generate) struct LoweredV2Clause {
    pub(super) package: CheckedSemanticId,
    pub(super) node_id: CheckedNodeId,
    oracle: V2ClauseOracle,
    kind: ObligationKind,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

pub(in crate::kani::generate) fn lower(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    kind: ObligationKind,
) -> Result<LoweredV2Clause, UnsupportedObligation> {
    let refusal = || UnsupportedObligation::V2ClauseConditionNotRendered {
        node_id: clause.clone(),
    };
    let oracle = generate_v2_clause_oracle(package, clause).map_err(|_| refusal())?;
    let [input] = oracle.descriptor.inputs.as_slice() else {
        return Err(refusal());
    };
    let supported_input = match kind {
        ObligationKind::Precondition => input.name.starts_with("parameter_"),
        ObligationKind::Invariant => input.name.starts_with("current_"),
        ObligationKind::Postcondition | ObligationKind::Frame => false,
    };
    if !supported_input
        || input.lower > input.upper
        || i64::try_from(input.lower).is_err()
        || i64::try_from(input.upper).is_err()
    {
        return Err(refusal());
    }
    // Kani includes the module in generated filenames. Use the full admitted pair so two
    // revisions of one declaration cannot collide on a readable clause name.
    let preimage = crate::core::canonical::preimage_value(&(package.package_id(), clause))
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let digest = crate::core::canonical::content_digest(&preimage)
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let module_symbol = format!(
        "kob_v2_{}_{}",
        match kind {
            ObligationKind::Precondition => "pre",
            ObligationKind::Invariant => "inv",
            ObligationKind::Postcondition | ObligationKind::Frame => return Err(refusal()),
        },
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    Ok(LoweredV2Clause {
        package: package.package_id().clone(),
        node_id: clause.clone(),
        oracle,
        kind,
        module_symbol,
        harness_symbol: match kind {
            ObligationKind::Precondition => "cover_precondition",
            ObligationKind::Invariant => "assert_invariant",
            ObligationKind::Postcondition | ObligationKind::Frame => return Err(refusal()),
        }
        .to_owned(),
    })
}

pub(in crate::kani::generate) fn render_clause(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    let [input] = lowered.oracle.descriptor.inputs.as_slice() else {
        return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: lowered.node_id.clone(),
        });
    };
    let minimum = i64::try_from(input.lower).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let maximum = i64::try_from(input.upper).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let mut body = vec![
        Statement::Input {
            name: input.name.clone(),
            ty: KaniPrimitiveType::I64,
            domain: (input.lower, input.upper),
            then: vec![LetBinding {
                name: input.name.clone(),
                ty: None,
                mutable: false,
                init: Init::Expression(ExprText::new(format!(
                    "rt::Integer::from({})",
                    input.name
                ))?),
            }],
        },
        Statement::Let(LetBinding {
            name: "meter".to_owned(),
            ty: None,
            mutable: true,
            init: Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
        }),
        Statement::Call {
            result: "outcome".to_owned(),
            callee: "oracle_clause".to_owned(),
            args: vec![
                Argument::Borrow(input.name.clone()),
                Argument::BorrowMut("meter".to_owned()),
            ],
        },
    ];
    if lowered.kind == ObligationKind::Invariant {
        body.push(Statement::Assert {
            condition: ExprText::new("matches!(&outcome, Ok(rt::Outcome::Completed(true)))")?,
            message: "the V2 invariant holds for every admitted state".to_owned(),
        });
    }
    let (label, cover) = match lowered.kind {
        ObligationKind::Precondition => (
            "precondition",
            Cover {
                condition: ExprText::new("matches!(outcome, Ok(rt::Outcome::Completed(true)))")?,
                message: "the V2 precondition is satisfiable within admitted parameter bounds"
                    .to_owned(),
            },
        ),
        ObligationKind::Invariant => (
            "invariant",
            Cover {
                condition: ExprText::new("matches!(&outcome, Ok(rt::Outcome::Completed(_)))")?,
                message: "the V2 invariant's admitted state domain is reachable".to_owned(),
            },
        ),
        ObligationKind::Postcondition | ObligationKind::Frame => {
            return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
                node_id: lowered.node_id.clone(),
            });
        }
    };
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!(
            "V2 {label} {}/{}",
            lowered.node_id.domain, lowered.node_id.digest,
        ),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![cover],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = V2ClauseObligationIdentity {
        ceilings: request.ceilings,
        package: lowered.package.clone(),
        clause: lowered.node_id.clone(),
        kind: lowered.kind,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments: vec![ScalarObligationArgument {
            identifier: input.name.clone(),
            minimum,
            maximum,
        }],
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        solver: spec.solver(),
        unwind: spec.unwind(),
        options: spec.options().to_vec(),
    };
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniV2ClauseObligationHarness::generated(
        identity, rust, record,
    ))
}
