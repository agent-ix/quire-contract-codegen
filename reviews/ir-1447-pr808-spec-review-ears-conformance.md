---
id: SR-17940486709564847047745
title: "IR-1447 PR #808 spec-review ears"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen PR #808 at 56f1887d; IR-1447; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md, docs/qualification/ir1447-kani-control-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1447. EARS conformance of the three new FR-015 requirement bullets
(spec/kani/functional/FR-015-bounded-kani-obligations.md:446-457) and the single-obligation
shape of FR-015-AC-152..163.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | The second bullet ("When the installed-Kani control runs ..., the test shall show ...") makes the test the subject. That is a verification statement placed in the requirements section, not system behaviour, and it bundles two obligations (healthy `Verified` and mutant `Falsified`). Restate it as system behaviour: "the generated harness shall verify ... and shall falsify ..." in two sentences, or move it to TC-025. | spec/kani/functional/FR-015-bounded-kani-obligations.md:450-453 |
| FND-002 | low | The first bullet is compound ("shall lower it ... and keep the independent assertion"). Split it, or make the assertion part of the lowered form. The third bullet's "refuse ... and emit no harness" follows the existing FR-015 pattern and is acceptable. | spec/kani/functional/FR-015-bounded-kani-obligations.md:446-449 |
| FND-003 | low | The second bullet says "a mutant of the construct", but FR-015-AC-156 to FR-015-AC-159 all mutate the subject, and the let mutant subject `amount + 2 <= 1000` contains no `let` at all. Make the bullet and the ACs name the same mutant site (see base FND-001 to FND-003). | spec/kani/functional/FR-015-bounded-kani-obligations.md:451-452, 984-987 |

## Verdict

EARS shape is acceptable apart from these wording issues. All are low.

## Dispositions

Round 1, reviewed head 92686c1e8bc308f2a2b825d4b2d4a096f604aa10.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The verification bullet became two system-behaviour statements: "When the installed backend runs the healthy harness ..., the harness shall be `Verified` with its cover satisfied" and "When the installed backend runs the harness ... with the mutant at the site its criterion names, the harness shall be `Falsified` at the step and with the input that criterion names". |
| FND-002 | fixed | Lowering and the independent `i128` assertion are now separate bullets ("the generator shall lower it ..."; "the generated harness shall assert the clause result against an independent `i128` evaluation ..."). Laziness and precedence each have their own bullet. |
| FND-003 | fixed | The bullet now says "the mutant at the site its criterion names", and each AC names its site: subject mutants for AC-156/157/158/165/170, the generated oracle for AC-159. AC-156 says the mutant subject "computes the bound value as `amount + 2`". |
