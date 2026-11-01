// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic QSL emission, public IR admission, then the constrained family.

use ix_trace_rs::trace;
use qsl_replay::{
    call_site, ClauseName, DependencyInput, Identifier, SnapshotValue, SourceIdentity,
};
use quire_contract_codegen::*;
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, StateClauseKind,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, num::NonZeroU64, time::Duration};

const MODEL: &str = "cg/constrained";
const OBJECT: &str = "ix://cg/constrained/Node";
const SMALL: &str = "ix://cg/constrained/Small";
fn origin() -> Value {
    json!({"generated":{"generatorIdentity":"ix://cg/constrained/spec","generatorVersion":"1.0.0","inputIdentities":["ix://cg/constrained/spec"]}})
}
fn document() -> Value {
    let slot = |owner: &str, name: &str, ty: &str| json!({"identity":format!("{owner}/{name}"),"name":name,"typeRef":ty,"presence":"required","nullable":false,"defaultKind":"none","origin":origin(),"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
    let op = |name: &str, parameter: &str, ty: &str, result: bool| {
        let owner = format!("{OBJECT}/{name}");
        let mut value = json!({"identity":owner,"name":name,"params":[slot(&owner,parameter,ty)],"pre":[],"post":[],"origin":origin(),"frame":{"modifies":[format!("{OBJECT}/n"),format!("{OBJECT}/flag"),format!("{OBJECT}/aux")],"creates":[],"deletes":[]}});
        if result {
            value["returns"] = json!({"typeRef":"ix://quire/native/Boolean","nullable":false,"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
        }
        value
    };
    json!({"contractVersion":"2.0.0","canonicalization":{"algorithm":"rfc8785-v1"},"source":{"identity":"ix://cg/constrained/spec","version":"1.0.0","dialect":"spec-bundle"},"package":{"identity":MODEL,"version":"1.0.0","mappingVersions":[],"profileVersions":[]},"constructs":[
        {"kind":{"module":MODEL,"name":"entity"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.object-type/v1"}},
        {"kind":{"module":MODEL,"name":"value_type"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.value-type/v1"}}
    ],"occurrences":[],"extensions":[],"types":[
        {"identity":SMALL,"displayName":"Small","kind":{"module":MODEL,"name":"value_type"},"roles":[],"origin":origin(),"extensions":[],"unknownPolicy":"reject","scalar":"integer","constraints":[
            {"identity":format!("{SMALL}/min"),"keyword":"min","operands":{"value":"0"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"small.minimum","origin":origin()},
            {"identity":format!("{SMALL}/max"),"keyword":"max","operands":{"value":"3"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"small.maximum","origin":origin()}]},
        {"identity":OBJECT,"displayName":"Node","kind":{"module":MODEL,"name":"entity"},"roles":[],"origin":origin(),"constraints":[],"extensions":[],"unknownPolicy":"reject","supertypes":[],"fields":[slot(OBJECT,"n",SMALL),slot(OBJECT,"flag","ix://quire/native/Boolean"),slot(OBJECT,"aux",SMALL)],"operations":[op("nudge","delta",SMALL,false),op("idle","enabled","ix://quire/native/Boolean",true)]}
    ],"populations":[]})
}

struct Fixture {
    source: String,
    document: Vec<u8>,
    name: String,
    package: CheckedPackageV2,
    context: CheckedNodeId,
    safety: CheckedNodeId,
    constraints: Vec<CheckedNodeId>,
}
fn fixture(doc: &Value, name: &str, safety: &str, constraints: &[&str]) -> Fixture {
    let bytes = quire_canonical::to_vec(doc, quire_canonical::Limits::new(1 << 20)).unwrap();
    let mut source=format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\ninvariant Safe using v on M::Node at current {{ {safety} }}\n");
    for (index, body) in constraints.iter().enumerate() {
        source.push_str(&format!(
            "constraint Limit{index} using v on M::Node at current {{ {body} }}\n"
        ));
    }
    let function = |name: &str, body: &str| {
        format!("function {name} using v(n: Int[0, 3], flag: Boolean, aux: Int[0, 3]): Boolean pure {{ {} }}\n",body.replace("self.n","n").replace("self.flag","flag").replace("self.aux","aux"))
    };
    source.push_str(&function("EvalSafety", safety));
    for (index, body) in constraints.iter().enumerate() {
        source.push_str(&function(&format!("EvalConstraint{index}"), body));
    }
    let site = call_site(
        SourceIdentity::new("agent-ix", format!("{name}.native"), "git", "fixture"),
        name.to_owned(),
        "constrained.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new("Safe").unwrap()),
    )
    .expect("authentic QSL clause emission");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(MODEL, bytes.clone())
        .unwrap();
    let package = match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("public IR admission: {other:?}"),
    };
    let context = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "object_type")
        .unwrap()
        .node_id
        .clone();
    let clauses = |kind| {
        package.graph().nodes.iter().filter(|node|matches!(package.term(&node.node_id),Some(CheckedTerm::Application{operation,..}) if operation.state_clause_kind().ok().flatten()==Some(kind))).map(|node|node.node_id.clone()).collect::<Vec<_>>()
    };
    let safety = clauses(StateClauseKind::Invariant)
        .into_iter()
        .next()
        .unwrap();
    let constraints = clauses(StateClauseKind::Constraint);
    Fixture {
        source,
        document: bytes,
        name: name.to_owned(),
        package,
        context,
        safety,
        constraints,
    }
}
fn initial(n: i64) -> BTreeMap<String, SnapshotValue> {
    BTreeMap::from([
        ("n".into(), SnapshotValue::integer(n.to_string())),
        ("flag".into(), SnapshotValue::boolean(false)),
        ("aux".into(), SnapshotValue::integer("0")),
    ])
}
fn bindings(f: &Fixture) -> Vec<ConstrainedOperationBinding<'_>> {
    vec![
        ConstrainedOperationBinding {
            declaring_node: &f.context,
            name: "idle",
            subject_path: "crate::subject::idle",
        },
        ConstrainedOperationBinding {
            declaring_node: &f.context,
            name: "nudge",
            subject_path: "crate::subject::nudge",
        },
    ]
}
fn request<'a>(
    f: &'a Fixture,
    initial: &'a BTreeMap<String, SnapshotValue>,
    bindings: &'a [ConstrainedOperationBinding<'a>],
) -> ConstrainedMethodRequest<'a> {
    ConstrainedMethodRequest {
        safety: ConstrainedClauseSelection {
            package: &f.package,
            clause: &f.safety,
        },
        constraint: Some(ConstrainedClauseSelection {
            package: &f.package,
            clause: &f.constraints[0],
        }),
        context: &f.context,
        initial,
        operations: bindings,
        maximum_steps: 3,
        ceilings: ProofCeilings {
            memory_bytes: NonZeroU64::new(2 * 1024 * 1024 * 1024).unwrap(),
            wall_clock: Duration::from_secs(120),
        },
        unwind: 8,
        evaluator_limits: quire_exact::ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        },
        census_work: 100_000,
    }
}

#[trace("TC-062", "FR-048-AC-1", "FR-048-AC-2")]
#[test]
fn authentic_method_selection_is_exact_and_deterministic() {
    let f = fixture(
        &document(),
        "constrained-selection",
        "self.n <= 3",
        &["self.n < 2", "self.n < 3"],
    );
    assert_eq!(f.constraints.len(), 2);
    let initial = initial(0);
    let bindings = bindings(&f);
    let mut request = request(&f, &initial, &bindings);
    let a = generate_constrained_method(&request).unwrap();
    let again = generate_constrained_method(&request).unwrap();
    assert_eq!(a.identity(), again.identity());
    assert_eq!(a.rust(), again.rust());
    request.constraint.as_mut().unwrap().clause = &f.constraints[1];
    let b = generate_constrained_method(&request).unwrap();
    assert_eq!(a.identity().obligation, b.identity().obligation);
    assert_ne!(a.identity(), b.identity());
    assert_ne!(a.rust().path, b.rust().path);
    request.constraint = None;
    let ordinary = generate_constrained_method(&request).unwrap();
    assert_eq!(ordinary.identity().obligation, a.identity().obligation);
    assert_ne!(ordinary.identity(), a.identity());
    assert_ne!(ordinary.identity(), b.identity());
    assert_eq!(ordinary.rust().contents.matches("kani::cover!").count(), 1);
    request.constraint = Some(ConstrainedClauseSelection {
        package: &f.package,
        clause: &f.constraints[1],
    });

    request.constraint.as_mut().unwrap().clause = &f.safety;
    assert_eq!(
        generate_constrained_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::NotConstraint
    );
    let mut missing = f.constraints[0].clone();
    missing.ordinal += 100;
    request.constraint.as_mut().unwrap().clause = &missing;
    assert_eq!(
        generate_constrained_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::MissingConstraint
    );
    request.constraint.as_mut().unwrap().clause = &f.constraints[0];
    request.context = &f.safety;
    assert_eq!(
        generate_constrained_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::WrongContext
    );
    let foreign = fixture(
        &document(),
        "constrained-foreign",
        "self.n <= 3",
        &["self.n < 2"],
    );
    request.context = &f.context;
    request.constraint = Some(ConstrainedClauseSelection {
        package: &foreign.package,
        clause: &foreign.constraints[0],
    });
    assert_eq!(
        generate_constrained_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::WrongPackage
    );
}

#[trace("TC-062", "FR-048-AC-1", "FR-048-AC-11")]
#[test]
fn complete_census_retains_unread_fields_and_uncalled_operations() {
    let f = fixture(
        &document(),
        "constrained-census",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let initial = initial(0);
    let bindings = bindings(&f);
    let request = request(&f, &initial, &bindings);
    let method = generate_constrained_method(&request).unwrap();
    assert_eq!(
        method
            .identity()
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<Vec<_>>(),
        ["n", "flag", "aux"]
    );
    assert_eq!(
        method
            .identity()
            .operations
            .iter()
            .map(|operation| operation.name.as_str())
            .collect::<Vec<_>>(),
        ["idle", "nudge"]
    );
    assert_eq!(method.identity().operations[0].parameters[0].0, "enabled");
    assert_eq!(
        method.identity().operations[0].parameters[0].1.bounds,
        SymbolicBounds::Boolean
    );
    assert_eq!(
        method.identity().operations[0]
            .result
            .as_ref()
            .map(|domain| &domain.bounds),
        Some(&SymbolicBounds::Boolean)
    );
    let mut incomplete = request;
    incomplete.operations = &bindings[..1];
    assert_eq!(
        generate_constrained_method(&incomplete).unwrap_err(),
        ConstrainedMethodRefusal::OperationBindings
    );
    let mut missing = initial.clone();
    missing.remove("aux");
    incomplete.initial = &missing;
    assert_eq!(
        generate_constrained_method(&incomplete).unwrap_err(),
        ConstrainedMethodRefusal::InitialState
    );
    let mut invalid = initial.clone();
    invalid.insert("n".into(), SnapshotValue::integer("4"));
    incomplete.initial = &invalid;
    assert_eq!(
        generate_constrained_method(&incomplete).unwrap_err(),
        ConstrainedMethodRefusal::InitialState
    );
}

#[trace("TC-062", "FR-048-AC-9")]
#[test]
fn ordinary_execution_cannot_publish_a_constrained_success() {
    let f = fixture(
        &document(),
        "constrained-route",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let initial = initial(0);
    let bindings = bindings(&f);
    let method = generate_constrained_method(&request(&f, &initial, &bindings)).unwrap();
    let installation = KaniInstallation::new("absent-no-spawn");
    let cancel = quire_exact::Cancel::new();
    let execution = KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::Constrained(&method),
        crate_directory: std::path::Path::new("absent"),
        target_directory: std::path::Path::new("absent-target"),
        home_directory: None,
    };
    assert!(matches!(
        execute_kani_obligation(&execution),
        Err(KaniExecutionRefusal::WrongMethodExecutionRoute)
    ));
    assert!(matches!(
        execute_kani_obligations(&[execution]),
        Err(KaniExecutionRefusal::WrongMethodExecutionRoute)
    ));
}

fn owner_evaluation(
    f: &Fixture,
    function: &str,
    n: i64,
    flag: bool,
    aux: i64,
    accounting: quire_exact::ScalarLimits,
) -> qsl_replay::CallOutcome {
    let call = qsl_replay::Call {
        function: function.to_owned(),
        arguments: vec![
            qsl_replay::CallArgument {
                parameter: "n".into(),
                value: quire_exact::Value::Integer(n.into()),
            },
            qsl_replay::CallArgument {
                parameter: "flag".into(),
                value: quire_exact::Value::Boolean(flag),
            },
            qsl_replay::CallArgument {
                parameter: "aux".into(),
                value: quire_exact::Value::Integer(aux.into()),
            },
        ],
        accounting,
        supplied: Default::default(),
    };
    qsl_replay::run_with_requirements(
        SourceIdentity::new("agent-ix", format!("{}.native", f.name), "git", "fixture"),
        f.name.clone(),
        "constrained.native",
        f.source.as_bytes(),
        &BTreeMap::from([(MODEL.to_owned(), f.document.clone())]),
        &DependencyInput::default(),
        qsl_replay::SpineLimits::default(),
        &call,
        &qsl_replay::ObjectEnvironment::default(),
        &quire_exact::Cancel::new(),
    )
    .expect("independent QSL owner evaluation")
    .into_parts()
    .0
}

#[trace("TC-062", "FR-048-AC-10")]
#[test]
fn qsl_owner_evaluates_the_complete_small_state_domain() {
    let f = fixture(
        &document(),
        "constrained-owner-domain",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let initial = initial(0);
    let bindings = bindings(&f);
    let provisions = request(&f, &initial, &bindings);
    for n in 0..=3 {
        for flag in [false, true] {
            for aux in 0..=3 {
                assert!(matches!(
                    owner_evaluation(&f, "EvalSafety", n, flag, aux, provisions.evaluator_limits),
                    qsl_replay::CallOutcome::Completed(qsl_replay::CallValue::Boolean(true))
                ));
                assert!(
                    matches!(owner_evaluation(&f,"EvalConstraint0",n,flag,aux,provisions.evaluator_limits),qsl_replay::CallOutcome::Completed(qsl_replay::CallValue::Boolean(value)) if value==(n<2))
                );
            }
        }
    }
}

const SUBJECT: &str = r#"
pub mod subject {
    pub fn idle(_n:&mut i64,_flag:&mut bool,_aux:&mut i64,enabled:bool)->bool { enabled }
    pub fn nudge(n:&mut i64,flag:&mut bool,aux:&mut i64,delta:i64) {
        *n=(*n+delta).min(3);
        *flag=!*flag;
        *aux=(*aux+1)%4;
    }
}
"#;

fn run_installed(method: &KaniConstrainedMethodHarness) -> ConstrainedBackendEvidence {
    let scratch = crate::scratch_crate::TemporaryDirectory::new("constrained-method-installed");
    std::fs::create_dir_all(scratch.path.as_path().join("src")).unwrap();
    std::fs::create_dir_all(scratch.path.as_path().join(".cargo")).unwrap();
    crate::scratch_crate::write_manifest(scratch.path.as_path(),"[package]\nname=\"constrained_method_control\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[workspace]\n[dependencies]\nquire-exact={git=\"https://github.com/agent-ix/quire-exact\",branch=\"main\"}\n");
    std::fs::write(
        scratch.path.as_path().join(".cargo/config.toml"),
        "[build]\njobs=2\n",
    )
    .unwrap();
    std::fs::write(
        scratch.path.as_path().join("src/lib.rs"),
        format!(
            "#![allow(dead_code)]\n{}\n{SUBJECT}",
            method.rust().contents
        ),
    )
    .unwrap();
    let installation = KaniInstallation::discover().expect("installed bounded Kani");
    let cancel = quire_exact::Cancel::new();
    let target =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/constrained-method-backend");
    let evidence = execute_constrained_method(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::Constrained(method),
        crate_directory: scratch.path.as_path(),
        target_directory: &target,
        home_directory: None,
    })
    .unwrap_or_else(|error| panic!("bounded constrained execution refused: {error}"));
    println!("BACKEND {}", serde_json::to_string(&evidence).unwrap());
    evidence
}

#[trace(
    "TC-062",
    "FR-048-AC-3",
    "FR-048-AC-4",
    "FR-048-AC-5",
    "FR-048-AC-8",
    "FR-048-AC-9",
    "FR-048-AC-10"
)]
#[test]
#[ignore = "installed bounded Kani lane; explicit 2 GiB per control, never the parked 16 GiB proof"]
fn installed_small_method_controls_retain_boundary_safety_and_no_boundary_success() {
    for (name, safety, constraint, initial_n, observation_n, status) in [
        (
            "healthy-boundary",
            "self.n <= 3",
            "self.n < 2",
            0,
            2,
            ConstrainedMethodStatus::ConstraintReached,
        ),
        (
            "healthy-no-boundary",
            "self.n <= 3",
            "self.n <= 3",
            0,
            3,
            ConstrainedMethodStatus::BoundedSuccess,
        ),
        (
            "later-boundary-violation",
            "self.n < 2",
            "self.n < 2",
            0,
            2,
            ConstrainedMethodStatus::Falsified,
        ),
        (
            "initial-boundary-violation",
            "self.n < 2",
            "self.n < 2",
            2,
            2,
            ConstrainedMethodStatus::Falsified,
        ),
    ] {
        let f = fixture(&document(), name, safety, &[constraint]);
        let initial = initial(initial_n);
        let bindings = bindings(&f);
        let provisions = request(&f, &initial, &bindings);
        let method = generate_constrained_method(&provisions).unwrap();
        let evidence = run_installed(&method);
        let safety = owner_evaluation(
            &f,
            "EvalSafety",
            observation_n,
            true,
            1,
            provisions.evaluator_limits,
        );
        let constraint = owner_evaluation(
            &f,
            "EvalConstraint0",
            observation_n,
            true,
            1,
            provisions.evaluator_limits,
        );
        let observation = ConstrainedOwnerObservation {
            method: method.identity(),
            safety: Ok(&safety),
            constraint: if status == ConstrainedMethodStatus::Falsified {
                None
            } else {
                Some(Ok(&constraint))
            },
            playback: evidence.playback(),
        };
        let result = classify_constrained_method(&evidence, &observation).unwrap_or_else(|error| {
            panic!(
                "{name}: {error}; {}",
                serde_json::to_string(&evidence).unwrap()
            )
        });
        assert_eq!(result.status(), status, "{name}");
        if status == ConstrainedMethodStatus::Falsified {
            assert!(
                evidence.playback().is_some(),
                "{name}: actual backend playback"
            );
            assert_eq!(
                constrained_terminal_value(&result, None).unwrap_err(),
                ConstrainedBindingRefusal::MissingReplay
            );
            assert_eq!(
                constrained_terminal_value(
                    &result,
                    Some(ConstrainedReplaySettlement {
                        method: method.identity(),
                        settlement: ReplaySettlement::Reproduced
                    })
                )
                .unwrap_err(),
                ConstrainedBindingRefusal::UnsupportedTerminalRepresentation
            );
        } else {
            assert_eq!(
                result.boundary(),
                Some(if status == ConstrainedMethodStatus::ConstraintReached {
                    ConstraintBoundary::Reached
                } else {
                    ConstraintBoundary::NotReached
                })
            );
            assert_eq!(
                constrained_terminal_value(&result, None).unwrap_err(),
                ConstrainedBindingRefusal::UnsupportedTerminalRepresentation
            );
            assert_eq!(
                constrained_terminal_value(
                    &result,
                    Some(ConstrainedReplaySettlement {
                        method: method.identity(),
                        settlement: ReplaySettlement::Reproduced
                    })
                )
                .unwrap_err(),
                ConstrainedBindingRefusal::UnexpectedReplay
            );
        }
        println!(
            "CONTROL {name}: {:?} {:?}",
            result.status(),
            result.boundary()
        );
    }
}
