# quire-contract-codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # locked clippy with -D warnings
make test             # locked cargo test
make build            # locked release build
make msrv
make spec             # Quire-validate the specification, plan and review documents
make clean            # cargo clean
make tools            # once per machine: llvm-tools (stable), cargo-llvm-cov and Kani, which the test suite drives
make deny             # cargo-deny dependency policy checks
make use-local        # patch first-party git deps to sibling checkouts via a gitignored .cargo/config.toml; snapshots Cargo.lock to .cargo/Cargo.lock.pre-local; fails if cargo metadata fails or a patch is unused
make use-remote       # delete the patch config and restore Cargo.lock from that snapshot (no snapshot: lock untouched)
make rustdoc          # build warning-free API documentation
make ci               # every local gate above except build and clean
```

`LOCKED` (default `--locked`, empty while `.cargo/config.toml` exists) is passed to the cargo
targets that resolve dependencies (`lint`, `test`, `build`, `msrv`, `kani`, `rustdoc`); override
it, e.g. `make lint LOCKED=`, when a patch or a deliberate lock change makes `--locked` wrong.

## Bounded Kani execution

Today the bounded execution API requires Linux, readable procfs RSS and task `children` files
(`CONFIG_PROC_CHILDREN`), pidfd support, and bubblewrap (`bwrap`) supporting `--unshare-user`,
`--unshare-pid`, `--info-fd` and `--block-fd`, and the host must permit user and PID namespaces
(AppArmor can deny this). `make tools` installs the Rust and Kani tools, not bubblewrap. Missing or
denied support returns `MemoryMechanismUnavailable` before backend dispatch. A caller killed before
the startup gate is armed can leave the backend running.

Planned (ADR-007, IR-1510; not yet in the code): the runner will start Kani as the leader of its own
process group, keep a wall-clock timeout that kills the group, and report peak memory only when the
environment provides it, with any memory cap left to the caller's environment (for example
`systemd-run -p MemoryMax=16G`). That removes the namespace, bubblewrap, procfs and pidfd
requirements and runs on Linux and macOS (FR-017-AC-65 to AC-68, FR-028-AC-21).

## Safety scaffolding

- `clippy.toml` caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- The root crate opts into the workspace's denied Clippy `undocumented_unsafe_blocks` lint; `make lint` and CI enforce it.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.

## Layout

```
src/lib.rs                 # crate root
tests/it/main.rs           # the merged integration test binary; each former tests/*.rs file is a mod here
schemas/                   # domain output contracts included by their owning library producers
spec/                      # spec.md (Subsystem Registry), tests.md (matrix index), <subsystem>/{stakeholder,functional,non-functional,matrix}/ per quire-contract-ir:ADR-0056; the suite registry is spec/core/matrix/suites.md
reviews/                   # quire-validated SpecReview artifacts
scripts/                   # dependency gate data and one-copy check
scripts/cg_gates.rs        # Rust Kani scope/gate and QSpec preflight selftest
```

New integration tests go under `tests/it/` as a module of the single `it` binary (add the file plus a
`mod <name>;` line in `tests/it/main.rs`), not as a new top-level `tests/<name>.rs` -- Cargo links and
starts a whole new process per top-level `tests/*.rs` file, which is exactly what `it` exists to avoid
(IR-237). A file only stays a separate `[[test]]` target when it genuinely needs its own process
(racy process-global state, or a subprocess assumption of being the sole executable); none of the
current files needed that.
