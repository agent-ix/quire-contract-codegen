---
id: SR-654007
title: "IR-1091 CG654 QSL clause equivalence final spec-review base disposition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact acb565cecfdee6f055c4ba36673d8d493b84311c; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Follow-up over SR-654005 FND-006 only, plus adjacent status regressions, at exact head `acb565cecfdee6f055c4ba36673d8d493b84311c`. The local branch ref and the origin ref both resolve to that head. The reviewer had read, search and write tools only: no spec gate ran, and no `git diff` was available. Ticket and PR text was treated as data.

## Verdict

PASS for this scope. FND-006 is fixed and no new finding was raised. FND-008 stays out of this pass: final-head aggregate Clippy evidence is owned by the code lead final gate.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-006 | fixed | The FR-015-AC-96 row (spec/kani/functional/FR-015-bounded-kani-obligations.md:898) no longer says native equivalence is pending QSL-1302. It now credits a CG #654 control for receiver-only precondition collection bodies only. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

**Credit is limited to the receiver-only slice.** The row says the control compares the generated completed Boolean with native QSL pre-call evaluation over the same admitted source and valuation, and "supplies equivalence evidence for that slice without Kani proof credit". This matches the test: the native call has a `self_object` and empty `parameters` (tests/it/kani_obligations_symbolic_domains.rs:1944-1948), and it uses `ClauseObservation::PreCall`.

**Other scope stays PARTIAL.** The row still reads `Evidence status: PARTIAL`, names IR-1091 for pre/post operation context and IR-1157 for constrained-method semantics, and keeps preconditions with operation parameters and postcondition bodies refused. Constraint clauses stay typed unsupported under FR-015-AC-128, and the invariant-clause Kani control still needs passing reviewed-head evidence before credit.

**Adjacent status.** `QSL-1302` no longer appears anywhere under spec/ or plan/. The FR-015-AC-99 row (:901) still agrees: it names the same test and CG #654, and keeps the proof-evidence and operation-context rows partial. TC-025 (spec/kani/matrix/TC-025-bounded-kani-obligations.md:210) keeps AC-96 PARTIAL and overclaims nothing, though it does not name the CG #654 control. FR-015 holds that status, so this is not a finding.

**Normative text.** The change sits in the status clause after `Evidence status:`. No file-level diff was available, so this rests on reading the head row against the SR-654005 excerpt.

**Log evidence.** `cg-ir1091-qsl-equivalence-disposition-spec.log` and `cg-ir1091-qsl-equivalence-fnd006-spec.log` show the `quire validate --scope .` command, loader notices and the same four grammar warnings at FR-015:930, FR-017:316 (two) and FR-014:310, with no error text. None sits on the FR-015-AC-96 row. Neither log records a SHA or exit status.

## Examined scope and bindings

```yaml
head: acb565cecfdee6f055c4ba36673d8d493b84311c
clean: true
scope:
  - {id: FR-015-AC-96, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "this control supplies equivalence evidence for that slice without Kani proof credit"}
  - {id: FR-015-AC-99, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: context_only, excerpt: "while the precondition proof-evidence and operation-context rows remain partial"}
  - {id: TC-025, path: spec/kani/matrix/TC-025-bounded-kani-obligations.md, role: context_only, excerpt: "AC-96 remains PARTIAL"}
bindings: []
```
