---
id: SR-576001
title: "IR-709 PR #576 spec-review base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #576 head 5188f6f45447ce2d33024e1c55c3813ddb692daf on base b60b18ef; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md (context: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md, spec/oracle/matrix/tests.md, quire-exact FR-369 on main and PR #84 head 2266b435); IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: reviews
---

## Summary

Base checklist review of draft PR #576 at frozen head 5188f6f4, scoped to its 3-file diff against main b60b18ef. Ticket: IR-709. Spec-only: no code, Kani or test runs. `quire validate` over the three changed files and TC-057 passes (module warnings only).

The audit claim was measured independently. A `git grep` for `GeneratedUnexpectedOutcome|GeneratedEnvironmentRejected` outside `spec/` returns nothing at CG main b60b18ef, the audit base 1944268f, #514 at af1b6ccb and #514's current head 8c7b3598. The seven named producers (BodyPlaceholderInvoked, ArgumentShapeMismatch, OperandKindUnsupported, IntervalInvalid, TypeCheckRejected, EqualityCheckRejected, DescriptorReconstructionFailed) are all in `src/oracle/{function,equality}/mod.rs` and `src/oracle/boolean_v1.rs`. The historic sites of the two missing causes still exist at 1b225f4c, the revision FR-369 cites. There, `_ =>` fallback arms after the four `Outcome` arms appear in `carry` and in `render_body`, and `TypeEnvironment::new(..)` is matched with `Err(_)`. On current main those arms are gone. `quire_exact::Outcome` is a four-variant enum without `#[non_exhaustive]`, so the matches are exhaustive. Boolean equality uses `planned_equality`, and the function equality body uses the infallible `sv::TypeEnvironment::default()`. The fallible equality-environment admission (`TypeEnvironment::new` in the emitted environment constructor) returns `EnvironmentError::Declaration`, which FR-018 requires as its own typed variant, not a checked-invariant cause. The audit claim is true: neither cause has a genuine producer path, and constructing one would fabricate a failure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Companion PRs contradict each other. FR-042 Dependencies now says quire-exact FR-369's "table correction is tracked with IR-709 at that owner", and Status says the two causes "are not claimed as current producers". But the companion quire-exact #84 (head 2266b435) marks both rows "PLANNED under CG FR-042-AC-1/IR-709" and states "CG FR-042-AC-1 still requires all nine assignments under IR-709". #84 does not make the correction #576 cites. Whichever PR merges, CG and quire-exact disagree on whether CG owes nine producers or seven. Align both to the owner's ruling before either merges. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:35,41 |
| FND-002 | medium | FR-042-AC-1 never names the seven causes. "The exact seven CG-owned causes with genuine producer paths in the table" defines the set by current reachability, and the FR-369 table it points to still lists nine. The AC therefore cannot say which two are excluded, and a removed producer could be read as one fewer "genuine path" instead of as a failure. List the seven variant names in the AC, or in Behavior with the AC referring to that list. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:25,31 |
| FND-003 | medium | TC-057 was not updated. Step 1 still says "exercise every ... constructor listed in the second cause table of quire-exact FR-369", which has nine rows, and Expected Results says "every CG constructor retains the table cause". Read literally, the procedure requires controls for GeneratedUnexpectedOutcome and GeneratedEnvironmentRejected. That conflicts with the new FR-042-AC-1 sentence "no cause is counted from an unreachable branch or a fabricated failure". Point TC-057 at FR-042's seven named causes. | spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md:15-21 |
| FND-004 | low | The FR-018 Behavior bullet on reconstruction failure and FR-018-AC-17 (with TC-029 step) now say the oracle returns `Refusal::CheckedInvariant { cause }` without naming the cause. Reconstruction runs in the emitted environment constructor, which returns `EnvironmentError::Reconstruction`, so it is unclear which typed cause, if any, the oracle-side alternative carries. FR-042 forbids an invented default. Name the cause (for example GeneratedDescriptorReconstructionFailed), or drop the oracle-side alternative if it has no path. | spec/oracle/functional/FR-018-composite-equality-oracles.md:604-606,675; spec/oracle/matrix/TC-029-composite-equality-oracles.md:180 |

## Verdict

**Not mergeable at 5188f6f4** (FND-001 high; FND-002/003 medium). It is also independently HELD for Peter's direct ruling on the count. The reviewer does not decide that ruling.

Checked clean:
- The audit claim is true, and the reduction is source-backed (see Summary).
- The only obligation reduced is the count, nine to seven, which drops exactly the two causes with no path. No other AC text is narrowed. FR-042-AC-1 keeps nested `IllTypedCause`, downstream preservation or explicit match, charge/failure behaviour, and the bans on the unit constructor and invented defaults. It adds an anti-fabrication sentence and requires TC-057 controls for every one of the seven paths.
- FR-018-AC-8 is byte-identical to main except for the payload. Its new `GeneratedTypeCheckRejected { cause: error.cause }` is consistent with FR-042 and the emitted source at equality/mod.rs:1963. It is testable by comparing with `check_type`'s own `IllTypedCause` on the same environment, and TC-029 step 3's check that those environments pass `check_equality` pins the variant. Existing tagged tests (`tests/composite_equality_support/cases/record.rs:133`) match only `{ .. }`, so they must be tightened. That is test work that is already open, not a spec defect.
- No compat layer, shim or default cause is introduced.
- Both variants stay in quire-exact's closed thirty-variant set. FR-042 does not touch FR-369.
- The PR body states the hold clearly: what is reduced (FR-042-AC-1 nine to seven), why (no genuine path), the evidence (the source scan), and the merge condition (Peter's direct authorisation).

## Coverage

```yaml
scope:
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "CG's native Boolean, equality, function and pre-check placeholder constructors use the exact seven CG-owned causes with genuine producer paths in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; ... no cause is counted from an unreachable branch or a fabricated failure."
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "The code generator SHALL use the seven CG-owned causes with genuine producer paths in the second cause table of quire-exact FR-369 at their owning constructors."
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "yields `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })` with the original `IllTypedCause` and no charge admitted"
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "every call of one is followed by `?` or a `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or `Outcome::Refused(Refusal::CheckedInvariant { cause })`"
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "confirm `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })`, retaining the original `IllTypedCause`, no panic, and no charge admitted on the `Meter`."
- id: TC-057
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: "Exercise every generated Boolean, equality, function and pre-check placeholder checked-invariant constructor listed in the second cause table of quire-exact FR-369."
- id: FR-369
  path: "quire-exact:spec/functional/FR-369-typed-checked-invariant-causes.md"
  role: context_only
  excerpt: "PLANNED under CG FR-042-AC-1/IR-709 ... CG FR-042-AC-1 still requires all nine assignments under IR-709. (quire-exact PR #84 head 2266b435)"
```
