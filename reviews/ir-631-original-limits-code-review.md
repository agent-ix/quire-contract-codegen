---
id: SR-14220
title: IR631 PR481 original renderer limits code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-631 PR481; complete delta plus owning source
  and cfg(test) include graph
review_set: subset
---

## Summary

Original renderer limits only. The same typed ScalarLimits drives production meter emission and private retention; exact current all-ten u64::MAX policy, source/record bytes, namespace, proposition, bounds and proof-content preimage remain unchanged. Mutation of retained public identity/source/record refuses. No budget echo or unavailable-context default. Full FR-032-AC-3 remains partial: no admitted QSL plan, original checked-source/call-site/sent-wire authentication, Falsified native observation, driver call or converter/settlement. IR-940 is Backlog and IR-631 In Progress. Existing scalar-native receipt remains Verified-only. Author logs at frozen head report 70 focused passes, fmt/all-target lint PASS, and one full Kani37/37 PASS; reviewer ran no heavy commands. Final root make ci plus separate full Kani on latest exact artifact custody remain UNRUN.

## Verdict

CONDITIONAL (inherited architecture debt); no current-delta defect

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The actual cfg(test) include graph violates L-2 through self-facade imports and nonroot re-exports; native_twin also imports replay into kani::generate. | spec/assurance/AD-004-cg-crate-layout.md:842 | implementation-bug-despite-evidence |

## Coverage

L-2 inspection recursively resolved 14 real #[path] edges and 106 library-source/included files. Its four nonroot re-export lines and twenty self-facade references are inherited; role_parity_tests includes native_twin which imports FrameReplay/ReplayInputs from replay through the library self alias. tests/it/layout.rs scans src only and does not follow these external files, so its source-only passes do not establish full L-2. No new delta architecture violation, copied owner types, tracking digest, test stub or gate weakening found. No applicable AssuranceProfile exists. Finite meter literals emit a decimal u64-context expression; current policy is all MAX. Generated-only no-meter controls use generated() without limits and retain typed Unavailable semantics; they are not a new production meter context.

## Full ordered scope and bindings

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-631
pr: quire-contract-codegen#481
pr_url: https://github.com/agent-ix/quire-contract-codegen/pull/481
source_head: 22cf3a5e174c25eb6dacf5a01945e0a38691d5a7
base_head: 47c4a6adaaea4faa7bfe82b5eda6581aaff53897
model: gpt-6.1-sol
runtime: Codex 0.163.0
run: a16f2703-674d-4f58-917e-5d0348341741
quoin: 0.28.4
quire: 0.36.2 (engine 0.50.2)
module: spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502
visibility: public
no_heavy_commands: true
id: SR-14220
method: code-review
clean: false
scope:
- id: file:src/core/artifact.rs
  path: src/core/artifact.rs
  lines: '1'
  role: examined
  excerpt: //! Generated files and the validated bundle every generator builds (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/canonical.rs
  path: src/core/canonical.rs
  lines: '1'
  role: examined
  excerpt: //! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/contract_package.rs
  path: src/core/contract_package.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/diagnostic.rs
  path: src/core/diagnostic.rs
  lines: '1'
  role: examined
  excerpt: //! The generation diagnostic vocabulary shared by every generator (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/identity.rs
  path: src/core/identity.rs
  lines: '1'
  role: examined
  excerpt: //! Identity newtypes of a generated Kani harness (AD-004 `core/identity`, L-10).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/clause.rs
  path: src/core/ir/clause.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted checked-package state-clause body access.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/mod.rs
  path: src/core/ir/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Typed access to admitted V2 clause conditions and their model-owned fields.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/mod.rs
  path: src/core/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/naming.rs
  path: src/core/naming.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded readable name components, unique names and symbol derivation shared by every
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/profile.rs
  path: src/core/profile.rs
  lines: '1'
  role: examined
  excerpt: '//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/provenance.rs
  path: src/core/provenance.rs
  lines: '1'
  role: examined
  excerpt: //! Headers for generated Rust, built only from identities carried by generation inputs.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/source_map.rs
  path: src/core/source_map.rs
  lines: '1'
  role: examined
  excerpt: //! Source-map records that trace generated source back to a requirement clause (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/test_support.rs
  path: src/core/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Scratch directories shared by unit tests across subsystems.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/bound_coverage.rs
  path: src/evidence/bound_coverage.rs
  lines: '1'
  role: examined
  excerpt: //! Complete bound observations. No execution, authentication, or assurance verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/mod.rs
  path: src/evidence/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The evidence subsystem: bounded coverage observations (AD-004, FR-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/vacuity.rs
  path: src/evidence/vacuity.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded LLVM observation primitives, not a campaign or coverage-obligation verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/abi.rs
  path: src/kani/abi.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/census.rs
  path: src/kani/census.rs
  lines: '1'
  role: examined
  excerpt: //! The proof-dependency census types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/classify.rs
  path: src/kani/classify.rs
  lines: '1'
  role: examined
  excerpt: //! Classification of one Kani run from the report Kani exported for it (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/census_validation.rs
  path: src/kani/generate/census_validation.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: census validation and the V1 bundle''s error type it returns.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/clause.rs
  path: src/kani/generate/clause.rs
  lines: '1'
  role: examined
  excerpt: '//! The V1 clause lowering that the precondition and contract families share: the lowered clause'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/collection.rs
  path: src/kani/generate/collection.rs
  lines: '1'
  role: examined
  excerpt: //! Source-bound collection domains derived from admitted V2 model-field projections.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/contract.rs
  path: src/kani/generate/contract.rs
  lines: '1'
  role: examined
  excerpt: '//! The contract family: rendering the Kani contract harness of a V1 postcondition or invariant'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/bounded_kani_corpus.rs
  path: src/kani/generate/corpus/bounded_kani_corpus.rs
  lines: '1'
  role: examined
  excerpt: //! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/mod.rs
  path: src/kani/generate/corpus/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded-Kani corpus generator.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/frame.rs
  path: src/kani/generate/frame.rs
  lines: '1'
  role: examined
  excerpt: //! Operation-contract and frame-effect Kani obligations for one admitted `state_clause`
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_collections.rs
  path: src/kani/generate/lower/bounded_collections.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned ordered collection queries over IR's validated finite input and typed outcome.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_kani_profile.rs
  path: src/kani/generate/lower/bounded_kani_profile.rs
  lines: '1'
  role: examined
  excerpt: //! Codegen-side admission for Contract IR's public bounded-Kani profile.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/finite_reference_graphs.rs
  path: src/kani/generate/lower/finite_reference_graphs.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned finite graph reachability over IR's validated object and reference ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/mod.rs
  path: src/kani/generate/lower/mod.rs
  lines: '1'
  role: examined
  excerpt: // Bounded collection query admission.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  lines: '1'
  role: examined
  excerpt: //! Separate bounded Kani obligations with per-item backend negotiation (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  lines: '1'
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/precondition.rs
  path: src/kani/generate/precondition.rs
  lines: '1'
  role: examined
  excerpt: '//! The precondition family: rendering the Kani harness of a V1 precondition clause (FR-015).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/record.rs
  path: src/kani/generate/record.rs
  lines: '1'
  role: examined
  excerpt: //! The validated harness path and the persisted per-harness record that every family's render
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  lines: '1'
  role: examined
  excerpt: //! Pure ordered emission of validated harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/role_parity_tests.rs
  path: src/kani/generate/role_parity_tests.rs
  lines: '1'
  role: examined
  excerpt: //! Crate-internal parity checks for state-frame role negotiation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  lines: '1'
  role: examined
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/spec.rs
  path: src/kani/generate/spec.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, private harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/v1_bundle.rs
  path: src/kani/generate/v1_bundle.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: the V1 bounded Boolean Kani bundle generator, still live. `generate_kani_bundle`'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/identity.rs
  path: src/kani/identity.rs
  lines: '1'
  role: examined
  excerpt: //! The harness and identity record types of a generated Kani obligation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/mod.rs
  path: src/kani/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subsystem: harness generation, the run and the reading of its output (AD-004,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/diagnostics.rs
  path: src/kani/output/diagnostics.rs
  lines: '1'
  role: examined
  excerpt: //! The bounded diagnostic record of a run Kani gave no verdict for, or whose CBMC failed (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/mod.rs
  path: src/kani/output/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded record of a run Kani gave no verdict for.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/playback.rs
  path: src/kani/output/playback.rs
  lines: '1'
  role: examined
  excerpt: //! The concrete-playback block Kani prints on the console.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/report.rs
  path: src/kani/output/report.rs
  lines: '1'
  role: examined
  excerpt: //! The typed parse of the machine-readable report Kani writes (IR-277, IR-288).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  lines: '1'
  role: examined
  excerpt: //! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/harness.rs
  path: src/kani/run/harness.rs
  lines: '1'
  role: examined
  excerpt: //! The generated harness of any kind this crate can run, and exactly what execution reads from
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/launch.rs
  path: src/kani/run/launch.rs
  lines: '1'
  role: examined
  excerpt: '//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/memory.rs
  path: src/kani/run/memory.rs
  lines: '1'
  role: examined
  excerpt: //! Linux procfs observation of the sum of backend-process resident memory (FR-028-AC-21).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/mod.rs
  path: src/kani/run/mod.rs
  lines: '1'
  role: examined
  excerpt: '// One execution: the request, refusal and evidence types and `execute_kani_obligation`.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/namespace.rs
  path: src/kani/run/namespace.rs
  lines: '1'
  role: examined
  excerpt: '//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/owners.rs
  path: src/kani/run/owners.rs
  lines: '1'
  role: examined
  excerpt: //! Immutable dependencies of one bounded execution's existing observation and teardown owners.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/report_file.rs
  path: src/kani/run/report_file.rs
  lines: '1'
  role: examined
  excerpt: '//! The report file Kani exports for one run: its launch-unique path, the removal of a stale file'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  lines: '1'
  role: examined
  excerpt: //! Paired proof and native execution of one retained routed scalar artifact (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/tool.rs
  path: src/kani/run/tool.rs
  lines: '1'
  role: examined
  excerpt: //! The Kani backend this crate runs and where it is located (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/terminal.rs
  path: src/kani/terminal.rs
  lines: '1'
  role: examined
  excerpt: '//! The terminal-value maps of a Kani run (FR-029) and of a Contract IR outcome (FR-030): one'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/test_support.rs
  path: src/kani/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Test helpers shared by the tests of more than one `kani/` file.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/lib.rs
  path: src/lib.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/boolean_v1.rs
  path: src/oracle/boolean_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/bound_v1.rs
  path: src/oracle/bound_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Complete-package oracle lowering through the public IR binding boundary.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/claim.rs
  path: src/oracle/claim.rs
  lines: '1'
  role: examined
  excerpt: //! The generation-result and claim vocabulary every oracle generator returns.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/clause.rs
  path: src/oracle/clause.rs
  lines: '1'
  role: examined
  excerpt: //! V2 state-clause condition oracles over admitted checked packages.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/resolution.rs
  path: src/oracle/equality/resolution.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded, iterative reconstruction of equality operand types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 function-application oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  lines: '1'
  role: examined
  excerpt: //! Target-neutral, bounded flat SSA scheduling for FR-021.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/mod.rs
  path: src/oracle/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  lines: '1'
  role: examined
  excerpt: //! Closed typed identities consumed by admitted V2 generators.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/mod.rs
  path: src/publication/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The publication subsystem: validated, rollback-protected publication of a generated bundle'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/publish.rs
  path: src/publication/publish.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, rollback-protected publication of generated artifact bundles.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite.rs
  path: src/replay/composite.rs
  lines: '1'
  role: examined
  excerpt: //! Bind QSL composite parity reports to the exact claim sent by the driver (FR-033).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite_builder.rs
  path: src/replay/composite_builder.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/frame.rs
  path: src/replay/frame.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a frame counterexample: a Kani falsification of an operation''s frame is put'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/function.rs
  path: src/replay/function.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6 `replay` facade, and'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/mod.rs
  path: src/replay/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/obligation.rs
  path: src/replay/obligation.rs
  lines: '1'
  role: examined
  excerpt: //! The obligation identity of the function path and of the frame path (ADR-013 O-09,
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/scalar.rs
  path: src/replay/scalar.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/state_clause.rs
  path: src/replay/state_clause.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a postcondition state-clause counterexample: a Kani falsification of an'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/witness.rs
  path: src/replay/witness.rs
  lines: '1'
  role: examined
  excerpt: //! Reads a real Kani concrete-playback transcript against the generator's own persisted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/capability.rs
  path: src/routed/capability.rs
  lines: '1'
  role: examined
  excerpt: '//! Capability settlement: the one point where a requested claim is settled.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/generate.rs
  path: src/routed/generate.rs
  lines: '1'
  role: examined
  excerpt: '//! FR-022: generation for items the driver has already settled and routed.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/mod.rs
  path: src/routed/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The routed subsystem: capability settlement and generation for routed items (AD-004, FR-019,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/census.rs
  path: src/strategy/bound/census.rs
  lines: '1'
  role: examined
  excerpt: //! Domain and relation boundary census for one admitted relation (FR-010).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/generation.rs
  path: src/strategy/bound/generation.rs
  lines: '1'
  role: examined
  excerpt: //! Public bound-package admission and complete numeric strategy bundle generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/mod.rs
  path: src/strategy/bound/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Bound numeric strategy generation over admitted single-comparison clauses.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/population.rs
  path: src/strategy/bound/population.rs
  lines: '1'
  role: examined
  excerpt: //! Constructive `Satisfying`, `Violating`, and `Broad` populations for one bound comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/relation.rs
  path: src/strategy/bound/relation.rs
  lines: '1'
  role: examined
  excerpt: //! IR-independent model of one admitted integer comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/campaign.rs
  path: src/strategy/campaign.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic shaped proptest strategy generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/harness.rs
  path: src/strategy/harness.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic tri-state harness generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/mod.rs
  path: src/strategy/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The strategy subsystem: generated proptest strategies and tri-state harnesses (AD-004, FR-002,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  lines: '1'
  role: examined
  excerpt: //! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/common/proof_ceilings.rs
  path: tests/common/proof_ceilings.rs
  lines: '1'
  role: examined
  excerpt: //! Resource ceilings for test fixtures; never production defaults.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/codes.rs
  path: tests/composite_equality_support/codes.rs
  lines: '1'
  role: examined
  excerpt: //! The composite/structural corpus's fixture codes and readable node keys.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/package.rs
  path: tests/composite_equality_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for composite/structural equality
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_function_support/package.rs
  path: tests/exact_function_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for function-application oracle
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_render_private.rs
  path: tests/it/kani_render_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private libtest orchestration of a generated unsatisfiable-assumption control.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_digest.rs
  path: tests/it/kani_scalar_native_digest.rs
  lines: '1'
  role: examined
  excerpt: //! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_fixture.rs
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '1'
  role: examined
  excerpt: //! Shared admitted scalar routing fixture for the integration and private library controls.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private orchestration controls for scalar-native cleanup after real execution.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_terminal_private.rs
  path: tests/it/kani_terminal_private.rs
  lines: '1'
  role: examined
  excerpt: '//! Private orchestration: reuse bounded report stand-ins without reversing kani layer imports.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/layout.rs
  path: tests/it/layout.rs
  lines: '1'
  role: examined
  excerpt: //! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/model.rs
  path: tests/state_frame_support/model.rs
  lines: '1'
  role: examined
  excerpt: '//! The one source of the state-frame fixture''s model facts: the object''s integer fields with the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  lines: '1'
  role: examined
  excerpt: '//! The QSL twin of the state-frame fixture: the `test/bank` domain package, the native unit'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '384'
  role: examined
  excerpt: '| FR-032-AC-3 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before
    replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and
    harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original
    generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16. | Test |'
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '724'
  role: examined
  excerpt: '| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node
    whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments
    `[0, 9]` and `[1, 1]`. | Test (TC-033) |'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '745'
  role: examined
  excerpt: '| FR-015-AC-37 | A scalar function-application harness asserts the oracle''s outcome against the clause''s
    operation evaluated natively in `i128` over the same symbolic operands, independently of the oracle: `Completed`
    with exactly that value when it lies in the result bound, `Refused` when it does not, and any other outcome
    fails. With the installed backend, the healthy oracle verifies and an oracle whose arithmetic is mutated is
    falsified on that assertion. | Test (TC-025) |'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '823'
  role: examined
  excerpt: '| FR-015-AC-115 | Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of
    `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred
    operand; a claim with each operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path and source bytes), `ScalarObligationIdentity`,
    `options` and `ObligationRecord`, and the same digests derived from them, as a test-only reference copy of the
    pre-port `render_scalar` computes '
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '868'
  role: examined
  excerpt: '| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay`
    path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.
    |'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '869'
  role: examined
  excerpt: '| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in
    UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness
    and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing
    one byte of any entry changes the digest. No'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '870'
  role: examined
  excerpt: '| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence
    appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |'
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '871'
  role: examined
  excerpt: '| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest
    was built from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries.
    Recomputing the digest from only those exposed entries equals the receipt''s `DigestRecord`. A recomputation
    that substitutes one byte in the package, harness or harness record (which a driver holding different bytes
    would do) differs from the receipt''s digest. The exposed e'
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '872'
  role: examined
  excerpt: '| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change;
    the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema
    string unchanged. The domain remains `sh'
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '873'
  role: examined
  excerpt: '| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the
    native meter was constructed with. They are read back from the native invocation''s own output, not copied from
    the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper''s
    structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the meter-observed limits are unc'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '375'
  role: examined
  excerpt: '| FR-029-AC-15 | For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given
    no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`,
    `cover-unsatisfied`, `cancelled` and every inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement input is governed by AC-17
    and AC-19 to AC-28, not this criterion. | Test |'
- id: context:spec/spec.md
  path: spec/spec.md
  lines: '11'
  role: context_only
  excerpt: '# Master Requirements Specification'
- id: context:spec/assurance/AD-002-cg-qsl-replay-seam.md
  path: spec/assurance/AD-002-cg-qsl-replay-seam.md
  lines: '22'
  role: context_only
  excerpt: '# CG to QSL replay seam'
- id: context:spec/assurance/AD-003-evidence-chain.md
  path: spec/assurance/AD-003-evidence-chain.md
  lines: '30'
  role: context_only
  excerpt: '# Evidence chain across IR, CG and QSL'
- id: context:spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '32'
  role: context_only
  excerpt: '# CG crate layout'
- id: context:spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  lines: '11'
  role: context_only
  excerpt: '# TC-047: Check public routed scalar lowering replay and generated-content binding'
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  role: examined
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
- id: include:src/kani/generate/role_parity_tests.rs:5
  path: src/kani/generate/role_parity_tests.rs
  lines: '5'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/model.rs"]'
  included_target: tests/state_frame_support/model.rs
- id: include:src/kani/generate/role_parity_tests.rs:9
  path: src/kani/generate/role_parity_tests.rs
  lines: '9'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/native_twin.rs"]'
  included_target: tests/state_frame_support/native_twin.rs
- id: include:src/kani/test_support.rs:172
  path: src/kani/test_support.rs
  lines: '172'
  role: examined
  excerpt: '#[path = "../../tests/common/proof_ceilings.rs"]'
  included_target: tests/common/proof_ceilings.rs
- id: include:src/kani/test_support.rs:200
  path: src/kani/test_support.rs
  lines: '200'
  role: examined
  excerpt: '#[path = "../../tests/it/kani_scalar_native_digest.rs"]'
  included_target: tests/it/kani_scalar_native_digest.rs
- id: include:src/lib.rs:12
  path: src/lib.rs
  lines: '12'
  role: examined
  excerpt: '#[path = "../tests/it/kani_render_private.rs"]'
  included_target: tests/it/kani_render_private.rs
- id: include:src/lib.rs:16
  path: src/lib.rs
  lines: '16'
  role: examined
  excerpt: '#[path = "../tests/it/kani_scalar_native_private.rs"]'
  included_target: tests/it/kani_scalar_native_private.rs
- id: include:src/lib.rs:20
  path: src/lib.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "../tests/it/kani_terminal_private.rs"]'
  included_target: tests/it/kani_terminal_private.rs
- id: include:src/routed/contract_package_tests.rs:8
  path: src/routed/contract_package_tests.rs
  lines: '8'
  role: examined
  excerpt: '#[path = "../../tests/exact_function_support/package.rs"]'
  included_target: tests/exact_function_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:2
  path: tests/it/kani_scalar_native_private.rs
  lines: '2'
  role: examined
  excerpt: '#[path = "../exact_scalar_support/package.rs"]'
  included_target: tests/exact_scalar_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:20
  path: tests/it/kani_scalar_native_private.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "kani_scalar_native_fixture.rs"]'
  included_target: tests/it/kani_scalar_native_fixture.rs
- id: include:tests/exact_function_support/package.rs:23
  path: tests/exact_function_support/package.rs
  lines: '23'
  role: examined
  excerpt: '#[path = "../composite_equality_support/package.rs"]'
  included_target: tests/composite_equality_support/package.rs
- id: include:tests/exact_scalar_support/package.rs:41
  path: tests/exact_scalar_support/package.rs
  lines: '41'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:30
  path: tests/composite_equality_support/package.rs
  lines: '30'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:34
  path: tests/composite_equality_support/package.rs
  lines: '34'
  role: examined
  excerpt: '#[path = "codes.rs"]'
  included_target: tests/composite_equality_support/codes.rs
- id: L2:tests/composite_equality_support/package.rs:21
  path: tests/composite_equality_support/package.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:27
  path: tests/exact_function_support/package.rs
  lines: '27'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:80
  path: tests/exact_function_support/package.rs
  lines: '80'
  role: examined
  excerpt: 'result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,'
- id: L2:tests/exact_function_support/package.rs:526
  path: tests/exact_function_support/package.rs
  lines: '526'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:558
  path: tests/exact_function_support/package.rs
  lines: '558'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:571
  path: tests/exact_function_support/package.rs
  lines: '571'
  role: examined
  excerpt: 'operator: quire_contract_codegen::EqualityOperatorKind::Equal,'
- id: L2:tests/exact_function_support/package.rs:599
  path: tests/exact_function_support/package.rs
  lines: '599'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:625
  path: tests/exact_function_support/package.rs
  lines: '625'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:638
  path: tests/exact_function_support/package.rs
  lines: '638'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:651
  path: tests/exact_function_support/package.rs
  lines: '651'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:664
  path: tests/exact_function_support/package.rs
  lines: '664'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:677
  path: tests/exact_function_support/package.rs
  lines: '677'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:836
  path: tests/exact_function_support/package.rs
  lines: '836'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_scalar_support/package.rs:24
  path: tests/exact_scalar_support/package.rs
  lines: '24'
  role: examined
  excerpt: use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
- id: L2:tests/exact_scalar_support/package.rs:25
  path: tests/exact_scalar_support/package.rs
  lines: '25'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/it/kani_scalar_native_fixture.rs:4
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '4'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:21
  path: tests/state_frame_support/native_twin.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:39
  path: tests/state_frame_support/native_twin.rs
  lines: '39'
  role: examined
  excerpt: const fn unbounded_decode_limits() -> quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:40
  path: tests/state_frame_support/native_twin.rs
  lines: '40'
  role: examined
  excerpt: quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:871
  path: tests/state_frame_support/native_twin.rs
  lines: '871'
  role: examined
  excerpt: 'limits: quire_contract_codegen::WitnessDecodeLimits,'
- id: L2:tests/composite_equality_support/package.rs:32
  path: tests/composite_equality_support/package.rs
  lines: '32'
  role: examined
  excerpt: pub use rekey::FixtureIds;
- id: L2:tests/composite_equality_support/package.rs:36
  path: tests/composite_equality_support/package.rs
  lines: '36'
  role: examined
  excerpt: pub use codes::*;
- id: L2:tests/exact_function_support/package.rs:25
  path: tests/exact_function_support/package.rs
  lines: '25'
  role: examined
  excerpt: pub use ce::*;
- id: L2:tests/exact_scalar_support/package.rs:43
  path: tests/exact_scalar_support/package.rs
  lines: '43'
  role: examined
  excerpt: pub use rekey::FixtureIds;
bindings:
- id: FR-015-AC-115
  path: src/kani/generate/render.rs
  lines: '333'
  role: examined
  excerpt: fn renderer_refuses_size_before_invalid_syntax_with_exact_byte_count() {
  symbol: tests::renderer_refuses_size_before_invalid_syntax_with_exact_byte_count
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1220'
  role: examined
  excerpt: fn scalar_port_is_differentially_identical_for_all_operations_and_boundaries() {
  symbol: tests::scalar_port_is_differentially_identical_for_all_operations_and_boundaries
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1333'
  role: examined
  excerpt: fn scalar_port_preserves_resource_and_syntax_refusals() {
  symbol: tests::scalar_port_preserves_resource_and_syntax_refusals
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '202'
  role: examined
  excerpt: fn routed_draw_order_is_preserved_with_distinct_domains() {
  symbol: routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '241'
  role: examined
  excerpt: fn negate_and_literal_singleton_keep_every_draw() {
  symbol: negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '283'
  role: examined
  excerpt: fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
  symbol: subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '300'
  role: examined
  excerpt: fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
  symbol: inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '417'
  role: examined
  excerpt: fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
  symbol: schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '441'
  role: examined
  excerpt: fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
  symbol: unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '476'
  role: examined
  excerpt: fn original_renderer_limits_are_retained_with_genuine_routed_source() {
  symbol: original_renderer_limits_are_retained_with_genuine_routed_source
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '519'
  role: examined
  excerpt: fn original_limits_refuse_changed_retained_identity_source_or_record() {
  symbol: original_limits_refuse_changed_retained_identity_source_or_record
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: other
  artifact_id: AD-004-L-2
  related_ids:
  - AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
  finding: The actual cfg(test) include graph violates L-2 through self-facade imports and nonroot re-exports; native_twin
    also imports replay into kani::generate.
  classification: inherited baseline architecture debt; unchanged by PR481
  owner: AD-004 L-2 / IR-348 layout owner
  recommended_disposition: deferred outside this independently specified limits slice; preserve measured whole-graph
    limitation
  refs:
  - src/lib.rs:8
  - src/kani/generate/role_parity_tests.rs:9
  - tests/state_frame_support/native_twin.rs:21
  - tests/exact_function_support/package.rs:25
  - tests/exact_scalar_support/package.rs:43
  - tests/composite_equality_support/package.rs:32
  - tests/it/kani_scalar_native_fixture.rs:4
  - tests/it/layout.rs:489
  escape_cause: implementation-bug-despite-evidence
verdict: CONDITIONAL (inherited architecture debt); no current-delta defect
qualification: 'Original renderer limits only. The same typed ScalarLimits drives production meter emission and
  private retention; exact current all-ten u64::MAX policy, source/record bytes, namespace, proposition, bounds
  and proof-content preimage remain unchanged. Mutation of retained public identity/source/record refuses. No budget
  echo or unavailable-context default. Full FR-032-AC-3 remains partial: no admitted QSL plan, original checked-source/call-site/sent-wire
  authentication, Falsified native observation, driver call or converter/settlement. IR-940 is Backlog and IR-631
  In Progress. Existing scalar-native receipt remains Verified-only. Author logs at frozen head report 70 focused
  passes, fmt/all-target lint PASS, and one full Kani37/37 PASS; reviewer ran no heavy commands. Final root make
  ci plus separate full Kani on latest exact artifact custody remain UNRUN.'
coverage: 'L-2 inspection recursively resolved 14 real #[path] edges and 106 library-source/included files. Its
  four nonroot re-export lines and twenty self-facade references are inherited; role_parity_tests includes native_twin
  which imports FrameReplay/ReplayInputs from replay through the library self alias. tests/it/layout.rs scans src
  only and does not follow these external files, so its source-only passes do not establish full L-2. No new delta
  architecture violation, copied owner types, tracking digest, test stub or gate weakening found. No applicable
  AssuranceProfile exists. Finite meter literals emit a decimal u64-context expression; current policy is all MAX.
  Generated-only no-meter controls use generated() without limits and retain typed Unavailable semantics; they are
  not a new production meter context.'
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | UNFIXED inherited AD-004 L-2 defect. Deferred from the independently specified PR481 limits slice to existing IR-348 layout ownership, remaining In Progress: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc. Same-coder full prescope confirms at least ten repair surfaces plus shared-fixture callers and frozen PR397 overlap. No direct-import/partial move resolves the complete include-graph finding; no waiver, alias, scanner relaxation, public test SPI, domain change or full-L2 claim. The actual source remains unchanged. |

## Disposition round 1 coverage and full ordered records

Source remains 22cf3a5e174c25eb6dacf5a01945e0a38691d5a7; base 47c4a6adaaea4faa7bfe82b5eda6581aaff53897. Original Findings, Verdict and entire original artifact prefix remain unchanged. No new finding, code fix, heavy command or source edit. Existing debt is deferred, not fixed or accepted. No full L-2, AC-3, TC-047 or IR-631 completion claim.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-631
pr: quire-contract-codegen#481
pr_url: https://github.com/agent-ix/quire-contract-codegen/pull/481
source_head: 22cf3a5e174c25eb6dacf5a01945e0a38691d5a7
base_head: 47c4a6adaaea4faa7bfe82b5eda6581aaff53897
model: gpt-6.1-sol
runtime: Codex 0.163.0
run: 83e6dbcb-4c2f-4788-a22d-eac93c5b9b1b
quoin: 0.28.4
quire: 0.36.2 (engine 0.50.2)
module: spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502
visibility: public
no_heavy_commands: true
original_review_run: a16f2703-674d-4f58-917e-5d0348341741
round: 1
record_kind: reviewer-dispositions
source_unchanged: true
new_findings: []
owner_comment_id: fc122bdc-ee05-41ea-99bb-c5a377cf860b
owner_comment_url: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc
no_source_edits: true
root_final_gates: UNRUN pending exact same-coder custody
disposition_authority: Root confirmed existing ownership through actual IR-348 owner handoff; all inherited findings
  remain unfixed/unaccepted
id: SR-14220
method: code-review
original_verdict: CONDITIONAL (inherited architecture debt); no current-delta defect
current_verdict: CONDITIONAL (inherited architecture debt); no current-delta defect
scope:
- id: file:src/core/artifact.rs
  path: src/core/artifact.rs
  lines: '1'
  role: examined
  excerpt: //! Generated files and the validated bundle every generator builds (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/canonical.rs
  path: src/core/canonical.rs
  lines: '1'
  role: examined
  excerpt: //! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/contract_package.rs
  path: src/core/contract_package.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/diagnostic.rs
  path: src/core/diagnostic.rs
  lines: '1'
  role: examined
  excerpt: //! The generation diagnostic vocabulary shared by every generator (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/identity.rs
  path: src/core/identity.rs
  lines: '1'
  role: examined
  excerpt: //! Identity newtypes of a generated Kani harness (AD-004 `core/identity`, L-10).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/clause.rs
  path: src/core/ir/clause.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted checked-package state-clause body access.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/mod.rs
  path: src/core/ir/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Typed access to admitted V2 clause conditions and their model-owned fields.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/mod.rs
  path: src/core/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/naming.rs
  path: src/core/naming.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded readable name components, unique names and symbol derivation shared by every
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/profile.rs
  path: src/core/profile.rs
  lines: '1'
  role: examined
  excerpt: '//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/provenance.rs
  path: src/core/provenance.rs
  lines: '1'
  role: examined
  excerpt: //! Headers for generated Rust, built only from identities carried by generation inputs.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/source_map.rs
  path: src/core/source_map.rs
  lines: '1'
  role: examined
  excerpt: //! Source-map records that trace generated source back to a requirement clause (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/test_support.rs
  path: src/core/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Scratch directories shared by unit tests across subsystems.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/bound_coverage.rs
  path: src/evidence/bound_coverage.rs
  lines: '1'
  role: examined
  excerpt: //! Complete bound observations. No execution, authentication, or assurance verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/mod.rs
  path: src/evidence/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The evidence subsystem: bounded coverage observations (AD-004, FR-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/vacuity.rs
  path: src/evidence/vacuity.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded LLVM observation primitives, not a campaign or coverage-obligation verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/abi.rs
  path: src/kani/abi.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/census.rs
  path: src/kani/census.rs
  lines: '1'
  role: examined
  excerpt: //! The proof-dependency census types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/classify.rs
  path: src/kani/classify.rs
  lines: '1'
  role: examined
  excerpt: //! Classification of one Kani run from the report Kani exported for it (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/census_validation.rs
  path: src/kani/generate/census_validation.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: census validation and the V1 bundle''s error type it returns.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/clause.rs
  path: src/kani/generate/clause.rs
  lines: '1'
  role: examined
  excerpt: '//! The V1 clause lowering that the precondition and contract families share: the lowered clause'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/collection.rs
  path: src/kani/generate/collection.rs
  lines: '1'
  role: examined
  excerpt: //! Source-bound collection domains derived from admitted V2 model-field projections.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/contract.rs
  path: src/kani/generate/contract.rs
  lines: '1'
  role: examined
  excerpt: '//! The contract family: rendering the Kani contract harness of a V1 postcondition or invariant'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/bounded_kani_corpus.rs
  path: src/kani/generate/corpus/bounded_kani_corpus.rs
  lines: '1'
  role: examined
  excerpt: //! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/mod.rs
  path: src/kani/generate/corpus/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded-Kani corpus generator.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/frame.rs
  path: src/kani/generate/frame.rs
  lines: '1'
  role: examined
  excerpt: //! Operation-contract and frame-effect Kani obligations for one admitted `state_clause`
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_collections.rs
  path: src/kani/generate/lower/bounded_collections.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned ordered collection queries over IR's validated finite input and typed outcome.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_kani_profile.rs
  path: src/kani/generate/lower/bounded_kani_profile.rs
  lines: '1'
  role: examined
  excerpt: //! Codegen-side admission for Contract IR's public bounded-Kani profile.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/finite_reference_graphs.rs
  path: src/kani/generate/lower/finite_reference_graphs.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned finite graph reachability over IR's validated object and reference ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/mod.rs
  path: src/kani/generate/lower/mod.rs
  lines: '1'
  role: examined
  excerpt: // Bounded collection query admission.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  lines: '1'
  role: examined
  excerpt: //! Separate bounded Kani obligations with per-item backend negotiation (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  lines: '1'
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/precondition.rs
  path: src/kani/generate/precondition.rs
  lines: '1'
  role: examined
  excerpt: '//! The precondition family: rendering the Kani harness of a V1 precondition clause (FR-015).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/record.rs
  path: src/kani/generate/record.rs
  lines: '1'
  role: examined
  excerpt: //! The validated harness path and the persisted per-harness record that every family's render
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  lines: '1'
  role: examined
  excerpt: //! Pure ordered emission of validated harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/role_parity_tests.rs
  path: src/kani/generate/role_parity_tests.rs
  lines: '1'
  role: examined
  excerpt: //! Crate-internal parity checks for state-frame role negotiation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  lines: '1'
  role: examined
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/spec.rs
  path: src/kani/generate/spec.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, private harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/v1_bundle.rs
  path: src/kani/generate/v1_bundle.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: the V1 bounded Boolean Kani bundle generator, still live. `generate_kani_bundle`'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/identity.rs
  path: src/kani/identity.rs
  lines: '1'
  role: examined
  excerpt: //! The harness and identity record types of a generated Kani obligation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/mod.rs
  path: src/kani/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subsystem: harness generation, the run and the reading of its output (AD-004,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/diagnostics.rs
  path: src/kani/output/diagnostics.rs
  lines: '1'
  role: examined
  excerpt: //! The bounded diagnostic record of a run Kani gave no verdict for, or whose CBMC failed (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/mod.rs
  path: src/kani/output/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded record of a run Kani gave no verdict for.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/playback.rs
  path: src/kani/output/playback.rs
  lines: '1'
  role: examined
  excerpt: //! The concrete-playback block Kani prints on the console.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/report.rs
  path: src/kani/output/report.rs
  lines: '1'
  role: examined
  excerpt: //! The typed parse of the machine-readable report Kani writes (IR-277, IR-288).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  lines: '1'
  role: examined
  excerpt: //! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/harness.rs
  path: src/kani/run/harness.rs
  lines: '1'
  role: examined
  excerpt: //! The generated harness of any kind this crate can run, and exactly what execution reads from
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/launch.rs
  path: src/kani/run/launch.rs
  lines: '1'
  role: examined
  excerpt: '//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/memory.rs
  path: src/kani/run/memory.rs
  lines: '1'
  role: examined
  excerpt: //! Linux procfs observation of the sum of backend-process resident memory (FR-028-AC-21).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/mod.rs
  path: src/kani/run/mod.rs
  lines: '1'
  role: examined
  excerpt: '// One execution: the request, refusal and evidence types and `execute_kani_obligation`.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/namespace.rs
  path: src/kani/run/namespace.rs
  lines: '1'
  role: examined
  excerpt: '//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/owners.rs
  path: src/kani/run/owners.rs
  lines: '1'
  role: examined
  excerpt: //! Immutable dependencies of one bounded execution's existing observation and teardown owners.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/report_file.rs
  path: src/kani/run/report_file.rs
  lines: '1'
  role: examined
  excerpt: '//! The report file Kani exports for one run: its launch-unique path, the removal of a stale file'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  lines: '1'
  role: examined
  excerpt: //! Paired proof and native execution of one retained routed scalar artifact (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/tool.rs
  path: src/kani/run/tool.rs
  lines: '1'
  role: examined
  excerpt: //! The Kani backend this crate runs and where it is located (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/terminal.rs
  path: src/kani/terminal.rs
  lines: '1'
  role: examined
  excerpt: '//! The terminal-value maps of a Kani run (FR-029) and of a Contract IR outcome (FR-030): one'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/test_support.rs
  path: src/kani/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Test helpers shared by the tests of more than one `kani/` file.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/lib.rs
  path: src/lib.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/boolean_v1.rs
  path: src/oracle/boolean_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/bound_v1.rs
  path: src/oracle/bound_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Complete-package oracle lowering through the public IR binding boundary.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/claim.rs
  path: src/oracle/claim.rs
  lines: '1'
  role: examined
  excerpt: //! The generation-result and claim vocabulary every oracle generator returns.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/clause.rs
  path: src/oracle/clause.rs
  lines: '1'
  role: examined
  excerpt: //! V2 state-clause condition oracles over admitted checked packages.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/resolution.rs
  path: src/oracle/equality/resolution.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded, iterative reconstruction of equality operand types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 function-application oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  lines: '1'
  role: examined
  excerpt: //! Target-neutral, bounded flat SSA scheduling for FR-021.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/mod.rs
  path: src/oracle/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  lines: '1'
  role: examined
  excerpt: //! Closed typed identities consumed by admitted V2 generators.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/mod.rs
  path: src/publication/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The publication subsystem: validated, rollback-protected publication of a generated bundle'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/publish.rs
  path: src/publication/publish.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, rollback-protected publication of generated artifact bundles.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite.rs
  path: src/replay/composite.rs
  lines: '1'
  role: examined
  excerpt: //! Bind QSL composite parity reports to the exact claim sent by the driver (FR-033).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite_builder.rs
  path: src/replay/composite_builder.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/frame.rs
  path: src/replay/frame.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a frame counterexample: a Kani falsification of an operation''s frame is put'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/function.rs
  path: src/replay/function.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6 `replay` facade, and'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/mod.rs
  path: src/replay/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/obligation.rs
  path: src/replay/obligation.rs
  lines: '1'
  role: examined
  excerpt: //! The obligation identity of the function path and of the frame path (ADR-013 O-09,
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/scalar.rs
  path: src/replay/scalar.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/state_clause.rs
  path: src/replay/state_clause.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a postcondition state-clause counterexample: a Kani falsification of an'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/witness.rs
  path: src/replay/witness.rs
  lines: '1'
  role: examined
  excerpt: //! Reads a real Kani concrete-playback transcript against the generator's own persisted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/capability.rs
  path: src/routed/capability.rs
  lines: '1'
  role: examined
  excerpt: '//! Capability settlement: the one point where a requested claim is settled.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/generate.rs
  path: src/routed/generate.rs
  lines: '1'
  role: examined
  excerpt: '//! FR-022: generation for items the driver has already settled and routed.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/mod.rs
  path: src/routed/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The routed subsystem: capability settlement and generation for routed items (AD-004, FR-019,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/census.rs
  path: src/strategy/bound/census.rs
  lines: '1'
  role: examined
  excerpt: //! Domain and relation boundary census for one admitted relation (FR-010).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/generation.rs
  path: src/strategy/bound/generation.rs
  lines: '1'
  role: examined
  excerpt: //! Public bound-package admission and complete numeric strategy bundle generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/mod.rs
  path: src/strategy/bound/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Bound numeric strategy generation over admitted single-comparison clauses.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/population.rs
  path: src/strategy/bound/population.rs
  lines: '1'
  role: examined
  excerpt: //! Constructive `Satisfying`, `Violating`, and `Broad` populations for one bound comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/relation.rs
  path: src/strategy/bound/relation.rs
  lines: '1'
  role: examined
  excerpt: //! IR-independent model of one admitted integer comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/campaign.rs
  path: src/strategy/campaign.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic shaped proptest strategy generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/harness.rs
  path: src/strategy/harness.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic tri-state harness generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/mod.rs
  path: src/strategy/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The strategy subsystem: generated proptest strategies and tri-state harnesses (AD-004, FR-002,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  lines: '1'
  role: examined
  excerpt: //! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/common/proof_ceilings.rs
  path: tests/common/proof_ceilings.rs
  lines: '1'
  role: examined
  excerpt: //! Resource ceilings for test fixtures; never production defaults.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/codes.rs
  path: tests/composite_equality_support/codes.rs
  lines: '1'
  role: examined
  excerpt: //! The composite/structural corpus's fixture codes and readable node keys.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/package.rs
  path: tests/composite_equality_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for composite/structural equality
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_function_support/package.rs
  path: tests/exact_function_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for function-application oracle
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_render_private.rs
  path: tests/it/kani_render_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private libtest orchestration of a generated unsatisfiable-assumption control.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_digest.rs
  path: tests/it/kani_scalar_native_digest.rs
  lines: '1'
  role: examined
  excerpt: //! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_fixture.rs
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '1'
  role: examined
  excerpt: //! Shared admitted scalar routing fixture for the integration and private library controls.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private orchestration controls for scalar-native cleanup after real execution.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_terminal_private.rs
  path: tests/it/kani_terminal_private.rs
  lines: '1'
  role: examined
  excerpt: '//! Private orchestration: reuse bounded report stand-ins without reversing kani layer imports.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/layout.rs
  path: tests/it/layout.rs
  lines: '1'
  role: examined
  excerpt: //! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/model.rs
  path: tests/state_frame_support/model.rs
  lines: '1'
  role: examined
  excerpt: '//! The one source of the state-frame fixture''s model facts: the object''s integer fields with the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  lines: '1'
  role: examined
  excerpt: '//! The QSL twin of the state-frame fixture: the `test/bank` domain package, the native unit'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '384'
  role: examined
  excerpt: '| FR-032-AC-3 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before
    replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and
    harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original
    generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16. | Test |'
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '724'
  role: examined
  excerpt: '| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node
    whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments
    `[0, 9]` and `[1, 1]`. | Test (TC-033) |'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '745'
  role: examined
  excerpt: '| FR-015-AC-37 | A scalar function-application harness asserts the oracle''s outcome against the clause''s
    operation evaluated natively in `i128` over the same symbolic operands, independently of the oracle: `Completed`
    with exactly that value when it lies in the result bound, `Refused` when it does not, and any other outcome
    fails. With the installed backend, the healthy oracle verifies and an oracle whose arithmetic is mutated is
    falsified on that assertion. | Test (TC-025) |'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '823'
  role: examined
  excerpt: '| FR-015-AC-115 | Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of
    `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred
    operand; a claim with each operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path and source bytes), `ScalarObligationIdentity`,
    `options` and `ObligationRecord`, and the same digests derived from them, as a test-only reference copy of the
    pre-port `render_scalar` computes '
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '868'
  role: examined
  excerpt: '| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay`
    path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.
    |'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '869'
  role: examined
  excerpt: '| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in
    UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness
    and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing
    one byte of any entry changes the digest. No'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '870'
  role: examined
  excerpt: '| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence
    appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |'
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '871'
  role: examined
  excerpt: '| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest
    was built from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries.
    Recomputing the digest from only those exposed entries equals the receipt''s `DigestRecord`. A recomputation
    that substitutes one byte in the package, harness or harness record (which a driver holding different bytes
    would do) differs from the receipt''s digest. The exposed e'
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '872'
  role: examined
  excerpt: '| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change;
    the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema
    string unchanged. The domain remains `sh'
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '873'
  role: examined
  excerpt: '| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the
    native meter was constructed with. They are read back from the native invocation''s own output, not copied from
    the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper''s
    structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the meter-observed limits are unc'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '375'
  role: examined
  excerpt: '| FR-029-AC-15 | For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given
    no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`,
    `cover-unsatisfied`, `cancelled` and every inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement input is governed by AC-17
    and AC-19 to AC-28, not this criterion. | Test |'
- id: context:spec/spec.md
  path: spec/spec.md
  lines: '11'
  role: context_only
  excerpt: '# Master Requirements Specification'
- id: context:spec/assurance/AD-002-cg-qsl-replay-seam.md
  path: spec/assurance/AD-002-cg-qsl-replay-seam.md
  lines: '22'
  role: context_only
  excerpt: '# CG to QSL replay seam'
- id: context:spec/assurance/AD-003-evidence-chain.md
  path: spec/assurance/AD-003-evidence-chain.md
  lines: '30'
  role: context_only
  excerpt: '# Evidence chain across IR, CG and QSL'
- id: context:spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '32'
  role: context_only
  excerpt: '# CG crate layout'
- id: context:spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  lines: '11'
  role: context_only
  excerpt: '# TC-047: Check public routed scalar lowering replay and generated-content binding'
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  role: examined
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
- id: include:src/kani/generate/role_parity_tests.rs:5
  path: src/kani/generate/role_parity_tests.rs
  lines: '5'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/model.rs"]'
  included_target: tests/state_frame_support/model.rs
- id: include:src/kani/generate/role_parity_tests.rs:9
  path: src/kani/generate/role_parity_tests.rs
  lines: '9'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/native_twin.rs"]'
  included_target: tests/state_frame_support/native_twin.rs
- id: include:src/kani/test_support.rs:172
  path: src/kani/test_support.rs
  lines: '172'
  role: examined
  excerpt: '#[path = "../../tests/common/proof_ceilings.rs"]'
  included_target: tests/common/proof_ceilings.rs
- id: include:src/kani/test_support.rs:200
  path: src/kani/test_support.rs
  lines: '200'
  role: examined
  excerpt: '#[path = "../../tests/it/kani_scalar_native_digest.rs"]'
  included_target: tests/it/kani_scalar_native_digest.rs
- id: include:src/lib.rs:12
  path: src/lib.rs
  lines: '12'
  role: examined
  excerpt: '#[path = "../tests/it/kani_render_private.rs"]'
  included_target: tests/it/kani_render_private.rs
- id: include:src/lib.rs:16
  path: src/lib.rs
  lines: '16'
  role: examined
  excerpt: '#[path = "../tests/it/kani_scalar_native_private.rs"]'
  included_target: tests/it/kani_scalar_native_private.rs
- id: include:src/lib.rs:20
  path: src/lib.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "../tests/it/kani_terminal_private.rs"]'
  included_target: tests/it/kani_terminal_private.rs
- id: include:src/routed/contract_package_tests.rs:8
  path: src/routed/contract_package_tests.rs
  lines: '8'
  role: examined
  excerpt: '#[path = "../../tests/exact_function_support/package.rs"]'
  included_target: tests/exact_function_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:2
  path: tests/it/kani_scalar_native_private.rs
  lines: '2'
  role: examined
  excerpt: '#[path = "../exact_scalar_support/package.rs"]'
  included_target: tests/exact_scalar_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:20
  path: tests/it/kani_scalar_native_private.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "kani_scalar_native_fixture.rs"]'
  included_target: tests/it/kani_scalar_native_fixture.rs
- id: include:tests/exact_function_support/package.rs:23
  path: tests/exact_function_support/package.rs
  lines: '23'
  role: examined
  excerpt: '#[path = "../composite_equality_support/package.rs"]'
  included_target: tests/composite_equality_support/package.rs
- id: include:tests/exact_scalar_support/package.rs:41
  path: tests/exact_scalar_support/package.rs
  lines: '41'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:30
  path: tests/composite_equality_support/package.rs
  lines: '30'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:34
  path: tests/composite_equality_support/package.rs
  lines: '34'
  role: examined
  excerpt: '#[path = "codes.rs"]'
  included_target: tests/composite_equality_support/codes.rs
- id: L2:tests/composite_equality_support/package.rs:21
  path: tests/composite_equality_support/package.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:27
  path: tests/exact_function_support/package.rs
  lines: '27'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:80
  path: tests/exact_function_support/package.rs
  lines: '80'
  role: examined
  excerpt: 'result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,'
- id: L2:tests/exact_function_support/package.rs:526
  path: tests/exact_function_support/package.rs
  lines: '526'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:558
  path: tests/exact_function_support/package.rs
  lines: '558'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:571
  path: tests/exact_function_support/package.rs
  lines: '571'
  role: examined
  excerpt: 'operator: quire_contract_codegen::EqualityOperatorKind::Equal,'
- id: L2:tests/exact_function_support/package.rs:599
  path: tests/exact_function_support/package.rs
  lines: '599'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:625
  path: tests/exact_function_support/package.rs
  lines: '625'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:638
  path: tests/exact_function_support/package.rs
  lines: '638'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:651
  path: tests/exact_function_support/package.rs
  lines: '651'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:664
  path: tests/exact_function_support/package.rs
  lines: '664'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:677
  path: tests/exact_function_support/package.rs
  lines: '677'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:836
  path: tests/exact_function_support/package.rs
  lines: '836'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_scalar_support/package.rs:24
  path: tests/exact_scalar_support/package.rs
  lines: '24'
  role: examined
  excerpt: use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
- id: L2:tests/exact_scalar_support/package.rs:25
  path: tests/exact_scalar_support/package.rs
  lines: '25'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/it/kani_scalar_native_fixture.rs:4
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '4'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:21
  path: tests/state_frame_support/native_twin.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:39
  path: tests/state_frame_support/native_twin.rs
  lines: '39'
  role: examined
  excerpt: const fn unbounded_decode_limits() -> quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:40
  path: tests/state_frame_support/native_twin.rs
  lines: '40'
  role: examined
  excerpt: quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:871
  path: tests/state_frame_support/native_twin.rs
  lines: '871'
  role: examined
  excerpt: 'limits: quire_contract_codegen::WitnessDecodeLimits,'
- id: L2:tests/composite_equality_support/package.rs:32
  path: tests/composite_equality_support/package.rs
  lines: '32'
  role: examined
  excerpt: pub use rekey::FixtureIds;
- id: L2:tests/composite_equality_support/package.rs:36
  path: tests/composite_equality_support/package.rs
  lines: '36'
  role: examined
  excerpt: pub use codes::*;
- id: L2:tests/exact_function_support/package.rs:25
  path: tests/exact_function_support/package.rs
  lines: '25'
  role: examined
  excerpt: pub use ce::*;
- id: L2:tests/exact_scalar_support/package.rs:43
  path: tests/exact_scalar_support/package.rs
  lines: '43'
  role: examined
  excerpt: pub use rekey::FixtureIds;
- id: disposition-context:l2-finding-prescope.md
  path: /home/peter/dev/worktrees/kani-lead-codex/ir631-context/l2-finding-prescope.md
  lines: '1'
  role: context_only
  excerpt: '# PR481 inherited L2 finding — read-only scope assessment'
- id: disposition-context:l2-owner-handoff.md
  path: /home/peter/dev/worktrees/kani-lead-codex/ir631-context/l2-owner-handoff.md
  lines: '1'
  role: context_only
  excerpt: PR481 review SR-14220 FND-001 is inherited AD-004 L-2 debt, retained OPEN under the existing IR-348 layout
    owner. The finding is unchanged between reviewed base47c4a6adaaea4faa7bfe82b5eda6581aaff53897 and source22cf3a5e174c25eb6dacf5a01945e0a38691d5a7.
    PR481 only retains the actual original renderer ScalarLimits and does not claim full L-2 or full IR-631 acceptance.
bindings:
- id: FR-015-AC-115
  path: src/kani/generate/render.rs
  lines: '333'
  role: examined
  excerpt: fn renderer_refuses_size_before_invalid_syntax_with_exact_byte_count() {
  symbol: tests::renderer_refuses_size_before_invalid_syntax_with_exact_byte_count
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1220'
  role: examined
  excerpt: fn scalar_port_is_differentially_identical_for_all_operations_and_boundaries() {
  symbol: tests::scalar_port_is_differentially_identical_for_all_operations_and_boundaries
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1333'
  role: examined
  excerpt: fn scalar_port_preserves_resource_and_syntax_refusals() {
  symbol: tests::scalar_port_preserves_resource_and_syntax_refusals
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '202'
  role: examined
  excerpt: fn routed_draw_order_is_preserved_with_distinct_domains() {
  symbol: routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '241'
  role: examined
  excerpt: fn negate_and_literal_singleton_keep_every_draw() {
  symbol: negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '283'
  role: examined
  excerpt: fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
  symbol: subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '300'
  role: examined
  excerpt: fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
  symbol: inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '417'
  role: examined
  excerpt: fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
  symbol: schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '441'
  role: examined
  excerpt: fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
  symbol: unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '476'
  role: examined
  excerpt: fn original_renderer_limits_are_retained_with_genuine_routed_source() {
  symbol: original_renderer_limits_are_retained_with_genuine_routed_source
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '519'
  role: examined
  excerpt: fn original_limits_refuse_changed_retained_identity_source_or_record() {
  symbol: original_limits_refuse_changed_retained_identity_source_or_record
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: 'UNFIXED inherited AD-004 L-2 defect. Deferred from the independently specified PR481 limits slice to
    existing IR-348 layout ownership, remaining In Progress: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc.
    Same-coder full prescope confirms at least ten repair surfaces plus shared-fixture callers and frozen PR397
    overlap. No direct-import/partial move resolves the complete include-graph finding; no waiver, alias, scanner
    relaxation, public test SPI, domain change or full-L2 claim. The actual source remains unchanged.'
  artifact_id: AD-004-L-2
  related_ids:
  - AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
  owner: AD-004 L-2 / IR-348 layout owner
  severity: medium
  confidence: high
  finding: The actual cfg(test) include graph violates L-2 through self-facade imports and nonroot re-exports; native_twin
    also imports replay into kani::generate.
  source_status: unchanged; not fixed
  original_classification: inherited baseline architecture debt; unchanged by PR481
qualification: 'Original renderer limits only. The same typed ScalarLimits drives production meter emission and
  private retention; exact current all-ten u64::MAX policy, source/record bytes, namespace, proposition, bounds
  and proof-content preimage remain unchanged. Mutation of retained public identity/source/record refuses. No budget
  echo or unavailable-context default. Full FR-032-AC-3 remains partial: no admitted QSL plan, original checked-source/call-site/sent-wire
  authentication, Falsified native observation, driver call or converter/settlement. IR-940 is Backlog and IR-631
  In Progress. Existing scalar-native receipt remains Verified-only. Author logs at frozen head report 70 focused
  passes, fmt/all-target lint PASS, and one full Kani37/37 PASS; reviewer ran no heavy commands. Final root make
  ci plus separate full Kani on latest exact artifact custody remain UNRUN.'
coverage: 'L-2 inspection recursively resolved 14 real #[path] edges and 106 library-source/included files. Its
  four nonroot re-export lines and twenty self-facade references are inherited; role_parity_tests includes native_twin
  which imports FrameReplay/ReplayInputs from replay through the library self alias. tests/it/layout.rs scans src
  only and does not follow these external files, so its source-only passes do not establish full L-2. No new delta
  architecture violation, copied owner types, tracking digest, test stub or gate weakening found. No applicable
  AssuranceProfile exists. Finite meter literals emit a decimal u64-context expression; current policy is all MAX.
  Generated-only no-meter controls use generated() without limits and retain typed Unavailable semantics; they are
  not a new production meter context.'
prescope: Complete coder L2 scope assessment read; real included-file violation and full repair scope remain. Full311
  inherited matrix findings remain open; partial FR032-AC3 retention evidence is unchanged.
```

## Disposition round 2 outcomes

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | UNFIXED inherited AD-004 L-2 defect. Deferred from the independently specified PR481 limits slice to existing IR-348 layout ownership, remaining In Progress: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc. Same-coder full prescope confirms at least ten repair surfaces plus shared-fixture callers and frozen PR397 overlap. No direct-import/partial move resolves the complete include-graph finding; no waiver, alias, scanner relaxation, public test SPI, domain change or full-L2 claim. The actual source remains unchanged. |

## Disposition round 2 current-head coverage and full ordered records

Current frozen PR481 head b362bf5905b2a1ecc6cdcbe7b6422aacf1d006fc / actual base b64330c731079f9190158936f8b9b4d80d551a97. Independent current-head source review and whole computed matrix refreshed after controlled rebase. The exact seven-file limits delta and all169 Rust/Cargo/Makefile paths are byte-identical to original reviewed22cf; same typed value drives production emission/retention, all10limits remain u64MAX, identity/source/record mutation refuses, and no canonical proof-content preimage or Verified-only receipt change. No new original-limits code defect, new source gap, stub, gate weakening or included-graph change. Full L2 remains unfixed under IR348, full AC3/TC047/IR631 remains unaccepted and IR940-gated. No original checked source/callsite/sentwire/Falsified native/QSL driver/converter completion is claimed. Earlier author logs were read at22cf, not rerun/current-head proof evidence. Root final CI and separate full Kani remain UNRUN.

Plan completion: not assessed
Current full computed matrix:40 requirements,869criteria,1411binder edges;510tagged,291untagged,18ignored-only,50method-without-symbol. All869 have no bound run evidence. All868 prior matrix records exactly unchanged; the sole addition is FR015AC119 untagged. Current gap findings312 (291untagged+18ignored-only+3real stale explicit attributes). Previous311 remain open/unaccepted; newFND312 is upstream planned normative scope under actual IR917 Done spec-delivery state. Library graph unchanged106files/14includes/20self-facade references/4nonroot reexports; no full-L2 claim. Whole code/test inventory remains158Rust files/745public headers including test helpers. Coverage engine diagnostics retained, no measured code coverage or optional exhaustive semantic-review claim. No new tag or test discharges clause-body AC119; existing scalar implies AC118 and renderer retention tests do not substitute.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-631
pr: quire-contract-codegen#481
pr_url: https://github.com/agent-ix/quire-contract-codegen/pull/481
source_head: b362bf5905b2a1ecc6cdcbe7b6422aacf1d006fc
base_head: b64330c731079f9190158936f8b9b4d80d551a97
model: gpt-6.1-sol
runtime: Codex 0.163.0
run: 5db5a19c-024a-4f12-8546-1c0f22cf6d5c
quoin: 0.28.4
quire: 0.36.2 (engine 0.50.2)
module: spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502
visibility: public
no_heavy_commands: true
original_review_run: a16f2703-674d-4f58-917e-5d0348341741
round: 2
record_kind: reviewer-dispositions
source_unchanged: false
owner_comment_id: fc122bdc-ee05-41ea-99bb-c5a377cf860b
owner_comment_url: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc
no_source_edits: true
root_final_gates: UNRUN pending exact same-coder custody
disposition_authority: Root confirmed existing ownership through actual IR-348 owner handoff; all inherited findings
  remain unfixed/unaccepted
previous_disposition_run: 83e6dbcb-4c2f-4788-a22d-eac93c5b9b1b
implementation_source_unchanged: true
ac119_owner_comment_id: 24503521-fffd-4b8a-8622-f05b6e48c74a
ac119_owner_comment_url: https://linear.app/agent-ix/issue/IR-917/restore-qsl-authored-v2-clause-implication-kani-and-runtime-helper#comment-24503521
actual_ir917_tracker_state: Done (specification delivery; no implementation acceptance)
rebase_basis: All169 Rust/Cargo/Makefile paths and entire seven-file original-limits diff independently byte-identical
  to22cf; upstream base adds only planned FR015AC119/TC025 and6own reviews
current_head_author_gate: NOT RERUN; earlier author 37/37 Kani and70 focused PASS occurred at22cf only. Root final
  aggregate CI and separate full Kani remain UNRUN pending latest exact custody
id: SR-14220
method: code-review
original_verdict: CONDITIONAL (inherited architecture debt); no current-delta defect
current_verdict: CONDITIONAL (inherited architecture debt); no current-delta defect
scope:
- id: file:src/core/artifact.rs
  path: src/core/artifact.rs
  lines: '1'
  role: examined
  excerpt: //! Generated files and the validated bundle every generator builds (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/canonical.rs
  path: src/core/canonical.rs
  lines: '1'
  role: examined
  excerpt: //! The one place CG calls `quire-canonical`'s RFC 8785 encoder and digest (AD-004, AD-003 E-1).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/contract_package.rs
  path: src/core/contract_package.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/diagnostic.rs
  path: src/core/diagnostic.rs
  lines: '1'
  role: examined
  excerpt: //! The generation diagnostic vocabulary shared by every generator (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/identity.rs
  path: src/core/identity.rs
  lines: '1'
  role: examined
  excerpt: //! Identity newtypes of a generated Kani harness (AD-004 `core/identity`, L-10).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/clause.rs
  path: src/core/ir/clause.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted checked-package state-clause body access.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/ir/mod.rs
  path: src/core/ir/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Typed access to admitted V2 clause conditions and their model-owned fields.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/mod.rs
  path: src/core/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/naming.rs
  path: src/core/naming.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded readable name components, unique names and symbol derivation shared by every
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/profile.rs
  path: src/core/profile.rs
  lines: '1'
  role: examined
  excerpt: '//! The version profile: spellings this build emits (AD-004 `core/profile`, L-11).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/provenance.rs
  path: src/core/provenance.rs
  lines: '1'
  role: examined
  excerpt: //! Headers for generated Rust, built only from identities carried by generation inputs.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/source_map.rs
  path: src/core/source_map.rs
  lines: '1'
  role: examined
  excerpt: //! Source-map records that trace generated source back to a requirement clause (AD-004 step 2a).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/core/test_support.rs
  path: src/core/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Scratch directories shared by unit tests across subsystems.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/bound_coverage.rs
  path: src/evidence/bound_coverage.rs
  lines: '1'
  role: examined
  excerpt: //! Complete bound observations. No execution, authentication, or assurance verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/mod.rs
  path: src/evidence/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The evidence subsystem: bounded coverage observations (AD-004, FR-004).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/evidence/vacuity.rs
  path: src/evidence/vacuity.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded LLVM observation primitives, not a campaign or coverage-obligation verdict.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/abi.rs
  path: src/kani/abi.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/census.rs
  path: src/kani/census.rs
  lines: '1'
  role: examined
  excerpt: //! The proof-dependency census types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/classify.rs
  path: src/kani/classify.rs
  lines: '1'
  role: examined
  excerpt: //! Classification of one Kani run from the report Kani exported for it (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/census_validation.rs
  path: src/kani/generate/census_validation.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: census validation and the V1 bundle''s error type it returns.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/clause.rs
  path: src/kani/generate/clause.rs
  lines: '1'
  role: examined
  excerpt: '//! The V1 clause lowering that the precondition and contract families share: the lowered clause'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/collection.rs
  path: src/kani/generate/collection.rs
  lines: '1'
  role: examined
  excerpt: //! Source-bound collection domains derived from admitted V2 model-field projections.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/contract.rs
  path: src/kani/generate/contract.rs
  lines: '1'
  role: examined
  excerpt: '//! The contract family: rendering the Kani contract harness of a V1 postcondition or invariant'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/bounded_kani_corpus.rs
  path: src/kani/generate/corpus/bounded_kani_corpus.rs
  lines: '1'
  role: examined
  excerpt: //! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/corpus/mod.rs
  path: src/kani/generate/corpus/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded-Kani corpus generator.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/frame.rs
  path: src/kani/generate/frame.rs
  lines: '1'
  role: examined
  excerpt: //! Operation-contract and frame-effect Kani obligations for one admitted `state_clause`
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_collections.rs
  path: src/kani/generate/lower/bounded_collections.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned ordered collection queries over IR's validated finite input and typed outcome.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/bounded_kani_profile.rs
  path: src/kani/generate/lower/bounded_kani_profile.rs
  lines: '1'
  role: examined
  excerpt: //! Codegen-side admission for Contract IR's public bounded-Kani profile.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/finite_reference_graphs.rs
  path: src/kani/generate/lower/finite_reference_graphs.rs
  lines: '1'
  role: examined
  excerpt: //! CG-owned finite graph reachability over IR's validated object and reference ABI.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/lower/mod.rs
  path: src/kani/generate/lower/mod.rs
  lines: '1'
  role: examined
  excerpt: // Bounded collection query admission.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/negotiate.rs
  path: src/kani/generate/negotiate.rs
  lines: '1'
  role: examined
  excerpt: //! Separate bounded Kani obligations with per-item backend negotiation (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/outcome.rs
  path: src/kani/generate/outcome.rs
  lines: '1'
  role: examined
  excerpt: '//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/precondition.rs
  path: src/kani/generate/precondition.rs
  lines: '1'
  role: examined
  excerpt: '//! The precondition family: rendering the Kani harness of a V1 precondition clause (FR-015).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/record.rs
  path: src/kani/generate/record.rs
  lines: '1'
  role: examined
  excerpt: //! The validated harness path and the persisted per-harness record that every family's render
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  lines: '1'
  role: examined
  excerpt: //! Pure ordered emission of validated harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/role_parity_tests.rs
  path: src/kani/generate/role_parity_tests.rs
  lines: '1'
  role: examined
  excerpt: //! Crate-internal parity checks for state-frame role negotiation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  lines: '1'
  role: examined
  excerpt: '//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/spec.rs
  path: src/kani/generate/spec.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, private harness descriptions (FR-015).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/generate/v1_bundle.rs
  path: src/kani/generate/v1_bundle.rs
  lines: '1'
  role: examined
  excerpt: '//! INTERIM: the V1 bounded Boolean Kani bundle generator, still live. `generate_kani_bundle`'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/identity.rs
  path: src/kani/identity.rs
  lines: '1'
  role: examined
  excerpt: //! The harness and identity record types of a generated Kani obligation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/mod.rs
  path: src/kani/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The Kani subsystem: harness generation, the run and the reading of its output (AD-004,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/diagnostics.rs
  path: src/kani/output/diagnostics.rs
  lines: '1'
  role: examined
  excerpt: //! The bounded diagnostic record of a run Kani gave no verdict for, or whose CBMC failed (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/mod.rs
  path: src/kani/output/mod.rs
  lines: '1'
  role: examined
  excerpt: // The bounded record of a run Kani gave no verdict for.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/playback.rs
  path: src/kani/output/playback.rs
  lines: '1'
  role: examined
  excerpt: //! The concrete-playback block Kani prints on the console.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/output/report.rs
  path: src/kani/output/report.rs
  lines: '1'
  role: examined
  excerpt: //! The typed parse of the machine-readable report Kani writes (IR-277, IR-288).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  lines: '1'
  role: examined
  excerpt: //! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/harness.rs
  path: src/kani/run/harness.rs
  lines: '1'
  role: examined
  excerpt: //! The generated harness of any kind this crate can run, and exactly what execution reads from
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/launch.rs
  path: src/kani/run/launch.rs
  lines: '1'
  role: examined
  excerpt: '//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/memory.rs
  path: src/kani/run/memory.rs
  lines: '1'
  role: examined
  excerpt: //! Linux procfs observation of the sum of backend-process resident memory (FR-028-AC-21).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/mod.rs
  path: src/kani/run/mod.rs
  lines: '1'
  role: examined
  excerpt: '// One execution: the request, refusal and evidence types and `execute_kani_obligation`.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/namespace.rs
  path: src/kani/run/namespace.rs
  lines: '1'
  role: examined
  excerpt: '//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/owners.rs
  path: src/kani/run/owners.rs
  lines: '1'
  role: examined
  excerpt: //! Immutable dependencies of one bounded execution's existing observation and teardown owners.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/report_file.rs
  path: src/kani/run/report_file.rs
  lines: '1'
  role: examined
  excerpt: '//! The report file Kani exports for one run: its launch-unique path, the removal of a stale file'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/scalar.rs
  path: src/kani/run/scalar.rs
  lines: '1'
  role: examined
  excerpt: //! Paired proof and native execution of one retained routed scalar artifact (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/run/tool.rs
  path: src/kani/run/tool.rs
  lines: '1'
  role: examined
  excerpt: //! The Kani backend this crate runs and where it is located (FR-017).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/terminal.rs
  path: src/kani/terminal.rs
  lines: '1'
  role: examined
  excerpt: '//! The terminal-value maps of a Kani run (FR-029) and of a Contract IR outcome (FR-030): one'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/kani/test_support.rs
  path: src/kani/test_support.rs
  lines: '1'
  role: examined
  excerpt: //! Test helpers shared by the tests of more than one `kani/` file.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/lib.rs
  path: src/lib.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/boolean_v1.rs
  path: src/oracle/boolean_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/bound_v1.rs
  path: src/oracle/bound_v1.rs
  lines: '1'
  role: examined
  excerpt: //! Complete-package oracle lowering through the public IR binding boundary.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/claim.rs
  path: src/oracle/claim.rs
  lines: '1'
  role: examined
  excerpt: //! The generation-result and claim vocabulary every oracle generator returns.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/clause.rs
  path: src/oracle/clause.rs
  lines: '1'
  role: examined
  excerpt: //! V2 state-clause condition oracles over admitted checked packages.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact composite/structural equality oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/equality/resolution.rs
  path: src/oracle/equality/resolution.rs
  lines: '1'
  role: examined
  excerpt: //! Bounded, iterative reconstruction of equality operand types.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/mod.rs
  path: src/oracle/function/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 function-application oracle generation from one admitted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/function/schedule.rs
  path: src/oracle/function/schedule.rs
  lines: '1'
  role: examined
  excerpt: //! Target-neutral, bounded flat SSA scheduling for FR-021.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/mod.rs
  path: src/oracle/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/operator_catalog.rs
  path: src/oracle/operator_catalog.rs
  lines: '1'
  role: examined
  excerpt: //! Closed typed identities consumed by admitted V2 generators.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/oracle/scalar/mod.rs
  path: src/oracle/scalar/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/mod.rs
  path: src/publication/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The publication subsystem: validated, rollback-protected publication of a generated bundle'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/publication/publish.rs
  path: src/publication/publish.rs
  lines: '1'
  role: examined
  excerpt: //! Validated, rollback-protected publication of generated artifact bundles.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite.rs
  path: src/replay/composite.rs
  lines: '1'
  role: examined
  excerpt: //! Bind QSL composite parity reports to the exact claim sent by the driver (FR-033).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/composite_builder.rs
  path: src/replay/composite_builder.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/frame.rs
  path: src/replay/frame.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a frame counterexample: a Kani falsification of an operation''s frame is put'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/function.rs
  path: src/replay/function.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay adapter: a decoded Kani falsification goes to QSL''s layer-6 `replay` facade, and'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/mod.rs
  path: src/replay/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/obligation.rs
  path: src/replay/obligation.rs
  lines: '1'
  role: examined
  excerpt: //! The obligation identity of the function path and of the frame path (ADR-013 O-09,
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/scalar.rs
  path: src/replay/scalar.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/state_clause.rs
  path: src/replay/state_clause.rs
  lines: '1'
  role: examined
  excerpt: '//! Native replay of a postcondition state-clause counterexample: a Kani falsification of an'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/replay/witness.rs
  path: src/replay/witness.rs
  lines: '1'
  role: examined
  excerpt: //! Reads a real Kani concrete-playback transcript against the generator's own persisted
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/capability.rs
  path: src/routed/capability.rs
  lines: '1'
  role: examined
  excerpt: '//! Capability settlement: the one point where a requested claim is settled.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/contract_package_tests.rs
  path: src/routed/contract_package_tests.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/generate.rs
  path: src/routed/generate.rs
  lines: '1'
  role: examined
  excerpt: '//! FR-022: generation for items the driver has already settled and routed.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/routed/mod.rs
  path: src/routed/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The routed subsystem: capability settlement and generation for routed items (AD-004, FR-019,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/census.rs
  path: src/strategy/bound/census.rs
  lines: '1'
  role: examined
  excerpt: //! Domain and relation boundary census for one admitted relation (FR-010).
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/generation.rs
  path: src/strategy/bound/generation.rs
  lines: '1'
  role: examined
  excerpt: //! Public bound-package admission and complete numeric strategy bundle generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/mod.rs
  path: src/strategy/bound/mod.rs
  lines: '1'
  role: examined
  excerpt: //! Bound numeric strategy generation over admitted single-comparison clauses.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/population.rs
  path: src/strategy/bound/population.rs
  lines: '1'
  role: examined
  excerpt: //! Constructive `Satisfying`, `Violating`, and `Broad` populations for one bound comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/bound/relation.rs
  path: src/strategy/bound/relation.rs
  lines: '1'
  role: examined
  excerpt: //! IR-independent model of one admitted integer comparison.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/campaign.rs
  path: src/strategy/campaign.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic shaped proptest strategy generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/harness.rs
  path: src/strategy/harness.rs
  lines: '1'
  role: examined
  excerpt: //! Deterministic tri-state harness generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:src/strategy/mod.rs
  path: src/strategy/mod.rs
  lines: '1'
  role: examined
  excerpt: '//! The strategy subsystem: generated proptest strategies and tri-state harnesses (AD-004, FR-002,'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/checked_package_support/rekey.rs
  path: tests/checked_package_support/rekey.rs
  lines: '1'
  role: examined
  excerpt: //! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/common/proof_ceilings.rs
  path: tests/common/proof_ceilings.rs
  lines: '1'
  role: examined
  excerpt: //! Resource ceilings for test fixtures; never production defaults.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/codes.rs
  path: tests/composite_equality_support/codes.rs
  lines: '1'
  role: examined
  excerpt: //! The composite/structural corpus's fixture codes and readable node keys.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/composite_equality_support/package.rs
  path: tests/composite_equality_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for composite/structural equality
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_function_support/package.rs
  path: tests/exact_function_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for function-application oracle
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  lines: '1'
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_render_private.rs
  path: tests/it/kani_render_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private libtest orchestration of a generated unsatisfiable-assumption control.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_digest.rs
  path: tests/it/kani_scalar_native_digest.rs
  lines: '1'
  role: examined
  excerpt: //! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_fixture.rs
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '1'
  role: examined
  excerpt: //! Shared admitted scalar routing fixture for the integration and private library controls.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_scalar_native_private.rs
  path: tests/it/kani_scalar_native_private.rs
  lines: '1'
  role: examined
  excerpt: //! Private orchestration controls for scalar-native cleanup after real execution.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/kani_terminal_private.rs
  path: tests/it/kani_terminal_private.rs
  lines: '1'
  role: examined
  excerpt: '//! Private orchestration: reuse bounded report stand-ins without reversing kani layer imports.'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/layout.rs
  path: tests/it/layout.rs
  lines: '1'
  role: examined
  excerpt: //! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/it/scalar_playback.rs
  path: tests/it/scalar_playback.rs
  lines: '1'
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/model.rs
  path: tests/state_frame_support/model.rs
  lines: '1'
  role: examined
  excerpt: '//! The one source of the state-frame fixture''s model facts: the object''s integer fields with the'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: file:tests/state_frame_support/native_twin.rs
  path: tests/state_frame_support/native_twin.rs
  lines: '1'
  role: examined
  excerpt: '//! The QSL twin of the state-frame fixture: the `test/bank` domain package, the native unit'
  inspection: source and included-file architecture audit; changed files reviewed in full, unmodified files for
    ownership/edges
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '384'
  role: examined
  excerpt: '| FR-032-AC-3 | PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before
    replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and
    harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original
    generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16. | Test |'
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '729'
  role: examined
  excerpt: '| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node
    whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments
    `[0, 9]` and `[1, 1]`. | Test (TC-033) |'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '750'
  role: examined
  excerpt: '| FR-015-AC-37 | A scalar function-application harness asserts the oracle''s outcome against the clause''s
    operation evaluated natively in `i128` over the same symbolic operands, independently of the oracle: `Completed`
    with exactly that value when it lies in the result bound, `Refused` when it does not, and any other outcome
    fails. With the installed backend, the healthy oracle verifies and an oracle whose arithmetic is mutated is
    falsified on that assertion. | Test (TC-025) |'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '828'
  role: examined
  excerpt: '| FR-015-AC-115 | Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of
    `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred
    operand; a claim with each operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path and source bytes), `ScalarObligationIdentity`,
    `options` and `ObligationRecord`, and the same digests derived from them, as a test-only reference copy of the
    pre-port `render_scalar` computes '
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '868'
  role: examined
  excerpt: '| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord`
    re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay`
    path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
    "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.
    |'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '869'
  role: examined
  excerpt: '| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in
    UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`,
    `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs`
    (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness
    and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing
    one byte of any entry changes the digest. No'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '870'
  role: examined
  excerpt: '| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits`
    and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence
    appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |'
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '871'
  role: examined
  excerpt: '| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest
    was built from, including both manifests, both independently generated locks, the source map and the canonical
    `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries.
    Recomputing the digest from only those exposed entries equals the receipt''s `DigestRecord`. A recomputation
    that substitutes one byte in the package, harness or harness record (which a driver holding different bytes
    would do) differs from the receipt''s digest. The exposed e'
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '872'
  role: examined
  excerpt: '| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization
    or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change;
    the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test
    cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema
    string unchanged. The domain remains `sh'
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  lines: '873'
  role: examined
  excerpt: '| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the
    native meter was constructed with. They are read back from the native invocation''s own output, not copied from
    the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper''s
    structured result unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result` / `EnvironmentFault`
    and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()`
    are equal, and the meter-observed limits are unc'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  lines: '375'
  role: examined
  excerpt: '| FR-029-AC-15 | For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given
    no replay settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`,
    `cover-unsatisfied`, `cancelled` and every inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement input is governed by AC-17
    and AC-19 to AC-28, not this criterion. | Test |'
- id: context:spec/spec.md
  path: spec/spec.md
  lines: '11'
  role: context_only
  excerpt: '# Master Requirements Specification'
- id: context:spec/assurance/AD-002-cg-qsl-replay-seam.md
  path: spec/assurance/AD-002-cg-qsl-replay-seam.md
  lines: '22'
  role: context_only
  excerpt: '# CG to QSL replay seam'
- id: context:spec/assurance/AD-003-evidence-chain.md
  path: spec/assurance/AD-003-evidence-chain.md
  lines: '30'
  role: context_only
  excerpt: '# Evidence chain across IR, CG and QSL'
- id: context:spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '32'
  role: context_only
  excerpt: '# CG crate layout'
- id: context:spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  path: spec/replay/matrix/TC-047-routed-scalar-replay-binding.md
  lines: '11'
  role: context_only
  excerpt: '# TC-047: Check public routed scalar lowering replay and generated-content binding'
- id: AD-004-L-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  role: examined
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
- id: include:src/kani/generate/role_parity_tests.rs:5
  path: src/kani/generate/role_parity_tests.rs
  lines: '5'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/model.rs"]'
  included_target: tests/state_frame_support/model.rs
- id: include:src/kani/generate/role_parity_tests.rs:9
  path: src/kani/generate/role_parity_tests.rs
  lines: '9'
  role: examined
  excerpt: '#[path = "../../../tests/state_frame_support/native_twin.rs"]'
  included_target: tests/state_frame_support/native_twin.rs
- id: include:src/kani/test_support.rs:172
  path: src/kani/test_support.rs
  lines: '172'
  role: examined
  excerpt: '#[path = "../../tests/common/proof_ceilings.rs"]'
  included_target: tests/common/proof_ceilings.rs
- id: include:src/kani/test_support.rs:200
  path: src/kani/test_support.rs
  lines: '200'
  role: examined
  excerpt: '#[path = "../../tests/it/kani_scalar_native_digest.rs"]'
  included_target: tests/it/kani_scalar_native_digest.rs
- id: include:src/lib.rs:12
  path: src/lib.rs
  lines: '12'
  role: examined
  excerpt: '#[path = "../tests/it/kani_render_private.rs"]'
  included_target: tests/it/kani_render_private.rs
- id: include:src/lib.rs:16
  path: src/lib.rs
  lines: '16'
  role: examined
  excerpt: '#[path = "../tests/it/kani_scalar_native_private.rs"]'
  included_target: tests/it/kani_scalar_native_private.rs
- id: include:src/lib.rs:20
  path: src/lib.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "../tests/it/kani_terminal_private.rs"]'
  included_target: tests/it/kani_terminal_private.rs
- id: include:src/routed/contract_package_tests.rs:8
  path: src/routed/contract_package_tests.rs
  lines: '8'
  role: examined
  excerpt: '#[path = "../../tests/exact_function_support/package.rs"]'
  included_target: tests/exact_function_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:2
  path: tests/it/kani_scalar_native_private.rs
  lines: '2'
  role: examined
  excerpt: '#[path = "../exact_scalar_support/package.rs"]'
  included_target: tests/exact_scalar_support/package.rs
- id: include:tests/it/kani_scalar_native_private.rs:20
  path: tests/it/kani_scalar_native_private.rs
  lines: '20'
  role: examined
  excerpt: '#[path = "kani_scalar_native_fixture.rs"]'
  included_target: tests/it/kani_scalar_native_fixture.rs
- id: include:tests/exact_function_support/package.rs:23
  path: tests/exact_function_support/package.rs
  lines: '23'
  role: examined
  excerpt: '#[path = "../composite_equality_support/package.rs"]'
  included_target: tests/composite_equality_support/package.rs
- id: include:tests/exact_scalar_support/package.rs:41
  path: tests/exact_scalar_support/package.rs
  lines: '41'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:30
  path: tests/composite_equality_support/package.rs
  lines: '30'
  role: examined
  excerpt: '#[path = "../checked_package_support/rekey.rs"]'
  included_target: tests/checked_package_support/rekey.rs
- id: include:tests/composite_equality_support/package.rs:34
  path: tests/composite_equality_support/package.rs
  lines: '34'
  role: examined
  excerpt: '#[path = "codes.rs"]'
  included_target: tests/composite_equality_support/codes.rs
- id: L2:tests/composite_equality_support/package.rs:21
  path: tests/composite_equality_support/package.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:27
  path: tests/exact_function_support/package.rs
  lines: '27'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/exact_function_support/package.rs:80
  path: tests/exact_function_support/package.rs
  lines: '80'
  role: examined
  excerpt: 'result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,'
- id: L2:tests/exact_function_support/package.rs:526
  path: tests/exact_function_support/package.rs
  lines: '526'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:558
  path: tests/exact_function_support/package.rs
  lines: '558'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:571
  path: tests/exact_function_support/package.rs
  lines: '571'
  role: examined
  excerpt: 'operator: quire_contract_codegen::EqualityOperatorKind::Equal,'
- id: L2:tests/exact_function_support/package.rs:599
  path: tests/exact_function_support/package.rs
  lines: '599'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:625
  path: tests/exact_function_support/package.rs
  lines: '625'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:638
  path: tests/exact_function_support/package.rs
  lines: '638'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:651
  path: tests/exact_function_support/package.rs
  lines: '651'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:664
  path: tests/exact_function_support/package.rs
  lines: '664'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:677
  path: tests/exact_function_support/package.rs
  lines: '677'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_function_support/package.rs:836
  path: tests/exact_function_support/package.rs
  lines: '836'
  role: examined
  excerpt: 'operator: quire_contract_codegen::oracle::operator_catalog::IntegerOperator::Add,'
- id: L2:tests/exact_scalar_support/package.rs:24
  path: tests/exact_scalar_support/package.rs
  lines: '24'
  role: examined
  excerpt: use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
- id: L2:tests/exact_scalar_support/package.rs:25
  path: tests/exact_scalar_support/package.rs
  lines: '25'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/it/kani_scalar_native_fixture.rs:4
  path: tests/it/kani_scalar_native_fixture.rs
  lines: '4'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:21
  path: tests/state_frame_support/native_twin.rs
  lines: '21'
  role: examined
  excerpt: use quire_contract_codegen::{
- id: L2:tests/state_frame_support/native_twin.rs:39
  path: tests/state_frame_support/native_twin.rs
  lines: '39'
  role: examined
  excerpt: const fn unbounded_decode_limits() -> quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:40
  path: tests/state_frame_support/native_twin.rs
  lines: '40'
  role: examined
  excerpt: quire_contract_codegen::WitnessDecodeLimits {
- id: L2:tests/state_frame_support/native_twin.rs:871
  path: tests/state_frame_support/native_twin.rs
  lines: '871'
  role: examined
  excerpt: 'limits: quire_contract_codegen::WitnessDecodeLimits,'
- id: L2:tests/composite_equality_support/package.rs:32
  path: tests/composite_equality_support/package.rs
  lines: '32'
  role: examined
  excerpt: pub use rekey::FixtureIds;
- id: L2:tests/composite_equality_support/package.rs:36
  path: tests/composite_equality_support/package.rs
  lines: '36'
  role: examined
  excerpt: pub use codes::*;
- id: L2:tests/exact_function_support/package.rs:25
  path: tests/exact_function_support/package.rs
  lines: '25'
  role: examined
  excerpt: pub use ce::*;
- id: L2:tests/exact_scalar_support/package.rs:43
  path: tests/exact_scalar_support/package.rs
  lines: '43'
  role: examined
  excerpt: pub use rekey::FixtureIds;
- id: disposition-context:l2-finding-prescope.md
  path: /home/peter/dev/worktrees/kani-lead-codex/ir631-context/l2-finding-prescope.md
  lines: '1'
  role: context_only
  excerpt: '# PR481 inherited L2 finding — read-only scope assessment'
- id: disposition-context:l2-owner-handoff.md
  path: /home/peter/dev/worktrees/kani-lead-codex/ir631-context/l2-owner-handoff.md
  lines: '1'
  role: context_only
  excerpt: PR481 review SR-14220 FND-001 is inherited AD-004 L-2 debt, retained OPEN under the existing IR-348 layout
    owner. The finding is unchanged between reviewed base47c4a6adaaea4faa7bfe82b5eda6581aaff53897 and source22cf3a5e174c25eb6dacf5a01945e0a38691d5a7.
    PR481 only retains the actual original renderer ScalarLimits and does not claim full L-2 or full IR-631 acceptance.
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '832'
  role: context_only
  excerpt: '| FR-015-AC-119 | PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source authors an
    operation with one parameter `amount: Int[0, 1000]` and its V2 postcondition `amount < 1000 implies amount +
    1 <= 1000`; both occurrences of `amount` read that operation parameter, not a state field or subject-produced
    post-state value. The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and an FR-015
    postcondition harness for that same clause, not a scalar implication harness. Under FR-015-AC-39 the harness
    draws the parameter once within its declared finite domain an'
  static_status: untagged
  verification_method: Test
  evidence_status: no run evidence
- id: disposition-context:ac119-inherited-disposition.md
  path: /home/peter/dev/worktrees/kani-lead-codex/ir631-context/ac119-inherited-disposition.md
  lines: '1'
  role: context_only
  excerpt: PR481 current-head review after rebasing onto merged PR477/b64330c found the new FR-015-AC-119 has no
    tagged test or run evidence. This is newly inherited upstream planned scope, not a defect introduced by PR481's
    original-renderer-limits implementation. The actual computed whole-repository matrix now contains869 criteria,291
    untagged,510 tagged,18 ignored-only and1411 binder edges; the original criterion records are unchanged.
bindings:
- id: FR-015-AC-115
  path: src/kani/generate/render.rs
  lines: '333'
  role: examined
  excerpt: fn renderer_refuses_size_before_invalid_syntax_with_exact_byte_count() {
  symbol: tests::renderer_refuses_size_before_invalid_syntax_with_exact_byte_count
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1220'
  role: examined
  excerpt: fn scalar_port_is_differentially_identical_for_all_operations_and_boundaries() {
  symbol: tests::scalar_port_is_differentially_identical_for_all_operations_and_boundaries
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-015-AC-115
  path: src/kani/generate/scalar.rs
  lines: '1333'
  role: examined
  excerpt: fn scalar_port_preserves_resource_and_syntax_refusals() {
  symbol: tests::scalar_port_preserves_resource_and_syntax_refusals
  ac_id: FR-015-AC-115
  kind: test_function
  column: 5
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '202'
  role: examined
  excerpt: fn routed_draw_order_is_preserved_with_distinct_domains() {
  symbol: routed_draw_order_is_preserved_with_distinct_domains
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '241'
  role: examined
  excerpt: fn negate_and_literal_singleton_keep_every_draw() {
  symbol: negate_and_literal_singleton_keep_every_draw
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '283'
  role: examined
  excerpt: fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
  symbol: subtraction_decodes_signed_extreme_operands_without_evaluating_them
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '300'
  role: examined
  excerpt: fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
  symbol: inclusive_endpoints_and_adjacent_values_use_each_operands_range
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '417'
  role: examined
  excerpt: fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
  symbol: schema_then_byte_ceiling_then_decode_then_domain_precedence
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '441'
  role: examined
  excerpt: fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
  symbol: unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '476'
  role: examined
  excerpt: fn original_renderer_limits_are_retained_with_genuine_routed_source() {
  symbol: original_renderer_limits_are_retained_with_genuine_routed_source
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
- id: FR-032-AC-3
  path: tests/it/scalar_playback.rs
  lines: '519'
  role: examined
  excerpt: fn original_limits_refuse_changed_retained_identity_source_or_record() {
  symbol: original_limits_refuse_changed_retained_identity_source_or_record
  ac_id: FR-032-AC-3
  kind: test_function
  column: 1
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: 'UNFIXED inherited AD-004 L-2 defect. Deferred from the independently specified PR481 limits slice to
    existing IR-348 layout ownership, remaining In Progress: https://linear.app/agent-ix/issue/IR-348/refactor-cg-to-its-layout-ad-directories-cycles-helpers-newtypes-typed#comment-fc122bdc.
    Same-coder full prescope confirms at least ten repair surfaces plus shared-fixture callers and frozen PR397
    overlap. No direct-import/partial move resolves the complete include-graph finding; no waiver, alias, scanner
    relaxation, public test SPI, domain change or full-L2 claim. The actual source remains unchanged.'
  artifact_id: AD-004-L-2
  related_ids:
  - AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  lines: '842'
  excerpt: '- L-2. The import graph is acyclic and follows the dependency direction above, `#[cfg(test)]`'
  owner: AD-004 L-2 / IR-348 layout owner
  severity: medium
  confidence: high
  finding: The actual cfg(test) include graph violates L-2 through self-facade imports and nonroot re-exports; native_twin
    also imports replay into kani::generate.
  source_status: unchanged; not fixed
  original_classification: inherited baseline architecture debt; unchanged by PR481
  reaffirmed_round: 2
  previous_outcome: deferred
  current_status: UNFIXED and unaccepted; prior disposition carried, no fix claim
new_findings: []
qualification: Current frozen PR481 head b362bf5905b2a1ecc6cdcbe7b6422aacf1d006fc / actual base b64330c731079f9190158936f8b9b4d80d551a97.
  Independent current-head source review and whole computed matrix refreshed after controlled rebase. The exact
  seven-file limits delta and all169 Rust/Cargo/Makefile paths are byte-identical to original reviewed22cf; same
  typed value drives production emission/retention, all10limits remain u64MAX, identity/source/record mutation refuses,
  and no canonical proof-content preimage or Verified-only receipt change. No new original-limits code defect, new
  source gap, stub, gate weakening or included-graph change. Full L2 remains unfixed under IR348, full AC3/TC047/IR631
  remains unaccepted and IR940-gated. No original checked source/callsite/sentwire/Falsified native/QSL driver/converter
  completion is claimed. Earlier author logs were read at22cf, not rerun/current-head proof evidence. Root final
  CI and separate full Kani remain UNRUN.
coverage: 'Plan completion: not assessed

  Current full computed matrix:40 requirements,869criteria,1411binder edges;510tagged,291untagged,18ignored-only,50method-without-symbol.
  All869 have no bound run evidence. All868 prior matrix records exactly unchanged; the sole addition is FR015AC119
  untagged. Current gap findings312 (291untagged+18ignored-only+3real stale explicit attributes). Previous311 remain
  open/unaccepted; newFND312 is upstream planned normative scope under actual IR917 Done spec-delivery state. Library
  graph unchanged106files/14includes/20self-facade references/4nonroot reexports; no full-L2 claim. Whole code/test
  inventory remains158Rust files/745public headers including test helpers. Coverage engine diagnostics retained,
  no measured code coverage or optional exhaustive semantic-review claim. No new tag or test discharges clause-body
  AC119; existing scalar implies AC118 and renderer retention tests do not substitute.'
```
