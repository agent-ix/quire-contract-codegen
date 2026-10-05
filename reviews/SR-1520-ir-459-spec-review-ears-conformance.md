---
id: SR-1520
title: "ears-conformance review of quire-contract-codegen#280 (new FR-024 statements)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@70f5e08ac399d0880b50c1610592220c15298e09; spec/replay/functional/FR-024-counterexample-envelope-intake.md (the 14 statements added under Requirements, diff only)"
review_set: subset
---

## Summary

Ticket: IR-459. I examined the 14 new statements in FR-024's requirement list. They cover:

- minting the identity;
- `subject`/`occurrence`/`anchor` from `OperationSite`;
- `kind` and `arguments`;
- no caller identity;
- `NotAFrame`;
- the one decoder;
- the one renderer;
- the decode refusal;
- `OutOfDomain`;
- `PreState`;
- `ScopeMismatch`;
- empty `declared_domains`.

Each is ubiquitous ("The generator shall ...") or unwanted-behaviour ("If ..., then the
generator shall ..."), with one named subject. `quire validate` adds no EARS warning: the six
warnings at the head are main's same six, shifted by line. The "shall return X and shall not
call Y" compound follows the file's existing convention for refusal statements.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-024-AC-22 refuses, with `NotAFrame`, "a frame whose granted and checked fields are not exactly its state fields", and FR-024-AC-23 requires the `state_fields` record member. Neither behaviour has an EARS statement: the only `NotAFrame` statement covers "property is not `frame`", and no statement requires the record to carry `state_fields`. The ACs exceed their statements. The refusal name also misreads, because a malformed frame record is still a frame. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:288-289 |

## Dispositions

Round 1, reviewed at b094aaca23d924831ee362d8865d31b74acbb753.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b094aac |
