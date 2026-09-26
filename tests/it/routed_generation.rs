//! FR-022 routed generation: `generate_routed` runs the routed kind's generation arm keyed by the
//! driver's request index, and takes the routed backend and kind as given.
//!
//! The fixtures are TC-024/TC-025's: `kani_obligations::scalar_package` and its claim map.

use std::fs;

use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, generate_routed,
    negotiate_kani_obligations, BackendKind, Candidate, ClaimDerivationRefusal, ClaimDisposition,
    ExactScalarRefusal, GenerationContexts, InvalidObligationItem, KaniGenerationContext,
    KaniObligationError, KaniObligationOutcome, KaniObligationRequest, KaniPinField,
    KaniScalarObligationHarness, KaniToolPins, KindOutput, ObligationDisposition, ObligationItem,
    ObligationRecord, RoutedGeneration, RoutedGenerationError, RoutedGenerationItem,
    UnsupportedObligation,
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
    pins: &'a KaniToolPins,
    subject_path: &'a str,
    unwind: u32,
) -> GenerationContexts<'a> {
    GenerationContexts {
        kani: Some(KaniGenerationContext {
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
        &kani_context(&pins, "crate::subject", 1),
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
    let contexts = kani_context(&pins, "crate::subject", 1);
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
    let contexts = kani_context(&pins, "crate::subject", 1);
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
            &kani_context(&drifted, "crate::subject", 1)
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
            &kani_context(&pinned, "crate::subject", 0)
        ),
        Err(RoutedGenerationError::Kani(
            KaniObligationError::InvalidUnwind { unwind: 0 }
        ))
    );
    assert_eq!(
        generate_routed(
            &fixture.package,
            &routed,
            &kani_context(&pinned, "not a path", 1)
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
        kani_context(&pinned, "crate::subject", 1),
    ] {
        assert_eq!(
            generate_routed(&fixture.package, &[], &contexts),
            Ok(RoutedGeneration {
                items: Vec::new(),
                rejected: Vec::new(),
                claim_map: None,
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
/// with a scalar harness carrying the inclusive `[0, 9]` domain (IR-297 with IR-296). No claim map
/// is supplied: the operation and domain are derived from the package alone (IR-294).
///
/// Trace: FR-014-AC-16, FR-014-AC-17, FR-022-AC-2, FR-022-AC-10, TC-024, TC-033
#[test]
fn tc_033_bounded_increment_over_a_bounded_parameter_is_supported_with_a_scalar_harness() {
    let package = package::bounded_increment_package().admit();
    let node_id = package::code_id(package::BOUNDED_INCREMENT);
    let pins = pins();
    let generation = generate_routed(
        &package,
        &[route(0, &node_id, "A")],
        &kani_context(&pins, "crate::subject", 1),
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

/// The corpus package with the nodes derivation refuses, and the claims of what it derives.
struct Derived {
    package: CheckedPackageV2,
    /// The three rendered nodes and the unrendered one of [`Fixture`].
    generated: Vec<CheckedNodeId>,
    rem: CheckedNodeId,
    integer_eq: CheckedNodeId,
}

fn derived() -> Derived {
    let fixture = fixture();
    let package = package::derivation_package().admit();
    let mut generated = fixture.rendered.clone();
    generated.push(fixture.unrendered.clone());
    Derived {
        package,
        generated,
        rem: package::code_id(package::DERIVE_REM),
        integer_eq: package::code_id(package::DERIVE_INTEGER_EQ),
    }
}

/// An underivable item keeps its typed refusal, `no_derivable_claim` naming the node and the
/// derivation refusal, with no harness; the group is not rejected and its siblings' records and
/// harnesses equal what they are without it.
///
/// Trace: FR-022-AC-11, FR-015-AC-15, TC-033
#[test]
fn tc_033_an_underivable_item_keeps_its_refusal_and_its_siblings_are_unaffected() {
    let derived = derived();
    let pins = pins();
    let contexts = kani_context(&pins, "crate::subject", 1);
    let with_siblings = derived
        .generated
        .iter()
        .enumerate()
        .map(|(index, node)| route(index, node, "A"))
        .collect::<Vec<_>>();
    let alone = generate_routed(&derived.package, &with_siblings, &contexts).expect("generates");

    for (refused, identity) in [
        (&derived.rem, "quire.op.integer.rem"),
        (&derived.integer_eq, "quire.op.integer.eq"),
    ] {
        let mut routed = with_siblings.clone();
        routed.push(route(9, refused, "A"));
        let mixed = generate_routed(&derived.package, &routed, &contexts).expect("generates");
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
}

/// `RoutedGeneration.claim_map` equals FR-014 over the derived items, plus one `NoDerivableClaim`
/// claim per underivable node, ascending by node id; it is `None` with nothing routed.
///
/// Trace: FR-022-AC-12, TC-033
#[test]
fn tc_033_the_returned_claim_map_is_fr014_over_the_derived_items() {
    let derived = derived();
    let pins = pins();
    let contexts = kani_context(&pins, "crate::subject", 1);
    let nodes = derived
        .generated
        .iter()
        .chain([&derived.rem, &derived.integer_eq])
        .cloned()
        .collect::<Vec<_>>();
    let routed = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| route(index, node, "A"))
        .collect::<Vec<_>>();
    let generation = generate_routed(&derived.package, &routed, &contexts).expect("generates");
    let claim_map = generation.claim_map.expect("a Kani group ran");

    let mut sorted = nodes.clone();
    sorted.sort();
    let items = derive_exact_scalar_items(&derived.package, &sorted)
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    assert_eq!(
        items.len(),
        4,
        "the four generated nodes derive; rem and eq do not"
    );
    let mut expected = generate_exact_scalar_oracles(&derived.package, &items)
        .expect("generates")
        .claim_map;
    assert!(expected
        .items
        .iter()
        .all(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));
    assert_eq!(claim_map.items.len(), 6);
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
    let mut underivable = vec![derived.rem.clone(), derived.integer_eq.clone()];
    underivable.sort();
    assert_eq!(refused, underivable);
    // Dropping the refused claims leaves exactly FR-014's map.
    let mut generated_only = claim_map.clone();
    generated_only
        .items
        .retain(|claim| matches!(claim.result, ClaimDisposition::Generated(_)));
    expected.items.sort_by(|a, b| a.node_id.cmp(&b.node_id));
    assert_eq!(generated_only, expected);
    let ids = claim_map
        .items
        .iter()
        .map(|claim| claim.node_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(ids, sorted, "claims ascend by node id");

    let empty = generate_routed(&derived.package, &[], &contexts).expect("generates");
    assert_eq!(empty.claim_map, None);
}
