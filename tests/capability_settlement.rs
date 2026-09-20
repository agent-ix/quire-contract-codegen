//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules, and the last one asserts
//! the seam itself: that no `Disposition` is constructed outside a `negotiate_*`
//! function anywhere in `src/`. That gate is the reason the property is checkable
//! rather than prose — the arms settle correctly today either way, and nothing
//! but the scan says so tomorrow.

use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use quire_contract_codegen::{
    negotiate_backend_provider, record_tool_probe, BackendDescriptor, BackendKind,
    BackendProviderEnvelope, Candidate, Candidates, CapabilityKind, Cause, Disposition,
    EnvelopeRefusal, ExtentClassification, ItemResult, ItemSettlement, Mode, ProbePhase,
    RequestItem, RequestedKind, ToolObservation, BACKEND_PROVIDER_CONTRACT, CAPABILITY_VOCABULARY,
};

const KANI_DIGEST: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";
const OTHER_DIGEST: &str = "f0e1d2c3b4a5968778695a4b3c2d1e0ff0e1d2c3b4a5968778695a4b3c2d1e0f";

fn kani(advertised: Vec<(CapabilityKind, Mode)>) -> BackendDescriptor {
    BackendDescriptor {
        identity: BackendKind::Kani.identity().to_owned(),
        manifest_digest: KANI_DIGEST.to_owned(),
        advertised,
        pinned_tool: "cargo-kani 0.67.0".to_owned(),
    }
}

fn candidate(identity: &str, digest: &str) -> Candidate {
    Candidate {
        identity: identity.to_owned(),
        manifest_digest: digest.to_owned(),
    }
}

fn bounded() -> Option<ExtentClassification> {
    Some(ExtentClassification {
        extent: Mode::Bounded,
        finite_bound_available: true,
    })
}

fn unbounded(finite_bound_available: bool) -> Option<ExtentClassification> {
    Some(ExtentClassification {
        extent: Mode::Unbounded,
        finite_bound_available,
    })
}

fn envelope(manifest: Vec<BackendDescriptor>, items: Vec<RequestItem>) -> BackendProviderEnvelope {
    BackendProviderEnvelope {
        contract_version: BACKEND_PROVIDER_CONTRACT.to_owned(),
        capability_vocabulary: CAPABILITY_VOCABULARY.to_owned(),
        manifest,
        items,
    }
}

fn settle_one(manifest: Vec<BackendDescriptor>, item: RequestItem) -> ItemSettlement {
    let settled = negotiate_backend_provider(&envelope(manifest, vec![item]))
        .expect("a well-formed envelope is not refused");
    assert_eq!(settled.len(), 1, "one item settles once");
    settled.into_iter().next().expect("one settlement")
}

fn item(kind: RequestedKind, candidates: Candidates) -> RequestItem {
    RequestItem {
        kind,
        extent: bounded(),
        named_backend: None,
        candidates,
    }
}

/// Every variant of the closed backend kind reaches a dispatched arm, and each
/// one settles rather than falling through.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_every_backend_kind_has_a_dispatched_arm() {
    for backend in BackendKind::ALL {
        let descriptor = kani(vec![(CapabilityKind::OperationContract, Mode::Bounded)]);
        let descriptor = BackendDescriptor {
            identity: backend.identity().to_owned(),
            ..descriptor
        };
        let settlement = settle_one(
            vec![descriptor],
            item(
                RequestedKind::Known(CapabilityKind::OperationContract),
                Candidates::Set(vec![candidate(backend.identity(), KANI_DIGEST)]),
            ),
        );
        assert_eq!(
            settlement.disposition,
            Disposition::Supported {
                backend: backend.identity().to_owned(),
            },
            "{} has no arm that settles",
            backend.identity()
        );
    }
}

/// An absent kind and an unknown label settle before the candidate table is
/// consulted, each with its own cause and the unknown label reported verbatim.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_an_absent_or_unknown_kind_settles_before_the_candidate_table() {
    // Candidates that would otherwise settle `supported`: the kind rules win.
    let candidates = Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]);
    let manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];

    let absent = settle_one(
        manifest.clone(),
        item(RequestedKind::Absent, candidates.clone()),
    );
    assert_eq!(
        absent.disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentKind
        }
    );

    let unknown = settle_one(
        manifest,
        item(
            RequestedKind::Unknown("value-validity-v2".to_owned()),
            candidates,
        ),
    );
    assert_eq!(
        unknown.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownKind {
                received: "value-validity-v2".to_owned()
            }
        },
        "the refused label is reported with its exact received bytes"
    );
}

/// An item carrying no extent classification settles `invalid-request`, and is
/// never defaulted to `bounded` to reach a disposition.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_an_absent_extent_classification_settles_invalid_request() {
    let settlement = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            extent: None,
            ..item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
            )
        },
    );
    assert_eq!(
        settlement.disposition,
        Disposition::InvalidRequest {
            cause: Cause::AbsentExtent
        }
    );
}

/// The unknown-backend mark, and a registered backend with no negotiation arm,
/// both settle `invalid-request` naming the backend.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_an_unroutable_backend_settles_invalid_request() {
    let marked = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::UnknownBackend("cvc5".to_owned()),
        ),
    );
    assert_eq!(
        marked.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: "cvc5".to_owned()
            }
        }
    );

    // Registered, advertises the kind, and still has no arm here: FR-290's
    // first candidate row refuses it rather than treating registration as
    // routability.
    let registered_without_arm = BackendDescriptor {
        identity: "cvc5".to_owned(),
        manifest_digest: OTHER_DIGEST.to_owned(),
        advertised: vec![(CapabilityKind::ValueValidity, Mode::Unbounded)],
        pinned_tool: "cvc5 1.2.0".to_owned(),
    };
    let unarmed = settle_one(
        vec![registered_without_arm],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate("cvc5", OTHER_DIGEST)]),
        ),
    );
    assert_eq!(
        unarmed.disposition,
        Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: "cvc5".to_owned()
            }
        }
    );
}

/// A candidate absent from the manifest, and one that does not advertise the
/// item's kind, each settle `invalid-request` naming the offender.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_inconsistent_candidates_settle_invalid_request() {
    let absent_from_manifest = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), OTHER_DIGEST)]),
        ),
    );
    assert_eq!(
        absent_from_manifest.disposition,
        Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate(BackendKind::Kani.identity(), OTHER_DIGEST)]
            }
        },
        "a manifest digest that is not in the manifest is not the same candidate"
    );

    let does_not_advertise = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::Refinement),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    );
    assert_eq!(
        does_not_advertise.disposition,
        Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]
            }
        }
    );
}

/// An empty candidate set settles `unsupported`, warned, naming the kind and
/// the named backend when the request names one.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_an_empty_candidate_set_settles_unsupported_with_a_warning() {
    let unnamed = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        item(
            RequestedKind::Known(CapabilityKind::Realizability),
            Candidates::Set(Vec::new()),
        ),
    );
    assert_eq!(
        unnamed.disposition,
        Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind: CapabilityKind::Realizability,
                backend: None
            }
        }
    );
    let warning = unnamed.warning().expect("an unsupported settlement warns");
    assert!(
        warning.contains("realizability"),
        "the warning names the item's kind: {warning}"
    );

    let named = settle_one(
        vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
        RequestItem {
            named_backend: Some(BackendKind::Kani.identity().to_owned()),
            ..item(
                RequestedKind::Known(CapabilityKind::Realizability),
                Candidates::Set(Vec::new()),
            )
        },
    );
    let warning = named.warning().expect("an unsupported settlement warns");
    assert!(
        warning.contains("realizability") && warning.contains(BackendKind::Kani.identity()),
        "the warning names the kind and the named backend: {warning}"
    );
}

/// Two candidates with no named backend settle `invalid-request`, naming every
/// candidate in candidate order, identically under either registration order.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_two_candidates_with_no_named_backend_settle_ambiguous() {
    let second = BackendDescriptor {
        identity: "kani-nightly".to_owned(),
        manifest_digest: OTHER_DIGEST.to_owned(),
        advertised: vec![(CapabilityKind::ValueValidity, Mode::Unbounded)],
        pinned_tool: "cargo-kani 0.68.0".to_owned(),
    };
    let first = kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]);
    // Candidate order is bytewise by identity then digest, and is a property of
    // the set rather than of the manifest it was read from.
    let ordered = vec![
        candidate(BackendKind::Kani.identity(), KANI_DIGEST),
        candidate("kani-nightly", OTHER_DIGEST),
    ];

    for manifest in [
        vec![first.clone(), second.clone()],
        vec![second.clone(), first.clone()],
    ] {
        // Registration order is the manifest's order; the settlement may not read it.
        let settlement = settle_one(
            manifest,
            item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(ordered.clone()),
            ),
        );
        assert_eq!(
            settlement.disposition,
            Disposition::InvalidRequest {
                cause: Cause::AmbiguousBackend {
                    candidates: ordered.clone()
                }
            },
            "registration order changed the disposition or the candidate order"
        );
    }
}

/// Each row of the advertised-mode table settles its own disposition, and an
/// unbounded extent never settles `supported` on a bounded-only advertisement.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_the_advertised_mode_table_settles_each_row() {
    let kind = CapabilityKind::ValueValidity;
    let only = vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)];
    let kani_backend = BackendKind::Kani.identity().to_owned();

    let rows = [
        // (advertised mode, extent, finite bound available, expected)
        (
            Mode::Bounded,
            bounded(),
            Disposition::Supported {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Unbounded,
            bounded(),
            Disposition::Supported {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Unbounded,
            unbounded(false),
            Disposition::Supported {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Bounded,
            unbounded(true),
            Disposition::RequiresBound {
                backend: kani_backend.clone(),
            },
        ),
        (
            Mode::Bounded,
            unbounded(false),
            Disposition::Unsupported {
                cause: Cause::UnboundedExtent {
                    kind,
                    backend: kani_backend.clone(),
                },
            },
        ),
    ];

    for (advertised, extent, expected) in rows {
        let settlement = settle_one(
            vec![kani(vec![(kind, advertised)])],
            RequestItem {
                extent,
                ..item(RequestedKind::Known(kind), Candidates::Set(only.clone()))
            },
        );
        assert_eq!(
            settlement.disposition, expected,
            "advertised {advertised:?} against extent {extent:?}"
        );
    }
}

/// An envelope whose capability vocabulary is absent or another identity is
/// refused whole, and no kind of it is read.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_a_foreign_capability_vocabulary_refuses_the_carrier() {
    for supplied in ["", "quire.capability-kind/v2"] {
        let refusal = negotiate_backend_provider(&BackendProviderEnvelope {
            capability_vocabulary: supplied.to_owned(),
            ..envelope(
                vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
                vec![item(
                    RequestedKind::Known(CapabilityKind::ValueValidity),
                    Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
                )],
            )
        })
        .expect_err("a foreign vocabulary refuses the carrier");
        assert_eq!(
            refusal,
            EnvelopeRefusal::InvalidCapability {
                cause: "unsupported-version",
                received: supplied.to_owned()
            },
            "no settlement is returned, so no label of the carrier was read"
        );
    }
}

/// No `Disposition` is constructed outside a `negotiate_*` function anywhere in
/// `src/`.
///
/// This is the S9 seam stated as a gate. The arms settle correctly today with or
/// without it; what it catches is the settlement added next year somewhere else,
/// which would give the same behaviour at run time and leave "negotiate_* is the
/// only settlement point" true only by coincidence.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_no_capability_is_settled_outside_a_negotiate_arm() {
    let mut offences = Vec::new();
    for file in rust_sources(Path::new("src")) {
        let text = fs::read_to_string(&file).expect("a source file in src/ reads");
        let mut enclosing = String::from("<file scope>");
        for (number, line) in text.lines().enumerate() {
            if let Some(name) = function_name(line) {
                enclosing = name;
            }
            if constructs_disposition(line) && !enclosing.starts_with("negotiate_") {
                offences.push(format!(
                    "{}:{}: `{}` constructs a disposition outside a negotiate_* arm",
                    file.display(),
                    number + 1,
                    enclosing
                ));
            }
        }
    }
    assert!(
        offences.is_empty(),
        "capability settled outside a negotiate_* arm:\n{}",
        offences.join("\n")
    );
}

/// The scan finds the settlement it is meant to find.
///
/// Without this, a scan that matched nothing — a moved directory, a renamed
/// type — would report the seam intact by looking at nothing at all.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_the_settlement_scan_reads_the_settlement_point() {
    let mut dispositions = 0_usize;
    let mut arms = 0_usize;
    for file in rust_sources(Path::new("src")) {
        let text = fs::read_to_string(&file).expect("a source file in src/ reads");
        for line in text.lines() {
            if function_name(line).is_some_and(|name| name.starts_with("negotiate_")) {
                arms += 1;
            }
            if constructs_disposition(line) {
                dispositions += 1;
            }
        }
    }
    assert!(
        arms >= BackendKind::ALL.len(),
        "the scan found {arms} negotiate_* functions, fewer than the {} backend kinds",
        BackendKind::ALL.len()
    );
    assert!(
        dispositions > 0,
        "the scan found no disposition at all, so it is asserting over an empty set"
    );
}

/// Whether a line *constructs* a capability [`Disposition`], as opposed to
/// matching one or naming a different crate's disposition type.
///
/// Two distinctions the scan has to make, and why each one is drawn where it is:
///
/// - `ObligationDisposition::`, `ExactScalarDisposition::` and the IR's own
///   `CapabilityDisposition::` all end in the same token. The occurrence counts
///   only when the character before it cannot continue an identifier, so those
///   three are not this seam and do not report as breaches of it.
/// - A pattern reads a disposition; an expression builds one, and the two are
///   separated by where the occurrence sits. On a line with `=>` the pattern is
///   to its left and the built value to its right, so only an occurrence after
///   the last `=>` counts; and an occurrence that directly follows `let `,
///   `if let ` or `while let ` is that binding's pattern, not a value. Anything
///   else — `let settled = Disposition::…` included — is a construction, so
///   `negotiate_kani`'s own `Mode::Bounded => Disposition::…` arms stay inside
///   the gate rather than every match arm in the crate being exempt.
fn constructs_disposition(line: &str) -> bool {
    let cut = line.rfind("=>").map_or(0, |index| index + 2);
    let Some(tail) = line.get(cut..) else {
        return false;
    };
    tail.match_indices("Disposition::").any(|(index, _)| {
        let before = &tail[..index];
        let continues_identifier = before
            .chars()
            .next_back()
            .is_some_and(|character| character.is_alphanumeric() || character == '_');
        let binds_a_pattern = before.ends_with("let ");
        !continues_identifier && !binds_a_pattern
    })
}

/// The name of the function a line declares, if it declares one.
fn function_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let rest = trimmed
        .strip_prefix("pub fn ")
        .or_else(|| trimmed.strip_prefix("fn "))
        .or_else(|| trimmed.strip_prefix("pub const fn "))
        .or_else(|| trimmed.strip_prefix("const fn "))?;
    let name: String = rest
        .chars()
        .take_while(|character| character.is_alphanumeric() || *character == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

/// Every `.rs` file under `root`, recursively.
fn rust_sources(root: &Path) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    let entries = fs::read_dir(root).expect("the crate has a src/ directory");
    for entry in entries {
        let path = entry.expect("a directory entry reads").path();
        if path.is_dir() {
            found.extend(rust_sources(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
    found.sort();
    assert!(!found.is_empty(), "no Rust sources found under {root:?}");
    found
}

// ---------------------------------------------------------------------------
// FR-019-AC-6: the pinned-tool probe, after routing
// ---------------------------------------------------------------------------

/// A routed item whose pinned tool is absent, and one whose tool is another
/// identity, each record `unsupported`/`tool-unavailable` while keeping the
/// item's `supported` disposition.
///
/// The tool identities come from a real `KaniInstallation` measured out of a
/// scratch directory — one with no launcher at all, one with a launcher that
/// reports a version other than the pin — rather than from a hand-written
/// string, so the probe under test is the one the execution lane runs.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_an_absent_or_mismatched_pinned_tool_records_unsupported() {
    let manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];
    let settlement = settle_one(
        manifest.clone(),
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    );
    let routed = settlement
        .routed(&manifest)
        .expect("a supported item routes to its one candidate");
    assert_eq!(routed.backend, BackendKind::Kani.identity());
    assert_eq!(routed.pinned_tool, "cargo-kani 0.67.0");

    let absent = record_tool_probe(&routed, ProbePhase::BeforeRun, &observe_launcher(None))
        .expect("an absent tool is not a passing probe");
    assert_eq!(
        absent,
        ItemResult::Unsupported {
            cause: Cause::ToolUnavailable {
                kind: CapabilityKind::ValueValidity,
                backend: BackendKind::Kani.identity().to_owned(),
                expected: "cargo-kani 0.67.0".to_owned(),
                actual: None,
            }
        }
    );

    let mismatched = record_tool_probe(
        &routed,
        ProbePhase::BeforeRun,
        &observe_launcher(Some("0.66.0")),
    )
    .expect("a mismatched tool is not a passing probe");
    assert_eq!(
        mismatched,
        ItemResult::Unsupported {
            cause: Cause::ToolUnavailable {
                kind: CapabilityKind::ValueValidity,
                backend: BackendKind::Kani.identity().to_owned(),
                expected: "cargo-kani 0.67.0".to_owned(),
                actual: Some("cargo-kani 0.66.0".to_owned()),
            }
        },
        "the record names the expected and the actual identity, not just a failure"
    );

    // The disposition is not touched by either outcome. FR-290 keeps the two
    // apart so a consumer can tell a claim no backend discharges from a claim
    // whose backend was not installed.
    assert_eq!(
        settlement.disposition,
        Disposition::Supported {
            backend: BackendKind::Kani.identity().to_owned(),
        }
    );

    // A matching tool records nothing at all, so the probe is not a second place
    // a result can be invented.
    assert_eq!(
        record_tool_probe(
            &routed,
            ProbePhase::BeforeRun,
            &observe_launcher(Some("0.67.0"))
        ),
        None
    );
}

/// A tool that changes after a passing probe records `failed`, with the same
/// cause as absence found at probe time.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_a_tool_that_changes_after_a_passing_probe_records_failed() {
    let manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];
    let routed = settle_one(
        manifest.clone(),
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    )
    .routed(&manifest)
    .expect("a supported item routes");

    let during = record_tool_probe(
        &routed,
        ProbePhase::DuringRun,
        &observe_launcher(Some("0.66.0")),
    )
    .expect("a changed tool is recorded");
    let expected_cause = Cause::ToolUnavailable {
        kind: CapabilityKind::ValueValidity,
        backend: BackendKind::Kani.identity().to_owned(),
        expected: "cargo-kani 0.67.0".to_owned(),
        actual: Some("cargo-kani 0.66.0".to_owned()),
    };
    assert_eq!(
        during,
        ItemResult::Failed {
            cause: expected_cause.clone()
        },
        "the result, not the cause, separates a change mid-run from absence at probe"
    );
    assert_eq!(
        record_tool_probe(
            &routed,
            ProbePhase::BeforeRun,
            &observe_launcher(Some("0.66.0"))
        ),
        Some(ItemResult::Unsupported {
            cause: expected_cause
        })
    );
}

/// Nothing but a `supported` item routes, so no probe has a tool to run against
/// before settlement has chosen one.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_no_item_routes_before_it_is_settled_supported() {
    let manifest = vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])];
    let unsettled = [
        // requires-bound
        RequestItem {
            extent: unbounded(true),
            ..item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
            )
        },
        // unsupported
        item(
            RequestedKind::Known(CapabilityKind::Realizability),
            Candidates::Set(Vec::new()),
        ),
        // invalid-request
        item(
            RequestedKind::Absent,
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    ];
    for item in unsettled {
        let settlement = settle_one(manifest.clone(), item);
        assert_eq!(
            settlement.routed(&manifest),
            None,
            "{:?} routed a backend to probe",
            settlement.disposition
        );
    }
}

/// The identity a real `cargo-kani` launcher reports, or `Absent` when the
/// scratch installation holds no launcher at all.
///
/// `KaniInstallation` is constructed against a scratch directory rather than by
/// mutating `PATH`: `env::set_var` races every other thread reading the
/// environment, including a child process snapshotting it at fork.
///
/// Each call takes a fresh directory, and clears it first. `CARGO_TARGET_TMPDIR`
/// survives between runs while the counter restarts at zero, so the absent case
/// inherited a launcher an earlier run's mismatch case had written into the same
/// numbered directory — which is how this helper failed once inside a full
/// `cargo test` and never in isolation. The exec is also retried on `ETXTBSY`,
/// which Linux returns for a file still open for writing anywhere in the system.
fn observe_launcher(version: Option<&str>) -> ToolObservation {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let directory = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "codegen-86-probe-{}",
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("the scratch directory is created");
    let launcher = directory.join("cargo-kani");
    let Some(version) = version else {
        // No launcher at all: the probe finds nothing to measure, which is the
        // absent-tool fault without a process in it.
        assert!(
            !launcher.exists(),
            "the absent case starts from an empty directory"
        );
        return ToolObservation::Absent;
    };
    fs::write(
        &launcher,
        format!("#!/bin/sh\necho 'cargo-kani {version}'\n"),
    )
    .expect("the fake launcher is written");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755))
            .expect("the fake launcher is executable");
    }
    for attempt in 0..8 {
        match std::process::Command::new(&launcher)
            .args(["kani", "--version"])
            .output()
        {
            Ok(output) => {
                return ToolObservation::Identity(
                    String::from_utf8(output.stdout)
                        .expect("the launcher prints UTF-8")
                        .trim()
                        .to_owned(),
                )
            }
            Err(error) if error.raw_os_error() == Some(26) => {
                std::thread::sleep(std::time::Duration::from_millis(10 * (attempt + 1)));
            }
            Err(error) => panic!("the fake launcher did not run: {error}"),
        }
    }
    panic!("the fake launcher stayed busy for every attempt");
}
