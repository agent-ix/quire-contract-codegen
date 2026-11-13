---
id: SR-17940486709564847047903
title: "IR-948 PR #812 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #812 at e1e3ec92; IR-948; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-126, FR-015-AC-129), docs/qualification/ir948-kani-profiles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-948. The spec delta is two status edits in FR-015: FR-015-AC-126's IR-948 sentence
changes from "remain PLANNED" to "PARTIAL ... the precondition, impossible-precondition and
invariant-mutant members are executed (FR-015-AC-129) and the postcondition and frame members
remain planned", and FR-015-AC-129 changes from "PLANNED (IR-948 slice 3)" to a PARTIAL preamble
naming the executed members, the test file and the record, and stating the postcondition is
authored as a `<=`/`>=` conjunction because QSL refuses the `=` form. The rest of both rows is
unchanged.

Checked against the planner ruling for this slice (IR-948 stays In Progress; AC-126/AC-129
PARTIAL naming the gaps; postcondition blocked by the missing state-reading renderer and the QSL
refusal; frame has nothing to forbid here and is covered by FR-015-AC-31/80): no AC is marked
Implemented, and every member called executed has a reproduced installed-Kani verdict. The
wording falls short of the ruling in four places below.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-015-AC-129 keeps, unchanged, its dependency "on the memory-fit work IR-1436 orders (IR-844, IR-241, IR-1022), because this control measured `MemoryExhausted`" and the closing sentence "the recorded measurement is 17,205,592,064 B against the 17,179,869,184 B ceiling at unwind 8, and unwind 4 ended `UnwindBoundExhausted`, neither of which meets the control". The new PARTIAL preamble of the same row says that control is now executed as `CoverUnsatisfied`. A reader cannot tell whether the impossible-precondition control is met; the record explains the 17.2 GB run was a different (parameter-only) package. Drop or re-scope the stale dependency and measurement, or mark them historical and superseded by the record | spec/kani/functional/FR-015-bounded-kani-obligations.md:967 |
| FND-002 | medium | The postcondition gap names no owner for its QSL half. The PARTIAL says "QSL refuses `self.balance = pre(self.balance) - amount` as `unproved-range`" but cites no ticket; QSL-1901 ("blocks IR-948", handed to terra:QSL:code) exists and should be named, together with IR-948 itself as owner of the CG state-reading postcondition renderer (the dependency text points at "slice 2b", whose renderer accepts only parameter inputs). Same gap in the record's "Not established" section | spec/kani/functional/FR-015-bounded-kani-obligations.md:967, docs/qualification/ir948-kani-profiles.md:39-41 |
| FND-003 | medium | FR-015-AC-129's normative body still requires the unit to author `post Debited ... { self.balance = pre(self.balance) - amount }`, while the new preamble says "the clause is authored as the conjunction `self.balance <= ... and self.balance >= ...`". It is ambiguous which text the closing four-kind run must use: whether the conjunction run would close AC-129 or must wait for QSL-1901. State one closing condition (e.g. "the `=` form once QSL-1901 lands; the conjunction is the interim fixture text and earns no credit") | spec/kani/functional/FR-015-bounded-kani-obligations.md:967 |
| FND-004 | medium | Frame member: AC-129's preamble says "the postcondition and frame members of the four-kind run remain PLANNED", and AC-126 says the same, yet the ruling (and the record: "the profile package grants its only field, so the frame generator has nothing forbidden to check") is that the frame member is covered by FR-015-AC-31 and FR-015-AC-80 over the two-field twin. With the package as specified a frame harness has nothing to forbid, so "PLANNED" has no closure path and no owner. AC-126's IR-948 clause is about "V2 pre/post/invariant profiles", so the frame does not belong there at all. Reword AC-129 to say the frame member is met by reference to AC-31/AC-80 (or name what package change would close it), and drop "frame" from AC-126 | spec/kani/functional/FR-015-bounded-kani-obligations.md:964, spec/kani/functional/FR-015-bounded-kani-obligations.md:967 |
| FND-005 | low | AC-126's list of executed IR-948 members ("precondition, impossible-precondition and invariant-mutant members") omits the healthy invariant `Verified`, which AC-129 and the record list as executed | spec/kani/functional/FR-015-bounded-kani-obligations.md:964 |

## Verdict

Changes requested (wording only). The status flips are honest: nothing is marked Implemented,
and every member called executed has an executed, reproduced verdict. FND-001 to FND-004 are
medium because a reader of AC-129 cannot tell whether the impossible control is met, who owns
the QSL half of the postcondition gap, which postcondition text closes the AC, or how the frame
member would ever close. All four are spec-text fixes in this PR.
