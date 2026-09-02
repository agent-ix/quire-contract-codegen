---
id: REV-007
title: "Tri-state harness and shaped-strategy preimplementation review"
type: SpecReview
analysis: gap-analysis
scope: "issue #3 harness and proptest strategy generation"
review_set: subset
---

# Tri-state harness and shaped-strategy preimplementation review

## Summary

FR-002 and TC-004 require generated code to distinguish rejected preconditions, accepted failures,
and passes. The accepted runtime supplies the tri-state verdict, proptest adapter, and indivisible
accepted/rejected/failed/discarded accounting types. The shared-assurance `main` revision supplies
deterministic Boolean lowering and packaged ProofAttestationV1 generation. This issue #3 remediation
is based directly on that revision and adds no local evidence framework.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-401 | high | A caller-assembled wrapper could evaluate postconditions against pre-state or invoke the subject before checking preconditions. Generated harness source therefore owns snapshot, precondition, invocation, and postcondition order. | FR-002, TC-004 |
| FND-402 | high | A generic `prop_filter` fallback could make supported correlated domains statistically improbable and hide discards. Supported ranges, memberships, and relations are shaped directly; residual rejection is distinct. | FR-002-AC-3, TC-004 |
| FND-403 | medium | Boundary campaigns can exercise only valid values and therefore miss immediately adjacent rejected cases. Boundary plans contain tagged inside and outside cases with overflow-safe endpoint handling. | FR-002-AC-2, TC-004 |
| FND-404 | medium | Shrinking can leave a shaped domain or silently convert a rejected case to success. Generated strategies preserve shaped constraints; residual constraints retain rejection and discard accounting. | FR-002-AC-4, TC-004 |
| FND-405 | medium | The accepted IR does not yet bind full executable entry points to typed pre/post clauses. This slice accepts explicit typed-clause and subject-binding inputs and invents no package-level adapter. | interface-001, IR PR #19 |

## Decision

Proceed with a fail-closed first slice that generates deterministic harness and strategy artifacts,
uses the runtime's tri-state/accounting APIs, and tests seeded generation and shrinking behavior.
Keep TC-004 and FR-002 matrix rows planned until the complete issue scope and independent review are
closed.

## Verdict

**READY FOR INDEPENDENT RE-REVIEW**, not accepted. The current remediation makes the generated
artifact own execution order, campaign accounting and conclusion, executable expected-domain checks,
and deterministic ProofAttestationV1 identity. Inputs outside the supported binding or campaign
shapes terminate as structured `unsupported` or `invalid-input` results without a partial artifact.
The planned matrix status remains unchanged until an independent review accepts the exact head.

## Coverage

| Finding | Disposition | Verification |
|---|---|---|
| FND-401 | Closed: generated harness owns snapshot, precondition, invocation, and postcondition order. | Executed generated-crate tests for rejected, passed, and failed paths. |
| FND-402 | Closed: supported populations are directly shaped and use no filtering fallback. | Seeded shrink execution for range, membership, correlation, and residual constraints. |
| FND-403 | Closed: boundary cases include overflow-safe inside/outside values and residual exclusions with neighbors. | Generated boundary census plus full-width fail-closed test. |
| FND-404 | Closed: expected-domain tags are checked against runtime tri-state verdicts; harness conclusion retains accepted/rejected/failed/discarded counts and enforces an accepted floor. | Executed accepted, rejected, mismatch, discard, and all-rejected conclusion tests. |
| FND-405 | Closed within the declared slice: callers supply explicit typed clauses/bindings and an attestation context; no package adapter is inferred. | API inspection and deterministic ProofAttestationV1 tests. |

## Round 2 external-review reconciliation

PR #12's final review at `a003f4c2` reported FND-1218 through FND-1231. That PR and its bespoke
evidence stack were superseded by the shared-assurance migration. The table below distinguishes
domain findings that still require implementation from findings retired by that migration. A
disposition of `implemented` means present and locally testable on this branch; it does not mean an
independent reviewer has accepted it.

| Finding | Disposition on current branch | Verification or rationale |
|---|---|---|
| FND-1218 | Implemented: the public generated campaign runner owns `TestRunner::run`, the only explicit discard path, verdict adaptation, and mandatory conclusion. The direct adapter and conclusion helper are private. | Generated-crate tests call only the owned runner for accepted, rejected, mismatch, discarded, and zero-case campaigns. |
| FND-1219 | Implemented: `minimum_accepted_cases` and `maximum_discarded_cases` are request inputs, generated constants, enforced gates, and deterministic identity inputs. | Policy changes alter both source and attestation identity; tests exercise the floor and ceiling. |
| FND-1220 | Implemented: an empty residual exclusion is invalid rather than silently equivalent to a range. | Structured `InvalidMembership` assertion at `constraint.excluded`. |
| FND-1221 | Implemented: boundary generation classifies the actual emitted population and requires both accepted and rejected entries for every constraint family, including correlation. The empty correlated branch is gone. | Full-width zero-offset correlation fails closed at `campaign.boundary`; ordinary correlated boundaries compile and execute. |
| FND-1222 | Implemented: enum cases now carry the same executable accepted/rejected domain classification and `VerdictKind` check as integer cases. | Generated enum test accepts `Passed` and rejects a mismatched `RejectedPrecondition`. |
| FND-1223 | Retired by migration: the deleted local manifest made the unenforced maximum-source claim. The shared ProofAttestationV1 shape makes no such claim, and interface-001 now says so explicitly. | Schema/API inspection; no replacement bespoke size field or evidence code is introduced. |
| FND-1224 | Retired by migration: Oracle, harness, and strategy implementations now share one accepted `main` base and one implementation-digest path. This branch does not alter Oracle identity logic. | Branch merge-base and diff inspection. |
| FND-1225 | Retired by migration: the mislabeled derivation-manifest input and manifest itself were deleted in favor of packaged ProofAttestationV1. | No local derivation-envelope serializer remains. |
| FND-1226 | Implemented for the declared Boolean harness slice: generated accepted, rejected, and discarded constructors bind disposition to the exact Boolean values in a private-field case type, and the generated runner consumes only that type. | A caller cannot pair a bare expectation Boolean with values; mismatch and discard tests execute the typed path. Integer and customer-enum entry-point bindings remain outside this explicit-Boolean harness interface. |
| FND-1227 | Implemented: FR-002-AC-5 and AC-6 cover owned campaign policy and value-bound expected domains, and TC-004 traces both. | Requirements, test case, interface, and matrix inspection. Matrix status intentionally remains planned pending review. |
| FND-1228 | Implemented: the internal shell generator returns source text rather than constructing a path/digest-bearing artifact that no consumer retains. | Read-site inspection. |
| FND-1229 | Implemented: one terminal-state function derives clause failures from the preserved lower-level generation code; diagnostics no longer contain a separately assigned contradictory answer. | Invalid clause and invalid attestation tests assert the preserved machine code and terminal state. |
| FND-1230 | Retired by migration: the bespoke Python evidence policy and its local test-count floor were deleted. | Shared Quoin/Quire assurance gates are reused unchanged. |
| FND-1231 | Implemented against the replacement attestation contract: invalid record digests and candidate revisions fail at the harness boundary before clause generation and preserve `InvalidAttestationContext` plus `InvalidInput`. | Dedicated TC-004 harness test covers both invalid fields and stable diagnostic path. |
