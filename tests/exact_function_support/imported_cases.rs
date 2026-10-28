//! Executed imported-owner FR-021 parity through the FR-485 facade.

mod generated {
    pub use quire_exact_function_imported_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use qsl_replay::{
    acquire_checked_closure, ApplicationEnvironment, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits, SuppliedLibrary,
};
use quire_contract_codegen::{
    checked_function_wire_id, checked_package_digest, CheckedFunctionSource,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use quire_eval::FamilyOutcome;
use quire_exact::{Integer, LimitKind, Meter, Outcome, ScalarLimits, Value};
use quire_semantic_value::object_closure::ObjectClosure;
use std::sync::Arc;

fn library_identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "function-library", "git", "1")
}

fn importer_identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "function-importer", "git", "1")
}

fn meter() -> Meter {
    let ceiling = u64::from(u32::MAX);
    Meter::new(ScalarLimits {
        integer_bits: ceiling,
        decimal_digits: ceiling,
        scale_expansion: ceiling,
        text_input_bytes: ceiling,
        text_scalars: ceiling,
        normalized_scalars: ceiling,
        unit_edges: ceiling,
        value_occurrences: ceiling,
        work_units: ceiling,
        result_units: ceiling,
    })
}

fn admitted(bytes: &[u8], library: Option<&CheckedPackageV2>) -> CheckedPackageV2 {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    if let Some(library) = library {
        evidence
            .insert_dependency_package("test/function-library", Arc::new(library.clone()))
            .expect("one locked library owner");
    }
    let read = CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), &evidence);
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("IR admits exact QSL bytes: {read:?}");
    };
    *package
}

fn function_id(package: &CheckedPackageV2) -> CheckedNodeId {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration.as_ref().is_some_and(|declaration| {
                declaration.qualified_name == [Box::<str>::from("imported")]
            })
        })
        .expect("library declares imported")
        .node_id
        .clone()
}

/// Trace: FR-021-AC-2, FR-021-AC-32, FR-021-AC-35, FR-021-AC-36, TC-031.
#[test]
fn generated_library_body_matches_importer_facade_application() {
    let compiled_library = acquire_checked_closure(
        library_identity(),
        "function-library.native",
        generated::LIBRARY_SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the source library");
    let library = admitted(compiled_library.package().bytes(), None);
    let dependencies = DependencyInput::new([SuppliedLibrary {
        identity: "test/function-library".to_owned(),
        source: library_identity(),
        path: "function-library.native".to_owned(),
        bytes: generated::LIBRARY_SOURCE.as_bytes().to_vec(),
    }])
    .expect("one genuine QSL library dependency");
    let compiled_unit = acquire_checked_closure(
        importer_identity(),
        "function-importer.native",
        generated::IMPORTER_SOURCE.as_bytes(),
        [],
        &dependencies,
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL admits the source importer");
    let unit = admitted(compiled_unit.package().bytes(), Some(&library));
    let arguments = vec![
        Value::Integer(Integer::from(3_u64)),
        Value::Integer(Integer::from(4_u64)),
    ];
    let source = CheckedFunctionSource {
        source: importer_identity(),
        path: "function-importer.native",
        bytes: generated::IMPORTER_SOURCE.as_bytes(),
        packages: &[],
        dependencies: &dependencies,
        limits: StageLimits::default(),
        replay_limits: ReplayLimits::default(),
    };
    let mut generated_meter = meter();
    let actual = generated::oracle_imported(
        source,
        &unit,
        &library,
        &library,
        arguments.clone(),
        ObjectClosure::default(),
        &mut generated_meter,
    )
    .expect("QSL selects and accepts generated imported body")
    .expect("generated imported body completes");

    let fresh = acquire_checked_closure(
        importer_identity(),
        "function-importer.native",
        generated::IMPORTER_SOURCE.as_bytes(),
        [],
        &dependencies,
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("independent importer acquisition");
    let handle = fresh.handle();
    let owner = checked_package_digest(library.package_id()).expect("exact library owner id");
    let wire = checked_function_wire_id(&function_id(&library)).expect("exact function wire id");
    let selection = handle
        .lookup_callable(owner, wire)
        .expect("QSL selects imported callable");
    assert_eq!(selection.owner_package_id(), owner);
    let mut registry = handle.registry();
    let environment = ApplicationEnvironment::new(ObjectClosure::default());
    let mut direct_meter = meter();
    let expected = registry
        .begin_call(&selection, arguments, &environment, &mut direct_meter)
        .expect("QSL begins without generated registration")
        .run()
        .expect("QSL interpreter completes");
    assert_eq!(
        format!("{:?}", actual.outcome),
        format!("{:?}", expected.outcome)
    );
    assert!(matches!(
        actual.outcome,
        FamilyOutcome::Evaluated(Outcome::Completed(_))
    ));
    assert_eq!(actual.location, expected.location);
    assert_eq!(actual.losses, expected.losses);
    assert_eq!(
        generated_meter.admitted_charges(),
        direct_meter.admitted_charges()
    );
    for kind in LimitKind::ALL {
        assert_eq!(
            generated_meter.consumed(kind),
            direct_meter.consumed(kind),
            "{kind:?}"
        );
    }
}
