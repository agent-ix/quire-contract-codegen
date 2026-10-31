---
id: SR-2026103025
title: spec-review/integrity of IR-1133 PR 661
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md,
  spec/replay/matrix/TC-035-counterexample-envelope-intake.md
review_set: subset
---

## Summary

Independent spec-review/integrity of IR-1133, PR #661 at 419b7c4857bd6a5aa72ade0aa559cc5ed67b29ed. Examined: TC-035-matrix, AD-002-seam, FR-024-AC-13, FR-024-AC-14, FR-024-AC-52, FR-024-AC-53.

## Verdict

**CONDITIONAL** — 2 finding(s) require author disposition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The replay test matrix still credits retired AC-13/14 as covered and omits replacement AC-52/53; its row contradicts the changed FR and TC-035 status. | spec/replay/matrix/tests.md:25,47 |
| FND-002 | low | AD-002 still cites retired AC-13 as the live state-document seam; point it at replacement AC-52 so the architecture trace follows the active criterion. | spec/assurance/AD-002-cg-qsl-replay-seam.md:55 |

## Examined scope

- **TC-035-matrix** (spec/replay/matrix/tests.md): | FR-024 | FR-024-AC-7, FR-024-AC-8, FR-024-AC-47 through FR-024-AC-51 | TC-035 | 🚧 Planned; these criteria still lack tests of their own |
- **AD-002-seam** (spec/assurance/AD-002-cg-qsl-replay-seam.md): | `quire.state.invocation/v1` and `quire.state.snapshot/v1` documents (pre from the Kani playback, post from the caller's native run of the subject), in the request's byte provision (FR-024-AC-13, IR-460) | CG to QSL | QSL grammar (FR-106) | CG builds them from its own inputs and encodes them only through `core::canonical`; QSL admits their typed values |
- **FR-024-AC-13** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): RETIRED, replaced by FR-024-AC-52; the former document-digest address and changed-digest checks are obsolete and earn no implementation credit.
- **FR-024-AC-14** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): RETIRED, replaced by FR-024-AC-53; the former document-digest equality check is obsolete and earns no implementation credit.
- **FR-024-AC-52** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): PLANNED (IR-1133; replaces AC-13). The pre snapshot holds, for every declared field of the state object, the integer Kani playback bound to that field, and the post snapshot holds the integer post-state value the caller supplied for it; each has one `complete` population with one object and no defaulted field. The invocation document holds every member stated in Behavior, with `model` naming the admitted owner by identity only, `parameters` empty, `result` null, `created` and `deleted` empty, and `pre` and `post` each naming the respective snapshot by its four identity labels alone. Each snaps
- **FR-024-AC-53** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): PLANNED (IR-1133; replaces AC-14). For a document vector with members in non-sorted source order, text containing a quote, backslash, control character and non-ASCII letter, and numeric members containing a large integer, a fractional number and an exponent form, the state-clause builder's provided bytes equal `core::canonical`'s bytes for the same value exactly. Reordering object members or changing JSON whitespace in an otherwise equal admitted document does not change its decoded value; changing one value does. The state-clause module uses that one encoder and names none of `quire_canonical

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The matrix lists AC-52/53 as Planned, removes AC-13/14 from Covered and TC-035 trace, and retains the unchanged active ACs. |
| FND-002 | fixed | AD-002 now cites AC-52/53 as planned and names ReplayRequestWire.documents and its four-label self-declaration. |
