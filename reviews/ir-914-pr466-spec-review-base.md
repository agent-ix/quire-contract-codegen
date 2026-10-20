---
id: SR-13451
title: "Base spec review of Boolean implication mutation"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: reviews
---

## Summary

Ticket: IR-914. The new criterion and TC step intend a healthy proof and a mutant falsification, but the criterion table cell is broken.

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
| FND-001 | high | FR-015-AC-118 contains unescaped `\|\|` inside a Markdown table cell. Quire truncates the criterion at `!left` and loses its Test verification cell, so the native expectation, installed-Kani outcomes, mutation and clause boundary are not parsed as criterion text. | spec/kani/functional/FR-015-bounded-kani-obligations.md:823 |
