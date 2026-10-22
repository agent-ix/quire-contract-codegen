---
id: FR-044
title: "Export generated-closure cases for QSL evaluator comparison"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-402
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/FR-262
    type: depends_on
---
# FR-044: Export generated-closure cases for QSL evaluator comparison

## Description

The code generator (CG) SHALL publish its generated-closure applications as
cases of the QSL FR-402
evaluator differential corpus (IR-591). FR-402 owns the case record, loader,
admission and compared-record encoding in `quire-eval-corpus`, a crate in
`agent-ix/quire-eval`. CG depends on that crate to write cases and on
`quire-eval` to record expected evaluations once that implementation exists.
CG does not define a second case or evaluator wire format. QSL's harness owns
the std/no_std comparison and its CI placement.

## Inputs

- An independently enumerated live set of CG-owned scalar, equality and
  nested generated-to-generated-to-interpreted source applications admitted
  by QSL before generation. Each application carries the unit's source text,
  display path and all four `SourceIdentity` string labels (`authority`,
  `identity`, `revision_namespace`, `revision`), its ordered dependency units,
  selected callable name, completed FR-071 `WitnessValue` arguments, execution
  limits and the names of generated bodies registered for the run.
- The `package_id` from `qsl_replay::compile_package` of that source and its
  dependencies, called inside CG's existing replay module. Source authority,
  identity and byte digest bind that package id; revision labels do not.
- When `quire-eval` has the implementation that runs registered generated
  bodies, one result and meter from that implementation for each selected
  application. Until then, no recorded expected result is available.

## Outputs

The explicit `make evaluator-corpus` producer SHALL write `cases.jsonl` in
UTF-8 JSON Lines under `EVALUATOR_CORPUS_DIR` (default
`target/evaluator-corpus`). It SHALL use `quire-eval-corpus::CaseRecord::line`
for each `quire.evaluator-case/v1` line, in strictly ascending UTF-8 `case_id`
order, with one final newline per line and no other bytes. The target is a
complete generated bundle, never a checked-in copy. The producer SHALL use
`quire-eval-corpus::load` to check the finished bundle; CG does not implement
its own case codec or loader refusal order.

Each record SHALL carry exactly FR-402's `format`, `case_id`, `source`,
`dependencies`, `package_id`, `function`, `arguments`, `object_environment`,
`limits`, `generated_bodies` and `expected` members. `source` and each
dependency's source retain the four labels, path and text. Dependency entries
are strictly ordered by identity; `generated_bodies` is a sorted unique list
of **function names**, not registration identities, and is empty when
`expected` is `null`. `object_environment` is
`null`: FR-402 version 1 admits no domain package or object environment.
Limits use FR-402's named counters and decimal-string encoding. Until
`quire-eval` implements registered-body execution, `expected` is `null` and
`generated_bodies` is empty. Once it can run this case, a non-null
`expected` is one `quire-eval-corpus::ComparedRecord` from that run, with
the registered generated-body function names in `generated_bodies`. A
completed value outside FR-402 version 1's Boolean and Integer result set
refuses `unencodable_value`, rather than becoming a CG-defined value encoding.

## Behavior

The producer SHALL emit only source applications admitted by QSL before
generation. It SHALL retain their callable, source, argument and limit
choices, and SHALL retain registered-body names in the record only when a
non-null expectation was recorded; it SHALL not serialize a private checked-closure
or registration-identity object in place of FR-402's source case. The
`package_id` SHALL come from QSL compilation in CG's replay module, and the
bundle SHALL load and admit through FR-402's QSL case admission. A mutation of
an identity label that binds `package_id` without recompilation refuses as
`package_mismatch`; a revision-label change alone does not change
`package_id`.

The loaded case-id set SHALL equal an independent census of the live admitted
source applications, so removing a record fails the census. Repeating the
export with the same cases and interface version SHALL produce byte-identical
output. The producer SHALL replace the bundle atomically only after all cases
are constructed and encoded and the completed bundle loads. A producer-side
compile, case-construction, case-encoding or load failure SHALL leave a previous complete bundle
byte-identical, or leave the target absent if none existed. Once a non-null
expectation is available, its result, refusal, location, losses and consumed
counters SHALL come from one `quire-eval` run with those `generated_bodies`;
its ordered `charges` are present only when FR-402's `charge-log` test support
is enabled and the log is not truncated, otherwise `charges` is `null`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-044-AC-1 | Given an admitted scalar, an equality and a nested generated-to-generated-to-interpreted source application, `make evaluator-corpus` writes one stable unique case id per application in `cases.jsonl` at `EVALUATOR_CORPUS_DIR`. Every line is a `quire-eval-corpus` `quire.evaluator-case/v1` record with FR-402's exact members: four source-identity labels, display path and text, sorted dependency units, QSL-compiled `package_id`, callable function name, FR-071 arguments, decimal-string limits, `object_environment: null`, `generated_bodies: []` and `expected: null` while `quire-eval` has no registered-body implementation. FR-402's loader and QSL admission admit all three as the same source applications; a changed authority, identity or valid source text with the old `package_id` refuses `package_mismatch`, while changed revision labels alone do not. | Test |
| FR-044-AC-2 | Repeating export of the same cases is byte-identical. Loaded case ids equal an independent enumeration of the live QSL-admitted source applications; deleting a record fails that census. Perturbing a callable, argument, dependency or limit changes the loaded case or causes FR-402's stated load/admission refusal. A producer-side compile, case-construction, case-encoding or load failure leaves the previous complete bundle byte-identical, or the target absent when none existed. | Test |
| FR-044-AC-3 | Inspection finds CG uses `quire-eval-corpus` for case writing and loading, obtains `package_id` via `qsl_replay::compile_package` inside CG's replay module, defines no local case/compared-record codec or loader refusal, and computes no expectation from emitted source or Contract Runtime `exact`. CG uses no `qsl-eval` dependency or direct QSL evaluator call outside the existing replay facade. | Inspection |
| FR-044-AC-4 | Once `quire-eval` implements registered generated-body execution, each selected case records a non-null `ComparedRecord` from one run with its sorted unique generated-body function names and fresh meter; FR-402's expected/std check passes. Perturbing a generated-body name changes the loaded case or causes FR-402's stated refusal. Boolean and Integer completed values encode; a completed value of another kind refuses `unencodable_value`. The compared record includes the exact result arm, refusal, location, losses and all ten consumed counters, with `charges` present only under FR-402's non-truncated `charge-log` test support and `null` otherwise. | Test |

## Dependencies

- QSL FR-402 owns `quire-eval-corpus`, its `CaseRecord`, loader and compared
  record. FR-401 owns the `quire-eval` interface; its registered-body
  implementation is required for AC-4's non-null expectation.
- QSL FR-262 supplies the selected function applications. CG's existing
  `qsl_replay::compile_package` caller is inside its replay module; this
  requirement does not request an FR-060 T12-A amendment.
- IR-590 supplies the second evaluator for QSL's differential run. IR-591
  owns CG's producer and the cross-repository bundle check; QSL FR-402-AC-10
  owns the QSL harness run over CG's bundle.

## Status

PLANNED (IR-591, IR-976). CG's current corpus is held in test builders; no
`quire.evaluator-case/v1` export or non-null expected result is claimed.
