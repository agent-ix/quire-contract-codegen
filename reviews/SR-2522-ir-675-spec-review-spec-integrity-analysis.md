---
id: "SR-2522"
title: "spec-integrity-analysis of IR-675 FR-034 cross-role cause representation"
type: "SpecReview"
analysis: "integrity"
scope: "agent-ix/quire-contract-codegen@47dd57f79207eb4517e318752e316e696c9974bb; base 5ab6249658d9bd5f8977b95991b55cb9af1e7522; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: "subset"
---

## Summary

Ticket: IR-675. Reviewer session dcb5e3e7-8fe4-422e-aef1-3ca57d78bee2, model claude-opus-5-5, run e86ea724-594f-49cb-ad01-8568cc4c64d8. Reviewed head 47dd57f79207eb4517e318752e316e696c9974bb against measured main 5ab6249658d9bd5f8977b95991b55cb9af1e7522; SPEC-only diff of three files. Method: spec-review/spec-integrity-analysis.

Completeness, consistency and atomicity of the new section against the unchanged FR-034 original-cause promises (lines 659-664, 789-794, mapping row 1086), FR-034-AC-40, the TC-049 coverage row, step 27 and Expected Results row, and the tests.md index rows (FR-034 range AC-1..AC-40 and TC-049 criterion list). Index range and criterion lists are consistent: AC-40 is added to both and to no other row.

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
| FND-001 | medium | The new section redefines cross-role 'original cause' retention for generic startup/observation causes without amending or cross-referencing the unchanged clauses that still promise the original cause unqualified (line 791 'retain the original cause', mapping row 1086 'MemoryMechanismUnavailable with original io::Error', and line 664 'No other original-cause retention obligation is relaxed by this scoped allocation'), so for cross-role admission failures those clauses and the new table give different requirements. Scenario: An inner-role PrivateProc admission failure produced as io::Error::other(String) crosses to C. Read under line 791 and row 1086, MemoryMechanismUnavailable.cause must be the original io::Error with its custom payload; under the new table it is a reconstructed Other with the payload marked unrepresented. A reviewer applying line 791 rejects the conforming implementation, or an implementer applying it attempts the impossible object transport the new section rules out. | spec/kani/functional/FR-034-caller-death-ownership.md:821-827; spec/kani/functional/FR-034-caller-death-ownership.md:664; spec/kani/functional/FR-034-caller-death-ownership.md:791; spec/kani/functional/FR-034-caller-death-ownership.md:1086 |
| FND-002 | medium | FR-034 lines 849-850 require public bounded-execution documentation to explain the cross-role projection limit, distinguish local original-source retention and not promise a public role/stage query, but FR-034-AC-40, TC-049 step 27 and the AC-40 Expected Results row carry no documentation obligation (unlike AC-38, which states 'Public bounded rustdoc explains ...'). Scenario: CODE delivers the transport and the representation; the public rustdoc for MemoryMechanismUnavailable still says it carries the original `cause: std::io::Error`. Every AC-40 Test and the step 27 Analysis pass, and the documentation requirement of lines 849-850 is never checked, so a public consumer is told the original error object survives a role boundary. | spec/kani/functional/FR-034-caller-death-ownership.md:849-850; spec/kani/functional/FR-034-caller-death-ownership.md:1165; spec/kani/matrix/TC-049-caller-death-ownership.md:655-681; spec/kani/matrix/TC-049-caller-death-ownership.md:781 |
| FND-003 | low | TC-049 step 27 allocates its ordinary production case to 'the first slice' where TC-049 defines the term 'Slice 1', and the change adds two doubled blank lines, one of them (lines 756-757) in a region between the AC-32 section and '## Expected Results' that the change otherwise does not touch. Scenario: A reader or tool searching TC-049 for 'Slice 1'/'slice 1' allocations (lines 35-50, 81, 118) does not find step 27's ordinary-transport allocation; the extra blank line at 757 enlarges the diff in an unrelated section without content. | spec/kani/matrix/TC-049-caller-death-ownership.md:675; spec/kani/matrix/TC-049-caller-death-ownership.md:682-683; spec/kani/matrix/TC-049-caller-death-ownership.md:756-757; spec/kani/matrix/TC-049-caller-death-ownership.md:35-36 |

## Verdict

The index is consistent and every old AC row is unchanged. Two medium gaps: the narrowing of 'original cause' is stated only in the new section and never reconciled with the unchanged clauses that still promise the original io::Error, and the new public-documentation obligation has no acceptance criterion or procedure. One low layout/terminology nit in TC-049.
