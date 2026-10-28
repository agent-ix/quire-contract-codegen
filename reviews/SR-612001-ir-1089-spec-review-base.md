---
id: SR-612001
title: "base spec review of PR #612 (FR-018-AC-7 reference reader causes, TC-029)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-018-composite-equality-oracles.md (FR-039 accounting prose, FR-018-AC-7); spec/oracle/matrix/TC-029-composite-equality-oracles.md (steps and expected result)"
review_set: base
---

## Summary

Ticket: IR-1089. PR: quire-contract-codegen#612 at frozen head
af01bb065bb530b6e5fb7663465d4eecb7f11d4c, diffed against origin/main fb50acdc.
The diff is spec-only: it edits the FR-018 FR-039 accounting prose, the FR-018-AC-7 row and
TC-029's steps and expected result.

The review ran the base checklist only. The diff adds no requirement statement, no
`relationships:` edge and no new object, so the EARS, dependency and object lenses do not
apply. The repository has no `AssuranceProfile` document, so no profile requires an enforced
spec-review run.

The causes and loci are correct. I checked them against Contract IR source at IR main
250fc959. For a `composite_type/reference` node whose target is not
`Model(ObjectType|Process|StateMachine|SystemsInterface)` or `Relation(Relationship)`,
derived-key verification returns `ValidationFailure::refused_at(InvalidPackage, …,
Some(InvalidDerivedShape), node.node_id)`. That is the reference type's own node id. When
the target is valid, `check_conforming_reference` refuses through `ModelRefusal::ineligible()`
(`IllTyped`, `OperatorIneligible`) at `/body/arguments/<n>`. The previous text's
`InvalidPackage/InvalidValue` was wrong. At CG's locked IR rev a98e63a2, the traced test
`tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today` already asserts
`IllTyped/OperatorIneligible`.

The PR does not narrow AC-7. It keeps the reader-stage refusal ("not CG blockers"), each
root's own requested key, node tag and source occurrences, the unchanged independent
admitted sibling, and the `call` / quire-contract-runtime#34 clause word for word. The
single generic reader-rejection clause becomes two named controls. The failure-mode row,
"allow a strict-reader-invalid reference composite into CG", stays generic.

Three defects remain. Two are medium and one is low.

## Findings

| ID      | Severity | Summary | Refs | Escape Cause |
| ------- | -------- | ------- | ---- | ------------ |
| FND-001 | medium | The cited Contract IR line ranges `derived_keys.rs:452-475` and `operations.rs:2677-2738` do not resolve to the cited code at IR main 250fc959. There the reference-target check is at lines 650-677 and `check_conforming_reference` at 2852-2915. At main, lines 2677-2738 are relationship end-selection code. At CG's locked IR rev a98e63a2 the derived-shape check does not exist; it landed in IR df9ed4ad. The ranges match only the intermediate IR commit 79524d43. Line numbers into another repo with no revision decay with every IR commit, so a reader following them lands on unrelated code. Cite the symbols instead: the `CompositeTypeForm::Reference` target check in `derived_keys.rs` key verification, `check_conforming_reference` in `operations.rs`, and `ModelRefusal::ineligible` in `model_members.rs`. Apply this in both files. | spec/oracle/functional/FR-018-composite-equality-oracles.md:553-555; spec/oracle/matrix/TC-029-composite-equality-oracles.md:89-91 | wrong-requirement |
| FND-002 | medium | TC-029 says to "retain the independently admitted reference-bearing record and unchanged healthy sibling", and its expected result says that record "remains admitted". It does not say these live in a package separate from the two controls. Both reader refusals refuse the whole package as `CheckedPackageV2ReadResult::Refused`: the derived-shape refusal returns `Err` from key verification, and the operator refusal is an `IllTyped` read refusal. So nothing in a control's package stays admitted. The corpus's current admitted reference-bearing record `R_WITH_REF` binds `REF_TYPE`, which targets the record `R_POINT`. That is exactly the record-value-type target the new AC-7 says IR refuses as `InvalidDerivedShape`. Read literally, the "retain the admitted sibling" step either cannot be satisfied or keeps a fixture the AC declares invalid. TC-029 should state three things. Each reader control is its own package. The admitted reference-bearing record and its healthy sibling live in a separately admitted package. That record's reference targets a permitted model declaration. | spec/oracle/matrix/TC-029-composite-equality-oracles.md:84-89,565-569; tests/composite_equality_support/package.rs:1329-1347 | wrong-requirement |
| FND-003 | low | AC-7 calls the valid case "a permitted model object target". The FR-039 prose calls it "Contract IR's permitted model declaration kinds". IR permits model forms `ObjectType`, `Process`, `StateMachine` and `SystemsInterface`, plus `Relation(Relationship)`. "Model object target" reads as object types only. A `Process` or `StateMachine` target passes the derived-shape check and is then refused at equality as `OperatorIneligible`, because `object_type()` is `None`. The narrower wording therefore leaves unclear which AC clause covers that case. Align the AC wording with the FR prose ("permitted model declaration target"), or list the permitted kinds. | spec/oracle/functional/FR-018-composite-equality-oracles.md:683 | wrong-requirement |

## Verdict

**Changes requested.** FND-001 and FND-002 should be fixed in this PR before merge. FND-003
is a one-word alignment and should go in the same fix round.

These items were checked and are clean:

- The new causes and loci agree with IR source. The derived-shape refusal sits at the
  reference type node id; `OperatorIneligible` sits at the offending argument, which for a
  non-conforming pair is the second operand.
- The PR removes the wrong `InvalidValue` cause.
- The PR keeps the reader-stage classification, the requested-key, node-tag and
  source-occurrence accounting, the independent-admitted-sibling clause for Unsupported
  roots, and the `call` blocker unchanged.
- The TC-029 corpus list names both controls.
- The PR adds no hash, digest or pin.

Context, not findings against this diff:

1. CG's `Cargo.lock` pins `quire-contract-model` at IR a98e63a2, which predates the
   derived-shape check (IR df9ed4ad). Under the locked IR, the traced test
   `tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today` uses a record-target
   `REF_TYPE` and correctly measures `IllTyped/OperatorIneligible`. After the IR bump that
   input becomes the `InvalidDerivedShape` control. Updating the test belongs to the
   ticket's follow-up criterion and the IR-bumping consumer PR. AC-7 stays `🚧 Planned` in
   `spec/oracle/matrix/tests.md`, which is consistent with that.
2. A pre-existing inconsistency sits outside this diff. The FR-015 unsupported-shape table
   (`spec/kani/functional/FR-015-bounded-kani-obligations.md:713`) says a `reference`
   composite is "`unsupported`, blocked on quire-spec-language#120, as FR-018-AC-7 states".
   FR-018-AC-7 states a reader-stage control, not that blocker.

## Dispositions

Round 1. Fix delta af01bb065bb530b6e5fb7663465d4eecb7f11d4c..560219800788e03a792b56f107822811e54be095,
reviewed at frozen head 560219800788e03a792b56f107822811e54be095. GitHub PR head matches. The PR
is still a draft.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-018 prose and TC-029 step now cite symbols, not line numbers: the `CompositeTypeForm::Reference` target check in `derived_keys.rs`, `check_conforming_reference` in `operations.rs`, and `ModelRefusal::ineligible` in `model_members.rs`. All three resolve at IR main 250fc959 (`derived_keys.rs:650`, `operations.rs:2855`, `model_members.rs:152`). |
| FND-002 | fixed | TC-029 now puts each reader control in its own package. One refused package holds the record-target reference (`InvalidDerivedShape`). A second holds the valid-target ineligible equality (`IllTyped/OperatorIneligible`). The reference-bearing record lives in a separate admitted package and targets a permitted model object type, beside an unchanged healthy sibling. The step and the expected result agree. |
| FND-003 | fixed | AC-7 now lists all five permitted targets: model object type, process, state machine, systems interface and relationship. That matches IR's `Model(ObjectType\|Process\|StateMachine\|SystemsInterface) \| Relation(Relationship)`. The FR prose now says "permitted model or relationship declaration kinds". |

I checked the delta for regressions and found none:

- The causes and loci are unchanged and still match IR.
- AC-7 keeps its reader-stage clause, the requested-key, node-tag and source-occurrence
  accounting, the independent admitted sibling for Unsupported roots, and the
  `call`/quire-contract-runtime#34 clause word for word.
- The FR-018 failure-mode row is unchanged.
- The admitted package's "permitted model object type" target is a TC witness choice. It does
  not narrow the AC.
- The committed `reviews/SR-612001-ir-1089-spec-review-base.md` is the reviewer artifact,
  with two stray trailing tool-markup lines removed. Those lines were the reviewer's
  authoring error.
- `quire validate` passes on both spec files and on the committed SR.

Round-1 verdict: review-clear. Every finding is fixed.
