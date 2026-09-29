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
| SUITE-003 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-004 | Static specification and coverage export | `quire coverage --scope . --json` | quire 0.31.0 / quire-rs 0.46.0 | Static |
| SUITE-007 | Minimum supported Rust version build | `rustup run 1.98.1 cargo check --locked --all-targets --message-format=json` | rustc 1.98.1 | Static |
| SUITE-008 | Bounded Kani generation and execution | `cargo test --locked --target-dir target-codex-backends --test it kani_generation -- --test-threads=1` | cargo-kani / rustc | Analysis |
| SUITE-010 | Atomic generated-boundary publication | `cargo test --lib publication` | quire-contract-codegen 0.1.0 / rustc | Integration |
| SUITE-011 | Kani obligation execution | `make kani` | cargo-kani / rustc | Analysis |

## Notes

SUITE-001 is the bounded generation corpus over the oracle, harness and strategy slices, with the
rejection cases that keep the Interface-001 terminal states apart.

SUITE-008 exists for the implemented FR-003 draft. It validates both output
schemas, checks every dependency classification and source-site edge, compares
the embedded predicates with the executable-oracle output, and runs representative
bundles under the installed Kani backend. The numeric/state increment adds v2 typed subject bindings,
IR-owned integer assumptions, exact boundary/outside controls, and successful plus
falsifying concrete-playback runs without changing the generation-time `not_run`
classification. Its issue #2 current-head Rust review and gap analysis accept this local evidence;
the suite still does not classify graph readiness as a completed proof. FR-004 still has no suite:
`src/vacuity.rs` and `src/bound_coverage.rs` implement primitives and bound observations with focused
TC-006-tagged tests, but no qualified native-campaign result producer is registered. The bound
analyzer's observations are constructed with `provenance: "unqualified"` unconditionally.

SUITE-010 exercises deterministic bundle identity, every injectable staging and swap boundary,
failed-rollback recovery, distinct ownership I/O failures, interior-dot path refusal,
complete ownership-census verification, and refusal of modified, extra-entry, unmarked, and symlinked
destinations. It is local pre-review evidence for the publication portion of TC-002; the
serialized-package CLI remains blocked on an IR expression-binding design. The suite command is a
focused local check; the full repository test target also includes these tests, but SUITE-010 has no
separate structured execution-result producer yet.

SUITE-011 is the FR-015 and FR-017 obligation lane. `make kani` runs `tests/it/kani_obligations.rs`'s
`#[ignore]`d lane serially under a host-wide lock and in its own target directory, because Kani and
CBMC are memory-heavy. It verifies the precondition, postcondition and invariant harnesses of a healthy subject,
falsifies a seeded defect with a concrete counterexample, reports a jointly unsatisfiable contract as
cover-unsatisfied rather than verified, and refuses a crate that does not contain its harness. It is
deliberately not a `make ci` gate: it needs a real Kani installation,
which is why the FR-017 rows it alone backs are `🚧 Planned`. SUITE-008 is the FR-003 lane and does
not cover this one; that gap is agent-ix/quire-contract-codegen#66.

`make ci` is deliberately not a suite. A suite whose command is "everything"
cannot say which obligation a result discharged, and `make ci` is a gate rather
than a producer of transcribable results.
