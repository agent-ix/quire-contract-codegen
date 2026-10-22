---
id: SR-15410
title: "IR-961 PR 520 CI suite deduplication code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@308598b3954d100adfcfb3ca511273d611212e6b; Makefile; rust-toolchain.toml; .github/workflows/ci.yml; scripts/test_qspec_preflight.sh; IR-961"
review_set: subset
---

## Summary

Ticket: IR-961. Reviewed CG PR 520 at exact head 308598b3954d100adfcfb3ca511273d611212e6b on base 5c19d08a2e363d56ec3a34f86baf801a73fe15da. The one-line Makefile change makes `msrv` compile all targets with Rust 1.98.1 and leaves `test` as the only full suite invocation. No CI workflow or QSpec gate line changed.

## Verdict

**PASS** — the red/green dry-run logs show two test invocations before the edit and one test invocation plus one all-target MSRV check after it. An independent `make -n ci` at the exact head confirms the same command graph. `make msrv` completed successfully on the head; the final aggregate gate remains with the PR owner in the CG quiet window.

## Coverage

- Makefile (examined): `ci` still invokes `fmt-check spec lint msrv deny rustdoc test` serially after the QSpec fixture preflight. `test` still depends on `qspec-preflight` and `qspec-preflight-selftest`, then runs `cargo test --locked` once.
- Makefile (examined): `msrv` now runs `cargo +1.98.1 check --locked --all-targets`, compiling library, binary, test, example and benchmark targets without executing the test suite a second time.
- rust-toolchain.toml (context_only): the default channel is 1.98.1, equal to the Makefile MSRV.
- .github/workflows/ci.yml (examined): unchanged by the PR; its Test and Rust 1.98.1 compatibility jobs remain present.
- scripts/test_qspec_preflight.sh (context_only): unchanged and still scheduled by `make test`.
- Evidence (examined): `red-make-n-ci.log`, `green-make-n-ci.log`, `make-msrv.log`, and reviewer `make -n ci` output in qualification-logs/ir961-pr-review. The MSRV check log ends in a successful dev-profile finish. `git diff --check` passed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
