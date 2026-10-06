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

## Bounded Kani execution

Linux bounded execution requires an explicit `guardian_path` naming this package's
`quire-kani-guardian` executable. C internally captures original inherited stdin before any per-run
descriptor creation; callers supply no stdin request field.
The helper authenticates the actual compiled library artifact. A separately compiled or stale
helper is refused even when its source is identical.

Library consumers deliberately build the helper from their **consumer manifest**, selecting both
the consumer package and this dependency so Cargo uses the same normal library compilation. For
example, a consumer package and executable named `caller` builds both with:

```bash
cargo build --manifest-path caller/Cargo.toml \
  -p caller -p quire-contract-codegen \
  --bin caller --bin quire-kani-guardian
```

Use the same target directory, target, profile, compiler, features and compiler flags for delivery.
Forward dependency features through a feature declared on the consumer, and select that consumer
feature in this command. Dependency compilation alone does not deliver the helper executable.
Rebuild both after changing those inputs; Cargo may reuse the existing library artifact when the
inputs are unchanged. Supply the resulting helper path directly in `KaniExecutionRequest`.

`guardian-test-support` is test-only, off by default, and exposes one operation,
`observe_guardian_fixture`. It selects the shared executor's exact startup prefixes or records
live-caller lease-close facts before immediate ordinary cleanup. It exports no lease, process
handle, cancellation API or cleanup-deferring callback. The packaged
`quire-kani-caller-fixture` uses the ordinary public execution API without that feature; explicitly
enabling it additionally permits the single observation operation. The harness judges immutable
raw facts; eventual emergency cleanup cannot make an earlier observation pass.

Keep verification invocations separate: **guardian-feature-off** builds the normal library,
helper and caller without default features; **guardian-feature-on** explicitly selects
`guardian-test-support` for all three. These names describe invocations, not Cargo profiles or
additional target directories. Build all participating packages and binaries from the consumer
manifest as above, forwarding the feature through the consumer. A self dev-dependency must not
unify the feature into the feature-off configuration. Mismatched feature builds refuse Dispatch.
The feature-off configuration must also verify that the operation is unavailable at compile time.

QSL production-driver dependency-edge exclusion belongs to
[IR-649](https://linear.app/agent-ix/issue/IR-649): every production build profile must reject
direct or transitively unified `guardian-test-support`. Publishing this CG contract does not verify
that downstream gate. The known unclaimed Bootstrap and inherited parent-death artifact-unlink
gaps remain gated on [IR-652](https://linear.app/agent-ix/issue/IR-652); process-only fixture
observations establish neither complete Bootstrap cleanup nor report removal.

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
