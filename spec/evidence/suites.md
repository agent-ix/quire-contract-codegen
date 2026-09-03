---
id: SUR-001
title: "Contract codegen v0.1 evidence suite registry"
type: SuiteRegistry
---

# Contract codegen v0.1 evidence suite registry

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-001 | Bounded generation conformance corpus | `cargo run --quiet --example generation_conformance` | quire-contract-codegen 0.1.0 / rustc | Integration |
| SUITE-002 | Upstream identity agreement | `python3 scripts/check_upstream_pins.py --json` | quire-contract-codegen 0.1.0 | Static |
| SUITE-003 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-004 | Static specification and coverage export | `quire coverage --scope . --json` | quire 0.31.0 / quire-rs 0.46.0 | Static |
| SUITE-006 | Shared assurance intake chain | `python3 scripts/assurance_chain.py --candidate-revision <sha>` | quoin 0.23.1 change-assurance and evidence surfaces | Integration |
| SUITE-007 | Minimum supported Rust version build | `rustup run 1.75.0 cargo check --locked --all-targets --message-format=json` | rustc 1.75.0 | Static |
| SUITE-010 | Atomic generated-boundary publication | `cargo test --lib publication` | quire-contract-codegen 0.1.0 / rustc | Integration |

## Notes

This registry is new with the shared-assurance migration. Before it, the retained
records named their commands only inside a collection manifest, which meant the
suite a result discharged an obligation for was a fact recoverable only by
reading the collector's shell script. Those records are deleted under the
pre-stable preservation release (`agent-ix/engineering-assurance#7`); this
registry is now the only place a suite's command is declared, which is where it
should have been.

SUITE-005 was the retained-evidence compatibility view. It is removed with the
records it read. The identifiers of the remaining suites are not renumbered: a
suite identifier that changes meaning is worse than a gap in a sequence.

SUITE-001 is the suite whose run this repository transcribes into Quoin's
evidence store. It is the crate's headline verification — the bounded generation
corpus over the oracle, harness and strategy slices, with the rejection cases
that keep the Interface-001 terminal states apart — and its rows carry trace
bindings the corpus declares per case.

SUITE-003, SUITE-004 and SUITE-006 were previously performed by the deleted
collector, which conflated schema validation and envelope conformance in one
lane and added the local traceability reimplementation and the local verifier. Every one of those concerns moved
upstream. Quire is the authority on static specification, obligation and coverage
facts; Quoin owns intake, retention, audit and receipts.

There is no suite for FR-003 or FR-004. Kani obligation lowering and vacuity
evidence are specified and not implemented at this revision, and a suite whose
command exercises nothing would report `pass` for a capability that does not
exist. Their absence from this table is the honest statement; TM-001 keeps their
rows 🚧 Planned.

SUITE-010 exercises deterministic bundle identity, every injectable staging and swap boundary,
complete ownership-census verification, and refusal of modified, extra-entry, unmarked, and symlinked
destinations. It is local pre-review evidence for the publication portion of TC-002; the
serialized-package CLI remains blocked on an IR expression-binding design.

`make ci` is deliberately not a suite. A suite whose command is "everything"
cannot say which obligation a result discharged, and `make ci` is a gate rather
than a producer of transcribable results.
