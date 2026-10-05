---
id: SR-1570
title: "base review of the IR-629 fix round (quire-contract-codegen#287, disposition round 1)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@48a7a96eca9d7dbc66a3c7a808f042151da2607c; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/decisions/ADR-002-backend-adapter-boundary.md, spec/routed/matrix/tests.md, spec/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---
# New findings, disposition round 1: IR-629 (PR #287)

## Summary

Ticket: IR-629. This covers the fix commit 48a7a96 (diff 43fbf2f..48a7a96) under the base, integrity,
scope-boundary, EARS and failure-domain lenses. All twelve findings in SR-1565..1569 are fixed (see
each file's `## Dispositions`). The new and reworded text is FR-019-AC-12, AC-13 and AC-14
(now Analysis), the new FR-019-AC-15, open questions 5 and 6, the `from_identity` bullets, TC-046 and
the matrices. Measured: `make spec` exits 0, and `quire coverage --strict` gives 52 unbacked rows
against 44 at origin/main 28880ac. The 8 new rows are FR-019-AC-11..15, the two FR-019 matrix rows
(TC-046 and Analysis) and TC-046, all Planned. FR-019-AC-15 occurs nowhere else in `spec/`. No CI
file is touched and nothing from a research repository appears.

## Verdict

Not mergeable yet: three medium and three low new findings. AC-12 and AC-13 compare dispositions
"identically" across descriptors whose identity text differs, but a `Disposition` names its backend.
AC-15 constrains `from_identity` ahead of open question 2.

## Examined

- FR-019-AC-11, AC-12, AC-13, AC-14, AC-15 (examined)
- FR-019 bullets at lines 102-119 and the paragraph at 121-123 (examined)
- Open questions 5 and 6 against QSpec FR-290's advertised-mode table and PV-4 (examined; both accurate)
- TC-046 procedure, mutants and Status (examined)
- ADR-002 Q4 step 5 and Consequences (examined, clean)
- `Disposition`, `Cause::UnboundedExtent` and `negotiate_kani` in src/routed/capability.rs (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-12 requires two descriptors with "different identity text" to "settle identically", and says "a mutant arm that reads the identity text" fails. But `Disposition::Supported`/`RequiresBound { backend }` and `Cause::UnboundedExtent { backend }` carry the backend identity (FR-019 Outputs require it), so every correct arm reads that text (`negotiate_kani` clones `backend.identity`) and the two dispositions differ. As written, the criterion fails a correct implementation | spec/routed/functional/FR-019-capability-settlement.md:169 |
| FND-002 | medium | FR-019-AC-13 requires a descriptor naming a non-existent executable to "settle identically to one with an ordinary identity". The two dispositions differ in the backend they name, so the comparison fails for a correct arm unless it excludes the backend field, and the criterion does not say so | spec/routed/functional/FR-019-capability-settlement.md:170 |
| FND-003 | medium | FR-019-AC-15 and the bullet at lines 117-119 fix `from_identity` to return `None` for an identity no variant names, and forbid the process-provider variant from acting as a catch-all. Open question 2 still asks "or does `from_identity` change?", and PV-4 says nothing about `from_identity`. The criterion settles part of question 2 (and of question 6, for `kani`) before the QSL owner answers. "A built-in variant names" is also undefined: it does not say whether the process-provider variant names plugin identities | spec/routed/functional/FR-019-capability-settlement.md:115-119,136-140,172 |
| FND-004 | low | TC-046 Status says AC-11 to AC-13 and AC-15 "wait on questions 1, 2 and 3 as FR-019 states each". FR-019 actually gates AC-12 on 5 and 1, AC-13 on nothing and AC-15 on 2 and 6. Only AC-11 names question 3, although every one of them needs the variant, and so FR-022's arm, to compile | spec/routed/matrix/TC-046-process-provider-settlement.md:53-54 |
| FND-005 | low | Non-singular EARS: "shall never settle an unbounded extent `supported` ... and shall never narrow an extent" is two requirements | spec/routed/functional/FR-019-capability-settlement.md:109-110 |
| FND-006 | low | Non-singular EARS: "the generator shall return `None` ... and the process-provider variant shall not act as a catch-all" is two requirements with two subjects | spec/routed/functional/FR-019-capability-settlement.md:117-119 |

## Dispositions

Round 2, reviewed at a1359c260e8216ad3b095d0402a00e3ee05ee450. No new findings this round.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a1359c2 |
| FND-002 | fixed | a1359c2 |
| FND-003 | fixed | a1359c2 |
| FND-004 | fixed | a1359c2 |
| FND-005 | fixed | a1359c2 |
| FND-006 | fixed | a1359c2 |
