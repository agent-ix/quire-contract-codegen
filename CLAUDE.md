# quire-contract-codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Hash / digest / pin antipattern: do not introduce

Hashes, digests, SHAs, pins, checksum catalogs and records that track files, versions or
tools are an antipattern and have been removed from this repository. Do not introduce
any new use of them. If you find one, remove it as part of the change. The only hash
that stays is a canonical identity digest that binds a proof to the exact content it
proved. Package versions live in Cargo.toml / package.json and their lockfiles only;
reports name the app version they ran.

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
make deny             # cargo-deny lanes plus the one-copy check (scripts/check_one_copy.awk: one Cargo.lock entry per agent-ix git crate)
make use-local        # patch first-party git deps to sibling checkouts via a gitignored .cargo/config.toml; snapshots Cargo.lock to .cargo/Cargo.lock.pre-local; fails if cargo metadata fails or a patch is unused
make use-remote       # delete the patch config and restore Cargo.lock from that snapshot (no snapshot: lock untouched)
make audit-unsafe     # check that every unsafe block has a // SAFETY: comment
make rustdoc          # build warning-free API documentation
make ci               # every local gate above except build and clean
```

`LOCKED` (default `--locked`, empty while `.cargo/config.toml` exists) is passed to the cargo
targets that resolve dependencies (`lint`, `test`, `build`, `msrv`, `kani`, `rustdoc`); override
it, e.g. `make lint LOCKED=`, when a patch or a deliberate lock change makes `--locked` wrong.

## Safety scaffolding

- `clippy.toml` caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. A line whose every occurrence is written `"unsafe {`, the token immediately preceded by a double quote, is a mention rather than a block and is not audited: this repository generates Rust and asserts properties of the generated text, so `"unsafe {"` appears as data in the tests that forbid unsafe code in generated output. The test is quote-prefixing, not string-literal membership, and only the exact spelling `"unsafe {` counts: a mention with a leading space, one mid-literal, or one inside a multi-line expected-output fixture is still audited and needs a baseline entry. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`. A tree with no Rust source roots, or a scan that fails, exits 2 as inconclusive rather than printing "unsafe audit passed"; the earlier version reported success when it had nothing to audit.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.

## Layout

```
src/lib.rs                 # crate root
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
