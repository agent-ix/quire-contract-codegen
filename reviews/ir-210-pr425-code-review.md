---
id: SR-12320
title: "IR-210 CG #425 code review (rust lane): ix-trace-rs dev-dependency"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@f8a648c5efad597bd0be50901ce58eabf29ec996; Cargo.toml, Cargo.lock, deny.toml"
review_set: subset
---

## Summary

Ticket: IR-210, PR quire-contract-codegen#425, head f8a648c, branch code/ir-210-cg-ix-trace-dep. The rust-review lane is folded into this file. The diff is 3 files, +9/-0. It adds an `ix-trace-rs` dev-dependency to `Cargo.toml`, one `ix-trace-rs` package entry plus its name in the root crate's dependency list to `Cargo.lock`, and one `allow-git` entry to `deny.toml`. No `.rs` file changes. No `/// Trace:` line is added and no Kani-lane file is touched.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| Cargo.toml:48-49 | examined | `ix-trace-rs = { version = "=0.1.0", git = "https://github.com/agent-ix/ix-trace-rs", branch = "main" }` under `[dev-dependencies]` |
| Cargo.lock:802-805,1434 | examined | `name = "ix-trace-rs"`, `version = "0.1.0"`, `source = "git+https://github.com/agent-ix/ix-trace-rs?branch=main#9378a67762a9871fbad0179028185bb8785a825b"`; `"ix-trace-rs",` in the quire-contract-codegen dependency list |
| deny.toml:81 | examined | `"https://github.com/agent-ix/ix-trace-rs",` in `[sources] allow-git` |
| quire-contract-ir Cargo.toml:37, crates/quire-contract-model/Cargo.toml:56 | context_only | `ix-trace-rs = { version = "=0.1.0", git = "https://github.com/agent-ix/ix-trace-rs", branch = "main" }` (dev-dependency in both) |
| quire-contract-ir deny.toml:31 (origin/main) | context_only | `"https://github.com/agent-ix/ix-trace-rs",` in `allow-git`; no license exception for it |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Approved. No defects.

Dependency spelling:

- The CG line is byte-identical in spelling to the IR root and quire-contract-model dev-dependencies: `version = "=0.1.0"`, the same git URL, `branch = "main"`. It sits in `[dev-dependencies]`, so it does not enter any consumer's graph.

Lock:

- `git diff origin/main...HEAD -- Cargo.lock` is 5 added lines and nothing removed: the 4-line package entry and the root crate's `"ix-trace-rs",`. No other version, source or checksum moves.
- `cargo metadata --locked --offline` resolves.
- `awk -f scripts/check_one_copy.awk Cargo.lock` passes. Cargo.lock holds exactly one `ix-trace-rs` entry.
- ix-trace-rs 0.1.0 has no dependencies of its own (`[dependencies]` is empty; it is a proc-macro crate). It adds no transitive crates.

Rev 9378a67 versus IR's 83c970b:

- The GitHub compare 83c970b...9378a67 is `ahead` by one commit, 9378a67 "docs: refresh Discord invite link (#13)". It changes only CONTRIBUTING.md and README.md.
- IR's ix-trace-rs is a dev-dependency of IR, so it never appears in CG's graph. CG's lock has one ix-trace-rs source and one rev. That is the property that matters, and it holds.
- The two repos resolving `branch = "main"` at different commits is normal lock behaviour. It is not a finding, and no rev should be pinned to match IR.

deny.toml:

- The change is one `allow-git` line, in alphabetical order. No license, advisory, bans or other sources setting changes.
- The line is necessary. With it removed (a probe copy of the manifest, lock and config in the scratchpad), `cargo deny check sources` fails with `source-not-allowed` on `ix-trace-rs 0.1.0 git+https://github.com/agent-ix/ix-trace-rs?branch=main`, reached through `(dev) quire-contract-codegen`.
- No license exception is needed. ix-trace-rs declares `AGPL-3.0-or-later`, but cargo-deny 0.20.2 does not license-check dev-only dependencies by default (`licenses.include-dev = false`). With `include-dev = true` in a probe copy, the licenses check rejects it. IR's deny.toml makes the same choice: an allow-git line and no exception.
- `cargo deny check` on the head passes: advisories ok, bans ok, licenses ok, sources ok. The only warning is the pre-existing `advisory-not-detected` for RUSTSEC-2026-0009.

Rust lane: there is no source change, so there is no new panic, unsafe, conversion, async or lock surface. An unused dev-dependency triggers no lint: the repo sets no `unused_crate_dependencies` lint and runs no udeps/machete gate.
