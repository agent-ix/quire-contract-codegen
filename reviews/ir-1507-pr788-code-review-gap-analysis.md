---
id: SR-788002
title: "IR-1507 PR #788 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #788 at 12d9a54c; IR-1507; FR-015-AC-48; TC-025 step 14; tests/it/v2_clause_obligation_identity.rs; tests/common/v2_claim.rs; tests/it/kani_collection_operation.rs; src/kani/generate/negotiate.rs (claim_for)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1507. Planless gap analysis of FR-015-AC-48 against its tagged tests and the code. `quire matrix` lists AC-48 as `tagged` by three tests in `tests/it/v2_clause_obligation_identity.rs` (lines 95, 136, 201), each carrying `#[trace("TC-025","FR-015-AC-48")]`; no `/// Trace:` lines were added. Plan completion: not assessed.

A mutation control was run. At 12d9a54c, `seal_with_selected` was changed to compare the claim against an empty selection (`(Vec::new(), Vec::new())`) instead of the clause's own arguments and state reads. The focused job (`cargo test --test it -- v2_clause_obligation_identity kani_collection_operation`) then passed all 19 tests, exactly as at baseline. The mutation was reverted.

## Verdict

**CHANGES REQUIRED.** The identity-carry tests are sound: the identity is carried unchanged, unwind and subject leave it unchanged, and a changed ordinal leaves the AC-47 record bytes equal. The occurrence and kind refusals are exercised. The argument and state-read half of AC-48 cannot fail as tested, because every claim, including the AC-48 matching control, is read back from the generator's own refusal. The whole-object collection route has no mismatch test at all.

The ~40 non-AC-48 call sites that use `claim_for` / `claims_for` are acceptable scaffolding, since those tests do not claim AC-48. The AC-48 tests themselves must not use the same bootstrap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Mutation survived: the argument and state-read cross-check can be made to ignore the clause, and every AC-48 test still passes. `claim_for` builds the record by negotiating once and copying `selected` back from the `V2ClauseClaimMismatch` refusal. A generator that selects the wrong parameters, reads or domains (or none) therefore produces a claim that agrees with it. The `extra_*` cases only show that an appended entry differs. The `widened` cases run only `if let Some(first)`, and nothing asserts that the record is non-empty. Fix: for each of the three fixtures, write the expected record by hand from the source, with parameter names, declaring node and field, and the `Int[..]` domains. Assert the control against that record, assert that the arguments and reads are non-empty where the clause has them, and add a renamed-parameter case and a changed-declaring-node case. | tests/it/v2_clause_obligation_identity.rs:201-286; tests/common/v2_claim.rs:30-80 |
| FND-002 | medium | The whole-object collection route of AC-48 is untested. No test drives `V2ClauseClaimMismatch` through `render_collection_operation`, and `operation_selected_reads` is unverified: the sequence cardinality used as a domain, and every drawn object field counted as a read. The `kani_collection_operation` tests only use the matching `claims_for`. The AC-48 row nevertheless says it is implemented for this route. Add a tagged whole-object fixture with matching, changed-parameter, changed-read and changed-cardinality cases. | src/kani/generate/collection.rs:612-663; tests/it/kani_collection_operation.rs:110-121 |
| FND-003 | low | `span_unwind_and_subject_leave_the_identity_and_the_ordinal_leaves_the_harness_record` varies unwind, subject and ordinal but never the source span. The AC-48 clause "changing only the span ... leaves the carried identity unchanged" is unevidenced. Add a span-only variant of a fixture package, or rename the test and record the span as untested. | tests/it/v2_clause_obligation_identity.rs:136-188 |

## Coverage

Bindings checked: the three tests in `tests/it/v2_clause_obligation_identity.rs` are bound to FR-015-AC-48. The binding is correct for the identity-carry and exclusion parts, and the evidence is insufficient for the record cross-check (FND-001, FND-002). Tests changed only for the new field (state_frame, symbolic_domains, integer_literals, collection_operation, and the in-crate negotiate and v2 tests) are bound to their own ACs, and those bindings are unchanged. No stubs, `todo!` or placeholder production paths were added.

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-015-AC-48 | examined | spec/kani/functional/FR-015-bounded-kani-obligations.md:860 | CG compares the complete source occurrence, obligation kind, parameter identities and declared domains, and any state-field read identities and domains, as separate obligation-record fields against the independently selected claim ... A mismatch refuses before harness generation even when structural identity is unchanged. ... changing only the span, unwind bound or subject leaves the carried identity unchanged. |
| TC-025-step-14 | examined | spec/kani/matrix/TC-025-bounded-kani-obligations.md:547-571 | AC-48 evidence is `tests/it/v2_clause_obligation_identity.rs`: the carried identity equals the supplied one for a changed node, model and ordinal; unwind and subject changes leave it unchanged while the harness identity changes; and a changed occurrence, kind, parameter identity or domain, or state-field read identity or domain refuses with the named record member and no harness, while the matching control is admitted. |
| FR-015-AC-47 | context_only | spec/kani/functional/FR-015-bounded-kani-obligations.md:859 | Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head b1fa48c8: the AC-48 tests build records by hand from the fixture sources (`other`/`amount` over 0..=1000, `balance` read on the Account declaration), assert they are non-empty, and add renamed, narrowed, missing and other-declaring-node cases. Reviewer mutation (state-read comparison ignores domains) fails `a_state_read_mismatch_refuses_before_any_harness` and `whole_object_claim_is_carried_and_every_record_member_is_cross_checked`. Baseline 24/24 pass. |
| FND-002 | fixed | Round 1: two tagged whole-object tests in `tests/it/kani_collection_operation.rs` use a hand-written claim and nine mismatch cases (occurrence, kind, renamed, domain and dropped parameter, cardinality, field name, declaring node, unread field), plus a mismatch reported ahead of a render refusal. |
| FND-003 | fixed | Round 1: `a_changed_source_span_alone_changes_neither_the_identity_nor_the_record` shifts the span with `Twin::with_blank_lines_before`, asserts the regions differ, and finds the obligation, the identity and the record bytes equal. |
