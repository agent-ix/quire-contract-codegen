# Quire Contract Codegen

[![Discord](https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white)](https://discord.gg/k8DVhuYBR2)

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Development status

This crate is pre-release. Dependency revisions are whatever `Cargo.toml` and `Cargo.lock` name.

## Local validation

```bash
make ci
```

This runs formatting, specification/plan validation, Clippy, the test suite under the minimum
supported Rust version, license and source checks, the unsafe-code audit, and the API documentation
build. CI workflows are manual-only.

## Generated artifacts

Every generated artifact is a path and its contents. The documents under `schemas/` are domain
output contracts for those artifacts:

- `generated-rust-oracle-v1.schema.json` and `oracle-source-map-v1.schema.json`: generated oracle
  Rust and its source map.
- `generated-rust-kani-v2.schema.json` and `kani-proof-graph-v2.schema.json`: generated Kani Rust
  and its proof graph.
- `kani-corpus-proof-graph-v1.schema.json`: the bounded Kani corpus proof graph.
- `bound-coverage-observations-v1.schema.json`: bound coverage observations.

## Release boundary

The public API is not stable, registry publication is disabled, and no foundation artifact is a
source-release approval. Agent-assisted contributions remain subject to requirements traceability,
testing, and the human release decision.

## License

Licensed under the GNU Affero General Public License, version 3 or (at your option) any later
version (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).

Rust source emitted by the generator carries its own `SPDX-License-Identifier: MIT OR Apache-2.0`
header, as NFR-002 requires; that identifies the generated output, not this crate.
