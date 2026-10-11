---
id: SR-7201
title: IR-746 candidate gap analysis
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@952e81e5d694f8ec3e2c6ae320b56a45d178ef02; AD-004 L-9, NFR-005-AC-1, FR-015-AC-93, FR-015-AC-94, FR-028-AC-1, TC-023, TC-042, changed source and integration tests"
---

## Summary

Pre-PR candidate trace and evidence check for IR-746 at the exact local head above. The computed repository matrix has unrelated existing untagged criteria; this review scopes findings to obligations affected by the diff. Plan completion: not assessed.

## Examined Units

- AD-004 L-9 (examined): one canonical encoder boundary, bounded canonical generated JSON, and a distinct canonical corpus-name preimage.
- NFR-005-AC-1 (examined): zero production panic tokens with no corpus digest exception.
- FR-015-AC-93 (examined): typed corpus refusals without an artifact or registry claim.
- FR-015-AC-94 (examined): corpus artifact identity invariance and sensitivity to every identity field.
- FR-028-AC-1 (examined): ceiling sensitivity of proof identity.
- TC-023 (examined): corpus generation, canonical names, and artifact bytes.
- TC-042 (examined): no-generation-panic scan.

## Verdict

**CONDITIONAL.** The new canonical corpus name needs an independent expected-value assertion. Existing source-map and graph tests cover changed artifact bytes; the over-bound identity and proof-graph projection tests reach typed refusal paths. The computed matrix's pre-existing repository-wide gaps are not attributed to this candidate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The canonical corpus-name obligation has no test that checks the emitted digest against an independently known RFC 8785 value; the current relational assertions would pass with field-order serde hashing. | src/kani/generate/corpus/bounded_kani_corpus.rs:1716 |

## Coverage

Plan completion: not assessed. `quire matrix --scope . --strict --format json` returned exit 1 with 281 repository-wide untagged criteria and 11 criteria tagged only by ignored tests. The result was not used as a fresh finding for unchanged obligations. The changed `TC-023` and `TC-042` checks were read for behavioral oracle strength, including the old serializer and `expect` mutants.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b259f3eaaf72cfda87739eeb2f8e787026593cca |

After correction (`src/kani/generate/corpus/bounded_kani_corpus.rs:950-981`, `b259f3eaaf72cfda87739eeb2f8e787026593cca`): the hand-authored canonical preimage uses the declared `source_id` field in the arithmetic struct variant, because enum-level `rename_all = "camelCase"` changes the variant name only. The 635-byte literal independently hashes to fixed SHA-256 `4afb9c72a943cbd2db73a206e083012480ee4218adb534e96cbc13c44cc7a525`; the generated name is asserted against `arithmetic_<digest>`. The initial 634-byte `sourceId` literal failed its focused test; the corrected test selected one and passed at this SHA, 2026-10-11 04:11:03–04:11:07 UTC (`/tmp/ir746-focused-corrected.meta`). The FND-001 oracle gap is closed.
