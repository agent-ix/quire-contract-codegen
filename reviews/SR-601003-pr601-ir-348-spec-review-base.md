---
id: SR-601003
title: "IR-348 PR #601 spec-review/base (FR-016, TC-026, replay matrix, interface-001)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #601 head 64992f5be8aebb7d036fba5491fa9ef61af70d29 against merge-base 46dbaccacbd96f55654fc4b331a9d42883e70a2c; spec/core/functional/interface-001-codegen-api.md, spec/replay/functional/FR-016-witness-native-replay.md, spec/replay/matrix/TC-026-witness-native-replay.md, spec/replay/matrix/tests.md; IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
---

## Summary

Ticket: IR-348. This review covers the four spec files the PR changes:

- **interface-001:** the `generate_kani_bundle` operation and its row in the operation table are
  removed. No compatibility entry is left behind.
- **FR-016 and TC-026:** new Status text and the replay matrix split AC-9 and AC-10 out as
  "Partially backed, Open under IR-1047".

The AC text and verification methods are unchanged, and no PASS is claimed for the
installed-Kani E7-E9 control. The FR-016 Status text, however, still describes the PR's own
deleted tests as present.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-016 Status describes files and tests deleted by this same PR in the present tense. It says `skeleton_spine.rs`'s installed E7-E9 control "uses the retired V1 `BoundClause` generator". It says `kani_argument_order.rs` and `kani_witness_join.rs` "use the retiring V1 `BoundClause` producer". It says `tc_026_installed_kani_falsification_is_bound_to_process_stdout` "uses a V1 `BoundClause` harness". All of them are gone at this head. TC-026 (lines 165-173) and the replay matrix AC-8 row repeat the same claims. | spec/replay/functional/FR-016-witness-native-replay.md:163-185; spec/replay/matrix/TC-026-witness-native-replay.md:165-173; spec/replay/matrix/tests.md:15 |

## Verdict

Changes requested (medium). Rewrite the three FR-016 Status paragraphs, the matching TC-026
paragraphs and the AC-8 matrix row to say the files were removed by IR-348. Keep the Open status
and the follow-up owners (IR-1047, IR-604) unchanged. The substance of the change is correct:

- AC-9 and AC-10 stay Open and partially backed, and their native replay evidence is kept.
- The fixture is labelled test-only and is not a generator claim.
- The V2 control remains owned by IR-1047 and is not counted as a substitute.
- The interface-001 removal is complete.

## Coverage

```yaml
scope:
- {id: FR-016-AC-9, path: spec/replay/functional/FR-016-witness-native-replay.md, role: examined, excerpt: "FR-016-AC-9 and AC-10 retain their default native replay tests, but the skeleton_spine.rs installed-prover E7-E9 control uses the retired V1 BoundClause generator. That installed path is Open and partially backed until IR-1047 supplies a genuine V2 generated harness and real-Kani counterexample control."}
- {id: FR-016-AC-10, path: spec/replay/matrix/tests.md, role: examined, excerpt: "FR-016 | FR-016-AC-9, FR-016-AC-10 | TC-026 | Partially backed, Open under IR-1047: default native replay tests survive; the V1 skeleton_spine.rs installed-prover E7-E9 control retires"}
- {id: FR-016-AC-21, path: spec/replay/matrix/TC-026-witness-native-replay.md, role: examined, excerpt: "FR-016-AC-21 to AC-23 are implemented and tested in the default suite (tests/it/skeleton_spine.rs with a direct test-only identity fixture"}
- {id: FR-016-AC-8, path: spec/replay/functional/FR-016-witness-native-replay.md, role: examined, excerpt: "The only emission-order and real-Kani persisted-schema controls in tests/it/kani_argument_order.rs and tests/it/kani_witness_join.rs use the retiring V1 BoundClause producer."}
- {id: FR-016-AC-25, path: spec/replay/functional/FR-016-witness-native-replay.md, role: examined, excerpt: "In tests/it/kani_stdout_capture.rs, the ignored tc_026_installed_kani_falsification_is_bound_to_process_stdout uses a V1 BoundClause harness."}
- {id: FR-016-AC-1, path: spec/replay/matrix/tests.md, role: examined, excerpt: "FR-016-AC-1 through FR-016-AC-5, FR-016-AC-11 through FR-016-AC-23 | TC-026 | Covered; AC-1 through AC-5 and AC-21 through AC-23 use a direct test-only KaniObligationIdentity fixture"}
- {id: interface-001:generate_kani_bundle, path: spec/core/functional/interface-001-codegen-api.md, role: examined, excerpt: "generate_kani_bundle operation entry and table row removed"}
```

## Dispositions

Re-checked at exact head 5380318cb380788ba0512e8765d2faa651078965.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-016 Status now says the skeleton_spine E7-E9 control "was removed with the V1 BoundClause generator", the AC-8 controls "used the retired V1 BoundClause producer and were removed", and "The removed tc_026_installed_kani_falsification_is_bound_to_process_stdout used a V1 BoundClause harness". TC-026:165-173 and the replay matrix AC-8 row use the same past tense. Open status, IR-1047 and IR-604 ownership, and the AC text are unchanged. |
