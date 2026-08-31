# quire-contract-codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Commands

```bash
make fmt            # format with rustfmt
make fmt-check      # verify formatting (CI gate)
make lint           # clippy with -D warnings
make test           # cargo test
make build          # release build
make msrv           # compile the library with exact Rust 1.75.0
make spec           # validate specification, planning, and typed-plan documents
make clean          # cargo clean
make deny           # cargo deny check licenses
make audit-unsafe   # check that every unsafe block has a // SAFETY: comment
make rustdoc        # build warning-free API documentation
make coverage       # reject coverage-status contradictions; report unavailable classification
make evidence-tool  # test the evidence builder, validators, verifier, and ownership controls
make verify-evidence # verify the anchored authoritative and historical evidence set
make ci             # every local gate above except build, clean, and cargo-audit
```

Evidence collection requires a completely clean Git worktree, including no untracked files. The
collector refuses to run until work is committed or moved outside the checkout; this makes the
recorded source revision an exact description of every input under test.

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- This unsafe-audit script intentionally strengthens the shared seven-repository version by scanning tests, benches, and examples and emitting a positive completion marker; the shared policy owner should upstream those differences.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.

## Layout

```
src/lib.rs             # crate root
tests/integration.rs   # end-to-end tests
benches/               # criterion benchmarks (opt-in; add criterion to dev-deps)
spec/                  # requirements artifacts (from /spec-create-spec)
scripts/               # local tooling
```
