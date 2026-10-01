---
id: "SR-655"
title: "CG PR 212 spec review: FR-018 convert read-through clause, FR-018-AC-15 and Test Matrix edits"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@58dd43d829f4ee742c87775455e73dc98adde05e; spec/functional/complete-v1/FR-018-composite-equality-oracles.md, spec/test-matrix.md"
---

# SR-655: CG PR 212 spec review

## Summary

Ticket: IR-480. PR: agent-ix/quire-contract-codegen#212 at 58dd43d. The spec changes are:

- a new FR-018 Behavior clause (convert read-through)
- FR-018-AC-15 and its counterexample row
- the FR-014-AC-2 matrix split
- the FR-018 matrix row and the TC-029 row extended through AC-15
- the notes prose for AC-7 and the recursive type

Sub-analyses applied: EARS and integrity on the new clause and AC, and matrix consistency.
`quire validate` passes as part of `make spec` in `make ci`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new clause says an `application` of operator `convert` is "how a checked package spells `convert<T>`". QSL a28a5578, the producer of checked packages, keys the conversion as its own `expression` node and puts a `reference` to it in the equality's `arguments`. An inline convert application is a shape only the hand-built corpus emits. As written, the clause asserts a producer contract that is false and specifies behavior no real package reaches | spec/functional/complete-v1/FR-018-composite-equality-oracles.md:155-158 |
| FND-002 | low | Clause and AC disagree on scope. The clause covers any `convert` application operand and reads "the type of the operand it converts". AC-15 covers only "a `convert` application over a literal". A nested convert, or a convert over a non-literal, is unspecified. The implementation reads through to the innermost literal, which differs from "the operand it converts" for a nested convert | spec/functional/complete-v1/FR-018-composite-equality-oracles.md:201 |
| FND-003 | low | The new notes prose for the recursive type says IR finds the text-leaf count "undecidable ... although QSL emits a recursion leaf". For `R_SELF`, which reaches no text, QSL emits `leaves: []`, and IR refuses on the cycle alone. The prose misstates the disagreement the open research run is deciding (see SR-653 FND-003). It also sits as one very long line inside the AC-8 paragraph | spec/test-matrix.md:94 |

## Verdict

Otherwise sound:

- The clause is EARS-shaped ("Where ..., the generator shall ...").
- AC-15 can fail: its counterexample (reading `result_type`) is a mutant that the traced test
  kills.
- The AC-15 row, the FR-018 Covered row and the TC-029 coverage list agree.
- The FR-014-AC-2 split row is honest and actionable: it states the reason, the pin and the
  unblock condition.
- The AC-7 note is accurate.

## Dispositions

Round 1, reviewed at bdbad7af90728ce194bb685dae2cb247923c0582.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed bdbad7a | The clause now says each operand is a `reference` to the node that denotes it, as QSL keys operands. That matches qsl-semantics a28a5578 `typed_application`/`value_node`/`parameter` |
| FND-002 | fixed bdbad7a | The clause and AC-15 now share one scope: reference operands, nested conversions read to the first non-conversion node, and a non-convert application read as its own `semantic_type`. A non-reference operand disagrees. Each branch has a traced test, and the counterexample row names the read-through mutants |
| FND-003 | fixed bdbad7a | The misleading notes prose is removed, and the accurate statement is on the FR-018-AC-2 row |
