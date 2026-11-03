---
id: SR-2026110283848
title: "IR-724 PR #735 spec-review/evidence review"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen PR #735; IR-724; spec/evidence/functional/FR-004-vacuity-evidence.md; spec/evidence/matrix/TC-006-vacuity.md; spec/evidence/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
---

## Summary

The authored Test methods for AC-8 and AC-10 through AC-15 have no quoin advise mismatch. Quoin advises Test for AC-17, but the criterion asks specifically for source inspection of the V2 analyzer, so Inspection is the correct method for that obligation. The advisor is generic and its AC-17 mismatch is rejected on direct criterion inspection. All nine targets remain unbacked without false credit. No applicable AssuranceProfile was found under spec/. The diff changes only three specification files and adds no copied artifact, new pin, digest, source code, test, or coverage tag. Quire validation and git diff --check pass.

## Verdict

**PASS. Evidence methods remain appropriate; AC-17 advisory mismatch is inapplicable to source inspection.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-004-AC-8 | examined | PLANNED / UNRUN. Adverse coverage and non-success native execution cannot discharge the coverage obligation merely because a report serialized successfully. |
| FR-004-AC-10 | examined | PLANNED / UNRUN. CG binds coverage inside the same live campaign-runner invocation, using the original `/2` wire bytes and their QSL strict-reader typed result directly, the producer execution record, and the same generated source/map buffers used by that campaign. The runner starts the campaign with fresh private profile output, performs any required LLVM merge over only that output, starts LLVM export over only that profile, and captures exact export bytes and tool/version from that LLVM child's direct output stream while retaining the linked child handles. Binding verifies this live campaig |
| FR-004-AC-11 | examined | PLANNED / UNRUN. The consuming report retains vacuity and native outcome as evidence for IR FR-045; an oracle-success run with an unobserved consequent denies the coverage obligation, and LLVM probe observations alone produce no completed-proof credit or separate eligible denominator. |
| FR-004-AC-12 | examined | PLANNED / UNRUN. CG consumes QSL's typed strict-reader clause-run or command-error variant for exact `native-run-result/2`: a decisive basis retains its witness with assigned members, while `closed-scope` or `unavailable` retains none. Missing or other versions, missing/unknown basis, malformed witness, contradictory basis/witness and mixed/invalid variant shape are located wire refusals before binding; no `/1` fallback or CG copy of the wire reader exists. |
| FR-004-AC-13 | examined | PLANNED / UNRUN. QSL `run_clause` success and a decisive witness remain source-clause facts and do not count as generated-campaign execution or LLVM coverage; QSL violation, refusal, undefined, incomplete, cancelled, unsupported and internal failure remain distinct typed adverse/non-success facts. A bound live run does not upgrade any QSL disposition or itself grant campaign success, measured classification, proof credit or coverage discharge. The same reader's command-error variant retains stage/code/optional cause/message/details and unavailable basis as non-success, with no campaign qualifi |
| FR-004-AC-14 | examined | PLANNED / UNRUN. Only CG's campaign runner constructs the private, non-cloneable, non-serializable live run context; callers cannot construct or restore it from a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent, foreign or expired context, or an omitted association yields structured binding non-success retaining the typed QSL outcome and available identities, with no qualification, classification or discharge. A complete same-invocation context admits binding without changing the QSL outcome. The caller supplies maximum encoded-input and decoded-working-byte lim |
| FR-004-AC-15 | examined | PLANNED / UNRUN. A controlled attempt to replace the strict-reader typed result, execution record, generated source, source map, LLVM export stream or LLVM tool/version with a foreign-run value cannot construct a valid same-invocation context, even when paths, QSL `package_id`, source/map and tool/version declarations match. In particular, foreign export bytes offered through a detached buffer or a different producer stream/child handle refuse as `producer_run_mismatch` before classification because neither is the current runner-owned campaign/profile/export chain. A malformed association obse |
| FR-004-AC-17 | examined | PLANNED / UNRUN (IR-815; pending IR-1013). Source inspection shows the V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names no `BoundPackage` or `ClauseRef`, both of which today's `analyze_bound_coverage` names. |
| FR-004 | examined | \| FR-004 \| FR-004-AC-1 through FR-004-AC-7 \| TC-006 \| 🚧 Planned / UNRUN; primitive controls are partial evidence, while production live campaign binding and consuming-obligation evidence are unrun \| |
| TC-006 | examined | \| TC-006 \| Distinguish vacuity and unexecuted flow \| Integration \| P0 \| FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7, FR-004-AC-8, FR-004-AC-9, FR-004-AC-10, FR-004-AC-11, FR-004-AC-12, FR-004-AC-13, FR-004-AC-14, FR-004-AC-15, FR-004-AC-16, FR-004-AC-18, FR-004-AC-19, FR-004-AC-20, FR-004-AC-21, FR-004-AC-22, FR-004-AC-23, FR-004-AC-24, FR-004-AC-25, FR-004-AC-26 \| 🚧 Planned / UNRUN; AC-25 superseded with no credit; bounded observation primitives are partial evidence only; QSL-520 public `/2` reader is delivered, while CG consumer, production live  |
