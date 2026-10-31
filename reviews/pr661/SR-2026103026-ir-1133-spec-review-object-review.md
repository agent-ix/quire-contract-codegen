---
id: SR-2026103026
title: spec-review/object-review of IR-1133 PR 661
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/replay/functional/FR-024-counterexample-envelope-intake.md,
  spec/replay/matrix/TC-035-counterexample-envelope-intake.md
review_set: subset
---

## Summary

Independent spec-review/object-review of IR-1133, PR #661 at 419b7c4857bd6a5aa72ade0aa559cc5ed67b29ed. Examined: FR-024-AC-52, FR-024-AC-53, TC-035-step-11.

## Verdict

**CONDITIONAL** — 1 finding(s) require author disposition.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-52 describes state-document bytes as provided under four labels, but QSL's ReplayRequestWire puts document bytes in documents: Vec<Vec<u8>>; byte_provision is a separate two-label source map. Specify document-vector supply and identity parsed from each document so CG targets the actual API. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:507 |

## Examined scope

- **FR-024-AC-52** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): PLANNED (IR-1133; replaces AC-13). The pre snapshot holds, for every declared field of the state object, the integer Kani playback bound to that field, and the post snapshot holds the integer post-state value the caller supplied for it; each has one `complete` population with one object and no defaulted field. The invocation document holds every member stated in Behavior, with `model` naming the admitted owner by identity only, `parameters` empty, `result` null, `created` and `deleted` empty, and `pre` and `post` each naming the respective snapshot by its four identity labels alone. Each snaps
- **FR-024-AC-53** (spec/replay/functional/FR-024-counterexample-envelope-intake.md): PLANNED (IR-1133; replaces AC-14). For a document vector with members in non-sorted source order, text containing a quote, backslash, control character and non-ASCII letter, and numeric members containing a large integer, a fractional number and an exponent form, the state-clause builder's provided bytes equal `core::canonical`'s bytes for the same value exactly. Reordering object members or changing JSON whitespace in an otherwise equal admitted document does not change its decoded value; changing one value does. The state-clause module uses that one encoder and names none of `quire_canonical
- **TC-035-step-11** (spec/replay/matrix/TC-035-counterexample-envelope-intake.md): 11. Read the invocation document and its pre and post snapshots and the bytes provided under
    their four identity labels. Change one playback value while retaining all labels, then
    change one post-state value while retaining all labels; read the provided bytes and decoded
    values after each change. Omit a referenced snapshot, supply it under different labels,
    and supply two documents with the same four labels, first with equal bytes and then with
    different bytes. Record the typed refusal and whether replay was called.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-52 and Behavior now direct CG to append raw bytes to ReplayRequestWire.documents; QSL reads four labels from each document, and source byte_provision remains separate. |
