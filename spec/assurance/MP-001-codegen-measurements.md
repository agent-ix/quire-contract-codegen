---
id: MP-001
title: Contract codegen v0.1 measurement plan
type: MeasurementPlan
status: proposed
owner: codegen-maintainers
metric: codegen_conformance_reproducibility_and_parity
definition_version: quire-contract-codegen.measurement-v2
stage: gate
statistical_design:
  population: every corpus package backend platform profile failure state and artifact kind
  sampling: exhaustive canonical fixtures plus seeded generated order and fault-injection variations
  repetitions: 3
  estimator: count
  error_model: platform toolchain backend fixture and coverage mapping differences
  uncertainty: retain unavailable skipped inconclusive unsupported and differential states
  decision_rule: the escalation count over all three repetitions equals zero
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: measures
---
# Contract codegen v0.1 measurement plan

## Decision Use

Measurements inform the human v0.1 source-release decision for one source candidate; they do not
approve release or confer validation, accreditation, or certification.

## Decision Rule

The metric is a `count` of escalations, and the rule holds only when that count is exactly zero: any
single occurrence fails it. The count is taken over all three repetitions together, so one occurrence
in any repetition is enough, and a clean repetition cannot be reported in place of one that observed a
failure. An escalation is one observed occurrence, in any population item, of any of these four
classes:

1. **regeneration drift**: an artifact whose bytes differ between repetitions from equal inputs
   within a declared supported profile;
2. **silent state**: an unsupported, inconclusive or failed outcome that is not reported explicitly,
   such as a construct or obligation silently dropped or approximated, an unsupported state reported
   as success (NFR-002-AC-3, FR-001-AC-4), or a producer row dropped without an error;
3. **parity mismatch**: generated output and the independently evaluated executable oracle
   disagreeing on a classification or diagnostic;
4. **partial publication**: a failed or interrupted publish that leaves the destination other than
   `unchanged`, complete, or reported `unknown` with both prior and staged bundles preserved.

Unavailable, skipped, inconclusive, unsupported and differential states are retained and reported
beside the count, as `statistical_design.uncertainty` says. Reported explicitly, they are not
escalations and are not counted; hidden or reported as success, they are a silent state. A count of
zero is therefore not a pass for them: they remain limitations, as the Interpretation section states.

## Population

The population is every public IR construct and positive/negative canonical fixture, all supported
executable/proptest/Kani/coverage backends, supported platforms, output types, diagnostics, dependency
states, and injected publication failure points. The Kani population includes direct Boolean and
bounded-`i64` input/current/pre/post bindings, all six integer comparisons, exact inclusive model
endpoints, immediately outside values, dependency-readiness states, and healthy/falsifying subjects.

## Collection Procedure

Campaign outcome controls execute in TC-004's generated crate tests. The library-only public
bound-package consumer has synthetic-projection tests and a byte-accounting unit control, including
actual publication and native execution of generated sources.

`cargo test --locked --target-dir target-codex-backends --test it kani_generation -- --test-threads=1`
is SUITE-008. It validates the v2 Rust/graph schemas, compiles the generated `publish = false`
crates, and runs exact harnesses under the installed Kani backend. The bounded scalar cases exercise
signed 0 through 1000 domains plus -1 and 1001 controls. An identity state subject must prove; a
changed-value subject and falsifiable integer comparison must fail and print concrete playback data
under `-Z concrete-playback --concrete-playback print`. The executable-oracle corpus is evaluated
independently so a shared renderer alone cannot establish parity. Kani results are test
observations; the generated graph's `proofExecutionState` stays `not_run`.

`quire coverage --scope . --json` is the static specification, obligation and coverage export. Quire
exports; it never executes a producer.

TC-006 has primitive and bound-observation tests, including a nested native LLVM fixture over the LLVM
full JSON export format with per-file segments. The analyzer consumes export bytes and never invokes
LLVM, Cargo, Quire, or Quoin. Mutation controls independently remove the evaluation hit, remove one of
several consequent hits, replace the export with summary-only output, and alter a source path. Native
producer/run binding and shared retained-result integration are incomplete; existing analysis cannot
discharge those obligations.

## Evidence Verification Control

Static specification, obligation and coverage facts are Quire's. This repository retains no evidence
of its own and computes no aggregate verdict.

A green `make ci` is a statement about the tree as committed. Make can be told to ignore failure, and
one line does it: `.IGNORE:` at the top of the file, a `-` prefix on a recipe line, or an assignment
to `SHELL` each make a recipe report success without its exit status being consulted. The decision
owner reviews the diff, not only the result; this is tracked as agent-ix/quire-contract-codegen#14.

## Interpretation

Byte-identical regeneration is required only within a declared supported profile. A missing backend,
draft dependency, unsupported fixture, inconclusive proof, or differential result remains a
limitation, not a pass.

Two limitations are load-bearing and are stated here rather than left to be inferred.

FR-003 Kani obligation lowering has reviewed Boolean and numeric/state implementations in the same
local suite. Its generated graph states `proofExecutionState: not_run`: dependency readiness is not
proof completion. Successful and failing cargo-kani observations qualify only the installation,
profile and options they ran under. Native counterexample replay is QSL's `qsl_replay::replay`
evaluation and cannot be claimed from codegen's generated source or Kani text alone. FR-004 has
bounded LLVM observation primitives, generated-oracle native controls, and complete bound observations
through `analyze_bound_coverage`; it has no campaign-run binding and no consuming obligation gate.
Neither backend has a shared proof obligation claiming completed assurance here. FR-003's TM-001 rows
are covered by SUITE-008 and its ticket-scoped review; FR-004's rows stay 🚧 Planned.

The atomic publisher's fault model injects refusal before every artifact write, before destination
swap, during replacement swap and failed rollback, and after commit before backup cleanup.
Initial-publication and successful-rollback failures must report `unchanged`; their destination, an
adjacent developer-owned file, and sibling staging census are checked. Failed rollback must report
`unknown` and preserve complete prior/staged bundles for recovery. I/O failures while inspecting the
destination must report `io_failed`, distinct from observed missing inputs. The post-commit control
requires `published`, requires the new bundle to be complete, and observes the backup residue. On
portable filesystems, replacing a non-empty directory requires two renames: rollback covers
pre-commit errors, but a process crash between those renames can temporarily leave only the sibling
backup. This boundary is retained as FND-905. Directory entries are not synchronized, and the
publisher makes no power-loss durability promise after a successful return.

This measurement plan supports neither a semantic implementation decision nor a release decision, and
confers no validation, accreditation, or certification.
