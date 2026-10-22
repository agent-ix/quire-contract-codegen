---
id: SR-15393
title: IR-850 public semantic-value source code-review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-850; exact a3893118..fd44d097; src/lib.rs and src/core/profile.rs"
review_set: subset
---

## Summary

Ticket IR-850; IR-17 is related context only. Independent source-only review of exactly a3893118f8563518482a2f418a5c26fe157a4c7a..fd44d097ad6dd43e5a4cc81276b77d411e9b85bd on origin/ir850-type-unification-ir17-codex2, before a PR exists. The exact delta is two Rust files, 14 insertions/1 deletion: a crate-root public re-export of an existing source constant and one focused profile regression. This is not PR513 or its Round6 acceptance. No product/branch/worktree edits, integration, push, lock update, resolver, Cargo/compiler/test/native execution, Kani/backend or namespaces were performed by the reviewer. The coder's local dirty qualification lock/worktree was not touched. An exact git archive supplies read-only trace analysis and validation context.

Source review PASS with no finding. This permits source handoff only; integration/mergeability remains conditional on the actual required controls and the owner-published supported cohort. No uncommitted Runtimea60 qualification lock, RT exception, type bridge, dependency-source workaround or published-cohort acceptance is approved. Five local exact controls pass; the sixth real LLVM control fails because its native fixture still imports deleted Meter/Integer/Outcome/ScalarLimits through Runtime exact. That unchanged-base fixture failure is already owned by the lead/coder, is outside this tiny export diff and is not invented as a finding against this export. Existing vacuity assertions must remain. There is no six-control PASS, aggregate PASS, canonical proof/receipt or final PR513 acceptance.

Inspected red/green evidence: owner-source-red.exit=101 and log shows E0425 for missing crate::SEMANTIC_VALUE_DEPENDENCY_SOURCE in the new test, before executing tests. owner-source-green.exit=0 and raw log names the exact new profile regression, one PASS. They used the authorized local combined Runtimea60 qualification cohort, not the approved published lock; no Cargo file belongs to this delta. six-control-results.json now has six completed entries, five exit0 and LLVM exit101; each raw log names the requested outer test and its actual result. Earlier partial two-test state is not mistaken for completion. These are author-execution observations only. Final correction/requalification remains parent/coder owned.

The constant already exists in core/profile and is the one manifest authority for the semantic-value Git URL and branch main. pub use exposes that exact binding; it does not create a second literal or owner implementation. The real generated-v2-clause-corpus integration fixture already accesses quire_contract_codegen::SEMANTIC_VALUE_DEPENDENCY_SOURCE externally, so its external compilation also checks public visibility; the new crate-root unit regression itself discriminates removal of the binding and exact manifest-line agreement. It does not independently validate Git availability or compiled owner ABI, and no such claim is made. The manifest authority, public source spellings and generated callers are unchanged.

Checked ownership aliases remain rt=quire_contract_runtime::exact, exact=quire_exact and sv=quire_semantic_value::declaration in function output; scalar aliases remain as previously authored. No function/scalar generator, manifest construction, dependency graph, lock, Kani observer/root independent-resolution path, proof profile, workflow or gate changes exist in this diff. No fabricated identity/schema, owner API copy, unit-table reconstruction, mock, unsafe, panic path, new data bound or dependency cycle is added. Existing source values/domain/charge/refusal behavior is unaffected. Code/Rust gates were held by the explicit source-only brief; no false pass is substituted for them. Broader baseline issues and the unrelated LLVM fixture are not silently remediated or waived here.

AGENTS.md and CLAUDE.md were read from the candidate; no applicable AssuranceProfile exists in its spec tree. No profile-selected context is missing and no assurance exception is introduced. Historical IR17 reviews and baseline inventory are context only. No CG spec/plan changes occur, so spec-review and spec sub-analyses are not applicable to this delta. Optional semantic gap analysis was not opted in. The user explicitly limited the gap method to this incremental AC interface check; this review does not claim a new repository-wide gap audit or baseline completion. Plan completion: not assessed.

## Verdict

PASS source review; no incremental finding. Integration acceptance remains pending actual qualification and the supported published owner route.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The Rust lane checked real public re-export visibility, existing integration consumer use, constant identity, idioms, trace attribute, oracle strength, no generated implementation change and unchanged CI surface. No extra Rust artifact is allocated.

## Examined scope

```yaml
clean: true
scope:
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: SEMANTIC_VALUE_DEPENDENCY_SOURCE
  path: src/lib.rs
  role: examined
  excerpt: "pub use crate::core::profile::{\n    EXACT_DEPENDENCY_SOURCE, RUNTIME_DEPENDENCY_SOURCE,\
    \ SEMANTIC_VALUE_DEPENDENCY_SOURCE,\n};"
- id: generated_manifest_exposes_semantic_value_owner_source
  path: src/core/profile.rs
  role: examined
  excerpt: "#[trace(\"TC-024\", \"FR-014-AC-8\")]\n    #[test]\n    fn generated_manifest_exposes_semantic_value_owner_source()\
    \ {"
findings: []
base: a3893118f8563518482a2f418a5c26fe157a4c7a
head: fd44d097ad6dd43e5a4cc81276b77d411e9b85bd
ticket: IR-850
related_ticket: IR-17
pr: null
qualification: source-only PASS; local six controls five PASS/one LLVM fixture FAIL;
  owner-published cohort/integration pending
```

## Final independent source readback

Final independent source readback at 9b01b443a86f3b12ae2960109a29ad56e033ddb8, exact base a3893118f8563518482a2f418a5c26fe157a4c7a. Scope is the full three-file IR850 handoff, not PR513 Round6: src/lib.rs existing constant re-export; src/core/profile.rs new FR014AC8 traced regression; tests/it/v2_llvm_coverage.rs native harness import. The first fd44 review above and its 5PASS/1FAIL state remain historical, with original comments/artifacts unmodified. Latest verdict: PASS for this narrow source handoff; no new finding at any severity. Source acceptance does not authorize or accept a lock integration, published cohort, whole PR513 feature/gate, native Quantity oracle, full gate or Kani/backend proof. Parent owns integration and final combined acceptance; no new permission is needed for its already authorized work.

The only fd44..9b01 source change is the native test string import from quire_contract_runtime::exact as rt to quire_exact as rt. Independent whole-file comparison proves replacing that one import yields the exact final file. Source literals, integer value1, every ScalarLimits field/value1000, expected Completed(true), generated clause source, real llvm-cov invocation, source-map/report path and evaluation1/consequent0/classification=vacuous assertions are byte-identical. The native fixture now names the actual authoritative kernel Meter/Integer/Outcome/ScalarLimits, already supplied by its manifest. It no longer assumes Runtime re-exports those kernel carriers. This does not alter function generator aliases (rt=Runtime exact, exact=kernel, sv=QSV declaration) or scalar production aliases; no generated caller or dependency-source implementation workaround was added.

The crate-root export remains a real public re-export of the single existing profile authority, and the FR014AC8/TC024 unit test remains unchanged from fd44. Existing external corpus consumer names quire_contract_codegen::SEMANTIC_VALUE_DEPENDENCY_SOURCE, checking actual external visibility when compiled. The manifest-line unit regression catches deletion of the crate-root binding with E0425 and validates authority consistency; it is not misrepresented as a remote Git/API availability or all-owner execution oracle. Test/source/code ownership remains direct Exact/QSV plus only actual Runtime-owned residues. No constant value duplicate, private API copy, compatibility layer, RT exception, type bridge, fabricated nominal identity or graph-edge/lock modification appears in the delta.

Incremental AC review: the export chain remains owned/tagged by FR014AC8. LLVM test tags FR004AC16/19/24 are unchanged and correspond to real coverage/vacuity observations. The one import change repairs compilation rather than relaxing the outcome/coverage oracle. Exact static row extraction is retained from the fd44 snapshot: none of these trace blocks or requirement texts changed in 9b01. There are no new criteria, untagged additions or orphaned implementation in this three-file scope. Full repository gap inventory was deliberately not re-audited under the explicit incremental-only brief, and no baseline completion or broader repository PASS is asserted. Plan completion: not assessed.

Actual final local evidence: six-control-final-results.json now has six exit0 entries and every referenced raw log names exactly its requested outer test with one PASS. First five controls passed at fd44; their source and cohort are unchanged by the isolated sixth-test import, so they were not needlessly rerun. Sixth control initially failed compilation through missing Runtime Meter/Integer/Outcome/ScalarLimits (control-6.log, exit101); the fixed 9b01 command passes exactly the LLVM test (control-6-fixed.log, one PASS). Original red and original results remain retained. These six runs and the focused public-export compiler red/green used the explicitly authorized LOCAL combined Runtimea60 validation lock, preserved outside the worktree and never committed/pushed. Neither that temporary lock nor a published supported-cohort acceptance is part of this handoff. No reviewer Cargo/compiler/test/native/LLVM/Kani execution or worktree/branch/source/lock mutation occurred.

The routed Quantity control inspects source/disposition and does not compile or execute a Quantity oracle. Thus six PASS is not a positive Quantity execution, conversion carry, UnitTable, native Union or proof acceptance claim. Those separate owner workstreams remain untouched. Additional broad/aggregate/Kani gates did not run. The parent will retain the latest official artifacts and manage combined integration/cohort acceptance separately. AGENTS/CLAUDE and no applicable AssuranceProfile remain as recorded above; no new CG spec text or profile exception is introduced. No spec-analysis artifact is required for this source-only increment.

```yaml
dispositions: []
clean: true
new_findings: []
scope:
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: SEMANTIC_VALUE_DEPENDENCY_SOURCE
  path: src/lib.rs
  role: examined
  excerpt: "pub use crate::core::profile::{\n    EXACT_DEPENDENCY_SOURCE, RUNTIME_DEPENDENCY_SOURCE,\
    \ SEMANTIC_VALUE_DEPENDENCY_SOURCE,\n};"
- id: generated_manifest_exposes_semantic_value_owner_source
  path: src/core/profile.rs
  role: examined
  excerpt: "#[trace(\"TC-024\", \"FR-014-AC-8\")]\n    #[test]\n    fn generated_manifest_exposes_semantic_value_owner_source()\
    \ {"
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2`
    clause bodies.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected consequent
    probe was observed is `vacuous`.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
base: a3893118f8563518482a2f418a5c26fe157a4c7a
head: 9b01b443a86f3b12ae2960109a29ad56e033ddb8
ticket: IR-850
related_ticket: IR-17
pr: null
qualification: narrow source handoff PASS; all six exact local combined-cohort controls
  PASS; no published-cohort/whole PR513/final-gate acceptance
```
