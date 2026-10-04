---
id: "SR-1430"
title: "CG PR 262 spec review (EARS conformance): FR-015 non-vacuity cover Behavior bullets"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@a5cd4bd54fece9ad071a399691d4dd106d9ade2d; spec/kani/functional/FR-015-bounded-kani-obligations.md:248-265 (four new Behavior bullets), compared with the existing FR-015 bullets (When/If ... the generator shall ...)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-1430: CG PR 262 spec review (EARS conformance)

## Summary

Ticket: IR-464. The PR adds four Behavior bullets. `quire validate` raises no EARS warning on
FR-015. Its grammar check passes them. The review below is a manual read against the EARS
patterns and the style of the surrounding FR-015 bullets.

- Bullet 1 (line 248, "The generator shall end every harness it emits, of every kind, with
  exactly one non-vacuity cover ..."): ubiquitous, single subject, single response. It conforms.
  It duplicates the existing line-110 bullet; that is recorded in SR-1429 FND-006.
- Bullet 2 (line 253, "The generator shall end a V1 bundle harness ... and shall end every
  bounded-corpus harness ..."): ubiquitous, but with two `shall` responses over two subjects.
- Bullet 3 (line 258, "A healthy run of a V1 bundle harness and of a bounded-corpus harness under
  the installed backend shall classify `Verified`; a V1 bundle run whose requires clause no
  bounded argument satisfies shall classify `CoverUnsatisfied` ..."): the subject before `shall`
  is "a run", not the system. It joins a ubiquitous response and an unwanted-behaviour response
  in one sentence.
- Bullet 4 (line 262, "The crate's test suite shall fail for a harness emitted without a cover
  ..."): the subject is the test suite, so this is a verification obligation phrased as system
  behaviour.

## Verdict

Bullets 1 and 2 are acceptable. Bullet 3 should be restated with the classifier as the subject,
as one ubiquitous clause and one If/then clause. Bullet 4 is better carried by AC-58 and TC-025
than by a Behavior bullet. None of these block on its own.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Bullet 3 names no system subject ("A healthy run ... shall classify") and is compound: a ubiquitous success case and an unwanted vacuous case in one sentence. It also uses the undefined term "healthy" (see SR-1431 FND-001). Suggested form: "When the installed backend runs a V1 bundle or bounded-corpus harness whose ..., the classifier shall return `Verified`. If a V1 bundle's requires clause is satisfied by no bounded argument, then the classifier shall return `CoverUnsatisfied` ...". | spec/kani/functional/FR-015-bounded-kani-obligations.md:258-261 |
| FND-002 | low | Bullet 2 is compound, with two `shall` responses for two harness kinds (V1 bundle, corpus). Split it into two bullets, matching the one-response bullets around it. | spec/kani/functional/FR-015-bounded-kani-obligations.md:253-257 |
| FND-003 | low | Bullet 4's subject is "The crate's test suite": it states how the requirement is verified, not what the system does. The AC-58 row and TC-025 item 18 already carry it. As a Behavior bullet it should be "If a harness ... lacks a cover ..., then ..." with the generator or build as the subject, or be dropped. | spec/kani/functional/FR-015-bounded-kani-obligations.md:262-265 |

## Dispositions

Re-checked at 5a9fe3e040dd83251a17540d0fe623f69d4fadba (Behavior bullets now at FR-015 lines 260-277; ACs at 335-340).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5a9fe3e (bundle bullets restated in 1216b32, corpus bullet split in 5a9fe3e): each run outcome is now its own When- or If/then-clause, with the generator as subject (the same subject FR-017's classification bullets use), and "healthy" is replaced by the stated condition. |
| FND-002 | fixed | 1216b32: the V1 bundle and corpus cover obligations are now two separate single-response bullets. |
| FND-003 | fixed | 1216b32: the bullet's subject is now the crate ("The crate shall carry a gate that fails when ..."). That states a system obligation, not a test-suite procedure. |
