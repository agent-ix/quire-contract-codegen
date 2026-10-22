---
id: SR-15363
title: "Code review of IR-947 first-party lock cohort"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; Cargo.lock"
review_set: subset
---

## Summary

Ticket: IR-947. Reviewed PR #516 at ddaa62ce2b5c8f9f9156ceac63122ffc9e368b73 against base 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd. The only changed file is Cargo.lock (15 source-line replacements). The Rust review checklist was applied to the dependency graph; there is no changed Rust source or test. Gap analysis and spec review are out of scope because the diff changes neither production code nor spec or plan artifacts.

## Verdict

**PASS** — The selective lock update resolves the requested coherent first-party cohort and retains the runtime revision. No diff-scoped defect found. The author owns final CI and real Kani gates.

## Scope examined

- Cargo.lock (examined): filament-core-data extraction frontend, semantic IR, and semantic schema all resolve commit ac62202af46327ce40b783d9063dd8024b644ea0, despite the schema's branch selector and the other two crates' exact revision selectors.
- Cargo.lock (examined): quire-rs and quire-rust-extraction both resolve fd1cdd9d302bfd870517cc2e009b8646757c12f1.
- Cargo.lock (examined): eight quire-spec-language packages resolve 4bd6f62876c0b8061f64bc70b4670b3b3eed0d02; two quire-contract-ir packages resolve d0737774acfbe64b1019b265952dc4c9720c2f23; quire-semantic-value resolves 1a38c06f39ece40e044a7bef5b868bd478b41c24; quire-contract-runtime remains 3cc88c480d4efe5179fe91acf4634def5d2d82fd.
- Cargo.toml (context only): direct first-party dependencies use branch selectors; no local Cargo patch configuration is active.

## Evidence

- `git diff --check 43d5a6fd...ddaa62ce` passed, and the diff is limited to Cargo.lock.
- Parsed all 268 Cargo.lock packages and all 268 packages from `cargo +1.98.1 metadata --locked --offline --format-version 1`; source, name, and version tuples match exactly with no extras on either side.
- `awk -f scripts/check_one_copy.awk Cargo.lock` passed. Parsed source revisions yield one commit per named first-party repository.
- No production code, Rust test, specification, plan, manifest, or CI workflow file changed; the Rust source idiom, trace, panic, unsafe, and gate-weakening checks have no changed surface.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
