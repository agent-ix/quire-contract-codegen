// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic QSL inputs exercise the single generation entry and dedicated method execution.
mod support;
use ix_trace_rs::trace;
use qsl_replay::SnapshotValue;
use quire_contract_codegen::*;
use serde_json::json;
use support::*;
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
    let a = emit_method(&request).unwrap();
    let again = emit_method(&request).unwrap();
    assert_eq!(a.identity(), again.identity());
    assert_eq!(a.rust(), again.rust());
    request.constraint.as_mut().unwrap().clause = &f.constraints[1];
    let b = emit_method(&request).unwrap();
    assert_eq!(a.identity().obligation, b.identity().obligation);
    assert_ne!(a.identity(), b.identity());
    assert_ne!(a.rust().path, b.rust().path);
    request.constraint = None;
    let ordinary = emit_method(&request).unwrap();
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
        emit_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::NotConstraint
    );
    let mut missing = f.constraints[0].clone();
    missing.ordinal += 100;
    request.constraint.as_mut().unwrap().clause = &missing;
    assert_eq!(
        emit_method(&request).unwrap_err(),
        ConstrainedMethodRefusal::MissingConstraint
    );
    request.constraint.as_mut().unwrap().clause = &f.constraints[0];
    request.context = &f.safety;
    assert_eq!(
        emit_method(&request).unwrap_err(),
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
        emit_method(&request).unwrap_err(),
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
    let method = emit_method(&request).unwrap();
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
        emit_method(&incomplete).unwrap_err(),
        ConstrainedMethodRefusal::OperationBindings
    );
    let mut missing = initial.clone();
    missing.remove("aux");
    incomplete.initial = &missing;
    assert_eq!(
        emit_method(&incomplete).unwrap_err(),
        ConstrainedMethodRefusal::InitialState
    );
    let mut invalid = initial.clone();
    invalid.insert("n".into(), SnapshotValue::integer("4"));
    incomplete.initial = &invalid;
    assert_eq!(
        emit_method(&incomplete).unwrap_err(),
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
    let method = emit_method(&request(&f, &initial, &bindings)).unwrap();
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
#[ignore = "installed bounded Kani lane; explicit 4 GiB per control, never the parked 16 GiB proof"]
fn installed_small_method_controls_retain_boundary_safety_and_no_boundary_success() {
    for (name, safety, constraint, initial_n, observation_n, status) in [
        (
            "healthy-boundary",
            "true",
            "self.flag",
            0,
            1,
            ConstrainedMethodStatus::ConstraintReached,
        ),
        (
            "healthy-no-boundary",
            "true",
            "true",
            0,
            1,
            ConstrainedMethodStatus::BoundedSuccess,
        ),
        (
            "later-boundary-violation",
            "self.flag",
            "self.flag",
            0,
            1,
            ConstrainedMethodStatus::Falsified,
        ),
        (
            "initial-boundary-violation",
            "self.flag",
            "self.flag",
            1,
            1,
            ConstrainedMethodStatus::Falsified,
        ),
    ] {
        let mut document = document();
        document["types"][0]["constraints"][1]["operands"]["value"] = json!("1");
        let f = fixture(&document, name, safety, &[constraint]);
        let mut initial = initial(0);
        initial.insert("flag".into(), SnapshotValue::boolean(initial_n == 0));
        let bindings = bindings(&f);
        let mut provisions = request(&f, &initial, &bindings);
        provisions.maximum_steps = 1;
        generate_v2_clause_native_oracle(&f.package, &f.safety).expect("native safety condition");
        generate_v2_clause_native_oracle(&f.package, &f.constraints[0])
            .expect("native constraint condition");
        let method = emit_method_with_unwind(&provisions, 3).unwrap();
        // Independently evaluate every state in this admitted domain, including every
        // possible initial/successor/boundary state of either complete-census operation.
        for n in 0..=1 {
            for flag in [false, true] {
                for aux in 0..=1 {
                    for (function, expression) in
                        [("EvalSafety", safety), ("EvalConstraint0", constraint)]
                    {
                        let original = owner_evaluation(
                            &f,
                            function,
                            n,
                            flag,
                            aux,
                            provisions.evaluator_limits,
                        );
                        assert!(
                            matches!(original, qsl_replay::CallOutcome::Completed(qsl_replay::CallValue::Boolean(value)) if value == (expression == "true" || flag)),
                            "{name}: {function} at ({n},{flag},{aux})"
                        );
                    }
                }
            }
        }
        let evidence = run_installed(&method);
        let safety = owner_evaluation(
            &f,
            "EvalSafety",
            observation_n,
            false,
            1,
            provisions.evaluator_limits,
        );
        let constraint = owner_evaluation(
            &f,
            "EvalConstraint0",
            observation_n,
            false,
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

#[trace("TC-062", "FR-048-AC-1", "FR-048-AC-6", "FR-048-AC-11")]
#[test]
fn boolean_and_partial_conditions_use_the_owner_oracle_and_unbounded_domains_refuse() {
    for condition in ["self.flag", "true", "false", "self.n mod self.aux = 0"] {
        let f = fixture(
            &document(),
            "constrained-conditions",
            "self.n <= 3",
            &[condition],
        );
        let initial = initial(0);
        let bindings = bindings(&f);
        let request = request(&f, &initial, &bindings);
        let method = emit_method(&request).expect("Boolean and partial integer owner conditions");
        if condition == "self.n mod self.aux = 0" {
            assert!(method
                .rust()
                .contents
                .contains("undefined constraint evaluation"));
        }
        let outcome =
            owner_evaluation(&f, "EvalConstraint0", 0, false, 0, request.evaluator_limits);
        if condition != "self.n mod self.aux = 0" {
            assert!(matches!(
                outcome,
                qsl_replay::CallOutcome::Completed(qsl_replay::CallValue::Boolean(value)) if value == (condition == "true")
            ));
        } else {
            assert!(matches!(outcome, qsl_replay::CallOutcome::Undefined { .. }));
            assert!(method.rust().contents.contains("rt::modulo"));
        }
    }
    for slot in ["unread-field", "uncalled-parameter", "uncalled-result"] {
        let mut doc = document();
        match slot {
            "unread-field" => {
                doc["types"][1]["fields"][2]["typeRef"] = json!("ix://quire/native/Integer")
            }
            "uncalled-parameter" => {
                doc["types"][1]["operations"][0]["params"][0]["typeRef"] =
                    json!("ix://quire/native/Integer")
            }
            "uncalled-result" => {
                doc["types"][1]["operations"][1]["returns"]["typeRef"] =
                    json!("ix://quire/native/Integer")
            }
            _ => unreachable!(),
        }
        let f = fixture(
            &doc,
            "constrained-unbounded",
            "self.n <= 3",
            &["self.n < 2"],
        );
        let initial = initial(0);
        let bindings = bindings(&f);
        assert_eq!(
            emit_method(&request(&f, &initial, &bindings)).unwrap_err(),
            ConstrainedMethodRefusal::UnsupportedDomain,
            "{slot}"
        );
    }
}

#[trace("TC-062", "FR-048-AC-1", "FR-048-AC-2", "FR-048-AC-8")]
#[test]
fn equivalent_package_instances_cannot_mix_handles_and_census_refusal_retains_owner_payload() {
    let a = fixture(
        &document(),
        "constrained-instance",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let b = fixture(
        &document(),
        "constrained-instance",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let initial = initial(0);
    let bindings_a = bindings(&a);
    let bindings_b = bindings(&b);
    let mut request_a = request(&a, &initial, &bindings_a);
    let request_b = request(&b, &initial, &bindings_b);
    assert_eq!(
        emit_method(&request_a).unwrap().identity(),
        emit_method(&request_b).unwrap().identity()
    );
    request_a.constraint = request_b.constraint;
    assert_eq!(
        emit_method(&request_a).unwrap_err(),
        ConstrainedMethodRefusal::WrongPackage
    );
    request_a.constraint = request(&a, &initial, &bindings_a).constraint;
    request_a.census_work = 0;
    let refusal = emit_method(&request_a).unwrap_err();
    let ConstrainedMethodRefusal::Census { source } = &refusal else {
        panic!("original census refusal: {refusal:?}")
    };
    assert_eq!(
        source.code,
        quire_contract_model::CheckedOperationReadErrorCode::WorkLimit
    );
    assert_eq!(source.context, a.context);
    assert_eq!(source.limit, 0);
    assert!(source.work > source.limit);
    let retained = serde_json::to_value(&refusal).unwrap();
    assert!(retained.to_string().contains("WorkLimit"));
}

fn run_installed(method: &KaniConstrainedMethodHarness) -> ConstrainedBackendEvidence {
    let scratch = crate::scratch_crate::TemporaryDirectory::new("constrained-method-installed");
    std::fs::create_dir_all(scratch.path.as_path().join("src")).unwrap();
    std::fs::create_dir_all(scratch.path.as_path().join(".cargo")).unwrap();
    crate::scratch_crate::write_manifest(scratch.path.as_path(),"[package]\nname=\"constrained_method_control\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[workspace]\n[dependencies]\nquire-exact={git=\"https://github.com/agent-ix/quire-exact\",branch=\"main\"}\nquire-contract-runtime={git=\"https://github.com/agent-ix/quire-contract-runtime\",branch=\"main\",features=[\"exact\"]}\n");
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

const SUBJECT: &str = r#"
pub mod subject {
    pub fn idle(_n:&mut i64,_flag:&mut bool,_aux:&mut i64,enabled:bool)->bool { enabled }
    pub fn nudge(n:&mut i64,flag:&mut bool,aux:&mut i64,delta:i64) {
        *n=(*n+delta).min(1);
        *flag=!*flag;
        *aux=(*aux+1)%2;
    }
}
"#;

#[trace("TC-062", "FR-048-AC-1")]
#[test]
fn structurally_identical_constraint_occurrences_require_an_explicit_selection() {
    let f = fixture(
        &document(),
        "constrained-ambiguous",
        "self.n <= 3",
        &["self.n < 2", "self.n < 2"],
    );
    let initial = initial(0);
    let bindings = bindings(&f);
    assert_eq!(
        f.constraints.len(),
        1,
        "canonical interning retains both occurrences on one node"
    );
    assert_eq!(
        emit_method(&request(&f, &initial, &bindings)).unwrap_err(),
        ConstrainedMethodRefusal::AmbiguousConstraint
    );
}

pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let f = fixture(
        &document(),
        "constrained-cover-audit",
        "self.n <= 3",
        &["self.n < 2"],
    );
    let initial = initial(0);
    let bindings = bindings(&f);
    let mut request = request(&f, &initial, &bindings);
    let constrained = emit_method(&request).unwrap().rust().contents.clone();
    request.constraint = None;
    let unconstrained = emit_method(&request).unwrap().rust().contents.clone();
    vec![
        ("constrained method", constrained),
        ("unconstrained method", unconstrained),
    ]
}

#[trace("TC-062", "FR-048-AC-11")]
#[test]
fn a_standalone_collection_constraint_remains_unsupported_without_a_harness() {
    let mut doc = document();
    let mut field = doc["types"][1]["fields"][2].clone();
    field["name"] = json!("items");
    field["identity"] = json!(format!("{OBJECT}/items"));
    field["multiplicity"] = json!({"lower":0,"upper":3,"ordered":true,"unique":false});
    doc["types"][1]["fields"]
        .as_array_mut()
        .unwrap()
        .push(field);
    let bytes = quire_canonical::to_vec(&doc, quire_canonical::Limits::new(1 << 20)).unwrap();
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\ninvariant Safe using v on M::Node at current {{ self.n <= 3 }}\nconstraint Limit using v on M::Node at current {{ forall(item in self.items: item >= 0) }}\n");
    let f = fixture_source(bytes, "constrained-standalone-collection", source);
    let quantifier = f.package.graph().nodes.iter().find(|node| matches!(f.package.term(&node.node_id),Some(quire_contract_model::CheckedTerm::Application {operation,..}) if operation.identity()==Some("quire.op.collection.forall"))).expect("QSL emitted forall");
    let items = [ObligationItem::CollectionClaim {
        package: &f.package,
        function_id: &f.constraints[0],
        node_id: &quantifier.node_id,
    }];
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: proof_ceilings(),
        unwind: 8,
        items: &items,
        subject_path: "crate::subject",
    })
    .unwrap();
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        constrained_harnesses,
        ..
    } = outcome
    else {
        panic!("per-item typed unsupported")
    };
    assert!(collection_harnesses.is_empty());
    assert!(constrained_harnesses.is_empty());
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: CollectionUnsupportedReason::UnsupportedClaimShape,
                ..
            }
        }
    ));
}
