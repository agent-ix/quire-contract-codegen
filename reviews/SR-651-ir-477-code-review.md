---
id: "SR-651"
title: "CG PR 211 code review: drop artifact-digest evidence calls from package test builders"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@9ddf5a96e78bbf878ca34d2ad48eb07314a25424; tests/exact_scalar_support/package.rs, tests/composite_equality_support/package.rs"
---

# SR-651: CG PR 211 code review

## Summary

Ticket: IR-477. PR: agent-ix/quire-contract-codegen#211, head 9ddf5a9, branch
`fix/ir-477-drop-artifact-digest-calls`, base main 94ab14d. Main is now 113b624 (#208 merged).
#208 touched none of these files, and `git merge-tree` of 113b624 with 9ddf5a9 is clean. This
review is code-review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD`:
2 test files, +8/-74. No spec file changed, so no spec-review ran.

## Method

I read the diff and both builders' `admit`/`admit_with`. I checked the post-#235 IR
`CheckedPackageEvidence` at quire-contract-ir main b995245
(`crates/quire-contract-model/src/checked_package/evidence.rs`). It now has only
`insert_domain_package_document`, `insert_dependency_package(identity, package)` (no version
parameter) and `support_feature`. The artifact-digest inserters are gone.

I grepped the whole repo at 9ddf5a9 (src, tests, spec, plan, Cargo files) for
`insert_artifact_bytes`, `insert_artifact_digest`, `insert_dependency_package`, `RevisionMismatch`,
`CheckedArtifactLocator`, `ArtifactDigest(s)`, `stale_dependency`/`StaleDependency` and
`DigestMismatch`. Nothing matches except one doc comment at
`tests/exact_scalar_support/package.rs:906`. That comment cites IR's `same_non_graph_lock`
`StaleDependency` refusal, which IR main still has (`checked_package/v2/mod.rs`), so it is still
accurate. No test reads a package with its own evidence, and no test asserted a digest-based
refusal. So no test lost its meaning.

The net diff has no `Cargo.lock` change. The branch history is 0837177 (the change), f31168a (fmt)
and 9ddf5a9 ("Restore Cargo.lock"). The net `origin/main...HEAD` diff touches only the two test
files.

I ran the gate myself against IR main b995245 and QSL main c0d35b69 (#548 merged). I used a scratch
`CARGO_HOME` `[patch]` config over detached worktrees of both, a private scratchpad TRUSTED_HOME,
and ran `make -k ci LOCKED=`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. These were checked and found sound:

- The removal is complete. The `locator` helper and the `artifacts` collection existed only to feed
  `insert_artifact_digest`, and both are gone. `evidence()` takes no argument and declares only
  `quire.value.complete/v1`. Clippy `-D warnings --all-targets` passes, so no import or helper
  became unused (`CheckedArtifactLocator` was dropped from both `use` lists, and `Value`/`Sha256`
  are still used by `wire()`).
- The builders still write the `digest`/`digest_domain` members on lock entries. These are
  wire-schema members, not evidence, and IR still admits packages that carry them (241/241 pass
  with #212 stacked, below).
- The change matches the repo's own CLAUDE.md rule against hash/digest antipatterns. It removes a
  digest-tracking path and adds nothing to replace it.

Gate results at 9ddf5a9, patched to IR main and QSL main: fmt-check passes. `spec` passes (quire
validate, warnings only, all pre-existing). `lint` passes. `deny` passes (advisories, bans,
licenses and sources ok, one-copy awk ok). `audit-unsafe` and its selftest pass. `rustdoc` passes.
`msrv` and `test` fail the same way: lib 101/101, `it` 129 passed, 109 failed, 8 ignored (246).
All 109 failures are `expected V2 admission, got Refused(IllTyped ... cause: OperatorIneligible)`,
raised from the builders' admit panic: 73 at `exact_scalar_support/package.rs:1034` (node 145), 16
at `composite_equality_support/package.rs:483` (node 45), and 20 via the exact-function path
through the composite builder (node 220). None is a digest or evidence refusal. Attribution
control: PR #212 head f5707f2 (IR-480, stacked on 9ddf5a9) under the same patch gives
`cargo test --test it` 241 passed, 0 failed, 8 ignored. So every failure is the IR-480 corpus
family and none comes from this PR.

Mergeable in the planned order: together with #212 and the CG lock bump to IR main and QSL main.
Alone on the current lock (IR 54f9a48), this PR does not compile.
