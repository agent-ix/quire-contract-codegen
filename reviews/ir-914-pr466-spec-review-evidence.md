---
id: SR-13454
title: "Evidence review of Boolean implication mutation"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: reviews
---

## Summary

Ticket: IR-914. TC-025 step 48 names installed Kani, healthy cover and a concrete mutant playback; the advisor cannot attach its Test method because the criterion row is malformed.

## Verdict

FAIL.

## Examined

- FR-015-AC-118 (examined): Test (TC-025) |
- TC-025-step-48 (examined):     claim over two symbolic Boolean operands. Inspect the embedded oracle bytes
    against FR-014's function and the independent native `!left || right`
    assertion. Run the healthy harness with installed Kani and require
    `Verified` with its final cover satisfied. In a second run, change only the
    compiled `implies_short_circuit` subject path to return `false` for every
    input, confirming that the generated claim calls that path; leave the
    embedded oracle and native assertion unchanged. Require `Falsified` with a
    concrete playback in the Boolean domain
- FR-015-AC-118-matrix (examined): | FR-015 | FR-015-AC-118 | TC-025 | 🚧 Planned (IR-914): V2 Boolean implication renderer and installed-Kani healthy/mutated control, after FR-014-AC-35; V2 clause implication stays planned under FR-015-AC-40 |
- FR-014-AC-35 (context_only): Boolean connective nodes, including implies, derive an ir_confirmed oracle that calls evaluate_boolean or evaluate_boolean_short_circuit and returns Outcome<bool>.
- FR-015-AC-20 (context_only): A Boolean-connective scalar harness embeds its FR-014 oracle byte-identical to the oracle crate function.
- FR-015-AC-40 (context_only): The V2 clause Boolean harness remains planned and depends on the inline oracle FR-014-AC-38.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The unescaped `\|\|` splits FR-015-AC-118 so `quoin advise --json` reports `authored: null` despite the visible `Test (TC-025)` cell. The declared Test method is not attached to the parsed obligation until both pipe characters are escaped. | spec/kani/functional/FR-015-bounded-kani-obligations.md:823 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | With escaped pipes, quoin advise now reports authored Test for FR-015-AC-118 and no method mismatch. |
