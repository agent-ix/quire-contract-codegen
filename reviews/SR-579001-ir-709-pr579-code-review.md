---
id: SR-579001
title: "IR-709 PR #579 code-review (spec claims against generated source)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #579 head 3fc48f7e33e4a99c2dd738862f794d1cb45e74de on base c62487bf; spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md, spec/oracle/matrix/tests.md; IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-029
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-057
    type: reviews
---

## Summary

Ticket: IR-709. Independent Opus review of draft PR #579 at frozen head 3fc48f7e, scoped to
`git diff origin/main...HEAD` (one commit, five spec files, no Rust). This lane checks every
cause or producer claim the diff makes against the generated-source templates on this head.

The seven-present count is accurate. A grep of `src/` for `CheckedInvariantCause::` finds exactly
seven distinct causes with constructor sites:

- `GeneratedIntervalInvalid` in `boolean_v1.rs:913`.
- `GeneratedDescriptorReconstructionFailed`, `GeneratedTypeCheckRejected` (x2) and
  `GeneratedEqualityCheckRejected` in `equality/mod.rs:1963`.
- `GeneratedBodyPlaceholderInvoked` in `function/mod.rs:2911`.
- `GeneratedArgumentShapeMismatch` (x2), `GeneratedOperandKindUnsupported` and
  `GeneratedEqualityCheckRejected` in `function/mod.rs:3459-3487`.

`GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` have no site. No changed text
claims they landed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-018-AC-17 and the TC-029 step that mirrors it now name `GeneratedDescriptorReconstructionFailed` as the oracle-side `Err` arm of a `rebuild_*` call. The generated oracle makes no `rebuild_*` call. Every reconstruction helper is called only from `environment_{symbol}` and its descriptor helpers, through `?` into `EnvironmentError`. The emitted `oracle_{symbol}` uses this cause only for a binding, package-identity or context-requirement mismatch on the prepared wrapper. The AC therefore attaches the cause to a path that does not exist. A test written to it would have to fabricate one, which TC-057 step 1 now forbids. | spec/oracle/functional/FR-018-composite-equality-oracles.md:675; spec/oracle/matrix/TC-029-composite-equality-oracles.md:178-180; src/oracle/equality/mod.rs:1961; src/oracle/equality/mod.rs:1963; src/oracle/equality/mod.rs:2111-2280 |

## Verdict

Mergeable after one medium fix. The status prose in FR-042, TC-057 and `tests.md` is accurate
and nine-preserving: seven producers present, two named open, and tagged verification open. It
does not claim the two missing causes landed. The AC-8 and TC-029 step 6 payload,
`GeneratedTypeCheckRejected { cause: error.cause }`, matches the emitted `check_type` arms. The
FR-018 Behavior payload text at line 430 matches both oracle arms. FND-001 is the only claim that
disagrees with source.

## Dispositions

Round 1. Checked at head 81acc78aaee6f4ca2da85d697b736d69b21bf656 against fix diff 3fc48f7e..81acc78a, with merge-base main c62487bf. `make spec` passes at that head.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-018-AC-17 and TC-029 step 7 now say that a `rebuild_*` Err arm yields `EnvironmentError::Reconstruction` "without calling the oracle". `GeneratedDescriptorReconstructionFailed` moves to a separate clause covering a wrong prepared binding, package identity, context requirement or admitted-context identity, which matches `oracle_{symbol}` at src/oracle/equality/mod.rs:1963. |

## Coverage

Not run, per brief: Rust gates (`make test`, `make lint`, `make ci`). `make spec` passed at the
head. Its only warnings are in FR-017 and FR-014, which this PR does not touch.

```yaml
scope:
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "every call of one is followed by `?` or a `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedDescriptorReconstructionFailed })`"
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "yields `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })` with the original `IllTypedCause` and no charge admitted"
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source. IR-709 still owns genuine producer paths for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected`"
- id: TC-057
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: "Inspect the emitted source for each of the nine named paths; do not count an unreachable branch or a fabricated failure as evidence."
- id: tests.md FR-042 row
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: "seven of nine typed CG producer paths are present; genuine paths for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` and TC-057 tagged verification remain open"
- id: src/oracle/equality/mod.rs
  path: src/oracle/equality/mod.rs
  role: context_only
  excerpt: "return exact::Outcome::Refused(exact::Refusal::CheckedInvariant {{ cause: exact::CheckedInvariantCause::GeneratedDescriptorReconstructionFailed }});"
```
