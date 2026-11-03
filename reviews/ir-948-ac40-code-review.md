---
id: SR-202611020185
title: "IR-948 AC-40 clause-oracle harness review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #716 at 6da47f4dc65e367e65ac99751ad0b44172265062; FR-015-AC-40, FR-014-AC-38, TC-025 step 11"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Reviewed the frozen PR diff with Rust/code review, scoped gap analysis, and the base specification checklist. No applicable AssuranceProfile was found under `spec/`. The twelve QSL-authored referenced operators and one IR-admitted inline Boolean literal traverse the public V2 request and compare the embedded oracle source with the clause-identified FR-014 function; the computed Test Matrix tags FR-015-AC-40. QSpec FR-322 at 68be77f8 requires the flat body grammar used by these fixtures. The code adds no copied upstream artifact or compatibility path. The code lead owns the final aggregate gate.

## Verdict

**FAIL pending FND-001 and FND-002.** The inline direct-Boolean cover branch lacks a regression assertion, and two status statements contradict the newly implemented AC-40 evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new inline-literal test checks exact oracle bytes and clause identity but never inspects the direct-Boolean cover. Reverting only the new `if direct_boolean { "outcome" }` cover branch to its old `matches!(outcome, Ok(...))` form leaves both new AC-40 tests green while emitting an ill-typed cover because this oracle returns `bool`. Assert the generated inline harness cover condition, or compile the generated proof path. The existing compile fixture covers a referenced operator clause instead. | tests/it/kani_obligations_state_frame.rs:512; src/kani/generate/clause/v2.rs:411; tests/it/kani_obligations_state_frame.rs:1281 |
| FND-002 | low | Two authored status statements still say AC-40 harness embedding is planned even though this PR marks it implemented and adds its controls. Update the FR-014-AC-38 matrix row and TC-025 section introduction to state the narrower remaining planned work without contradicting the AC-40 row and controls. | spec/oracle/matrix/tests.md:33; spec/kani/matrix/TC-025-bounded-kani-obligations.md:447; spec/kani/functional/FR-015-bounded-kani-obligations.md:851 |

## Coverage

Plan completion: not assessed. The computed `quire matrix --scope . --format json` row for FR-015-AC-40 is tagged by both new tests and prior invariant controls. The QSL table asserts each named operation identity and the clause node identity; the inline fixture asserts the absence of a referenced condition node, zero operations and the clause node identity. Both request a single V2 clause through the public negotiation API and assert the exact FR-014 oracle source occurs once in the returned harness. The typed reader at this point relies on QSpec FR-322's flat application grammar: no nested application, reader shim, copied package fixture or source-authored inadmissible nested clause was added. No CI workflow or dependency file changed. Optional repository-wide semantic gap review was not run; the scoped requirement-to-code-to-test chain was inspected directly.

```yaml
clean: false
scope:
  - id: FR-015-AC-40
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: examined
    excerpt: "A harness for a V2 clause whose admitted condition is an inline Boolean literal or a reference to an admitted Boolean expression node embeds an FR-014 oracle byte-identical to the clause-identified function."
  - id: TC-025-step-11
    path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
    role: examined
    excerpt: "Compare each embedded oracle with the FR-014 crate's clause-identified function byte for byte."
bindings:
  - criterion: FR-015-AC-40
    tests:
      - tests/it/kani_obligations_state_frame.rs::qsl_v2_precondition_embeds_exact_referenced_clause_oracle
      - tests/it/kani_obligations_state_frame.rs::inline_v2_precondition_embeds_exact_clause_oracle
    source:
      - src/kani/generate/clause/v2.rs::lower
      - src/kani/generate/clause/v2.rs::render_clause
      - src/oracle/clause.rs::generate_v2_clause_oracle
```

## Dispositions

Round 1 at 9fd1b75e. **PASS after fixes.** The changed test now guards the exact direct-Boolean cover branch, and the two authored status statements agree with AC-40's implemented controls. Final aggregate gates remain with the code lead.

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | `inline_v2_precondition_embeds_exact_clause_oracle` asserts the generated source contains `kani::cover!(outcome,` and excludes `kani::cover!(matches!(outcome,`, so restoring the old ill-typed branch fails this control. |
| FND-002 | fixed | `spec/oracle/matrix/tests.md:33` now credits the TC-025 AC-40 public harness controls, and `spec/kani/matrix/TC-025-bounded-kani-obligations.md:447` distinguishes implemented AC-40/AC-47 controls from remaining planned rows. |
