---
id: SR-2026102764
title: "Base review of IR-1150 structural parity claim specification"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md; IR-1150"
review_set: subset
org: agent-ix
---

## Summary

PR #656 head 1bd64093 was reviewed against IR-1150, QSL-1293 and QSpec FR-322. The replacement preserves the separate positional, membership, changed-member and artifact-authentication obligations, but FR-032-AC-15 misassigns scalar-kind and operand production to the IR reader. This conflicts with FR-032's own explicit CG generation gate. No QSL-1293 implementation or CG runtime acceptance is claimed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-032-AC-15 says the IR reader supplies scalar kind and positional arguments, but FR-032 assigns typed scalar-kind production and argument retention to CG generation; this makes the replacement criterion demand an API that IR does not own. State that IR supplies application identity and checked occurrence/operand metadata, while CG generation supplies and retains the scalar kind and positional claim arguments. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:348; same file:202,264,317; QSpec FR-322 | wrong-requirement |

## Scope and verdict

Examined AD-003 E-1 scalar and composite replacement and E-2 context; FR-024-AC-44; FR-032-AC-9 through AC-18, including the tombstones and replacements; FR-033-AC-11, AC-12, AC-14 through AC-18. FR-024 function/frame identity, QSpec FR-322 node-key shape and QSL-1293 are context only. The new tests remain CODE-GATED, so the current matrix cannot discharge them. No applicable AssuranceProfile with review_policy was found under this repository's spec tree. The replacement retains distinct draws and bounds, no fabricated identity, membership refusal, body-only mutation detection, and independent same-artifact binding. The finding blocks readiness until ownership wording is corrected.

## Validation

The four changed specification files passed scoped Quire validation. No Cargo or code gate was run for this specification-only review.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At cbe8155a, FR-032-AC-15 now assigns only the original application structural identity and authentic expression occurrence to the IR reader; CG typed generation supplies and retains scalar kind and ordered arguments. Adjacent FR-032 prose and AC-18 use the same ownership split. |

## Final validation

Scoped Quire validation of all four changed artifacts and `git diff --check` passed at cbe8155a. The earlier finding is resolved; this spec-only PR is ready for the code lead's merge path. The QSL-1293-dependent code leg remains gated.
