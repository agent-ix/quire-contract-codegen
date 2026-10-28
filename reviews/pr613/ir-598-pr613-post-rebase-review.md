---
id: SR-613008
title: 'Post-rebase review — IR-598 PR #613 collection retirement'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen PR #613 at c9f56be13e91860d6f210135d610991527e0677e, rebased onto 43521146; conflict resolution, spec status, and collection retirement preservation'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

Independent narrow review of the post-rebase PR head. Earlier code/Rust/gap findings are recorded in SR-613001 and SR-613002. All previously reviewed production Rust, schema, and corpus test blobs are unchanged from a2c8ceb7. The integration test difference is the merged #619 AC-100 launcher control. The collection arm, finite collection lowerer, public export, and schema family remain removed, while the graph arm remains. FR-015-AC-96, AC-100, AC-101, and AC-123 normative text is preserved; AC-100 has a tagged launcher control, and AC-101 representability has tagged typed-refusal/no-harness tests. The upstream QSL/IR selected-field tests are still not claimed by CG.

## Verdict

CHANGES REQUESTED for stale status text. The source and test rebase is review-clear. No full gate or real-Kani run was performed for this narrow review. The AC-110 Kani precondition remains unverified on this exact head and is recorded separately in SR-613002. This artifact does not assert that the whole IR-598 ticket is complete; graph retirement remains FR-015-AC-111.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-015 overview claims AC-100 and AC-101 controls are untagged despite the updated criteria and tests | spec/kani/functional/FR-015-bounded-kani-obligations.md:125-129 |
| FND-002 | low | Test Matrix says the collection corpus and lowerer remain live after this PR removes them | spec/kani/matrix/tests.md:35,38,111 |

## Finding Detail

### FND-001

The overview says AC-101's representational-ceiling clause has no tagged test and AC-100 lacks an AC-100-tagged ordered-options comparison. The AC-101 row now cites two tagged generator tests, and the AC-100 row cites `collection_run_evidence_preserves_every_ordered_identity_option`, tagged AC-100/TC-025. Both criteria remain PARTIAL for their other clauses; update the overview to name the actual gaps.

### FND-002

The TC-023 rows still say collection and graph corpus paths and finite-input lowerers remain, and that AC-110/111 are planned because both remain live. On this head the collection arm, lowerer, and public operation are absent; only graph remains. Keep AC-110 partial or pending its stated Kani precondition and AC-111 planned, but describe the current code accurately.

## Examined Scope

```yaml
scope:
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The feasible-unique clause remains PARTIAL and capacity refusal remains separately planned.'
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Collection run evidence records solver kind, unwind and options in invocation order; the tagged launcher control compares them.'
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Representability is tagged; source-located QSL and Contract IR selected-field controls remain open.'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Collection corpus removal and V2 harness control are present; stated real-Kani precondition remains unrun on this head.'
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Retired collection operation has no replacement alias or wrapper; retained graph operations keep direct signatures.'
bindings:
- test_id: collection_run_evidence_preserves_every_ordered_identity_option
  ac_id: FR-015-AC-100
  trace: correct
- test_id: domain_representability_failures_keep_the_typed_reason
  ac_id: FR-015-AC-101
  trace: correct
- test_id: rendering_refuses_an_unrepresentable_element_domain_without_a_harness
  ac_id: FR-015-AC-101
  trace: correct
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | TC-023 still describes retired collection corpus controls as active current tests and results | spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:13-22,42-43,103,159-174,198-201 |

### FND-003

The new TC-023 status at lines 24 and 190 correctly says this slice removes the collection corpus path. But the document still says current AC-94 graph/collection corpus identity controls remain; the procedure still asks for collection bound exhaustion; Expected Results says real-Kani false collection corpus cases replay; the implementation status cites retired collection Kani tests and says graph/collection replay remains; and IR-621 status says three public results and collection bounds remain. The current source has graph-only corpus requests and no exported collection lowerer. These are false current-state/test claims, beyond the original Test Matrix finding.

## Dispositions

Disposition round 1 at exact head `d3628d5f60295a79d03ff8dc86c01f207e108b1e`. The diff from the review head `c9f56be1` edits only FR-015 overview, TC-023 and the Test Matrix. No Rust, full gate or real-Kani run was performed. Both original status findings are fixed. FND-003 was discovered while checking TC-023 as requested.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-015 overview now names the AC-100 tagged collection launcher control and AC-101 tagged representability controls, with the graph and source-located boundary gaps still partial. |
| FND-002 | fixed | Test Matrix now says the collection corpus arm and lowerer are removed, with AC-110 exact-head real-Kani evidence pending and AC-111 graph retirement planned. |
| FND-003 | still-open | TC-023's other current-state and expected-result passages still claim retired collection corpus tests, requests, replay, and public results remain active. |

### Disposition round 2

Disposition round 2 at exact head `80c5f23e10b2f798224ae2565a56dfee7d5d02d2`, reviewing only the TC-023 spec diff from `d3628d5f`. FND-003 is fixed: current graph controls and historical collection controls are distinguished throughout Description, Procedure, Expected Results and implementation status. The three named graph Kani/replay tests and the retained external consumer test exist; retired collection names are absent from `src/` and `tests/`. `quire validate` of TC-023 passed. No build, full gate or Kani run was performed. Source/test rebase remains review-clear; AC-110 exact-head real-Kani evidence remains pending under SR-613002, and graph/final retirement remains AC-111.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | TC-023 now names only retained graph corpus cases, tests, replay and public results as current; former collection controls are explicitly historical, and its deletion is stated. |
