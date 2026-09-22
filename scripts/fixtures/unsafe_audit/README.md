# unsafe-audit fixture corpus

This directory exists outside every root `scripts/check_unsafe_comments.sh`
scans (`src`, `tests`, `benches`, `examples` at the repository root), so the
deliberately uncommented `unsafe {` blocks under `src/` here never make the
real `make audit-unsafe` red. It is exercised only by
`scripts/test_check_unsafe_comments.sh`, which runs the scanner with this
directory as its working directory (so its own nested `src/` is the only
search root the scanner finds) and diffs the flagged lines against
`expected_missing.txt`.

`scripts/unsafe_comment_baseline.txt` here is intentionally empty. The real
scanner accepts baseline-listed misses silently; an empty baseline forces
every flagged line in this fixture to be reported on stderr, which is what the
harness parses.

None of these `.rs` files are part of any Cargo target -- they are not
declared in `Cargo.toml` and are unreachable from `src/lib.rs`'s module tree
-- so `cargo fmt`, `cargo clippy`, and `cargo test` never touch them, and the
files do not need module declarations that would compile.

Each file under `src/` demonstrates one case:

- `escaped_quote.rs`, `char_quote.rs`, `raw_string.rs`, `multiline_string.rs`,
  `mention_then_block.rs` -- the five constructs from
  agent-ix/quire-contract-codegen#118: a real, uncommented `unsafe` block
  immediately after or alongside an escaped quote, a char literal, a raw
  string, a multi-line string, and (on the same line) a quote-prefixed mention
  of the word `unsafe`. All five must be flagged.

  `mention_then_block.rs` is the only one of the five that a naive
  string-span-deletion mutant (the #118 shape) still flags correctly, so it can
  look redundant. It is not: it is the *only* fixture that catches the other
  obvious weakening -- skipping any line that contains `"unsafe {` at all --
  which every other fixture here passes. Do not drop it as a duplicate.
- `trivial_positive.rs` -- an uncommented `unsafe` block with none of the
  above adjacent. Must be flagged; this is the case every escape-adjacent
  fixture is a harder variant of.
- `mention_only_negative.rs` -- a line that is only a quote-prefixed mention
  of the word `unsafe`, the same shape as
  `tests/exact_scalar_generation.rs:1118` in this repository. Must NOT be
  flagged.
- `compliant_with_safety.rs` -- an `unsafe` block with a proper `// SAFETY:`
  comment. Must NOT be flagged.
