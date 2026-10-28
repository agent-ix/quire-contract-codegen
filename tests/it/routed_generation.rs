//! FR-022 routed generation: `generate_routed` runs the routed kind's generation arm keyed by the
//! driver's request index, and takes the routed backend and kind as given.
//!
//! The fixtures are TC-024/TC-025's: `kani_obligations::scalar_package` and its claim map.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_exact_scalar_oracles, generate_routed as generate_routed_impl,
    negotiate_kani_obligations, routed_contract_lowering_profile, BackendDescriptor, BackendKind,
    Candidate, ClaimDerivationRefusal, ClaimDisposition, ContractDriverResultSelection,
    ContractRequestPosition, ContractScalarError, ExactScalarRefusal, GenerationContexts,
    InvalidObligationItem, KaniGenerationContext, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, KaniScalarObligationHarness, KindOutput, ObligationDisposition,
    ObligationItem, ObligationRecord, OracleGenerationError, ProviderOrigin, RoutedGeneration,
    RoutedGenerationError, RoutedGenerationItem, UnsupportedObligation,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies,
    ContractFeatureRecordV1, ContractOccurrenceKeyV1, ContractPackageReadResult, ContractPackageV1,
    ContractPackageWireV1,
};

use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};

use crate::kani_obligations::{
    admitted_scalar_claims, package, routed_scalar_package, scalar_package,
};

fn application_arguments(body: &CheckedTerm) -> Option<&[CheckedTerm]> {
    match body {
        CheckedTerm::Application { arguments, .. } => Some(arguments),
        _ => None,
    }
}

fn reference_target(term: &CheckedTerm) -> Option<&CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target),
        _ => None,
    }
}

fn operation_identity(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation.identity()
}

const RENDERED: [&str; 3] = [
    "quire.op.integer.add",
    "quire.op.integer.sub",
    "quire.op.integer.negate",
];

/// An external driver can name the scalar rendering envelope's typed source
/// ceiling without reaching through a private module.
#[trace("FR-022-AC-25", "TC-033")]
#[test]
fn routed_scalar_source_ceiling_is_publicly_matchable() {
    fn source_ceiling(error: &RoutedGenerationError) -> Option<(usize, usize, &'static str)> {
        match error {
            RoutedGenerationError::ContractScalar(ContractScalarError::Oracle(
                OracleGenerationError::SourceTooLarge {
                    bytes,
                    limit,
                    setting,
                },
            )) => Some((*bytes, *limit, setting)),
            _ => None,
        }
    }

    let error = RoutedGenerationError::ContractScalar(ContractScalarError::Oracle(
        OracleGenerationError::SourceTooLarge {
            bytes: 1025,
            limit: 1024,
            setting: "cg.scalar.source_bytes",
        },
    ));
    assert_eq!(
        source_ceiling(&error),
        Some((1025, 1024, "cg.scalar.source_bytes"))
    );
}

struct Fixture {
    package: CheckedPackageV2,
    ids: package::FixtureIds,
    /// Three nodes FR-014 generates and FR-015 renders.
    rendered: Vec<CheckedNodeId>,
    /// A generated node FR-015 refuses as `OperationNotRendered` without invalidating the request.
    unrendered: CheckedNodeId,
}

struct AdmittedRoutingInput {
    package: ContractPackageV1,
    positions: Vec<ContractRequestPosition>,
}

fn generate_routed(
    package: &ContractPackageV1,
    positions: &[ContractRequestPosition],
    routed: &[RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<RoutedGeneration, RoutedGenerationError> {
    let selections = positions
        .iter()
        .filter_map(|position| match position {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => Some(Some(result_selection)),
            ContractRequestPosition::Excluded { .. } => Some(None),
            ContractRequestPosition::ContextRoot { .. } => None,
        })
        .collect::<Vec<_>>();
    let bound = routed
        .iter()
        .cloned()
        .map(
            |item| match selections.get(item.request_index).copied().flatten() {
                Some(selection) => item.with_scalar_selection(package, selection.clone()),
                None => item,
            },
        )
        .collect::<Vec<_>>();
    generate_routed_impl(package, positions, &bound, contexts)
}

/// Lower the complete indexed driver request once and read its FR-330 package.
/// Unrouted positions remain in accounting; sparse routed indexes are still
/// their original driver indexes, not positions in the routed subset.
fn admitted_routing_input(
    checked: &CheckedPackageV2,
    routed: &[RoutedGenerationItem],
) -> AdmittedRoutingInput {
    admitted_routing_input_with_limits(checked, routed, CheckedPackageReadLimits::bounded())
}

fn admitted_routing_input_with_limits(
    checked: &CheckedPackageV2,
    routed: &[RoutedGenerationItem],
    read_limits: CheckedPackageReadLimits,
) -> AdmittedRoutingInput {
    let mut requested = routed
        .iter()
        .map(|item| item.request_index)
        .max()
        .map_or_else(Vec::new, |last| {
            vec![
                routed
                    .first()
                    .expect("nonempty routed set has a first root")
                    .node_id
                    .clone();
                last + 1
            ]
        });
    for item in routed {
        requested[item.request_index] = item.node_id.clone();
    }
    let mut roots = Vec::new();
    let mut positions = Vec::new();
    for node_id in requested {
        let application = checked.source_map().iter().find(|source| {
            source.node_id == node_id && source.role == CheckedOccurrenceRole::Expression
        });
        let conversion = checked.graph().nodes.iter().find(|node| {
            operation_identity(&node.body) == Some("quire.op.numeric.narrow")
                && application_arguments(&node.body)
                    .and_then(|arguments| arguments.first())
                    .and_then(reference_target)
                    == Some(&node_id)
        });
        let selection =
            application.map_or(ContractDriverResultSelection::NotApplicable, |source| {
                ContractDriverResultSelection::OwnResult {
                    application: ContractOccurrenceKeyV1 {
                        node_id: node_id.clone(),
                        role: source.role.clone(),
                        ordinal: source.ordinal,
                    },
                    conversion: conversion.map(|node| {
                        let source = checked
                            .source_map()
                            .iter()
                            .find(|source| {
                                source.node_id == node.node_id
                                    && source.role == CheckedOccurrenceRole::Expression
                            })
                            .expect("authored conversion occurrence");
                        ContractOccurrenceKeyV1 {
                            node_id: node.node_id.clone(),
                            role: source.role.clone(),
                            ordinal: source.ordinal,
                        }
                    }),
                }
            });
        roots.push(node_id.clone());
        positions.push(ContractRequestPosition::DriverItem {
            node_id,
            result_selection: selection,
        });
        if let Some(conversion) = conversion {
            roots.push(conversion.node_id.clone());
            positions.push(ContractRequestPosition::ContextRoot {
                node_id: conversion.node_id.clone(),
            });
        }
    }
    let lowered = checked.lower(&roots, &routed_contract_lowering_profile());
    let package = match ContractPackageWireV1::read_with_source(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        read_limits,
        &ContractDependencies::new(),
        checked,
    ) {
        Ok(ContractPackageReadResult::Admitted(package)) => *package,
        other => panic!("Contract IR must admit its routed package: {other:?}"),
    };
    AdmittedRoutingInput { package, positions }
}

fn fixture() -> Fixture {
    let (_, baseline, baseline_ids) = scalar_package();
    let generated = baseline
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .collect::<Vec<_>>();
    let rendered = RENDERED
        .iter()
        .map(|identity| {
            generated
                .iter()
                .find(|claim| claim.operation.identity == *identity)
                .map(|claim| claim.node_id.clone())
                .unwrap_or_else(|| panic!("the corpus generates {identity}"))
        })
        .collect::<Vec<_>>();
    let unrendered = generated
        .iter()
        .find(|claim| {
            !RENDERED.contains(&claim.operation.identity.as_str())
                && claim.operation.identity != "quire.op.integer.mul"
        })
        .map(|claim| claim.node_id.clone())
        .expect("the corpus generates a family Kani does not render");
    let selected = rendered
        .iter()
        .chain(std::iter::once(&unrendered))
        .map(|node_id| {
            package::corpus()
                .into_iter()
                .find(|expression| {
                    baseline_ids.resolve(&package::code_id(expression.code)) == *node_id
                })
                .expect("selected scalar expression belongs to corpus")
        })
        .collect::<Vec<_>>();
    let mut builder = package::corpus_package();
    for (index, expression) in selected.iter().enumerate() {
        let bound = expression
            .bounds
            .first()
            .expect("selected scalar result bound");
        let target = builder.bound(bound);
        builder.narrow(
            49_000 + u32::try_from(index).expect("four selected expressions"),
            package::code_id(expression.code).digest.as_ref(),
            &target,
        );
    }
    builder.expression_occurrences();
    let (package, ids) = builder.admit_resolved();
    let items = package::golden_items()
        .into_iter()
        .map(|mut item| {
            item.node_id = ids.resolve(&item.node_id);
            item
        })
        .collect::<Vec<_>>();
    let _claims = generate_exact_scalar_oracles(&package, &items)
        .expect("same admitted routed fixture supplies the direct scalar baseline");
    let selected = selected
        .iter()
        .map(|expression| ids.resolve(&package::code_id(expression.code)))
        .collect::<Vec<_>>();
    Fixture {
        package,
        ids,
        rendered: selected[..3].to_vec(),
        unrendered: selected[3].clone(),
    }
}

/// Routed-only source occurrences: each selected result is an authored direct
/// narrow, so FR-049 can choose it from the admitted package.
fn routed_bounded_increment_package() -> (CheckedPackageV2, package::FixtureIds) {
    let mut builder = package::bounded_increment_package();
    let result = builder.bound(&package::Bound::Integer(0, 9));
    builder.narrow(
        49_100,
        &package::code_id(package::BOUNDED_INCREMENT).digest,
        &result,
    );
    builder.expression_occurrences();
    builder.admit_resolved()
}

fn routed_two_parameter_package() -> (CheckedPackageV2, package::FixtureIds) {
    let mut builder = package::two_parameter_package();
    for (code, bound) in [
        (package::TWO_PARAMETER_SUM, package::Bound::Integer(0, 29)),
        (
            package::TWO_PARAMETER_LITERAL,
            package::Bound::Integer(0, 29),
        ),
        (package::TWO_PARAMETER_WIDE, package::Bound::Integer(0, 29)),
        (package::BOUNDED_NEGATE, package::Bound::Integer(-9, -1)),
        (package::BOUNDED_PRODUCT, package::Bound::Integer(100, 1800)),
        (
            package::TWO_PARAMETER_UNBOUNDED_OPERAND,
            package::Bound::Integer(0, 29),
        ),
        (
            package::TWO_PARAMETER_PLAIN_PAIR,
            package::Bound::Integer(0, 29),
        ),
    ] {
        let result = builder.bound(&bound);
        builder.narrow(49_100 + code, &package::code_id(code).digest, &result);
    }
    builder.expression_occurrences();
    builder.admit_resolved()
}

fn kani_backend() -> Candidate {
    Candidate {
        identity: "kani".to_owned(),
    }
}

fn route(request_index: usize, node_id: &CheckedNodeId) -> RoutedGenerationItem {
    RoutedGenerationItem::from_descriptor(
        request_index,
        node_id.clone(),
        kani_backend(),
        &BackendDescriptor {
            identity: "kani".to_owned(),
            advertised: Vec::new(),
            origin: ProviderOrigin::Linked,
            domains: None,
            bounds: None,
        },
    )
    .expect("linked Kani descriptor")
}

fn process_route(
    request_index: usize,
    node_id: &CheckedNodeId,
    identity: &str,
) -> RoutedGenerationItem {
    let descriptor = BackendDescriptor {
        identity: identity.to_owned(),
        advertised: Vec::new(),
        origin: ProviderOrigin::Process,
        domains: None,
        bounds: None,
    };
    RoutedGenerationItem::from_descriptor(
        request_index,
        node_id.clone(),
        Candidate {
            identity: identity.to_owned(),
        },
        &descriptor,
    )
    .expect("matching process descriptor")
}

#[trace("FR-022-AC-21", "TC-046")]
#[test]
fn routed_process_construction_preserves_optional_manifest_bounds() {
    let fixture = fixture();
    let absent = BackendDescriptor {
        identity: "solver".to_owned(),
        advertised: Vec::new(),
        origin: ProviderOrigin::Process,
        domains: None,
        bounds: None,
    };
    let present_empty = BackendDescriptor {
        bounds: Some(Default::default()),
        ..absent.clone()
    };
    assert_ne!(absent, present_empty);
    let construct = |descriptor: &BackendDescriptor| {
        RoutedGenerationItem::from_descriptor(
            0,
            fixture.rendered[0].clone(),
            Candidate {
                identity: "solver".to_owned(),
            },
            descriptor,
        )
        .expect("matching process descriptor")
    };
    assert_eq!(construct(&absent), construct(&present_empty));
}

#[trace("FR-022-AC-17", "FR-022-AC-19", "FR-022-AC-21", "TC-046")]
#[test]
fn process_items_generate_empty_outputs_in_request_order() {
    let fixture = fixture();
    let process_only = [process_route(8, &fixture.rendered[0], "second")];
    let process_input = admitted_routing_input(&fixture.package, &process_only);
    let empty_context = generate_routed(
        &process_input.package,
        &process_input.positions,
        &process_only,
        &GenerationContexts::default(),
    )
    .unwrap();
    assert_eq!(empty_context.items[0].output, KindOutput::Process);
    assert!(empty_context.claim_map.is_none());
    assert!(empty_context.oracle_artifacts.is_none());
    let routed = vec![
        process_route(8, &fixture.rendered[0], "second"),
        route(3, &fixture.rendered[0]),
        process_route(1, &fixture.rendered[0], "kani"),
    ];
    let contexts = kani_context("crate::subject", 1);
    let input = admitted_routing_input(&fixture.package, &routed);
    let output = generate_routed(&input.package, &input.positions, &routed, &contexts).unwrap();
    assert_eq!(
        output
            .items
            .iter()
            .map(|item| item.request_index)
            .collect::<Vec<_>>(),
        [1, 3, 8]
    );
    assert_eq!(output.items[0].output, KindOutput::Process);
    assert_eq!(output.items[0].backend.identity, "kani");
    assert_eq!(output.items[2].output, KindOutput::Process);
    let mut reversed = routed;
    reversed.reverse();
    assert_eq!(
        generate_routed(&input.package, &input.positions, &reversed, &contexts).unwrap(),
        output
    );
}

#[trace("FR-022-AC-22", "TC-046")]
#[test]
fn routed_item_constructor_refuses_mismatched_and_unknown_linked_descriptors() {
    let fixture = fixture();
    let descriptor = BackendDescriptor {
        identity: "solver".to_owned(),
        advertised: Vec::new(),
        origin: ProviderOrigin::Process,
        domains: None,
        bounds: None,
    };
    let mismatch = RoutedGenerationItem::from_descriptor(
        0,
        fixture.rendered[0].clone(),
        kani_backend(),
        &descriptor,
    );
    assert_eq!(
        mismatch,
        Err(
            quire_contract_codegen::RoutedItemConstructionError::DescriptorBackendMismatch {
                backend: kani_backend(),
                descriptor: "solver".to_owned(),
            }
        )
    );
    let unknown = RoutedGenerationItem::from_descriptor(
        0,
        fixture.rendered[0].clone(),
        Candidate {
            identity: "solver".to_owned(),
        },
        &BackendDescriptor {
            origin: ProviderOrigin::Linked,
            ..descriptor
        },
    );
    assert_eq!(
        unknown,
        Err(
            quire_contract_codegen::RoutedItemConstructionError::UnknownLinkedBackend {
                backend: Candidate {
                    identity: "solver".to_owned()
                },
            }
        )
    );
}

fn kani_context(subject_path: &str, unwind: u32) -> GenerationContexts<'_> {
    GenerationContexts {
        kani: Some(KaniGenerationContext {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            subject_path,
            unwind,
        }),
    }
}

/// A Kani route must carry the exact selected result used at preflight.
#[trace("FR-022-AC-23", "FR-022-AC-24", "TC-033")]
#[test]
fn routed_scalar_selection_must_match_admitted_driver_position() {
    let fixture = fixture();
    let routed_item = route(0, &fixture.rendered[0]);
    let input = admitted_routing_input(&fixture.package, std::slice::from_ref(&routed_item));
    let context = kani_context("crate::subject", 1);
    assert_eq!(
        generate_routed_impl(
            &input.package,
            &input.positions,
            std::slice::from_ref(&routed_item),
            &context
        ),
        Err(RoutedGenerationError::ScalarSelectionMismatch { request_index: 0 }),
    );
    let selection = match &input.positions[0] {
        ContractRequestPosition::DriverItem {
            result_selection, ..
        } => result_selection.clone(),
        ContractRequestPosition::Excluded { .. } | ContractRequestPosition::ContextRoot { .. } => {
            unreachable!("first position is a driver")
        }
    };
    let changed = if selection == ContractDriverResultSelection::NotApplicable {
        ContractDriverResultSelection::OwnResult {
            application: ContractOccurrenceKeyV1 {
                node_id: routed_item.node_id.clone(),
                role: CheckedOccurrenceRole::Expression,
                ordinal: 0,
            },
            conversion: None,
        }
    } else {
        ContractDriverResultSelection::NotApplicable
    };
    let wrong = routed_item.with_scalar_selection(&input.package, changed);
    assert_eq!(
        generate_routed_impl(
            &input.package,
            &input.positions,
            std::slice::from_ref(&wrong),
            &context
        ),
        Err(RoutedGenerationError::ScalarSelectionMismatch { request_index: 0 }),
    );
    let other_route = route(0, &fixture.rendered[1]);
    let other_package =
        admitted_routing_input(&fixture.package, std::slice::from_ref(&other_route));
    assert_ne!(input.package.identity(), other_package.package.identity());
    let bound_elsewhere =
        route(0, &fixture.rendered[0]).with_scalar_selection(&other_package.package, selection);
    assert_eq!(
        generate_routed_impl(
            &input.package,
            &input.positions,
            std::slice::from_ref(&bound_elsewhere),
            &context,
        ),
        Err(RoutedGenerationError::ScalarSelectionMismatch { request_index: 0 }),
    );
}

#[trace("FR-028-AC-1")]
#[test]
fn routed_scalar_identity_records_both_required_ceilings() {
    let fixture = fixture();
    let routed = [route(0, &fixture.rendered[0])];
    let input = admitted_routing_input(&fixture.package, &routed);
    let mut context = kani_context("crate::subject", 1);
    let generate = |context: &GenerationContexts<'_>| {
        let generated =
            generate_routed(&input.package, &input.positions, &routed, context).unwrap();
        kani_parts(&generated.items[0].output).1.unwrap().clone()
    };
    let baseline = generate(&context);
    context.kani.as_mut().unwrap().ceilings.memory_bytes =
        std::num::NonZeroU64::new(128 * 1024 * 1024).unwrap();
    let changed_memory = generate(&context);
    assert_ne!(baseline.identity, changed_memory.identity);
    assert_ne!(baseline.record.contents, changed_memory.record.contents);
    assert_eq!(
        changed_memory.identity.ceilings,
        context.kani.unwrap().ceilings
    );
    context.kani.as_mut().unwrap().ceilings = baseline.identity.ceilings;
    context.kani.as_mut().unwrap().ceilings.wall_clock /= 2;
    let changed_wall = generate(&context);
    assert_ne!(baseline.identity, changed_wall.identity);
    assert_ne!(baseline.record.contents, changed_wall.record.contents);
    assert_eq!(
        changed_wall.identity.ceilings,
        context.kani.unwrap().ceilings
    );
}

/// What FR-015 returns for `nodes` in the given (ascending) order over `claim_map`, which names
/// the oracles the obligations embed.
fn fr015(
    fixture: &Fixture,
    claim_map: &quire_contract_codegen::ClaimMap<quire_contract_codegen::ExactScalarClaim>,
    nodes: &[&CheckedNodeId],
) -> (Vec<ObligationRecord>, Vec<KaniScalarObligationHarness>) {
    let admitted = admitted_scalar_claims(&fixture.package, claim_map);
    let items = nodes
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &admitted,
            claim_map,
            node_id,
            claim_index: claim_map
                .items
                .iter()
                .position(|claim| &claim.node_id == *node_id)
                .expect("selected claim"),
            result_selection: &ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &items,
        subject_path: "crate::subject",
        unwind: 1,
    })
    .expect("FR-015 accepts the request");
    match outcome {
        KaniObligationOutcome::Emitted {
            records,
            scalar_harnesses,
            ..
        } => (records, scalar_harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("rejected: {records:#?}"),
    }
}

fn kani_parts(output: &KindOutput) -> (&ObligationRecord, Option<&KaniScalarObligationHarness>) {
    match output {
        KindOutput::Kani { record, harness } => (record, harness.as_deref()),
        KindOutput::PackageRefused => panic!("expected Kani output, got IR refusal"),
        KindOutput::Process => panic!("expected Kani output"),
    }
}

/// The four-item routing of TC-033 step 1: three rendered nodes and the unrendered one at the
/// non-contiguous indexes 7, 2, 11 and 4.
fn step_one_routing(fixture: &Fixture) -> Vec<RoutedGenerationItem> {
    vec![
        route(7, &fixture.rendered[0]),
        route(2, &fixture.rendered[1]),
        route(11, &fixture.rendered[2]),
        route(4, &fixture.unrendered),
    ]
}

fn generate_step_one(fixture: &Fixture, routed: &[RoutedGenerationItem]) -> RoutedGeneration {
    let input = admitted_routing_input(&fixture.package, routed);
    generate_routed(
        &input.package,
        &input.positions,
        routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds")
}

/// Every routed Kani item equals what FR-015 returns in ascending request-index order, with the
/// driver's index in every record, and each harness is joined by `harness_symbol`.
#[trace("FR-022-AC-2", "FR-022-AC-8", "TC-033")]
#[test]
fn tc_033_routed_kani_items_equal_fr015_output_keyed_by_request_index() {
    let fixture = fixture();
    let generation = generate_step_one(&fixture, &step_one_routing(&fixture));
    // The routed arm's own claim map: generated oracle names count within one generation run.
    let (records, harnesses) = fr015(
        &fixture,
        generation.claim_map.as_ref().expect("a Kani group ran"),
        &[
            &fixture.rendered[1],
            &fixture.unrendered,
            &fixture.rendered[0],
            &fixture.rendered[2],
        ],
    );
    let driver = [2, 4, 7, 11];

    assert!(generation.rejected.is_empty());
    assert_eq!(
        generation
            .items
            .iter()
            .map(|item| item.request_index)
            .collect::<Vec<_>>(),
        driver
    );
    assert_eq!(harnesses.len(), 3);
    for ((item, expected), index) in generation.items.iter().zip(&records).zip(driver) {
        assert_eq!(item.backend, kani_backend());
        let (record, harness) = kani_parts(&item.output);
        let mut expected = expected.clone();
        expected.request_index = index;
        assert_eq!(record, &expected);
        match &expected.disposition {
            ObligationDisposition::Supported { harness_symbol } => {
                let expected_harness = harnesses
                    .iter()
                    .find(|harness| &harness.identity.harness_symbol == harness_symbol)
                    .expect("FR-015 emitted the harness the record names");
                let harness = harness.expect("a supported item carries its harness");
                assert_eq!(harness, expected_harness);
                assert_eq!(&harness.identity.harness_symbol, harness_symbol);
            }
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::OperationNotRendered { .. },
            } => assert!(
                harness.is_none(),
                "an item refused at lowering has no harness"
            ),
            other => panic!("unexpected disposition {other:?}"),
        }
    }
}

/// The entry point takes no settlement input.
#[trace("FR-022-AC-3", "TC-033")]
#[test]
fn tc_033_the_entry_point_takes_no_settlement_input() {
    // The coercion fails to compile if the signature gains a manifest, candidate set, extent or
    // capability kind.
    type RoutedEntry = for<'a> fn(
        &ContractPackageV1,
        &[ContractRequestPosition],
        &[RoutedGenerationItem],
        &GenerationContexts<'a>,
    ) -> Result<RoutedGeneration, RoutedGenerationError>;
    let _: RoutedEntry = generate_routed;
}

/// Duplicate request indexes and a missing kind context each refuse the whole call.
#[trace("FR-022-AC-5", "TC-033")]
#[test]
fn tc_033_duplicate_index_and_missing_context_refuse_the_call() {
    let fixture = fixture();
    let contexts = kani_context("crate::subject", 1);
    let duplicated = [
        route(8, &fixture.rendered[0]),
        route(3, &fixture.rendered[1]),
        route(3, &fixture.rendered[2]),
        route(1, &fixture.rendered[0]),
        route(1, &fixture.rendered[1]),
    ];
    let duplicate_input = admitted_routing_input(&fixture.package, &duplicated);
    assert_eq!(
        generate_routed(
            &duplicate_input.package,
            &duplicate_input.positions,
            &duplicated,
            &contexts
        ),
        Err(RoutedGenerationError::DuplicateRequestIndex { request_index: 1 })
    );
    let valid = [route(0, &fixture.rendered[0])];
    let valid_input = admitted_routing_input(&fixture.package, &valid);
    assert_eq!(
        generate_routed(
            &valid_input.package,
            &valid_input.positions,
            &valid,
            &GenerationContexts { kani: None }
        ),
        Err(RoutedGenerationError::MissingKindContext {
            kind: BackendKind::Kani
        })
    );
    // Duplicates are checked before the missing context.
    assert_eq!(
        generate_routed(
            &duplicate_input.package,
            &duplicate_input.positions,
            &duplicated,
            &GenerationContexts { kani: None }
        ),
        Err(RoutedGenerationError::DuplicateRequestIndex { request_index: 1 })
    );
}

/// A Kani group-level refusal is returned unchanged as `Kani`.
#[trace("FR-022-AC-6", "TC-033")]
#[test]
fn tc_033_a_kani_group_refusal_is_returned_unchanged() {
    let fixture = fixture();
    let routed = step_one_routing(&fixture);
    let input = admitted_routing_input(&fixture.package, &routed);
    assert_eq!(
        generate_routed(
            &input.package,
            &input.positions,
            &routed,
            &kani_context("crate::subject", 0)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::InvalidUnwind { unwind: 0 }
        ))
    );
    assert_eq!(
        generate_routed(
            &input.package,
            &input.positions,
            &routed,
            &kani_context("not a path", 1)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::InvalidSubjectPath
        ))
    );
}

/// An `invalid_request` item rejects the Kani group: every record, no harness, and the duplicate
/// names the driver's index of the first occurrence.
#[trace("FR-022-AC-7", "TC-033")]
#[test]
fn tc_033_an_invalid_kani_item_rejects_the_group_with_driver_indexes() {
    let fixture = fixture();
    let routed = [
        route(9, &fixture.rendered[0]),
        route(6, &fixture.rendered[0]),
        route(20, &fixture.rendered[1]),
    ];
    let generation = generate_step_one(&fixture, &routed);
    assert_eq!(generation.rejected, [BackendKind::Kani]);
    // The rejected group retains the package-derived oracle crate and claim map.
    assert_eq!(
        generation
            .oracle_artifacts
            .as_ref()
            .map(|artifacts| artifacts
                .iter()
                .map(|artifact| artifact.path.as_str())
                .collect::<Vec<_>>()),
        Some(vec![
            "Cargo.toml",
            "src/lib.rs",
            "source-maps/lib.json",
            "claim-map.json"
        ])
    );
    assert_eq!(
        generation
            .items
            .iter()
            .map(|item| item.request_index)
            .collect::<Vec<_>>(),
        [6, 9, 20]
    );
    for item in &generation.items {
        let (record, harness) = kani_parts(&item.output);
        assert_eq!(record.request_index, item.request_index);
        assert!(harness.is_none(), "a rejected group has no harness");
    }
    let (duplicate, _) = kani_parts(&generation.items[1].output);
    assert_eq!(
        duplicate.disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::DuplicateItem { first_index: 6 }
        }
    );
    let (valid, _) = kani_parts(&generation.items[2].output);
    assert!(matches!(
        valid.disposition,
        ObligationDisposition::Supported { .. }
    ));
}

/// An empty routed set is an empty result, whether or not a context is supplied.
#[trace("FR-022-AC-8", "TC-033")]
#[test]
fn tc_033_an_empty_routed_set_returns_an_empty_result() {
    let fixture = fixture();
    let input = admitted_routing_input(&fixture.package, &[]);
    for contexts in [
        GenerationContexts { kani: None },
        kani_context("crate::subject", 1),
    ] {
        assert_eq!(
            generate_routed(&input.package, &input.positions, &[], &contexts),
            Ok(RoutedGeneration {
                items: Vec::new(),
                rejected: Vec::new(),
                claim_map: None,
                oracle_artifacts: None,
            })
        );
    }
}

/// Regeneration and slice order do not change the result, and the request index does not reach
/// the harness.
#[trace("FR-022-AC-9", "TC-033")]
#[test]
fn tc_033_output_is_deterministic_and_independent_of_the_request_index() {
    let fixture = fixture();
    let routed = step_one_routing(&fixture);
    let input = admitted_routing_input(&fixture.package, &routed);
    let contexts = kani_context("crate::subject", 1);
    let generate = |items: &[RoutedGenerationItem]| {
        generate_routed(&input.package, &input.positions, items, &contexts)
            .expect("same admitted package routes")
    };
    let first = generate(&routed);
    assert_eq!(first, generate(&routed));
    let mut reversed = routed.clone();
    reversed.reverse();
    assert_eq!(first, generate(&reversed));

    let low_route = [route(0, &fixture.rendered[0])];
    let high_route = [route(40, &fixture.rendered[0])];
    let same_input = admitted_routing_input(
        &fixture.package,
        &[low_route[0].clone(), high_route[0].clone()],
    );
    let contexts = kani_context("crate::subject", 1);
    let low = generate_routed(
        &same_input.package,
        &same_input.positions,
        &low_route,
        &contexts,
    )
    .expect("same admitted package routes low index");
    let high = generate_routed(
        &same_input.package,
        &same_input.positions,
        &high_route,
        &contexts,
    )
    .expect("same admitted package routes high index");
    let (low_record, low_harness) = kani_parts(&low.items[0].output);
    let (high_record, high_harness) = kani_parts(&high.items[0].output);
    let (low_harness, high_harness) = (
        low_harness.expect("the node renders"),
        high_harness.expect("the node renders"),
    );
    assert_eq!(low_harness.rust, high_harness.rust);
    assert_eq!(low_harness.record, high_harness.record);
    assert_eq!(low_harness.identity, high_harness.identity);
    assert_eq!(
        (low_record.request_index, high_record.request_index),
        (0, 40)
    );
    assert_eq!(low.items[0].backend, kani_backend());
    assert_eq!(high.items[0].backend, kani_backend());
}

/// The harness of the item at `position` in `generation`, which must render.
fn harness_at(generation: &RoutedGeneration, position: usize) -> &KaniScalarObligationHarness {
    kani_parts(&generation.items[position].output)
        .1
        .expect("the node renders")
}

/// A node routed alone and the same node routed among siblings of other operations get
/// byte-identical harnesses: a name depends only on siblings that share its readable stem.
#[trace("FR-022-AC-9", "TC-033")]
#[test]
fn tc_033_a_node_gets_identical_bytes_whatever_its_differently_named_siblings() {
    let fixture = fixture();
    let alone_route = [route(0, &fixture.rendered[0])];
    let among_route = [
        route(3, &fixture.rendered[1]),
        alone_route[0].clone(),
        route(7, &fixture.rendered[2]),
    ];
    let input = admitted_routing_input(&fixture.package, &among_route);
    let contexts = kani_context("crate::subject", 1);
    let alone = generate_routed(&input.package, &input.positions, &alone_route, &contexts)
        .expect("routes alone within the admitted request");
    let among = generate_routed(&input.package, &input.positions, &among_route, &contexts)
        .expect("routes among siblings within the admitted request");
    assert_eq!(among.items[0].request_index, 0);
    assert_eq!(harness_at(&alone, 0), harness_at(&among, 0));
}

/// Two nodes of one operation share a readable stem and take ordinals in node-id order, so
/// swapping their request positions leaves each node's harness byte-identical.
#[trace("FR-022-AC-9", "TC-033")]
#[test]
fn tc_033_nodes_sharing_a_stem_take_stable_ordinals_in_node_order() {
    let (package, ids) = routed_two_parameter_package();
    let mut same_operation = [
        ids.resolve(&package::code_id(package::TWO_PARAMETER_SUM)),
        ids.resolve(&package::code_id(package::TWO_PARAMETER_LITERAL)),
    ];
    same_operation.sort();
    let [lower, higher] = &same_operation;
    let forward_routes = [route(0, lower), route(1, higher)];
    let swapped_routes = [route(2, higher), route(3, lower)];
    let input = admitted_routing_input(
        &package,
        &[
            forward_routes[0].clone(),
            forward_routes[1].clone(),
            swapped_routes[0].clone(),
            swapped_routes[1].clone(),
        ],
    );
    let generate = |routed: &[RoutedGenerationItem]| {
        generate_routed(
            &input.package,
            &input.positions,
            routed,
            &kani_context("crate::subject", 1),
        )
        .expect("routed generation succeeds")
    };
    let forward = generate(&forward_routes);
    let swapped = generate(&swapped_routes);
    assert_eq!(
        harness_at(&forward, 0).identity.operation_identity,
        harness_at(&forward, 1).identity.operation_identity
    );
    assert_eq!(harness_at(&forward, 0), harness_at(&swapped, 1));
    assert_eq!(harness_at(&forward, 1), harness_at(&swapped, 0));
    let lower_symbol = &harness_at(&forward, 0).identity.harness_symbol;
    let higher_symbol = &harness_at(&forward, 1).identity.harness_symbol;
    assert_ne!(lower_symbol, higher_symbol);
    assert!(
        lower_symbol.as_str().ends_with("_1_proof"),
        "{lower_symbol}"
    );
    assert!(
        higher_symbol.as_str().ends_with("_2_proof"),
        "{higher_symbol}"
    );
}

/// `x + 1` over `Int[0, 9]`, QSL's shape (a parameter typed by an `integer_range`
/// `bounded_domain` with binding-shaped `min`/`max` members), routes to Kani and is Supported
/// with a scalar harness ranging `x` over the inclusive `[0, 9]` domain (IR-297 with IR-296) and
/// the literal `1` at its own value (IR-302). No claim map
/// is supplied: the operation and domain are derived from the package alone (IR-294).
#[trace(
    "FR-014-AC-16",
    "FR-014-AC-17",
    "FR-022-AC-2",
    "FR-022-AC-10",
    "TC-024",
    "TC-033"
)]
#[test]
fn tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness() {
    let (package, ids) = routed_bounded_increment_package();
    let node_id = ids.resolve(&package::code_id(package::BOUNDED_INCREMENT));
    let routed = [route(0, &node_id)];
    let input = admitted_routing_input(&package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let (record, harness) = kani_parts(&item.output);
    assert!(
        matches!(record.disposition, ObligationDisposition::Supported { .. }),
        "{record:#?}"
    );
    let harness = harness.expect("a supported item carries its harness");
    assert_eq!(harness.identity.operation_identity, "quire.op.integer.add");
    assert_eq!(
        harness
            .identity
            .arguments
            .iter()
            .map(|argument| (argument.minimum, argument.maximum))
            .collect::<Vec<_>>(),
        [(0, 9), (1, 1)]
    );
}

fn route_two_parameter(code: u32) -> (ObligationRecord, Option<KaniScalarObligationHarness>) {
    let (package, ids) = routed_two_parameter_package();
    let node_id = ids.resolve(&package::code_id(code));
    let routed = [route(0, &node_id)];
    let input = admitted_routing_input(&package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let (record, harness) = kani_parts(&item.output);
    (record.clone(), harness.cloned())
}

/// How the generated source spells an `i64` bound.
fn lit(value: i64) -> String {
    format!("{value}_i64")
}

fn ranges(harness: &KaniScalarObligationHarness) -> Vec<(i64, i64)> {
    harness
        .identity
        .arguments
        .iter()
        .map(|argument| (argument.minimum, argument.maximum))
        .collect()
}

/// `a + b` over `a: Int[0, 9]` and `b: Int[10, 20]` with the result typed `[0, 29]` routes to
/// Kani and is Supported: the harness ranges each argument over its own operand's bound and its
/// generated source asserts the result against the result bound (IR-298). `-e` over `[1, 9]` with
/// a `[-9, -1]` result and `e * f` over `[1, 9]` and `[100, 200]` with a `[100, 1800]` result are
/// Supported too: operand bounds need not lie inside the result bound. A literal operand has no
/// bound of its own and is ranged at its own value.
#[trace("FR-014-AC-20", "FR-014-AC-24", "FR-022-AC-13", "TC-024", "TC-033")]
#[test]
fn tc_033_bounded_parameters_are_supported_with_a_per_operand_scalar_harness() {
    for (code, identity, expected, result) in [
        (
            package::TWO_PARAMETER_SUM,
            "quire.op.integer.add",
            vec![(0, 9), (10, 20)],
            (0, 29),
        ),
        (
            package::TWO_PARAMETER_LITERAL,
            "quire.op.integer.add",
            vec![(0, 9), (1, 1)],
            (0, 29),
        ),
        (
            package::BOUNDED_NEGATE,
            "quire.op.integer.negate",
            vec![(1, 9)],
            (-9, -1),
        ),
        (
            package::BOUNDED_PRODUCT,
            "quire.op.integer.mul",
            vec![(1, 9), (100, 200)],
            (100, 1800),
        ),
        (
            package::TWO_PARAMETER_WIDE,
            "quire.op.integer.add",
            vec![(0, 9), (0, 50)],
            (0, 29),
        ),
    ] {
        let (record, harness) = route_two_parameter(code);
        assert!(
            matches!(record.disposition, ObligationDisposition::Supported { .. }),
            "node {code}: {record:#?}"
        );
        let harness = harness.expect("a supported item carries its harness");
        assert_eq!(harness.identity.operation_identity, identity);
        assert_eq!(ranges(&harness), expected, "node {code}");
        let (lower, upper) = result;
        assert!(
            harness.rust.contents.contains(&format!(
                "exact >= i128::from({}) && exact <= i128::from({});",
                lit(lower),
                lit(upper)
            )),
            "node {code}: the result assertion names the result bound"
        );
        let names: &[&str] = if expected.len() == 1 {
            &["operand"]
        } else {
            &["left", "right"]
        };
        for (name, (minimum, maximum)) in names.iter().zip(expected) {
            assert!(
                harness.rust.contents.contains(&format!(
                    "kani::assume({name} >= {} && {name} <= {});",
                    lit(minimum),
                    lit(maximum)
                )),
                "node {code}: {name} is assumed over its own range"
            );
        }
    }
}

/// A `reference` operand typed by a plain scalar type beside bounded typing is `RequiresBound`
/// with no harness: `x + y` (`x` bounded, `y` plain, result bounded), `y + y`, and `x + y` with a
/// scalar-typed result.
#[trace("FR-014-AC-25", "FR-022-AC-13", "TC-033")]
#[test]
fn tc_033_a_plain_typed_reference_operand_is_requires_bound_with_no_harness() {
    for code in [
        package::TWO_PARAMETER_UNBOUNDED_OPERAND,
        package::TWO_PARAMETER_PLAIN_PAIR,
        package::TWO_PARAMETER_PLAIN_SCALAR_RESULT,
    ] {
        let (record, harness) = route_two_parameter(code);
        assert!(
            matches!(
                record.disposition,
                ObligationDisposition::RequiresBound { .. }
            ),
            "node {code}: {record:#?}"
        );
        assert!(harness.is_none(), "node {code}: no harness");
    }
}

fn route_qsl_shaped(code: u32) -> (ObligationRecord, Option<KaniScalarObligationHarness>) {
    let mut builder = package::qsl_shaped_package();
    builder.expression_occurrences();
    let (package, ids) = builder.admit_resolved();
    route_qsl_shaped_in(&package, &ids, code)
}

fn route_qsl_shaped_in(
    package: &CheckedPackageV2,
    ids: &package::FixtureIds,
    code: u32,
) -> (ObligationRecord, Option<KaniScalarObligationHarness>) {
    let node_id = ids.resolve(&package::code_id(code));
    let routed = [route(0, &node_id)];
    let input = admitted_routing_input(package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let (record, harness) = kani_parts(&item.output);
    (record.clone(), harness.cloned())
}

fn qsl_shaped_routing_input_in(
    package: &CheckedPackageV2,
    ids: &package::FixtureIds,
    requests: &[(usize, u32)],
) -> (Vec<RoutedGenerationItem>, AdmittedRoutingInput) {
    let routed = requests
        .iter()
        .map(|(index, code)| route(*index, &ids.resolve(&package::code_id(*code))))
        .collect::<Vec<_>>();
    let input = admitted_routing_input(package, &routed);
    (routed, input)
}

/// Routed operand refusals produce no harness and leave an independently supported sibling's
/// record, harness and claim unchanged. InvalidBody is covered at the inference/lowering seam:
/// the checked V2 reader rejects malformed bodies before a package can enter routed generation.
#[trace("FR-014-AC-46", "FR-015-AC-104", "TC-033")]
#[test]
fn tc_033_routed_operand_refusals_leave_healthy_sibling_unchanged() {
    const DIVISION: u32 = 9_300_001;
    const NESTED: u32 = 9_300_002;
    const NESTED_NARROW: u32 = 9_300_003;

    let mut builder = package::qsl_shaped_package();
    let integer = package::key(package::T_INTEGER);
    let division_law =
        package::integer_division_definition(quire_exact::DivisionProfile::Truncating);
    builder.select_definition(division_law.clone());
    let division = package::application(
        "binary",
        package::op_full(
            "quire.op.integer.div",
            vec![package::law("integer_division", division_law)],
            None,
            None,
        ),
        &integer,
        vec![
            package::reference(&package::key(package::QSL_PARAM_X)),
            package::reference(&package::key(package::QSL_LITERAL_TWO)),
        ],
    );
    builder.application_bounded(DIVISION, "expression", "binary", &integer, division, &[]);
    let nested = package::application(
        "binary",
        package::op("quire.op.integer.add"),
        &integer,
        vec![
            package::reference(&package::code_id(DIVISION).digest),
            package::reference(&package::key(package::QSL_LITERAL_TWO)),
        ],
    );
    builder.application_bounded(NESTED, "expression", "binary", &integer, nested, &[]);
    let division_bound = builder.bound(&package::Bound::Integer(0, 10));
    builder.narrow(
        NESTED_NARROW,
        &package::code_id(NESTED).digest,
        &division_bound,
    );
    builder.expression_occurrences();
    let (checked, ids) = builder.admit_resolved();

    let (routed, input) = qsl_shaped_routing_input_in(
        &checked,
        &ids,
        &[
            (0, NESTED),
            (1, package::QSL_PLUS_TEXT),
            (2, package::QSL_SHARED),
        ],
    );
    let combined = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("the routed QSL-shaped batch is admitted");
    let (division_record, division_harness) = kani_parts(&combined.items[0].output);
    assert!(
        matches!(
            division_record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::OracleRefused {
                    refusal: ExactScalarRefusal::OperandUnsupported { .. }
                }
            }
        ),
        "division operand refusal: {division_record:#?}"
    );
    assert!(
        division_harness.is_none(),
        "unsupported operand emits no harness"
    );

    let (type_record, type_harness) = kani_parts(&combined.items[1].output);
    assert!(
        matches!(
            type_record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::OracleRefused {
                    refusal: ExactScalarRefusal::OperandTypeMismatch { .. }
                }
            }
        ),
        "operand type mismatch: {type_record:#?}"
    );
    assert!(type_harness.is_none(), "type mismatch emits no harness");

    let sibling_node = ids.resolve(&package::code_id(package::QSL_SHARED));
    let (sibling_record, sibling_harness) = kani_parts(&combined.items[2].output);
    assert!(matches!(
        sibling_record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    let sibling_harness = sibling_harness.expect("healthy sibling has its harness");

    let isolated = generate_routed(
        &input.package,
        &input.positions,
        &[routed[2].clone()],
        &kani_context("crate::subject", 1),
    )
    .expect("the supported sibling generates alone against the same admitted package");
    let (isolated_record, isolated_harness) = kani_parts(&isolated.items[0].output);
    let isolated_harness = isolated_harness.expect("isolated healthy sibling has its harness");
    assert_eq!(
        sibling_record, isolated_record,
        "sibling record is unchanged"
    );
    assert_eq!(
        sibling_harness, isolated_harness,
        "sibling harness is unchanged"
    );
    let claim = |generation: &RoutedGeneration| {
        generation
            .claim_map
            .as_ref()
            .and_then(|claims| {
                claims
                    .items
                    .iter()
                    .find(|item| item.node_id == sibling_node)
            })
            .expect("healthy sibling claim is present")
            .clone()
    };
    assert_eq!(
        claim(&combined),
        claim(&isolated),
        "sibling claim is unchanged"
    );
}

/// The three functions the driver compiles through real QSL, in the shape QSL emits them (a
/// literal as a reference to its own `value` node, the declared bounded result on a narrowing
/// `conversion` that consumes the plain-typed arithmetic node), each route to Kani and are
/// Supported: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` ranges `x` over `[0, 9]` and the
/// literal at its own value `[1, 1]`; `x + y` over `Int[0, 9]` and `Int[10, 20]` into
/// `Int[10, 29]` ranges each over its own bound; `-z` over `Int[0, 9]` into `Int[-9, 0]` ranges
/// `z` over `[0, 9]`. Each harness asserts the result against the conversion's bound.
#[trace(
    "FR-014-AC-26",
    "FR-014-AC-27",
    "FR-015-AC-16",
    "FR-022-AC-15",
    "TC-033"
)]
#[test]
fn tc_033_qsl_shaped_increment_sum_and_negation_are_supported() {
    for (code, identity, expected, result) in [
        (
            package::QSL_INC,
            "quire.op.integer.add",
            vec![(0, 9), (1, 1)],
            (0, 10),
        ),
        (
            package::QSL_ADD,
            "quire.op.integer.add",
            vec![(0, 9), (10, 20)],
            (10, 29),
        ),
        (
            package::QSL_NEGATE,
            "quire.op.integer.negate",
            vec![(0, 9)],
            (-9, 0),
        ),
    ] {
        let (record, harness) = route_qsl_shaped(code);
        assert!(
            matches!(record.disposition, ObligationDisposition::Supported { .. }),
            "node {code}: {record:#?}"
        );
        let harness = harness.expect("a supported item carries its harness");
        assert_eq!(harness.identity.operation_identity, identity);
        assert_eq!(ranges(&harness), expected, "node {code}");
        let (lower, upper) = result;
        assert!(
            harness.rust.contents.contains(&format!(
                "exact >= i128::from({}) && exact <= i128::from({});",
                lit(lower),
                lit(upper)
            )),
            "node {code}: the result assertion names the conversion's bound"
        );
    }
}

/// In the same QSL shape, a plain-Integer parameter beside a bounded one (`x + p`) and a reference
/// to a `value` node whose body is not a literal each settle `requires_bound` with no harness.
/// With two represented conversions, the routed package uses the caller's selected first
/// conversion rather than inferring an ambiguous result from the shared expression node.
#[trace(
    "FR-014-AC-28",
    "FR-014-AC-29",
    "FR-014-AC-32",
    "FR-022-AC-15",
    "TC-033"
)]
#[test]
fn tc_033_qsl_shaped_plain_operands_refuse_and_selected_narrowing_generates() {
    for code in [package::QSL_PLAIN_OPERAND, package::QSL_NOT_LITERAL_OPERAND] {
        let (record, harness) = route_qsl_shaped(code);
        assert!(
            matches!(
                record.disposition,
                ObligationDisposition::RequiresBound { .. }
            ),
            "node {code}: {record:#?}"
        );
        assert!(harness.is_none(), "node {code}: no harness");
    }
    let (record, harness) = route_qsl_shaped(package::QSL_TWICE_NARROWED);
    assert!(
        matches!(record.disposition, ObligationDisposition::Supported { .. }),
        "{record:#?}"
    );
    let harness = harness.expect("selected first narrow generates");
    assert!(harness
        .rust
        .contents
        .contains("exact >= i128::from(10_i64) && exact <= i128::from(29_i64);"));
}

const NESTED_QSL_SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
function add_nested using v(x: Int[0, 9]): Int[0, 20] pure { (x + 1) + 1 }\n\
function multiply_negative using v(x: Int[1, 9]): Int[-9, -1] pure { x * -1 }\n\
function negate_nested using v(x: Int[0, 9]): Int[-10, -1] pure { -(x + 1) }\n\
function plain_add_nested using v(x: Int[0, 9], p: Integer): Integer pure { (x + p) + 1 }\n\
function plain_negate_nested using v(x: Int[0, 9], p: Integer): Integer pure { -(x + p) }\n\
function huge_nested using v(x: Int[0, 9]): Int[0, 9] pure { (x + 100000000000000000000000) + -100000000000000000000000 }\n";

fn nested_qsl_package() -> CheckedPackageV2 {
    let compiled = compile_package(
        SourceIdentity::new("test", "ir306-nested.native", "git", "fixture"),
        "ir306-nested.native",
        NESTED_QSL_SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("nested operand fixture compiles through QSL's public facade");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("QSL-emitted nested operand package is admitted by IR: {other:?}"),
    }
}

fn deep_qsl_package() -> CheckedPackageV2 {
    let expression = (0..66).fold("x".to_owned(), |value, _| format!("({value} + 0)"));
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction deep_nested using v(x: Int[0, 9]): Int[0, 9] pure {{ {expression} }}\nfunction sibling using v(x: Int[0, 9]): Int[0, 10] pure {{ x + 1 }}\n"
    );
    let compiled = compile_package(
        SourceIdentity::new("test", "ir306-deep.native", "git", "fixture"),
        "ir306-deep.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("deep operand fixture compiles through QSL's public facade");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("QSL-emitted deep operand package is admitted by IR: {other:?}"),
    }
}

fn wide_qsl_package() -> CheckedPackageV2 {
    fn product_tree(start: u128, count: u128) -> String {
        if count == 1 {
            return format!("(x * {})", 10_u128.pow(38) - start);
        }
        let half = count / 2;
        format!(
            "({} * {})",
            product_tree(start, half),
            product_tree(start + half, count - half)
        )
    }
    let wide_product = product_tree(0, 40);
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction wide_nested using v(x: Int[1, 1]): Int[0, 0] pure {{ ({wide_product}) - ({wide_product}) }}\n"
    );
    let compiled = compile_package(
        SourceIdentity::new("test", "ir306-wide.native", "git", "fixture"),
        "ir306-wide.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("wide endpoint fixture compiles through QSL's public facade");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("QSL-emitted wide operand package is admitted by IR: {other:?}"),
    }
}

fn work_qsl_package() -> CheckedPackageV2 {
    fn balanced(start: u64, count: u64) -> String {
        if count == 1 {
            // Internal groups preserve the balanced shape; additive leaves need no extra
            // parentheses, keeping the source inside QSL's default syntax-node limit.
            return format!("x + {start}");
        }
        let half = count / 2;
        format!(
            "({} + {})",
            balanced(start, half),
            balanced(start + half, count - half)
        )
    }
    let expression = format!("({}) + 0", balanced(0, 2049));
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction work_nested using v(x: Int[0, 0]): Int[0, 2098176] pure {{ {expression} }}\nfunction sibling using v(x: Int[0, 9]): Int[0, 10] pure {{ x + 1 }}\n"
    );
    let compiled = compile_package(
        SourceIdentity::new("test", "ir306-work.native", "git", "fixture"),
        "ir306-work.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("high-work operand fixture compiles through QSL's public facade");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits {
            bytes: 16 << 20,
            ..CheckedPackageReadLimits::bounded()
        },
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("QSL-emitted high-work operand package is admitted by IR: {other:?}"),
    }
}

fn qsl_function_application(package: &CheckedPackageV2, function: &str) -> CheckedNodeId {
    let declaration = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration.as_ref().is_some_and(|declaration| {
                declaration.qualified_name.len() == 1
                    && declaration.qualified_name[0].as_ref() == function
            })
        })
        .unwrap_or_else(|| panic!("QSL output declares {function}"));
    let CheckedTerm::Aggregate { members } = &declaration.body else {
        panic!("QSL function {function} has aggregate members");
    };
    let body_id = members
        .iter()
        .find_map(|member| match member {
            CheckedTerm::Binding { name, value } if name.as_ref() == "body" => {
                reference_target(value)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("QSL function {function} has a body binding"))
        .clone();
    let body_node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == body_id)
        .unwrap_or_else(|| panic!("QSL function {function} body is in the graph"));
    if body_node.semantic_form.as_ref() == "conversion" {
        application_arguments(&body_node.body)
            .and_then(|arguments| arguments.first())
            .and_then(reference_target)
            .unwrap_or_else(|| panic!("QSL narrowing references its application"))
            .clone()
    } else {
        body_id
    }
}

fn route_qsl_function(
    package: &CheckedPackageV2,
    function: &str,
) -> (
    ObligationRecord,
    Option<KaniScalarObligationHarness>,
    RoutedGeneration,
) {
    let node_id = qsl_function_application(package, function);
    let routed = [route(0, &node_id)];
    let input = admitted_routing_input(package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("QSL-emitted application routes through public generation");
    let [item] = generation.items.as_slice() else {
        panic!("one routed QSL application, got {:?}", generation.items);
    };
    let (record, harness) = kani_parts(&item.output);
    (record.clone(), harness.cloned(), generation)
}

fn route_qsl_functions(
    package: &CheckedPackageV2,
    functions: &[&str],
    read_limits: CheckedPackageReadLimits,
) -> RoutedGeneration {
    let routed = functions
        .iter()
        .enumerate()
        .map(|(index, function)| route(index, &qsl_function_application(package, function)))
        .collect::<Vec<_>>();
    let input = admitted_routing_input_with_limits(package, &routed, read_limits);
    generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("QSL-emitted applications route through public generation")
}

/// The QSL compiler emits nested applications as references to plain-Integer expression nodes;
/// only leaves carry the authored bounded parameter or literal. Each routed outer operation must
/// infer a finite range through that graph, while a plain-Integer parameter at any nested depth
/// remains a typed refusal.
#[trace(
    "FR-014-AC-45",
    "FR-014-AC-46",
    "FR-015-AC-103",
    "FR-015-AC-104",
    "TC-024",
    "TC-033"
)]
#[test]
fn tc_033_qsl_emitted_nested_integer_operands_are_bounded_or_require_bounds() {
    let package = nested_qsl_package();
    for (function, identity, expected, result) in [
        (
            "add_nested",
            "quire.op.integer.add",
            vec![(1, 10), (1, 1)],
            (0, 20),
        ),
        (
            "multiply_negative",
            "quire.op.integer.mul",
            vec![(1, 9), (-1, -1)],
            (-9, -1),
        ),
        (
            "negate_nested",
            "quire.op.integer.negate",
            vec![(1, 10)],
            (-10, -1),
        ),
    ] {
        let (record, harness, generation) = route_qsl_function(&package, function);
        assert!(
            matches!(record.disposition, ObligationDisposition::Supported { .. }),
            "QSL function {function} must be Supported: {record:#?}"
        );
        let harness = harness.expect("Supported nested operation carries a harness");
        assert_eq!(harness.identity.operation_identity, identity, "{function}");
        assert_eq!(ranges(&harness), expected, "{function}");
        let [inferred] = harness.identity.inferred_operands.as_slice() else {
            panic!("{function} records one inferred operand source");
        };
        let inferred_position = usize::from(function == "multiply_negative");
        assert_eq!(
            inferred.position, inferred_position,
            "{function} inferred argument position"
        );
        assert_eq!(
            (inferred.lower.as_str(), inferred.upper.as_str()),
            match function {
                "multiply_negative" => ("-1", "-1"),
                _ => ("1", "10"),
            },
            "{function} exact inferred endpoints"
        );
        let outer = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == harness.identity.node_id)
            .expect("routed node is in the admitted graph");
        assert_eq!(
            &inferred.source_node_id,
            application_arguments(&outer.body)
                .and_then(|arguments| arguments.get(inferred_position))
                .and_then(reference_target)
                .expect("outer body references its direct nested expression"),
            "{function} identity names the direct nested expression"
        );
        assert!(
            harness.rust.contents.contains(&format!(
                "exact >= i128::from({}) && exact <= i128::from({});",
                lit(result.0),
                lit(result.1)
            )),
            "{function} asserts the outer declared result interval"
        );
        let claim = generation
            .claim_map
            .as_ref()
            .and_then(|claims| {
                claims
                    .items
                    .iter()
                    .find(|claim| claim.node_id == harness.identity.node_id)
            })
            .expect("the routed outer application has a scalar claim");
        assert!(
            matches!(
                claim.operation.provenance,
                quire_contract_codegen::OperationProvenance::IrConfirmed
            ),
            "{function} claim is IR-confirmed: {claim:#?}"
        );
    }

    for function in ["plain_add_nested", "plain_negate_nested"] {
        let (record, harness, _) = route_qsl_function(&package, function);
        assert!(
            matches!(
                record.disposition,
                ObligationDisposition::RequiresBound { .. }
            ),
            "nested plain parameter in {function} requires a bound: {record:#?}"
        );
        assert!(harness.is_none(), "{function} emits no harness");
    }

    let (huge, harness, _) = route_qsl_function(&package, "huge_nested");
    assert_eq!(
        huge.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::DomainNotRepresentableInI64 {
                operation_identity: "quire.op.integer.add".to_owned(),
                lower: "100000000000000000000000".to_owned(),
                upper: "100000000000000000000009".to_owned(),
            }
        }
    );
    assert!(
        harness.is_none(),
        "out-of-i64 inferred range emits no harness"
    );
}

#[trace("FR-014-AC-46", "FR-015-AC-104", "TC-024", "TC-033")]
#[test]
fn tc_033_qsl_nested_operand_depth_limit_is_typed_and_keeps_sibling_supported() {
    let package = deep_qsl_package();
    let (record, harness, _) = route_qsl_function(&package, "deep_nested");
    assert!(matches!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::OracleRefused {
                refusal: ExactScalarRefusal::OperandRangeLimitExceeded {
                    resource_kind: quire_contract_codegen::OperandRangeLimitKind::Depth,
                    limit: 64,
                    consumed: 65,
                    ..
                }
            }
        }
    ));
    assert!(harness.is_none(), "a limit refusal emits no harness");

    let package = wide_qsl_package();
    let (record, harness, _) = route_qsl_function(&package, "wide_nested");
    assert!(
        matches!(
            record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::OracleRefused {
                    refusal: ExactScalarRefusal::OperandRangeLimitExceeded {
                        resource_kind: quire_contract_codegen::OperandRangeLimitKind::EndpointBits,
                        limit: 4096,
                        consumed: 5050,
                        ..
                    }
                }
            }
        ),
        "wide endpoint disposition: {:#?}",
        record.disposition
    );
    assert!(
        harness.is_none(),
        "an endpoint-width refusal emits no harness"
    );

    let package = work_qsl_package();
    // Count the genuine producer's referenced operand graph, excluding inline literals and
    // type metadata. The fixture must cross the inference budget even when literals have no
    // graph nodes. Its balanced shape stays below the independent depth/endpoint ceilings.
    let work_node = qsl_function_application(&package, "work_nested");
    let graph = package
        .graph()
        .nodes
        .iter()
        .map(|node| (&node.node_id, node))
        .collect::<std::collections::BTreeMap<_, _>>();
    let operand = application_arguments(&graph[&work_node].body)
        .and_then(|arguments| arguments.first())
        .and_then(reference_target)
        .expect("work fixture has a referenced inference operand")
        .clone();
    let mut pending = vec![operand];
    let mut visited = std::collections::BTreeSet::new();
    while let Some(node_id) = pending.pop() {
        if !visited.insert(node_id.clone()) {
            continue;
        }
        let node = graph
            .get(&node_id)
            .expect("operand belongs to the checked graph");
        for argument in application_arguments(&node.body).into_iter().flatten() {
            if let Some(target) = reference_target(argument) {
                pending.push(target.clone());
            }
        }
    }
    assert_eq!(
        visited.len(),
        4098,
        "actual graph visits include 4097 applications and the shared parameter"
    );
    assert!(
        visited.len() > 4096,
        "the authored operand exceeds the inference work ceiling"
    );
    // This oversized package exists only to cross the scalar inference work ceiling; keep its
    // larger read limit local to this test path. Normal fixture readers retain the default 1 MiB.
    let work_read_limits = CheckedPackageReadLimits {
        bytes: 16 << 20,
        ..CheckedPackageReadLimits::bounded()
    };
    let generation = route_qsl_functions(&package, &["work_nested", "sibling"], work_read_limits);
    let (record, harness) = kani_parts(&generation.items[0].output);
    assert!(
        matches!(
            record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::OracleRefused {
                    refusal: ExactScalarRefusal::OperandRangeLimitExceeded {
                        resource_kind: quire_contract_codegen::OperandRangeLimitKind::Nodes,
                        limit: 4096,
                        consumed: 4097,
                        ..
                    }
                }
            }
        ),
        "work-limit disposition: {:#?}",
        record.disposition
    );
    assert!(harness.is_none(), "a work refusal emits no harness");
    let (sibling_record, sibling_harness) = kani_parts(&generation.items[1].output);
    assert!(matches!(
        sibling_record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    let isolated = route_qsl_functions(&package, &["sibling"], work_read_limits);
    let (isolated_record, isolated_harness) = kani_parts(&isolated.items[0].output);
    assert_eq!(
        sibling_record.disposition, isolated_record.disposition,
        "work refusal leaves sibling disposition unchanged"
    );
    assert_eq!(sibling_record.subject, isolated_record.subject);
    let sibling_node = qsl_function_application(&package, "sibling");
    let combined_claim = generation
        .claim_map
        .as_ref()
        .and_then(|claims| {
            claims
                .items
                .iter()
                .find(|claim| claim.node_id == sibling_node)
        })
        .expect("combined generation retains sibling claim");
    let isolated_claim = isolated
        .claim_map
        .as_ref()
        .and_then(|claims| {
            claims
                .items
                .iter()
                .find(|claim| claim.node_id == sibling_node)
        })
        .expect("isolated generation retains sibling claim");
    assert_eq!(combined_claim.node_id, isolated_claim.node_id);
    assert_eq!(combined_claim.operation, isolated_claim.operation);
    match (&combined_claim.result, &isolated_claim.result) {
        (ClaimDisposition::Generated(combined), ClaimDisposition::Generated(isolated)) => {
            assert_eq!(combined.symbol, isolated.symbol);
            assert_eq!(combined.semantic_form, isolated.semantic_form);
            assert_eq!(combined.semantic_type, isolated.semantic_type);
            assert_eq!(combined.source_map, isolated.source_map);
            assert_eq!(combined.claims, isolated.claims);
            assert_eq!(combined.bounds, isolated.bounds);
            assert_eq!(combined.checked_bounds, isolated.checked_bounds);
            assert_eq!(combined.dependencies, isolated.dependencies);
            assert_eq!(combined.oracle_source, isolated.oracle_source);
            // These identities describe the selected request package and vary with selection.
        }
        (combined, isolated) => assert_eq!(combined, isolated),
    }
    assert_eq!(
        sibling_harness, isolated_harness,
        "work refusal leaves sibling harness unchanged"
    );

    let package = nested_qsl_package();
    let (supported, harness, _) = route_qsl_function(&package, "add_nested");
    assert!(matches!(
        supported.disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert!(
        harness.is_some(),
        "a refused sibling cannot suppress a supported harness"
    );
}

/// A literal operand is constrained to its own value, so a QSL-shaped node the literal pushes
/// outside the result bound for every `x` has no harness: `x + 100` over `x: Int[0, 9]` into
/// `Int[0, 10]` reaches only `[100, 109]` and is `result_bound_unreachable`, and `x + 10^23`,
/// whose literal does not fit `i64`, is `domain_not_representable_in_i64` naming it. `x + "a"`
/// (a text literal in an Integer-typed `value` node) and `x + 2`, narrowed and also consumed by
/// `(x + 2) + x`, is `oracle_refused` with `OperandTypeMismatch`; the shared
/// expression's own selected narrow remains unambiguous on the package path.
#[trace(
    "FR-015-AC-17",
    "FR-015-AC-18",
    "FR-014-AC-31",
    "FR-014-AC-34",
    "TC-033"
)]
#[test]
fn tc_033_qsl_shaped_literal_and_consumer_refusals_have_no_harness() {
    let mut builder = package::qsl_shaped_package();
    builder.expression_occurrences();
    let (package, ids) = builder.admit_resolved();
    let routed = |code| route_qsl_shaped_in(&package, &ids, code);
    let (record, harness) = routed(package::QSL_PLUS_HUNDRED);
    assert_eq!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::ResultBoundUnreachable {
                operation_identity: "quire.op.integer.add".to_owned(),
                lower: 0,
                upper: 10,
                reachable_lower: "100".to_owned(),
                reachable_upper: "109".to_owned(),
            }
        }
    );
    assert!(harness.is_none());
    let huge = "100000000000000000000000".to_owned();
    let (record, harness) = routed(package::QSL_PLUS_HUGE);
    assert_eq!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::DomainNotRepresentableInI64 {
                operation_identity: "quire.op.integer.add".to_owned(),
                lower: huge.clone(),
                upper: huge,
            }
        }
    );
    assert!(harness.is_none());
    let (record, harness) = routed(package::QSL_PLUS_TEXT);
    assert_eq!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::OracleRefused {
                refusal: ExactScalarRefusal::OperandTypeMismatch {
                    position: 1,
                    expected: quire_contract_codegen::ScalarForm::Integer,
                    found: Some("text".to_owned()),
                },
            },
        }
    );
    assert!(harness.is_none());
    let (record, harness) = routed(package::QSL_SHARED);
    assert!(matches!(
        record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    let harness = harness.expect("selected own narrow generates");
    assert!(harness
        .rust
        .contents
        .contains("exact >= i128::from(0_i64) && exact <= i128::from(11_i64);"));
}

/// The corpus package with the nodes derivation refuses, and the claims of what it derives.
struct Derived {
    package: CheckedPackageV2,
    /// The three rendered nodes and one unrendered node, resolved in this package.
    generated: Vec<CheckedNodeId>,
    rem: CheckedNodeId,
    /// The quantity conversion nodes, one per node code.
    quantity_conversions: Vec<CheckedNodeId>,
}

fn derived() -> Derived {
    let fixture = fixture();
    let mut builder = package::derivation_package();
    let corpus = package::corpus();
    for (index, node) in fixture
        .rendered
        .iter()
        .chain(std::iter::once(&fixture.unrendered))
        .enumerate()
    {
        let expression = corpus
            .iter()
            .find(|expression| fixture.ids.resolve(&package::code_id(expression.code)) == *node)
            .expect("selected scalar expression belongs to corpus");
        let result = builder.bound(expression.bounds.first().expect("selected result bound"));
        builder.narrow(
            49_200 + u32::try_from(index).expect("four selected expressions"),
            &package::code_id(expression.code).digest,
            &result,
        );
    }
    builder.expression_occurrences();
    let (package, ids) = builder.admit_resolved();
    let generated = fixture
        .rendered
        .iter()
        .chain(std::iter::once(&fixture.unrendered))
        .map(|node| {
            let code = corpus
                .iter()
                .find(|expression| fixture.ids.resolve(&package::code_id(expression.code)) == *node)
                .map(|expression| expression.code)
                .expect("generated node belongs to the authored scalar corpus");
            ids.resolve(&package::code_id(code))
        })
        .collect();
    Derived {
        package,
        generated,
        rem: ids.resolve(&package::code_id(package::DERIVE_REM)),
        quantity_conversions: package::QUANTITY_CONVERSIONS
            .iter()
            .map(|code| ids.resolve(&package::code_id(*code)))
            .collect(),
    }
}

/// An underivable item keeps its typed refusal, `no_derivable_claim` naming the node and the
/// derivation refusal, with no harness; the group is not rejected and its siblings' records and
/// harnesses equal what they are without it.
#[trace("FR-022-AC-11", "FR-015-AC-15", "TC-033")]
#[test]
fn tc_033_an_underivable_item_keeps_its_refusal_and_its_siblings_are_unaffected() {
    let derived = derived();
    let contexts = kani_context("crate::subject", 1);
    let with_siblings = derived
        .generated
        .iter()
        .enumerate()
        .map(|(index, node)| route(index, node))
        .collect::<Vec<_>>();
    let (refused, identity) = (&derived.rem, "quire.op.integer.rem");
    let mut routed = with_siblings.clone();
    routed.push(route(9, refused));
    let mixed_input = admitted_routing_input(&derived.package, &routed);
    let alone = generate_routed(
        &mixed_input.package,
        &mixed_input.positions,
        &with_siblings,
        &contexts,
    )
    .expect("generates siblings from the same admitted package");
    let mixed = generate_routed(
        &mixed_input.package,
        &mixed_input.positions,
        &routed,
        &contexts,
    )
    .expect("generates");
    assert!(
        mixed.rejected.is_empty(),
        "an underivable item must not reject the group"
    );
    let (record, harness) = kani_parts(&mixed.items[4].output);
    assert_eq!(mixed.items[4].request_index, 9);
    assert!(harness.is_none());
    assert_eq!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::NoDerivableClaim {
                node_id: refused.clone(),
                reason: ClaimDerivationRefusal::OperationNotDerivable {
                    operation_identity: identity.to_owned()
                },
            }
        }
    );
    let wire = serde_json::to_value(&record.disposition).expect("serializes");
    assert_eq!(wire["reason"]["code"], "no_derivable_claim");
    assert_eq!(wire["reason"]["reason"]["code"], "operation_not_derivable");
    assert_eq!(wire["reason"]["reason"]["operation_identity"], identity);
    // The siblings are unaffected: record and harness equal the run without the refused item.
    assert_eq!(mixed.items[..4], alone.items[..]);
    assert_eq!(
        mixed.items[..4]
            .iter()
            .filter(|item| kani_parts(&item.output).1.is_some())
            .count(),
        3
    );
}

/// Exact quantity conversions generate an oracle and settle Kani as OperationNotRendered.
#[trace(
    "FR-014-AC-2",
    "FR-014-AC-51",
    "FR-014-AC-55",
    "FR-022-AC-11",
    "FR-022-AC-30",
    "FR-015-AC-15",
    "TC-033"
)]
#[test]
fn tc_033_quantity_conversion_has_an_oracle_and_no_harness() {
    let derived = derived();
    let contexts = kani_context("crate::subject", 1);
    for refused in &derived.quantity_conversions {
        let routed = vec![route(0, refused)];
        let input = admitted_routing_input(&derived.package, &routed);
        let generated = generate_routed(&input.package, &input.positions, &routed, &contexts)
            .expect("generates");
        assert!(generated.rejected.is_empty());
        let (record, harness) = kani_parts(&generated.items[0].output);
        assert!(harness.is_none(), "no harness is emitted");
        let claim_map = generated.claim_map.as_ref().expect("claim map");
        assert!(
            claim_map
                .items
                .iter()
                .all(|claim| matches!(claim.result, ClaimDisposition::Generated(_))),
            "quantity conversion must generate every claim: {claim_map:?}"
        );
        let oracle_artifacts = generated.oracle_artifacts.as_ref().expect("oracle crate");
        let oracle_source = oracle_artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("oracle lib.rs");
        assert!(
            oracle_source
                .contents
                .contains("quantity_owner::convert_quantity"),
            "the exact conversion oracle is emitted"
        );
        assert!(
            matches!(&record.disposition, ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::OperationNotRendered { operation_identity, .. }
        } if operation_identity == "quire.op.quantity.convert")
        );
        let wire = serde_json::to_value(&record.disposition).expect("serializes");
        assert_eq!(wire["reason"]["code"], "operation_not_rendered");
    }
}

/// A plain V1 read has the same wire and accounting but lacks authenticated operation views.
#[trace(
    "FR-014-AC-51",
    "FR-014-AC-55",
    "FR-022-AC-11",
    "FR-022-AC-30",
    "TC-024",
    "TC-033"
)]
#[test]
fn tc_033_quantity_conversion_requires_authenticated_operation_views() {
    let derived = derived();
    let node_id = derived
        .quantity_conversions
        .first()
        .expect("genuine quantity conversion");
    let routed = [route(0, node_id)];
    let input = admitted_routing_input(&derived.package, &routed);
    let roots = input
        .positions
        .iter()
        .map(|position| position.node_id().clone())
        .collect::<Vec<_>>();
    let lowered = derived
        .package
        .lower(&roots, &routed_contract_lowering_profile());
    let plain = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("plain V1 package must admit: {other:?}"),
    };
    let contexts = kani_context("crate::subject", 1);
    let authenticated = generate_routed(&input.package, &input.positions, &routed, &contexts)
        .expect("source-backed package generates the quantity claim");
    assert!(authenticated
        .claim_map
        .as_ref()
        .expect("quantity claim map")
        .items
        .iter()
        .all(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));
    assert!(matches!(
        generate_routed(&plain, &input.positions, &routed, &contexts),
        Err(RoutedGenerationError::UnauthenticatedOperationView {
            request_index: 0,
            node_id: refused,
        }) if refused == *node_id
    ));
}

/// The lowest affected driver index wins even when the caller permutes routed items.
#[trace("FR-014-AC-55", "FR-022-AC-30", "TC-033")]
#[test]
fn tc_033_unauthenticated_view_refusal_uses_lowest_request_index() {
    let derived = derived();
    let conversion = derived
        .quantity_conversions
        .first()
        .expect("genuine quantity conversion");
    let division = &derived
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| operation_identity(&node.body) == Some("quire.op.integer.div"))
        .expect("genuine integer division")
        .node_id;
    let integer = derived.generated.first().expect("genuine integer add");
    let routed = [route(0, integer), route(1, division), route(2, conversion)];
    let input = admitted_routing_input(&derived.package, &routed);
    let roots = input
        .positions
        .iter()
        .map(|position| position.node_id().clone())
        .collect::<Vec<_>>();
    let lowered = derived
        .package
        .lower(&roots, &routed_contract_lowering_profile());
    let plain = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("plain V1 package must admit: {other:?}"),
    };
    let contexts = kani_context("crate::subject", 1);
    let integer_only = generate_routed(&plain, &input.positions, &routed[..1], &contexts)
        .expect("plain-read integer route keeps its result");
    assert_eq!(integer_only.items.len(), 1);
    assert_eq!(integer_only.items[0].request_index, 0);
    assert!(matches!(
        kani_parts(&integer_only.items[0].output).0.disposition,
        ObligationDisposition::Supported { .. }
    ));
    for order in [[2, 0, 1], [1, 2, 0]] {
        let permuted = order.map(|index| routed[index].clone());
        assert!(matches!(
            generate_routed(&plain, &input.positions, &permuted, &contexts),
            Err(RoutedGenerationError::UnauthenticatedOperationView {
                request_index: 1,
                node_id,
            }) if node_id == *division
        ));
    }
}

/// `RoutedGeneration.claim_map` holds the four generated package claims and the
/// `NoDerivableClaim` underivable node, in node-id order; it is `None`
/// with nothing routed.
#[trace("FR-022-AC-12", "TC-033")]
#[test]
fn tc_033_the_returned_claim_map_orders_package_claims_and_refusals() {
    let derived = derived();
    let contexts = kani_context("crate::subject", 1);
    let nodes = derived
        .generated
        .iter()
        .chain([&derived.rem])
        .cloned()
        .collect::<Vec<_>>();
    let routed = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| route(index, node))
        .collect::<Vec<_>>();
    let input = admitted_routing_input(&derived.package, &routed);
    let generation =
        generate_routed(&input.package, &input.positions, &routed, &contexts).expect("generates");
    let claim_map = generation.claim_map.expect("a Kani group ran");

    let mut sorted = nodes.clone();
    sorted.sort();
    assert_eq!(claim_map.items.len(), 5);
    let refused = claim_map
        .items
        .iter()
        .filter(|claim| {
            matches!(
                &claim.result,
                ClaimDisposition::Refused {
                    refusal: ExactScalarRefusal::NoDerivableClaim { .. }
                }
            )
        })
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    let mut underivable = vec![derived.rem.clone()];
    underivable.sort();
    assert_eq!(refused, underivable);
    let generated = claim_map
        .items
        .iter()
        .filter_map(|claim| match &claim.result {
            ClaimDisposition::Generated(value) => Some((claim.node_id.clone(), value)),
            ClaimDisposition::Refused { .. } => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(generated.len(), 4);
    let mut expected_generated = derived.generated.clone();
    expected_generated.sort();
    assert_eq!(
        generated.iter().map(|(node, _)| node).collect::<Vec<_>>(),
        expected_generated.iter().collect::<Vec<_>>()
    );
    assert!(generated.iter().all(|(node, value)| {
        value.node_id == *node && value.contract_package_identity == *input.package.identity()
    }));
    let ids = claim_map
        .items
        .iter()
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(ids, sorted, "claims ascend by node id");

    let empty_input = admitted_routing_input(&derived.package, &[]);
    let empty = generate_routed(&empty_input.package, &empty_input.positions, &[], &contexts)
        .expect("generates");
    assert_eq!(empty.claim_map, None);
}

/// Which node each lowering and bound refusal is exercised on (TC-025's fixture).
const UNSATISFIABLE: u32 = 3001;

/// The record `generate_routed` returns for `node` routed alone, with whether the group rejected.
fn routed_alone(package: &CheckedPackageV2, node: &CheckedNodeId) -> (ObligationRecord, bool) {
    let routed = [route(0, node)];
    let input = admitted_routing_input(package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let (record, harness) = kani_parts(&item.output);
    assert!(harness.is_none(), "a refused item has no harness");
    (record.clone(), !generation.rejected.is_empty())
}

/// An item derivation cannot build a descriptor for because its node is absent, does not lower or
/// has a refused bound keeps the FR-015 disposition it had when the caller supplied the claim:
/// a node absent from the graph is `invalid_request` and rejects the group; the others are
/// `requires_bound`, `blocked_on_upstream` or `unsatisfiable_bound`, and reject nothing. The
/// noncanonical `UNREADABLE` shape is tested at the reader boundary in TC-052.
#[trace("FR-022-AC-8", "FR-015-AC-3", "FR-015-AC-5", "FR-015-AC-14", "TC-033")]
#[test]
fn tc_033_lowering_and_bound_refusals_keep_their_fr015_dispositions() {
    let (package, _, ids) = routed_scalar_package();
    let node = |code| ids.resolve(&package::code_id(code));

    let missing = node(package::MISSING);
    let missing_route = [route(0, &missing)];
    let missing_input = admitted_routing_input(&package, &missing_route);
    let missing_generation = generate_routed(
        &missing_input.package,
        &missing_input.positions,
        &missing_route,
        &kani_context("crate::subject", 1),
    )
    .expect("FR-330 accounts the absent request");
    assert_eq!(
        missing_generation.items[0].admitted.record,
        ContractFeatureRecordV1::InvalidInput { node_id: missing }
    );
    assert_eq!(
        missing_generation.items[0].output,
        KindOutput::PackageRefused
    );
    assert!(missing_generation.oracle_artifacts.is_none());

    let requires_bound = |code, unbounded_type: Option<CheckedNodeId>| {
        let (record, rejected) = routed_alone(&package, &node(code));
        assert!(!rejected, "node {code} must not reject the group");
        match (record.disposition, unbounded_type) {
            (
                ObligationDisposition::RequiresBound {
                    unbounded_type: found,
                },
                Some(expected),
            ) => {
                assert_eq!(found, expected, "node {code}");
            }
            (ObligationDisposition::RequiresBound { .. }, None) => {}
            (other, _) => panic!("node {code}: {other:?}"),
        }
    };
    requires_bound(package::UNBOUNDED, Some(node(package::T_INTEGER)));
    requires_bound(package::WRONG_BOUND_FORM, None);

    let unsupported = |code| {
        let (record, rejected) = routed_alone(&package, &node(code));
        assert!(!rejected, "node {code} must not reject the group");
        match record.disposition {
            ObligationDisposition::Unsupported { reason } => reason,
            other => panic!("node {code}: {other:?}"),
        }
    };
    assert_eq!(
        unsupported(package::FUNCTION),
        UnsupportedObligation::BlockedOnUpstream {
            node_id: node(package::FUNCTION),
            node_tag: "function",
            issue: quire_contract_codegen::UpstreamBlocker::QuireContractRuntime34,
        }
    );
    assert!(matches!(
        unsupported(UNSATISFIABLE),
        UnsupportedObligation::UnsatisfiableBound { .. }
    ));
}

/// Repeating one selected application/conversion pair keeps FR-039's typed
/// `DuplicateRequest` claim refusal at both positions. FR-015 still names the
/// first driver's index when it rejects the second item as a duplicate.
#[trace("FR-022-AC-7", "FR-022-AC-12", "TC-033")]
#[test]
fn tc_033_a_selected_pair_routed_twice_retains_its_duplicate_refusal() {
    let fixture = fixture();
    let node = &fixture.rendered[0];
    let routed = [route(5, node), route(3, node)];
    let input = admitted_routing_input(&fixture.package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    assert_eq!(generation.rejected, [BackendKind::Kani]);
    assert_eq!(
        generation
            .items
            .iter()
            .map(|item| item.request_index)
            .collect::<Vec<_>>(),
        [3, 5]
    );
    let (first, _) = kani_parts(&generation.items[0].output);
    assert_eq!(
        first.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::OracleRefused {
                refusal: ExactScalarRefusal::DuplicateRequest,
            },
        }
    );
    let (second, _) = kani_parts(&generation.items[1].output);
    assert_eq!(
        second.disposition,
        ObligationDisposition::InvalidRequest {
            reason: InvalidObligationItem::DuplicateItem { first_index: 3 }
        }
    );
    let claim_map = generation.claim_map.expect("a Kani group ran");
    assert_eq!(claim_map.items.len(), 2);
    assert!(claim_map.items.iter().all(|claim| matches!(
        claim.result,
        ClaimDisposition::Refused {
            refusal: ExactScalarRefusal::DuplicateRequest
        }
    )));
}

/// A refused derivation is an `Underived` claim carrying the node's operation
/// identity. An absent sibling keeps IR's prior `InvalidInput` record.
#[trace("FR-014-AC-18", "FR-022-AC-12", "FR-015-AC-109", "TC-033")]
#[test]
fn tc_033_an_underivable_claim_is_underived_and_carries_the_nodes_identity() {
    let derived = derived();
    let routed = [
        route(0, &derived.rem),
        route(1, &package::code_id(package::MISSING)),
    ];
    let input = admitted_routing_input(&derived.package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let claim_map = generation.claim_map.expect("a Kani group ran");
    let claim = claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == derived.rem)
        .expect("the rem node has a claim");
    assert_eq!(claim.operation.identity, "quire.op.integer.rem");
    assert_eq!(
        claim.operation.provenance,
        quire_contract_codegen::OperationProvenance::Underived
    );
    assert!(
        matches!(claim.result, ClaimDisposition::Refused { .. }),
        "remainder has no generated scalar family"
    );
    let (record, harness) = kani_parts(&generation.items[0].output);
    assert!(harness.is_none(), "underived remainder emits no harness");
    assert_eq!(
        record.disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::NoDerivableClaim {
                node_id: derived.rem.clone(),
                reason: ClaimDerivationRefusal::OperationNotDerivable {
                    operation_identity: "quire.op.integer.rem".to_owned(),
                },
            },
        }
    );
    assert_eq!(claim_map.items.len(), 1);
    assert_eq!(
        generation.items[1].admitted.record,
        ContractFeatureRecordV1::InvalidInput {
            node_id: package::code_id(package::MISSING)
        }
    );
    assert_eq!(generation.items[1].output, KindOutput::PackageRefused);
}

/// `x + 1` over `Int[0, 9]` returns the package-backed oracle crate,
/// defining every `Generated` claim's symbol referenced by its harness.
#[trace("FR-022-AC-14", "TC-033")]
#[test]
fn tc_033_the_oracle_crate_is_returned_and_defines_the_symbol_the_harness_references() {
    let (package, ids) = routed_bounded_increment_package();
    let node_id = ids.resolve(&package::code_id(package::BOUNDED_INCREMENT));
    let routed = [route(0, &node_id)];
    let input = admitted_routing_input(&package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("routed generation succeeds");
    let artifacts = generation.oracle_artifacts.expect("a Kani group ran");
    let paths = artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        paths,
        [
            "Cargo.toml",
            "src/lib.rs",
            "source-maps/lib.json",
            "claim-map.json"
        ]
    );
    let lib = &artifacts[1].contents;

    let claim_map = generation.claim_map.expect("a Kani group ran");
    let symbols = claim_map
        .items
        .iter()
        .filter_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated) => Some(generated.symbol.clone()),
            ClaimDisposition::Refused { .. } => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(symbols.len(), 1, "the one routed node generates");
    let [item] = generation.items.as_slice() else {
        panic!("one routed item, got {:?}", generation.items);
    };
    let (_, harness) = kani_parts(&item.output);
    let harness = harness.expect("a supported item carries its harness");
    for symbol in &symbols {
        assert!(
            lib.contains(&format!("fn {symbol}(")),
            "src/lib.rs defines {symbol}"
        );
        assert!(
            harness.rust.contents.contains(symbol.as_str()),
            "the harness references {symbol}"
        );
    }
}

/// A group with no generated node still returns its package-backed crate,
/// and nothing routed returns no crate.
#[trace("FR-022-AC-14", "TC-033")]
#[test]
fn tc_033_an_all_underivable_group_returns_package_crate_and_no_group_returns_none() {
    let derived = derived();
    let contexts = kani_context("crate::subject", 1);
    let routed = [route(0, &derived.rem)];
    let input = admitted_routing_input(&derived.package, &routed);
    let generation =
        generate_routed(&input.package, &input.positions, &routed, &contexts).expect("generates");
    let artifacts = generation.oracle_artifacts.expect("a Kani group ran");
    assert!(!artifacts.is_empty());
    assert_eq!(
        generation.claim_map.as_ref().map(|map| map.items.len()),
        Some(1)
    );
    let empty_input = admitted_routing_input(&derived.package, &[]);
    let empty = generate_routed(&empty_input.package, &empty_input.positions, &[], &contexts)
        .expect("generates");
    assert_eq!(empty.oracle_artifacts, None);
}

/// The FR-330 package path serializes the complete claim map, including an
/// underivable sibling's typed `NoDerivableClaim` refusal.
#[trace("FR-022-AC-14", "TC-033")]
#[test]
fn tc_033_the_returned_claim_map_file_keeps_underivable_refusals() {
    let derived = derived();
    let routed = [route(0, &derived.generated[0]), route(1, &derived.rem)];
    let input = admitted_routing_input(&derived.package, &routed);
    let generation = generate_routed(
        &input.package,
        &input.positions,
        &routed,
        &kani_context("crate::subject", 1),
    )
    .expect("generates");
    let claim_map = generation.claim_map.expect("a Kani group ran");
    assert_eq!(claim_map.items.len(), 2);
    let artifacts = generation.oracle_artifacts.expect("a Kani group ran");
    let file = artifacts
        .iter()
        .find(|artifact| artifact.path == "claim-map.json")
        .expect("the crate carries its claim map");
    let on_file: serde_json::Value = serde_json::from_str(&file.contents).expect("json");
    assert_eq!(on_file["items"].as_array().map(Vec::len), Some(2));
    assert_eq!(
        on_file,
        serde_json::to_value(&claim_map).expect("typed claim map")
    );
    assert!(claim_map.items.iter().any(|claim| {
        claim.node_id == derived.rem
            && matches!(
                claim.result,
                ClaimDisposition::Refused {
                    refusal: ExactScalarRefusal::NoDerivableClaim { .. }
                }
            )
    }));
}
