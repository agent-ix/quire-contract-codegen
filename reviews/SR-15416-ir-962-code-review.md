---
id: SR-15416
title: "IR-962 seeded native agreement Rust code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@6c7dafe2bc0659771be68555faa6f46d78393de8; tests/it/scratch_crate.rs; tests/it/exact_scalar_agreement.rs; tests/it/composite_equality_generation.rs; FR-014-AC-6; FR-018-AC-2,12; FR-021-AC-2"
review_set: subset
---

## Summary

Ticket: IR-962. Reviewed the three-file test-only diff from 4c39bd54dd3fc6833e09baaf90556918295c9e65 to 6c7dafe2bc0659771be68555faa6f46d78393de8. Both former `generate-lockfile` call sites now seed the scratch crate from CG's root lock, adapt the different scratch package graph through offline Cargo metadata, assert each resolved Git source is present in the root lock, and retain locked/offline native build or test. The code that generates cases and source, validates native assertion outcomes, and recognizes named failing mutants is unchanged. The committed root Cargo.lock is unchanged and has SHA-256 43dd9ef84f43ea430aa0e3408e5a77063a732adda9a3d294b903642b7a0f9582. The prior eight CI panics are preserved in the IR-962 ticket as red evidence; they failed at dependency download before native assertions.

## Verdict

**PASS** for the scoped Rust/test implementation. The new helper resolves only from locally available dependency metadata and refuses a Git source outside the root cohort. It adds no production change, copied repository artifact, compatibility layer, unsafe block, ignored assertion, or weakened gate. The author-reported three focused green controls and twelve scratch-lock checks are not independently certified here because their raw logs are on a different host and were not attached to IR-962 at review time. Five remaining controls and one final combined-head gate are still pending with the PR owner.

## Assurance Context

Applicable repository conventions are AGENTS.md and CLAUDE.md. The diff changes no CI workflow, Make target, spec file, dependency manifest, root lock, generated corpus, native agreement case, or assertion. The scratch crate is an ephemeral test output under the Cargo target directory; no external file was committed into the repository. Selected architecture, impact, measurement, exception, independence, structural-gap, and producer-reliance assurance artifacts were not supplied for this confined test repair. The ticket's author handoff is treated as a claim; source and root-lock identity were remeasured from the fetched head.

## Coverage

- FR-014-AC-6 (examined): the scalar and V2 native agreement callers still use the same generated artifacts, support cases, charges, counters and direct runtime comparisons; only the prerequisite scratch lock resolution changed.
- FR-018-AC-2 (examined): admitted composite, enum, and record/tuple callers still dispatch the same native agreement and intended failing mutants through the shared runner.
- FR-018-AC-12 (examined): the standalone generated manifest still compiles with `cargo build --locked --offline`; only its lock preparation changed.
- FR-021-AC-2 (examined): the function agreement caller still uses the shared runner and retained native assertions.
- Rust idioms (examined): checked process status and metadata JSON, bounded iteration over package entries, no new async or concurrency state, no unsafe or production panic surface, no clock assertion and no test-only behavior branch in production code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
