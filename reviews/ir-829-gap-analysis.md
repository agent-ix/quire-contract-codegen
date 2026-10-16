---
id: SR-11966
title: "IR-829 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@ae138524cf6c4d0eeeb78589999f47d109f4acb3; spec/core/functional/FR-005-cli-conformance.md, spec/core/non-functional/NFR-001-reproducible-atomic.md, spec/core/non-functional/NFR-005-no-generation-panics.md, src/publication/publish.rs, src/core/artifact.rs, src/routed/generate.rs, src/kani/generate/scalar.rs, src/evidence/bound_coverage.rs, tests/it/no_generation_panics.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: references
---

## Summary

Ticket: IR-829. Planless gap analysis. Plan completion: not assessed. Computed matrix on origin/main (494937f) and head (ae13852): `quire coverage --strict` 221 to 219 unbacked rows, 0 contradicted, reproduced. `quire matrix` delta: FR-005-AC-6..9 newly tagged; FR-005-AC-3 moves from method-without-symbol (Inspection) to untagged (Test); NFR-001-M-1..3 and NFR-005-M-1..3 newly tagged. Each new binding was read against the test body it names.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-005-AC-6 | examined | When an artifact path is absolute, or contains a `..` segment, a `.` segment, an empty segment, a trailing separator or a backslash, or aliases an earlier path in the same bundle, bundle construction is refused with `UnsafeArtifactPath` naming the offending `bundle.artifacts[i].path`, before any destination or staging I/O begins. |
| FR-005-AC-7 | examined | When publication fails before the destination is replaced, or fails after replacement began and the prior destination is restored, the publisher reports destination state `unchanged`, ... |
| FR-005-AC-8 | examined | A failure to remove the replaced bundle's backup after the staged bundle has replaced the destination is reported with destination state `published`; ... |
| FR-005-AC-9 | examined | When replacement fails and restoring the prior destination also fails, the publisher reports destination state `unknown`, ... |
| FR-005-AC-3 | examined | For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row ... |
| NFR-001-M-1 | examined | Byte differences between repeated generations / 0 / 0 / Test (TC-001) |
| NFR-001-M-2 | examined | Partial published bundles after injected failure / 0 / 0 / Test (TC-002) |
| NFR-001-M-3 | examined | Developer-owned files modified / 0 / 0 / Test (TC-002) |
| NFR-005-M-1 | examined | Panic tokens in the non-test, literal-free code of `src/` / 0 / 0 |
| NFR-005-M-2 | examined | Measured sites a test can reach that abort instead of returning a typed value / 0 / 0 |
| NFR-005-M-3 | examined | Index, slice and unchecked subtraction expressions in the bodies of the five IR-577 functions / 0 / 0 |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-005-AC-8 is bound to `post_commit_cleanup_failure_reports_that_the_new_bundle_is_published`, which drives the `AfterCommitBeforeBackupCleanup` injection; that branch returns `injected_with_state(.., Published)` before cleanup runs. The production branch the AC describes, `if let Err(mut error) = cleanup(&backup, ..) { error.destination_state = Published }`, is never executed by any test, so deleting the state assignment survives. The test also checks one of the two new artifacts ("complete new bundle") and does not check that the one residue entry is the backup sibling. The matrix marks AC-8 Covered. | src/publication/publish.rs:136-148, src/publication/publish.rs:407-437, spec/core/matrix/tests.md:18 |
| FND-002 | medium | NFR-001-M-1 (byte differences between repeated generations) is tagged on `publication_is_order_independent_and_replaces_only_its_destination`, which never generates anything twice: it asserts that `ArtifactBundle::new` sorts a reversed artifact list equal to the original and then publishes two different bundles. No byte comparison of repeated generation exists in that test. | src/publication/publish.rs:302-332, spec/core/non-functional/NFR-001-reproducible-atomic.md:34 |
| FND-003 | low | FR-005-AC-6 requires the refusal to name the offending `bundle.artifacts[i].path`, but the per-form loop over the eight noncanonical paths asserts only `error.code`; the path is asserted once, for the `a/b` + `a/./b` pair. | src/publication/publish.rs:441-467 |
| FND-004 | low | FR-005-AC-7 requires that the destination hold its prior bytes; the test reads only `src/generated.rs`, and the second artifact `src/generated/nested.rs` is `{}` in both old and new bundles, so a restore that kept the new nested file is not distinguishable. | src/publication/publish.rs:334-368, src/publication/publish.rs:281-287 |

## Verdict

CHANGES REQUESTED (two medium, two low). Correct bindings, recorded as clean: FR-005-AC-6 forms (absolute, `..`, `.`, empty segment, trailing separator, backslash, alias pair) are each exercised against `UnsafeArtifactPath` at construction, which takes no destination, so the no-I/O clause holds by signature; FR-005-AC-7 covers both triggers (BeforeArtifact/BeforeSwap, and DuringSwap with `replacing = true`, which runs the real rollback) and asserts `Unchanged`, absence, the developer file and empty residue; FR-005-AC-9 asserts `Unknown`, absent destination, and every artifact of both the backup and staged siblings through the real rollback-failure branch. NFR-001-M-2 and M-3 bind the injected-failure test, which asserts no partial bundle and an unchanged developer file. NFR-005-M-1 binds the `src/` panic scan, M-3 the five-body scan, and each of the seven M-2 tests (scalar.rs AC-2, the it AC-2 and AC-3, generate.rs AC-5 and AC-6, bound_coverage.rs AC-4 and AC-7) asserts the typed refusal at its site, covering the six sites the metric names. FR-005-AC-3 is correctly untagged and Planned: the register is empty and no test joins differing cases against it. FR-039-AC-4/5 are untagged as the matrix now says.
