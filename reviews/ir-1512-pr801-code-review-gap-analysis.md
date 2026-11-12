---
id: SR-17940486709564847047722
title: "IR-1512 PR #801 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #801 at 4ca035e6; IR-1512; FR-024-AC-58, FR-024-AC-69, FR-024-AC-70, FR-029-AC-16, TC-035 steps 40, 48, 49; src/replay/state_clause/invariant.rs, tests/it/state_clause_invariant_replay.rs, tests/it/terminal_map.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1512. Plan completion: not assessed. `quire matrix` shows FR-024-AC-58 and
FR-024-AC-69 tagged on `tc_035_unread_declared_field_is_completed_from_the_supplied_value`
(state_clause_invariant_replay.rs:347), FR-024-AC-70 on
`tc_035_each_unread_value_refusal_is_a_distinct_variant_before_any_document` (:485) and
FR-029-AC-16 on `tc_040_the_state_clause_replay_reads_as_fr029_ac16` (terminal_map.rs:775). The
new production code (`complete_snapshot`, `clause_read_fields`, the two variants) is owned by
FR-024-AC-58/AC-70 and interface-001; no unowned code. No stub, no tautological assertion: each
assertion is on a decoded wire document, a typed variant with its field, or QSL's own result.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | AC-69 and TC-035 step 48 say the snapshot is decoded from the request captured through `replay_through`; the test reads `replay.wire.documents` of the built replay before it runs (the same bytes `replay_through` hands over, so equivalent, but not what the step says) | tests/it/state_clause_invariant_replay.rs:209-219 |
| FND-002 | low | AC-58's own clause that an unsupplied unread field is `MissingField` (step 40, tagged FR-024-AC-58 and AC-70) is asserted only in the test tagged FR-024-AC-70; the AC-58 test does not assert it | tests/it/state_clause_invariant_replay.rs:505-509 |

## Verdict

Every clause of AC-58, AC-69 and AC-70 that names an outcome has an executed assertion, apart
from the negative-range and string/unbounded sub-cases recorded in the code-review SR. Step 40, 48
and 49's mutation controls match what the tests kill (confirmed by the coder's list and the
reviewer's M4/M5).
