---
id: SR-15619
title: "Code review — IR-961 MSRV command follow-up"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; Makefile:130-131; rust-toolchain.toml; .github/workflows/ci.yml; IR-961"
review_set: subset
---

## Summary

Ticket: IR-961. Narrow independent review of CG PR 520 at exact head `14a5ae9ab972bcf80076f25d1eb8795ba62b22aa`, compared with prior reviewed head `56348be1ce15d02ea73e0e15d201d9ea1611e326`. The only new change replaces the `msrv` recipe's cargo launcher with `rustup run $(MSRV) cargo`.

## Verdict

**PASS** — the command selects Rust 1.98.1, compiles all targets, retains conditional `--locked`, and does not run the test suite. The repository already invokes `rustup run $(MSRV) rustc` for its gate binary. The changed recipe avoids expansion of the Makefile's Python-derived `$(CARGO)` path; the exact-head `make msrv` log shows successful compilation. Final aggregate `make ci` remains with the PR owner.

## Coverage

- `Makefile:130-131` (examined): `msrv` executes `rustup run $(MSRV) cargo check $(LOCKED) --all-targets`.
- `Makefile:235-236` (examined): `ci` continues to run `fmt-check spec lint msrv deny rustdoc test`; `test` remains the sole full test execution within `make ci`.
- `Makefile:61-63` (context_only): existing gate-binary recipe uses `rustup run $(MSRV) rustc`.
- `rust-toolchain.toml` (context_only): default channel is 1.98.1, matching `MSRV`.
- `.github/workflows/ci.yml` (context_only): unchanged in the PR; review of this follow-up finds no workflow gate edit.
- `qualification-logs/ir961-pr520-msrv-fix.log` (examined): exact-head `make msrv` finished successfully.
- Independent `make -n msrv`, `rustup run 1.98.1 cargo --version`, and `git diff --check` (examined): expected command, Cargo 1.98.1, and clean diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
