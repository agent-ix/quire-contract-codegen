use super::*;
use crate::oracle::function::owner_path::{CheckedFunctionErrorCode, CheckedFunctionResources};
use ix_trace_rs::trace;
use qsl_replay::{
    acquire_checked_closure, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_model::{CheckedPackageLimit, CheckedPackageReadLimits};

const SAME: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function same using v(left: Int[0, 0], right: Int[0, 0]): Boolean pure { left = right }\n",
);

const COMPOSITE: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "tuple Pair(Boolean, Integer);\n",
    "union Choice { Empty, Some(Pair) }\n",
    "record Holder { pair: Pair; shape: Choice; flag: Boolean?; }\n",
    "function eq using v(x: Holder, y: Holder): Boolean pure { x = y }\n",
);

const CALLS: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function add using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { x + y }\n",
    "function nested using v(x: Int[0, 9], y: Int[0, 9]): Integer pure { add(x, y) }\n",
);

const PATH: &str = "function-owner.native";

/// One genuine QSL emission with the evidence the reader needs to admit it.
struct Fixture {
    source: &'static str,
    bytes: Vec<u8>,
    evidence: CheckedPackageEvidence,
    package: CheckedPackageV2,
    dependencies: DependencyInput,
}

fn identity() -> SourceIdentity {
    SourceIdentity::new("agent-ix", "function-owner")
}

fn evidence() -> CheckedPackageEvidence {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
}

impl Fixture {
    fn new(source: &'static str) -> Self {
        let compiled = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-owner".to_owned(),
                path: PATH,
                bytes: source.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine checked source");
        let bytes = compiled.package().bytes().to_vec();
        let evidence = evidence();
        let CheckedPackageV2ReadResult::Admitted(package) =
            CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence)
        else {
            panic!("the QSL emission must be admitted by Contract IR");
        };
        Self {
            source,
            bytes,
            evidence,
            package: *package,
            dependencies: DependencyInput::default(),
        }
    }

    fn function(&self, name: &str) -> CheckedNodeId {
        self.package
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "function"
                    && node
                        .declaration
                        .as_ref()
                        .is_some_and(|decl| decl.qualified_name == [Box::<str>::from(name)])
            })
            .expect("named source function")
            .node_id
            .clone()
    }

    fn request<'a>(
        &'a self,
        function: CheckedNodeId,
        package_bytes: &'a [u8],
        source_bytes: &'a [u8],
    ) -> KaniFunctionRequest<'a> {
        KaniFunctionRequest {
            package_bytes,
            read_limits: CheckedPackageReadLimits::bounded(),
            evidence: &self.evidence,
            source: CheckedFunctionSource {
                source: identity(),
                path: PATH,
                bytes: source_bytes,
                packages: &[],
                dependencies: &self.dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            },
            function,
        }
    }

    fn generate(&self, name: &str) -> Result<KaniFunctionHarness, OracleGenerationError> {
        generate_kani_function_harness(&self.request(
            self.function(name),
            &self.bytes,
            self.source.as_bytes(),
        ))
    }
}

fn library(harness: &KaniFunctionHarness) -> &str {
    &harness
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .expect("the harness crate has a lib")
        .contents
}

fn has_cfg_kani(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && matches!(&attribute.meta, syn::Meta::List(list) if list.tokens.to_string() == "kani")
    })
}

/// The whitespace-free token text of every `cfg(kani)` item of an emitted crate, including
/// `cfg(kani)` functions of otherwise ungated `impl` blocks.
fn kani_items(source: &str) -> String {
    use quote::ToTokens;
    let file = syn::parse_file(source).expect("emitted crate parses");
    let mut text = String::new();
    for item in &file.items {
        if let syn::Item::Impl(implementation) = item {
            for member in &implementation.items {
                if let syn::ImplItem::Fn(function) = member {
                    if has_cfg_kani(&function.attrs) {
                        text.push_str(&function.to_token_stream().to_string());
                    }
                }
            }
        }
        let attributes = match item {
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            _ => continue,
        };
        if has_cfg_kani(attributes) {
            text.push_str(&item.to_token_stream().to_string());
        }
    }
    text.retain(|character| !character.is_whitespace());
    text
}

/// What the `cfg(kani)` items of a harness must not hold (FR-021-AC-44).
fn kani_item_violations(kani: &str) -> Vec<&'static str> {
    let mut violations = Vec::new();
    for forbidden in ["CheckedPackageV2", "read(", "admits(", "EnvironmentFailure"] {
        if kani.contains(forbidden) {
            violations.push(forbidden);
        }
    }
    if kani.matches("TypeEnvironment::").count()
        != kani
            .matches("TypeEnvironment::bounded_with_cancel(")
            .count()
    {
        violations.push("a TypeEnvironment built other than by bounded_with_cancel");
    }
    for required in [
        "bounded_with_cancel(",
        "seal_union_verified(",
        "from_descriptors(",
    ] {
        if !kani.contains(required) {
            violations.push(required);
        }
    }
    violations
}

/// FR-021-AC-44: the `cfg(kani)` items rebuild through the genuine constructors, hold no
/// reader call, pre-built environment or membership verdict, and the predicate can fail.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn kani_items_rebuild_from_descriptors_through_genuine_constructors() {
    let fixture = Fixture::new(SAME);
    let harness = fixture
        .generate("same")
        .expect("the scalar equality case generates");
    assert_eq!(harness.symbol, "same");
    let source = library(&harness);
    syn::parse_file(source).expect("emitted crate parses");
    let kani = kani_items(source);
    assert_eq!(kani_item_violations(&kani), Vec::<&str>::new());
    assert!(kani.contains("fnkani_factory_same()"));
    // The signature's Int[0, 0] parameters are rebuilt, not summarized.
    assert_eq!(kani.matches("exact::ValueType::Int(").count(), 2);
    assert!(kani.contains("exact::ValueType::Boolean"));
    assert!(kani.contains("TypeEnvironmentLimits{ancestor_steps:"));

    // The inspection can fail: each forbidden shape is flagged once injected.
    let reader = kani.replace(
        "fnkani_factory_same()",
        "fnkani_factory_same(){CheckedPackageV2::read(b,l,e);}fnunused()",
    );
    assert!(kani_item_violations(&reader).contains(&"CheckedPackageV2"));
    let prebuilt = kani.replace(
        "TypeEnvironment::bounded_with_cancel(",
        "TypeEnvironment::new(",
    );
    assert!(!kani_item_violations(&prebuilt).is_empty());
    let verdict = kani.replace(
        "fnkani_factory_same()",
        "fnkani_factory_same(){e.admits(t,v);}fnx()",
    );
    assert!(kani_item_violations(&verdict).contains(&"admits("));
    let no_seal = kani.replace("seal_union_verified(", "seal_union(");
    assert!(kani_item_violations(&no_seal).contains(&"seal_union_verified("));
}

/// FR-021-AC-44: record, tuple and union declarations, presence and union bindings each
/// reach the descriptors from the owner's own declaration views.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn descriptors_cover_record_tuple_and_union_declarations() {
    let fixture = Fixture::new(COMPOSITE);
    let harness = fixture
        .generate("eq")
        .expect("the composite equality case generates");
    let source = library(&harness);
    let kani = kani_items(source);
    assert_eq!(kani_item_violations(&kani), Vec::<&str>::new());
    assert_eq!(
        kani.matches("DeclarationDescriptor{key:rebuilt(").count(),
        3
    );
    for shape in [
        "ShapeDescriptor::Record(vec![",
        "ShapeDescriptor::Tuple(vec![",
        "ShapeDescriptor::Union(vec![",
    ] {
        assert_eq!(kani.matches(shape).count(), 1, "{shape}");
    }
    assert!(kani.contains("exact::Presence::Optional"));
    assert!(kani.contains("exact::Presence::Required"));
    assert!(kani.contains("exact::ValueType::Composite(rebuilt("));
    assert!(kani.contains("\"Empty\""));
    assert!(kani.contains("\"Some\""));
    // A union carries the final key `seal_union_verified` needs beside its handle.
    assert!(kani.contains("node:rebuilt(exact::NodeKey::new("));
}

#[test]
fn function_origin_matches_the_existing_generator_location() {
    let fixture = Fixture::new(CALLS);
    let add = fixture.generate("add").expect("add generates");
    let kani = kani_items(library(&add));
    // `add` is the first source function.
    assert!(kani.contains("index:0}"));
    assert_eq!(
        source_body_indices(&fixture.package)
            .get(&fixture.function("nested"))
            .copied(),
        Some(1)
    );
}

fn truncated(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.len() / 2]
}

/// FR-021-AC-46: the reader's `Refused` is its own typed cause and emits nothing.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn a_reader_refusal_is_its_own_typed_generation_refusal() {
    let fixture = Fixture::new(SAME);
    assert!(fixture.generate("same").is_ok(), "untouched bytes generate");
    let request = fixture.request(
        fixture.function("same"),
        truncated(&fixture.bytes),
        fixture.source.as_bytes(),
    );
    assert!(matches!(
        generate_kani_function_harness(&request),
        Err(OracleGenerationError::KaniReaderRefused(_))
    ));
}

/// FR-021-AC-46: the reader's `Incomplete` is its own typed cause, not a refusal.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn a_reader_incomplete_is_its_own_typed_generation_refusal() {
    let fixture = Fixture::new(SAME);
    let mut request = fixture.request(
        fixture.function("same"),
        &fixture.bytes,
        fixture.source.as_bytes(),
    );
    request.read_limits = CheckedPackageReadLimits {
        bytes: 1,
        ..CheckedPackageReadLimits::bounded()
    };
    match generate_kani_function_harness(&request) {
        Err(OracleGenerationError::KaniReaderIncomplete(incomplete)) => {
            assert_eq!(incomplete.limit_kind, CheckedPackageLimit::Bytes);
        }
        other => panic!("expected the reader's Incomplete, got {other:?}"),
    }
}

/// FR-021-AC-46: the third reader result has its own variant. `CatalogReadError` has private
/// fields and no constructor, and the production catalog reads, so no test can obtain the
/// value; the exhaustive match in `admitted_package` fails to compile if a reader result is
/// added or merged, and this inspection fails if its arm stops naming its own variant.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn each_reader_result_maps_to_its_own_variant() {
    let source = include_str!("../kani_harness.rs");
    let function = source
        .split("fn admitted_package(")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("admitted_package is present");
    for (result, refusal) in [
        ("Refused(refusal)", "KaniReaderRefused(Box::new(refusal))"),
        (
            "Incomplete(incomplete)",
            "KaniReaderIncomplete(Box::new(incomplete))",
        ),
        (
            "CatalogUnavailable(error)",
            "KaniReaderCatalogUnavailable(Box::new(error))",
        ),
    ] {
        let arm = function
            .split(&format!("CheckedPackageV2ReadResult::{result} =>"))
            .nth(1)
            .unwrap_or_else(|| panic!("{result} has an arm"));
        let arm = arm
            .split("CheckedPackageV2ReadResult::")
            .next()
            .unwrap_or(arm);
        assert!(arm.contains(&format!("OracleGenerationError::{refusal}")));
    }
}

/// FR-021-AC-46: acquisition, selection, declaration reading and prepared-function
/// construction failures are one cause carrying their own stable code.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn owner_path_failures_carry_their_checked_function_error_code() {
    let fixture = Fixture::new(SAME);
    assert!(
        fixture.generate("same").is_ok(),
        "untouched inputs generate"
    );
    // Acquisition: the supplied source is not the retained package's source.
    let request = fixture.request(fixture.function("same"), &fixture.bytes, b"function (");
    assert!(matches!(
        generate_kani_function_harness(&request),
        Err(OracleGenerationError::KaniOwnerPathRefused {
            code: CheckedFunctionErrorCode::Acquisition
        })
    ));
    // Selection: an admitted node that is not a callable.
    let not_callable = fixture
        .package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() != "function")
        .expect("a non-function node")
        .node_id
        .clone();
    let request = fixture.request(not_callable, &fixture.bytes, fixture.source.as_bytes());
    assert!(matches!(
        generate_kani_function_harness(&request),
        Err(OracleGenerationError::KaniOwnerPathRefused {
            code: CheckedFunctionErrorCode::FacadeLookup
        })
    ));
}

/// FR-021-AC-46: retained bytes the reader admits but that differ from the acquisition's
/// package fail selection with the owner-identity cause, not a reader cause.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn admitted_bytes_that_differ_from_the_acquisition_refuse_on_the_owner_path() {
    let fixture = Fixture::new(SAME);
    let other = Fixture::new(CALLS);
    // The second source's bytes under the first source's function id and acquisition.
    let mut request = fixture.request(
        fixture.function("same"),
        &other.bytes,
        fixture.source.as_bytes(),
    );
    request.evidence = &other.evidence;
    assert!(
        matches!(
            generate_kani_function_harness(&request),
            Err(OracleGenerationError::KaniOwnerPathRefused {
                code: CheckedFunctionErrorCode::OwnerIdentityMismatch
            })
        ),
        "{:?}",
        generate_kani_function_harness(&request).map(|_| ())
    );
}

/// FR-021-AC-46: a requested nested-call body is refused with its own cause naming the
/// function and emits nothing, while a direct body in the same source still generates.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn a_nested_call_body_is_refused_naming_the_function() {
    let fixture = Fixture::new(CALLS);
    assert!(fixture.generate("add").is_ok(), "a direct body generates");
    let nested = fixture.function("nested");
    match fixture.generate("nested") {
        Err(OracleGenerationError::KaniNestedCallBody { function }) => {
            assert_eq!(function, nested);
        }
        other => panic!("expected the nested-call refusal, got {other:?}"),
    }
}

/// FR-021-AC-46: the entry takes the retained bytes and the function node id only; it accepts
/// no admitted package, declaring package or body classification. The exhaustive destructure
/// stops compiling if a field of that kind is added.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn the_entry_signature_takes_no_admitted_package_or_body_classification() {
    let _: fn(&KaniFunctionRequest<'_>) -> Result<KaniFunctionHarness, OracleGenerationError> =
        generate_kani_function_harness;
    let fixture = Fixture::new(SAME);
    let KaniFunctionRequest {
        package_bytes: _,
        read_limits: _,
        evidence: _,
        source: _,
        function: _,
    } = fixture.request(fixture.function("same"), &fixture.bytes, SAME.as_bytes());
}

const UNSUPPORTED: &str = concat!(
    "language \"ix:native\" edition \"1-draft\";\n",
    "profile v = \"quire.value.complete/v1\";\n",
    "function less using v(x: Int[0, 9], y: Int[0, 9]): Boolean pure { x < y }\n",
    "function same using v(x: Int[0, 9], y: Int[0, 9]): Boolean pure { x = y }\n",
);

/// FR-021-AC-46: a body that is neither a supported scalar nor a supported equality is the
/// sixth typed cause, not the renderer-internal `InvalidPlanState`; a supported sibling generates.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn an_unsupported_body_is_its_own_typed_refusal() {
    let fixture = Fixture::new(UNSUPPORTED);
    assert!(
        fixture.generate("same").is_ok(),
        "a supported body generates"
    );
    assert!(matches!(
        fixture.generate("less"),
        Err(OracleGenerationError::KaniUnsupportedBody {
            reason: KaniUnsupportedReason::BodyShape
        })
    ));
}

/// FR-021-AC-46: the multi-segment name and missing source-index refusals are their own typed
/// reasons; the single-segment and present-index inputs pass.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn name_and_index_failures_are_typed_unsupported_reasons() {
    let one: Vec<Box<str>> = vec!["f".into()];
    let two: Vec<Box<str>> = vec!["m".into(), "f".into()];
    assert_eq!(single_segment_name(&one).as_deref(), Some("f"));
    assert_eq!(single_segment_name(&two), None);
    assert_eq!(single_segment_name(&[]), None);
    let fixture = Fixture::new(SAME);
    let function = fixture.function("same");
    let mut indices = std::collections::BTreeMap::new();
    assert!(matches!(
        body_index(&indices, &function),
        Err(OracleGenerationError::KaniUnsupportedBody {
            reason: KaniUnsupportedReason::SourceIndex
        })
    ));
    indices.insert(&function, 3);
    assert!(matches!(body_index(&indices, &function), Ok(3)));
}

/// FR-021-AC-46: the nested-call cause is decided before the name and index lookups, so it is
/// never masked by a generic refusal; its position in the entry is checked structurally.
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn the_nested_call_refusal_precedes_the_name_and_index_lookups() {
    let source = include_str!("../kani_harness.rs");
    let entry = source
        .split("pub fn generate_kani_function_harness(")
        .nth(1)
        .expect("the entry is present");
    let nested = entry.find("KaniNestedCallBody").expect("nested refusal");
    for later in [
        "single_segment_name(",
        "body_index(",
        "serde_json::to_string(",
    ] {
        let position = entry
            .find(later)
            .unwrap_or_else(|| panic!("{later} present"));
        assert!(
            nested < position,
            "the nested-call refusal precedes {later}"
        );
    }
}

/// Every owner-path call in the entry maps its error to the one `KaniOwnerPathRefused` cause.
/// `NominalDeclaration` has no public constructor, so a failing declaration view or a
/// `checked_composite_declarations` failure cannot be built as a value; this inspection fails if
/// either call stops being mapped (FR-021-AC-46).
#[trace("TC-031", "FR-021-AC-46")]
#[test]
fn every_owner_path_call_maps_to_the_owner_path_cause() {
    let source = include_str!("../kani_harness.rs");
    let entry = source
        .split("pub fn generate_kani_function_harness(")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("the entry is present")
        .to_owned();
    let unmapped = |entry: &str| {
        [
            "acquire_checked_source(",
            "select_checked_callable(",
            "checked_function_declarations(",
            "checked_composite_declarations(",
        ]
        .into_iter()
        .filter(|call| {
            let after = entry.split(call).nth(1).unwrap_or("");
            !after
                .split(';')
                .next()
                .unwrap_or("")
                .contains(".map_err(owner_path)")
        })
        .collect::<Vec<_>>()
    };
    assert_eq!(unmapped(&entry), Vec::<&str>::new());
    let mutated = entry.replace(
        "checked_composite_declarations(&owner).map_err(owner_path)?",
        "checked_composite_declarations(&owner)?",
    );
    assert_eq!(unmapped(&mutated), vec!["checked_composite_declarations("]);
}

/// FR-021-AC-44 mutation row: the driven body must call `ready` before and `finish` after the
/// call, and `finish` must hold the `admits` guard. The predicate flags each removal.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn the_driven_body_keeps_ready_finish_and_the_admits_guard() {
    let fixture = Fixture::new(SAME);
    let harness = fixture.generate("same").expect("generates");
    let lib: String = library(&harness).split_whitespace().collect();
    let violations = |lib: &str| {
        let mut missing = Vec::new();
        for required in ["self.0.ready()?", "self.0.finish(", "environment.admits("] {
            if !lib.contains(required) {
                missing.push(required);
            }
        }
        missing
    };
    assert_eq!(violations(&lib), Vec::<&str>::new());
    assert_eq!(
        violations(&lib.replace("self.0.ready()?", "self.0.skip()?")),
        vec!["self.0.ready()?"]
    );
    assert_eq!(
        violations(&lib.replace("self.0.finish(", "self.0.keep(")),
        vec!["self.0.finish("]
    );
    assert_eq!(
        violations(&lib.replace("environment.admits(", "environment.kept(")),
        vec!["environment.admits("]
    );
}

/// FR-021-AC-44: the caller's environment limits and an already-cancelled handle are emitted
/// literally; an observing link is not (the emitted handle is then a fresh `Cancel::new()`).
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn limits_and_cancel_sources_are_emitted_literally() {
    let fixture = Fixture::new(SAME);
    let mut request = fixture.request(
        fixture.function("same"),
        &fixture.bytes,
        fixture.source.as_bytes(),
    );
    request
        .source
        .body_resources
        .environment_limits
        .ancestor_steps = 7;
    request.source.body_resources.environment_limits.work_units = 11;
    let uncancelled = generate_kani_function_harness(&request).expect("generates");
    let kani = kani_items(library(&uncancelled));
    assert!(kani.contains("TypeEnvironmentLimits{ancestor_steps:7,work_units:11}"));
    assert!(kani.contains("exact::Cancel::new()"));
    assert!(!kani.contains("exact::CancelCause"));
    request
        .source
        .body_resources
        .cancel
        .cancel(quire_exact::CancelCause::Deadline);
    let cancelled = generate_kani_function_harness(&request).expect("generates");
    assert!(kani_items(library(&cancelled)).contains("cancel.cancel(exact::CancelCause::Deadline)"));
}
