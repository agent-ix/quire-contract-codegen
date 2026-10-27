---
id: SR-593003
title: "IR-348 PR #593 spec-review/base review (AD-004 L-6 inventory)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #593 head 3ee11b66a9ed8ec6abd3d59b1d154a06e993844d on merge-base 68fafff7 (origin/main 8db3554e); spec/assurance/AD-004-cg-crate-layout.md (Witness decode row, L-6, Interim exceptions); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

## Summary

Ticket: IR-348. This base spec-review covers the three AD-004 edits in the diff:

- The "Witness decode" row of the Kani output placement table.
- The L-6 status paragraph.
- Removal of two Interim-exceptions rows: the V2 clause witness entry and `parse_scalar_playback`.

`quire validate --scope . spec/assurance/AD-004-cg-crate-layout.md` passes at head, with module
registry warnings only. Only the sub-analysis whose scope matches the diff was run. The diff edits
an inventory and adds no requirement statement, relationship edge or domain object, so EARS,
dependency and object review do not apply.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The three edits agree with each other and with the code at head:

- The Witness decode row names `function`, `state_clause` and `frame` as the remaining
  transcript-taking entries.
- The L-6 paragraph names the same three files.
- The Interim-exceptions table keeps exactly those three replay rows.

Each removed row's "Removed by" condition ("pass typed `Playback`") is met in the code. L-6 is
still stated as partial, so the slice does not overclaim. The FR-032 precedence evidence gap is
recorded under SR-593002 FND-001, not here, because FR-032 text is not changed by this PR.

## Coverage

```yaml
scope:
- id: AD-004:witness-decode-row
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "| Witness decode | `replay/witness.rs` | `decode_falsification` and the V2 clause entry take typed playback entries from `kani/output/playback.rs`; public replay entries in `function`, `state_clause` and `frame` still take transcript text and call the output parser before decoding (L-6 residual below) |"
- id: AD-004:L-6
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "Public or replay-facing entries in `replay/function.rs`, `replay/state_clause.rs` and `replay/frame.rs` still accept transcript text and call the output parser before decoding. L-6 remains partial until each entry takes parsed playback at its boundary, as itemized under Interim exceptions."
- id: AD-004:interim-exceptions
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "Rows removed: the public V2 clause witness entry (`decode_v2_clause_falsification`) and the scalar replay entry (`parse_scalar_playback`); remaining: function, state-clause and frame replay rows."
```
