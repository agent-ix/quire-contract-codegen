---
id: SR-794002
title: "IR-1518 PR #794 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #794 at 1da7a721; IR-1518; FR-015-AC-135, FR-015-AC-136, TC-025 step 53; src/kani/generate/negotiate.rs, tests/it/v2_clause_obligation_identity.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1518. Plan completion: not assessed. `quire matrix --scope .` lists FR-015-AC-135 as
tagged by three tests (`tests/it/v2_clause_obligation_identity.rs:658`, `:674`, `:710`) and
FR-015-AC-136 by one (`:726`), all `#[trace("TC-025", ...)]`. Each AC clause maps to code and an
asserting test:

- AC-135 refusal and naming: 53(b) asserts `V2ClauseNodeKeyMismatch { node_id == A }`, B still
  supported, only B's harness emitted, and the Debug of the disposition contains none of the key's
  package/path text, the read field names or `selected`.
- AC-135 full equality incl. node ordinal: 53(c), killed by the reviewer's M7 mutant.
- AC-135 selected clause, not another: 53(a)/(b), killed by the reviewer's M5 mutant.
- AC-135 resolver `None` = mismatch: 53(e) by comparison form; confirmed in review,
  `checked_node_key(..).as_ref() != Some(claim.obligation.node())` is true for `None`.
- AC-135 no request member, no hashing: no change to the request types; the check is an equality.
- AC-136 precedence over the record comparison: 53(d) plus its record-only control
  (`V2ClauseClaimMismatch { StateReads }` under A's own key).

No stub, no unowned code: every added production line belongs to AC-135/136.

## Verdict

**PASS (mergeable).** Both criteria are implemented and asserted by non-tautological tests. Two low
gaps on test procedure and unspecified ordering.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-025 step 53(d) specifies "B's key and also a changed parameter domain"; the test changes a state-read domain (`read_a(11)`) because the two-clause fixture has no parameters. Same discriminating power for M8, but test and procedure disagree and no step-53 case covers the parameter member | tests/it/v2_clause_obligation_identity.rs:726-751 |
| FND-002 | low | The key refusal now also precedes every lowering/render and collection-domain refusal (`lower_v2_clause`, `collection_domain_refusal`); the spec pins it only before harness generation and before the record comparison, and no test fixes its order relative to a render refusal (step 14 pins the record refusal ahead of a render refusal) | src/kani/generate/negotiate.rs:952-976 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-135 | examined | A V2 clause claim whose ObligationIdentity NodeKey is not equal, in every member (package, path and node ordinal), to the NodeKey CG resolves for the selected clause node (None counts as a mismatch) is refused with UnsupportedObligation::V2ClauseNodeKeyMismatch, which names the selected clause node and carries neither key nor CG's selection, before harness generation. |
| FR-015-AC-136 | examined | A V2 clause claim whose NodeKey is not the selected clause node's resolved key is refused as V2ClauseNodeKeyMismatch even when its record also differs, so the refusal precedes every record comparison. |
| TC-025 step 53 | examined | Two clauses A and B, B listed first, hand-written expected keys; cases (a)-(e); mutants M4-M8. |
| FR-015-AC-48 | context_only | Carried identity and record cross-check; residual-gap text updated by this PR. |
