//! FR-022 routed generation: `generate_routed` runs the routed kind's generation arm keyed by the
//! driver's request index, and takes the routed backend and kind as given.
//!
//! The fixtures are TC-024/TC-025's: `kani_obligations::scalar_package` and its claim map.

use std::fs;

use quire_contract_codegen::{
    generate_routed, negotiate_kani_obligations, BackendKind, Candidate, ClaimDisposition,
    GenerationContexts, InvalidObligationItem, KaniGenerationContext, KaniObligationError,
    KaniObligationOutcome, KaniObligationRequest, KaniPinField, KaniScalarObligationHarness,
    KaniToolPins, KindOutput, ObligationDisposition, ObligationItem, ObligationRecord,
    RoutedGeneration, RoutedGenerationError, RoutedGenerationItem, UnsupportedObligation,
};
use quire_contract_ir::{CheckedNodeId, CheckedPackageV2};

use crate::kani_obligations::{context, package, pins, scalar_package};

const RENDERED: [&str; 3] = [
    "quire.op.integer.add",
    "quire.op.integer.sub",
    "quire.op.integer.negate",
];

struct Fixture {
    package: CheckedPackageV2,
    claim_map: quire_contract_codegen::ClaimMap<quire_contract_codegen::ExactScalarClaim>,
    /// Three nodes FR-014 generates and FR-015 renders.
    rendered: Vec<CheckedNodeId>,
    /// A generated node FR-015 refuses as `OperationNotRendered` without invalidating the request.
    unrendered: CheckedNodeId,
}

fn fixture() -> Fixture {
    let (package, claim_map) = scalar_package();
    let generated = claim_map
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
    Fixture {
        package,
        claim_map,
        rendered,
        unrendered,
    }
}

fn kani_backend(digest: &str) -> Candidate {
    Candidate {
        identity: "kani".to_owned(),
        manifest_digest: digest.to_owned(),
    }
}

fn route(request_index: usize, node_id: &CheckedNodeId, digest: &str) -> RoutedGenerationItem {
    RoutedGenerationItem {
        request_index,
        node_id: node_id.clone(),
        backend: kani_backend(digest),
        kind: BackendKind::Kani,
    }
}

fn kani_context<'a>(
    fixture: &'a Fixture,
    pins: &'a KaniToolPins,
    subject_path: &'a str,
    unwind: u32,
) -> GenerationContexts<'a> {
    GenerationContexts {
        kani: Some(KaniGenerationContext {
            claim_map: &fixture.claim_map,
            subject_path,
            pins,
            unwind,
            attestation: context(),
        }),
    }
}

/// What FR-015 returns for `nodes` in the given (ascending) order.
fn fr015(
    fixture: &Fixture,
    nodes: &[&CheckedNodeId],
) -> (Vec<ObligationRecord>, Vec<KaniScalarObligationHarness>) {
    let items = nodes
        .iter()
        .map(|node_id| ObligationItem::ScalarClaim {
            package: &fixture.package,
            claim_map: &fixture.claim_map,
            node_id,
        })
        .collect::<Vec<_>>();
    let pins = pins();
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        items: &items,
        subject_path: "crate::subject",
        pins: &pins,
        unwind: 1,
        attestation: context(),
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
        KindOutput::Kani { record, harness } => (record, harness.as_ref()),
    }
}

/// The four-item routing of TC-033 step 1: three rendered nodes and the unrendered one at the
/// non-contiguous indexes 7, 2, 11 and 4.
fn step_one_routing(fixture: &Fixture) -> Vec<RoutedGenerationItem> {
    vec![
        route(7, &fixture.rendered[0], "A"),
        route(2, &fixture.rendered[1], "A"),
        route(11, &fixture.rendered[2], "A"),
        route(4, &fixture.unrendered, "A"),
    ]
}

fn generate_step_one(fixture: &Fixture, routed: &[RoutedGenerationItem]) -> RoutedGeneration {
    let pins = pins();
    generate_routed(
        &fixture.package,
        routed,
        &kani_context(fixture, &pins, "crate::subject", 1),
    )
    .expect("routed generation succeeds")
}

/// Every routed Kani item equals what FR-015 returns in ascending request-index order, with the
/// driver's index in every record, and each harness is joined by `harness_symbol`.
///
/// Trace: FR-022-AC-2, FR-022-AC-8, TC-033
#[test]
fn tc_033_routed_kani_items_equal_fr015_output_keyed_by_request_index() {
    let fixture = fixture();
    let generation = generate_step_one(&fixture, &step_one_routing(&fixture));
    let (records, harnesses) = fr015(
        &fixture,
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
        assert_eq!(item.backend, kani_backend("A"));
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

/// The entry point takes no settlement input and constructs no FR-019 disposition. The
/// FR-019-AC-5 scan (`capability_settlement`) reads `src/`, which includes this module.
///
/// Trace: FR-022-AC-3, TC-033
#[test]
fn tc_033_the_entry_point_takes_no_settlement_input() {
    // The coercion fails to compile if the signature gains a manifest, candidate set, extent or
    // capability kind.
    let _: for<'a> fn(
        &CheckedPackageV2,
        &[RoutedGenerationItem],
        &GenerationContexts<'a>,
    ) -> Result<RoutedGeneration, RoutedGenerationError> = generate_routed;
    let source = fs::read_to_string("src/routed_generation.rs").expect("the module reads");
    for settlement_type in [
        "BackendDescriptor",
        "Candidates",
        "ExtentClassification",
        "CapabilityKind",
    ] {
        assert!(
            !source.contains(settlement_type),
            "the module names settlement type {settlement_type}"
        );
    }
    assert_eq!(
        source.matches("Disposition").count(),
        source.matches("ObligationDisposition").count(),
        "the module names an FR-019 disposition"
    );
}

/// A backend whose identity converts to no kind refuses the whole call, and the refusal names the
/// lowest offending index even when duplicates are also present.
///
/// Trace: FR-022-AC-4, TC-033
#[test]
fn tc_033_a_routed_kind_the_backend_does_not_have_refuses_the_call() {
    let fixture = fixture();
    let pins = pins();
    let contexts = kani_context(&fixture, &pins, "crate::subject", 1);
    let foreign = |index: usize| RoutedGenerationItem {
        request_index: index,
        node_id: fixture.rendered[0].clone(),
        backend: Candidate {
            identity: "not-a-backend".to_owned(),
            manifest_digest: "A".to_owned(),
        },
        kind: BackendKind::Kani,
    };
    let expected = |index: usize| RoutedGenerationError::BackendKindDisagrees {
        request_index: index,
        backend: foreign(index).backend,
        routed: BackendKind::Kani,
        converted: None,
    };
    let routed = [route(1, &fixture.rendered[1], "A"), foreign(5)];
    assert_eq!(
        generate_routed(&fixture.package, &routed, &contexts),
        Err(expected(5))
    );
    // Checked before duplicates and missing context, lowest index first.
    let routed = [
        route(3, &fixture.rendered[1], "A"),
        route(3, &fixture.rendered[2], "A"),
        foreign(9),
        foreign(6),
    ];
    assert_eq!(
        generate_routed(
            &fixture.package,
            &routed,
            &GenerationContexts { kani: None }
        ),
        Err(expected(6))
    );
}

/// Duplicate request indexes and a missing kind context each refuse the whole call.
///
/// Trace: FR-022-AC-5, TC-033
#[test]
fn tc_033_duplicate_index_and_missing_context_refuse_the_call() {
    let fixture = fixture();
    let pins = pins();
    let contexts = kani_context(&fixture, &pins, "crate::subject", 1);
    let duplicated = [
        route(8, &fixture.rendered[0], "A"),
        route(3, &fixture.rendered[1], "A"),
        route(3, &fixture.rendered[2], "B"),
        route(1, &fixture.rendered[0], "A"),
        route(1, &fixture.rendered[1], "A"),
    ];
    assert_eq!(
        generate_routed(&fixture.package, &duplicated, &contexts),
        Err(RoutedGenerationError::DuplicateRequestIndex { request_index: 1 })
    );
    let valid = [route(0, &fixture.rendered[0], "A")];
    assert_eq!(
        generate_routed(&fixture.package, &valid, &GenerationContexts { kani: None }),
        Err(RoutedGenerationError::MissingKindContext {
            kind: BackendKind::Kani
        })
    );
    // Duplicates are checked before the missing context.
    assert_eq!(
        generate_routed(
            &fixture.package,
            &duplicated,
            &GenerationContexts { kani: None }
        ),
        Err(RoutedGenerationError::DuplicateRequestIndex { request_index: 1 })
    );
}

/// A Kani group-level refusal is returned unchanged as `Kani`.
///
/// Trace: FR-022-AC-6, TC-033
#[test]
fn tc_033_a_kani_group_refusal_is_returned_unchanged() {
    let fixture = fixture();
    let routed = step_one_routing(&fixture);
    let mut drifted = pins();
    drifted.kani_version = "0.0.0".to_owned();
    let expected_pin = pins().kani_version;
    assert_eq!(
        generate_routed(
            &fixture.package,
            &routed,
            &kani_context(&fixture, &drifted, "crate::subject", 1)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::UnpinnedBackend {
                field: KaniPinField::KaniVersion,
                expected: expected_pin,
                supplied: "0.0.0".to_owned(),
            }
        ))
    );
    let pinned = pins();
    assert_eq!(
        generate_routed(
            &fixture.package,
            &routed,
            &kani_context(&fixture, &pinned, "crate::subject", 0)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::InvalidUnwind { unwind: 0 }
        ))
    );
    assert_eq!(
        generate_routed(
            &fixture.package,
            &routed,
            &kani_context(&fixture, &pinned, "not a path", 1)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::InvalidSubjectPath
        ))
    );
}

/// An `invalid_request` item rejects the Kani group: every record, no harness, and the duplicate
/// names the driver's index of the first occurrence.
///
/// Trace: FR-022-AC-7, TC-033
#[test]
fn tc_033_an_invalid_kani_item_rejects_the_group_with_driver_indexes() {
    let fixture = fixture();
    let routed = [
        route(9, &fixture.rendered[0], "A"),
        route(6, &fixture.rendered[0], "A"),
        route(20, &fixture.rendered[1], "A"),
    ];
    let generation = generate_step_one(&fixture, &routed);
    assert_eq!(generation.rejected, [BackendKind::Kani]);
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
///
/// Trace: FR-022-AC-8, TC-033
#[test]
fn tc_033_an_empty_routed_set_returns_an_empty_result() {
    let fixture = fixture();
    let pinned = pins();
    for contexts in [
        GenerationContexts { kani: None },
        kani_context(&fixture, &pinned, "crate::subject", 1),
    ] {
        assert_eq!(
            generate_routed(&fixture.package, &[], &contexts),
            Ok(RoutedGeneration {
                items: Vec::new(),
                rejected: Vec::new()
            })
        );
    }
}

/// Regeneration and slice order do not change the result, and neither the request index nor the
/// manifest digest reaches the harness.
///
/// Trace: FR-022-AC-9, TC-033
#[test]
fn tc_033_output_is_deterministic_and_independent_of_index_and_digest() {
    let fixture = fixture();
    let routed = step_one_routing(&fixture);
    let first = generate_step_one(&fixture, &routed);
    assert_eq!(first, generate_step_one(&fixture, &routed));
    let mut reversed = routed.clone();
    reversed.reverse();
    assert_eq!(first, generate_step_one(&fixture, &reversed));

    let low = generate_step_one(&fixture, &[route(0, &fixture.rendered[0], "A")]);
    let high = generate_step_one(&fixture, &[route(40, &fixture.rendered[0], "B")]);
    let (low_record, low_harness) = kani_parts(&low.items[0].output);
    let (high_record, high_harness) = kani_parts(&high.items[0].output);
    let (low_harness, high_harness) = (
        low_harness.expect("the node renders"),
        high_harness.expect("the node renders"),
    );
    assert_eq!(low_harness.rust, high_harness.rust);
    assert_eq!(low_harness.record, high_harness.record);
    assert_eq!(low_harness.identity_sha256, high_harness.identity_sha256);
    assert_eq!(
        (low_record.request_index, high_record.request_index),
        (0, 40)
    );
    assert_eq!(low.items[0].backend, kani_backend("A"));
    assert_eq!(high.items[0].backend, kani_backend("B"));
}

/// `x + 1` over `Int[0, 9]`, QSL's shape (a parameter typed by an `integer_range`
/// `bounded_domain` with binding-shaped `min`/`max` members), routes to Kani and is Supported
/// with a scalar harness carrying the inclusive `[0, 9]` domain (IR-297 with IR-296).
///
/// Trace: FR-014-AC-16, FR-022-AC-2, TC-024, TC-033
#[test]
fn tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness() {
    let package = package::bounded_increment_package().admit();
    let node_id = package::code_id(package::BOUNDED_INCREMENT);
    let oracles = quire_contract_codegen::generate_exact_scalar_oracles(
        &package,
        &[quire_contract_codegen::ExactScalarItem {
            node_id: node_id.clone(),
            operation: package::integer_add_0_9(),
        }],
    )
    .expect("generation succeeds");
    let fixture = Fixture {
        package,
        claim_map: oracles.claim_map,
        rendered: vec![node_id.clone()],
        unrendered: node_id.clone(),
    };
    let pins = pins();
    let generation = generate_routed(
        &fixture.package,
        &[route(0, &node_id, "A")],
        &kani_context(&fixture, &pins, "crate::subject", 1),
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
        [(0, 9), (0, 9)]
    );
}
