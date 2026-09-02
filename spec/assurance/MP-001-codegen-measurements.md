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

One target runs producers: `make assurance-inputs`. Everything downstream consumes the files it
writes and refuses to create them, because a consumer that can produce its own input can produce a
green run out of nothing.

Five producers publish declared structured results.

`cargo run --example generation_conformance` walks the bounded generation corpus and publishes
`codegen.generation-conformance/v1`: one row per case, carrying the case's outcome, the Interface-001
terminal state it reached, the diagnostic code it produced, the number of declared checks that held,
and the floor those checks must meet. A case that holds every check it ran but runs fewer than its
floor is `vacuous`, not `pass` — a corpus can go green by getting smaller, and the floor is what
stops that. The corpus covers the oracle, harness and strategy slices and the rejection cases that
keep `unsupported` and `invalid-input` apart, and a census row reports how many distinct diagnostics
and terminal states the corpus actually reached.

`scripts/check_upstream_pins.py --json` publishes `codegen.upstream-identity/v1`: for each declared
upstream, the revision the crate's own constant states, the revision the dependency declaration pins,
and the revision the lockfile resolved. The lockfile value is read from that package's own stanza
rather than by searching the file for the manifest's answer, because a search for the answer can only
confirm it.

`quire coverage --scope . --json` is the authoritative static specification, obligation and coverage
export. Quire exports; it never executes a producer.

`rustup run 1.75.0 cargo check --locked --all-targets --message-format=json` is the MSRV build, whose
verdict is read from cargo's own `build-finished` message rather than from its transcript.

`scripts/assurance_chain.py` drives the official chain over those files. It projects
`assurance/change-assurance.json` into Quoin's FR-063 record body, deriving only the digests that
file's own `derived_fields` names; seals one FR-064 proof attestation per obligation, stating the
result read out of the producer's bytes; hands those exact bytes to Quoin's intake; and asks for an
FR-065 verification receipt. It runs `quoin` and nothing else.

## Evidence Verification Control

Retention, integrity checking, audit, attestation and receipts are Quoin's. Static specification,
obligation and coverage facts are Quire's. This repository retains no evidence of its own and
computes no aggregate verdict.

Two things are measured rather than asserted.

The execution boundary. `tests/shared_assurance.rs` runs the chain three times: once with every
producer replaced by a logging stub, requiring the log to be empty; once with `quoin` stubbed,
requiring the chain to fail and the log to be non-empty; and once with `quire` stubbed, requiring
every request made of Quire to be a static read. The second run is the control — without it, an empty
log in the first is equally consistent with `PATH` never being consulted at all.

The declared command. Every proof obligation's declared argv must appear verbatim in
`make -n assurance-inputs`. A declared command that is not the executed command is a lie inside a
sealed attestation, and it is the kind of lie nothing downstream can catch, because Quoin records
what the caller says the command was.

## Qualification Integrity

A green `make ci` is a statement about the tree as committed. It is not a statement about a tree whose
Makefile has been edited.

Make can be told to ignore failure, and one line does it: `.IGNORE:` at the top of the file, a `-`
prefix on a recipe line, or an assignment to `SHELL` each make a recipe report success without its
exit status being consulted. The 334-line recipe-failure policer that used to catch this went with
the collector it was protecting.

Measured on this repository at this candidate revision, not assumed and not
inherited from a sibling. Three defects were injected — a rustfmt violation in
`src/lib.rs`, a failing assertion in `tests/integration.rs`, and an upstream
constant name the crate does not declare — and `make ci` was run twice.

| | control | `.IGNORE:` prepended |
| --- | --- | --- |
| `make ci` exit status | **2** | **0** |
| where it stopped | `fmt-check`, the first of eleven prerequisites | nowhere; all eleven ran |
| recipes that failed | 1 observed (the other ten never ran) | **7**: `fmt-check`, `spec`, `lint`, `msrv`, `upstream-identity`, `test`, `assurance-chain` |
| recipes that failed the build | 1 | **0** |

Every one of those seven printed its own diagnostic. None of them changed the
exit code. `assurance-chain` is in that list and it is the interesting entry: the
chain did detect the injected defect — the upstream producer's rows became
`not-computed`, the attestation said so because the bytes said so, and the
`attested-results-are-read-from-producer-output` scenario and its paired control
both went red for exit 1. Make swallowed it anyway.

What that reaches and what it does not. Quoin binds retained inputs by digest and the chain derives
every attested result from the producer's own bytes, so a Makefile that lies about running a producer
yields an absent or unreadable input and the chain errors rather than passing. The gates that feed
nothing into the chain are simply neutered. `tests/shared_assurance.rs` asserts that the committed
Makefile declares no such directive, which protects a reviewer reading a diff; it does not make this
file's exit code trustworthy on a tree where it has been edited, because under `.IGNORE:` that test
also runs, also fails, and is also swallowed. The residual is recorded rather than closed, and
tracked as agent-ix/quire-contract-codegen#14.

## Interpretation

Exact digest equality is required only within a declared supported profile. A missing backend, draft
dependency, unsupported fixture, inconclusive proof, or differential result remains a limitation, not
a pass.

Two limitations are load-bearing and are stated here rather than left to be inferred.

FR-003 and FR-004 — Kani obligation lowering and vacuity evidence — are specified and have no
implementation at this revision. There is no suite for them and no proof obligation over them,
because a proof obligation whose subject does not exist is the most complete false green available.
Their TM-001 rows stay 🚧 Planned.

The shared verification vocabulary is twelve states and this repository demonstrates ten. The two it
does not are `unsupported` and `malformed`. Measured on the tree before anything was deleted, per
state and per source: the assurance chain alone demonstrated ten, and the compatibility census over
the retained `evidence/` tree supplied exactly those two. The repository owner released the
preservation constraint for the pre-stable phase on 2026-09-02
(`agent-ix/engineering-assurance#7`), those records are deleted, and both claims are withdrawn with
them.

`unsupported` is not in the assurance chain's producer outcome vocabulary and never was; it was
raised only by the compatibility mapping, against a retained record carrying an unknown PGM-01 schema
version. The generation corpus reaches an `unsupported` Interface-001 terminal state, which is a
different vocabulary on a different axis and is not borrowed to fill the gap.

`malformed` is in that vocabulary and was still withdrawn, because a declared key is not a
distinguishable state. The adapter maps `malformed` onto the same `fail` in both of its tables, so a
scenario feeding it a stream declaring that outcome produces receipt reasons byte-identical to the
`fail` case, and the chain's own anti-collapse scenario could not have included it. Such a scenario
was written, measured against the `fail` case, found to be that case under another name, and removed.

TC-012 asserts both states stay absent. That assertion, not a manufactured demonstration, is what
stops the gate weakening quietly: re-acquiring either state goes red and has to be argued for.

This measurement plan supports neither a semantic implementation decision nor a release decision, and
confers no validation, accreditation, or certification.
