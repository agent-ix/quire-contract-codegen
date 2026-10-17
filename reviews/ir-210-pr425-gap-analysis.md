---
id: SR-12321
title: "IR-210 CG #425 gap analysis: ix-trace-rs dev-dependency"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@f8a648c5efad597bd0be50901ce58eabf29ec996; Cargo.toml, Cargo.lock, deny.toml; tests/it/exact_scalar_agreement.rs (read as context)"
review_set: subset
---

## Summary

Ticket: IR-210, PR quire-contract-codegen#425, head f8a648c. Plan completion: not assessed. This PR only enables later work. It adds the `ix-trace-rs` dev-dependency that later IR-210 conversion PRs need for `use ix_trace_rs::trace;` plus `#[trace("TC-NNN", "FR-nnn-AC-n")]`. It changes no spec, test or source file. No acceptance criterion gains or loses a tagged test, so the computed Test Matrix is unchanged by construction. The analysis covered the one place where adding a dependency to this crate's manifest does not carry over: the scratch crates that the agreement tests build.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| Cargo.toml:48-49 | examined | `ix-trace-rs = { version = "=0.1.0", git = "https://github.com/agent-ix/ix-trace-rs", branch = "main" }` |
| tests/it/exact_scalar_agreement.rs:19-23 | examined | `/// Development dependencies the agreement support modules use beyond the Contract Runtime the generated crate already depends on. ...` `const AGREEMENT_DEV_DEPENDENCIES: &str = "serde_json = \"=1.0.151\"\nsha2 = \"=0.10.9\"\n";` |
| tests/it/exact_scalar_agreement.rs:55-70 | examined | the runner appends `[[test]] name = "agreement"` and `[dev-dependencies]\n{AGREEMENT_DEV_DEPENDENCIES}` to the generated manifest, copies this repository's `Cargo.lock`, and runs `cargo test --offline --test agreement` |
| tests/exact_scalar_support/agreement_cases.rs, tests/composite_equality_support/agreement_cases.rs, tests/exact_function_support/agreement_cases.rs | context_only | the three scratch-crate test sources; they carry 15, 7 and 4 `/// Trace:` lines today |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The agreement scratch crates do not inherit this crate's dev-dependencies. They get only `AGREEMENT_DEV_DEPENDENCIES` (serde_json, sha2). Converting the `/// Trace:` lines in the three `agreement_cases.rs` files to `#[trace]` therefore also needs `ix-trace-rs` added to that constant, or those files stop compiling. The failure is loud: the three agreement tests fail with a build error, not a silent coverage loss. Nothing records it as a follow-up. The PR body, the commit message and the IR-210 comments (including the trace-audit plan) do not mention it. | tests/it/exact_scalar_agreement.rs:23 |

## Verdict

No blocking finding. FND-001 is low, and it is outside this PR's diff: it needs no change here, only a recorded follow-up. Either the IR-210 conversion PR for the agreement-case families adds `ix-trace-rs = { version = "=0.1.0", git = "https://github.com/agent-ix/ix-trace-rs", branch = "main" }` to `AGREEMENT_DEV_DEPENDENCIES`, or the ticket notes that those files keep their current form.

Verified today:

- The diff adds no `/// Trace:` line and touches no Kani-lane file (`git diff --name-only origin/main...HEAD` is Cargo.lock, Cargo.toml, deny.toml).
- The scratch crate copies a lock that now carries one extra, unused `ix-trace-rs` entry. Cargo prunes unused entries, and `--offline` without `--locked` permits that, so the copied lock does not break the scratch build.
- I ran the three scratch-crate agreement tests at head f8a648c: `cargo test --offline -j2 --test it agrees_with_direct_runtime`, unsandboxed, with a private CARGO_TARGET_DIR under the worktree, deleted afterwards. All three passed: `tc_024_generated_scalar_crate_agrees_with_direct_runtime`, `tc_029_generated_composite_equality_crate_agrees_with_direct_runtime` and `tc_031_generated_function_crate_agrees_with_direct_runtime` (3 passed, 0 failed, 625 filtered out). Today, the new lock entry does not break them.
