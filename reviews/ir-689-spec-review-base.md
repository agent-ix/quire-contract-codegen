---
id: SR-3100
title: "IR-689 spec review (base checklist): feature-only stage-2 O observation transport"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir689-stage2-observation (frozen head named in the Linear marker; one author commit plus an ordinary merge of main); spec/kani/functional/FR-034-caller-death-ownership.md section 'Feature-only stage-2 observation transport' (lines 1453-1549) and AC-78..AC-82 (lines 1902-1906); spec/kani/matrix/TC-049-caller-death-ownership.md (allocation table lines 183-199, step 29 lines 945-974, expected rows 1266-1274); spec/kani/matrix/tests.md FR-034 and TC-049 rows; context: FR-034 opt-in fixture section (lines 512-541), stage-2 live-birth section (lines 1262-1361), AC-51..AC-56; newer published guardian review-source backup ref (head commit 'Retain producer clock failure with borrowed outer setup custody'): Cargo.toml features, README, src/kani/run/fixture.rs, fixture_execution.rs, owned.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: references
---

## Summary

Ticket: IR-689. Base checklist review of the new feature-only O-to-C observation transport: a C-created one-way pipe whose single write right travels C to L to O, five fixed once-only O event records, a separate C-owned authenticated I Completed carrier, fixed count-sized storage and a feature-off absence Analysis.

Recorded clean:

- **The five events match the merged AC-51..AC-56 needs.** OrdinarySampleCompleted backs AC-51's release precondition, InitConfirmed backs AC-52 and AC-54, MonitorReaped backs AC-53, ReportSealed backs the seal-order predicates in AC-52/53, and GateClosed backs AC-54. AC-55/56 stay Analysis-only and are not claimed by any record.
- **Records are claims, not proofs.** FR-034:1505-1510 keeps the independent genuine-operation predicates, the unchanged-emission fabricated-result mutants and the late-operation controls. A record alone cannot turn fabricated success into a pass. TC step 29b repeats this.
- **No new authority.** The endpoint, binding envelope and records grant no startup, Dispatch, report, cancellation or cleanup authority (FR-034:1465-1466). The channel is one-way, the reader is C-private, and O is forbidden to await an ACK, pause or scheduling permission (FR-034:1535-1537). The verifier cannot use the endpoint to kill, signal, wait on or extend O.
- **Measured source claims hold.** In the guardian source, the public fixture result (fixture.rs GuardianFixtureObservation) carries only C-side lease, publication, worker and marker facts and no O operation facts. The fixture runs through the RunOwner start_sequence path, not an O-to-C observation channel. A new surface is needed, as the spec says.
- **Status honesty.** Every new row is PLANNED/UNRUN. No runtime evidence is claimed. AC-82 is Analysis-only, and method rows are stated to supply no completion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Nothing defines which ordinary tick O reports as OrdinarySampleCompleted. The table says "the selected ordinary accounting tick", and the prose says O emits each selected event at most once, but no rule or input selects the tick. AC-51 and FR-034:1278-1280 need a completed sample after the adopted orphan is positively acknowledged, so that the controller then releases the worker. Only C's fixture knows when the acknowledgement happened. The pipe is one-way O-to-C, the binding envelope carries only build, run and report identity, and no C-to-O selection input is allowed. A once-only record that O picks itself, for example the first tick after Armed, can precede the acknowledgement and make AC-51's construction impossible. Making it work needs either an undeclared selection channel or a multi-tick stream, and both contradict the five fixed once-only events. | spec/kani/functional/FR-034-caller-death-ownership.md:1495; spec/kani/functional/FR-034-caller-death-ownership.md:1501-1503; spec/kani/functional/FR-034-caller-death-ownership.md:1278-1280 |
| FND-002 | medium | The M-reap provenance predicate that the record cannot carry is not stated normatively. FR-034:1508-1510 says only that a cached Child Some, pidfd readiness or the writer's claim does not prove a new consuming wait, and that the standalone probe does not establish the Guardian signal/waiter profile. The premises under which a Some(status) is genuine reap provenance appear nowhere in FR-034 or in an AC. Those premises are: O is the exact parent, O is the sole waiter, SIGCHLD is neither ignored nor SA_NOCLDWAIT, and there is no prior consuming wait in O's history (a waitable zombie, then Some, then ECHILD). TC step 29b says only "wait/cache/SIGCHLD/no-other-waiter provenance". Two CODE authors could validate different premise sets and both claim AC-53/AC-79. | spec/kani/functional/FR-034-caller-death-ownership.md:1508-1510; spec/kani/matrix/TC-049-caller-death-ownership.md:958-959 |
| FND-003 | low | The qualification sentence paraphrases a statement that does not exist. FR-034:1466-1467 qualifies "the earlier stage-2 statement that no external observation right was then allocated". The actual earlier text, FR-034:1360-1361, reads "No DTO, hook, new right, public role field or runtime implementation is delivered by this allocation". The new slice also extends the fixture result with immutable raw records (FR-034:1475), which is a DTO change the qualification does not name. A reader comparing the two sections cannot tell which of the earlier exclusions still bind. | spec/kani/functional/FR-034-caller-death-ownership.md:1466-1467; spec/kani/functional/FR-034-caller-death-ownership.md:1360-1361; spec/kani/functional/FR-034-caller-death-ownership.md:1475 |

## Verdict

**Changes requested (high).** Define how the OrdinarySampleCompleted tick is selected, using a rule O can evaluate from its own state with no C-to-O input. Alternatively, restate the AC-51 construction so that a self-selected once-only tick provably follows the orphan acknowledgement, or allocate a bounded per-tick record explicitly. State the M-reap provenance premises normatively. Quote and qualify the actual earlier exclusion sentence, including the DTO extension.
