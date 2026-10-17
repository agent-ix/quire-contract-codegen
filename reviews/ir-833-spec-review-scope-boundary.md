---
id: SR-12088
title: "IR-833 scope-boundary review: the QSL value-authority boundary and FR-014-AC-47"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@843588258cc236867caedfc34ef5683ea7b77076 (PR #416); spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket: IR-833. Checked that every boundary sentence the PR adds is consistent with FR-014-AC-47 and names the right owner (IR-842 for generated oracle against QSL, IR-669 for RT against QSL), and that the boundary is enforced by an Inspection criterion whose scope covers the artifacts it speaks for. Owner attribution is consistent throughout: IR-842 is named for every generated-oracle leg and IR-669 only for the RT composition (FR-014 Dependencies, tests.md FR-018-AC-2 row). The new FR-021-AC-2 and AC-18 mirror FR-014-AC-47's form (test name, trace tag, output line, doc comment, TC step, matrix row) for the function-application corpus.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-014-AC-47 | examined | No test name, trace tag, test output line or doc comment of the TC-187 agreement cases, no TC-024 step and no oracle-matrix row for a TC-187 vector names agreement with the QSL value authority. \| Inspection |
| FR-014-AC-37 | examined | This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle against QSL agreement), and FR-014-AC-47 keeps CG's artifacts from claiming it. |
| FR-014 Dependencies | examined | quire-integration IR-669 owns a Contract Runtime against QSL composition ...; generated-oracle against QSL agreement is owned by quire-integration IR-842, not by CG. |
| FR-018 Purpose :44-50 | examined | Agreement with the quire-spec-language value authority is outside CG's boundary and not claimed by CG (FR-014-AC-47) |
| FR-018-AC-2 | examined | This criterion claims no agreement with the QSL value authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47). |
| TC-029 Description, step 4 | examined | No comparison with the QSL value authority runs here (FR-014-AC-47). |
| FR-021-AC-2 | examined | That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47). |
| FR-021-AC-4 | examined | CG claims no agreement with the QSL value authority, owned by quire-integration IR-842 (FR-014-AC-47). |
| FR-021-AC-18 | examined | That agreement is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47). |
| TC-024 :221-226, step 1 | examined | No comparison with the QSL value authority runs here: it is outside CG's boundary (CG may call the `qsl-replay` facade only, QSL arch-lint T12-A), owned by quire-integration IR-842, and FR-014-AC-47 keeps CG's artifacts from claiming it. |
| tests.md FR-018-AC-2 row | examined | generated-oracle against QSL agreement is owned by quire-integration IR-842 and Contract Runtime against QSL composition by IR-669 |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Every new boundary sentence cites FR-014-AC-47 as what keeps CG's artifacts from claiming QSL agreement, but FR-014-AC-47's scope is the TC-187 agreement cases, TC-024 steps and matrix rows for a TC-187 vector only. It does not cover FR-014-AC-37's Boolean and integer `eq`/`ne` differential corpus, TC-024's integer/rational/ordering paragraph, or FR-018-AC-2's composite-equality corpus, TC-029 steps and FR-018 rows. For those, the boundary is a sentence that no Inspection enforces. Keeping AC-37 and FR-018-AC-2 as Test with a boundary sentence rather than new ids is sound only if FR-014-AC-47 is widened to every oracle corpus, TC step and oracle-matrix row (FR-014, FR-018, FR-021), or an FR-018 Inspection id is added in AC-47's form, as FR-021-AC-2/AC-18 did for FR-021. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:486, spec/oracle/functional/FR-014-exact-scalar-oracles.md:477, spec/oracle/functional/FR-018-composite-equality-oracles.md:376, spec/oracle/matrix/TC-029-composite-equality-oracles.md:19-20 |
| FND-002 | low | FR-021-AC-18 (with AC-2, and inherited from FR-014-AC-47) forbids any artifact that "names agreement" with the QSL authority, while the boundary sentences, matrix rows and test doc comments this ruling relies on all name that agreement in order to disclaim it, the FR-021-AC-18 matrix row included. The intended reading (no artifact claims or evidences the agreement) is not what the text says. Saying "claims or presents as evidence agreement" would make the inspection decidable; since the ruling asks for AC-47's form, fixing AC-47 at the same time keeps them mirrored. | spec/oracle/functional/FR-021-function-application-oracles.md:265, spec/oracle/matrix/tests.md:46, spec/oracle/functional/FR-014-exact-scalar-oracles.md:486 |

## Verdict

CHANGES REQUESTED (one medium, one low). The boundary is drawn in the right place and owners are attributed correctly: CG keeps its direct-runtime agreement, IR-842 owns the generated oracle against QSL and IR-669 the RT composition. T12-A is cited as the reason. No line in the diff moves a QSL-owned application API line (IR-845) or contradicts it.

## Dispositions
