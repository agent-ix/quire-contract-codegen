---
id: SUR-001
title: "Contract codegen v0.1 test suites"
type: SuiteRegistry
---

# Contract codegen v0.1 test suites

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-003 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` | quire | Analysis |
| SUITE-004 | Static specification and coverage export | `quire coverage --scope . --json` | quire | Static |
| SUITE-007 | Minimum supported Rust version build | `make msrv` | rustc | Static |
| SUITE-008 | Bounded Kani generation and execution | `cargo test --locked --target-dir target-codex-backends --test it kani_generation -- --test-threads=1` | cargo-kani / rustc | Analysis |
| SUITE-010 | Atomic generated-boundary publication | `cargo test --lib publication` | quire-contract-codegen / rustc | Integration |
| SUITE-011 | Kani obligation execution | `make kani` | cargo-kani / rustc | Analysis |
| SUITE-012 | Real-Kani lane gate (planned: NFR-006) | `make kani-gate` | cargo-kani / rustc | Analysis |
