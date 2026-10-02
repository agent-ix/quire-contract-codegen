---
id: "SR-825"
title: "CG PR 231 code review: rustdoc link fix and private-item rustdoc gate"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@fece43c6f06378f90abd74842d433873090e0e4c; Makefile (rustdoc target), src/oracle/equality/mod.rs (module doc line 18); PR body"
---

# SR-825: CG PR 231 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-500. PR: agent-ix/quire-contract-codegen#231 at fece43c, one commit on merge base
2130cd9. origin/main is now 1629715; the only commit since the base (#229) touches `spec/` and
`reviews/` only, so no code it adds can change the rustdoc result. The diff is two lines:

- `src/oracle/equality/mod.rs:18`: `[`DeclarationRefusal`]` becomes
  `[`CompositeEqualityRefusal::Declaration`]`.
- `Makefile` `rustdoc`: adds `--document-private-items` to
  `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --no-deps`.

Rust lane folded in, as `rust-review` says. Measured:

- Base doc line, with `cargo doc --locked --no-deps --document-private-items`: exactly one warning,
  `unresolved link to DeclarationRefusal` at `src/oracle/equality/mod.rs:18:44`
  (`rustdoc::broken_intra_doc_links`). The old `make rustdoc` command (no private items,
  `-Dwarnings`) passes on the base. The new flag is what makes this warning visible.
- Head, `make rustdoc`: passes with `-Dwarnings` and `--document-private-items`, so there are
  0 warnings, private items included.
- The link resolves to the right item. The generated `oracle/equality/index.html` links to
  `../../enum.CompositeEqualityRefusal.html#variant.Declaration`, the crate-root re-export. That
  variant's own doc reads "`TypeEnvironment::new` refused the reconstructed declaration
  closure". This is what the sentence describes: "admits it through `TypeEnvironment::new`
  (refusing per [..] on failure)".
- Line 18 is 95 characters, under the 100-column rustfmt width. `make fmt-check` passes.
- With `--document-private-items`, rustdoc turns off `rustdoc::private_intra_doc_links`. Nothing
  else is relaxed: `broken_intra_doc_links` and the other default lints are still errors under
  `-Dwarnings`.
- `make rustdoc` is already in the `ci` prerequisites (`Makefile:207`), so the wider check runs
  in the local gate.
- No Rust code, test, unsafe, panic or API surface changes. Rust idiom checks therefore apply
  only to the doc link. A path to the variant is the idiomatic form, better than dropping the
  brackets.
- PR title: "Fix unresolved rustdoc link and document private items in make rustdoc". It has
  no bare ticket id. The body ends `Closes IR-500`, which matches the ticket. It explains the
  path the ticket cites (`src/composite_equality.rs:18`): the file moved in AD-004 step 2d,
  confirmed as commit 879de44 (#227).

## Verdict

Mergeable. The fix is correct and minimal, and the measured warning claims in the PR body hold
(base 1 warning, head 0). One low finding: a sentence in the PR body says the GitHub CI runs
`make rustdoc`, which is false (FND-001). It does not block the code. Correct the body before
merge so that nobody reads it as CI enforcement.

Gate: every `make ci` lane was run at head fece43c in a separate worktree. The log is in the
reviewer scratchpad as `cg-ir-500-ci.log`. On this host, `make ci`'s `spec` lane is pinned by
`override QUIRE := $HOME/.npm-global/bin/quire`, and that file does not exist here. The spec
lane was therefore run with the `quire` on PATH, and the other lanes as make targets. This is a
host-environment issue that predates this PR. Every lane passed: spec, fmt-check, lint, msrv
(115 unit + 252 integration passed, 9 ignored), deny, audit-unsafe, rustdoc, and test
(115 + 252 passed, 9 ignored, + 1).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR body says "No CI workflow file is touched (CI runs `make rustdoc`, so it picks up the wider coverage on its own)". That is false. `.github/workflows/ci.yml` has no rustdoc or `cargo doc` step: its steps are fmt, clippy, test, the 1.98.1 check, quire validate and the unsafe audit, plus a cargo-deny job. The workflow is also `workflow_dispatch`-only. The wider check runs only in the local `make ci` gate. Correct the sentence to say that. | .github/workflows/ci.yml:3-49, Makefile:207 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3e3bbcf (round 1, reviewed at 3e3bbcf: a PR-body edit, no commit. The body now reads "The CI workflow has no rustdoc step and is dispatch-only; the wider check runs in local `make ci` (`Makefile:207`)". Verified: ci.yml is `workflow_dispatch`-only with no doc step, and Makefile:207 is the `ci:` line naming `rustdoc`. The head is a pure rebase of fece43c onto main 1629715: range-diff `=`, and the diff for src/ and Makefile against fece43c is empty. The gate line names the new head, and the coder's log ends `head=3e3bbcf... exit=0`, with every lane passing, rustdoc included (`--document-private-items`). The title is unchanged and has no bare ticket id) |
