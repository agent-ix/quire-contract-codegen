---
id: "SR-873"
title: "CG PR 237 spec review (criterion strength): FR-021-AC-19 to AC-21"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@6d0e02a4770c6b77bc4b08ea46a3978015c6c1bc; FR-021-AC-19, FR-021-AC-20, FR-021-AC-21 and their mutation rows in spec/oracle/functional/FR-021-function-application-oracles.md; emitted templates in src/oracle/function/mod.rs:1313-1380 and tests/it/exact_function_generation.rs (main corpus, Negate test) read as context"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

# SR-873: CG PR 237 spec review, criterion strength

## Summary

Ticket: IR-352. Judged by hand against the emitted templates (Jev was not used). For each AC: can
a test fail it, and does its text match what correct code emits.

- AC-19: can fail. Today's emitted source fails it (two `unreachable!` per scalar and equality
  body). The main FR-021 corpus has one Scalar, one CompositeEquality and one Call function, so a
  count over it is not vacuous. "In any delimiter form" matches the existing scanner.
- AC-20: the presence of a catch-all is forced by the compiler (a foreign `#[non_exhaustive]` enum),
  so that half is always true; the content half (returns `Refused(CheckedInvariant)`) can fail, and
  the mutation row (Completed fallthrough or another refusal) is the right adverse case. Because the
  unknown variant cannot be built from a test crate, text inspection is the only possible evidence;
  TC-031 step 8 says so.
- AC-21: the refusal half is already implemented and tested
  (`tc_031_unsupported_operator_refuses_unary_negate_scalar_body`); the source half can fail today.

## Verdict

All three criteria can fail, but two are worded so a literal test would either fail correct code or
be left to interpretation. Fix AC-20 and AC-21 wording before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-20 does not describe the real emitted match. The match is over `Result<Outcome<_>, Refusal>` (`match evaluated` / `match frame.meter(..)`), with an `Err(refusal)` arm the AC omits, and the `Completed` arm does not "return its payload unchanged": it rewraps it as `rt::Value::Integer(value)` or `rt::Value::Boolean(value)`. A test written to the text fails correct code or must be bent. Say: the `Ok(Completed)` arm wraps the value in the body's result `Value`, `Ok(Undefined | Refused | Incomplete)` and `Err` pass their payload through, and the final `Ok(_)` arm returns `Outcome::Refused(Refusal::CheckedInvariant)`. | spec/oracle/functional/FR-021-function-application-oracles.md:238; src/oracle/function/mod.rs:1327-1334, :1359-1366 |
| FND-002 | medium | AC-21's "the typed generation refusal" and "no function source" are undefined in FR-021. The code's refusal is `ExactFunctionRefusal::UnsupportedOperator`, reported on each item that names the function, and the refused function is absent from `checked_package()`'s declarations. Name both, so a test cannot pass on a different refusal (for example `BodyMismatch`). The AC also joins a behaviour claim and a source-text claim; consider splitting. | spec/oracle/functional/FR-021-function-application-oracles.md:239 |
| FND-003 | low | AC-19 says "every function oracle in the corpus" without naming the corpus. If the test used a corpus with only `Call` bodies, the count would pass vacuously. Name the main FR-021 corpus (TC-031 step 1(a): scalar, composite-equality and nested-call bodies). | spec/oracle/functional/FR-021-function-application-oracles.md:237 |

## Dispositions

Round 1, reviewed at cac5002cc137ad6297def98b225b9394b0673fef. Checked against the emitted templates at src/oracle/function/mod.rs:1327-1334 and :1359-1366, and tests/it/exact_function_generation.rs:30-36, :566-581.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (AC-20 now describes the match over Result<Outcome<_>, Refusal>: Ok(Completed) rewrapped as Value::Integer or Value::Boolean, Ok(Undefined), Ok(Refused), Ok(Incomplete), Err(refusal) as Outcome::Refused(refusal), final Ok(_) as Refused(CheckedInvariant); matches the real templates) |
| FND-002 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (AC-21 names ExactFunctionRefusal::UnsupportedOperator and absence from the emitted checked_package()) |
| FND-003 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (AC-19 names main_oracles(), which generates the scalar add_fn, equality eq_fn and nested-call call_fn, plus chain_oracles(); both exist at tests/it/exact_function_generation.rs:566 and :576) |
