//! Bounded cover harness for an admitted V2 parameter precondition (FR-015).

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

pub(super) struct LoweredV2Clause {
    pub(super) package: CheckedSemanticId,
    pub(super) node_id: CheckedNodeId,
    oracle: V2ClauseOracle,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

pub(super) fn lower(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
) -> Result<LoweredV2Clause, UnsupportedObligation> {
    let refusal = || UnsupportedObligation::V2ClauseConditionNotRendered {
        node_id: clause.clone(),
    };
    let oracle = generate_v2_clause_oracle(package, clause).map_err(|_| refusal())?;
    let [input] = oracle.descriptor.inputs.as_slice() else {
        return Err(refusal());
    };
    if input.name.strip_prefix("parameter_").is_none()
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
        "kob_v2_pre_{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    Ok(LoweredV2Clause {
        package: package.package_id().clone(),
        node_id: clause.clone(),
        oracle,
        module_symbol,
        harness_symbol: "cover_precondition".to_owned(),
    })
}

pub(super) fn render_clause(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    let input = &lowered.oracle.descriptor.inputs[0];
    let minimum = i64::try_from(input.lower).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let maximum = i64::try_from(input.upper).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let body = vec![
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
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!(
            "V2 precondition {}/{}",
            lowered.node_id.domain, lowered.node_id.digest
        ),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: ExprText::new("matches!(outcome, Ok(rt::Outcome::Completed(true)))")?,
            message: "the V2 precondition is satisfiable within admitted parameter bounds"
                .to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = V2ClauseObligationIdentity {
        ceilings: request.ceilings,
        package: lowered.package.clone(),
        clause: lowered.node_id.clone(),
        kind: ObligationKind::Precondition,
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
