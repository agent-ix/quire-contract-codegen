# quire-contract-codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # locked clippy with -D warnings
make test             # locked cargo test
make build            # locked release build
make msrv             # execute all tests with exact Rust 1.98.1
make spec             # Quire-validate the specification, planning, plan and review documents
make clean            # cargo clean
make deny             # all configured cargo-deny lanes
make audit-unsafe     # check that every unsafe block has a // SAFETY: comment
make rustdoc          # build warning-free API documentation
make conformance      # run the bounded generation conformance corpus
make ci               # every local gate above except build and clean
```

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.98.1` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. A line whose every occurrence is written `"unsafe {`, the token immediately preceded by a double quote, is a mention rather than a block and is not audited: this repository generates Rust and asserts properties of the generated text, so `"unsafe {"` appears as data in the tests that forbid unsafe code in generated output. The test is quote-prefixing, not string-literal membership, and only the exact spelling `"unsafe {` counts: a mention with a leading space, one mid-literal, or one inside a multi-line expected-output fixture is still audited and needs a baseline entry. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`. A tree with no Rust source roots, or a scan that fails, exits 2 as inconclusive rather than printing "unsafe audit passed"; the earlier version reported success when it had nothing to audit.
- This unsafe-audit script intentionally strengthens the shared seven-repository version by scanning tests, benches, and examples and emitting a positive completion marker; the shared policy owner should upstream those differences.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.

## Layout

```
src/lib.rs                 # crate root
examples/                  # the bounded generation conformance corpus
tests/it/main.rs           # the merged integration test binary; each former tests/*.rs file is a mod here
schemas/                   # domain output contracts included by their owning library producers
spec/                      # requirements artifacts, the test matrix, the suite registry
reviews/                   # quire-validated SpecReview artifacts
scripts/                   # the unsafe-comment audit
```

New integration tests go under `tests/it/` as a module of the single `it` binary (add the file plus a
`mod <name>;` line in `tests/it/main.rs`), not as a new top-level `tests/<name>.rs` -- Cargo links and
starts a whole new process per top-level `tests/*.rs` file, which is exactly what `it` exists to avoid
(IR-237). A file only stays a separate `[[test]]` target when it genuinely needs its own process
(racy process-global state, or a subprocess assumption of being the sole executable); none of the
current files needed that.
