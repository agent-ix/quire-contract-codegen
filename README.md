# Quire Contract Codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Development status

This crate is in its specification-first foundation phase. The requirements, assurance plan,
dependency pins, and shared-assurance intake procedure are reviewable now; semantic code generation is
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

This runs formatting, specification/plan validation, Clippy, tests, an explicit Rust 1.75
compatibility check, license checks, and the unsafe-code audit. CI workflows are manual-only; remote
runs must be deliberately dispatched and retained when they are used as evidence.

This repository retains no evidence of its own. `make assurance-inputs` runs the producers,
`make assurance-chain` seals, retains and verifies their output through Quoin, and nothing in between
computes a verdict. `assurance/README.md` is the guide.

The block that used to stand here invoked `scripts/collect_foundation_evidence.sh`. That collector was
deleted on the shared-assurance migration branch and squash-merged as `bbd5e67`
([#13](https://github.com/agent-ix/quire-contract-codegen/issues/13)), so the script has never existed
at any commit reachable from `main` while this instruction was in the README — the squash introduced
the reference already dangling. No census would have caught it: the reference scan in
`tests/shared_assurance.rs` does not read Markdown, and the collector's name is not in its
forbidden-name list either, so the file's absence is asserted while references to it are not.

## Generated artifacts

Every generated artifact is emitted with a proof attestation carrying Quoin's packaged
`ProofAttestationV1` shape — one attestation per artifact, because an attestation binds exactly one
retained output. The emitted body is that schema without `digest` and without `retained_output`:
`quoin change-assurance seal-attestation` derives both from the retained bytes and refuses a body that
supplies either, so the generator produces and Quoin seals.

The two documents under `schemas/` are domain output contracts for the generated Rust and the
generated source map. They describe the artifacts themselves, not evidence about them, and this
repository owns no evidence schema.

## Release boundary

The public API is not stable, registry publication is disabled, and no foundation artifact is a
source-release approval. Agent-assisted contributions remain subject to requirements traceability,
testing, provenance review, and the recorded human release decision.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
