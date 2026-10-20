---
id: FR-004
title: "Produce vacuity and rejection evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-contract-ir/FR-045
    type: references
---
# FR-004: Produce vacuity and rejection evidence

## Description

When LLVM/cargo coverage export and runtime campaign counts are supplied, the generator shall combine
oracle evaluation, consequent execution, rejection, discard, and test outcome into a deterministic
per-requirement vacuity report without executing a coverage producer itself.

## Inputs

- The IR-owned bound executable clause population, including clause identities, typed expressions,
  clause kinds, execution anchors, and dependency/declaration context.
- Planned (IR-815): for the V2 analyzer, the executable clauses of an admitted `CheckedPackageV2`,
  read through the `core/ir/` layer of AD-004, in place of the V1 bound population. The source of
  each clause's execution anchor and of the requirement and revision identity is open (below).
- Generated Rust/source-map bytes with entry probes and the independently derived implication census.
- LLVM coverage JSON export bytes, its producer tool and version, and native runtime campaign
  results identifying the generated source and source map actually used for that run.
- Separately, a QSL strict-reader typed clause-run or command-error variant for exact
  `native-run-result/2`, or its located wire refusal. This is QSL source-clause evaluation or
  command failure, not execution of the generated campaign; its producer implementation is
  pending QSL-520.
- A live CG runner context that owns the original `native-run-result/2` wire bytes and the typed
  result returned by QSL's strict reader for those bytes, together with the execution record,
  generated source and source map used by the coverage producer, its LLVM export, and its reported
  tool/version. These are the inputs being analyzed, not copies in a provenance record. The
  context borrows one private invocation lifetime, has no registry or serialized run key, and cannot
  be supplied by a caller or restored from wire data. Caller-declared identities, matching paths,
  QSL `package_id`, and self-declared
  producer metadata are insufficient.

## Outputs

- A versioned analysis outcome containing trusted observations, per-clause classifications where
  measurable, structured diagnostics, and an explicit non-success state for
  adverse, unavailable, unsupported, malformed, or inconclusive analysis.
- Distinct retained facts for the QSL clause-run disposition and the generated Rust campaign's
  execution/coverage state; neither fact is synthesized from the other.

## Behavior

- The analyzer shall consume an LLVM coverage JSON export without implementing instrumentation,
  profile merging, or a coverage engine.
- The analyzer shall derive its expected clause population and implication counts from typed IR,
  require exact source-map/artifact population equality, and represent a valid empty executable
  population explicitly as no executable work, retaining informational references and no positive
  coverage result. Missing map rows shall never turn an implication into an implication-free clause.
- The analyzer shall classify a clause as `vacuous` only when its oracle-evaluation probe was observed,
  its typed expression contains at least one implication, and no expected consequent probe was observed.
- The analyzer shall classify a mapped clause as `unexecuted` when its oracle-evaluation region has
  no positive execution count, regardless of its test outcome or campaign counts.
- The analyzer shall classify a clause as `partially_exercised` when some, but not all, expected
  implication consequents were observed; only observation of every mapped consequent may produce
  `exercised` for an implication-bearing clause.
- A clause whose typed expression contains no implications shall be `exercised` only when its oracle-evaluation
  region is observed; vacuity shall not be inferred when there is no implication.
- These four measured classifications shall be mutually exclusive. Missing/unusable observation
  data shall produce a diagnostic with no classification, distinct from a measured zero count.
- Positive observation requires one count-bearing, non-gap LLVM active span to contain the entire
  mapped entry-token probe. Partial intersection, summary counts, and an unterminated final span
  cannot establish observation. Consequent observation with zero oracle-entry count is inconsistent.
- In the bound generated Boolean-only profile, each consequent shall have a count no greater
  than its owning oracle evaluation count: generated expressions contain no loop or user call
  that can enter the consequent repeatedly per invocation. A contradiction retains both observed
  counts and an inconsistency diagnostic but no clause classification. This stronger aggregate
  rule does not change the unbound `classify_clause` primitive's compatibility contract.
- The analyzer shall preserve native accepted, rejected, failed, and discarded counts and execution
  outcome independently. It shall verify run/requirement/revision bindings; a successful test outcome
  cannot coexist with failed postconditions, and positive oracle observation cannot coexist with
  zero recorded invocations when the run declares the generated campaign as its only execution source.
- The planned aggregate analyzer shall consume `native-run-result/2` only through QSL's strict
  reader. It shall retain the typed clause-run stage, category, truth where present, settlement
  basis and optional witness without parsing display text or making an absent component `null`,
  zero or a default. The same reader's typed command-error variant shall retain stage, code,
  optional cause, message, details and unavailable basis as non-success, with no witness or
  inferred clause truth. A located wire refusal, including an absent or non-`/2` format or mixed
  invalid variant shape, is not a decoded outcome. The analyzer shall not add a `/1` reader,
  schema copy, or version inference from payload members.
- Before qualifying a generated campaign or deriving any measured coverage classification, the
  analyzer shall bind within the same live CG runner invocation that owns and produces its inputs.
  QSL's strict reader shall decode the runner-owned original `/2` wire bytes once.
  The analyzer shall consume that returned typed value directly, without reserializing it or
  inventing bytes for a decoded value. The runner shall pass the same owned generated source/map
  buffers it gave the coverage producer, plus the execution record, LLVM export and tool/version
  obtained through that invocation's owned producer channel, directly into binding before returning
  an outcome.
  Each association shall borrow one private invocation lifetime; a foreign-run value cannot be
  attached to it from a public API. The private, non-cloneable, non-serializable run context shall
  have no public constructor and shall expire with that invocation. CG shall create no separate
  stored or wire receipt, issuance set, file/tool tracking record or digest. For an absent, foreign,
  expired, incomplete or mismatched association, the analyzer shall yield structured
  non-success naming the failed association and
  retaining available input identities and the decoded QSL outcome separately. Equal paths or QSL
  `package_id` do not repair a mismatch. Such a result yields no campaign qualification, measured
  classification or coverage discharge; a located QSL wire refusal remains a wire refusal, not a
  binding failure.
- When inputs cross a process boundary, the runner shall treat the wire envelope as untrusted raw
  input, never as qualification from an earlier run. The caller shall set named maximum encoded
  input bytes and maximum decoded working bytes. CG shall refuse an over-limit envelope before
  allocating it, charge decoding and parsing allocations against the decoded limit, and return a
  structured resource refusal naming the exceeded limit before coverage classification. Encoding
  expansion and framing count toward the encoded limit; decoded objects and any copied buffers
  count toward the working limit. No partial admission or inherited qualification is allowed.
- The analyzer shall keep QSL source-clause evaluation separate from generated Rust campaign
  execution. QSL `success`, `closed-scope`, or a decisive witness cannot establish that CG's
  generated artifact ran; QSL `violation`, refusal, undefined, incomplete, cancelled, unsupported
  or internal failure remains a typed adverse/non-success semantic fact and cannot be promoted to
  passed generated execution. A command-error envelope retains its stage/code/cause/details as an
  error, never as a Boolean clause result. Even a bound live run cannot promote
  it to generated-campaign success. No QSL witness or LLVM probe alone grants IR FR-045
  completed-proof credit.
- Source-map requirement/revision identity shall equal the runtime campaign identity, and duplicate,
  missing, ambiguous, malformed, summary-only, or unsupported-version coverage input shall remain
  a structured non-success analysis outcome.
- The in-run binding shall consume the exact QSL wire input through its typed strict-reader result,
  the producer execution, generated source and source map actually used, and exact LLVM export
  bytes carrying probe observations for that same producing run and its requirement and revision.
  A different export from another execution of the same source and map shall not inherit the first
  run's qualification, even when its declared tool/version matches. Successful analysis shall use
  the producer's source and map, rather than an independently supplied map that merely names the
  same requirement. The report shall retain the LLVM export producer's tool and version as normal
  run facts, without a second tracking record. Missing or conflicting producer, result, run,
  revision, source, map or LLVM export association shall prevent campaign qualification, measured
  coverage classification and coverage discharge. A report or input forwarded after the run ends
  grants no qualification; a later process must run and bind afresh.
- A non-success analysis outcome shall retain its diagnostics without claiming that invalid or
  absent inputs were analyzed successfully.
- Coverage filenames shall match source-map artifact paths only after stripping the caller-declared
  source root and applying lexical normalization that rejects parent traversal and backslash aliases.
- The export's `manifest_path` shall name the caller-declared source root's `Cargo.toml`.
- The default coverage obligation succeeds only for a nonempty, completely bound population whose
  clauses are all exercised and whose native execution completed successfully. Vacuous, unexecuted,
  and partially exercised results are adverse; unavailable or inconclusive execution cannot pass.
  Artifact serialization success shall never stand in for that coverage result.
- The report shall expose its per-clause observations and native outcome to the IR FR-045 accounting
  boundary as evidence about execution and vacuity. It shall neither define another eligible proof
  denominator nor label a successful native run or observed probe as a completed proof.
- Planned (IR-815): when the analyzer is given an admitted `CheckedPackageV2`, it shall derive its
  expected clause population and per-clause implication counts from that package's executable
  clause bodies.
- Planned (IR-815): the V2 analyzer shall open a `CheckedPackageV2` body only through the
  `core/ir/` layer of AD-004, and shall not name or read a `BoundPackage` or a `ClauseRef`.
- Planned (IR-815): if the implication count declared by a V2 clause's generated source map
  differs from the count derived from that clause's body, then the analyzer shall return a
  structured binding non-success and no classification for that clause.
- Planned (IR-815): the analyzer shall apply the classification and non-success rules above to a
  V2 clause population without change.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | An evaluated implication whose consequent is unobserved yields a vacuity finding even when every oracle return was true; the consuming coverage obligation denies success for that clause. | Test |
| FR-004-AC-2 | A clause whose oracle-evaluation region was not observed is unexecuted, not vacuous. | Test (TC-006) |
| FR-004-AC-3 | The four measured clause classifications form a total partition; an implication-bearing clause is exercised only when every expected consequent entry probe is observed, and implication-free clauses require observed evaluation. | Test (TC-006) |
| FR-004-AC-4 | Successful analysis verifies the bound population, source/map, and native execution bindings. | Test |
| FR-004-AC-5 | Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported, or missing observation inputs retain structured non-success outcomes without invented classifications. | Test (TC-006) |
| FR-004-AC-6 | Campaign counts and test outcome remain complete facts independent of coverage classification. | Test (TC-006) |
| FR-004-AC-7 | Removing an expected clause, consequent, evaluation probe, or all clauses prevents successful analysis; the expected census comes from bound typed IR. | Test (TC-006) |
| FR-004-AC-8 | Adverse coverage and non-success native execution cannot discharge the coverage obligation merely because a report serialized successfully. | Test |
| FR-004-AC-9 | Complete bound observations without a native campaign run are reported as observations only and never as a passed coverage result. Valid informational-only populations retain their references as no executable work. | Test |
| FR-004-AC-10 | PLANNED. CG binds coverage inside the same live campaign-runner invocation, using the original `/2` wire bytes and their QSL strict-reader typed result directly, the producer execution record, the same generated source/map buffers used by that producer, and the exact LLVM export and tool/version obtained through its owned producer channel. Every association borrows one private invocation lifetime, with no registry or serialized run key. No separate stored or wire receipt, tracking record or digest carries these associations. The context expires when the invocation returns; a forwarded report or copied input from another run cannot inherit qualification. Requirement and revision match the campaign once their V2 source is resolved; a missing or conflicting same-run association yields structured non-success with no campaign qualification, measured classification or discharge. A different run's export cannot be attached to the live context even for the same source/map and tool/version. | Test |
| FR-004-AC-11 | PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045; an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe observations alone produce no completed-proof credit or separate eligible denominator. | Test |
| FR-004-AC-12 | PLANNED. CG consumes QSL's typed strict-reader clause-run or command-error variant for exact `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope` or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness, contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding; no `/1` fallback or CG copy of the wire reader exists. | Test |
| FR-004-AC-13 | PLANNED. QSL `run_clause` success and a decisive witness remain source-clause facts and do not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A bound live run does not upgrade any QSL disposition or itself grant campaign success, measured classification, proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional cause/message/details and unavailable basis as non-success, with no campaign qualification or measured classification even when the generated run is bound. | Test |
| FR-004-AC-14 | PLANNED. Only CG's campaign runner constructs the private, non-cloneable, non-serializable live run context; callers cannot construct or restore it from a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent, foreign or expired context, or an omitted association yields structured binding non-success retaining the typed QSL outcome and available identities, with no qualification, classification or discharge. A complete same-invocation context admits binding without changing the QSL outcome. The caller supplies maximum encoded-input and decoded-working-byte limits; CG refuses an over-limit wire envelope before allocation and an over-limit decode before further growth, naming the exceeded limit and admitting no partial context. | Test |
| FR-004-AC-15 | PLANNED. A controlled attempt to replace the strict-reader typed result, execution record, generated source, source map, LLVM export, or LLVM tool/version with a foreign-run value cannot construct a valid same-invocation context, even when paths, QSL `package_id`, source/map and tool/version declarations match. A malformed association observable inside one run yields structured non-success naming that association before classification; the original decoded outcome remains distinct from the binding diagnostic. An old context or serialized report cannot be replayed into a later invocation, and a later process must bind a fresh run. A valid typed command-error variant stays non-success with a bound run; an invalid `/2` or unsupported version refuses at QSL reading before binding. | Test |
| FR-004-AC-16 | PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies. | Test (TC-006) |
| FR-004-AC-17 | PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which today's `analyze_bound_coverage` names. | Inspection |
| FR-004-AC-18 | PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free. | Test (TC-006) |
| FR-004-AC-19 | PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation was observed, that has at least one implication and for which no expected consequent probe was observed is `vacuous`. | Test (TC-006) |
| FR-004-AC-20 | PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation region has no positive execution count is `unexecuted`. | Test (TC-006) |
| FR-004-AC-21 | PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause for which some but not all expected consequent probes were observed is `partially_exercised`. | Test (TC-006) |
| FR-004-AC-22 | PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause with every expected consequent probe observed, and an implication-free clause with its evaluation region observed, is `exercised`. | Test (TC-006) |
| FR-004-AC-23 | PLANNED (IR-815). Over a V2 clause population, removing one clause, one consequent probe, one evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis. | Test (TC-006) |
| FR-004-AC-24 | PLANNED (IR-815). Over a V2 clause population, each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification. | Test (TC-006) |

## Implementation boundary

The implementation provides source probes, bounded LLVM primitives, and complete bound
observations through `analyze_bound_coverage`. Its strict domain schema is
`codegen.bound-coverage-observations/v1`.
The complete immutable generated artifact population is checked against public typed IR
before any measured classification. Missing measurements retain null counts and diagnostics;
global binding failure emits no clause observations, with `population: not_emitted`.
Serialization is bounded to 16 MiB. Resource refusal may omit a population, explicitly
diagnosed, rather than allocate fabricated zero observations. Full ClauseRefs on every
ordered clause retain per-requirement membership without duplicating or summing campaign counts.
Native campaign-run binding is planned; FR-004/TC-006 remain planned until it and a consuming
obligation exist.

The V2 population input is planned (IR-815); `analyze_bound_coverage` reads the V1 `BoundPackage`
until a V2 analyzer replaces it, and the replacement deletes it. The V2 criteria FR-004-AC-16 to
FR-004-AC-24 are vacuity and rejection evidence only: they add no proof denominator or credit.

`native-run-result/2` consumption is also planned. QSL-520 must deliver the producer and strict
reader. QSL-688 allocated result/execution/source/map/exact-LLVM-export association to CG.
Within CG's no-tracking-record rule, the planned association is performed inside the live
campaign-runner invocation over its owned inputs; no separate persisted or wire receipt is
specified. A forwarded report is observational output and cannot qualify a later run. If an
externally verifiable cross-process receipt is required, its authority remains a separate design
gate rather than a capability claimed here. The V2 requirement/revision source remains FR-011
OQ-2. QSL's `/2` document gains no generated-source, map or coverage-producer member.

IR FR-045 owns source-derived proof eligibility, denominators, and completed-proof credit;
this analyzer supplies execution observations and the coverage-obligation decision only.

## Open Questions

- OQ-1 (anchor and identity): the V2 source of a clause's execution anchor and of the requirement
  and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the V2 entry
  shape follows the open interface-001 direction. FR-004-AC-16 to AC-24 do not depend on the
  answer and none invents a source. The V2 analyzer must also meet FR-004-AC-10, AC-11 and AC-14,
  which bind the run's requirement and revision, so deleting the V1 `analyze_bound_coverage` waits
  on the FR-011 OQ-2 identity answer as well as on these criteria.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) and [FR-002](../../strategy/functional/FR-002-tristate-proptest.md).
