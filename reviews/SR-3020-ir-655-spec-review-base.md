---
id: "SR-3020"
title: "CG IR-655 spec review (base): stage-2 live-birth and producer-operation oracles"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@spec/ir-655-stage2-temporal-oracle (frozen head, no PR yet; reviewed revision recorded in the IR-655 Linear marker only, per this repository's no-SHA rule); spec/kani/functional/FR-034-caller-death-ownership.md (new section Stage-2 live-birth and producer-operation observations, FR-034-AC-51..54, Dependencies staging paragraph), spec/kani/matrix/TC-049-caller-death-ownership.md (evidence delivery allocation, slice rows AC-8/10/22, AC-32, AC-34, AC-51..54, retirement paragraph, step 28, Stage-2 replacement expectations), spec/kani/matrix/tests.md (FR-034 and TC-049 rows); context: src/kani/run/namespace.rs old unit tests on CG main"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

# SR-3020: CG IR-655 spec review (base)

## Summary

Ticket: IR-655. Branch spec/ir-655-stage2-temporal-oracle, one commit over CG main, zero behind,
three Markdown files (+144/-13). Spec-only; no cargo, tests or Kani were run.

The PR implements the planner's Option A in outline. FR-034 AC-8 and AC-10 are byte-unchanged. The
first 50 AC rows and procedure steps 1-27 are unchanged. The new criteria FR-034-AC-51..54 are
PLANNED/UNRUN and their ids are not used anywhere else in the repo. No SHA, local path or conflict
marker was added. The computed matrix goes from 588 to 592 criteria: the 588 prior rows are
byte-identical and the four new rows compute untagged. `quire matrix --strict` exits 1 at both
main and head, so it is no worse.

This base review attacked each new criterion for vacuity: what is the cheapest implementation or
mutant that passes while breaking the intent? Four attacks succeed. The replacement temporal oracle
(AC-51) no longer catches the mutant the retired test existed to catch. The AC-52 mutant recipe
fails its own check whether or not the record is really bound to the operation. AC-52 and AC-53
have no mutant that tests their "strictly before seal" half. AC-52 confirms that I is dead, not
that O killed it. A fifth finding: the schedule claim behind the retirement is stated as fact, but
nothing requires it and it depends on source that is not published.

`quire validate --scope . <3 files>` (quire 0.36.1, engine 0.50.1) exits 0. It prints seven
module-loader notices: semantic.inline-data-schema, DuplicateArchetype x5 and
DuplicateInverseEdge. These come from the installed modules, not from the documents, but the run
is not notice-free.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-51 cannot catch a sampled-PID teardown. It keeps every later ordinary O tick, so a later tick usually samples the released child before cleanup. A mutant that kills only sampled PIDs (for example a non-INIT watcher plus a kill list) then kills both pins and passes. The retired test deliberately left the fork unsampled (namespace.rs:629). This removes the only witness for AC-8's absent-from-previous-samples clause and for the Expected Results "Kill only sampled PIDs" regression (TC-049:1020). | spec/kani/functional/FR-034-caller-death-ownership.md:1583; spec/kani/functional/FR-034-caller-death-ownership.md:1105-1124; spec/kani/matrix/TC-049-caller-death-ownership.md:861-867; spec/kani/matrix/TC-049-caller-death-ownership.md:1020; src/kani/run/namespace.rs:629 |
| FND-002 | high | The AC-52 mutant recipe also deletes the record: "Replace ONLY that operation with fabricated true; do not emit its producer record". The patch alone guarantees the predicate fails, so it does not test the operation-to-record binding. A token-minted record emitted at the settle call site would still see its mutant fail. AC-53's "omit ONLY that wait/reap operation" does not say to leave the record code intact either. | spec/kani/matrix/TC-049-caller-death-ownership.md:869-871; spec/kani/matrix/TC-049-caller-death-ownership.md:874-876; spec/kani/functional/FR-034-caller-death-ownership.md:1141-1143; spec/kani/functional/FR-034-caller-death-ownership.md:1584-1585 |
| FND-003 | high | AC-52 and AC-53 require each record "strictly before" the matching seal, but only omission mutants are allocated. A presence-only predicate passes every listed check. A schedule that reaps M or polls I after the seal is never exercised: a zombie M has closed its writers, so EOF and the seal still happen before a late reap. | spec/kani/functional/FR-034-caller-death-ownership.md:1584-1585; spec/kani/functional/FR-034-caller-death-ownership.md:1137-1141; spec/kani/matrix/TC-049-caller-death-ownership.md:868-876; spec/kani/matrix/TC-049-caller-death-ownership.md:1109-1110 |
| FND-004 | medium | On the post-Dispatch path AC-52 confirms that I is dead, not that O cancelled it. If O never sends its owned kill to I but still polls, an I that exits by itself still gives a real IN record before the seal. The kernel then reaps the descendants, so AC-51, AC-52 and AC-53 all pass. Only AC-54, where the gate is retained and I cannot exit by itself, witnesses kill authority. | spec/kani/functional/FR-034-caller-death-ownership.md:1584; spec/kani/functional/FR-034-caller-death-ownership.md:1137-1139; spec/kani/matrix/TC-049-caller-death-ownership.md:868-872 |
| FND-005 | high | The retirement rests on a sentence stated as fact: "The ordinary final report sample follows actual claimed I termination, retained M Child reap, writer EOF and sealing". No "shall", no AC and no Analysis receipt binds it. The guardian schedule (InnerSettlement, Commit, TerminalSampling) is not in CG main. AC-32 ("every original observer tick") allows a last tick between Completed and I termination. TC-049:58-60 requires this SPEC's source grounding to establish the schedule, but FR-034:1159-1160 defers that grounding until before fixture CODE. | spec/kani/functional/FR-034-caller-death-ownership.md:1105-1111; spec/kani/functional/FR-034-caller-death-ownership.md:1159-1160; spec/kani/matrix/TC-049-caller-death-ownership.md:58-60; spec/kani/matrix/TC-049-caller-death-ownership.md:150-151 |

### Failure scenarios and suggested repair

- FND-001. The fixture runs: sample k completes, the child is released, born and pinned. Ordinary
  tick k+1 completes and lists the child, then cleanup starts. Mutant teardown kills sampled PIDs
  and the watcher, then reports I done. Both pins are dead, Completed is genuine, and AC-51 passes.
  The old oracle's point, a descendant absent from all previous samples, was never exercised. The
  author's argument covers the window after the final report sample. The hole is the window
  between the last sample before cleanup and the start of teardown. Repair: require producer-bound
  evidence that the released child is in no completed sample's membership before cleanup starts.
  This is a precondition alongside the positive birth acknowledgement, not a proxy for it. A run
  that fails it is a typed fixture failure, never a pass. Also name a sampled-membership teardown
  mutant that AC-51 must fail. Sampling is not suppressed.
- FND-002. Repair: the mutant changes only the kernel call or its result and leaves the record
  code byte-unchanged. The record must then vanish through the producer binding alone. Say the
  same for AC-53.
- FND-003. Repair: add an I-poll-after-seal mutant and an M-reap-after-seal mutant. Each must
  fail its named ordering predicate before outer escalation.
- FND-004. Repair: either call AC-52 a confirmation witness and drop the authority claim, or add a
  producer-bound record of O's actual owned kill to I, ordered before the IN record, with an
  omitted-kill mutant in a scenario where I cannot exit by itself.
- FND-005. Repair: state the schedule as a normative shall-requirement with an Analysis criterion
  over the published source. Otherwise, make the retirement conditional on that grounding and say
  that a final sample before I teardown voids it.

## Verdict

Not merge-ready. The retirement is explicit, and the product guarantees AC-8 and AC-10 are
unchanged. But the replacement temporal oracle (AC-51) can be passed by the mutant the retired
oracle existed to kill. The new operation witnesses (AC-52, AC-53) have mutant recipes or ordering
gaps that let a weak implementation pass. Clean units: AC-54's ordering oracle and mutants (gate
retained, early close, omitted I confirmation); id allocation; byte-unchanged AC-1..50 and steps
1-27; PLANNED/UNRUN status rows; the matrix delta (+4, all prior rows unchanged, strict no worse);
no SHAs, local paths or conflict markers.

## Addendum round 1: published guardian source

The coordinator reported that the guardian source is now published. I read it read-only at the
CG published guardian review-source backup ref (older), commit subject "Retain original outer input on
pre-arm split failure". No hex is recorded here; it is in the Linear marker. This changes the
basis for FND-005 and shows the real lifecycle of I. All citations below are in
`src/kani/run/` at that ref.

### What the source shows about the schedule

The schedule claim is TRUE in the published source.

- `poll_settled` (namespace.rs:355-385) mints `InnerSettlement` only when two things hold:
  - I is terminated: the retained I pidfd polls IN (`init_terminated`, namespace.rs:364 and
    843-857).
  - M is reaped: the retained M `Child::try_wait` returns `Some` (namespace.rs:365-377).
- `InnerSettlement` carries the report identity only (namespace.rs:487-499).
- The O state machine runs `AwaitInnerSettlement`, then `AwaitReportEof`, then the seal
  (outer_sampling.rs:1400-1443).
  - It polls settlement only in `AwaitInnerSettlement` (1413-1423).
  - It requires report EOF (1424).
  - `seal_after_inner_settlement` consumes the matching settlement token, takes a complete
    pre-seal tick, then seals (3087-3114).
- The final complete observation is the `observation_and_peaks` sample taken in the same
  `advance` call that sends the successful `Commit`. That comes after descriptor delivery and the
  read ACK (outer_sampling.rs:1959-1960, 2021-2129). The doc comment says so: "Only the sample
  attached to the eventual successful complete commit is final" (1640-1641). A zero-progress
  attempt resamples (2112-2122).
- Ordinary samples continue throughout:
  - while waiting for the completed close (1704)
  - in the `AwaitInnerSettlement` and `AwaitReportEof` states (1409)
  - the pre-seal tick (3101)
  - every terminal-delivery step (1960)

So no claimed-I descendant can be alive at the final sample.

The real lifecycle of I matters for the attacks below. After it sends `Completed`, I does NOT exit
by itself. It stays INIT and keeps its descendants until lease EOF (guardian.rs:1156-1158 and
1176-1177). On the normal post-Dispatch path O sends no kill to I. Termination comes from C's
lease close, which is the AC-24 oracle, and O only confirms it. The direct `cleanup` path sends
SIGKILL to INIT through its pidfd and then confirms (namespace.rs:992-1041). That is the path the
old late-fork test drives (namespace.rs:1416-1540, "The later fork is deliberately not sampled" at
1468).

### Revised assessment of the original findings

The rows in `## Findings` above are unchanged. These are the revised severities.

| FND | Revised severity | Basis |
| --- | --- | --- |
| FND-001 | medium | Production teardown is INIT death only. No source path kills sampled PIDs. The live implementation therefore does not exploit the gap. AC-51 still cannot detect a regression to sampled-membership cleanup, and AC-8's absent-from-previous-samples clause still loses its dedicated witness. Still open. |
| FND-002 | high (unchanged) | `poll_settled` performs both operations and publishes ONE combined token (namespace.rs:364-384). A record emitted at the token (382) is exactly the token-minted record the spec forbids. The mutant recipe ("do not emit its producer record") cannot tell that apart from records at the real operations (364, 370). Still open. |
| FND-003 | medium | The source orders operation, then EOF, then seal by type: the seal consumes the settlement token (outer_sampling.rs:1431-1436 and 3087-3098). A reorder therefore needs a structural change, which lowers the practical risk. The spec still allocates no reorder mutant for the "strictly before" predicate. Still open. |
| FND-004 | low | The premise is superseded by FND-006. On the real path I's termination authority is C's lease EOF, which AC-24 covers. AC-52 only needs to confirm termination. What remains is wording: AC-52 should say it is a confirmation witness. |
| FND-005 | medium | The claim is TRUE in the published source, but it is still not normative. It has no shall, no AC and no Analysis receipt, and the spec neither cites nor binds the source schedule. The source is a review backup ref, not CG main. A later CODE change, such as a final tick before I teardown, would void the retirement and nothing would fail. Still open. |

## New findings (addendum round 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | AC-52's adverse case ("fails ... despite automatic I exit") and TC-049 step 28b rest on a premise the source contradicts. I does not exit by itself after Completed; it stays INIT until lease EOF (guardian.rs:1156-1158, 1176-1177). The case that matters is a fabricated IN while I is still alive. That would mint `InnerSettlement` early and allow a seal while claimed descendants live, but neither the AC nor step 28b names that case. The adverse case should be restated against the real lifecycle: a fabricated readiness while I is live must fail before any seal. | spec/kani/functional/FR-034-caller-death-ownership.md:1584; spec/kani/matrix/TC-049-caller-death-ownership.md:868-872 |

### Revised verdict (addendum round 1)

Still not merge-ready. FND-002 remains high, and SR-3022 FND-001, the outer-confirm deletion,
remains high. FND-001, FND-003 and FND-005 drop to medium; FND-004 drops to low; FND-006 is new
at medium.

## Dispositions

Round 1. Branch head "Strengthen stage-two mutation and temporal predicates"; its revision is
recorded in the Linear marker only. Each finding was checked against the actual text.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | New FR-034-AC-55 (FR-034:1609) and FR-034:1127-1139: producer-bound order showing the child in NO completed pre-cleanup sample; typed failure when the precondition fails; a sampled-membership-only teardown control that must fail the death predicate without lease-EOF or outer-kill rescue. TC-049 step 28e (881-888). The residual boundary gap is new FND-007. |
| FND-002 | fixed | FR-034:1150-1160 and step 28b/28c: each mutation changes ONLY the underlying call or result; emission and source-check code stay byte-unchanged; a record minted from fabricated success must fail the genuine-operation predicate; deleting the record does not count. The independent witness capability is an explicit PLANNED CODE gate. |
| FND-003 | fixed | FR-034:1157-1158 and steps 28b/28c: moving the real I poll, or the real M reap, after the seal must fail strict order even when the record is present. |
| FND-004 | fixed | FR-034:1142-1144: I-IN is a confirmation witness; termination authority remains the original C lease EOF under AC-24. No O-kill premise. |
| FND-005 | fixed | FR-034:1106-1111 now says "O shall complete the final accounting observation only after" I confirmation, M reap, writer EOF and seal; an earlier final sample invalidates the retirement. New FR-034-AC-56 (Analysis). The CG CODE author supplies it and the independent CODE reviewer checks it (TC-049:123, step 28f). This matches the published source (addendum round 1). |
| FND-006 | fixed | FR-034:1152-1155, AC-52 and step 28b: the I adverse case fabricates IN while retained I is positively live; an early InnerSettlement or seal must not be accepted while claimed descendants live; Completed is never I exit. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | high | AC-55 can still be passed by a sampled-membership teardown that uses its latest samples. "Cleanup starts" is not defined, and O keeps taking complete samples after the C lease close: in receive_completed_close (outer_sampling.rs:1704) and while awaiting inner settlement and report EOF (1409), each a complete tree observation (observe_live, 2223-2256, about every 20 ms). A realistic membership killer acting at settlement time has by then sampled the child and kills it, so the control passes. The text assumes an "omitted child". Define the boundary as the actual teardown-initiating operation, bound by the producer. Require the child to be in no sample completed before the control's kill. Have the control use every sample completed up to that kill. If this cannot be done without pausing sampling, say so and allocate the mutant class to source Analysis (no PID-list kill path; INIT-death only) together with AC-24's non-INIT-watcher mutant, rather than leaving a gate that is vacuous or can never be met. | spec/kani/functional/FR-034-caller-death-ownership.md:1127-1139; spec/kani/functional/FR-034-caller-death-ownership.md:1609; spec/kani/matrix/TC-049-caller-death-ownership.md:881-888 |

### Verdict (disposition pass 1)

All six earlier findings are fixed in the text. FND-007 (high) is new and open, so this artifact
is not yet merge-ready.

## Dispositions (round 2)

Round 2. Branch head "Allocate sampled-membership regression to honest source analysis"; its
revision is recorded in the Linear marker only. Earlier rows above are unchanged.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-007 | fixed | AC-55 (FR-034:1620) is now Analysis-only. It requires exhaustive final-source enumeration of the contained teardown authority paths (FR-034:1135-1149, step 28e at TC-049:881-892): a membership-dependent kill path fails the Analysis, and an earlier snapshot does not complete it. The prose says lease close is not a last-sample boundary and no child-absence-through-kill window is claimed (1127-1133). The AC-24 non-INIT-watcher runtime mutant stays separately owed with no source-only credit. Checked against both published guardian refs: the terminal-schedule and teardown files (namespace.rs, outer_sampling.rs, guardian.rs, memory.rs) are unchanged between them. Every kill site acts on a retained pin, Child or group, never on sampled membership (namespace.rs:432, 1006, 1016; launcher_owner.rs:227, 236; spawner.rs:380; owned.rs:96, 147). The memory children reads serve accounting only. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | The replacement oracle's runtime death predicate ("retain BOTH live then dead pins ... before outer escalation", FR-034:1132-1133; step 28e at TC-049:887-888) now sits under AC-55, which is Analysis-only, while AC-51 (Test) covers birth only. No Test-method AC-51..56 row owns that assertion. It stays owed only through the mixed AC-8/10/22 slice row, so a coder could mark AC-51 complete without it. Move it to a Test-method row, or state in AC-51 or the AC-8 slice row that it is the owning Test assertion. | spec/kani/functional/FR-034-caller-death-ownership.md:1132-1133; spec/kani/matrix/TC-049-caller-death-ownership.md:887-888; spec/kani/functional/FR-034-caller-death-ownership.md:1616 |

### Verdict (disposition pass 2)

All findings FND-001..007 are fixed. FND-008 is low and new.

One mutant class has no runtime row, and the spec says so honestly. Take a mutant that both breaks
INIT authority (a non-INIT watcher) and adds a sampled-membership kill at settlement. The AC-24
non-INIT-watcher runtime Test may not catch it, because the acknowledged worker is likely sampled
and killed. Only the AC-55 final-source Analysis catches it, and it does so reliably because the
membership kill path is visible in the source. The spec makes no runtime claim for it ("no
sampled-membership runtime control is claimed"), so this is a disclosed limit, not a defect.
