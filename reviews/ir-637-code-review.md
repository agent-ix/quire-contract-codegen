---
id: SR-1680
title: "IR-637 CG PR 300 code review: QSL lock update"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@6f46385b859904394e75db3cd6762bb39965169b; Cargo.lock, deny.toml against origin/main 3ccc11301ed6d0bf6e8330c8d36532dc8f5363a6"
review_set: subset
---

## Summary

Ticket: IR-637. PR: quire-contract-codegen#300. The code-review method includes the Rust dependency and gate checks from rust-review. The diff contains only `Cargo.lock` and `deny.toml`; no Rust source, test, CI, Makefile, or spec file changed.

I compared the locked first-party sources with QSL's `9dd5fa1432493f37df2e2d5e47357e0cddc5034c` lock, parsed both CG lockfiles, and inspected `cargo metadata --locked --offline`. The head has 264 packages, nine added packages and none removed. It has one QSL revision and one entry for each agent-ix crate. The QSL-related shared revisions, including `quire-semantic-value` at `660a126f`, match QSL's own lock. `quire-contract-model` and `quire-verification-contracts` have different CG-local revisions than QSL, but both were already different on main and neither changes in this PR.

The nine additions are `agent-ix-semantic-schema`, `quire-code-parse`, `quire-rust-extraction`, `streaming-iterator`, and five tree-sitter crates. Cargo metadata resolves the three new first-party crates, each with `AGPL-3.0-or-later`; their three crate-specific exceptions and the `quire-code-rs` Git source are the only additions to `deny.toml`. The pre-PR exact-head `make ci` and Kani passes were supplied in the reviewer brief; I did not rerun the full gates. The extra extraction dependency cost is tracked separately by IR-640.

## Verdict

**PASS** — the lock and source allowlist changes are scoped to the QSL revision and its resolved dependency graph. No weakened gate, unrelated lock update, or extra first-party dependency was found. This is the pre-merge review; the required second full-gate run remains for the merge stage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
