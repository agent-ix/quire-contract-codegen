# Quire Contract Codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Development status

This crate is in its specification-first foundation phase. The requirements, assurance plan,
dependency pins, and retained-evidence procedure are reviewable now; semantic code generation is
intentionally deferred until the upstream IR corpus and runtime interfaces are reconciled.

The provisional dependency state is recorded in
[`planning/draft-dependency-pins.md`](planning/draft-dependency-pins.md). Draft work may advance
against those exact pins, but it does not establish compatibility or release readiness. Before
semantic implementation or release, the branch must be rebased and the pins reconciled with the
accepted upstream revisions.

## Local validation

```bash
make ci
```

This runs formatting, specification validation, Clippy, tests, license checks, and the unsafe-code
audit. CI workflows are manual-only; remote runs must be deliberately dispatched and retained when
they are used as evidence.

To collect a checksummed local foundation evidence bundle from a clean commit:

```bash
scripts/collect_foundation_evidence.sh
```

## Release boundary

The public API is not stable, registry publication is disabled, and no foundation artifact is a
source-release approval. Agent-assisted contributions remain subject to requirements traceability,
testing, provenance review, and the recorded human release decision.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
