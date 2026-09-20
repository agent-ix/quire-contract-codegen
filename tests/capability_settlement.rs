//! FR-019 capability settlement: one `negotiate_*` arm over a closed backend kind.
//!
//! Each test walks one row of FR-290's ordered rules, and the last one asserts
//! the seam itself: that no `Disposition` is constructed outside a `negotiate_*`
//! function anywhere in `src/`. That gate is the reason the property is checkable
//! rather than prose — the arms settle correctly today either way, and nothing
//! but the scan says so tomorrow.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
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
                member: "capability_vocabulary",
                received: supplied.to_owned()
            },
            "no settlement is returned, so no label of the carrier was read"
        );
    }
}

/// No `Disposition` is constructed outside a `negotiate_*` function anywhere in
/// this repository's Rust sources.
///
/// This is the S9 seam stated as a gate. The arms settle correctly today with or
/// without it; what it catches is the settlement added next year somewhere else,
/// which would give the same behaviour at run time and leave "negotiate_* is the
/// only settlement point" true only by coincidence.
///
/// The scan parses each file with `syn` rather than reading lines. A line scan
/// was written first and measured wrong in both directions: it missed a
/// `pub(crate) fn` — the visibility this repository's own idioms prefer — so an
/// injected settlement outside every arm passed green, and it failed on a
/// rustdoc link naming a variant, blaming the function above the comment. Both
/// are gone here because a parser distinguishes a declaration from a comment and
/// an expression from a pattern by construction, instead of by a rule about
/// where `=>` sits on a line.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_no_capability_is_settled_outside_a_negotiate_arm() {
    let offences: Vec<String> = scan_roots()
        .iter()
        .flat_map(|root| rust_sources(root))
        .flat_map(|file| settlement_sites(&file))
        .filter(|site| !site.enclosing.starts_with("negotiate_"))
        .map(|site| {
            format!(
                "{}: `{}` constructs a disposition outside a negotiate_* arm",
                site.file, site.enclosing
            )
        })
        .collect();
    assert!(
        offences.is_empty(),
        "capability settled outside a negotiate_* arm:\n{}",
        offences.join("\n")
    );
}

/// The scan reads the settlement point, and reads every arm of it.
///
/// Without this, a scan that matched nothing — a moved directory, a renamed
/// type, a parser that silently failed — would report the seam intact by looking
/// at nothing at all. The counts are the measured ones rather than a floor of
/// one, so a refactor that moves settlement out of reach of the scan fails here
/// even while the gate above stays green.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_the_settlement_scan_reads_the_settlement_point() {
    let sites: Vec<SettlementSite> = scan_roots()
        .iter()
        .flat_map(|root| rust_sources(root))
        .flat_map(|file| settlement_sites(&file))
        .collect();
    let arms: BTreeSet<&str> = sites
        .iter()
        .map(|site| site.enclosing.as_str())
        .filter(|name| name.starts_with("negotiate_"))
        .collect();
    assert_eq!(
        arms,
        BTreeSet::from([
            "negotiate_item",
            "negotiate_kani",
            "negotiate_single_candidate"
        ]),
        "the settlement functions the scan reads are not the ones this crate has"
    );
    assert!(
        sites.len() >= 12,
        "the scan found only {} settlement sites; it was reading 12 when this gate was written",
        sites.len()
    );
}

/// The roots the seam covers: every Rust source this repository builds.
///
/// FR-019-AC-5 says "anywhere in this repository", and a settlement added to the
/// conformance producer under `examples/` would satisfy a `src/`-only scan while
/// settling capabilities outside every arm.
fn scan_roots() -> Vec<PathBuf> {
    ["src", "examples"]
        .into_iter()
        .map(PathBuf::from)
        .filter(|root| root.is_dir())
        .collect()
}

/// One place a [`Disposition`] is constructed.
struct SettlementSite {
    /// `path:line`, for a message that points at the construction itself.
    file: String,
    /// The innermost named function containing it, or `<file scope>`.
    enclosing: String,
}

/// Every [`Disposition`] construction in one file, with the function it sits in.
fn settlement_sites(file: &Path) -> Vec<SettlementSite> {
    let text = fs::read_to_string(file).expect("a Rust source reads");
    let parsed = syn::parse_file(&text).expect("a Rust source parses");
    let mut visitor = Settlements {
        file: file.to_path_buf(),
        enclosing: vec![],
        in_disposition_impl: false,
        aliases: disposition_aliases(&parsed),
        found: vec![],
    };
    syn::visit::Visit::visit_file(&mut visitor, &parsed);
    visitor.found
}

/// Every name that refers to [`Disposition`] in this file: the type itself, any
/// `type` alias of it, and — when the file glob-imports its variants — the empty
/// name, which stands for a bare `Supported { .. }`.
fn disposition_aliases(parsed: &syn::File) -> BTreeSet<String> {
    let mut names = BTreeSet::from(["Disposition".to_owned()]);
    for item in &parsed.items {
        match item {
            syn::Item::Type(alias) => {
                if let syn::Type::Path(path) = alias.ty.as_ref() {
                    if last_segment(&path.path).is_some_and(|name| name == "Disposition") {
                        names.insert(alias.ident.to_string());
                    }
                }
            }
            syn::Item::Use(import) if glob_imports_disposition(&import.tree) => {
                names.insert(String::new());
            }
            _ => {}
        }
    }
    names
}

fn glob_imports_disposition(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            (path.ident == "Disposition" && matches!(*path.tree, syn::UseTree::Glob(_)))
                || glob_imports_disposition(&path.tree)
        }
        syn::UseTree::Group(group) => group.items.iter().any(glob_imports_disposition),
        _ => false,
    }
}

fn last_segment(path: &syn::Path) -> Option<String> {
    path.segments
        .last()
        .map(|segment| segment.ident.to_string())
}

/// The four disposition variants. A construction names one of these.
const VARIANTS: [&str; 4] = [
    "Supported",
    "RequiresBound",
    "Unsupported",
    "InvalidRequest",
];

struct Settlements {
    file: PathBuf,
    enclosing: Vec<String>,
    in_disposition_impl: bool,
    aliases: BTreeSet<String>,
    found: Vec<SettlementSite>,
}

impl Settlements {
    /// Whether `path` names a disposition variant being built.
    ///
    /// Three spellings reach the same variant and all three count: the qualified
    /// `Disposition::Supported`, `Self::Supported` inside `impl Disposition`, and
    /// a bare `Supported` in a file that glob-imports the variants. A constructor
    /// method added to `impl Disposition` is the likely next edit, and it is the
    /// `Self::` case.
    fn is_settlement(&self, path: &syn::Path) -> bool {
        let Some(variant) = last_segment(path) else {
            return false;
        };
        if !VARIANTS.contains(&variant.as_str()) {
            return false;
        }
        match path.segments.len() {
            1 => self.aliases.contains(""),
            _ => {
                let qualifier = path.segments[path.segments.len() - 2].ident.to_string();
                self.aliases.contains(&qualifier)
                    || (qualifier == "Self" && self.in_disposition_impl)
            }
        }
    }

    fn record(&mut self) {
        self.found.push(SettlementSite {
            file: self.file.display().to_string(),
            enclosing: self
                .enclosing
                .last()
                .cloned()
                .unwrap_or_else(|| "<file scope>".to_owned()),
        });
    }
}

impl<'ast> syn::visit::Visit<'ast> for Settlements {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.enclosing.push(node.sig.ident.to_string());
        syn::visit::visit_item_fn(self, node);
        self.enclosing.pop();
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.enclosing.push(node.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, node);
        self.enclosing.pop();
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        self.enclosing.push(node.sig.ident.to_string());
        syn::visit::visit_trait_item_fn(self, node);
        self.enclosing.pop();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let was = self.in_disposition_impl;
        if let syn::Type::Path(path) = node.self_ty.as_ref() {
            self.in_disposition_impl = last_segment(&path.path)
                .is_some_and(|name| self.aliases.contains(&name) && !name.is_empty());
        }
        syn::visit::visit_item_impl(self, node);
        self.in_disposition_impl = was;
    }

    // Only expressions are visited for construction. A pattern that matches a
    // disposition reads one and is not a settlement, and the visitor never
    // reaches a pattern through these two methods.
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        if self.is_settlement(&node.path) {
            self.record();
        }
        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if self.is_settlement(&node.path) {
            self.record();
        }
        syn::visit::visit_expr_path(self, node);
    }
}

/// Every `.rs` file under `root`, recursively.
fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in fs::read_dir(root).expect("a scan root is a directory") {
        let path = entry.expect("a directory entry reads").path();
        if path.is_dir() {
            found.extend(rust_sources(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
    found.sort();
    found
}

// ---------------------------------------------------------------------------
// FR-019-AC-6: the record a probe's observation produces
// ---------------------------------------------------------------------------

/// A routed item whose pinned tool is absent, and one whose tool is another
/// identity, each record `unsupported`/`tool-unavailable` while keeping the
/// item's `supported` disposition.
///
/// The observation is constructed, not measured. Resolving a launcher through
/// `CARGO_HOME` and `PATH`, and parsing what it prints, is
/// `KaniInstallation::discover`/`observe`'s job and is covered by FR-017's own
/// tests; what this test owns is the record that an observation produces. An
/// earlier version wrote a shell script and ran it, which measured nothing the
/// assertions could catch — `record_tool_probe` compares two strings — while
/// adding a scratch-directory race that failed once in a full suite.
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

    let absent = record_tool_probe(&routed, ProbePhase::BeforeRun, &ToolObservation::Absent)
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
        &ToolObservation::Identity("cargo-kani 0.66.0".to_owned()),
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

    // The disposition is not touched by either outcome: a claim no backend
    // discharges and a claim whose backend is not installed are different facts.
    assert_eq!(
        settlement.disposition,
        Disposition::Supported {
            backend: BackendKind::Kani.identity().to_owned(),
        }
    );

    // A matching tool records nothing, so the probe is not a second place a
    // result can be invented.
    assert_eq!(
        record_tool_probe(
            &routed,
            ProbePhase::BeforeRun,
            &ToolObservation::Identity("cargo-kani 0.67.0".to_owned())
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

    let expected_cause = Cause::ToolUnavailable {
        kind: CapabilityKind::ValueValidity,
        backend: BackendKind::Kani.identity().to_owned(),
        expected: "cargo-kani 0.67.0".to_owned(),
        actual: Some("cargo-kani 0.66.0".to_owned()),
    };
    let observed = ToolObservation::Identity("cargo-kani 0.66.0".to_owned());
    assert_eq!(
        record_tool_probe(&routed, ProbePhase::DuringRun, &observed),
        Some(ItemResult::Failed {
            cause: expected_cause.clone()
        }),
        "the result, not the cause, separates a change mid-run from absence at probe"
    );
    assert_eq!(
        record_tool_probe(&routed, ProbePhase::BeforeRun, &observed),
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
        RequestItem {
            extent: unbounded(true),
            ..item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
            )
        },
        item(
            RequestedKind::Known(CapabilityKind::Realizability),
            Candidates::Set(Vec::new()),
        ),
        item(
            RequestedKind::Absent,
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    ];
    for unsettled in unsettled {
        let settlement = settle_one(manifest.clone(), unsettled);
        assert_eq!(
            settlement.routed(&manifest),
            None,
            "{:?} routed a backend to probe",
            settlement.disposition
        );
    }
}

/// A manifest that names one identity twice routes nothing, rather than handing
/// the probe whichever entry came first.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_a_repeated_backend_identity_routes_nothing() {
    let first = kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)]);
    let second = BackendDescriptor {
        manifest_digest: OTHER_DIGEST.to_owned(),
        pinned_tool: "cargo-kani 0.68.0".to_owned(),
        ..first.clone()
    };
    let manifest = vec![first, second];
    let settlement = settle_one(
        manifest.clone(),
        item(
            RequestedKind::Known(CapabilityKind::ValueValidity),
            Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
        ),
    );
    assert_eq!(
        settlement.disposition,
        Disposition::Supported {
            backend: BackendKind::Kani.identity().to_owned(),
        },
        "settlement matched identity and digest, so it still routes to one entry"
    );
    assert_eq!(
        settlement.routed(&manifest),
        None,
        "the probe is not handed one of two pins by position"
    );
}

// ---------------------------------------------------------------------------
// The emitted forms, and the censuses they are read against
// ---------------------------------------------------------------------------

/// Every kind and cause serialises in the spelling FR-290 writes.
///
/// Nothing else in this suite reads a serialised form, so without this the eight
/// `Serialize` derives are a wire contract with no test: a consumer matching
/// FR-290's `unknown-backend` never matched the `unknown_backend` an earlier
/// `rename_all` produced, and every assertion here still passed.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_kinds_and_causes_serialise_in_the_spelling_the_spec_writes() {
    for kind in CapabilityKind::ALL {
        assert_eq!(
            serde_json::to_value(kind).expect("a kind serialises"),
            serde_json::Value::String(kind.label().to_owned()),
            "a kind serialises as its own label, not as a second spelling of it"
        );
    }

    let causes = [
        (Cause::AbsentKind, "absent-kind", "invalid_capability"),
        (
            Cause::UnknownKind {
                received: "nope".to_owned(),
            },
            "unknown-kind",
            "invalid_capability",
        ),
        (Cause::AbsentExtent, "absent-extent", "invalid_capability"),
        (
            Cause::UnknownBackend {
                backend: "cvc5".to_owned(),
            },
            "unknown-backend",
            "invalid_capability",
        ),
        (
            Cause::InconsistentCandidates {
                candidates: Vec::new(),
            },
            "inconsistent-candidates",
            "invalid_capability",
        ),
        (
            Cause::AmbiguousBackend {
                candidates: Vec::new(),
            },
            "ambiguous-backend",
            "invalid_capability",
        ),
        (
            Cause::UnsupportedRequestedCapability {
                kind: CapabilityKind::Refinement,
                backend: None,
            },
            "unsupported-requested-capability",
            "unsupported_projection",
        ),
        (
            Cause::UnboundedExtent {
                kind: CapabilityKind::Refinement,
                backend: "kani".to_owned(),
            },
            "unbounded-extent",
            "unsupported_projection",
        ),
        (
            Cause::ToolUnavailable {
                kind: CapabilityKind::Refinement,
                backend: "kani".to_owned(),
                expected: "cargo-kani 0.67.0".to_owned(),
                actual: None,
            },
            "tool-unavailable",
            "unsupported_projection",
        ),
    ];
    for (cause, spelling, code) in causes {
        assert_eq!(
            cause.code(),
            code,
            "{spelling} is typed under the wrong code"
        );
        let emitted = serde_json::to_value(&cause).expect("a cause serialises");
        assert_eq!(
            emitted.get("cause").and_then(serde_json::Value::as_str),
            Some(spelling),
            "the emitted cause is not the spelling FR-290 writes: {emitted}"
        );
    }
}

/// Each census array holds every variant of its kind, in index order.
///
/// `index` is an exhaustive match, so a new variant cannot compile without one;
/// reading every index back out of the array is what makes forgetting to extend
/// the array fail too. Without it `ALL` is a hand-kept list whose length happens
/// to be right, and `tc_030_every_backend_kind_has_a_dispatched_arm` — which
/// iterates it — silently checks fewer variants than exist.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_each_census_holds_every_variant_of_its_kind() {
    for (position, kind) in CapabilityKind::ALL.into_iter().enumerate() {
        assert_eq!(kind.index(), position, "{kind:?} is not at its own index");
        assert_eq!(
            CapabilityKind::from_label(kind.label()),
            Some(kind),
            "a member's own label does not read back as that member"
        );
    }
    for (position, backend) in BackendKind::ALL.into_iter().enumerate() {
        assert_eq!(
            backend.index(),
            position,
            "{backend:?} is not at its own index"
        );
    }
    assert_eq!(
        CapabilityKind::from_label("value-validity-v2"),
        None,
        "a label outside the vocabulary is refused, never mapped onto a member"
    );
}

/// A foreign contract version refuses the carrier, and the refusal says which
/// member was wrong.
///
/// Trace: TC-030
/// Provenance: codegen#86
#[test]
fn tc_030_a_foreign_contract_version_refuses_the_carrier_and_names_the_member() {
    let refusal = negotiate_backend_provider(&BackendProviderEnvelope {
        contract_version: "quire.backend-provider/v2".to_owned(),
        capability_vocabulary: String::new(),
        ..envelope(
            vec![kani(vec![(CapabilityKind::ValueValidity, Mode::Bounded)])],
            vec![item(
                RequestedKind::Known(CapabilityKind::ValueValidity),
                Candidates::Set(vec![candidate(BackendKind::Kani.identity(), KANI_DIGEST)]),
            )],
        )
    })
    .expect_err("a foreign contract version refuses the carrier");
    assert_eq!(
        refusal,
        EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            member: "contract_version",
            received: "quire.backend-provider/v2".to_owned(),
        },
        "both members are wrong here, and the refusal names the one it read"
    );
}
