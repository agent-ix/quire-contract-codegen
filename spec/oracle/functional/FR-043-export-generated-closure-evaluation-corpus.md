---
id: FR-043
title: "Export generated-closure cases for QSL evaluator comparison"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-262
    type: depends_on
---
# FR-043: Export generated-closure cases for QSL evaluator comparison

## Description

When the shared QSL evaluation interface admits generated bodies, the code
generator SHALL make its generated-closure corpus loadable by the QSL-owned
differential harness (IR-591). The export is test input and reference evidence,
not a second evaluator or a claim that CG currently agrees with QSL. CG SHALL
use the public checked-closure and application interface, never QSL evaluator
internals or the retiring Contract Runtime `exact` port.

## Inputs

- The CG-owned scalar, equality and nested-call corpus cases admitted as exact
  checked closures, with their selected callable identities, completed
  arguments, object environments and execution limits.
- QSL's std reference `Evaluation` for each admitted application, obtained
  through the shared interface. QSL owns the meaning of that result and its
  serialization; CG does not calculate an expected result from emitted source.

## Outputs

The explicit `make evaluator-corpus` producer SHALL write a generated bundle
under the caller-selected `EVALUATOR_CORPUS_DIR` (default
`target/evaluator-corpus`). The bundle contains `cases.jsonl` in UTF-8 JSON
Lines format, one complete object per line, and no checked-in copy in either
repository. A record has `format: "quire.generated-closure-evaluation/v1"`, a
unique stable `case_id`, the QSL-owned checked input for the selected closure
and application, the generated-body registration identity, the limits, and the
QSL std reference's complete expected evaluation and ordered charge log.
The checked input and expected evaluation use the QSL-owned public corpus
encoding; this requirement does not define a second encoding of checked
authority or of `Evaluation`. QSL's loader rejects an unknown format or a
record it cannot admit as the named checked closure.

## Behavior

The producer SHALL emit only cases that QSL admitted before generation and
preserve each case's checked closure, callable identity and source
locations through export. It SHALL fail the bundle as a whole if an expected
reference evaluation or a checked-input encoding is unavailable. It SHALL
publish the complete bundle atomically, in stable case-id order, with byte-
identical output for the same admitted corpus and interface version. The QSL
differential harness loads the bundle from the named directory and runs those
same checked applications through its std and no_std implementations; QSL owns
the comparison and CI placement. CG's existing FR-021 target-neutral schedule
tests do not discharge that comparison.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-043-AC-1 | Given an admitted scalar case, an equality case and a nested generated-to-generated-to-interpreted call case, `make evaluator-corpus` writes `cases.jsonl` at `EVALUATOR_CORPUS_DIR` with one stable, unique case id per application; every line is a complete `quire.generated-closure-evaluation/v1` record containing the QSL-owned checked input, exact registration and application identity, limits, complete QSL std expected `Evaluation` and ordered charge log. QSL's corpus loader admits all three records and selects the same checked applications. | Test (TC-058) |
| FR-043-AC-2 | Repeated export of the same admitted cases is byte-identical. Removing a case, changing a selected callable, source location, limit, result arm, refusal, loss, consumed work or charge order changes or rejects the corresponding loaded record; an unavailable QSL reference or unencodable checked input produces no published partial bundle. No exported expectation is computed from CG's emitted source or from Contract Runtime `exact`. | Test (TC-058) |

## Dependencies

- QSL FR-262 and IR-583 provide the shared checked-closure interface; IR-590
  supplies the no_std implementation and IR-591 owns differential comparison.
- QSL STD-300 must define the public checked-input and `Evaluation` corpus
  encoding and loader. IR-853 must settle CG's concrete interface adoption
  without violating FR-021's current no-owner-path boundary.

## Status

PLANNED (IR-591). The current CG corpus is held in test builders and has no
loadable checked-input/expected-evaluation export. This requirement makes no
claim that the QSL differential harness or producer already runs.
