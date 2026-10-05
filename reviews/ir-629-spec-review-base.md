---
id: SR-1565
title: "base review of IR-629 process-provider BackendKind spec (quire-contract-codegen#287)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@43fbf2f6083663e680e1ebeeeeefe11b288fdef5; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---
# Base review: IR-629 process-provider BackendKind (PR #287)

## Summary

Ticket: IR-629. Base checklist over `git diff origin/main...43fbf2f` (6 files, +108/-3). IDs are
well formed and free: FR-019-AC-11..14 and TC-046 occur nowhere else in `spec/`; `make spec`
(quire 0.36.1, engine 0.50.1) exits 0 with no diagnostic on a changed file. All four new criteria
are `🚧 Planned` in both the routed matrix and `spec/tests.md`. `quire coverage --strict` measured
44 unbacked rows at origin/main 28880ac and 50 at the head; the delta is exactly FR-019-AC-11..14,
the FR-019 matrix row and TC-046. No CI file is touched. Three criterion-level defects follow.

## Verdict

Changes requested (three medium findings). Verified clean: ID format and uniqueness, Planned
status, strict delta of 6, no CI edits, no public-repo leak (no research findings, no private-repo
paths), TC-046 traces FR-019 with a `verifies` edge, open questions 1-4 quote QSL ADR-029 PV-4
(QSL origin/main a9cfe11, lines 458-465 and 808-809) verbatim.

## Examined

- FR-019-AC-11, FR-019-AC-12, FR-019-AC-13, FR-019-AC-14 (examined)
- FR-019 Behavior bullets at lines 102-115 and open questions 1-4 (examined)
- TC-046 procedure, expected results, seeded mutants, status (examined)
- FR-019-AC-10, FR-019 advertised-mode bullets, QSpec FR-290 advertised-mode table (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-13's clause "the arm takes no argument from which a plugin can be reached" contradicts its own fixture: the arm takes the `BackendDescriptor`, whose `identity` the same criterion says names an executable, so the clause is false as written and no test can establish it | spec/routed/functional/FR-019-capability-settlement.md:153 |
| FND-002 | medium | FR-019-AC-14 has no test-falsifiable content of its own: "no terminal value, verification result or artifact" is a type property of `Disposition` (four variants, none carries one); "no non-supported settlement routes" restates FR-019-AC-10, and its "routes an unsupported item" mutant has no site in the arm, which returns a `Disposition` and routes nothing (routing is QSL `route`, FR-022); its other mutant duplicates FR-019-AC-12 | spec/routed/functional/FR-019-capability-settlement.md:154 |
| FND-003 | medium | FR-019-AC-12 fixes a bounded extent, and an unbounded extent against advertised `unbounded`, to `supported`; QSpec FR-290's table leaves those three rows to "the negotiation arm's own disposition for the IR form", and PV-4 does not state them. The choice is CG's (it matches `negotiate_kani`), but the spec presents it as "the same dispositions ... as every other arm" rather than as a stated decision or an open question | spec/routed/functional/FR-019-capability-settlement.md:108-109,152 |

## Dispositions

Round 1, reviewed at 48a7a96eca9d7dbc66a3c7a808f042151da2607c. New defects found this round are recorded in SR-1570.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48a7a96 |
| FND-002 | fixed | 48a7a96 |
| FND-003 | fixed | 48a7a96 |
