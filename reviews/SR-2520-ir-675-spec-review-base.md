---
id: "SR-2520"
title: "spec-review (base) of IR-675 FR-034 cross-role cause representation"
type: "SpecReview"
analysis: "base"
scope: "agent-ix/quire-contract-codegen@47dd57f79207eb4517e318752e316e696c9974bb; base 5ab6249658d9bd5f8977b95991b55cb9af1e7522; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: "subset"
---

## Summary

Ticket: IR-675. Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run e86ea724-594f-49cb-ad01-8568cc4c64d8. Reviewed head 47dd57f79207eb4517e318752e316e696c9974bb against measured main 5ab6249658d9bd5f8977b95991b55cb9af1e7522; SPEC-only diff of three files. Method: spec-review/base.

Base spec-review of the whole three-file SPEC diff: the new FR-034 'Cross-role refusal cause representation' section, FR-034-AC-40, TC-049 coverage row/step 27/Expected Results row and the owning tests.md index rows. The sub-analyses (EARS, integrity, failure-domain, scope-boundary, dependency) and the source-backed code-review are separate artifacts SR-2521..SR-2526. This base artifact records the cross-cutting AC/statement soundness finding only.

Examined units (role examined):

- `FR-034#cross-role-L814-819` (spec/kani/functional/FR-034-caller-death-ownership.md:814-819)
- `FR-034#cross-role-L821-824` (spec/kani/functional/FR-034-caller-death-ownership.md:821-824)
- `FR-034#cross-role-L825-827` (spec/kani/functional/FR-034-caller-death-ownership.md:825-827)
- `FR-034#cross-role-table-row1-L829-831` (spec/kani/functional/FR-034-caller-death-ownership.md:829-831)
- `FR-034#cross-role-table-row2-L832` (spec/kani/functional/FR-034-caller-death-ownership.md:832)
- `FR-034#cross-role-table-row3-L833` (spec/kani/functional/FR-034-caller-death-ownership.md:833)
- `FR-034#cross-role-table-row4-L834` (spec/kani/functional/FR-034-caller-death-ownership.md:834)
- `FR-034#cross-role-table-row5-L835` (spec/kani/functional/FR-034-caller-death-ownership.md:835)
- `FR-034#cross-role-L837-842` (spec/kani/functional/FR-034-caller-death-ownership.md:837-842)
- `FR-034#cross-role-L843-846` (spec/kani/functional/FR-034-caller-death-ownership.md:843-846)
- `FR-034#cross-role-L847-851` (spec/kani/functional/FR-034-caller-death-ownership.md:847-851)
- `FR-034#cross-role-L853-858` (spec/kani/functional/FR-034-caller-death-ownership.md:853-858)
- `FR-034#cross-role-L860-864` (spec/kani/functional/FR-034-caller-death-ownership.md:860-864)
- `FR-034#cross-role-L865-868` (spec/kani/functional/FR-034-caller-death-ownership.md:865-868)
- `FR-034-AC-40#s1` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `FR-034-AC-40#s2` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `FR-034-AC-40#s3` (spec/kani/functional/FR-034-caller-death-ownership.md:1165)
- `TC-049#coverage-AC-40-L106` (spec/kani/matrix/TC-049-caller-death-ownership.md:106)
- `TC-049#step27-L655-660` (spec/kani/matrix/TC-049-caller-death-ownership.md:655-660)
- `TC-049#step27-L661-666` (spec/kani/matrix/TC-049-caller-death-ownership.md:661-666)
- `TC-049#step27-L667-672` (spec/kani/matrix/TC-049-caller-death-ownership.md:667-672)
- `TC-049#step27-L673-678` (spec/kani/matrix/TC-049-caller-death-ownership.md:673-678)
- `TC-049#step27-L679-681` (spec/kani/matrix/TC-049-caller-death-ownership.md:679-681)
- `TC-049#blank-L682-684` (spec/kani/matrix/TC-049-caller-death-ownership.md:682-684)
- `TC-049#blank-L755-758` (spec/kani/matrix/TC-049-caller-death-ownership.md:755-758)
- `TC-049#expected-AC-40-L781` (spec/kani/matrix/TC-049-caller-death-ownership.md:781)
- `tests.md#FR-034-row-L54` (spec/kani/matrix/tests.md:54)
- `tests.md#TC-049-row-L85` (spec/kani/matrix/tests.md:85)

Context-only units:

- `FR-034#abi-scope-L659-664` (spec/kani/functional/FR-034-caller-death-ownership.md:659-664)
- `FR-034#admission-original-cause-L789-794` (spec/kani/functional/FR-034-caller-death-ownership.md:789-794)
- `FR-034#mapping-row-L1086` (spec/kani/functional/FR-034-caller-death-ownership.md:1086)
- `TC-049#slice-terms-L35-37` (spec/kani/matrix/TC-049-caller-death-ownership.md:35-37)

Not run, by policy: Cargo, build, tests, full make ci, Kani, runtime probes, strict matrix re-run. All CODE/runtime criteria remain PLANNED/UNRUN; this review gives no CODE or proof credit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-034-AC-40 asserts unconditionally that the 'Current Other-wrapped TryReserveError remains Other', while the FR-034 statement (lines 853-858) conditions the kind on the actual producer conversion and expressly allows a separately reviewed CODE change to a different conversion; the 'current' producer (creator.rs:149-155) does not exist at the reviewed head, so the AC binds a fact with no referent that becomes false or vacuous when the permitted change lands. Scenario: IR-639 CODE, under its own review as lines 857-858 allow, changes PreparedIdentity::prepare to `?` through From<TryReserveError>. The FR then requires OutOfMemory with absent source, but AC-40 still says the 'Current Other-wrapped TryReserveError remains Other': a test written to AC-40 has no Other-wrapped producer left to exercise and either asserts nothing or reads the AC as forbidding the reviewed change. At 47dd57f, src/kani/run contains no creator.rs, so 'Current' already has no referent in the reviewed tree. | spec/kani/functional/FR-034-caller-death-ownership.md:1165; spec/kani/functional/FR-034-caller-death-ownership.md:853-858 |

## Verdict

The amendment makes an honest, source-grounded narrowing: it states that an arbitrary boxed error cannot cross a process boundary, keeps exact errno/kind for OS errors, separates the Other-wrapped TryReserveError producer from the standard OutOfMemory conversion, forbids message-derived classification, and leaves local-role sources and AC-39 unchanged. All 39 prior AC rows and the TC-049 old text are unchanged in the diff; AC-40 is standalone, PLANNED/UNRUN and untagged. One medium soundness gap: AC-40 states a present-tense producer fact that the FR text makes conditional and that has no referent at the reviewed head. Criterion-strength (Jev) was not run: no Jev client is installed (`jev` not on PATH; the installed quoin 0.28.2 criterion-strength skill states no client exists), so AC failability is judged here by reviewer reading only, without calibrated semantic judgment.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@16572f234284384a6117ea15b9ee0aed5aa204b5 (fix diff 47dd57f..16572f2; source fix commit 16572f234284384a6117ea15b9ee0aed5aa204b5). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run a5df61d0-d398-40ac-b2a5-5eacdbacb452. Changed lines re-checked for regressions of each finding.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 16572f2: AC-40 no longer asserts a timeless 'Current' producer fact: it conditions on the actual producer (Other wrapper stays Other; a standard conversion keeps its actual observed kind/source presence), matching FR lines 884-890. |
