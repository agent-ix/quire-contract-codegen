---
id: "SR-2521"
title: "spec-ears-analysis of IR-675 FR-034 cross-role cause representation"
type: "SpecReview"
analysis: "ears-conformance"
scope: "agent-ix/quire-contract-codegen@47dd57f79207eb4517e318752e316e696c9974bb; base 5ab6249658d9bd5f8977b95991b55cb9af1e7522; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: "subset"
---

## Summary

Ticket: IR-675. Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run e86ea724-594f-49cb-ad01-8568cc4c64d8. Reviewed head 47dd57f79207eb4517e318752e316e696c9974bb against measured main 5ab6249658d9bd5f8977b95991b55cb9af1e7522; SPEC-only diff of three files. Method: spec-review/spec-ears-analysis.

EARS conformance of every new requirement statement in the FR-034 'Cross-role refusal cause representation' section (lines 812-868) and the new FR-034-AC-40 row. The section opens with a conforming event-driven statement (line 814: 'When ... the sending owner shall capture ...') and the producer-conversion paragraph (lines 853-856) is in conforming 'When ... C shall ...' form. Two low findings on actor and grammar.

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
| FND-001 | low | The five-row representation table states its obligations as actor-less imperatives ('Retain ...', 'reconstruct ...', 'Mismatched metadata refuses') and line 837 ('The finite reconstructible set shall be source-grounded in the actual emitting paths') constrains the specification/Analysis rather than stating a system response; neither names the responsible role (sending owner or C) in EARS form. Scenario: Row 1 says 'reconstruct the public I/O cause directly from the errno and verify its kind on the same running platform'. One implementer verifies in the sending role before encoding; another verifies only in C after decoding. Both read the row as satisfied, and a TC-049 step 27 assertion placed on one side does not detect the other side omitting the check. | spec/kani/functional/FR-034-caller-death-ownership.md:829-835; spec/kani/functional/FR-034-caller-death-ownership.md:837 |
| FND-002 | low | 'The owner shall encode finite cause/provenance facts ... and charge their actual metadata/storage under the existing named caller-buffer and whole-run limits' (line 860) uses the undefined actor 'the owner', where line 815 says 'the sending owner' and line 816 says 'C', so it is unclear which role's buffers are charged for the encoding. Scenario: An implementation charges the decoded cause metadata only against C's caller buffer and leaves the sending role's encode-side control storage uncharged; another charges only the sender. Each satisfies 'the owner' as written, and the AC-40 'charged buffers' clause cannot tell them apart. | spec/kani/functional/FR-034-caller-death-ownership.md:860-862; spec/kani/functional/FR-034-caller-death-ownership.md:815-816 |

## Verdict

Most statements name the system and use 'shall'. The representation table and the 'source-grounded' constraint are not in requirement grammar, and 'the owner' is an undefined actor where the surrounding text distinguishes the sending owner from C. Neither changes behaviour as written; both leave room for two implementers to place the obligation on different roles.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@16572f234284384a6117ea15b9ee0aed5aa204b5 (fix diff 47dd57f..16572f2; source fix commit 16572f234284384a6117ea15b9ee0aed5aa204b5). Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run a5df61d0-d398-40ac-b2a5-5eacdbacb452. Changed lines re-checked for regressions of each finding.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 16572f2: The representation table now has separate 'Sending-owner obligation' and 'C receiving/projection obligation' columns, each stated as 'The sending owner shall ...' / 'C shall ...'; the source-grounding constraint is replaced by a declared closed set with actor-specific shall statements (lines 860-871). |
| FND-002 | fixed | 16572f2: 'The owner' is replaced by 'The sending helper shall charge ...' and 'C shall charge ... to named caller_run_buffers'; caller_run_buffers is an existing named term (FR-034 lines 941-969), and neither side's charge replaces the other's. |
