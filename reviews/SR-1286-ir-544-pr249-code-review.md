---
id: "SR-1286"
title: "CG PR 249 code review: Makefile resolves quire from PATH"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@1b9fc40ee730380089b8566472b3415adb8a9bc9; Makefile (line 12, spec target line 97, ci target line 210); .github/workflows/ci.yml; CLAUDE.md; PR body"
---

# SR-1286: CG PR 249 code review (config change)

## Summary

Ticket: IR-544. PR: agent-ix/quire-contract-codegen#249 at 1b9fc40, one commit on main 1c817d8.
`git diff origin/main...HEAD` touches one file, `Makefile`, with one line changed:
`override QUIRE := $(TRUSTED_HOME)/.npm-global/bin/quire` becomes `QUIRE ?= quire`. No other file
changes. The new line has the same spelling as quire-contract-ir's `Makefile:12` (`QUIRE ?= quire`).

What was checked and measured:

- Uses of `$(QUIRE)`: one, the `spec` recipe (`Makefile:97`). `spec` is a prerequisite of `ci`
  (`Makefile:210`). No script under `scripts/` uses QUIRE or the npm-global path, and no Rust
  source does either. A repo-wide grep for `QUIRE`, `npm-global` and `TRUSTED_HOME`, outside
  `reviews/`, finds only Makefile lines 8, 10, 12 and 97.
- `TRUSTED_HOME` stays in use by `override CARGO` (line 10), so the variable is not orphaned. The
  `override BASH`, `override CARGO` and `override MSRV` lines are unchanged.
- CI: `.github/workflows/ci.yml` is `workflow_dispatch`-only and never calls `make`. Its "Validate
  specification" step runs `npm install --global '@agent-ix/quire-cli' '@agent-ix/quoin'`, then
  `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` with quire on PATH.
  It never used `$(TRUSTED_HOME)/.npm-global/bin/quire`, so this change cannot break CI. The
  Makefile now resolves quire the same way CI does. `cla.yml` does not touch quire.
- Docs: CLAUDE.md describes `make spec` without naming a path. No spec or plan file names the old
  path. Four older `reviews/` files mention the old override as a host problem at the time they
  were written. They are history and correctly left unchanged.
- Behaviour change: `QUIRE` from the environment or the make command line now wins over the
  default. No workflow, doc, shell rc file (`~/.bashrc`, `~/.profile`) or the current dev shell
  sets a `QUIRE` variable. quire-contract-ir uses `QUIRE_SPECIFICATION_DIR`, which is a different
  name. No unrelated variable can take over the default.
- Measured at 1b9fc40 on this host (quire 0.33.0 on PATH at `~/.nvm/.../bin/quire`;
  `~/.npm-global/bin/quire` does not exist):
  - `make -n spec` prints `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'`.
  - `make spec` exits 0. Its output is only the existing `semantic.*` and
    `DuplicateArchetype`/`DuplicateInverseEdge` module warnings.
  - `QUIRE=/nonexistent make spec` and `make spec QUIRE=/nonexistent` both fail fast:
    `make: /nonexistent: No such file or directory`, recipe Error 127, make exit 2.
  - With `PATH=/usr/bin:/bin` (no quire), `make spec` fails the same way. It names `quire`, so
    the cause is clear.
  - Control: main's Makefile run with `make spec` fails with Error 127 on
    `/home/peter/.npm-global/bin/quire`, and `make -n spec QUIRE=x` still prints the hardcoded
    path. This confirms both the bug and that `override` ignored the caller.
  - `quire coverage --scope . --strict` (run outside `make spec`): 66 unbacked rows and 0
    contradicted statuses. This matches the stated baseline. The diff touches no spec, plan or
    test file, so the count cannot have moved.
- PR body: it says "Closes IR-544" and ends with the Claude Code footer. Its claims (`make -n
  spec`, `make spec` exit 0, `QUIRE=/nonexistent` exit 127) match the measurements above.

## Verdict

Clean, and mergeable once branch protection is satisfied. The change is the minimal fix the
ticket asks for. It follows the sibling repo's convention and matches how CI already provides
quire. No CI path depended on the removed override.

`gh pr view 249` reports `mergeable: MERGEABLE` and `mergeStateStatus: BLOCKED`. The only checks
are the CLA workflow's: `cla / gate` succeeded, `cla / cla-check` was skipped, and
`cla / cla-check-internal` was still in progress when this review read it. The Rust CI workflow
is dispatch-only and does not run on PRs.

Not a finding, for context: this line was added in bbd5e67 (#15) as one of the `override`
"trusted toolchain" lines that went with the Make-integrity residual in #14, and #14 is closed.
`override CARGO := $(TRUSTED_HOME)/.cargo/bin/cargo` keeps the same hardcoded-path shape. It is
out of scope for IR-544.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
