---
id: SR-12640
title: "IR-876 code review of CG PR 432"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR 432, 71182aef..3e1cbfa1; Makefile, scripts/test_qspec_preflight.sh, CI path context"
review_set: subset
---

# IR-876 code review of CG PR 432

## Summary

The change removes the sibling default for `QSPEC_DIR`, checks the three actual QSpec fixture paths before `make test` and `make ci`, and adds a preflight self-test. No Rust file or QSpec content changes.

## Verdict

**PASS** — no code-review finding in the PR diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No finding in the reviewed diff (placeholder). | - |

## Evidence

- `make qspec-preflight-selftest` and `bash -n scripts/test_qspec_preflight.sh` pass. The self-test exercises unset, empty, nonexistent, and each missing fixture root, plus `test`, `ci`, and `-k ci` rejection before Cargo. An independent `env -u QSPEC_DIR make -k ci` exits 2, prints an actionable `QSPEC_DIR` diagnostic, and invokes no Cargo command.
- The CI workflow's `cargo test` step still passes `${{ github.workspace }}/quire-specification` explicitly. The PR body accurately limits the gate to fixture presence, so a stale explicit checkout remains possible.
- The only Makefile changes are help text, the preflight and self-test targets, the `test` prerequisites, removal of the sibling default, and `ci` preflight with a serial recursive make over the unchanged ordered goal list. `-k` remains inherited by the child make; failure yields a nonzero status. `git diff --check` passes.
- The branch contains one implementation commit; base Makefile has no `qspec-preflight` target, so the added self-test's first target check is red on base. The author reports full `make -k ci` green on this head; this review independently ran the focused preflight checks.
