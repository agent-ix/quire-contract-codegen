---
id: "SR-1116"
title: "CG PR 245 spec review: FR-014-AC-42 rewording, planned-to-covered flips, AD-002 replay seam edits"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@5d60e06c836b96a3f803b53a267326d98656b57d; spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md; quire-spec-language@6f314877 (read only); diff origin/main...HEAD, base 27153ba"
---

# SR-1116: CG PR 245 spec review: FR-014-AC-42 rewording, planned-to-covered flips, AD-002 replay seam edits

## Summary

Ticket: IR-548 (also IR-547). PR: agent-ix/quire-contract-codegen#245 at 5d60e06. Only the spec hunks are in scope here.

- **FR-014-AC-42 rewording.** The merged text required each generator to "hold no `limit_kind`". That could not be met: FR-014's own Behavior bullet requires a `limit_kind` field on `LoweringLimitUnrecognised`. The new text drops that clause and tightens the rest:
  - no `CheckedPackageLimit`;
  - no `.limit_kind` access;
  - exactly one `Failed` arm, which binds nothing and calls `classify_lowering_failure`;
  - the name `limit_kind` only in that refusal and in the conversion into it.

  The new text can be implemented, it is at least as strong in spirit as the original, and the scan test enforces it. The TC-024 step 4 rewording matches it.
- **PLANNED to Covered flips.** FR-014-AC-40..42, FR-015-AC-50, FR-018-AC-20 and FR-021-AC-23 now have the tests that SR-1115 measured. The TC-031 row correctly drops FR-021-AC-23 from its planned list.
- **AD-002 edits.** Each was checked against QSL origin/main 6f31487:
  - The three version and vocabulary members are absent from `ReplayRequestWire` (request.rs:236-263), and `package_contract_version` is absent from `WitnessPacket` (witness.rs:761-792). The R-Q5 and R-8 rows ("Done", "Met") are therefore true.
  - The slot is named `obligation_identity` and is documented as the ADR-013 O-09 digest. R-Q7 and the slot row say the function path passes a transcript digest as a placeholder until AD-003 E-1, and that is honest.
  - qsl-replay re-exports `Category` from qsl-foundation (lib.rs:94-96), as the table row says.
  - The removed open-question row about request contract spellings was resolved by QSL.

## Verdict

The spec hunks are sound. One low finding: the Current-state bullets this PR re-measured still name files that no longer exist.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR rewrote AD-002's Current-state bullets "against the qsl-replay crate CG's lock selects", but they still cite `spine_replay.rs:400`, `:125`, `:498`, `:517` and `frame_replay.rs:115`, `:47`, `:169`. Those files are now `src/replay/function.rs` and `src/replay/frame.rs`, so a reader cannot follow the refs. | spec/assurance/AD-002-cg-qsl-replay-seam.md:137-160 |

## Dispositions

Round 1, reviewed at b8cf39ddc15e33d730d1dbc1d0deb349343f0d11.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed b8cf39d | The front-matter `system:` line and the Current-state bullets now name `src/replay/function.rs`, `frame.rs` and `witness.rs`. The reviewer re-measured every cited line at b8cf39d: `call_site` at function.rs:399, `replay` at :127, `first_out_of_domain` at :501, `verdict_of` at :520 and `ReplayPackage::request` at :438; `call_site` at frame.rs:112, `reconstruct` at :182, `replay_frame` at :183, `declared_domains` at :165 and `obligation_identity` at :47; `decode_falsification` at witness.rs:116. All are correct. |
