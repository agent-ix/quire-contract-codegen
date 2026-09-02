# quire-contract-codegen

Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # locked clippy with -D warnings
make test             # locked cargo test; depends on assurance-inputs
make build            # locked release build
make msrv             # execute all tests with exact Rust 1.75.0
make spec             # Quire-validate the specification, planning, plan and review documents
make clean            # cargo clean and drop the assurance environment
make deny             # all configured cargo-deny lanes
make audit-unsafe     # check that every unsafe block has a // SAFETY: comment
make rustdoc          # build warning-free API documentation
make conformance      # run the bounded generation conformance corpus
make upstream-identity# check the IR and runtime revisions agree in all three places
make assurance-env    # create the pinned shared-assurance interpreter
make assurance-inputs # run the producers and write their structured results
make pins             # classify the toolchain through the packaged compatibility matrix
make compat-view      # read retained evidence through the shared mapping
make assurance-chain  # seal, retain, and verify through Quoin
make assurance        # pins + compat-view + assurance-chain
make ci               # every local gate above except build and clean
```

## Shared assurance

`make assurance-inputs` is the only target that runs a producer. Everything downstream consumes the
files it writes and refuses to create them, because a consumer that can produce its own input can
produce a green run out of nothing. `assurance/README.md` is the guide; `assurance/pins.json` records
the adopted Engineering Assurance release and the digests of the artifacts read from it.

Retention, integrity checking, audit, attestation and receipts are Quoin's. Static specification,
obligation and coverage facts are Quire's. The read-only mapping of retained bytes is Engineering
Assurance's. This repository retains no evidence of its own and computes no aggregate verdict.

Everything under `evidence/` is immutable retained history and is not written by any gate. The anchor
writer and the verifier that read `evidence/ANCHORS` are gone; the file is left exactly as it was,
because the constraint is that retained bytes stay unchanged and an unread census inside an immutable
tree is a smaller problem than editing that tree to tidy it.

## Before trusting a green `make ci`

It is a statement about the tree as committed, and not about a tree whose Makefile has been edited.
`.IGNORE:`, a `-` recipe prefix, or an assignment to `SHELL` each make every recipe report success.
Measured here: with three injected defects the control exits 2 at `fmt-check`; the same tree with
`.IGNORE:` prepended exits 0, runs all eleven prerequisites, and fails seven of them without failing
the build. The Makefile header carries the numbers; the residual is issue #14.

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`. A tree with no Rust source roots, or a scan that fails, exits 2 as inconclusive rather than printing "unsafe audit passed"; the earlier version reported success when it had nothing to audit.
- This unsafe-audit script intentionally strengthens the shared seven-repository version by scanning tests, benches, and examples and emitting a positive completion marker; the shared policy owner should upstream those differences.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.

## Layout

```
src/lib.rs                 # crate root
examples/                  # the generation conformance producer
tests/integration.rs       # end-to-end tests
tests/shared_assurance.rs  # FR-006 gates; /// Trace: comments are Quire's census
schemas/                   # three live domain contracts, two frozen evidence schemas
spec/                      # requirements artifacts, the test matrix, the suite registry
reviews/                   # quire-validated SpecReview artifacts
assurance/                 # the change declaration and the adopted pins
scripts/                   # producers and the assurance chain driver
evidence/                  # immutable retained history; read, never written
```
