---
id: SR-35011
title: "Base spec review of IR-1007 CG PR 566 (FR-021 re-point to QSL FR-485)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-021-function-application-oracles.md; spec/oracle/matrix/TC-031-function-application-oracles.md; spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-1007. PR agent-ix/quire-contract-codegen#566 at frozen head
fd2a03a69a75130352358acc90c9036faeeba10c, diff against origin/main only.
Independent base checklist pass (prior gpt-6 reviewer artifacts under
`reviews/` read for reference only, not relied on). Checked against QSL draft
agent-ix/quire-spec-language#1470 at e66db5177758 (FR-485, FR-262-AC-13,
TC-1263).

The re-point is truthful in the main: every execution-bearing row (AC-2, AC-4,
AC-5 corpus half, AC-9 vectors, AC-17 runtime check, AC-32) stays PLANNED
pending IR-1007 (Linear: Coding) and IR-595 (Linear: In Progress); the
tests.md FR-021 rows and the TC-031 summary row were downgraded, not upgraded;
the current RT `exact` import (`src/oracle/function/mod.rs:154`) is stated
honestly; IR-853 is Done in Linear as the Status claims; no AC was narrowed
except AC-16 (see the integrity review); no duplicate evaluator, fallback or
compatibility wording was added, and the RT port is explicitly forbidden as
target or comparison. CG is routed only through `qsl_replay` (FR-485 AC-3/AC-8
consistent). `quire validate` on the three spec files exits 0; the engine
EARS check reports FR-021 grammar-clean.

Defects: AC-2 does not pin the direct side to the interpreted callable, AC-4's
unknown-function refusal has no FR-485 counterpart at Begin, and the new
`depends_on` edge targets an FR that exists only in an unmerged draft.

## Scope examined

- FR-021 Description (re-pointed paragraph), Inputs (new FR-485 bullet), Outputs, Constraints, Dependencies (QSL owner path), Status, Out of Scope: examined.
- FR-021-AC-1, AC-2, AC-3, AC-4, AC-5, AC-7, AC-8, AC-9, AC-10, AC-14, AC-16, AC-17, AC-19, AC-20, AC-26, AC-31, AC-32 (all changed rows): examined, each compared with its origin/main text for narrowing.
- FR-021-AC-18 and unchanged rows: context only.
- TC-031 Description, steps 6, 7, 11, 12, 12a, 12b, Expected Results: examined.
- spec/oracle/matrix/tests.md FR-021 rows (AC-1..3/5..14/17, AC-4, AC-31, AC-32) and the TC-031 summary row: examined.
- QSL FR-485 (#1470), FR-262 diff and AC-13, TC-1263: context only.
- spec/assurance/AD-004-cg-crate-layout.md lines 359-366 (outside diff): context only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021-AC-2 requires the direct application to be "independently assembled ... through QSL FR-485 acquisition, callable selection and session application" but never says that side runs QSL's interpreted callable with no generated body registered. Both sides go through the same leaf, so a test that registers the same generated body on both sides satisfies the AC tautologically. Only TC-031 step 12b says "using QSL's interpreted callable". State that in the AC. | spec/oracle/functional/FR-021-function-application-oracles.md:265; spec/oracle/matrix/TC-031-function-application-oracles.md:144-149 |
| FND-002 | medium | FR-021-AC-4 (and TC-031 step 12b) say the unknown-function input refusal is returned "unchanged ... through the FR-485 owner path". Under FR-485 an unknown callable is refused by `lookup_callable` as `Unknown` (or `Foreign`/`WrongKind`) before any selection or Begin exists, never as a Begin `CallFailure::Input`. The AC does not say which FR-485 result counts as the unchanged refusal, so two implementers would assert different things. | spec/oracle/functional/FR-021-function-application-oracles.md:267; spec/oracle/matrix/TC-031-function-application-oracles.md:151 |
| FND-003 | medium | FR-021 adds `depends_on ix://agent-ix/quire-spec-language/FR-485`, and its ACs cite FR-485 behavior, but FR-485 is not on QSL `main`; it exists only in draft #1470. Merging this PR first leaves a dangling cross-repo edge and ties CG rows to unreviewed text that may still change or be renumbered. Merge order: after #1470 lands, then re-check the cited FR-485 terms. | spec/oracle/functional/FR-021-function-application-oracles.md:24-25 |
| FND-004 | low | AD-004 (outside the diff) still says QSL must "publish" the checked-closure route and that "FR-021 and IR-853 retain the concrete-oracle scope decision". That contradicts FR-021's new Status, where IR-853 is Done and FR-485 is the route. Update the paragraph so the architecture doc and the FR agree. | spec/assurance/AD-004-cg-crate-layout.md:359-366 |
| FND-005 | low | The PR body says `git diff --check` passed, but on this head it fails on the committed prior-review artifacts `reviews/ir-1007-*.md`: their truncated excerpts end in trailing whitespace. The three spec files are clean. | reviews/ir-1007-code-review.md:27 |

## Verdict

Changes requested before merge (fix in this PR): FND-001 and FND-002 are
small AC wording fixes. FND-003 is a merge-order condition, not a text
defect: keep this PR draft until QSL #1470 merges. FND-004 and FND-005 are
low-severity clean-ups.

## Dispositions

### Round 1 — exact `221c5514399aba606d0c383e433bbc4e8f61309e` (fix diff `fd2a03a6..221c5514`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-021-AC-2 now requires the direct side to run "session application of the QSL interpreted callable, with no generated body registered in that direct registry"; TC-031 step 12b matches ("using QSL's interpreted callable with no generated body registered in the direct registry"). Registering the same generated body on both sides no longer satisfies the AC. |
| FND-002 | fixed | FR-021-AC-4 now splits the cases: arity, wrong-value-kind and dangling-reference are forwarded from `ApplicationBeginRefusal::Call(CallFailure::Input)`; an absent name keeps CG `UnknownFunction`; an invalid owner/callable id refuses at FR-485 `lookup_callable` as `Unknown`, `Foreign(package_id)` or `WrongKind` before selection or Begin. Checked against QSL #1470 at e66db517 FR-485 Inputs, behaviour 3 and AC-2/AC-6: the names match. TC-031 step 12b says the same. No case is dropped compared with origin/main. |
| FND-003 | still-open | This is a merge-order hold, not a text defect. QSL agent-ix/quire-spec-language#1470 is still an OPEN draft at e66db5177758, so FR-485 is not on QSL `main` and FR-021's `depends_on` edge has nothing on main to point at. The cited FR-485 terms (`lookup_callable`, `Unknown`/`Foreign`/`WrongKind`, `DigestRecord`, `WireNodeId`, `ApplicationBeginRefusal::Call(CallFailure::Input)`) match #1470 at that head. This clears when #1470 merges, after a re-check of those terms against the merged text. No CG text change is needed. |
| FND-004 | fixed | AD-004 lines 354-369 now say the concrete function oracle "still imports RT's `exact` port; replacing it is an open FR-021-AC-32 obligation". They name QSL FR-485 as the `qsl_replay` facade, say "FR-021 holds the concrete-oracle acceptance criteria Planned until that work lands" and that "IR-853's target decision is Done". This agrees with the FR-021 Status. IR-853 and IR-583 are Done in Linear, so saying the `quire-eval` leaf "provides" the interface is accurate. |
| FND-005 | fixed | The trailing whitespace in `reviews/ir-1007-*.md` is removed. `git diff --check origin/main..221c5514` and `git diff --check fd2a03a6..221c5514` both exit 0. |

No new findings this round. `quire validate --scope .` on FR-021, TC-031, `spec/oracle/matrix/tests.md` and AD-004 reports 4/4 docs grammar-clean and 0 findings (quire 0.36.2, engine 0.50.2).
