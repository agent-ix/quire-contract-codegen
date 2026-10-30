---
id: SR-654005
title: "IR-1091 CG654 QSL clause equivalence spec-review base disposition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1091; PR654; exact dbfcc961e66bb26d5d5c2cae47659feeb41594b0 on base d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae; spec/kani/functional/FR-015-bounded-kani-obligations.md, Cargo.lock"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-015", type: references }
---

## Summary

Ticket: IR-1091 (PARTIAL). Spec-review base pass over the mixed code and spec-pin PR at exact head `dbfcc961e66bb26d5d5c2cae47659feeb41594b0`, merged base `d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae`. The spec change is status prose only, so the EARS, failure-domain and dependency lenses do not apply. SR-654001 ran no base review, so this pass has no prior base finding to dispose; it records the QSL pin and the status consistency of FR-015.

## Verdict

CONDITIONAL. The QSL pin family is clean and the AC-99 row is correct. One new low finding: the FR-015-AC-96 row still says the native equivalence is pending QSL-1302, which this PR makes false.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | The FR-015-AC-96 status still says independent native equivalence remains pending QSL-1302, while the AC-99 row and an AC-96-tagged test now supply it. | spec/kani/functional/FR-015-bounded-kani-obligations.md:898 |

FND-006 detail: SR-654001 FND-003 named only row 901, and the fix corrected only that row. The sentence at row 898 is the same claim, and it is now wrong in a row the test is tagged to. The fix replaces `but independent native equivalence remains pending QSL-1302` with a reference to the CG #654 control, keeping AC-96 PARTIAL. This is status drift only: no normative text changes and no proof credit is overclaimed.

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-003 | fixed | Owned by SR-654004. Confirmed here: row 901 no longer defers to QSL-1302. |
| FND-006 | still open | Raised in this pass. Row 898 at exact head still carries the sentence. |

## Coverage

**QSL pin.** `Cargo.lock` at head has seven `qsl-*` entries (qsl-cst, qsl-eval, qsl-forms, qsl-foundation, qsl-package, qsl-replay, qsl-semantics), and every `quire-spec-language` source is `93fcdc171125a70d3fee4943bbb0110424900dcf`, so one copy holds. Commit b7d0e847 is the pin commit by message. The pinned checkout provides `evaluate_clause`, `ClauseObservation::PreCall` and the `ClauseRefusal::Coded` shape the test uses.

**Normative text.** The AC-99 statement keeps the facade requirement, and the new prose sits in the status clause after `PARTIAL`. Row 901 still reports the invariant, postcondition, graph and final Kani evidence as partial or planned.

**Log evidence.** `cg-ir1091-qsl-equivalence-review-fix-spec.log` and `review-artifact-spec.log` show `quire validate` output with four grammar warnings at FR-015:930, FR-017:316 (two) and FR-014:310, and no error text. Neither records a SHA or exit status. No file-level diff was available to this reviewer, so the claim that spec edits are status-only rests on reading the head rows against the SR-654001 excerpts.

## Examined scope and bindings

```yaml
head: dbfcc961e66bb26d5d5c2cae47659feeb41594b0
base: d91bf1bd75636d8ed98ac8bc9ceee401b26e94ae
clean: false
scope:
  - {id: Cargo.lock, path: Cargo.lock, role: examined, excerpt: "seven QSL source entries at 93fcdc171125a70d3fee4943bbb0110424900dcf"}
  - {id: FR-015-AC-96, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "independent native equivalence remains pending QSL-1302"}
  - {id: FR-015-AC-99, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "the precondition proof-evidence and operation-context rows remain partial"}
bindings: []
```
