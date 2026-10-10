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
- Generated Rust/source-map bytes with entry probes and the independently derived implication census.
- LLVM coverage JSON export bytes, its producer tool and version, and native runtime campaign
  results identifying the generated source and source map actually used for that run.
- Separately, a QSL strict-reader typed clause-run or command-error variant for exact
  `native-run-result/2`, or its located wire refusal. This is QSL source-clause evaluation or
  command failure, not execution of the generated campaign; its producer implementation is
  pending QSL-520.
- A separate CG-owned producer/artifact binding receipt whose trusted authority and verification
  authenticate the association of that exact decoded result, producer execution, generated source
  and source map used by the campaign. Caller-declared identities, matching paths, QSL
  `package_id` and self-declared producer metadata alone are insufficient.

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
  analyzer shall verify the separate CG-owned receipt's trusted producer authority and its
  association of the exact decoded QSL result, producer execution, generated source and source
  map with the supplied inputs. For an absent, unauthenticated, incomplete or mismatched
  association, the analyzer shall yield structured non-success naming the failed association and
  retaining available input identities and the decoded QSL outcome separately. Equal paths or QSL
  `package_id` do not repair a mismatch. Such a result yields no campaign qualification, measured classification or coverage
  discharge; a located QSL wire refusal remains a wire refusal, not a binding failure.
- The analyzer shall keep QSL source-clause evaluation separate from generated Rust campaign
  execution. QSL `success`, `closed-scope`, or a decisive witness cannot establish that CG's
  generated artifact ran; QSL `violation`, refusal, undefined, incomplete, cancelled, unsupported
  or internal failure remains a typed adverse/non-success semantic fact and cannot be promoted to
  passed generated execution. A command-error envelope retains its stage/code/cause/details as an
  error, never as a Boolean clause result. Even an authenticated matching receipt cannot promote
  it to generated-campaign success. No QSL witness or LLVM probe alone grants IR FR-045
  completed-proof credit.
- Source-map requirement/revision identity shall equal the runtime campaign identity, and duplicate,
  missing, ambiguous, malformed, summary-only, or unsupported-version coverage input shall remain
  a structured non-success analysis outcome.
- The generated-campaign producer receipt shall bind the exact QSL result and producer execution
  to the run's requirement and revision and the generated source and source map actually used,
  independently of the QSL clause-run document's members.
  Successful analysis shall use that same source map and generated source, rather than an
  independently supplied map that merely names the same requirement. The report shall retain the
  LLVM export producer's tool and version identity alongside the generated-run
  identity. Missing or conflicting producer, result, run, revision, source, or map identity shall
  prevent campaign qualification, measured coverage classification and coverage discharge.
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
| FR-004-AC-10 | PLANNED. A CG-owned authenticated producer/artifact receipt binds the exact decoded QSL result and producer execution to the analyzed generated source and source map actually run, matches their requirement and revision to the campaign, and retains the LLVM producer tool and version; a changed or missing result/run/map/source association or producer identity yields structured non-success with no campaign qualification, measured coverage classification or coverage discharge. A QSL `native-run-result/2` document alone does not supply this receipt. | Test |
| FR-004-AC-11 | PLANNED. The consuming report retains vacuity and native outcome as evidence for IR FR-045; an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe observations alone produce no completed-proof credit or separate eligible denominator. | Test |
| FR-004-AC-12 | PLANNED. CG consumes QSL's typed strict-reader clause-run or command-error variant for exact `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope` or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness, contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding; no `/1` fallback or CG copy of the wire reader exists. | Test |
| FR-004-AC-13 | PLANNED. QSL `run_clause` success and a decisive witness remain source-clause facts and do not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A matching authenticated receipt does not upgrade any QSL disposition or itself grant campaign success, measured classification, proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional cause/message/details and unavailable basis as non-success, with no campaign qualification or measured classification even when the receipt matches. | Test |
| FR-004-AC-14 | PLANNED. A valid `/2` clause-run result with absent receipt, omitted association or only caller-declared paths, QSL `package_id` or self-declared producer metadata yields a structured absent/unauthenticated binding diagnostic while retaining the decoded outcome and available identities; no campaign qualification, measured classification or coverage discharge occurs. A fully authenticated receipt associating the exact result, producer execution, generated source and source map admits binding and retains the unchanged QSL outcome. | Test |
| FR-004-AC-15 | PLANNED. Independently replacing the result, producer execution, generated source or source map after authenticated binding refuses with the mismatched association identified, even when paths or QSL `package_id` match; the original decoded outcome remains distinct from the binding diagnostic. A valid typed command-error variant stays non-success even with a matching receipt, while an invalid `/2` or unsupported version refuses at QSL reading before receipt verification. | Test |

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

`native-run-result/2` consumption is also planned. QSL-520 must deliver the producer and strict
reader. QSL-688 chose CG ownership of the separate authenticated producer/artifact receipt and
its result/execution/source/map association; its trusted authority, representation and verifier
remain CG implementation work. QSL's `/2` document does not carry this receipt or any added
generated-source, map or coverage-producer member.

IR FR-045 owns source-derived proof eligibility, denominators, and completed-proof credit;
this analyzer supplies execution observations and the coverage-obligation decision only.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) and [FR-002](../../strategy/functional/FR-002-tristate-proptest.md).
