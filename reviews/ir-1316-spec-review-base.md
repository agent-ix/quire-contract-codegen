---
id: SR-2026110283861
title: IR-1316 PR743 spec-review/base
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; IR-1316; spec/evidence/functional/FR-004-vacuity-evidence.md,
  spec/evidence/matrix/tests.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md,
  spec/replay/matrix/tests.md, spec/routed/functional/FR-019-capability-settlement.md,
  spec/strategy/functional/FR-002-tristate-proptest.md, spec/strategy/matrix/tests.md,
  tests/it/capability_settlement.rs
review_set: base
---

## Summary

Ticket: IR-1316. PR743 frozen head 05a51bd1f2d81af7f1573af7dedd3f15816c093d. Scope is only the FR-002/004/019/024 non-Kani triage slice, not completion of the entire parent ticket or repository. Examined 51 active scoped criteria: one gains real test evidence; 50 remain Planned with IR-1316 ownership and concrete reasons. Retired/superseded criteria retain their existing normative text and no implementation credit. FR-002 structural replacements AC-25/26 exist and remain Planned. No requirement/AC statement or verification cell changed, no row was withdrawn, and no new pin, digest or tracking mechanism was introduced.

Independent released Quire 0.38.3 / engine 0.52.1 strict coverage at the frozen head exits 1: 405 unbacked, 689/1086 backed, zero status lies. Baseline supplied at current main a6df4a23 has 406 unbacked, 688/1086 backed, zero lies. Comparing (document,row_id) sets independently identifies only FR-019-AC-3 removed from unbacked; no new unbacked row. Evidence: /tmp/ir1316-review-coverage.json and /tmp/ir1316/baseline-current.json. These are report measurements, not full repository acceptance.

No applicable AssuranceProfile exists under spec/. Gap-analysis skipped: diff contains tests and status/spec prose only, with no production code. Base spec review applies; additional lenses skipped because no normative statements, domain objects, relationships, system boundary or structural artifact contract changed. Direct review is used without an enforced workflow. Shared spec/routed/matrix/tests.md is untouched; its conservative Planned entry does not negate the new trace. FR021/FR045 production files, Cargo.toml and Cargo.lock are untouched.

Base checklist applied to changed status prose and its owning criteria. Source inspection confirms HarnessRequest still accepts V1 TypedExpression and lacks the selected V2/terminal path; BoundCoverageInputs and V2ClauseCoverageInput accept detached observation buffers without campaign outcome/live QSL /2 admission; v2_llvm population scanning still opens package.graph().nodes outside core/ir. BackendKind remains Kani/Process; exact generic settlement tests do not establish the source-to-driver concrete collection composition or upstream registry invariant. Replay has no production minimizer/common multi-backend typed lineage path; FrameReplayInputs omits caller obligation identity and frame_identity delegates the producer claim, but that alone does not establish all required grant and structural mutation controls. These narrower truths match the new Planned reasons without upgrading partial evidence.

The 50 residual gaps are the authorized output of this triage, not newly introduced defects or claims of implementation. The unchanged retired/superseded rows are explicitly distinguished from active replacement criteria. Existing source/doc statements outside this changed slice are context only; no whole-repository quality verdict is asserted.

## Examined scope

```yaml
scope:
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's
    FR-014 V2 clause oracle.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and postcondition oracle record that the snapshot precedes precondition
    evaluation, which precedes subject invocation, which precedes postcondition evaluation,
    for every case.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens
    a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names
    no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression` or `oracle::boolean_v1`
    item, each of which today's V1 harness names or uses.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same for both selections yields a typed, located diagnostic
    naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsupported diagnostic and no artifact, and the generator
    emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected checked clause ids, authentic `claim` occurrences, and campaign
    policy is byte-identical. The repeated selections include the same FR-011 identity
    inputs; the generated header, runtime identities, source map, symbols, and remaining
    bytes all agree across repetitions.
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: 'PLANNED (IR-364). For each selected precondition and postcondition oracle,
    a seeded `Ok(Outcome::Undefined)`, `Ok(Outcome::Refused)`, `Ok(Outcome::Incomplete)`,
    `Err(ClauseStop::Oracle(OracleStop))`, and `Err(ClauseStop::InvalidConstant)`
    returns CG-owned `V2HarnessCaseResult::NonCompleted` retaining the exact typed
    stop cause, selected claim identity, and selected accepted typed `ExecutionPoint`.
    The generated campaign returns `V2HarnessCampaignResult::NonCompleted` carrying
    that case and a summary whose `attempted` includes this stopped case once and
    whose `non_completed` is one. No runtime '
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not evaluate the postcondition oracle.
- id: FR-002-AC-19
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Each V2 selection preserves the accepted QSL `ClauseBinding.execution_point`
    for its exact admitted package, checked clause id, and authentic `claim` occurrence
    through the injective runtime rendering; precondition and postcondition points
    may differ. Two requests selecting different authentic claims on the same precondition
    node each retain that node's accepted `Pre` operation point while selecting the
    correct occurrence. A rejection carries the precondition point; a failed or passed
    verdict after completed postcondition evaluation carries the postcondition point;
    a non-comp
- id: FR-002-AC-20
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). After FR-002-AC-10's clause id, kind, and distinctness
    checks, a wrong-role, wrong-ordinal, or other-node `claim` occurrence for either
    selected V2 clause yields `HarnessErrorCode::MissingClaimOccurrence` and no artifact
    even if a sibling occurrence has a point; an authentic selected tuple with no
    accepted point yields `HarnessErrorCode::MissingExecutionPoint` and no artifact
    even if another authentic occurrence of that node has a point; a render mutant
    that changes only that selection's generated harness or verdict point yields `HarnessErrorCode::ExecutionPointMismatch`
    befo
- id: FR-002-AC-22
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For either selection, an out-of-driver missing or non-claim
    occurrence returns `HarnessErrorCode::MissingClaimOccurrence`, a referenced model
    element absent from the admitted V2 relation returns `HarnessErrorCode::UnboundModelElement`
    naming it, and a process FR-331 owner differing from the admitted package returns
    `HarnessErrorCode::OwnerIdentityMismatch`. Each produces no artifact. A linked
    request uses one driver-exported/E4-emitted/E5-admitted package and a selected
    claim checked against that package; a foreign claim absent from it returns `HarnessErrorCode::MissingClaimO
- id: FR-002-AC-24
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-364). With a supplied runtime report whose snapshot is `at_limit`
    before a seeded non-completed V2 oracle case, `V2HarnessCampaignResult::NonCompleted`
    retains that case and a summary with `non_completed == 1` and exact `attempted
    == None`; it does not infer an exact prior count from the saturated report. The
    report's existing counters and `at_limit` signal remain unchanged, and no runtime
    verdict is constructed or recorded for the stopped case.
- id: FR-002-AC-25
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-21). The V2 harness header, `RequirementId`,
    `RevisionId`, source map and symbols carry FR-011's canonical structural `qocc2_`/`qpkg2_`
    renderings of each driver's selected claim's complete node, role and ordinal and
    the same admitted package's name and complete lock. Changing any one structural
    identity component changes its rendered text. An unchanged node and lock identity
    never excuses a changed admitted body from the existing body comparison.
- id: FR-002-AC-26
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815; replaces AC-23). At a V2 harness identity text comparison
    boundary, malformed, truncated, case-folded or otherwise noncanonical structural
    `qocc2_`/`qpkg2_` text, including an invalid byte count or malformed nested path
    or lock encoding, returns `HarnessErrorCode::InvalidIdentityRendering` before
    publication and without normalizing or mutating a report. A different but canonical
    occurrence returns `MissingClaimOccurrence`; a different but canonical package
    name or lock returns `OwnerIdentityMismatch`, likewise before publication. Exact
    typed and canonical-text comparisons agre
- id: FR-002-AC-21
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: RETIRED, replaced by FR-002-AC-25; the former digest-shaped identity rendering
    is obsolete and earns no implementation credit.
- id: FR-002-AC-23
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: context_only
  excerpt: RETIRED, replaced by FR-002-AC-26; the former digest-shaped text control
    is obsolete and earns no implementation credit.
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. Adverse coverage and non-success native execution cannot
    discharge the coverage obligation merely because a report serialized successfully.
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. CG binds coverage inside the same live campaign-runner
    invocation, using the original `/2` wire bytes and their QSL strict-reader typed
    result directly, the producer execution record, and the same generated source/map
    buffers used by that campaign. The runner starts the campaign with fresh private
    profile output, performs any required LLVM merge over only that output, starts
    LLVM export over only that profile, and captures exact export bytes and tool/version
    from that LLVM child's direct output stream while retaining the linked child handles.
    Binding verifies this live campaig
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. The consuming report retains vacuity and native outcome
    as evidence for IR FR-045; an oracle-success run with an unobserved consequent
    denies the coverage obligation, and LLVM probe observations alone produce no completed-proof
    credit or separate eligible denominator.
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED / UNRUN. CG consumes QSL''s typed strict-reader clause-run or
    command-error variant for exact `native-run-result/2`: a decisive basis retains
    its witness with assigned members, while `closed-scope` or `unavailable` retains
    none. Missing or other versions, missing/unknown basis, malformed witness, contradictory
    basis/witness and mixed/invalid variant shape are located wire refusals before
    binding; no `/1` fallback or CG copy of the wire reader exists.'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. QSL `run_clause` success and a decisive witness remain
    source-clause facts and do not count as generated-campaign execution or LLVM coverage;
    QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal
    failure remain distinct typed adverse/non-success facts. A bound live run does
    not upgrade any QSL disposition or itself grant campaign success, measured classification,
    proof credit or coverage discharge. The same reader's command-error variant retains
    stage/code/optional cause/message/details and unavailable basis as non-success,
    with no campaign qualifi
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. Only CG's campaign runner constructs the private, non-cloneable,
    non-serializable live run context; callers cannot construct or restore it from
    a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent,
    foreign or expired context, or an omitted association yields structured binding
    non-success retaining the typed QSL outcome and available identities, with no
    qualification, classification or discharge. A complete same-invocation context
    admits binding without changing the QSL outcome. The caller supplies maximum encoded-input
    and decoded-working-byte lim
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN. A controlled attempt to replace the strict-reader typed
    result, execution record, generated source, source map, LLVM export stream or
    LLVM tool/version with a foreign-run value cannot construct a valid same-invocation
    context, even when paths, QSL `package_id`, source/map and tool/version declarations
    match. In particular, foreign export bytes offered through a detached buffer or
    a different producer stream/child handle refuse as `producer_run_mismatch` before
    classification because neither is the current runner-owned campaign/profile/export
    chain. A malformed association obse
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED / UNRUN (IR-815; pending IR-1013). Source inspection shows the
    V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer
    of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which today's `analyze_bound_coverage`
    names.
- id: FR-004-AC-25
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: context_only
  excerpt: 'SUPERSEDED by FR-004-AC-26 under QSpec #449 / IR-1070; no implementation
    credit. Original planned text: For any executable clause in the V2 analyzer''s
    expected population selected for strategy generation, the generator uses the accepted
    QSL point returned by Contract IR FR-058''s typed accessor for the admitted package,
    checked clause id and authentic `claim` `CheckedOccurrence`. Two authentic occurrences
    on one node with different accepted points retain their distinct rendered points
    when selected separately. A wrong-role, wrong-ordinal or other-node occurrence,
    an authentic tuple with no acce'
- id: FR-004-AC-26
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: context_only
  excerpt: 'PLANNED (IR-1070). For each selected executable V2 clause, the generator
    uses the exact accepted point of that point-qualified clause node and authentic
    claim through IR FR-058; separate nodes with equal condition and anchor but different
    points produce distinct rendered points when selected separately, while two authentic
    claims of one pointed node produce the same point. The analyzer retains one expected
    population entry per executable clause node and grants no campaign evidence for
    a refused selection. Wrong-role, wrong-ordinal, other-node, absent-point-node,
    and renderer-mutant selections '
- id: FR-019-AC-3
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement
    settles `requires-bound` when a finite bound is available, rather than `unsupported`;
    with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`;
    and it never settles `supported`.
- id: FR-019-AC-9
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A backend kind added without a negotiation arm does not compile: the dispatch
    is an exhaustive `match` over the closed kind with no catch-all arm.'
- id: FR-019-AC-10
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Only an item settled `supported` routes.
- id: FR-019-AC-14
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: The process-provider arm's return type is `Disposition`, which has no terminal-value,
    verification-result or artifact member, so settling returns none of them. A change
    that returns one does not compile against that type.
- id: FR-019-AC-15
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin`
    projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288,
    ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor
    from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised
    pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and
    `Process` to `Process`, with no wildcard or origin inference from identity, manifest
    or provider bytes or a side map. Two registry descriptors identical except for
    origin produce CG descriptors identical e
- id: FR-019-AC-24
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at
    an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported`
    with a warning naming `value-validity` and emits no artifact or Kani outcome;
    one linked Kani candidate advertising (`value-validity`, `bounded`) with `finite_bound_available=true`
    settles the unbounded item `requires-bound` without substituting a bound or emitting
    an artifact; a separately requested bounded `ProofBound::Cardinality{maximum:
    8}` item with that same sole candidate settles `supported` under its own request
    index, while the un'
- id: FR-019-AC-34
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1
    through TP-4, TP-5, SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles
    through its explicit-state arm with the recorded form and model-or-protocol subject
    intact; an unadvertised form or HP-1 code claim does not settle `supported` through
    EN-1.
- id: FR-019-AC-35
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's
    kind and form, TP-1 through TP-4 route only to an EN-2 or EN-3 method ADR-018
    assigns to that form and subject; TP-4 does not route to EN-3, and a form absent
    from the candidate's advertised form set settles `unsupported` with no engine
    request.
- id: FR-019-AC-36
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item
    retains its safety and optional liveness halves on the concrete subject through
    EN-1, and the same whole item reaches an advertising SMT arm for RE-3 refutation;
    a separately requested RE-2 `operation-contract` item reaches the SMT arm for
    per-step safety checking, but its disposition cannot substitute for the whole-refinement
    item. An arm lacking the required TP-5 form settles the item `unsupported`.
- id: FR-019-AC-37
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising
    EN-1 arm; only HP-2 reaches an advertising SMT arm under its applicable EN-2 or
    EN-3 method. HP-4 and an unadvertised HP form settle unsupported before an engine,
    HP-5 stays on its possible-property path, and an HP-1 code claim is not settled
    as an EN-1 model claim.
- id: FR-019-AC-38
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with
    its recorded form; only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against
    an SMT-only candidate settles unsupported and reaches no SMT engine.
- id: FR-019-AC-39
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising
    that exact reduction, with the selection preserved in the routed request. Symmetry
    declarations and POR enter the obligation identity, so their reduced and unreduced
    requests for one claim have distinct obligation identities; a state constraint
    changes request identity but leaves obligation identity unchanged. An unadvertised
    selection or POR on a subject with no footprints settles `unsupported` without
    an artifact, and no arm drops or adds a reduction.
- id: FR-019-AC-40
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising
    the subject's resolved `tso` or `ra` model receives that exact model; a candidate
    lacking it settles `unsupported` with no artifact, and no arm weakens the resolved
    model. An `sc` subject is a separate strong-model control and does not take the
    weak-memory branch.
- id: FR-019-AC-41
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model
    and protocol subject classes are read separately; an arm that discharges only
    one class settles the other `unsupported`, while a supported protocol item retains
    its protocol transition-system subject identity through routing.
- id: FR-019-AC-47
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: QSL's registry snapshot holds each backend identity at most once (FR-290:258-265;
    FR-331:57), so no driver-built FR-331 manifest repeats an identity.
- id: FR-019-AC-48
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: context_only
  excerpt: RETIRED, replaced by FR-019-AC-54; its old path and expected/actual digest
    projection is obsolete and earns no implementation credit.
- id: FR-019-AC-54
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: context_only
  excerpt: 'A `Refused(refusal)` outcome from the Kani oracle-function preflight reader
    produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection
    })` for the whole request, copying the reader''s `code`, nonempty ordered typed
    `loci` including paths and source regions, `cause`, `locus`, `contract_version`,
    `document_pointer`, `extension_detail`, `protocol_detail` and `terminal_duplicate`
    without loss or reinterpretation. The obsolete `path`, `expected` and `actual`
    projection members are absent. No item settlement or artifact is returned. Distinct
    extension, proto'
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both preserve domain validity and the failure; a candidate that
    changes domain validity or the verdict is discarded.
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the original counterexample's.
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignments, each adapter''s backend identity, the same opaque
    obligation identity, declared domains, source/provenance and replay packet members;
    decode refusal carries only backend identity, obligation identity and typed cause.
    Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each
    successful adapter''s original backend and source identity in a complete packet.'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode
    evidence failure retaining its cause, with no envelope or replay. Before native
    replay, an out-of-domain assignment yields a distinct typed domain evidence failure;
    an incomplete or QSL-refused witness/packet yields a distinct typed QSL admission
    evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract
    failure, and none of the pre-replay failures invokes replay. Kani's exact decode
    and refusal partition remain governed by FR
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure becomes a new CG lineage revision referring to its
    exact admitted QSL envelope, its exact parent revision and the unchanged opaque
    obligation identity. After retention, the original QSL envelope compares equal
    to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal
    to their captured values. No lineage member is added to QSL''s envelope or packet.'
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that changes the native verdict each create no revision. After each
    rejection, every prior QSL envelope compares equal to its pre-proposal clone and
    every prior CG record's revision identity, parent, obligation and typed evidence
    fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript.
- id: FR-024-AC-49
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-21). Structural identity changes for a different
    frame node, owning model or assigned ordinal and remains equal for unchanged components
    across comments, blank lines, source-span, symbols, solver, options and state-field
    display changes. A change of frame grant that rekeys its producer node changes
    identity; if the node remains unchanged, full grant and field comparison detects
    a changed body. A changed kind or occurrence under held identity refuses before
    replay. Distinct postcondition claims of one operation retain their own complete
    occurrences even if they sha
- id: FR-024-AC-51
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-1079; replaces AC-24). `FrameReplayInputs` has no `obligation_identity`
    member. The frame request and envelope carry the same producer-issued typed identity
    from the selected frame claim; no caller stand-in, CG mint or hash reaches either.
    A changed grant that rekeys the producer frame node changes both identities; a
    grant/body mismatch under a held node is refused by the full record comparison.
    The state-clause path remains outside this criterion.
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: RETIRED, replaced by FR-024-AC-47; the former CG-minted O-09 digest scheme
    is obsolete and earns no implementation credit.
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: RETIRED, replaced by FR-024-AC-53; the former document-digest equality
    check is obsolete and earns no implementation credit.
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: RETIRED, replaced by FR-024-AC-48; the former four-member O-09 preimage
    and encoder are obsolete and earn no implementation credit.
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: RETIRED, replaced by FR-024-AC-49; the former kind/occurrence-inside-digest
    control is obsolete and earns no implementation credit.
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: RETIRED, replaced by FR-024-AC-51; the former frame identity minting rule
    is obsolete and earns no implementation credit.
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: context_only
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsified frame or postcondition harness draws a present model field
    whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint
    below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value
    outside that model range. QSL refuses the pre state and settlement is `Inconclusive`
    with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange`
    reason; it never reports `Verified` or a violation. Playback inside the model
    range is unaffected, a
- id: spec/evidence/matrix/tests.md:changed-row-10
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-1 through FR-004-AC-7 | TC-006 | 🚧 Planned (IR-1316):
    primitive controls are partial; AC-6 has no campaign-count/native-outcome input
    or retention |'
- id: spec/evidence/matrix/tests.md:changed-row-11
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-8 | TC-006 | 🚧 Planned (IR-1316): consuming coverage-obligation
    boundary is absent |'
- id: spec/evidence/matrix/tests.md:changed-row-15
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-10 | TC-006 | 🚧 Planned (IR-1316): no live campaign/profile/export
    chain; analyzers accept detached bytes |'
- id: spec/evidence/matrix/tests.md:changed-row-16
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-11 | TC-006 | 🚧 Planned (IR-1316): consuming coverage-obligation/proof-accounting
    evidence is absent |'
- id: spec/evidence/matrix/tests.md:changed-row-19
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-17 | Inspection | 🚧 Planned (IR-1316; IR-815): V2
    population scan reads package.graph directly outside core/ir |'
- id: spec/evidence/matrix/tests.md:changed-row-26
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-12 | TC-006 | 🚧 Planned (IR-1316): CG analyzer has
    no QSL typed /2 reader input or consumer |'
- id: spec/evidence/matrix/tests.md:changed-row-27
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-13 | TC-006 | 🚧 Planned (IR-1316): CG has no typed
    QSL disposition retention/separation at the analyzer |'
- id: spec/evidence/matrix/tests.md:changed-row-28
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-14 | TC-006 | 🚧 Planned (IR-1316): private live run
    context and bounded process admission are absent |'
- id: spec/evidence/matrix/tests.md:changed-row-29
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-15 | TC-006 | 🚧 Planned (IR-1316): no live context
    to exercise foreign-run association refusals |'
- id: spec/replay/matrix/tests.md:changed-row-9
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-024 | FR-024-AC-7, FR-024-AC-8, FR-024-AC-47 through FR-024-AC-53
    | TC-035 | 🚧 Planned (IR-1316): AC-7/8 have no production minimizer; AC-49/51
    lack full frame structural/grant mutation controls; other listed structural criteria
    keep their existing status |'
- id: spec/replay/matrix/tests.md:changed-row-17
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: '| FR-024 | FR-024-AC-36 through FR-024-AC-40 | TC-035 | 🚧 Planned (IR-1316;
    IR-335): common typed adapter intake, typed evidence failures and CG-owned immutable
    minimization lineage; no production minimizer, lineage record or cross-backend
    intake test is claimed. |'
- id: spec/strategy/matrix/tests.md:changed-row-10
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-7, FR-002-AC-8, FR-002-AC-10 through FR-002-AC-20,
    FR-002-AC-22, FR-002-AC-24 | TC-004 | 🚧 Planned (IR-1316; IR-815/IR-364): selected
    V2 harness and non-completed terminals are absent |'
- id: spec/strategy/matrix/tests.md:changed-row-11
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-9 | Inspection | 🚧 Planned (IR-1316; IR-815): harness
    generator still uses V1 typed bodies outside the V2 access path |'
- id: spec/strategy/matrix/tests.md:changed-row-12
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-21, FR-002-AC-23 | TC-004 | Retired; no implementation
    credit; replaced by AC-25 and AC-26 respectively |'
- id: spec/strategy/matrix/tests.md:changed-row-13
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-25, FR-002-AC-26 | TC-004 | 🚧 Planned (IR-1316; IR-815):
    V2 harness structural identity propagation and text refusals are absent |'
- id: spec/strategy/matrix/tests.md:changed-row-22
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1,
    FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6, FR-002-AC-7,
    FR-002-AC-8, FR-002-AC-10, FR-002-AC-11, FR-002-AC-12, FR-002-AC-13, FR-002-AC-14,
    FR-002-AC-15, FR-002-AC-16, FR-002-AC-17, FR-002-AC-18, FR-002-AC-19, FR-002-AC-20,
    FR-002-AC-22, FR-002-AC-24, FR-002-AC-25, FR-002-AC-26 | 🚧 Planned; AC-7 to AC-20,
    AC-22 and replacement AC-25/26 are Planned (IR-1316; IR-815), AC-16/24 are Planned
    (IR-1316; IR-364); selected V2 harness and non-completed terminals are absent
    |'
- id: bounded_only_kani_requires_a_bound_or_warns_without_one
  path: tests/it/capability_settlement.rs
  role: examined
  excerpt: '#[trace("TC-030", "FR-019-AC-3")]

    #[test]

    fn bounded_only_kani_requires_a_bound_or_warns_without_one() {'
bindings:
- test_id: bounded_only_kani_requires_a_bound_or_warns_without_one
  ac_id: FR-019-AC-3
  trace: correct
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the scoped change. No introduced defect found. This does not close the 50 Planned criteria, parent ticket, full strict coverage, or final merge gate.
