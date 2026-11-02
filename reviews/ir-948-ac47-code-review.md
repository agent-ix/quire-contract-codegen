---
id: SR-202611020001
title: "IR-948 AC-47 code and specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #714 at 10d7c095137d44045a0d2f89b0ccacff00a8b421; FR-015-AC-47 and TC-025 step 14"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Reviewed the PR diff using Rust/code review, scoped gap analysis, and the base specification checklist. No applicable AssuranceProfile was found under `spec/`. The change adds no production branch, dependency, vendored artifact, workflow change, or gate narrowing. The computed matrix tags FR-015-AC-47 from both changed integration tests and an existing invariant test. The coder's focused scalar and collection targets each passed; aggregate gates belong to the code lead.

## Verdict

**FAIL pending FND-001.** The new tests check visible source and record contents, but remove the previous whole-harness equality assertion that also checked the sealed generated artifacts.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Both edited equal-input tests replace `assert_eq!(first, same)` with equality of visible source contents, record contents, and identity only. `KaniV2ClauseObligationHarness` also contains artifact metadata and private `generated_identity`, `generated_rust`, and `generated_record` fields. A regenerated harness with different sealed fields could pass the new assertions, weakening the existing test and the AC-47 whole-harness determinism claim. Keep the explicit byte assertions and restore full `assert_eq!(first, same)` in both tests. | tests/it/kani_obligations_state_frame.rs:1097; tests/it/kani_collection_operation.rs:732; src/kani/identity.rs:360 |

## Coverage

Plan completion: not assessed. The changed tests exercise authentic QSL precondition, postcondition, invariant, and collection postcondition generation. Each checks identical source and record bytes for equal requests and identity variation for changed unwind or subject. The invariant control checks that neither customer subject path appears in generated source. The FR-015 status edit and TC-025 scenario agree with these controls after FND-001 is fixed. There is no new requirement statement or relationship edge requiring a separate EARS or dependency analysis.

```yaml
clean: false
scope:
  - id: FR-015-AC-47
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: examined
    excerpt: "Equal V2 clause inputs regenerate byte-identically; changing unwind or customer subject changes the harness identity record."
  - id: TC-025-step-14
    path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
    role: examined
    excerpt: "Regenerate equal V2 requests and vary unwind and subject."
bindings:
  - criterion: FR-015-AC-47
    tests:
      - tests/it/kani_obligations_state_frame.rs::qsl_v2_scalar_clause_identity_tracks_request_facts
      - tests/it/kani_collection_operation.rs::whole_object_identity_regenerates_and_changes_with_unread_domains
    source:
      - src/kani/identity.rs::KaniV2ClauseObligationHarness
      - src/kani/generate/clause/v2.rs::lower
      - src/kani/generate/render/collection_operation.rs
```

## Dispositions

Round 1 at 08e31574cd24f89bd7077659e45ae6074da48b52. **PASS after fix.** Both focused integration targets passed again, one target per shared-lock acquisition. The code lead owns the final aggregate gate.

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | Both equal-input controls again assert full `KaniV2ClauseObligationHarness` equality. They retain explicit source and record byte comparisons, plus unwind and customer-subject identity-variation checks. |
