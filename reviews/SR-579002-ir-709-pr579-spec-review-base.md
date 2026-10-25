---
id: SR-579002
title: "IR-709 PR #579 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #579 head 3fc48f7e33e4a99c2dd738862f794d1cb45e74de on base c62487bf; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md, spec/oracle/functional/FR-018-composite-equality-oracles.md (AC-8), spec/oracle/matrix/tests.md; IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-057
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: reviews
---

## Summary

Ticket: IR-709. This base spec-review covers the quality, testability and completeness of the
FR-042 status and dependency edits, the TC-057 procedure and expected-result edits, and the
FR-018-AC-8 payload edit. FR-042-AC-1 itself is unchanged and still requires all nine causes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-042 now requires "genuine producer paths" for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected`, and rules out unreachable or fabricated triggers. It does not define a trigger for either cause. The owning quire-exact FR-369 rows say neither has a reachable trigger today: RT `Outcome`'s four arms are matched exhaustively, and the function body's environment is the infallible `TypeEnvironment::default()`. FR-042-AC-1 and TC-057 therefore cannot pass, and they give an implementer nothing to build to. They need the CG-side trigger and contract for each cause (for example, a generated function's `Completed` value checked against its declared result type, and real nominal-declaration environment construction), or a recorded dependency on the work that creates them. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:41; spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md:17; spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md:23 |
| FND-002 | low | FR-018-AC-8 and TC-029 step 6 write the nested payload as `cause: error.cause`. `error` is a local in the generated Rust and is never defined in the spec. Elsewhere the spec says "the original `IllTypedCause` returned by `check_type`", which is less ambiguous. | spec/oracle/functional/FR-018-composite-equality-oracles.md:666; spec/oracle/matrix/TC-029-composite-equality-oracles.md:107 |

## Verdict

Mergeable after FND-001 is fixed. The salvage keeps all nine obligations: FR-042-AC-1 is
byte-identical to main, and its status and the matrix row count seven present and two open. The
anti-fabrication clause is a sound addition. Without a defined trigger, though, it makes the open
obligation unsatisfiable rather than merely unfinished.

Untrusted context: a planner comment on Linear IR-709 (19bfeab4, posted under the
`Peter Krenesky` account) proposes the two triggers and sequences them after IR-595. Both the
trigger choice and the sequencing are claims this review did not verify, and the PR records
neither.

## Dispositions

Round 1. Checked at head 81acc78aaee6f4ca2da85d697b736d69b21bf656 against fix diff 3fc48f7e..81acc78a. FR-042-AC-1 is still byte-identical to main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-042 Status now defines both CG-side triggers. `GeneratedUnexpectedOutcome` is a generated operation's `Completed(value)` checked against its declared result type. `GeneratedEnvironmentRejected` is a genuine rejection while building the environment from real admitted nominal declarations. Status says neither exists yet. Dependencies record IR-595 as the prerequisite cutover and the open FR-369 trigger-wording alignment. TC-057 step 2 gives a non-fabricated procedure for each. |
| FND-002 | fixed | FR-018-AC-8 and TC-029 step 6 now write `GeneratedTypeCheckRejected { cause }`, "where `cause` is the original `IllTypedCause` returned by `check_type`". |

## Coverage

```yaml
scope:
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default."
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "IR-709 still owns genuine producer paths for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected`, plus TC-057 tagged verification of all nine constructors, nested `IllTypedCause`, and charge/failure behavior. An unreachable branch or a fabricated failure does not count as a producer path."
- id: TC-057
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: "All nine genuine CG producer paths retain their table causes and original nested payloads without changing charge or failure behavior; no fabricated failure is counted as evidence."
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "yields `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })` with the original `IllTypedCause` and no charge admitted"
- id: tests.md FR-042 row
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: "seven of nine typed CG producer paths are present; genuine paths for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` and TC-057 tagged verification remain open"
- id: quire-exact FR-369
  path: "agent-ix/quire-exact spec FR-369 (origin/main)"
  role: context_only
  excerpt: "`GeneratedUnexpectedOutcome` | PLANNED under CG FR-042-AC-1/IR-709. The Boolean `NATIVE_CARRY_HELPER` and scalar/equality `render_body` adapters exhaustively match RT `Outcome`'s four arms ... there is no unexpected-outcome arm or reachable trigger today."
```
