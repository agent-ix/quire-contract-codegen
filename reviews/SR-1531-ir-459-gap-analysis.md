---
id: "SR-1531"
title: "CG PR 284 gap analysis: FR-024-AC-20 to AC-30, AC-1, TC-035 steps 18 to 27, AD-002, AD-003 E-1 and FR-029 against the minted frame identity and decoded frame witness"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@528f46bad4abf8f2dbcd46672ce54a75f9cd00b2; src/replay/frame.rs, src/replay/obligation.rs, src/replay/witness.rs, src/kani/identity.rs, src/kani/generate/frame.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/native_twin.rs, tests/it/terminal_map.rs (diff 735e704...528f46b); spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/tests.md, spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md; QSL qsl-replay/src/call_site.rs at c8f0c28"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: references
---

# SR-1531: CG PR 284 gap analysis

## Summary

Ticket: IR-459. Plan completion: not assessed. Planless, scoped to the PR diff.

- Trace tags, counted by grep of `Trace:` lines in `src` and `tests`:

  | AC | Tagged tests |
  | --- | --- |
  | FR-024-AC-20 | 3 |
  | FR-024-AC-21 | 4 |
  | FR-024-AC-22 | 4 |
  | FR-024-AC-23 | 3 |
  | FR-024-AC-24 | 2 |
  | FR-024-AC-25 | 4 |
  | FR-024-AC-26 | 2 |
  | FR-024-AC-27 | 2 |
  | FR-024-AC-28 | 1 |
  | FR-024-AC-29 | 2 |
  | FR-024-AC-30 | 1 |

  The AC-30 test is `#[ignore]`d and runs in the kani lane. FR-024-AC-1 to AC-10 have zero tags,
  and FR-024 and the matrix keep them planned. FR-024's Current state says "meets none of AC-1 to
  AC-10 in full". AC-1 needs the envelope identity on every subject. Only the function path and
  the frame path build one, and the state-clause path is still caller-supplied (Q-1), so AC-1
  staying planned is truthful.
- `quire coverage --strict` reports 44 unbacked rows at the merge base, at the head, and on the
  merge preview with main 33d27e2. The count does not move, and that is a property of the tool,
  as the author says: TC-035 already had tagged tests. The per-AC tags above are the evidence for
  AC-20 to AC-30.
- AC by AC. Each of AC-20 to AC-29 holds on the code and has a test that can fail; the mutants in
  SR-1530 confirm this.
  - AC-22 and AC-25 to AC-28: the "not called" refusals are shown with an unlocatable operation,
    which would otherwise be a `CallSite` refusal. That oracle is sound.
  - AC-21's stated edge was measured: a clause on `deposit` changes `transfer`'s identity.
  - AC-23: a record without `state_fields` is refused, and so is a record with an unknown member.
  - AC-24: the twin passes no identity. The minted value equals a hand-written SHA-256 of the
    site's preimage, independent of CG's encoder.
  - AC-29: the header pin test passes, and `declared_domains` is `Some(Vec::new())`.
- AC-30 holds only in the form noted in FND-001.
- TC-035 steps 18 to 27 map one-to-one to these tests.
- AD-002 R-6 and AD-003 E-1 now say the frame path is met, and E-1 still lists the V1 and scalar
  paths and the state-clause Q-1 as open. That is truthful. AD-003 E-1 is not widened: frame
  arguments are empty.
- FR-029. The seven new `FrameReplayError` variants map to `CgDefect`, which reads as `Failed`,
  and `terminal_map.rs` asserts each one through `for_each_cg_failure`. That follows FR-029's rule
  that this repository raised them and they carry no QSL code.
  - Decode matches FR-029's existing "decode failure" clause.
  - OutOfDomain matches the state-clause AC-16 reading: "a value outside the proof bound are CG
    defects".
  - NotAFrame, FieldSetMismatch, ScopeMismatch, Identity and a PreState `Differs` are
    inconsistencies inside the driver's own harness, record and run.
  - The document-missing PreState faults are a judgement call; they are recorded in SR-1532
    FND-002.

## Verdict

CHANGES REQUESTED, light. AC-20 to AC-29 are implemented and backed by tests that can fail. The
statuses are truthful, AC-1 to AC-10 stay planned, and the strict count is unchanged. One medium
gap remains. Every positive frame-replay test, including the real-Kani AC-30 test, rebases the
generated harness's `scope.anchor` and `scope.frame` onto the ids QSL's `call_site` names
(`Twin::aligned`). So no test shows that a harness CG generates carries the node ids the replay
then requires. The fix can be either of two: put the rebase into AC-30's text and record a
deferral ticket for an end-to-end alignment test, or give the twin's generator the checked package
QSL emits.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-30 says the caller supplies "the harness's `StateFrameIdentity`". The test supplies `twin.aligned(..)`, which has `scope.anchor` and `scope.frame` replaced by the ids `call_site` names. No test shows that a generated harness already carries those ids; FR-024 and TC-035 only assert it in prose. If the generator's ids differed, every production frame replay would be `ScopeMismatch(Anchor)`, read as `Failed`, and no test would notice. | tests/it/kani_obligations_state_frame.rs:3070-3073; tests/state_frame_support/native_twin.rs:401-416 (Twin::aligned); spec/replay/functional/FR-024-counterexample-envelope-intake.md:341-344 (Current state) |

## Dispositions

Round 1, reviewed at 8f7db6cbc47dc8738d76d8035849ffbf126043ef.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | The deferral rests on a measurement that is wrong. FR-024's new "Open, unchecked" item says `call_site` "returns the site's ids and the package id only" and that qsl-replay exports nothing that returns the emitted checked package. In fact qsl-replay `CallSite` has `pub package: Vec<u8>`, the compiled package's `quire.checked-package/v2` bytes, at both c8f0c28 and bcca433 (`call_site.rs:144-147`). Probe on 8f7db6c, reverted after: `call_site` for the twin's `deposit` returned 50436 bytes holding the anchor and frame wire ids. `CheckedPackageV2::read` with the twin's domain document under `insert_domain_package_document` admitted it (`package_id` e6ea7972…). The fixture's anchor id is absent from those bytes. So the end-to-end test is in reach with CG's existing dependencies: generate the frame harness from that admitted package and assert its scope equals the site, with no `Twin::aligned`. Either that test, or a measured reason why generating from it fails, plus a corrected FR-024 sentence, is still owed. The AC-30 and TC-035 rebase text is accurate. |

Round 2, reviewed at 0ae79465cab8987525a4ae89ed10c888d70ce81e (fix 0ae7946 on 4191ed8, whose tree equals the clean merge of 8f7db6c and main 4502a21). 0ae7946 touches only FR-024, TC-035, `tests/it/kani_obligations_state_frame.rs` and `tests/state_frame_support/native_twin.rs`, so no `src` changes. `make ci` was re-run with my own target dir and the whole log read (`ci-cg284-disp2-0ae7946.log`): lib 173, it 389 with 25 ignored, doctest 1, under msrv and stable, with no make error. `quire coverage --strict` reports 44.

I re-measured the claims myself with a temporary probe, reverted afterwards:
- The object type node QSL emits for the twin has body `{"members":[],"term":"aggregate"}`.
- The package's single `bounded_domain/integer_range` node (0..1000) is referenced only from `expression/query` and `expression/pre_read` nodes, as the type of field reads, and never from the object body.
- This is QSL's design, not an accident. `qsl-semantics` `check/lowering/model.rs` `model_node_content` gives every model declaration node "no semantic type, a `null` `declaration` and an empty body". IR FR-040 ("Model forms") says a field is named by its declaring node and resolved through FR-038's model-owned member resolution against the selected domain package.
- FR-015-AC-27 itself specifies reading "the integer range that the framed object's body member of the same name declares". So the inability to generate from an emitted package is a gap in the generator's own spec that predates this PR, and IR-624 (Backlog) is filed for it.
- The new tests assert what they claim and are correct, including the block the coordinator flagged as written with a heredoc. The emitted package's single `operation_anchor` and `frame` nodes equal `call_site`'s `site.anchor` and `site.frame`. The anchor's `frame` member binds that frame. The fixture's anchor id differs. Generating from the emitted package refuses `BoundNotResolved`/`MemberAbsent`.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0ae7946: `tc_035_the_node_ids_of_the_package_qsl_emits_are_the_ids_call_site_names` shows that the package QSL emits carries exactly the anchor and frame ids `call_site` names, read through CG's own model reader with no rebase. The generator takes `scope.anchor` and `scope.frame` from the package's node ids, so the production risk this finding named, a generated harness failing with `ScopeMismatch(Anchor)`, is refuted by measurement. The AC-30, TC-035 and FR-024 text is now true. The remaining gap, that no harness can be generated from an emitted package because of the FR-015-AC-27 body-member reader, predates this PR, sits outside AC-20 to AC-30, and is tracked as IR-624. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-024's Current state frame-path bullet still ends "a harness generated from QSL's own emitted package already carries them". The next bullet says no harness can yet be generated from that package. The sentence should say the emitted package carries the ids. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:357-359 |
| FND-003 | low | `tc_035_the_generator_reads_no_field_range_from_the_object_shape_qsl_emits` is tagged `FR-015-AC-27, TC-035`, but FR-015-AC-27's verification is TC-025. The tag pairs the AC with the wrong test case. | tests/it/kani_obligations_state_frame.rs:2984-2992 |
