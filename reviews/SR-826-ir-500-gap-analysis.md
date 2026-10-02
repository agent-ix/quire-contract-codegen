---
id: "SR-826"
title: "CG PR 231 gap analysis: IR-500 acceptance against the rustdoc fix"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@fece43c6f06378f90abd74842d433873090e0e4c; Makefile (rustdoc target), src/oracle/equality/mod.rs:18, checked against the IR-500 ticket text"
---

# SR-826: CG PR 231 gap analysis

## Summary

Ticket: IR-500. PR: agent-ix/quire-contract-codegen#231 at fece43c. This run is planless and
was not run with quoin. Plan completion: not assessed. The PR changes one doc comment and one
local make target. No requirement, matrix row or tagged test owns either, and the PR says so
("no requirement owns either. No matrix change"). That holds: nothing under `spec/` or `tests/`
names the `rustdoc` target or this doc line. The ticket text is treated as data. Its fix
statements were re-measured against the code:

- "Resolve the link path or drop the brackets": met. The link now names
  `CompositeEqualityRefusal::Declaration`, and the generated HTML resolves it to
  `enum.CompositeEqualityRefusal.html#variant.Declaration`. The ticket's cited location
  (`src/composite_equality.rs:18`) is stale: that file became `src/oracle/equality/mod.rs` in
  commit 879de44 (AD-004 step 2d). The same line moved with it.
- "Consider making the rustdoc gate cover private items so these cannot hide": met for the
  `make rustdoc` / `make ci` gate.
- Oracle strength: the gate can fail. At the base doc line with the new flag, rustdoc emits the
  `broken_intra_doc_links` warning, and `-Dwarnings` turns that into an error. At head the same
  command passes. The check is therefore not vacuous: the broken state it exists to catch makes
  it fail.
- No stubs, no dead code and no unowned production code were added. No test was needed or
  added, because the gate itself is the test of a doc-only change.

What is left is outside this PR's diff and its ticket. The GitHub workflow runs no rustdoc step
and is dispatch-only, so the check holds only where `make ci` is run. That was so before this PR,
and the ticket asked only about `make rustdoc`. The PR body's wrong claim that CI runs it is
SR-825 FND-001.

## Verdict

Clean. Both fix statements in the ticket are met at fece43c, and the widened gate is a real
check, not a vacuous one.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
