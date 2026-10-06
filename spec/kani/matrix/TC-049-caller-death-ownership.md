---
id: TC-049
title: "Verify original-caller death cannot release an unowned backend"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: verifies
---
# TC-049: Verify original-caller death cannot release an unowned backend

## Description

Planned production scenarios for [FR-034](../functional/FR-034-caller-death-ownership.md). Guardian
CODE builds on the merged containment code slice (PR #295) and remains planned; this amendment does
not require completion of all parent IR-241 work. No executable coverage of the new allocation is
claimed. Use the real Cargo guardian as namespace PID 1, the real bounded executor, and explicit
helper configuration as a library consumer. Tiny research probes are not production fixtures.
AC-23/26 exact early-stage caller-death scenarios use the opt-in fixture operation with shared
private sequence prefixes; positive post-Dispatch scenarios use the ordinary feature-off public API.
The mandatory AC-24 private lease-EOF oracle uses that same documented opt-in fixture operation.
[TC-039](./TC-039-bounded-proof-ceilings.md) retains its broader ceiling/refinement scenarios.

The merged PR #295 recipe fails the actual post-clone/pre-`child_wait` monitor-death boundary;
IR-652 specifies outer kernel containment and bounded unnamed report storage to repair it.
Nine scratch cases do not cover actual original-caller death/exclusive lease, exact internal bwrap
boundary, production observer/deadline, cargo report inheritance or all-owner storage release.
AC-31 through AC-34 remain mandatory UNRUN CODE gates, with no implementation coverage claimed.

## Evidence delivery allocation

IR-639 lifecycle CODE is one PR. Slice 1 (ordinary production seams without fixture extension) and
slice 2 (IR-655's still-owed exact-boundary evidence) are internal commit stages and review scopes
of that PR, not separate merge deliveries. The table allocates which obligations each internal stage
backs. The rationale is the new O-owned gate/reap/accounting schedule: current single-M/I witnesses
and C-origin observations cannot prove O's exact transition or final whole-run sample. This changes
implementation/evidence order only; every existing guarantee remains required before complete IR-639
delivery or Kani MVP acceptance. Slice 1 preparation/readiness is not either acceptance. No new
atomic criterion, fixture DTO, rights, scenario, scheduling pause or numeric cap is allocated.

Both slices are PLANNED/UNRUN. The table allocates Test assertions, not coverage. A mixed criterion
must remain untagged until ALL its obligations have actual evidence; partial slice 1 assertions
must not carry its whole-criterion Trace tag or move strict coverage. A slice-1-only criterion may
be tagged only after actual complete assertions exist. Inspection, source schedule analysis and
scratch facilities cannot stand in for required Test evidence. If measurement shows an ordinary
seam cannot expose its allocated predicate without fixture extension, record that measured gap
explicitly and move the owed evidence to slice 2; do not weaken the criterion or fabricate coverage.
For ANY unlisted criterion, retain every existing obligation and verification method. Ordinary-seam
assertions belong to stage 1 only where they genuinely expose the required predicate; any remaining
topology/temporal/authority predicate belongs to stage 2 as explicitly recorded owed evidence.
No omission implies unaffected coverage or waived evidence. Every mixed whole criterion stays
untagged until both stages provide ALL required assertions, even if its stage-1 checks pass.

IR-655 fixture SPEC may be authored from actual integrated O production source on the UNMERGED
stage-1 branch. Its source grounding shall establish the real O/C whole-run sample schedule before
measuring a numeric coordination cap; the cap remains unselected/unmeasured until that proof and
measurement exist. IR-655 SPEC shall merge before any fixture CODE implementing its new allocation,
and before the single lifecycle CODE PR merges. This permits source-grounded specification without
a stage-1 merge prerequisite; it allocates no fixture DTO/design or cap now.

Before the single CODE PR merges, the integrated O/C schedule proof and coordination-cap evidence
shall be re-grounded against its exact FINAL frozen source, including rebases and review fixes.
Relevant scheduling, coordination, ownership or executable-path/bound changes invalidate earlier
proof applicability: reassess the real path and re-measure the cap when that path or bound changes.
Any required amended IR-655 SPEC shall merge BEFORE implementing its changed fixture allocation;
the final CODE source must agree with that merged SPEC before merge. Divergence blocks merge,
not merely a note attached to stale unmerged-source evidence. Metadata-only review custody changes
require source-correspondence confirmation, not pointless repeated runtime measurements. No cap
is selected or measured here; actual source/proof/cap correspondence is a final merge gate.

The single CODE PR may open only after all required pre-PR gates pass. It may merge only after both
internal stages are complete and every old test is genuinely adapted, or consciously retired by an
explicit listed SPEC delta preserving a stronger guarantee and its actual adverse witness. Full
gates, old assertions, no skipped tests, no compatibility and matched helper identity remain
mandatory. An internal stage may be uncompilable/unmergeable while the overall PR is pending;
partial readiness never substitutes for final acceptance or for passing pre-PR gates.

| Criterion | Slice 1: ordinary-seam Test evidence, without fixture extension | Slice 2: IR-655 owed Test evidence; PLANNED/untagged in slice 1 |
|---|---|---|
| FR-017-AC-19 | Real normal-library cargo/Kani export, actual N >= 5 argv, isolated unnamed descriptor authority, zero/partial/valid EOF and independent resource-stop classifications; no named report or fallback. | Any predicate actually found unavailable through ordinary seams is explicitly owed; collector/lifetime dependencies retain the AC-32/33 allocation below. |
| FR-034-AC-1 | Normal-helper startup/authority and observable C/group death refuse production creation; absent lease, failed setup and creator-thread death produce no backend marker. | Exact pre-handoff/O-origin initialization windows, missing role/independent termination authority and bounded-bootstrap/no-production assertions at those exact boundaries. Whole AC stays untagged while any predicate is owed. |
| FR-034-AC-2 | External harness retains actual C Child; bounded live procfs parent/start/namespace validation and pidfds observe O/M and death at positively observable production boundaries; no after-death PID reopen/host scan. Typed missing-authority refusal before M is tested. | O-origin immutable arm/gate/confirmation facts at EXACT installed bwrap internal child_wait/eventfd window; independent actual O and unclaimed-I whole-tree termination AT THAT WINDOW. Generic M-spawn observation is insufficient; whole AC stays untagged. |
| FR-034-AC-3 | Actual claimed-I pin and positive escaped-worker evidence at observable ClaimedBootstrap/InitReady/Dispatched caller-lease EOF; no fabricated dead-caller result. | Any initialized-boundary role/termination facts unavailable through ordinary seams remain explicitly owed; existing I pin never supplies independent O authority. Whole AC is tagged only after every required stage assertion. |
| FR-034-AC-5 | Actual live C→L→O→M→I identity translations and ordinary setup refusal tests where observable; real normal-helper authentication, not helper identity override. | Needed role-labelled chain and independent termination witness beyond existing single M/I pin; unavailable inner/unclaimed/outer facts cannot be inferred. Whole AC stays untagged where these facts are owed. |
| FR-034-AC-7 | Ordinary malformed/missing info, observable M failure and owned O cancellation/refusal; confirm positively pinned external O/tree termination where observable. | Exact internal-handoff failure/recovery and O-origin immutable gate/inner-confirm/M-reap order; no later C timestamp reconstructs it. Whole AC stays untagged. |
| FR-034-AC-8/10/22 | Positive escaped-worker acknowledgement/live pins, observable C/O/I death, ordinary completion/refusal/deadline and independent-run protection; record dead pins before emergency cleanup. | Literal child birth AFTER actual FINAL whole-run sample, authenticated Completed ordering and distinct O-origin inner-confirm/M-reap/outer-confirm observations. No earlier C-only sample or sample-membership absence proves that temporal witness. Whole mixed criteria stay untagged. |
| FR-034-AC-11 | Positively pinned guardian death at observable startup/Ready/post-Dispatch boundaries; live-C typed guardian failure even beside success output, with dead acknowledged worker before emergency cleanup. | Exact pre-peer/Ready or bootstrap windows and independent role/tree witnesses not observable through stage-1 seams. No later outer cleanup masks missing boundary evidence; whole AC stays untagged while owed. |
| FR-034-AC-12 | Real unnamed report final-close/no-named-artifact lifetime, ordinary opposite-owner failures, bounded role settlement and honest other-artifact limits. | Exact startup refusal/retained-gate cleanup order and all-owner independent role/tree witness unavailable through current operation. Whole AC stays untagged. |
| FR-034-AC-14 | Actual ordinary-caller inherited profile/errno and mapping/private-proc/pidfd capability refusals before Dispatch, no policy mutation/weaker mode; setup docs and role/storage costs. | No exact-window fixture allocation required for these predicates; any measured unavailable Test predicate remains expressly owed. |
| FR-034-AC-17 | Real backend raw argv/stdin/environment/cwd echo through normal artifacts with only exact allocated report locator changed. | No exact-window extension required for this recipe predicate. |
| FR-034-AC-23/26 | Existing authorized shared-prefix stage facts and normal feature-off positive post-Dispatch caller/group death controls; positively pinned externally observable processes/SID/PGID, reporter exclusion across L/O/M/I. | Exact early-boundary O/unclaimed-I teardown, needed role chain/independent termination witness, no fabricated outer death from existing single M/I pin. Whole mixed criteria stay untagged. |
| FR-034-AC-24/27/28 | Existing single observation operation and immutable live-C pre-escalation lease-EOF raw oracle/mutants, unchanged normal feature-off/on artifact identity and unchanged single-right DTO meaning; separate final report controls remain live. | Any additional O-origin order/role/termination facts need IR-655 allocation; no surface extension is authorized here. Outer teardown must never rescue ignored-inner-EOF. Whole criterion is tagged only on complete assertions. |
| FR-034-AC-4/6/9/13/15/16/18/19/20/21/25/29/30 | Existing authority/observer/independent-run, matched-helper, protocol/rights/raw capture, classification/deadline/setup-cap, build-feature and documentation predicates use ordinary normal-artifact or exact declared inspection seams. Existing method/classification obligations stay intact. | Any topology-sensitive authority/death/order predicate ordinary seams cannot expose is explicitly recorded as stage-2 owed; no whole criterion is tagged from a partial seam assertion or unaffected-status assumption. |
| FR-034-AC-31 | Actual ordinary caller/profile, creating-thread lifetime, mapping/private-proc/identity and original-deadline refusal tests; external owned live pins after C/O/M death at OBSERVABLE boundaries. | Exact bwrap internal eventfd window, O-origin immutable gate/confirmation/reap and actual unclaimed-I whole-tree termination there; required expanded role/independent authority evidence. Whole AC remains untagged. |
| FR-034-AC-32 | Actual kernel collector/accounting units and normal-library cargo/Kani pipe roundtrip; hard cap, resize reservation, unmapped backing, slow/over-cap no-deadlock, original deadline and single/batch resource classifications. | Integrated O/C schedule must be source-grounded before any final-whole-run-sample temporal witness; that witness stays owed, never inferred from collector units. Any unavailable ordinary-seam predicate is explicitly transferred, not waived. |
| FR-034-AC-33 | Actual pipe writers/EOF, O spawn-copy closure/M settlement where observable, four seals/consumer refusal/bounded OwnedFd read, independent lease/report channels, stable identity and unnamed backing lifetime through normal seams. | Independent all-owner role/tree termination authority or O-origin immutable EOF/confirmation/reap order if unavailable to ordinary seams; existing one M/I pin cannot establish O death. Whole AC stays untagged while any such assertion is owed. |
| FR-034-AC-34 | Existing real live-C lease-close observation keeps outer ownership/final controls, ignored-EOF mutant fails before escalation; ordinary accounting/deadline failures retain classifications. | O/C integrated sampling/confirmation/reap facts needed for literal final-sample and ownership ordering; source schedule alone supplies no runtime parity. Whole AC stays untagged while required facts are owed. |
| FR-034-AC-35 | PLANNED/UNRUN: independent host pathname/abstract listener and host proc-alias exclusion, actual confinement capability refusal and no contained writer export (step 22). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; no generic namespace label or prior death test supplies this new criterion. |
| FR-034-AC-36 | PLANNED/UNRUN: independent real socket-stdin admission, actual production capture-pipe inventory and unchanged non-socket/closed-input controls, exact unavailable refusal and absent execution/terminal result (step 23). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; C-only trusted reporter is not backend stdio. |
| FR-034-AC-37 | PLANNED/UNRUN: independent trusted I/O channel lifetime, actual arbitrary-backend/descendant/sibling-exec noninheritance/reachability and leaked-control mutant (step 24). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; existing EOF tests alone do not supply the new confinement criterion. |
| FR-034-AC-38 | PLANNED/UNRUN: independent normal settlement/join, original-window unavailable confirmation, typed diagnostic-only refusal and no new post-return custody (step 25). Untagged until actual complete Tests. | Any unavailable ordinary-seam observation is explicitly owed; prior death tests or source-only fault discussion do not supply this new criterion. |

External evidence is limited to positively owned live chain observations before death and retained
pidfds thereafter. It is Test evidence for the observed boundary only, never authority for a
host-wide scan, exact internal bwrap eventfd timing, or inferred O death. A single actual M or I
pin does not prove outer teardown. The numeric fixture cap remains unselected/unmeasured; no
extension design or cap selection is authorized by this allocation.

The original retained-gate unit test remains required:
`gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend`.
The original after-final-sample unit test also remains required:
`completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample`.
Their obsolete direct-M invocation may leave the unit-test target uncompilable during slice 1;
record that compile gap explicitly. Independently compiling normal-library integration tests do
not settle it or permit skip/deletion. There is no separate slice-1 merge: the compile gap remains
a blocker to the single CODE PR's pre-PR gates until resolved within its internal stages.
There is no compatibility prepare API, libtest helper-identity override or weakened assertion.
Genuine adaptation uses the required O-origin retained-gate and final-whole-run-sample witnesses
in internal stage 2 after merged IR-655 SPEC; it does not await a separate CODE-stage merge.
Preserve original assertions/mutants until genuine new-design adverse witnesses fail and restored
controls pass. Any retirement/re-aim requires a separately listed
spec delta naming the original temporal/ownership oracle, new O-origin boundary, stronger confirmed
whole-outer-tree guarantee and an adverse witness defeating actual new cancellation authority.
The independent AC-24 ignored-inner-lease-EOF oracle is never retired or masked by outer cleanup.

## Test Procedure

1. Use the separate named guardian-feature-off Cargo invocation to build the normal package library
   and real `quire-kani-guardian` from matching source/build inputs in the owning target directory.
   Do not enable the feature through a self dev-dependency. Supply the required explicit helper path
   to the original-caller fixture using the ordinary public API. Keep that caller in its own
   positively verified dedicated session/group. Observe a positive production backend startup marker
   and pin its actual escaped worker before immediate post-Dispatch caller death; no sleep or host
   scan establishes success. Feature-on early-stage cases require the matched build from step 12
   before invoking the single operation, not unavailable private reads from the feature-off API.
2. In separate feature-on cases select shared-sequence prefixes as data at BeforeMonitor, Bootstrap
   immediately after actual monitor spawn and before any INIT claim or gate release, ClaimedGated,
   ClaimedBootstrap and InitReady. Production traverses the same functions/order: verify armed outer
   INIT before spawning monitor, claim gated inner INIT and bind observer, release gate,
   authenticate Ready. Require unconditional monotonic read-only publication, no copied
   orchestrator, feature branch, hook or production report I/O. Map the harness-created anonymous
   socketpair only into fixture startup stdout; safely duplicate stdout as a >=3 CLOEXEC OwnedFd
   auxiliary and mark original stdout CLOEXEC BEFORE any spawn. Use only auxiliary for witness
   reporting and configure every child's stdio/mapping explicitly to exclude both report
   descriptors. At the exact prefix seal actual published stage, caller/owned monitor/claimed INIT
   start and namespace identities and transferred descriptor identity. Bootstrap carries ONE clone
   of the actual owned monitor pidfd and typed InitUnclaimed: INIT may exist, but no claim or gate
   release occurred; fabricate no INIT pin. ClaimedGated and later instead carry ONE cloned
   validated INIT pidfd. BeforeMonitor carries typed NoInit/no-spawn/setup facts and no rights.
   After complete bounded send, self-SIGKILL the original caller or its positively owned dedicated
   group before the next transition; no ACK/controller pause. Authenticate sender credentials
   against the harness's actual unreaped caller Child/start/ pidfd and match received safe CLOEXEC
   descriptor identity and pin kind to the sealed original. Reject wrong sender/type,
   unknown/truncated control or extra/missing rights. Test claimed-INIT receipt both live and
   already dead; verify live identity when available and retain the transferred validated pin after
   death, never reopen a reused PID. ClaimedGated and later require confirmed caller/INIT death and
   no backend marker. NoInit requires actual no INIT/marker, closed actual lease/pair and
   termination of any other positively owned setup child. At Bootstrap instead require sealed
   monitor-created/no-claim/gate-held/no-Dispatch facts, caller and pinned monitor death, closed
   original-caller gate/lease ownership and no backend marker under bounded observation. Do not
   infer guardian EOF exit or INIT death from monitor readiness or pipe EOF. Record the present PR
   #295 monitor-PDEATH/internal-child_wait leak as the named IR-652 gap; these facts do not measure
   INIT teardown or satisfy its repair. Separately analyze the unchanged cleanup guarantee under
   IR-652 through actual trusted gate-EOF bootstrap, exclusive lease loss, bounded guardian refusal
   and namespace-INIT teardown, including INIT creation, internal map/setup handoff before the
   public gate and actual parent-death installation order. Do not assume monitor death releases that
   internal wait. Held gate does not establish a stable descendant snapshot because bwrap may fork
   during setup; no after-death PID reopen/host scan supplies missing INIT authority. This is
   stage-only Test plus separately required Analysis, not full namespace-teardown coverage.
   ClaimedGated positively retains gate; ClaimedBootstrap does not label its released gate retained.
   Record failure before independent owned emergency cleanup. Missing stage/pin, partial send or
   failed exclusion/coordination fails and invokes unchanged cleanup while caller lives, within the
   original deadline. Repeat feature-off public post-Dispatch caller/group SIGKILL with step 1's
   positive marker. Retain abort and forced process-kill OOM-model cases without host memory
   pressure. Absent/closed lease and connection cap cause bounded helper refusal; gate EOF never
   authorizes production Dispatch. Verify actual SID/PGID isolation before Ready/Dispatch and
   isolated guardian lease cleanup after caller-group death; pre-session gated INIT remains
   startup-group owned. The successful self-kill operation cannot return or fabricate a dead caller
   result. The surviving harness judges raw witness/pins/marker before its own emergency cleanup.
   The queued-pidfd probe establishes only the Linux descriptor facility; actual caller Rust stage
   coverage, reporter inheritance exclusion, positive INIT tests and Bootstrap lifecycle
   repair/Analysis under IR-652 remain CODE gates.

3. Kill the actual guardian INIT through its pinned pidfd before its peer/Ready claim, in InitReady,
   and immediately after Dispatch. Require no pre-Dispatch backend marker and kernel cancellation of
   post-Dispatch descendants. With a live original caller, even confirmed teardown beside valid
   success output must produce the typed guardian-failure refusal. Do not model the helper as a
   fictional direct bounded executor Child.
4. Substitute another actual sender/control capability and require authority rejection. Alter the
   monitor/INIT start identity, parent, namespace identity and namespace PID independently; provide
   incomplete startup data and a positively dead INIT. Check refusal before Dispatch. Verify kernel
   Ready sender PID equals the claimed INIT chained to the actual unreaped monitor, and observer
   readiness precedes Dispatch. Substitute a stale helper built with different lifecycle
   capabilities and source/build inputs, even with a caller-supplied matching version or digest
   label. Require mismatch against the actual running library's identity and actual invoked
   executable, not a path inspected before replacement.
5. Before M creation, positively verify O actual outer PID 1, private proc/mount identity, mapping,
   parent-death arm and original-exclusive bootstrap authority. Fail each capability separately:
   require typed refusal before M, without host mount/profile/sysctl changes. Kill L before O arm;
   require no inner child and bounded exclusive EOF refusal. Kill L after arm and O during M spawn,
   gated I and live descendants; require actual owned outer pin termination and kernel whole-tree
   cancellation. Fail/malformed/overlimit inner info and reparent M children before claim: outer
   authority cancels without host scan or fabricated inner pin. Exercise actual original C death
   at the precise installed bwrap post-clone/pre-child_wait boundary and natural M/info-reader loss.
   Require the old merged PR #295 control to FAIL this named teardown assertion, then the new
   production allocation to pass; public gate timing or elapsed sleeps do not identify that window.
6. After Dispatch, create double-fork/setsid descendants which reparent before observation,
   positively acknowledged descendants born during cancellation, and nested PID namespaces where the
   required facilities permit them. Kill original caller or guardian separately and verify all owned
   descendants terminate through INIT teardown, including unsampled ones. Run a second independent
   namespace and unrelated fixture child beside the target; they stay live until their own cleanup.
   Record unsupported platform refusal rather than a passing skip.
7. Exercise whole-batch ownership with multiple FR-017 harness members in one launcher. Verify one
   guardian, monitor, namespace and lease own the entire group under its existing memory ceiling and
   multiplied wall deadline. Normal completion with adopted background work, nonzero/signal exit,
   exec failure, explicit cancellation, timeout, memory excess, capture overflow/read failure and
   observation failure all require confirmed teardown before outcomes. Close backend streams and
   fail the helper separately; require bounded capture settlement, appropriate typed refusal, no
   proof accepted from success output beside unconfirmed cleanup, monitor reaping with a live
   caller, and surviving-owner cleanup of other assigned artifacts after caller loss or helper
   failure. The report expectation uses FR-017's unnamed bounded pipe-to-memfd storage, not guardian
   Drop/unlink. Kill all actual report owners and verify final kernel descriptor/backing release
   without any persistent report pathname. Other assigned artifacts retain truthful surviving-owner
   cleanup and all-owner-death limits. No process-only fixture or emergency deletion proves report
   lifetime.

8. Supply a missing, non-executable or unusable explicit helper path. Require typed refusal and no
   backend marker, with no PATH/global discovery, copied fixture binary or alternate launcher.
   Inspect setup docs for deliberate matching library/helper delivery, and the Linux, procfs
   children/RSS and pidfd prerequisites. Distinguish merged containment user/PID namespace,
   die-with-parent/info/gate flags from planned guardian PID-1/new-session prerequisites.
9. Create the private anonymous connected pair with exclusive original-caller ownership. Verify the
   guardian's mapped creator UID and the host's kernel Ready sender PID against actual INIT; do not
   equate guardian-side PID0 or creator credentials to actual guardian sender identity. Kill the
   original caller before lease authentication. Have foreign same- and different-UID fixture
   processes bind publicly discoverable names and supply plausible live lease/Dispatch; the guardian
   never connects to them, and no backend marker appears. Try another pair, mismatched mapped UID,
   replayed control and leaked lease rights; each refuses. Pair creation failure is setup refusal
   with no listener/path fallback. Send malformed, unknown-field, oversized and excessive pending
   control records, plus unknown, extra/missing, wrong-type and truncated ancillary
   rights/credentials; received rights must be CLOEXEC and closed on refusal. Close the lease.
   Require bounded cancellation/refusal, never Dispatch from EOF or bootstrap-gate bytes. Race an
   unrelated exec and verify it and backend descendants inherit neither a caller lease endpoint nor
   bootstrap-control writer. In the feature-on fixture verify BOTH report descriptors are absent
   from the real monitor, gated INIT, execed guardian, backend/descendants and unrelated exec; check
   the actual report socket identity, not only its fd number. Require the harness report stream's
   EOF after fixture death without waiting for a leaked child writer. Explicit child stdio must
   never restore reporter inheritance or use the report as capture. Positively inspect both actual
   reporter descriptors' CLOEXEC flags before the first spawn. Remove each flag independently and
   require its named pre-spawn FD-flag assertion to fail, even if another exclusion defense prevents
   a leak. Mutate child stdio/mapping reporter exclusion separately and require its
   inherited-reporter or extra-writer/EOF assertion to fail before emergency cleanup closes leaked
   copies. Restore controls. Inspect safe standard-stdout AsFd duplication to OwnedFd auxiliary,
   original CLOEXEC marking BEFORE spawn and every child stdio/mapping; no arbitrary inherited-FD
   adoption or unsafe exception participates.
10. Run a backend echoing raw non-UTF8 argv, stdin, overridden/inherited environment and cwd, plus
    distinct stdout/stderr markers. Verify exact recipe/capture separation. Feed actual ordinary
    completed/refused/falsified reports, wall/memory stops and ambiguous live-worker RSS through the
    existing classifiers. When FR-028 AC-24 native refinement is implemented, repeat its
    resource-stop control through this ownership path and assert the original class/evidence.
11. Distinguish an already expired original deadline, expiry during handshake, and helper-cap expiry
    while that deadline remains live. The first causes no Dispatch and inconclusive with the
    timed-out reason; actual deadline expiry during setup has the same classification; helper-cap
    expiry is typed setup refusal. Verify no deadline reset, bounded shutdown and observation, and
    refusal on unconfirmed teardown without a physical-disappearance claim.
12. Before using the operation, build the normal library, packaged caller fixture and real helper
    with guardian-test-support enabled through the separate named guardian-feature-on Cargo
    invocation. Select the package helper from the consumer manifest with cargo -p and match
    target/profile/features/flags, verifying actual library/helper identity. This is separate from
    step 1's feature-off invocation, with no cfg-test library or self dev-dependency unification.
    First remove guardian lease-EOF cancellation. In the live original fixture caller invoke the
    single `guardian-test-support` fixture operation around the real private production
    `close_lease_and_observe` cancellation boundary after Dispatch. It consumes only `CallerLease`;
    retain `RunOwner`'s unreaped monitor and claimed INIT handles and an escaped worker's positive
    handshake/pidfd. Observe its actual bounded LeaseClosing phase, then capture its typed
    `LeaseCloseObservation` and the worker's pinned state as immutable raw facts before immediately
    and unconditionally invoking production escalation/cleanup. The test harness requires confirmed
    INIT termination and a dead positively acknowledged pinned worker in those pre-escalation raw
    observations; the ignored-EOF mutant records escalation-required with live INIT/worker and fails
    the harness-owned surviving-descendant predicate. Later successful escalation cannot turn that
    record into a passing EOF-cancellation oracle. Keep the original caller alive: no parent-death
    kill, resource deadline or controller INIT signal establishes this observation. Choose an
    original deadline with remaining observation budget; expiry or unavailable observation is a
    failed/inconclusive fixture, never a passing mutant result. No sleep or elapsed-time threshold
    establishes success. Before Dispatch, the fixture operation itself sends SIGSTOP through its
    claimed INIT pidfd after InitReady, positively verifies state T plus unchanged start/namespace
    identity, and invokes the unchanged private production Dispatch frame-send step while retaining
    live original-caller RunOwner ownership. Require the complete actual production frame and rights
    queued through its bounded nonblocking transport, not fixture-written serialization. The typed
    pending-frame result is separate from ACK wait; do not wait for stopped INIT acknowledgement.
    Partial-send failure, would-block beyond the original bounded budget or unavailable transport
    must record coordination failure and cleanup, never establish a passing fixture. Invoke
    unchanged synchronous close_lease_and_observe. Its actual lease close precedes unconditional
    private monotonic read-only LeaseClosing publication; at actual publication seal the optional
    actual-close completion ordinal and publication ordinal in one immutable per-run snapshot. An
    internal owned continuation thread reads that snapshot and sends SIGCONT through only that
    pinned pidfd. The harness requires a present completed-close ordinal strictly below publication;
    later closure cannot fill an earlier snapshot. This raw ordering assertion fails early
    publication independently of which thread is scheduled first, even when guardian termination and
    the marker oracle otherwise pass. Publication adds no production-stage branch, callback,
    blocking handoff or extra I/O. The fixture-owned continuation uses no external-controller
    permission. Capture publication/coordination facts and raw termination/marker observations
    before cancellation escalation. The correct guardian confirms INIT termination without a backend
    marker; ignored EOF fails the closed-lease authorization predicate or records
    escalation-required. The external harness evaluates these sealed pre-escalation facts only after
    unconditional cleanup returns. SIGCONT itself cannot satisfy the EOF predicate. Require named
    assertions to fail when publication precedes actual close, publication is removed, or actual
    lease close is skipped. Evaluate the retained early-publication snapshot after the later close
    and cleanup complete; its original missing/inverted close ordinal must remain unchanged and fail
    the ordering assertion. No scheduling restriction is needed to distinguish it from
    close-before-publication. Neither SIGCONT nor valid-looking ordinals alone satisfy the actual
    INIT/marker/worker predicate. A missing publication or failed owned continuation records
    coordination failure, not a passing platform skip, and cleanup joins/resumes or cancels only the
    fixture's pinned INIT. This checks observable EOF precedence, not future-death prediction. The
    normal production driver immediately follows this same typed observation boundary with bounded
    cleanup; the fixture uses the opt-in observation operation at that boundary without a cfg-test
    close hook, synthetic ownership, exported run handles or controller-blocked escalation. After
    the operation returns following unconditional cleanup, evaluate the AC-24 predicate from the
    immutable pre-escalation raw record, separately from the returned cleanup result; the library
    evaluates no pass/fail oracle. Never infer the predicate from the eventually dead worker.
    Independently remove positive Dispatch authorization, replace the PID-1 guardian with a non-INIT
    watcher, remove session isolation, and close/reap before pinned startup confirmation. Require
    the corresponding premature-backend-marker, surviving-descendant, actual SID/PGID or
    startup-order assertion to fail, rather than compilation or fixture setup. Record the ownership
    observation before emergency cleanup of only the fixture's pinned namespace/group. Restore
    production and require focused controls to pass. Trace each actual asserted AC.
13. Use the actual matched artifacts from steps 1 and 12 to test both feature-mismatch directions.
    Supply the feature-off helper to the feature-on caller, then the feature-on helper to the normal
    feature-off production caller. The bounded executor must refuse actual artifact identity before
    backend Dispatch in both cases; no supplied epoch/version/digest may bless a pair. Build a
    normal feature-off consumer in its separate invocation and require a compile-fail check
    specifically for the absent fixture operation. Run ordinary public caller-death controls for
    positive post-Dispatch death against the actual feature-off library. Exact early-stage controls
    use the feature-on shared-prefix operation; a feature-on library avoiding its operation cannot
    replace the feature-off controls. Inspect exactly one documented feature-on fixture operation
    and the absence of public lease, process-ownership, cancellation or cleanup-deferring callback
    exports. Inspect shared private production stage/cleanup paths for one shared sequence and
    identical functions/order selected as all-stages or prefix data, and unconditional read-only
    stage publication. Only the single fixture item may be feature-gated; no production feature
    branch or copied orchestrator may implement early-stage tests. Keep this structural evidence
    alongside actual runtime observations and the mandatory AC-24 mutants. Record structural
    inspection separately from runtime/compile Tests; no feature-dependent stage bypass or
    resource/identity weakening is permitted.
14. Through the feature-on operation, fail stage observation, overflow bounded observation storage
    and fail owned stop/T-state/publication/resume coordination separately. Verify publication
    cannot precede actual lease close and that SIGCONT alone yields no passing EOF predicate. Apply
    the missing-publication and skipped-close mutants beside the primary ignored-EOF mutant; each
    must fail its named raw-observation/EOF assertion before cleanup can mask the recorded state.
    Each raw result records failure/unavailability and cleanup still executes. The harness must
    reject these records as passing AC-24 evidence; retain pinned INIT/monitor observations for
    confirmed cleanup or typed unconfirmed refusal. Inspect CG's test-only publication and
    allocation of downstream production-feature exclusion to
    [IR-649](https://linear.app/agent-ix/issue/IR-649), owned by the QSL driver lane. CG inspection
    passes only the published contract; no CG-only runtime check satisfies the downstream gate.
    IR-649 must separately verify the driver's dependency edges for all its production-build
    profiles, reject direct and transitive feature unification, and pass the feature-off controls.
    Keep that gate pending until its owner delivers actual driver evidence.

15. Run FR-017 AC-19's actual cargo/Kani 0.68 descriptor export, hard writer cap, concurrent pipe/
    kernel-backing accounting, slow collector, actual EOF/all writer copies, sealing, final handoff
    and all-owner death scenarios. O concurrently services controls/report/deadline/accounting; no
    completion wait cycle or deadline reset is allowed. Confirm overflow yields single-run
    KaniRunOutcome::Inconclusive with MemoryExhausted; batches keep whole-batch memory-exhausted
    refusal with no member classified, while FR-029 maps ResourceExhausted, never Failed or
    truncated acceptance. Pipe incompatibility stops CODE and requires measured spec revision; never
    select another storage mode at runtime.

16. Positively validate actual host/outer/inner pidfd identity translation, original
    caller-exclusive lease liveness immediately before spawn/Dispatch, credential changes/rearm,
    real observer and original ceiling/deadline across every role. C/group, L, O and I death each
    has an owned bounded settlement path; actual C death must not be replaced by controller
    bootstrap EOF. During live-C AC-24 observation retain O/L handles and separate report control,
    withholding outer escalation until immutable raw observations are sealed. Require the
    ignored-EOF mutant to fail unchanged. The nine successful scratch cases cover feasibility only;
    these production gates are UNRUN.

17. Measure actual ordinary-caller inherited profile/unshare/mount permission independently of the
    codex-node scratch label; record typed refusal where bwrap-only execution formerly succeeded.
    Keep the actual C spawning thread alive through L settlement; exit it while C lives and require
    owned cleanup and typed ownership failure, never successful evidence. Verify O closes its report
    spawn writer, C/L inherit none, and M termination closes any inherited writer before final EOF.
    Inspect the exact nested flags and fd3/fd4/N mapping/owners. Attempt pipe growth past reserved
    capacity; missing enforceable bound gives capability refusal before Dispatch. Apply each missing
    consumer F_GET_SEALS seal and require refusal before read. These tests remain UNRUN.

18. Exercise the UNRUN report-writer entry successful control with the real matched normal helper
    and its actual nested UID mapping. O records the original pipe's device/inode before mapping
    and sends those values with actual N >= 5 over authenticated original-run/O control. Require I
    to compare its newly opened write-only/nonblocking CLOEXEC pipe File against that O-origin
    identity before original-slot close or backend Dispatch. Record actual access/identity/flags,
    backend positive startup/export and bounded final EOF/read, not a self-comparison of N with its
    proc link. Actual mapped-UID reopen denial is typed pre-Dispatch refusal, never passing skip or
    permission workaround. The genuine helper/bwrap/backend/Cargo/Kani roundtrip remains UNRUN.
19. In separate real protocol/owned-descriptor entry cases, supply foreign/replayed run or O
    authority, missing expected identity, changed device/inode and substituted wrong pipe at N;
    also supply wrong-type/read-only descriptors, unavailable N and actual denied reopen access.
    Require the named authentication, independent identity or access cause before Dispatch, with
    no backend marker and bounded owned cleanup. Use only positively owned descriptors/children.
    A skipped-authentication or skipped-O-identity-validation matched-helper mutant must fail
    this refusal/no-marker predicate despite plausible slot numbers and valid-looking bytes;
    restored controls pass. No caller/library build-identity override supplies a test seam.
20. Establish successful original-slot closure at the actual single-thread entry boundary before
    later descriptor allocation/reuse. Require the original N closed exactly once, the NEW owned
    writer retained/CLOEXEC, and a separately owned sentinel/control descriptor still usable.
    Then deliberately reuse the freed N with an owned sentinel and exercise later settlement;
    require that sentinel still usable, detecting a duplicate numeric close after reuse. A
    wrong-slot-close mutant must fail sentinel/ownership or original-writer-closure assertions;
    a skipped-close mutant must fail the actual closure/extra-writer predicate. The observation
    must describe actual close/descriptor state at that boundary, not a later reused N or an
    implementation counter detached from the close. Emergency cleanup follows sealed observations.
21. Positively identify the original report pipe in the actual backend's N >= 5 after exec and
    verify expected access and genuine exported bytes. Spawn an unrelated owned exec from the same
    entry path and require BOTH the original inherited slot and NEW writer identity absent there;
    CLOEXEC flags alone do not establish exclusion. C/L still hold no writer. Test actual reopened
    and descendant/Cargo writer copies, M settlement and actual pipe EOF after all holders close.
    A backend-mapping omission must fail the real export control; unrelated-exec writer leakage
    must fail actual inherited-identity/extra-holder or EOF assertions before emergency cleanup.
    Restore controls; no report-to-stdio mapping, named fallback or C memfd proc reopening is used.

Steps 18–21 are PLANNED/UNRUN Test procedures for the existing writer-entry obligations, not new
criteria or a claim that the current fixture operation exposes these boundaries. Use actual ordinary
production/protocol/kernel descriptor seams in internal production stage 1 where available. If exact
authentication, original-slot close/no-reuse or inheritance observations require a fixture
extension, record the measured missing seam and carry that evidence to IR-655 stage 2 with SPEC
merged before fixture CODE. No new DTO/rights/hook/cap is allocated here. Mixed whole criteria
remain untagged until all assertions are actual, alongside the unchanged independent AC-24 EOF
oracle and one-CODE-PR gate.

22. PLANNED/UNRUN (FR-034 AC-35). First source-audit safe backend-only installation before
    Dispatch and the arbitrary recipe, with no unsafe/pre_exec, unfiltered release, temporary trusted
    FD inheritance or supervisor-wide filter. Verify same owned PID under I and exact original
    argv0/argv/environment/cwd/stdio and ownership/deadline. Force policy/filter/privilege installation
    failure before positive Dispatch: require typed unavailable refusal over authenticated bounded
    startup channel, no arbitrary recipe exec and confirmed owned cleanup. For post-Dispatch exec
    failure, author a regular executable script with a shebang naming a deliberately absent absolute
    interpreter in the owned confined root. Close its writing handle and retain unchanged file/mode
    and interpreter absence throughout: no sleep, removal/replacement race, copied ELF or public hook.
    Require the actual KaniInstallation::require_executable precheck to pass, positive Dispatch to
    occur, and the actual recipe exec to fail ENOENT because that interpreter is absent. This avoids
    ENOEXEC shell fallback; require no shell substitution or unfiltered retry. Separately retain
    missing/non-executable launcher cases as pre-Dispatch Tool refusals, never post-Dispatch evidence.
    Source: execute.rs::start calls run/tool.rs::require_executable (regular-file/execute-bit check);
    Linux [execve](https://man7.org/linux/man-pages/man2/execve.2.html) names absent script interpreter
    as ENOENT. The authored fixture and actual positive barriers remain PLANNED/UNRUN. Require
    existing bounded backend-failure handling, teardown/deadline/capture settlement, no
    synthetic report/status/evidence and no unfiltered retry/fallback. Initial trusted-helper spawn
    or early EOF proves neither installation nor recipe exec. Startup transport remains CLOEXEC
    through successful recipe exec. Run trusted installer with separate sanitized loader environment;
    carry original raw backend environment as authenticated bounded metadata and restore it only
    at filtered recipe exec after Dispatch. Controlled original LD_PRELOAD/LD_LIBRARY_PATH inputs
    must distinguish trusted startup (no loader activation) from actual filtered backend exec
    (original environment preserved), with positive observations and an early-loader activation
    mutant before cleanup. These tests remain PLANNED/UNRUN and choose no entry mechanism. A matched
    helper entry remains CODE-plan candidate only; infeasibility stops CODE for SPEC revision.
    Require L NEWNET/private-root before O and O validation before M; nested bind / / names confined
    O root, not host root. Own positively live host pathname/abstract listeners. Place the pathname listener inside the actual shared cwd/crate/target prefix visible
    to the backend, not an absent or unbound host directory. Run the same real backend
    listener/client workload as an unconfined owned host control and require a successful connection
    and descriptor transfer; a merely listening server or generic host client is insufficient. Under
    the confined production path, positively release the real backend attempt and require failed
    socket acquisition/connection/send-to-address, no accepted backend channel and no writer export.
    After Dispatch, create another host listener in a still-writable, backend-visible shared prefix;
    repeat the actual backend attempt and require continuous exclusion, not a startup socket scan.
    Verify actual backend denial of AF_UNIX socket creation and datagram socketpairs, including
    legacy socketcall/ABI and io_uring alternatives, and absence of inherited
    sockets/listeners/rings. Exercise pathname stream and datagram recipients, explicit-address
    sendto/sendmsg where sockets are admitted, alternate proc/root and retained-directory aliases.
    Anonymous connected stream socketpairs shall exchange real local data/SCM_RIGHTS, proving
    sendmsg is not blanket-denied. A mutation disabling the actual continuous IPC enforcement must
    fail its raw export/reachability predicate before emergency cleanup; restored enforcement
    passes. Positively verify permitted contained-local IPC and named original trusted owner
    channels. Record actual mount/net/PID identities and enforcement facts. Deny each actual
    network/root/proc/IPC protection capability separately; require typed pre-Dispatch unavailable
    admission with original cause, no backend marker and confirmed owned cleanup. Run genuine
    installed Cargo plus Kani 0.68 through the actual filter using cached inputs as the decisive
    PLANNED/UNRUN compatibility gate. If incompatible, stop CODE for SPEC revision, never
    relax/filter-fallback. Missing registry/git inputs or unavailable host Unix rendezvous after
    successful admission produce the existing unsuccessful-no-report Inconclusive NoVerdict
    classification, not a made-up setup error. Preserve deadline/memory classifications; no
    prefetch, recipe rewrite or weaker network mode. Independently stolen host authority is outside
    the uniform channel fault domain; peer-created shared-path listeners and contained export remain
    in scope.
23. PLANNED/UNRUN (FR-034 AC-36). Drive actual original C stdin before ordinary execution,
    not a new public request field. Verify internal capture/pin before child/control fd allocation
    or reuse. Real socketpair input and a socket without a usable peer must be captured Open and
    rejected by actual S_IFSOCK inspection before Dispatch. Inspect actual production fd1/fd2 mapping
    and owned inventory: both are capture pipes, not caller-selectable socket positions. Analysis
    establishes no ordinary path replaces them with sockets; no public hook or unreachable output
    socket case. Exercise inspection failure through the allowed private syscall boundary, retaining
    original errno and the actual production admission function. Pipes, regular files, terminal and
    /dev/null are admitted stdin controls. Initial authoritative absent fd0 (EBADF) or original
    exec-CLOEXEC gives internal Closed and is preserved; transport clone CLOEXEC does not reclassify
    original Open. Keep caller fd0/fd1/fd2 stable throughout setup as the trusted-caller precondition.
    Inspect public execute_kani_obligation and bounded batch rustdoc for the fd0..2 stability
    precondition, exact setup window and observed-only refusal limit. Treat authenticated self-proc
    lstat as a link-presence probe, never compare its symlink inode/type/mode with the target pin.
    Require followed stat and pin fstat agreement on target S_IFMT type/st_dev/st_ino and separate
    agreement of original F_GETFL & O_ACCMODE and F_GETFD & FD_CLOEXEC observations; exclude
    intentional pin CLOEXEC from the original exec flag. Positive unchanged Open input must admit despite different link metadata.
    Through the existing private capture boundary, exercise observed absence/Open mismatch, target
    identity/type mismatch, original access-mode/FD_CLOEXEC change and unexpected EBADF after Open:
    require typed refusal. Positively admit unchanged Open identity/access/exec flags while a real
    separate process sharing the original open file description changes O_NONBLOCK/O_APPEND status
    flags at an acknowledged capture boundary. Those mutable flags do not drive instability refusal;
    require unchanged input/recipe handling, no rewrite or restoration of a status-flag snapshot. No public hook, atomicity or same-inode open-file-description identity claim.
    Concurrent caller close/rebind/replacement is caller contract breach outside backend fault domain; no assertion requires detecting every ambient
    mutation or preventing such a race. Authenticated self-proc/safe absent capture remains UNRUN;
    unavailable safe capture refuses, with no raw descriptor adoption or public request field.
    Later captured-Open inspection EBADF/error refuses, never becomes Closed or probes child fd0.
    Echo raw original argv0/non-report argv/environment/cwd/input bytes through normal matched
    artifacts to detect rewriting/reopening. Admission-site Unavailable is independent of errno;
    MemoryMechanismUnavailable retains original io::Error and mandatory KaniStartupAdmissionCause:
    BackendStdioSocket { descriptor: Stdin }, BackendStdioInspectionFailed { descriptor },
    CapabilityUnavailable { capability }, or MemoryEnforcement for existing checks. Verify capability
    metadata/broadened docs/Display, no Option/None/default/message discriminant. Planned API changes
    and tests remain UNRUN; no fabricated evidence/kind/outcome/terminal/Failed, code()==None. Expire
    original deadline separately and preserve classification.
24. PLANNED/UNRUN (FR-034 AC-37). Positively validate the actual trusted channel owners/mappings: I
    receives its exclusive lease through Dispatch; O/C final report/control remains usable after
    that lease closes. Test backend/descendant/sibling-exec inherited descriptor identities and
    attempts to reach those channels through their actual proc/mount view, including the same-UID
    trusted I at /proc/1/fd, pidfd_getfd and ptrace. Record I non-dumpability and the enforced
    absence of backend ptrace-equivalent privilege, including across exec/nested-userns attempts;
    ambient Yama/profile denial is not the protection proof. A protection omission mutant exposes
    the actual I endpoint through at least one named route and fails before cleanup; restored
    protection passes. Throughout this test, neither controls nor fixture reporter are reachable by
    arbitrary code. Trusted O/I retain their intended endpoints; no assertion requires trusted
    owners themselves to lose access. A leaked-control/inheritance mutant fails the named
    arbitrary-code access predicate before owned emergency cleanup; restored controls pass. Preserve
    the ignored-EOF mutant's pre-escalation failure with live C and retained outer ownership, actual
    all-writer EOF, four seals and final delivery. Early channel close, outer kill or blanket
    sendmsg denial cannot repair the predicate.

25. PLANNED/UNRUN (FR-034 AC-38). Exercise normal completion, startup refusal and cancellation
    through production entry: independently confirm actual L/O/M/I termination/reaping, captures and
    existing creator-thread join before any conclusion. Through an already permitted private
    settlement-observation boundary, make confirmation unavailable at the original deadline without
    fabricating kernel D-state or creating an unkillable task. Require Err(KaniExecutionRefusal::Guardian
    { kind: GuardianFailureKind::CleanupUnconfirmed, detail }), code()==None, absent execution
    evidence/verdict/outcome/cleanup claim even beside valid report bytes. Detail must fit 4096 UTF-8
    bytes under existing accounting and contain only bounded already-observed role/identity/stage
    data, no handles/authority or inferred kernel cause. Exercise omission at a UTF-8 boundary and
    prove it cannot change kind or manufacture settlement. Restore the observation and positively
    confirm settlement; absence of a residual witness alone is not a positive control.
    Exercise Drop/join error paths through their actual allowed private boundary: settle or report
    inside the call, every allowance clamped to original remaining T; no added 250 ms/five-second
    grace, reset or post-return observation. Source/ownership Analysis must establish no new cleanup
    thread/daemon/custodian and no io::Error/Result-owned authority or dependence on error Drop.
    Exceptional existing unjoined creator-role observations must remain explicitly unconfirmed;
    never claim join/retirement or diagnose kernel failure from timeout. No public hook is allocated
    and no real kernel-stuck task is required. If a required ordinary seam is absent, record owed
    evidence under the existing fixture SPEC gate, never claim this criterion backed.
    Inspect the public GuardianFailureKind definition: only CleanupUnconfirmed, no extra kind or
    non-exhaustive catchall. Exercise existing startup Unavailable and Io paths plus executable
    prechecks and verify their existing MemoryMechanismUnavailable/Tool mapping remains; retain
    separately allocated admission context and existing resource/report/capture mappings. No message
    parsing or adoption of a WIP kind catalog.
    Inspect single/batch public rustdoc for both caller-stdio and kernel-settlement preconditions,
    exact setup/original settlement windows, typed error and diagnostic-only residual/no cleanup
    guarantee outside the kernel precondition. Keep ordinary caller/group death and contained
    writer/death/EOF/seal adverse gates mandatory; this fault boundary cannot repair their failure.

Steps 22–25 allocate independent new confinement/admission/settlement Tests, all PLANNED/UNRUN. They add no
fixture DTO, rights, hook or coordination cap; where an ordinary production seam is unavailable,
record the missing evidence for the existing IR-655 SPEC-before-fixture-CODE process. All old
criteria, assertions, genuine mutant parity and one-CODE-PR/full-gate requirements remain intact.

## Expected Results

| Authority | Required observation | Regression caught |
|---|---|---|
| FR-034-AC-1/2/3/7 | Trusted bootstrap is distinct from production Dispatch; verified outer authority covers unclaimed inner stages; inner claim precedes Dispatch | EOF starts subject; signal/monitor exit falsely confirms teardown; claim-to-Dispatch gap |
| FR-034-AC-4/5/6 | Exclusive original-caller pair and actual kernel Ready sender/INIT chain precede Dispatch | Public rendezvous capture; wrong pair or creator/sender credentials; arbitrary parent |
| FR-034-AC-8/9 | Kernel INIT teardown cancels escaped, late-born and nested descendants, preserving other runs | Kill only sampled PIDs; cancel another run |
| FR-034-AC-10/11/12/26 | Later caller-group/guardian death confirms namespace teardown and lease closure; live-caller helper failure refuses; unnamed report backing ends on final close and exact pre-handoff teardown is independently tested; process-only success establishes no storage claim | External sole gate owner dies; accept proof after helper failure; leak helper/socket |
| FR-034-AC-13/14/25 | Explicit real package helper matches actual running library build/capabilities; setup docs and missing-helper refusal are accurate | PATH/copy fallback; stale helper blessed by caller label |
| FR-034-AC-15/16 | Invalid bounded controls refuse; exclusive lease and safe child-only mapping | EOF authorizes Dispatch; descendants inherit caller lease |
| FR-034-AC-17/18/19; FR-028-AC-2/3/21/24; FR-017-AC-14/24/25 | Whole-batch ownership, exact recipe, separate captures and existing resource/refinement outcomes | One helper per member; diagnostics become report; weaken ceilings; ambiguous RSS becomes zero |
| FR-034-AC-20/21/22 | Original deadline and expired-deadline outcome persist; setup cap distinct; bounded observation refuses ambiguity | Reset deadline; setup refusal falsely timed out; hang capture |
| FR-034-AC-23/24 | Real helper, exact preclaim Bootstrap stage facts distinguished from separate cleanup Analysis, positive claimed-INIT pins and production typed lease-close boundary before INIT escalation | Ignored EOF keeps worker or accepts closed-lease Dispatch; emergency teardown masks failure |
| FR-034-AC-27/28/29/30 | Opt-in single observation operation, immutable pre-escalation raw facts and harness predicate, unconditional cleanup, same normal artifacts and CG publication and separately owned driver exclusion | Default export; controller pause; changed production stage; false oracle after cleanup; cfg-test epoch override; feature enabled in production |
| FR-034-AC-31 | Actual outer pin and whole-tree termination at exact bwrap handoff failure; ordinary caller capabilities and creating-thread lifetime observed | Old295 orphaned unclaimed INIT; profile-only availability assumption; parent-thread exit misclassified as success |
| FR-034-AC-32 | Hard writer retention bound and defined conservative charge; actual Kani pipe roundtrip; slow/overflow cases stop under original deadline | Unmapped backing counted as zero; resizable pipe exceeds reservation; backend/collector deadlock; report cap becomes Failed or truncated pass |
| FR-034-AC-33 | O spawn writer closed, M and inner writers terminated, actual EOF, verified four seals and final descriptor read; final-close backing reclamation | Extra monitor writer prevents EOF; forged seal claim; report residue after all owners die |
| FR-034-AC-34 | Separate lease/report controls and live C retained outer ownership; ignored-EOF fails before outer escalation | Outer kill masks mandatory EOF mutant; deadline/ceiling reset or missing charge accepted |
| FR-034-AC-35 | PLANNED/UNRUN: safe same-PID backend-only installation, L/O/M allocation, real unconfined shared-prefix control and dynamic listener exclusion; actual Cargo/Kani compatibility; typed unavailable refusal and original build/resource classes | Policy bypass or inherited ring; peer socket created after Dispatch; unbound vacuous listener; unfiltered release or compatibility relaxation |
| FR-034-AC-36 | PLANNED/UNRUN: real socket stdin refuses, production fd1/fd2 capture inventory verified; explicit Closed and Open inspection error distinguished; mandatory typed cause and no fabricated result | Unreachable stdout socket test claimed; Open EBADF admitted as Closed; caller-fixable socket misreported as memory-only failure |
| FR-034-AC-37 | PLANNED/UNRUN: I non-dumpability/backend privilege exclusion block proc1fd/pidfd_getfd/ptrace; trusted channel lifetimes and unchanged EOF mutant retained; uniform authority fault domain | Ambient Yama masks absent protection; contained export excused as outside peer; final delivery closed early |
| FR-034-AC-38 | PLANNED/UNRUN: actual normal role/capture/spawner settlement; original-window typed CleanupUnconfirmed error with bounded diagnostic-only residuals, no new post-return custody; public precondition docs | Error carries authority; hidden cleanup worker; false joined/retired or kernel-cause claim; valid bytes accepted after unconfirmed settlement; grace resets T |

AC-31 through AC-34 and rewritten FR-017 AC-19 are UNRUN and must compute untagged until actual
production assertions are implemented. Scenario prose or research probes establish no executable
coverage. Native refinement remains planned until its actual typed entry is delivered. Host
destruction and uninterruptible tasks cannot justify fabricated teardown; live-caller unavailable
confirmation refuses. Caller-group signals and direct guardian death are included lifecycle cases,
not excluded double faults.
