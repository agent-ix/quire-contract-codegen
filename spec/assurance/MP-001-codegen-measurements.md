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

Every generated artifact carries a proof-attestation body, and that body's conformance to the shared
shape is measured rather than declared. `tests/oracle_generation.rs` fetches
`proof-attestation-v1.schema.json` from `quoin change-assurance schema` — the bytes the sealing and
verification code was written against, never a local copy — seals each emitted body over the artifact
it accompanies through the real `quoin change-assurance seal-attestation`, and validates what comes
back against that schema. Three controls sit beside it, because the assertion alone would be
satisfied by a schema that accepts everything and by a sealer that hashes nothing: a sealed
attestation with its `record_digest` replaced by a non-digest must be rejected by the validator; one
appended byte in the retained output must move `retained_output.digest`; and a body that states
`retained_output` itself must be refused by Quoin, which is what makes the omission of that field and
of `digest` the shared contract rather than a local shortcut.

Two of the eleven fields a proof attestation declares come from the caller; the generator states the
other nine from what it can see. The two are the sealed record digest and the candidate revision, and
both are checked before any artifact exists: `record_digest` against the shared schema's own digest pattern,
and `candidate_revision` against a 40-to-64 character lowercase hexadecimal rule that is deliberately
stricter than the shared schema's `minLength: 1`. Each rule is probed on its own against a binding
that is valid apart from the one field under test, with a control requiring the fully valid binding
to be accepted; a single joint probe over several bad fields shows only that some rule fired.

The attestation's `result` is derived and not supplied. A bundle exists only when generation
succeeded, so `passed` is the only honest answer, and the deprecated envelope's caller-supplied result
status permitted a cleanly generated artifact to carry `rejected` or `timed-out`. The six
Interface-001 terminal states are unaffected: they live in the diagnostic that arrives instead of a
bundle, and no attestation ever accompanies one.

Every check the migration to the shared shape added or repointed was measured by introducing the
defect it exists to catch and observing it go red. Twenty-two defects, each landing on a different
assertion, and the list is written out so the count can be re-derived rather than taken on trust:

1. the attestation's schema version moved off `1`;
2. its record type moved off `proof_attestation`;
3. a body that states `retained_output` itself;
4. the output schema digest dropped from the attested command;
5. the IR revision dropped from it;
6. the backend discriminator dropped from it;
7. the output media type dropped from it;
8. the canonical profile misreported in it;
9. an input digest naming bytes other than the ones lowered;
10. a reviewer identity reintroduced through the environment map;
11. the record-digest half of the binding rule removed;
12. the candidate-revision half removed;
13. the deprecated format's name reintroduced in a Rust source file;
14. one of the deleted serde types reintroduced by name;
15. both attestations of one bundle claiming one path;
16. the caller's binding ignored in favour of a different constant;
17. the caller's binding ignored in favour of the constants its callers already use;
18. the attestation emitted for the other artifact of the bundle;
19. the derived result moved off `passed`;
20. an archive build supplying a recorded time that is not RFC 3339;
21. the harness body's schema version moved;
22. the strategy body's record type moved.

One of those probes was measured wrong the first time and is recorded here because the correction is
the interesting part. "The caller's record digest ignored" was injected by substituting a *different*
constant, which the conformance corpus caught on an equality check. An adversarial review then
substituted the *same* constants the corpus supplies — the all-zero digest and `IR_CANDIDATE_REVISION`,
both of which the generator already has — so the caller's binding was ignored completely and every
test in this repository stayed green with 9 of 9 corpus cases passing. A value check cannot see that.
What replaced it is differential: two generations with two different bindings, requiring exactly those
two fields to move and everything else to stay identical. That form fails for any mutant, whatever
constants it picks.

Each defect was **committed** before its gate was run, and that detail is the measurement rather than
an aside. `build.rs` reads `git status`, so an uncommitted edit sets `QUIRE_CODEGEN_SOURCE_DIRTY` and
TC-001 fails on its dirty-tree assertion before reaching the assertion under test. A first pass of
this exercise left the edits in the working tree and reported four defects "caught" by a check that
had nothing to do with any of them.

Two of these gates were also observed red by accident before they were finished, which is worth
recording because it is stronger evidence than a deliberate probe: naming the deprecated format in a
doc comment in `tests/shared_assurance.rs`, and then again in one in `tests/oracle_generation.rs`,
both turned the reference census red on the first run after the name was added to it.

The adapter's undecodable-bytes arm is measured rather than assumed. A probe truncates a real row of
the real producer stream mid-object, calls the real adapter, and requires it to raise an error naming
the truncated line — pre-adapter, because that is the only point at which the bytes have not yet been
mapped onto a coarse result. It counts no verification state: a refusal is the adapter declining to
produce one. Its two failure directions were introduced and observed to turn it red, one where the
adapter drops the row silently and one where it raises without naming the line.

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
