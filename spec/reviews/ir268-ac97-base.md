---
id: SR-11650
title: "Base review of IR-268 unbounded collection dispositions"
type: SpecReview
analysis: base
scope: "FR-015-AC-97 and the corresponding TC-025 amendment"
review_set: base
org: agent-ix
---

## Summary

The corrected amendment passes the base checklist for this diff; the initial refusal-identity mismatch is resolved. The two missing-upper-bound cases match QSL's source contract: zero minimum admits an unbounded collection, while a positive minimum has no representable kernel type and is refused at QSL assembly. The amendment preserves the prohibition on substituting a finite ceiling and preserves unsupported collection, element, and predicate dispositions. The lead directed completion of the base checklist only for this narrow amendment; no optional analysis is claimed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | AC-97 requires a missing-upper-bound disposition naming the field, but TC-025 permits either the selected field or its unbounded collection type. The source-backed control asserts only the projected collection type. Align the required identity and its test expectation so a passing test establishes the criterion as written. | FR-015-AC-97; TC-025 Test Procedure; tests/it/kani_obligations_symbolic_domains.rs::qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness | wrong-requirement |

## Source assessment

QSL FR-056-AC-10 explicitly admits unbounded collections only with lower bound zero. QSL assembly's `AssemblyCause::UnsupportedModelMember` maps to `Code::UnknownRequiredFeature` and diagnostic `unsupported-feature`, confirming the upstream refusal terminology. The added test procedure uses lower one as its positive-minimum representative and prevents misreporting that refusal as an IR item disposition.

Read-only inspection of the active implementation worktree found three executable source-backed controls in `tests/it/kani_obligations_symbolic_domains.rs`: `qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness`, `qsl_rejects_a_missing_upper_bound_with_a_positive_minimum`, and `finite_sequence_of_unbounded_integer_elements_requires_bound_when_qsl_admits_it`. Their common helper calls QSL `call_site` on authored model/source bytes, then Contract IR `CheckedPackageV2::read`; it does not manufacture a V2 package. The unbounded-element control replaces the model field's type with native Integer, checks the admitted Collection<Integer> shape with finite cardinality, and requires a typed element RequiresBound plus no collection harness. The zero-minimum control checks bounds None and no collection harness. The positive-minimum control requires QSL's UnknownRequiredFeature and the selected Node/items location. Its early QSL failure prevents constructing an admitted item. These controls are in a separate uncommitted code worktree, not delivered by this spec amendment; no runtime pass is inferred from source inspection.

## Checklist and coverage

The changed criterion retains its existing valid unique ID and Test verification method. TC-025 retains its verifies relationship to FR-015. The changed content introduces no tracking artifact or tool-version pin; it adds a direct requirement reference to QSL FR-056. No applicable AssuranceProfile with review_selection was found in the scoped spec tree. The scope contains no changed user story, option, or state transition; cardinality partition, unsupported path, and unbounded-element edge are all described explicitly.

The computed matrix for the spec worktree reports FR-015-AC-97 untagged. This remains a declared PLANNED implementation obligation, not delivered proof or closure evidence. The three controls inspected above carry AC-97 tags in the active code worktree, but are not in this spec branch. The reviewer does not infer complete coverage or Kani/native replay qualification from them.

## Validation

The installed Quoin SpecReview skeleton and schema were fetched before authoring. SR-11650 was absent from the repository, relevant review scratch artifacts, and task logs before this document was created. Scoped Quire validation of `spec/**/*.md` exited 0; the only emitted document warnings were pre-existing EARS warnings in FR-014 and FR-017, outside this amendment. `git diff --check` also passed. Quoin module discovery emitted existing duplicate-archetype/inverse-edge and inline-schema advisories. Base checklist only; no optional analysis is represented by this artifact.

## Disposition

FND-001 is resolved. The revised AC-97 now permits `requires_bound` naming the selected field or unbounded collection type, matching both TC-025 and the inspected control. This restores the prior criterion's field-or-type policy and does not change the no-substituted-ceiling clause. No open findings remain in this spec diff.

The lead supplied the coder's focused receipt: `CARGO_TARGET_DIR=/home/peter/dev/worktrees/ir-268-symbolic-domains/target-ir268-fixture CARGO_BUILD_JOBS=2 cargo test --locked -p quire-contract-codegen --test it kani_obligations_symbolic_domains -- --nocapture` reported 10 passed. The reviewer independently inspected the three negative control bodies and their shared QSL-emission/IR-admission path, but did not rerun Cargo or independently inspect a saved terminal log. This supports source admissibility of the amendment; it does not close IR-268 or claim real Kani/playback coverage.
