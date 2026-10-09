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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The new M-reap provenance predicate gives the verifier operations it cannot perform. Only M's parent, O, can call waitid on M. A nonreaping waitid(WNOWAIT) "before that consuming Child wait" and the "same-child independent query ... ECHILD after the actual Some" must therefore run inside O's reap stage. From any non-parent, ECHILD is always returned, so a verifier-side query is vacuous. The text assigns these to "the verifier" and never allocates O-side probe calls. Those calls are not covered by the only allowed feature-on change, the "nonblocking observation sink" at named boundaries (FR-034:1460-1465). An implementer must either add unallocated O-stage operations or run a vacuous non-parent check, and AC-86 accepts both. | spec/kani/functional/FR-034-caller-death-ownership.md:1536-1546; spec/kani/functional/FR-034-caller-death-ownership.md:1460-1465; spec/kani/matrix/TC-049-caller-death-ownership.md:966 |

## Dispositions

Round 1 was re-checked at the branch's round-1 fix head: four commits ahead of main, after the review-export commit, with subject 'Resolve IR689 observation selection and independent evidence obligations'. The head is named in the Linear marker only. It was checked against the newer published guardian review-source backup ref, whose head commit is 'Retain producer clock failure with borrowed outer setup custody'. The check was static and read-only. make spec passes, and no build, test, Kani or replay was run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': C keeps the writer until authenticated Armed and, for birth, until the genuine adopted-orphan ACK. It then sends one write right directly to O in the binding envelope. O selects the NEXT completed ordinary tick after accepting the binding, never a startup or pre-binding tick, and preserves every due tick (FR-034:1467-1475, 1496-1505; AC-83, AC-92). No C-to-O ACK, pause or synthetic sample is allowed. |
| FND-002 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': exact parent, sole waiter, SIGCHLD neither ignored nor SA_NOCLDWAIT, no prior consuming wait, a FIRST uncached Some, and the distinctions between cache, competing waiter and zombie are now normative (FR-034:1536-1546) and owned by AC-86. The actor feasibility of the waitid probes is raised separately as FND-004. |
| FND-003 | fixed | fix commit 'Resolve IR689 observation selection and independent evidence obligations': FR-034:1456-1459 quotes the actual earlier exclusion verbatim and names the DTO/result extension and the single write right it qualifies. |

Round 2 was re-checked at the branch's round-2 fix head: six commits ahead of main, with subject 'Clarify stage-two parent observations and binding prerequisites'. The head is named in the Linear marker only. The check was static and read-only. make spec exits 0, and its only warnings are the two older ones at FR-017 line 174. The computed matrix has 631 records: the 16 additions plus the intended AC-24 text amendment, which stays untagged. No 7+ hex string, local path or timestamp appears in the committed diff.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | fix commit 'Clarify stage-two parent observations and binding prerequisites': O performs the parent-only checks on its own retained direct M Child: a nonblocking same-child waitid(WNOWAIT) before the consuming wait, then a same-child query returning ECHILD after the actual Some, under sole-waiter, disposition and history premises. MonitorReaped retains these fixed facts in checked order. The verifier checks the bound facts and the checker/mutation provenance, and is forbidden to issue a non-parent waitid or to treat its own ECHILD as proof (FR-034:1552-1567; AC-86; TC step 29i). The feature exception names these checks. |
