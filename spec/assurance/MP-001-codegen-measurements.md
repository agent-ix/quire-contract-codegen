---
id: MP-001
title: Contract codegen v0.1 measurement plan
type: MeasurementPlan
status: proposed
owner: codegen-maintainers
metric: codegen_conformance_reproducibility_and_parity
definition_version: quire-contract-codegen.measurement-v1
stage: gate
statistical_design:
  population: every pinned corpus package backend platform profile failure state and artifact kind
  sampling: exhaustive canonical fixtures plus seeded generated order and fault-injection variations
  repetitions: 3
  estimator: exact digest equality diagnostic counts parity classifications and atomicity outcomes
  error_model: platform toolchain backend version fixture provenance and coverage mapping differences
  uncertainty: retain unavailable skipped inconclusive unsupported and differential states
  decision_rule: escalate any digest drift silent state parity mismatch partial publication or missing identity
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: measures
---
# Contract codegen v0.1 measurement plan

## Decision Use

Measurements inform the human v0.1 source-release decision for one pinned candidate; they do not
approve release or confer validation, accreditation, or certification.

## Population

The population is every public IR construct and positive/negative canonical fixture, all supported
executable/proptest/Kani/coverage backends, supported platforms, output types, diagnostics, dependency
states, and injected publication failure points.

## Collection Procedure

Foundation runs use `scripts/collect_foundation_evidence.sh`. They record the current toolchain and
the explicit Rust 1.75 compatibility lane, validate requirements plus the typed plan bundle, and
retain exact accepted or explicitly pending dependency identities. Every invoked foundation gate retains stdout,
stderr, and its numeric exit status; the builder derives outcomes from those records, represents
missing records as inconclusive, and rejects zero-status records whose retained transcripts contain
command-specific failure markers. The collector self-test exercises nonzero propagation and checksum
fixed-point detection; the builder never manufactures a pass from a command name. The foundation
builder derives and verifies the digest of the vendored PGM-01 envelope schema; unit tests exercise
dependency-pin agreement, envelope identities, roles, digests, extensions, pin mismatch failure,
outcome truthfulness, and accepted/rejected local schema validation. These tests establish
evidence-tooling behavior only and do not back a semantic TestMatrix row.
Every zero-exit transcript also requires command-specific positive corroboration. Formatting uses an
explicit `FMT_CHECK_PASSED` marker after `cargo fmt --check`; Rust test transcripts must report at
least the branch's complete 10-test census, the Python runner must report at least 60 passed tests,
and every positive transcript must retain at least 16 bytes. The byte floor catches empty or
truncated records; command-specific markers and parsers provide the substantive discrimination.
These floors are defined by `scripts/evidence_policy.py` and mutation-tested, so a reduced transcript
is inconclusive.
`scripts/run_python_tests.py` recursively loads and executes every `unittest` case, including cases in
nested non-package directories, and fails if zero tests execute. Implementation plans will extend this with a stable
candidate runner that records source and dependency revisions, tool/backend versions, configuration,
corpus/input digests, repeated bundle digests, compile/proptest/Kani/coverage results, fault-injection
outcomes, differential dispositions, and output digests beneath `evidence/`.

## Evidence Verification Control

`scripts/verify_foundation_evidence.py` is the independent retained-record verifier. It requires the
out-of-record `evidence/ANCHORS` census, deterministically maintained by
`scripts/update_evidence_anchors.py`, verifies each anchored checksum manifest and every nested
historical/remote file, rejects added files and directories, re-derives every outcome value from its
numeric status plus retained transcripts, re-derives the envelope result, checks manifest and
envelope artifact links, verifies the complete manifest artifact census and exact limitations,
re-derives the parameter digest from the controlling source and Python test files, and binds the
vendored PGM-01 schema to the digest retained from the external checkout. It also re-runs and exactly
compares all seven retained tool identities (Cargo, rustc, MSRV rustc, Python, jsonschema, installed
Python packages, and Quire provenance) and re-derives the command declaration from the collector.
The legacy `gateScripts` field contains 11 parameter/control files, including the unsafe-comment
baseline data file. Its live-tree digest comparison is a redundant binding already implied by the
complete parameter digest; it is not claimed as an independent implementation or an executable-gate
census.
Behavioral mutation tests
cover outcome derivation and census, parameter and artifact census, result status and summary,
limitations, revision availability, symlink refusal, historical disposition, and availability
enumeration. A missing anchor or empty authoritative record set is
verification unavailable, not success. An anchor is a committed review boundary, not proof that the
originally retained bytes were semantically correct.

Collection verifies the new record directly but does not rewrite `evidence/ANCHORS`. Updating the
anchor census and running the complete verifier are separate, explicit review-boundary steps before
the evidence commit.

Every quarantined envelope carries a machine-readable `historicalDisposition` extension whose
`retracted` status removes any embedded historical result from the authoritative claim set. The
verifier requires that exact disposition on every historical envelope; the recursively anchored
historical tree prevents it from being removed without detection.

`scripts/validate_json_schema.py` fails closed unless every package in
`requirements-evidence.txt` and every required format checker is present. The collector records the
installed package set. `scripts/check_coverage_status.py` gates contradicted status rows and ignored
trace-bearing Rust tests, rejects matrix acceptance/test identifiers absent from the minted trace
population, and treats every reported diagnostic reason as inconclusive. While upstream status-column
classification remains unavailable it emits an explicit inconclusive marker, so a zero process exit
cannot become a passed evidence outcome. The foundation collector accepts that one disclosed pending
coverage outcome as a usable pending record, but never relabels it as conclusive.
`scripts/check_unsafe_comments.sh` owns the unsafe-code census, and
`scripts/build_foundation_envelope.py` owns deterministic outcome/result derivation and assembly.
`scripts/check_failure_propagation.py` pins trusted local tool identities, rejects Make execution
modifiers that suppress work, verifies the exact composite prerequisite census, substitutes a
failing command at every mandatory recipe position, and requires substantive output from the
coverage, Python-test, and retained-evidence verification entry points.
The parse-time Make control rejects dry-run, touch, and ignore-error modes, included Makefiles, and
command-line or in-file `MAKEFLAGS` overrides while explicitly allowing
parallelism and load-limit flags (`-j`/`--jobs`, `-l`/`--load-average`, and jobserver descriptors).
The local unsafe-comment audit is a strengthened fork of the seven-repository baseline: it scans
tests, benches, and examples in addition to `src`, retains a reviewed baseline, and emits an
explicit success marker. Those improvements should be upstreamed by the shared policy owner.

## Interpretation

Exact digest equality is required only within a declared supported profile. A missing backend, draft
dependency, unsupported fixture, inconclusive proof, or differential result remains a limitation, not
a pass. Foundation evidence cannot support a semantic implementation or release decision.
