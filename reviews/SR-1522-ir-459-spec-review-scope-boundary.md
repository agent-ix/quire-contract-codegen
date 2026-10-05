---
id: SR-1522
title: "scope-boundary review of quire-contract-codegen#280 (IR-459 slice vs QSL-345 and the function path)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@70f5e08ac399d0880b50c1610592220c15298e09; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md (diff only)"
review_set: subset
---

## Summary

Ticket: IR-459. I checked the slice's boundaries: the CG/QSL seam, the frame path versus the
function and state-clause paths, and the public-repo rules.

What holds:

- Nothing is built for QSL-345 or field domains. FR-024-AC-29 keeps `declared_domains` empty,
  builds no `DeclaredDomain` or `DomainKey`, and cites QSL-345 by ticket id only.
- The state-clause path is left alone (Q-1).
- No compatibility layer appears: AC-20 keeps no reader of the old spelling.
- The diff contains no private planning text, research-repo paths or branch names.
- The pre-state tie is a check, not a build (AC-27).
- IR-459's text asks for "the declared domains". This PR defers them, which is consistent with
  "Part of IR-459": the PR does not close the ticket.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-024-AC-20 renames the function path's preimage members `function`/`declaration` to `subject`/`occurrence` and rewrites the golden vectors. That changes every function-path identity once. IR-459 does not ask for a rename. Under the team rule, an unrequested rename is an owner question, yet AC-20 mandates it while Q-6 still asks the owner. The break is stated plainly: AC-20 says "its identities change once, with no reader of the old spelling kept", and AD-003 E-1 says "nothing reads the old spelling". It is also harmless, measured: QSL only carries the slot as a label, and CG produces the identity in `obligation.rs` and its tests only. The rename is not strictly necessary, because one function could mint both preimages with a subject-specific member name. But ADR-013 O-09 itself describes the function and clause subjects as one shape ("identified the same way ... in place of the clause's"), so neutral names fit it. Recommendation: the owner answers Q-6 before merge. The reviewer leans toward accepting the rename. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:286,371-373 |

## Dispositions

Round 1, reviewed at b094aaca23d924831ee362d8865d31b74acbb753.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b094aac |
