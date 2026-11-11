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
        ("Refused(refusal)", "KaniReaderRefused(refusal)"),
        ("Incomplete(incomplete)", "KaniReaderIncomplete(incomplete)"),
        (
            "CatalogUnavailable(error)",
            "KaniReaderCatalogUnavailable(error)",
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
                | CheckedFunctionErrorCode::InvalidIdentity
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
