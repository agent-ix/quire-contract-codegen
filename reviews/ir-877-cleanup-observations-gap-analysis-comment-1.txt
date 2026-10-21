<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 1/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-001 | high | coverage | Inherited FR-002-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-002 | high | coverage | Inherited FR-002-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-003 | high | coverage | Inherited FR-002-AC-10 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-004 | high | coverage | Inherited FR-002-AC-11 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-005 | high | coverage | Inherited FR-002-AC-12 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-006 | high | coverage | Inherited FR-002-AC-13 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-007 | high | coverage | Inherited FR-002-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-008 | high | coverage | Inherited FR-002-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-009 | high | coverage | Inherited FR-002-AC-16 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-010 | high | coverage | Inherited FR-002-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-011 | high | coverage | Inherited FR-002-AC-18 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-012 | high | coverage | Inherited FR-004-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-013 | high | coverage | Inherited FR-004-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-014 | high | coverage | Inherited FR-004-AC-10 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-015 | high | coverage | Inherited FR-004-AC-11 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-016 | high | coverage | Inherited FR-004-AC-12 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-017 | high | coverage | Inherited FR-004-AC-13 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-018 | high | coverage | Inherited FR-004-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-002-AC-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Rejected inputs remain distinct from passing cases in generated reports.
- id: FR-002-AC-2
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Boundary strategies exercise immediately inside and outside supported constraints.
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002-AC-4
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Shrinking preserves generated constraints or records a residual rejection.
- id: FR-002-AC-5
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: The generated campaign runner owns execution, explicit-discard accounting, and conclusion,
    returns a typed terminal result, and rejects campaigns below either configured coverage floor or above
    the explicit-discard ceiling; exhausted searches retain their reason and any missed floor without
    becoming a completed floor failure.
- id: FR-002-AC-6
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected
    expectations; enum cases execute an all-admitted membership expectation.
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`,
    the generated harness embeds each selected clause's FR-014 V2 clause oracle.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition oracle, subject
    and postcondition oracle record that the snapshot precedes precondition evaluation, which precedes
    subject invocation, which precedes postcondition evaluation, for every case.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens a `CheckedPackageV2`
    body only through the `core/ir/` layer of AD-004, and names no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression`
    or `oracle::boolean_v1` item, each of which today's V1 harness names or uses.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or
    the same for both selections yields a typed, located diagnostic naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape yields a structured
    unsupported diagnostic and no artifact, and the generator emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause
    identities and campaign policy is byte-identical. The requirement and revision identity that the generated
    header, `RequirementId` and symbols carry is not an input here and waits on OQ-1.
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true runs the subject once
    and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes false is a rejected
    precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes true and the
    postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition oracle returns
    an outcome that does not complete (stopped or undefined) is reported as a non-completed case retaining
    the oracle's cause, is not counted as a pass.
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that
    does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that
    does not complete does not evaluate the postcondition oracle.
- id: FR-004-AC-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: An evaluated implication whose consequent is unobserved yields a vacuity finding even when
    every oracle return was true; the consuming coverage obligation denies success for that clause.
- id: FR-004-AC-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: A clause whose oracle-evaluation region was not observed is unexecuted, not vacuous.
- id: FR-004-AC-3
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: The four measured clause classifications form a total partition; an implication-bearing clause
    is exercised only when every expected consequent entry probe is observed, and implication-free clauses
    require observed evaluation.
- id: FR-004-AC-4
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Successful analysis verifies the bound population, source/map, and native execution bindings.
- id: FR-004-AC-5
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported, or missing observation
    inputs retain structured non-success outcomes without invented classifications.
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage classification.
- id: FR-004-AC-7
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Removing an expected clause, consequent, evaluation probe, or all clauses prevents successful
    analysis; the expected census comes from bound typed IR.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Adverse coverage and non-success native execution cannot discharge the coverage obligation
    merely because a report serialized successfully.
- id: FR-004-AC-9
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Complete bound observations without a native campaign run are reported as observations only
    and never as a passed coverage result. Valid informational-only populations retain their references
    as no executable work.
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. CG binds coverage inside the same live campaign-runner invocation, using the original
    `/2` wire bytes and their QSL strict-reader typed result directly, the producer execution record,
    and the same generated source/map buffers used by that campaign. The runner starts the campaign with
    fresh private profile output, performs any required LLVM merge over only that output, starts LLVM
    export over only that profile, and captures exact export bytes and tool/version from that LLVM child's
    direct output stream while retaining the linked child handles. Binding verifies this live campaign/profil
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045;
    an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe
    observations alone produce no completed-proof credit or separate eligible denominator.
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. CG consumes QSL''s typed strict-reader clause-run or command-error variant for exact
    `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope`
    or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness,
    contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding;
    no `/1` fallback or CG copy of the wire reader exists.'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause facts and do
    not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete,
    cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A bound
    live run does not upgrade any QSL disposition or itself grant campaign success, measured classification,
    proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional
    cause/message/details and unavailable basis as non-success, with no campaign qualification o
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable, non-serializable
    live run context; callers cannot construct or restore it from a `/2` document, paths, QSL `package_id`,
    metadata, or a forwarded report. Absent, foreign or expired context, or an omitted association yields
    structured binding non-success retaining the typed QSL outcome and available identities, with no qualification,
    classification or discharge. A complete same-invocation context admits binding without changing the
    QSL outcome. The caller supplies maximum encoded-input and decoded-working-byte limits; CG '
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result, execution record,
    generated source, source map, LLVM export stream or LLVM tool/version with a foreign-run value cannot
    construct a valid same-invocation context, even when paths, QSL `package_id`, source/map and tool/version
    declarations match. In particular, foreign export bytes offered through a detached buffer or a different
    producer stream/child handle refuse as `producer_run_mismatch` before classification because neither
    is the current runner-owned campaign/profile/export chain. A malformed association observable i
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer
    equal those independently derived from the admitted `CheckedPackageV2` clause bodies.
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2` body only
    through the `core/ir/` layer of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which
    today's `analyze_bound_coverage` names.
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal
    to the count derived from its body yields a structured binding non-success naming the clause and no
    classification, and is never treated as implication-free.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation was observed,
    that has at least one implication and for which no expected consequent probe was observed is `vacuous`.
- id: FR-004-AC-20
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation region
    has no positive execution count is `unexecuted`.
- id: FR-004-AC-21
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause for which some
    but not all expected consequent probes were observed is `partially_exercised`.
- id: FR-004-AC-22
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause with every expected
    consequent probe observed, and an implication-free clause with its evaluation region observed, is
    `exercised`.
- id: FR-004-AC-23
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, removing one clause, one consequent probe, one
    evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class of FR-004-AC-5 yields
    a non-success outcome with no invented classification.
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer region.
- id: FR-005-AC-2
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Supported platforms reproduce byte-identical files.
- id: FR-005-AC-3
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007
    Differential Disposition Register holds exactly one row naming the case, the backends, and either
    a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing
    case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails
    the check.
- id: FR-005-AC-4
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference.
- id: FR-005-AC-5
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: A bundle naming more than 4096 artifacts, an artifact whose contents exceed 16,777,216 bytes,
    or a complete artifact byte total exceeding 134,217,728 bytes is refused at construction as a bounded-resource
    `InvalidBundle` diagnostic before any staging or destination I/O begins, so no partial bundle is ever
    published.
- id: FR-005-AC-6
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When an artifact path is absolute, or contains a `..` segment, a `.` segment, an empty segment,
    a trailing separator, a backslash or a control character, or is empty, bundle construction is refused
    with `UnsafeArtifactPath` naming the offending `bundle.artifacts[i].path`, before any destination
    or staging I/O begins.
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after replacement began
    and the prior destination is restored, the publisher reports destination state `unchanged`, the destination
    holds its prior bytes or remains absent, an adjacent developer-owned file is byte-identical, and no
    staging or backup sibling remains.
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle has replaced the
    destination, the publisher reports destination state `published`, the destination holds every artifact
    of the new bundle, and the backup sibling remains as visible residue.
findings:
- fnd: FND-001
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-7
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '91'
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`,
    the generated harness embeds each selected clause's FR-014 V2 clause oracle.
  finding: 'Inherited FR-002-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-002
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-8
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '92'
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition oracle, subject
    and postcondition oracle record that the snapshot precedes precondition evaluation, which precedes
    subject invocation, which precedes postcondition evaluation, for every case.
  finding: 'Inherited FR-002-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-003
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-10
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '94'
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or
    the same for both selections yields a typed, located diagnostic naming the selection and no artifact.
  finding: 'Inherited FR-002-AC-10 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-004
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-11
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '95'
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape yields a structured
    unsupported diagnostic and no artifact, and the generator emits no filtering strategy for it.
  finding: 'Inherited FR-002-AC-11 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-005
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-12
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '96'
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause
    identities and campaign policy is byte-identical. The requirement and revision identity that the generated
    header, `RequirementId` and symbols carry is not an input here and waits on OQ-1.
  finding: 'Inherited FR-002-AC-12 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-006
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-13
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '97'
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true runs the subject once
    and is a pass.
  finding: 'Inherited FR-002-AC-13 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-007
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-14
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '98'
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes false is a rejected
    precondition and the subject is not invoked.
  finding: 'Inherited FR-002-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-008
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-15
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '99'
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes true and the
    postcondition oracle completes false is a failed postcondition.
  finding: 'Inherited FR-002-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-009
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-16
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '100'
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition oracle returns
    an outcome that does not complete (stopped or undefined) is reported as a non-completed case retaining
    the oracle's cause, is not counted as a pass.
  finding: 'Inherited FR-002-AC-16 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-010
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-17
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '101'
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that
    does not complete does not invoke the subject.
  finding: 'Inherited FR-002-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-011
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-002-AC-18
  related_ids: []
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  lines: '102'
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns an outcome that
    does not complete does not evaluate the postcondition oracle.
  finding: 'Inherited FR-002-AC-18 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-012
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-6
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '186'
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage classification.
  finding: 'Inherited FR-004-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-013
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-8
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '188'
  excerpt: Adverse coverage and non-success native execution cannot discharge the coverage obligation
    merely because a report serialized successfully.
  finding: 'Inherited FR-004-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-014
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-10
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '190'
  excerpt: PLANNED. CG binds coverage inside the same live campaign-runner invocation, using the original
    `/2` wire bytes and their QSL strict-reader typed result directly, the producer execution record,
    and the same generated source/map buffers used by that campaign. The runner starts the campaign with
    fresh private profile output, performs any required LLVM merge over only that output, starts LLVM
    export over only that profile, and captures exact export bytes and tool/version from that LLVM child's
    direct output stream while retaining the linked child handles. Binding verifies this live campaign/profil
  finding: 'Inherited FR-004-AC-10 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-015
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-11
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '191'
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045;
    an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe
    observations alone produce no completed-proof credit or separate eligible denominator.
  finding: 'Inherited FR-004-AC-11 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-016
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-12
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '192'
  excerpt: 'PLANNED. CG consumes QSL''s typed strict-reader clause-run or command-error variant for exact
    `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope`
    or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness,
    contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding;
    no `/1` fallback or CG copy of the wire reader exists.'
  finding: 'Inherited FR-004-AC-12 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-017
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-13
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '193'
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause facts and do
    not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete,
    cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A bound
    live run does not upgrade any QSL disposition or itself grant campaign success, measured classification,
    proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional
    cause/message/details and unavailable basis as non-success, with no campaign qualification o
  finding: 'Inherited FR-004-AC-13 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-018
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-004-AC-14
  related_ids: []
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  lines: '194'
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable, non-serializable
    live run context; callers cannot construct or restore it from a `/2` document, paths, QSL `package_id`,
    metadata, or a forwarded report. Absent, foreign or expired context, or an omitted association yields
    structured binding non-success retaining the typed QSL outcome and available identities, with no qualification,
    classification or discharge. A complete same-invocation context admits binding without changing the
    QSL outcome. The caller supplies maximum encoded-input and decoded-working-byte limits; CG '
  finding: 'Inherited FR-004-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings:
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-54
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-55
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-040-AC-4
  trace: correct
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 1
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++