---
id: "SR-652"
title: "CG PR 211 gap analysis: drop artifact-digest evidence calls from package test builders"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@9ddf5a96e78bbf878ca34d2ad48eb07314a25424; tests/exact_scalar_support/package.rs, tests/composite_equality_support/package.rs, spec/**"
---

# SR-652: CG PR 211 gap analysis

## Summary

Ticket: IR-477. PR: agent-ix/quire-contract-codegen#211 at 9ddf5a9. This analysis is planless and
scoped to the PR diff: does any requirement, acceptance criterion or Test Matrix row depend on the
removed lock-vs-evidence digest evidence? Plan completion: not assessed.

## Method

I grepped `spec/` and `plan/` at 9ddf5a9 for artifact-digest, lock-digest, raw-artifact,
digest-evidence and staleness wording. The only digest text near this area is
`spec/interface/interface-001-codegen-api.md:154-156` (replay inputs provided by digest). That is
the replay request path through `qsl_replay::call_site`, not the checked-package reader's evidence,
and this PR does not touch it. The two changed files are shared test-support builders, not traced
tests. No `tc_` function, trace tag or matrix row changes. The IR-477 acceptance asks for the
removed calls to be deleted with no replacement, and that is what the diff does.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. No requirement owned the digest evidence, so its removal needs no spec edit, AC deletion or
matrix change. All traced tests that use these builders still run them. Their current admission
failures against IR main are the IR-480 corpus family, which PR #212 fixes (see SR-651).
