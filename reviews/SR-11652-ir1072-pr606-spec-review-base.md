---
id: SR-11652
title: "IR-1072 PR 606 specification base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-011-numeric-harness-campaigns.md; spec/strategy/matrix/TC-020-numeric-harness-campaigns.md; spec/strategy/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed new FR-011-AC-13, TC-020 step 12, and the Planned TestMatrix mapping at frozen head ab7cb945c3c2c8a0c32b058ae8a2055709568035. The merged IR accessor takes the owner-bound key and returns MissingClaimOccurrence for a wrong owner. The existing criteria remain Planned. No applicable AssuranceProfile was found under spec/.

## Scope

- examined: FR-011-AC-13 — public V2 selected-claim entry, typed complete key, matching and wrong-owner behavior.
- examined: TC-020 step 12 and expected result — positive and wrong-owner selection evidence.
- examined: TestMatrix FR-011-AC-13 and TC-020 rows — Planned status and test mapping.
- context_only: FR-011-AC-6, FR-011-AC-7, FR-011-AC-9, FR-008-AC-4, FR-008-AC-10; all remain Planned.
- context_only: Contract IR execution_point_accessor.rs at f773a462 or later.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-020 step 12 lacks a competing authentic claim, so a test could not distinguish an exact selected point from a substituted point | spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:74 |
| FND-002 | medium | The now-live IR accessor returns `ExecutionPoint`; FR-011 names a nonexistent `CheckedExecutionPoint` return type | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:54 |

## Verdict

CONDITIONAL. Add a competing claim with a different checked point to TC-020 step 12, and name IR's actual accessor return type before implementation.

## New findings (disposition pass 1)

Reviewed fix head 0e2b18b81d7375af3970365d490b240b8f590c91 against fresh Contract IR origin/main 3cf56bc49e11bce6822ca5b53b3ee0619375a917.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | Current IR main returns unified `ExecutionPoint`, while FR-011 still names retired `CheckedExecutionPoint` and a conversion boundary that no longer exists | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:54-58,145 |
| FND-004 | high | Step 12 requires two distinct points for authentic occurrences of one clause, but the IR accessor reads one point from that clause's term | spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:74-78 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | still-open | The fix adds a competing claim, but requires two different accepted points for one checked clause. Contract IR accessor reads one point from `self.term(clause)` after checking occurrence membership, so this fixture cannot be constructed. Use distinct clauses and claims with different points to test cross-selection. |
| FND-002 | rejected | At reviewed IR main d4f75fc29bdacb51bfbcf16cc55d22836b80c1fe, `execution_point_accessor.rs:82` returned `CheckedExecutionPoint`, `typed_terms.rs:143` defined it, and `lib.rs:188` reexported it. The original finding inspected a later IR-1068 checkout and attributed its type to the earlier reviewed head. New drift from current IR main is separately FND-003. |
| FND-003 | still-open | Current IR main 3cf56bc4 has unified the accessor return type to `ExecutionPoint` with `ExecutionPointName` and removed `CheckedExecutionPoint`; FR-011 still names the retired type at line 55 and forbids conversion through a legacy model point at lines 57-58 and 145, but that model point is now the accessor return. |
| FND-004 | still-open | The new same-clause two-point fixture is impossible with the merged IR accessor. Use two distinct clauses and claims with different points for a supported competing-selection test. |

### Round 2 at 5089f74212cd9b08affeafbdaede669e5943a4c1

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | TC-020 step 12 now uses two distinct checked state clauses and claims with different points, giving a constructible competing-selection fixture. |
| FND-003 | fixed | FR-011 now names the IR accessor's unified `ExecutionPoint` and `ExecutionPointName`, preserves its nonempty name domain, and removes the legacy conversion claim at lines 54-58 and 145. |
| FND-004 | fixed | TC-020 step 12 now requires distinct clauses rather than distinct points for occurrences of one clause; IR can retain one different point per clause. |
| FND-005 | still-open | The new swapped-occurrence check does not require an occurrence value absent from the selected clause. `CheckedOccurrence` carries only role and ordinal, so two claims at the same ordinal compare equal and the swap succeeds. Require distinct ordinals or use an explicitly absent ordinal/wrong role. |

### Round 3 at ae2803b0474534e10e00d7752ccc3587985bb587

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-005 | fixed | TC-020 step 12 now supplies a wrong-role occurrence and a `claim` occurrence with an ordinal absent from the selected clause, retaining the correct owner. IR rejects the first by role and the second by membership; the restored authentic occurrence with wrong owner is rejected by the owner check. The two distinct-clause positive fixture remains. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Swapping the other clause's occurrence can be a no-op because occurrence identity contains only role and ordinal; step 12 does not require distinct ordinal values | spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:78-80,124-126 |

## Verdict (round 2)

CONDITIONAL. FND-001, FND-003 and FND-004 are fixed, but new FND-005 needs a specified absent occurrence value. FR-011-AC-6 is verbatim unchanged and remains Planned.

## Verdict (round 3)

PASS. Every applicable finding is fixed or rejected with a recorded reason. The new negative controls match Contract IR main 93063f795966be94d0d3085e486d1976fcae44d8; FR-011-AC-6 and AC-7 are verbatim unchanged and remain Planned. No new defect was found in the round-three diff.
