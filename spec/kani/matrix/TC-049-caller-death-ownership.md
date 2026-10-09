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
and C-origin observations cannot prove O's exact transition or integrated accounting order. This changes
implementation/evidence order and the explicitly retired temporal oracle below; every existing guarantee remains required before complete IR-639
delivery or Kani MVP acceptance. Slice 1 preparation/readiness is not either acceptance. The stage-2 allocation below adds only its
planned live-birth and independent operation predicates;
it allocates no new external authority/right, scheduling pause or numeric cap.

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
| FR-034-AC-2 | External harness retains actual C Child; bounded live procfs parent/start/namespace validation and pidfds observe O/M and death at positively observable production boundaries; no after-death PID reopen/host scan. Typed missing-authority refusal before M is tested. | Independently checked actual O-origin arm/gate/confirmation behavior at EXACT installed bwrap internal child_wait/eventfd window; independent actual O and unclaimed-I whole-tree termination AT THAT WINDOW. Generic M-spawn observation is insufficient; whole AC stays untagged. |
| FR-034-AC-3 | Actual claimed-I pin and positive escaped-worker evidence at observable ClaimedBootstrap/InitReady/Dispatched caller-lease EOF; no fabricated dead-caller result. | Any initialized-boundary role/termination facts unavailable through ordinary seams remain explicitly owed; existing I pin never supplies independent O authority. Whole AC is tagged only after every required stage assertion. |
| FR-034-AC-5 | Actual live C→L→O→M→I identity translations and ordinary setup refusal tests where observable; real normal-helper authentication, not helper identity override. | Needed role-labelled chain and independent termination witness beyond existing single M/I pin; unavailable inner/unclaimed/outer facts cannot be inferred. Whole AC stays untagged where these facts are owed. |
| FR-034-AC-7 | Ordinary malformed/missing info, observable M failure and owned O cancellation/refusal; confirm positively pinned external O/tree termination where observable. | Exact internal-handoff failure/recovery and O-origin immutable gate/inner-confirm/M-reap order; no later C timestamp reconstructs it. Whole AC stays untagged. |
| FR-034-AC-8/10/22 | Positive escaped-worker acknowledgement/live pins, observable C/O/I death, ordinary completion/refusal/deadline and independent-run protection; record dead pins before emergency cleanup. | The explicitly amended live-I birth-after-completed-ordinary-O-sample oracle below, genuine Completed and distinct O-origin inner-confirm/M-reap/outer-confirm observations, plus actual I-confirm/M-reap/seal order independent of records. Outer confirmation and confirmed whole-outer-tree termination remain separately owed; inner settlement never supplies them. The former literal birth AFTER FINAL whole-run sample obligation is retired only as stated below; membership absence and later outer cleanup do not supply replacement parity. Sampled-membership teardown authority is separately allocated to AC-55 source Analysis plus the unchanged AC-24 non-INIT-watcher runtime mutant, not a claimed unsampled-through-kill runtime window. The existing AC-8/10 Test obligations own the separate BOTH-formerly-live-pins-dead-before-outer-escalation check in step 28a; AC-55 Analysis never backs that runtime predicate. Whole mixed criteria stay untagged until every obligation is genuinely backed. |
| FR-034-AC-11 | Positively pinned guardian death at observable startup/Ready/post-Dispatch boundaries; live-C typed guardian failure even beside success output, with dead acknowledged worker before emergency cleanup. | Exact pre-peer/Ready or bootstrap windows and independent role/tree witnesses not observable through stage-1 seams. No later outer cleanup masks missing boundary evidence; whole AC stays untagged while owed. |
| FR-034-AC-12 | Real unnamed report final-close/no-named-artifact lifetime, ordinary opposite-owner failures, bounded role settlement and honest other-artifact limits. | Exact startup refusal/retained-gate cleanup order and all-owner independent role/tree witness unavailable through current operation. Whole AC stays untagged. |
| FR-034-AC-14 | Actual ordinary-caller inherited profile/errno and mapping/private-proc/pidfd capability refusals before Dispatch, no policy mutation/weaker mode; setup docs and role/storage costs. | No exact-window fixture allocation required for these predicates; any measured unavailable Test predicate remains expressly owed. |
| FR-034-AC-17 | Real backend raw argv/stdin/environment/cwd echo through normal artifacts with only exact allocated report locator changed. | No exact-window extension required for this recipe predicate. |
| FR-034-AC-23/26 | Shared-prefix selection input and independent original identity/right/kernel observations and normal feature-off positive post-Dispatch caller/group death controls; positively pinned externally observable processes/SID/PGID, reporter exclusion across L/O/M/I. | Exact early-boundary O/unclaimed-I teardown, needed role chain/independent termination witness, no fabricated outer death from existing single M/I pin. Whole mixed criteria stay untagged. |
| FR-034-AC-24/27/28 | Existing single operation and independently checked actual live-C pre-escalation lease-EOF/worker/marker predicates and mutants, unchanged normal feature-off/on artifact identity and unchanged single-right DTO meaning; separate final report controls remain live. | Any additional O-origin order/role/termination facts need IR-655 allocation; no surface extension is authorized here. Outer teardown must never rescue ignored-inner-EOF. Whole criterion is tagged only on complete assertions. |
| FR-034-AC-4/6/9/13/15/16/18/19/20/21/25/29/30 | Existing authority/observer/independent-run, matched-helper, protocol/rights/raw capture, classification/deadline/setup-cap, build-feature and documentation predicates use ordinary normal-artifact or exact declared inspection seams. Existing method/classification obligations stay intact. | Any topology-sensitive authority/death/order predicate ordinary seams cannot expose is explicitly recorded as stage-2 owed; no whole criterion is tagged from a partial seam assertion or unaffected-status assumption. |
| FR-034-AC-31 | Actual ordinary caller/profile, creating-thread lifetime, mapping/private-proc/identity and original-deadline refusal tests; external owned live pins after C/O/M death at OBSERVABLE boundaries. | Exact bwrap internal eventfd window, Independently checked actual O-origin gate/confirmation/reap and actual unclaimed-I whole-tree termination there; required expanded role/independent authority evidence. Whole AC remains untagged. |
| FR-034-AC-32 | Actual kernel collector/accounting units and normal-library cargo/Kani pipe roundtrip; hard cap, resize reservation, unmapped backing, slow/over-cap no-deadlock, original deadline and single/batch resource classifications. | Integrated O/C schedule must establish the amended live-I birth and ordinary continuing accounting before its temporal witness; runtime parity stays owed, never inferred from collector units. Any unavailable ordinary-seam predicate is explicitly transferred, not waived. |
| FR-034-AC-33 | Actual pipe writers/EOF, O spawn-copy closure/M settlement where observable, four seals/consumer refusal/bounded OwnedFd read, independent lease/report channels, stable identity and unnamed backing lifetime through normal seams. | Independent all-owner role/tree termination authority or independently checked actual O-origin EOF/confirmation/reap order if unavailable to ordinary seams; existing one M/I pin cannot establish O death. Whole AC stays untagged while any such assertion is owed. |
| FR-034-AC-34 | Existing real live-C lease-close observation keeps outer ownership/final controls, ignored-EOF mutant fails before escalation; ordinary accounting/deadline failures retain classifications. | O/C integrated continuing sampling and producer confirmation/reap/seal facts needed for amended live-birth and ownership ordering; source schedule alone supplies no runtime parity. Whole AC stays untagged while required facts are owed. |
| FR-034-AC-35 | PLANNED/UNRUN: independent host pathname/abstract listener and host proc-alias exclusion, actual confinement capability refusal and no contained writer export (step 22). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; no generic namespace label or prior death test supplies this new criterion. |
| FR-034-AC-36 | PLANNED/UNRUN: independent real socket-stdin admission, actual production capture-pipe inventory and unchanged non-socket/closed-input controls, exact unavailable refusal and absent execution/terminal result (step 23). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; C-only trusted reporter is not backend stdio. |
| FR-034-AC-37 | PLANNED/UNRUN: independent trusted I/O channel lifetime, actual arbitrary-backend/descendant/sibling-exec noninheritance/reachability and leaked-control mutant (step 24). Untagged until actual Tests. | Any missing ordinary-seam predicate is explicitly owed; existing EOF tests alone do not supply the new confinement criterion. |
| FR-034-AC-38 | PLANNED/UNRUN: independent normal settlement/join, original-window unavailable confirmation, typed diagnostic-only refusal and no new post-return custody (step 25). Untagged until actual complete Tests. | Any unavailable ordinary-seam observation is explicitly owed; prior death tests or source-only fault discussion do not supply this new criterion. |
| FR-034-AC-39 | PLANNED/UNRUN ordinary production Tests: actual native installation success/failure, syscall process-kill and real unconfined effect controls plus unchanged classification/settlement. Unsupported-native cfg/support and unconditional native-x86_64 x32 policy require source Analysis, not an invented Test. | No new IR-655 fixture facility is allocated. Any predicate unavailable through ordinary production seams remains explicitly owed through the SPEC-before-fixture-CODE process; missing tools/kernel/workload or source-only Analysis supplies no whole-criterion Test credit. |
| FR-034-AC-40 | PLANNED/UNRUN: declared helper role/site/phase source-flow and public rustdoc Analysis, plus available genuine ordinary transport/consumer Tests for actual kind/errno, finite typed provenance, diagnostic independence and explicit loss; Analysis gives no Test credit. | Exact-boundary or unavailable genuine producer/transport predicates remain owed through IR-655 SPEC-before-fixture-CODE; no new hook or forced allocation failure. Whole criterion stays untagged while any required Test obligation is unavailable. |
| FR-034-AC-41 | PLANNED/UNRUN Analysis: Exact consumer manifest/package/bin and resolved workspace configuration. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-42 | PLANNED/UNRUN Analysis: Final linked executable and loaded-library GNU/LLVM disassembly with stable inputs. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-43 | PLANNED/UNRUN Analysis: Final address mappings for operation and thread roots, all indirect targets. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-44 | PLANNED/UNRUN Analysis: No reachable recursive call cycle. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-45 | PLANNED/UNRUN Analysis: Per-thread live-frame maxima summed for all concurrent instances. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-46 | PLANNED/UNRUN Analysis: Actual panic unwind/cleanup edges or demonstrated abort absence. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-47 | PLANNED/UNRUN Analysis: Finite external bounds matching loaded runtime ABI/configuration. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-48 | PLANNED/UNRUN Analysis: Returned heap capacities, metadata and overlap inside declared charge. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-49 | PLANNED/UNRUN Analysis: Reanalysis after source/build/compiler/runtime/artifact/premise change. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-50 | PLANNED/UNRUN Analysis: Bound fits independent declaration; missing proof is UNPROVEN, missing charge uses existing refusal. CG CODE author supplies the operational receipt; independent CODE reviewer checks it before IR-639 CODE merge. Completion is zero until actual evidence, never inferred from a matrix method row. | No fixture extension; unresolved Analysis remains UNPROVEN and supplies no Test credit. |
| FR-034-AC-51 | PLANNED/UNRUN: Release-dependent live-I birth and child ACK (step28a). CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-52 | PLANNED/UNRUN: Genuine I confirmation/strict pre-seal order (step28b). CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-53 | PLANNED/UNRUN: Genuine retained M reap/strict pre-seal order (step28c). CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-54 | PLANNED/UNRUN: Retained-gate I confirmation before close (step28d). CG CODE author supplies per-predicate source/bounds and applicable runtime receipts; independent CODE reviewer checks before CODE acceptance. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-55 | PLANNED/UNRUN: Final-source contained teardown authority/no sampled-membership kill path Analysis (step28e), paired with the independently owed AC-24 non-INIT-watcher Test. CG CODE author supplies exhaustive authority-path Analysis; independent CODE reviewer checks before CODE acceptance. Completion is zero until actual evidence, never inferred from a matrix method row. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-56 | PLANNED/UNRUN: Published successful-Commit final-accounting schedule (step28f). CG CODE author supplies per-predicate source/bounds Analysis; independent CODE reviewer checks before retirement/CODE acceptance. Completion is zero until actual evidence, never inferred from a matrix method row. | No existing test supplies backing; Analysis is not Test credit. Genuine authority and bounded integrated schedule remain owed. |
| FR-034-AC-57 | PLANNED/UNRUN: New O→L pre-local-Armed negative uses zero rights. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-58 | PLANNED/UNRUN: L authenticates actual O Child PID/UID/GID0/build/run/source/state/stamp. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-59 | PLANNED/UNRUN: Invalid negative rights refuse with actual owner/capability custody. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-60 | PLANNED/UNRUN: Exact O-only cleanup/source capability scope; real Child stays at L. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-61 | PLANNED/UNRUN: No positive Armed/namespace/phase/report/Dispatch admission. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-62 | PLANNED/UNRUN: Original authentic producer stamp adopted before wait, earliest cutoff preserved. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-63 | PLANNED/UNRUN: Final original cause only after actual O/L/capture/creator/EOF settlement. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-64 | PLANNED/UNRUN: No new envelope type/ACK/window/cap/public category. Actual source/ordinary seam evidence required; no existing test credit. | Inspection only: no fixture extension or Test completion credit. |
| FR-034-AC-65 | PLANNED/UNRUN: Fresh exhausting pre-send tick retires zero-progress Failure into OwnerStop. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-66 | PLANNED/UNRUN: L-local preArm same-byte/one-clone route | Genuine source clone and state authentication required; no current route execution credit. |
| FR-034-AC-67 | PLANNED/UNRUN: C authenticates full L negative only AwaitArm through actual chain. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-68 | PLANNED/UNRUN: No pidfd_getfd/pidfd setns/process_madvise/numeric reopening. Actual source/ordinary seam evidence required; no existing test credit. | Inspection only: no fixture extension or Test completion credit. |
| FR-034-AC-69 | PLANNED/UNRUN: Unconfirmed or abnormal settlement keeps CleanupUnconfirmed/no evidence. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-70 | PLANNED/UNRUN: First emitted byte fixes immutable single candidate. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-71 | PLANNED/UNRUN: All due pre-send ticks and actual fresh complete named inputs. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-72 | PLANNED/UNRUN Analysis only: Follow OuterRunOwner.sampling and actual ledger/history/owner_stop ownership until settlement. | Ordinary runtime retention seam is absent; AC73 settlement and AC77 immediate cancellation Test witnesses remain separately owed. |
| FR-034-AC-73 | PLANNED/UNRUN: Claimed I termination plus separate real M reap/writerEOF/finaltick precede O normal return. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-74 | PLANNED/UNRUN: Absent peak field means no peak transport, no inferred zero/history/current validity. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-75 | PLANNED/UNRUN: Later owned observation/collector/transport error blocks normal O Code0. Actual source/ordinary seam evidence required; no existing test credit. | Any exact O/role/order/later-state predicate unavailable to ordinary seams remains owed through IR-655 SPEC-before-fixture-CODE; no new implemented witness or passing skip. |
| FR-034-AC-76 | PLANNED/UNRUN: Genuine admitted KCMP_FILE/equivalent OFD Test oracle, with independently opened same-O and foreign negatives. | Unavailable oracle remains UNRUN; inode equality is not proof and no runtime KCMP requirement is added. |
| FR-034-AC-77 | PLANNED/UNRUN: Fresh complete exhaustion with real C lease held open triggers actual retained I and separate M cancellation immediately. | Genuine ordinary actor/order witness owed; existing blocking cleanup is not the bounded claimed step. |
| FR-034-AC-94 | PLANNED/UNRUN: Standalone queued-claim cleanup Test and source/custody Analysis below. | No existing negative/phase test backs close-only provisional receipt; unavailable schedule/authority predicates are owned through IR-655 SPEC-before-fixture-CODE, with actual CG CODE author receipts checked by the independent CODE reviewer. Test completion remains zero without actual evidence. |

External evidence is limited to positively owned live chain observations before death and retained
pidfds thereafter. It is Test evidence for the observed boundary only, never authority for a
host-wide scan, exact internal bwrap eventfd timing, or inferred O death. A single actual M or I
pin does not prove outer teardown. The numeric fixture cap remains unselected/unmeasured; no
numeric cap selection or fixture implementation is authorized by this allocation; its observation
properties are allocated below and require grounded CODE evidence before implementation.

The original retained-gate unit test remains required:
`gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend`.
The original after-final-sample unit test and all its assertions remain required pending measured
replacement parity:
`completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample`.
Their obsolete direct-M invocation may leave the unit-test target uncompilable during slice 1;
record that compile gap explicitly. Independently compiling normal-library integration tests do
not settle it or permit skip/deletion. There is no separate slice-1 merge: the compile gap remains
a blocker to the single CODE PR's pre-PR gates until resolved within its internal stages.
There is no compatibility prepare API, libtest helper-identity override or weakened assertion.
Genuine adaptation uses the required O-origin retained-gate and amended live-I birth/positive-operation witnesses
in internal stage 2 after merged IR-655 SPEC; it does not await a separate CODE-stage merge.
Preserve original assertions/mutants until genuine new-design adverse witnesses fail and restored
controls pass. Any retirement/re-aim requires a separately listed
spec delta naming the original temporal/ownership oracle, new O-origin boundary, stronger confirmed
whole-outer-tree guarantee and an adverse witness defeating actual new cancellation authority.
The independent AC-24 ignored-inner-lease-EOF oracle is never retired or masked by outer cleanup.

The [stage-2 behavioral allocation](../functional/FR-034-caller-death-ownership.md#stage-2-live-birth-and-producer-operation-observations)
explicitly retires only the literal birth AFTER FINAL whole-run sample obligation in the old
late-fork oracle. Successful-Commit final accounting shall follow actual I confirmation/M reap/writer EOF/seal,
as independently checked against published source under AC-56 before retirement. No claimed-I
descendant can fork after confirmed I teardown. Its replacement uses actual live-I birth after a
positively completed ordinary O sample, genuine I Completed, both live pins and confirmed I/M/both
descendant termination before outer escalation, with all later ordinary samples preserved. The stronger confirmed
whole-outer-tree guarantee remains required, with independent O-origin
outer-confirm observations in addition to inner I/M and descendant confirmation. Kernel
success is not inferred; unconfirmed teardown remains refusal. Old assertions/mutants are retained
until genuine restored/mutant parity is measured. The gate-close ordering case remains independent.

### Behavioral evidence and certificate retirement (PLANNED/UNRUN)

The former feature-only observation pipe/binding, five-record codec/slots, producer ordinals,
copied completion carrier and early report field are retired. They cannot be substituted with
renamed records or cached flags. Genuine operations, operational rights, report bytes/kernel seals,
accounting, scheduling and conditional signal safety remain required. Retirement is not completion
of any Test or Analysis. The following independent rows retain their criterion identifiers;
step29 and the conditional-safety checks below own their amended procedures.

| Criterion | Verification status and independent procedure | Evidence boundary |
|-----------|-----------------------------------------------|-------------------|
| FR-034-AC-78 | PLANNED/UNRUN: step 29, independent AC-78 check below. | C admits actual original O Armed only through genuine retained O/build/run authentication and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission. No prior Test or matrix row supplies completion. |
| FR-034-AC-79 | PLANNED/UNRUN: step 29, independent AC-79 check below. | Actual production control reception rejects malformed, partial or forbidden-right traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies a missing authenticated control. No prior Test or matrix row supplies completion. |
| FR-034-AC-80 | PLANNED/UNRUN: step 29, independent AC-80 check below. | Genuine original authenticated I Completed alone establishes completion reception; M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate is retired. No prior Test or matrix row supplies completion. |
| FR-034-AC-81 | PLANNED/UNRUN: step 29, independent AC-81 check below. | Every surviving actual simultaneous named fixture/control storage and pipe/backing reservation is charged before exposure; retired certificate storage is removed from charges only after its actual allocation disappears. No prior Test or matrix row supplies completion. |
| FR-034-AC-82 | PLANNED/UNRUN: step 29, independent AC-82 check below. | Actual supported-source/artifact Analysis and mechanical paired consumer/helper checks establish feature-off absence and unchanged production frames/rights after certificate retirement. No prior Test or matrix row supplies completion. |
| FR-034-AC-83 | PLANNED/UNRUN: step 29, independent AC-83 check below. | The original exclusive caller lease closes independently of retained monitor/INIT ownership and does not leak into children; a leaked alias cannot preserve authorization after actual caller death. No prior Test or matrix row supplies completion. |
| FR-034-AC-84 | PLANNED/UNRUN: step 29, independent AC-84 check below. | Every remaining fixture reporter/control/report endpoint is excluded from untrusted child/exec mappings and public control handles through actual descriptor checks. No prior Test or matrix row supplies completion. |
| FR-034-AC-85 | PLANNED/UNRUN: step 29, independent AC-85 check below. | Actual retained M consuming reap is not established by repeated or cached Child Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success. No prior Test or matrix row supplies completion. |
| FR-034-AC-86 | PLANNED/UNRUN: step 29, independent AC-86 check below. | O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD provenance under the original sole-waiter/disposition/history; independent checks reject fabricated, cached or missing consuming-operation facts without stored history as proof. No prior Test or matrix row supplies completion. |
| FR-034-AC-87 | PLANNED/UNRUN: step 29, independent AC-87 check below. | Actual I confirmation/M consuming reap precede report seal and actual I confirmation precedes retained-gate close; independent late-operation/early-close mutants fail. Producer ordinal certificates are retired. No prior Test or matrix row supplies completion. |
| FR-034-AC-88 | PLANNED/UNRUN: step 29, independent AC-88 check below. | O preserves every due ordinary sample and existing control/cutoff transition without observation ACK, pause, stored publication permission or suppressed tick. No prior Test or matrix row supplies completion. |
| FR-034-AC-89 | PLANNED/UNRUN: step 29, independent AC-89 check below. | Genuine original I Completed reception and original stop adoption precede actual original lease close, without a separate stored completion certificate. No prior Test or matrix row supplies completion. |
| FR-034-AC-90 | PLANNED/UNRUN: step 29, independent AC-90 check below. | Any remaining optional fixture write failure does not terminate O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer is retired, not activated by this condition. No prior Test or matrix row supplies completion. |
| FR-034-AC-91 | PLANNED/UNRUN: step 29, independent AC-91 check below. | Ordinary feature-on execution remains available without retired observation binding, endpoint, event emission or default identity. No prior Test or matrix row supplies completion. |
| FR-034-AC-92 | PLANNED/UNRUN: step 29, independent AC-92 check below. | Genuine adopted-orphan acknowledgement and release-dependent child birth after an actual complete ordinary O sample remain required without a stored sample-announcement certificate; missing real coordination is UNBACKED. No prior Test or matrix row supplies completion. |
| FR-034-AC-93 | PLANNED/UNRUN: step 29, independent AC-93 check below. | Every surviving feature-on named native workspace has separate supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose certificate workspace retires only with its actual allocation. No prior Test or matrix row supplies completion. |
| FR-034-AC-95 | PLANNED/UNRUN: step 29, independent AC-95 check below. | Actual O retains the original report backing identity through collection, kernel sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate. No prior Test or matrix row supplies completion. |
| FR-034-AC-96 | PLANNED/UNRUN: step 29, independent AC-96 check below. | Missing, malformed, wrong or unvalidated original O/build/run/control authority refuses without positive admission, retaining actual received-right custody and original cutoff; no default identity supplies authority. No prior Test or matrix row supplies completion. |
| FR-034-AC-97 | PLANNED/UNRUN: step 29, independent AC-97 check below. | Retired observation-binding echo creates no replacement comparison; actual original report backing equality and delivery/settlement authority remain required. No prior Test or matrix row supplies completion. |
| FR-034-AC-98 | PLANNED/UNRUN: step 29, independent AC-98 check below. | Feature-off paired consumers/helpers retain unchanged phase bytes/rights with no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding requirement. No prior Test or matrix row supplies completion. |
| FR-034-AC-99 | PLANNED/UNRUN: step 29, independent AC-99 check below. | Any remaining optional O fixture write requires actual ignored/unblocked/untraced stable single-thread proof, including independent across-PID-namespace initial untraced evidence, pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement supplies no signal proof or activation. No prior Test or matrix row supplies completion. |
| FR-034-AC-100 | PLANNED/UNRUN: step 29, independent AC-100 check below. | Unsupported, malformed, unavailable or unproved profiles permit no optional fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or private-proc zero alone repairs eligibility. No prior Test or matrix row supplies completion. |

## Test Procedure

1. Use the separate named guardian-feature-off Cargo invocation to build the normal package library
   and real `quire-kani-guardian` from matching source/build inputs in the owning target directory.
   Do not enable the feature through a self dev-dependency. Supply the required explicit helper path
   to the original-caller fixture using the ordinary public API. Keep that caller in its own
   positively verified dedicated session/group. Observe a positive production backend startup marker
   and pin its actual escaped worker before immediate post-Dispatch caller death; no sleep or host
   scan establishes success. Feature-on early-stage cases require the matched build from step 12
   before invoking the single operation, not unavailable private reads from the feature-off API.
2. In separate feature-on cases select BeforeMonitor, Bootstrap, ClaimedGated,
   ClaimedBootstrap and InitReady as shared-sequence inputs only. Inspect that actual production
   functions/order and admission are unchanged. Normal phases are control state, not live-state
   proof. Use the genuine matching normal helper and original C owner. Before any spawn safely
   duplicate the fixture's stdout reporter into a non-stdio CLOEXEC OwnedFd, mark stdout CLOEXEC
   and exclude both actual descriptors from every M/I/backend/unrelated-exec mapping. Independently
   test missing marks and leaked reporters. No raw-fd adoption or reporter-as-capture is allowed.
   Transfer only the actual original caller identity and genuine owned pin metadata: one original
   M pidfd for unclaimed Bootstrap or one already-validated I pidfd for claimed cases; BeforeMonitor
   carries no right and fabricates no INIT. Authenticate actual original caller Child/start/pidfd
   and kernel sender, transferred descriptor/pin kind, and actual predeath identity. Reject damaged,
   missing/extra/wrong rights and retain close custody; never reopen a PID after death. After complete
   bounded delivery, immediately self-SIGKILL actual C or its verified dedicated group without ACK
   or controller pause. Independently confirm caller/group death, original claimed I termination,
   no backend Dispatch/marker and no leaked owned inner processes before emergency cleanup. Verify
   live and already-dead original pin receipt without relabeling an unvalidated pin as claimed.
   Bootstrap M death proves neither unclaimed I death nor held gate; AC-31/IR-652 native-window
   cleanup evidence remains separately owed. Cached prefix, gate_retained, publication or returned
   category shall never pass an oracle. The exact fatal/concurrent-O early-gate-release class remains
   UNBACKED where no independent construction exists; do not replace it with marker absence or
   eventual dead I. Missing capability/coordination fails, never skip/pass. Preserve immediate
   unchanged owned cleanup for every returning failure and all ordinary feature-off post-Dispatch
   caller/group, abort and forced-kill OOM-model controls under original cutoff.

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
12. Using the single normal feature-on fixture and actual production lease-close operation,
    independently exercise ignored inner EOF and skipped actual lease close. Retain original C,
    actual M/I ownership and the positively acknowledged escaped worker's original live pin.
    Confirm actual I/worker death before escalation and no backend Dispatch/marker; later cleanup
    cannot repair a failure. The immediate private LeaseCloseObservation guides normal escalation,
    but its stored copy is not the Test oracle or death-cause proof. Independent kernel observations
    must distinguish required termination from escalation, resource cancellation or outer rescue.
    For pending authorization, genuinely SIGSTOP the original InitReady I, verify live T/start/
    namespace identity, and queue the complete actual production Dispatch frame/rights separately
    from ACK wait. Close the real original lease. A genuine bounded construction for continuation
    after removal of stored publication is still owed; no replacement snapshot, channel, timer,
    callback or pause is allocated. Never strand stopped I: unavailable coordination fails and
    unchanged cleanup resumes or cancels only its actual pinned I and joins owned continuations.
    SIGCONT is resumption, not cancellation or passing EOF evidence. Independently retain removed
    positive Dispatch, non-INIT watcher, broken session isolation and startup close/reap mutants;
    each must fail its actual behavior before emergency cleanup. Restored controls must pass.
    Normal library/helper artifacts, original stop/cutoff and urgent cancellation remain unchanged.
    No partial Test binds the whole AC-24 while its stopped-I or independent EOF predicates are
    unavailable. All replacement runtime evidence remains PLANNED/UNRUN.

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
    identical functions/order selected as all-stages or prefix data, without stored stage-publication acceptance. Only the single fixture item may be feature-gated; no production feature
    branch or copied orchestrator may implement early-stage tests. Keep this structural evidence
    alongside actual runtime observations and the mandatory AC-24 mutants. Record structural
    inspection separately from runtime/compile Tests; no feature-dependent stage bypass or
    resource/identity weakening is permitted.
14. Fail genuine identity/pin acquisition, reporter exclusion, bounded actual observation and
    stop/T-state/resume coordination at available owning seams. Each fails the fixture and invokes
    unchanged cleanup; no stored phase/ordinal/boolean repairs it. Independently verify the original
    skipped-close and ignored-EOF controls. No source-only retirement or schema test supplies their
    runtime credit. Inspect absence of retired publication snapshots/certificate transport, without
    changing legitimate production control states. Preserve IR-649's separately owned downstream
    production-feature exclusion gate: actual dependency edges/profiles reject direct or transitive
    guardian-test-support unification. CG inspection cannot supply that owner's driver evidence.

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
    independently checking actual I/worker termination before outer escalation. Require the
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
    implementation counter detached from the close. Emergency cleanup follows independent kernel assertions.
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
    confirmed owned teardown and original deadline/capture settlement, then the one existing public
    result: KaniRunOutcome::Inconclusive { reason: KaniInconclusiveReason::NoVerdict } for the single
    run and every compatible batch member. Actual observed unsuccessful backend exit with no report
    supports this result; it is not synthetic evidence, Tool Io, pre-Dispatch admission or Failed.
    Preserve resource/deadline and CleanupUnconfirmed precedence. Separately exercise C/launcher
    boundary I/O as the existing Tool refusal, never conflate it with the backend exec exit. Require
    no fabricated report/status/evidence, shell substitution or unfiltered retry/fallback. Initial trusted-helper spawn
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

25. PLANNED/UNRUN (FR-034 AC-38). Independently inspect the recorded SETTLE_RESERVE R=1 second
    and research receipt below: R=max(1 second,10*measuredP99), rounded up to whole seconds,
    fixed/non-caller-tunable. Remeasure actual integrated production whole-tree settlement as a
    mandatory CODE gate; the representative kernel probe is not product evidence. Verify short finite ceilings remain admitted with R_eff=min(R,T/2),
    duration division rounded down, no minimum-budget cause/refusal. Preserve the existing real
    single 200ms and two-member 200ms-per-member batch timeout cases; the whole batch bound is
    still N*T, with a reserve once from that outer bound. For finite deadlines, positively observe
    workdeadline=T-R_eff cancellation then all-role/capture/creator settlement by original T;
    require existing single TimedOut/batch BatchTimedOut paths naming original ceilings.
    Exercise zero/already-expired identity with current no-Dispatch timeout/capability ordering and
    no reserve-derived admission error or reset. A passed workdeadline with T still live immediately
    cancels/settles without Dispatch. Inspect rounding at integral/subsecond measured durations:
    least whole second >=max, no extra second at an integral boundary.
    Preserve checked_add None and overflowed N*T/Duration::MAX as admitted never-elapsing work,
    without workdeadline subtraction/forced timeout/Tool InvalidInput. Above4294967295 member
    timeout omission remains; a finite fitting outer bound still expires. Positively observe actual
    completion/cancellation/resource/caller-loss stop separately in None cases: the FIRST actual
    trigger starts one R settlement deadline, subsequent triggers cannot restart it. Confirm actual
    settlement before classification, and unavailable confirmation by that deadline gives only
    CleanupUnconfirmed. Do not create a synthetic initial deadline or change backend timeout flags.
    Use a real still-live worker at finite workdeadline and positive whole-chain termination
    witnesses, not only an artificially expired deadline with zero children. No early report or
    incomplete candidate may publish during settlement; original recipe/capture/memory accounting
    remains unchanged.
    Exercise normal completion, startup refusal and cancellation
    through production entry: independently confirm actual L/O/M/I termination/reaping, captures and
    existing creator-thread join before any conclusion. Through an already permitted private
    settlement-observation boundary, make confirmation unavailable at the applicable settlement deadline without
    fabricating kernel D-state or creating an unkillable task. Require Err(KaniExecutionRefusal::Guardian
    { kind: GuardianFailureKind::CleanupUnconfirmed, detail }), code()==None, absent execution
    evidence/verdict/outcome/cleanup claim even beside valid report bytes. Detail must fit 4096 UTF-8
    bytes under existing accounting and contain only bounded already-observed role/identity/stage
    data, no handles/authority or inferred kernel cause. Exercise omission at a UTF-8 boundary and
    prove it cannot change kind or manufacture settlement. Restore the observation and positively
    confirm actual settlement by the applicable deadline and candidate classification; absence of a residual witness alone is not a positive control.
    Exercise Drop/join error paths through their actual allowed private boundary: settle or report
    inside the call, every finite allowance fits R_eff/remaining whole T and each None allowance fits
    its single FIRST-stop-trigger-plus-R deadline; no accumulated phase
    allowance, post-expiry grace, reset or post-return observation. Source/ownership Analysis must establish no new cleanup
    thread/daemon/custodian and no io::Error/Result-owned authority or dependence on error Drop.
    Exceptional existing unjoined creator-role observations must remain explicitly unconfirmed;
    never claim join/retirement or diagnose kernel failure from timeout. No public hook is allocated
    and no real kernel-stuck task is required. If a required ordinary seam is absent, record owed
    evidence under the existing fixture SPEC gate, never claim this criterion backed.
    Inspect the public #[non_exhaustive] GuardianFailureKind definition: this amendment allocates
    only CleanupUnconfirmed, no other current kind or WIP kind catalog. Exercise existing startup Unavailable and Io paths plus executable
    prechecks and verify their existing MemoryMechanismUnavailable/Tool mapping remains; retain
    separately allocated admission context and existing resource/report/capture mappings. No message
    parsing or adoption of a WIP kind catalog.
    Inspect single/batch public rustdoc for both caller-stdio and kernel-settlement preconditions,
    exact setup/workdeadline/applicable settlement windows, finite R_eff and preserved None work/stop
    behavior, no minimum admission cause, and timeout
    versus unconfirmed-error precedence, typed error and diagnostic-only residual/no cleanup
    guarantee outside the kernel precondition. Keep ordinary caller/group death and contained
    writer/death/EOF/seal adverse gates mandatory; this fault boundary cannot repair their failure.

26. PLANNED/UNRUN (FR-034 AC-39). Perform source/cfg Analysis of actual native policy support
    and the safe trusted guardian/backend-installer boundary. For an unsupported-native branch
    unavailable on the current compiled target, inspect the actual typed branch and public refusal
    route; label it Analysis, not an executed Test. If source is not yet implemented, that Analysis
    remains UNRUN. Establish native installation success and real installation-failure Test cases
    through ordinary production paths when available; absent capability failure controls remain
    unavailable/UNRUN, with no assertion-skipping Test credit or invented admission seam. Require
    planned mandatory admission context naming BackendIpcExclusion for unsupported ABI/filter
    installation and TrustedOwnerProtection for privilege/protection failure, with typed cause
    provenance, no backend Dispatch and confirmed owned settlement. Header/open/helper spawn/EOF
    observations do not prove target-image ABI or installation success.
    Analysis must distinguish actual unsupported support provenance (semantic Unsupported with no
    raw errno) from original OS install errno/kind and finite reconstructible non-OS variant/sites.
    Inspect authenticated run/build and owned pre-recipe PID/state custody before
    classification. A kind-only projection or Display/message discriminator fails the allocation;
    do not demand an unavailable public stage query or universal original boxed-chain fidelity.
    Positively distinguish NNP Prctl/privilege/protection failures (TrustedOwnerProtection) from
    actual Seccomp filter installation failure (BackendIpcExclusion), even under a Filter wrapper.
    Source-audit process-kill architecture rejection and x32-number/alias rejection for EVERY native
    x86_64 policy independent of host kernel x32 support. For runtime Tests on applicable x86_64
    hosts, author tiny repository-owned standalone assembly fixtures with explicit entry and no
    libc/interpreter, using existing host GNU as/ld: --64/elf_x86_64 for native entry issuing compat
    int 0x80 and x32-number syscalls, --32/elf_i386 and --x32/elf32_x86_64 for actual compat/x32 exec.
    Do not add unsafe Rust, inline/global Rust assembly, copied binaries, dependencies, multilib
    runtimes, Rust targets or tooling fallback. Assembly/linking/runtime success is UNRUN until
    actually measured at the CODE gate. Require the same unconfined owned workload to produce the
    attempted syscall's actual observable effect; a merely attempted instruction, emulator or
    kernel-rejected image is insufficient. Missing tools/emulation/workload/kernel support is
    unavailable/UNRUN Test evidence with zero whole-criterion completion credit. Other native
    architectures require their own applicable source/workload evidence; these fixtures cover only
    x86_64. Native-emulator translated syscalls are actual native enforcement, not compat evidence.
    Execute the same workload through the real filtered backend boundary after authenticated
    Dispatch, including native entry followed by a compat syscall and an exec/descendant case.
    Positively observe actual policy termination before the denied syscall's observable effect,
    actual backend exit and whole-chain settlement. An actual architecture/x32-rule omission
    mutant must fail that effect/termination oracle before emergency cleanup; restored policy must
    pass. Preserve all other IPC/privilege policies in these controls. Do not infer image rejection
    at exec/entry from a syscall kill, or count a kernel-unsupported image as a filter success.
    For actual unsuccessful termination with no report, require existing Inconclusive NoVerdict
    after confirmed settlement and original capture/deadline handling; preserve memory/deadline
    precedence and CleanupUnconfirmed, and distinguish present/malformed/success-without-report
    cases through their existing mappings. Require no pre-Dispatch unavailable classification for
    the actual post-Dispatch event, synthetic Failed, fabricated evidence or unfiltered retry.
    Inspect the production exact-recipe construction and positively exercise admitted script/path
    and loader/environment behavior under the filter. Report pathname/content/PATH/execvp,
    shebang/PT_INTERP, loader and kernel binfmt_misc handler check-to-exec uncertainty explicitly: an earlier header/open
    observation is not actual image authority. No installation stability precondition, recipe
    substitution, inherited execution-FD shortcut or changed writable-path semantics is allowed.

Steps 22–26 allocate independent new confinement/admission/settlement/ABI Tests and named source
Analysis, all PLANNED/UNRUN. Analysis is never runtime Test credit. They add no
fixture DTO, rights, hook or coordination cap; where an ordinary production seam is unavailable,
record the missing evidence for the existing IR-655 SPEC-before-fixture-CODE process. All old
criteria, assertions, genuine mutant parity and one-CODE-PR/full-gate requirements remain intact.

27. PLANNED/UNRUN (FR-034 AC-40). Perform independent source-flow Analysis for the SPEC's
    declared trusted role/operation/phase cases; distinguish the actual producing owner from a
    forwarder. Treat M process creation/status/read errors as its trusted owner's errors, never
    invented M-origin Rust causes. Exclude arbitrary backend payloads and keep C-only control,
    capture and report-read preparation errors local with original sources intact. Record each
    actual outer ErrorKind/raw errno, typed operation/category, finite private fields, observed
    payload presence and precise opaque loss. Map implementation variants by actual typed predicate
    to the closed SPEC categories; adding/renaming a CODE variant does not enlarge that set.
    An unlisted semantic case follows the declared opaque or SPEC-allocation rule, never a guessed
    category. Source availability or an unmerged type name establishes no implementation credit.
    Read the actual standard From<TryReserveError> implementation and its source-presence contract
    for the selected toolchain. Compare real Other-wrapper producers and actual standard conversions
    without assuming every version drops its source or produces an executed allocation failure.
    Require the actual producer's resulting kind and observed payload presence/absence. Do not use
    unstable allocation-kind/layout APIs, fabricate a TryReserveError/errno or force allocation/
    capacity exhaustion. These source checks are Analysis, not transport/consumer Test credit.
    For available genuine production refusal cases, exercise the existing authenticated bounded
    transport and real public consumer before/after whole-chain settlement. Require its actual
    existing public mapping, exact original OS errno/kind or non-OS kind, typed originating operation
    and declared fidelity. Preserve observed source absence or explicit opaque loss, never original
    object/downcast identity. Admission emits no evidence/outcome/Failed; local causes and stronger
    AC-39 duties remain. Compare sender-owned storage charge and C's named receive/context metadata
    bound independently; both fit existing whole-control/whole-run caps and original cutoffs.
    Analyze optional diagnostic formatting-error and over-bound branches: the representable original
    typed cause is retained first, diagnostic detail is omitted, and the original cause/mapping wins.
    Audit receiver-side required context presence/type/framing separately from optional diagnostic
    byte rendering. With independently authenticated valid original EPERM metadata and a complete
    bounded context byte array containing invalid UTF-8 or a trailing incomplete UTF-8 scalar,
    require unchanged original raw errno/kind/mapping, no integrity/loss marker on this OS cause,
    and omitted diagnostic text. A diagnostic retention-bound excess within the whole-control cap
    likewise drops text. A missing/duplicate context field, non-u8 or non-array value, broken or
    truncated complete-frame structure or invalid mandatory kind/authority is required metadata
    failure: require its actual integrity predicate and existing site-first carrier/detail treatment,
    not alleged EPERM replay. Empty context bytes satisfy required presence while omitting text.
    Analyze the frozen combined seed as a CODE separation obligation, not executed acceptance.
    Mutants treating malformed optional UTF-8 as cause integrity or accepting broken mandatory
    framing as an omitted diagnostic must fail independently; genuine unavailable seams remain UNRUN.
    Required representation failure is distinct from a valid original operational cause even when wrapped
    as Other with a first-party boxed payload. Audit the sender's actual typed required-representation
    bound/format/unnamed-kind/OS-kind-mismatch branches and actual non-installation dependency
    cause/policy-site-domain checking failures against the single CauseIntegrity inventory.
    A valid original cause instead requires successfully captured original facts. Check each
    actual predicate and its checker role/site: sender capture and C projection are distinct;
    actual C-side rejection needs no delivered sender report and shall not invent sender provenance.
    Verify that these required encoding/checking predicates match only the integrity consumer row,
    never any valid-original declared-category, reservation or other custom-loss row; the wrapper
    kind/payload does not select another domain. Conversely, a genuine independent operational
    boxed cause with valid original representation must match only its loss row, preserving its
    actual kind. Source Analysis checks this disjointness even when genuine runtime cases are unavailable.
    For an available genuine complete bounded fault report in an existing I/O-cause-bearing result,
    require public InvalidData/no-errno,
    KaniCauseMetadataIntegrityError downcast Some, KaniCrossRoleCauseLoss downcast None, actual finite
    sender predicate/origin retained and the explicit authenticated-site Unavailable/non-site
    startup/protocol path. The public cause is required-representation failure, never original replay.
    A non-delivered fault report shall not produce a fabricated received predicate: require the actual
    C-observed incomplete/malformed cause, transport or ownership branch and original settlement.
    No artificial allocation failure, alternate frame/cap/hook or inferred EOF-as-sender-fault credit.
    Independently inspect actual non-installation dependency capture rejection and policy-site/cause
    projection rejection. For genuine available existing I/O-carrier cases require the actual
    NonInstallationDependencyCause or PolicySiteCauseMismatch integrity predicate, InvalidData/no
    errno, integrity downcast Some and loss downcast None. Source Analysis shall reject missing
    inventory/consumer allocation, fake bound/format predicates or generic original-loss projection.
    C-side rejection retains independently known C checker/site facts; a sender capture fault uses
    only actual delivered/authenticated sender facts. Existing detail-only observation cases retain
    private finite predicates without a new public carrier. Unavailable genuine tests remain owed.
    Independently audit every actual declared sender's
    no-errno producer/build domain against all39 closed named kinds, including standard-library and
    dependency branches and actual errno conversions. An added emitted kind must update producer-side
    encoding before CODE delivery. Fail-closed UnnameableOriginalKind integrity is an unexpected encoding
    fault, not original-kind normalization or acceptance of a supported producer/build gate;
    a genuine unsupported original emission leaves that gate unsatisfied. Unimplemented generic sender paths remain owed/UNRUN, not proved
    by available source or hypothetical future toolchains. No unstable kind API or artificial error
    construction shall stand in for genuine producer reachability. Distinguish malformed/unknown
    metadata from a valid original cause; source Analysis must show no fake original io-error replay,
    kind-driven Tool normalization or Dispatch/evidence. Exercise an available real cause-metadata
    corruption/omission control while independently retaining the actual negative admission site's
    authenticated run/build/role/identity and unchanged startup state. Unknown kind, malformed cause
    and missing cause metadata must each retain their actual private predicate within public KaniCauseMetadataIntegrityError, retain any actual local decoder source and return MemoryMechanismUnavailable with
    that site's exact admission context, cause.kind()==InvalidData, cause.raw_os_error()==None,
    code()==None and no evidence/outcome after confirmed settlement. The cause is the actual local
    integrity failure; no assertion shall claim an original producer kind/source or successful replay.
    Require cause.get_ref().and_then(|error| error.downcast_ref::<KaniCauseMetadataIntegrityError>())
    to be Some; verify the actual predicate/provenance and retained decoder source through internal
    source Analysis or an available ordinary test seam. Require a valid original payload-free
    InvalidData/no-errno control and genuine local custom InvalidData/no-errno control to produce
    None for that same integrity downcast, preserving their actual local source identity. Merely
    get_ref()==Some, InvalidData or diagnostic text is not an integrity oracle. Source Analysis shall
    verify public accessibility/private constructors and lack of diagnostic-derived classification.
    For a role-authenticated/site-unavailable control and a control lacking both facts, require private
    role/site fields to be present exactly when independently authenticated, otherwise None; no packet
    label fills missing facts. Public integrity downcast remains Some in the applicable cause-bearing
    result despite absent provenance fields, without fabricated admission authority. Available ordinary
    controls only; missing real cases remain UNRUN.
    Assert absence of KaniCrossRoleCauseLoss on the integrity cause. In a distinct control, fail independent
    site authentication: no fabricated admission context; require the explicit startup/protocol
    path. When a distinct actual C-side ownership/protection check fails, require that check's real
    Unavailable cause and known C-side context, without substituting a metadata error. Otherwise
    require ordinary Tool(KaniToolError::Io) with actual InvalidData/no-errno error and public
    KaniCauseMetadataIntegrityError downcast Some; the same metadata-integrity error remains typed
    regardless of the selected existing path.
    Preserve original settlement deadline and CleanupUnconfirmed override. A genuine valid original
    producer cause is a positive control retaining its own actual kind/errno and site-first mapping.
    Mutants routing authenticated cause-metadata faults to Tool, inventing an original kind/errno,
    attaching a loss marker or omitting the actual integrity source must fail this independent oracle.
    A missing real ordinary control seam leaves these Tests UNRUN/owed to Slice 2, not a fabricated
    fixture, assertion skip or source-Analysis completion credit.
    For genuine work/settlement observation/accounting failures, retain the actual private finite
    kind/errno/provenance/loss and representation/integrity predicate. Require the existing public
    MemoryObservationFailed { detail } with no evidence/outcome after settlement; there is no public
    io::Error/downcast carrier. Changing detail/omitting diagnostics shall not select the mapping.
    A malformed observation cause report retains only actual private integrity facts, not a fake
    original kind/source. Mutants promoting observation failure to Tool/Unavailable to expose a
    marker or treating detail as typed cause evidence must fail. Independent protocol/settlement
    failures retain their actual paths and CleanupUnconfirmed override; no public field is added.
    For genuine available no-errno custom-source loss in an existing I/O-cause-bearing public refusal,
    require public get_ref() downcast to
    KaniCrossRoleCauseLoss, exact original kind and absent raw errno; the marker source() is None
    and it claims no original source object/chain. Source Analysis shall verify private typed
    role/site fields retain the actual authenticated origin with no public getter/stage query or
    diagnostic-derived discriminant. Loss-free actual OS and payload-free errors must
    have no marker, with public OS errno preserved directly. Public finite sources already required
    by AC-39 and local sources must remain intact. Private finite provenance is not an original
    public downcast reconstruction or stage query. Unavailable genuine cases remain zero Test credit.
    Inspect the public single/batch bounded API and MemoryMechanismUnavailable rustdoc for listed
    cross-role scope, typed public marker detection, finite private provenance/opaque loss, observed
    source presence, absent/lossy optional diagnostics versus mandatory envelope structure, intact local-source
    retention and absence of a public role/stage query promise. Verify explicit public projection
    scope versus existing detail-only observation/settlement results: private finite facts/loss only,
    no original-cause/downcast promise or diagnostic-derived classification. Missing/misleading docs fail this
    independent Analysis even when runtime transport assertions pass; no rendered docs are claimed
    before CODE supplies them. Also require the public docs to identify the authenticated-site
    integrity exception: get_ref/downcast to KaniCauseMetadataIntegrityError distinguishes actual
    local InvalidData/no-errno integrity from genuine original payload-free or local custom causes;
    original producer kind/source is unknown for integrity, and the loss marker is absent. Missing or misleading
    integrity semantics fail the independent docs Analysis. Explicitly require docs to distinguish
    required sender encoding/checking failures from valid original boxed operational categories,
    including the disjoint typed detectors, the single finite capture/check/representation inventory
    (including non-installation dependency and policy-site mismatch), actual checker provenance
    and the fail-closed fault versus unsatisfied CODE gate.
    These public duties apply only to existing I/O-cause carriers; detail-only results retain private
    finite facts/loss without a public original-cause/downcast or stage-query promise.
    Mutants normalizing an actual Other kind to OutOfMemory, erasing OS errno, substituting a typed
    category/site, promoting a new unlisted variant, dropping/misapplying the public marker or
    integrity type, classifying genuine original InvalidData as integrity from source presence,
    erasing site-first admission mapping because of kind, omitting loss metadata or replacing an original
    cause after diagnostic failure must fail the corresponding kind/provenance/custody oracle.
    Changed/absent bounded diagnostic text leaves the typed mapping unchanged. Wrong run/build/role/
    phase, malformed or overlimit controls refuse without Dispatch/evidence and retain real owners.
    Restored controls pass. Unavailable genuine cases remain UNRUN with zero Test credit; no invented
    seam or artificial allocation failure. Ordinary production transport cases belong to Slice 1;
    exact-boundary controls unavailable there remain owed under Slice 2/IR-655 SPEC-before-fixture-CODE.
    No old fixture/source-Analysis row backs AC-40 or supplies whole-criterion completion credit.

The following AC-40 public-consumer matrix is PLANNED/UNRUN. It allocates the named controls within
procedure 27, not tests for artificial errors or implementation credit. For each available genuine
I/O-cause-bearing case, the ordinary public consumer shall use typed get_ref/downcast detection; changing diagnostic
text or testing source presence alone cannot supply the result. Unavailable real cases retain zero
Test credit and their existing Slice 2 obligation.

| Actual cause/control | Public cause facts | KaniCrossRoleCauseLoss downcast | KaniCauseMetadataIntegrityError downcast | Classification/source obligation |
|---|---|---|---|---|
| Valid original raw OS error | Exact original raw errno and verified kind | None | None | Original site-first admission or other actual existing mapping; direct errno reconstruction, no wrapper. |
| Valid original payload-free InvalidData | InvalidData, raw errno None, no custom payload | None | None | Original site-first admission; no fabricated custom source. |
| Genuine local custom InvalidData | Original InvalidData/raw errno and original custom object | None | None | Original local mapping/source retained; get_ref Some alone is not integrity. |
| Genuine valid original generic no-errno custom source lost across roles, excluding required cause encoding/checking faults | Exact original named kind, raw errno None, authenticated finite private provenance | Some | None | Original site-first admission or actual existing mapping; loss type denotes no original source identity. |
| Finite typed public source required and representable under AC-39 | Its required original kind/errno and finite typed source facts | None | None | Stronger AC-39 duties retained; generic loss/integrity types shall not replace that source. |
| Required sender capture/check/representation fails with an actual predicate from the single CauseIntegrity inventory, even if wrapped as Other with a boxed payload; complete authenticated finite fault report delivered to an existing I/O-cause-bearing result | InvalidData/raw None, actual finite required-representation predicate and only independently authenticated optional role/site facts | None | Some | Actual integrity cause, not original replay; explicit site-authenticated Unavailable or non-site startup/protocol path. A report not delivered cannot supply this predicate. |
| Independently authenticated valid original EPERM with complete required metadata/context byte-array framing; optional diagnostic UTF-8 invalid/incomplete or diagnostic retention bound exceeded within whole-control cap | Original raw errno/kind unchanged; diagnostic text dropped | None | None | Original admission/cause mapping; optional rendering failure supplies no integrity predicate. Missing/invalid required field/framing instead uses actual metadata-integrity row. |
| Actual C-side non-installation dependency-domain or policy-site/cause checking/projection rejection | InvalidData/raw None, actual NonInstallationDependencyCause or PolicySiteCauseMismatch and independently known checker/site facts | None | Some | Existing I/O-cause-bearing site-first integrity mapping; no sender report or invented sender origin. A detail-only observation result instead retains this predicate privately. |
| Unknown, malformed or incomplete cause metadata; negative admission site/state independently authenticated | InvalidData, raw errno None, actual local integrity error with actual private predicate/provenance and decoder source when present | None | Some | MemoryMechanismUnavailable with exact actual admission context, code None/no evidence after settlement; public cause is integrity, never original producer replay. |
| Cause metadata fault without independently authenticated originating admission site/state; required ownership/protection remains established | InvalidData/no-errno, actual local metadata-integrity error | None | Some | Tool(KaniToolError::Io) with actual launcher path/error; typed cause remains detectable, no invented admission-site context or Dispatch/evidence. |
| Independent actual C-side ownership/protection check fails while a cause packet is invalid | That check's actual original cause/context, not a substituted metadata error | No generic loss marker on the local cause | None unless that check's actual original cause itself is metadata integrity; its real source identity is preserved | Existing Unavailable from the actual C-side check; false remote admission context/integrity-source substitution is forbidden. |
| Actual post-admission work/settlement observation/accounting failure, including actual required-representation or metadata integrity fault preventing observation | Existing MemoryObservationFailed { detail }; actual finite cause/provenance/loss/integrity facts private, detail diagnostic-only | Inapplicable: no public I/O carrier | Inapplicable: no public I/O carrier | Existing no-evidence result after settlement; no Tool/Unavailable reclassification, fabricated public cause/downcast or detail parsing. Independent protocol/settlement failures keep actual mappings/override. |

28. PLANNED/UNRUN (FR-034 AC-51 through AC-56). Use ONE off-default fixture operation and
    matched normal-library caller/helper. The CG CODE author supplies genuine source/bounds and
    runtime receipts; the independent CODE reviewer checks them. No synthetic I, helper override,
    new production pause or borrowed settlement allowance. Construction, precondition, oracle and
    cleanup facts are reported separately; unavailable capability is owed, never skip/pass.
    a. AC-51: positively acknowledge the real adopted worker under live I. Retain a complete
       ordinary O sample; release that worker to fork ONE child and require its own birth ACK.
       Validate BOTH identities, pin BOTH live and retain genuine authenticated I Completed before
       cleanup. Check each construction predicate explicitly. Membership alone is not birth proof.
       Separately, the existing AC-8/10 Test obligations own this runtime death predicate: after
       unchanged cleanup, require BOTH formerly live pins dead and actual I/M settlement before
       any outer escalation. Independently check the actual pre-escalation kernel state; later emergency cleanup
       cannot repair a failed death predicate. This Test remains PLANNED/UNRUN and independent of
       AC-51's birth predicate and AC-55's source Analysis; no partial assertion binds a mixed AC.
    b. AC-52: require actual retained-I pidfd poll-IN confirmation strictly before matching seal.
       Mutate ONLY its underlying call/result to fabricated IN while I is positively live;
       preserve independent kernel/source-check code unchanged; no stored record proves the call. Require genuine-operation
       rejection, not a test-created absent observation. Independently move actual poll after seal;
       require actual order rejection; stored ordinals supply no proof. Restore actual pre-seal poll and pass.
    c. AC-53: require actual retained M consuming Child wait/reap before matching seal.
       Mutate ONLY the underlying call/result to fabricated reap with actual Child unreaped;
       preserve independent kernel/source-check code unchanged. Independently move actual reap after
       seal. Require genuine-operation and strict-order rejection separately, then restored pass.
    d. AC-54: retain real original gate during I cancellation; require actual I-positive confirmation
       before gate close. Independently close early or omit actual confirmation with independent checks
       unchanged. Both must fail actual order despite no backend marker; restored order passes.
    e. AC-55: source Analysis shall exhaustively enumerate actual contained cancellation and
       settlement paths in the final implementation: claimed-I lease EOF/INIT exit or retained-INIT
       signalling, unclaimed bootstrap recovery/cancellation, and L/O outer cancellation/settlement.
       Require that observation/sample membership never supplies descendant kill-list authority.
       A membership-dependent teardown path fails this Analysis regardless of current sample timing.
       C lease close is not the actual kill/exit boundary, and O samples continue afterward; do not
       assert a child-absence-through-control-kill runtime precondition. Independently retain the actual AC-24 non-INIT-watcher adverse/restored Test; no outer cleanup rescue and
       no source-only completion or sampled-membership-mutant execution credit. The CODE author
       supplies actual final-source Analysis and the CODE reviewer accepts it separately from Test.
    f. AC-56: inspect published integrated source for actual I confirmation/M Child reap, writer EOF,
       immutable seal and final complete observation attached to successful report Commit. Check
       all later due ticks remain active. A final sample before settlement invalidates retirement.
       The CG CODE author supplies this Analysis; independent CODE reviewer checks before replacement.
    Preserve distinct O-origin inner-confirm/M-reap/outer-confirm observations and stronger confirmed
    whole-outer-tree termination; these independent owed facts are not supplied by copied inner records.
    Capture failures before emergency cleanup. Independently exercise ignored inner EOF under AC-24;
    no paired mutant or outer kill may rescue it. Retain every old assertion/adverse patch and
    FR-028-AC-21/FR-017-AC-24 trace obligation until the CODE reviewer accepts restored/mutant parity
    for every named predicate. Missing full obligations leave old mixed criteria untagged.

29. Behavioral evidence after certificate retirement (all PLANNED/UNRUN):
    a. AC-78: Exercise genuine original O Armed/build/run admission and retained actual rights. Substitute a wrong actual sender/run or unvalidated capability and require refusal/no positive advance with owned cleanup.
    b. AC-79: Exercise actual existing production control malformed/partial/forbidden-right refusals; attempt positive advance from EOF or copied event metadata and require refusal without Dispatch/report authority.
    c. AC-80: Receive genuine original authenticated I Completed. Substitute M exit, EOF, phase and constructed metadata independently; none supplies completion. No stored completion carrier.
    d. AC-81: Inventory all surviving actual simultaneous capacities/backing/native lifetimes before exposure; actual removed allocations alone lose their charges. Omit a required retained input and require existing refusal, not fabricated zero.
    e. AC-82: Inspect supported feature-off source/artifacts and mechanical paired consumer/helper checks for absent certificate items and unchanged production frames/rights. Public operation absence alone is insufficient.
    f. AC-83: Independently leak an original lease alias into a real child and require the caller-death/EOF authorization predicate to fail. Restored exclusion and actual original lease close must pass while genuine M/I ownership remains retained.
    g. AC-84: Inspect actual child/exec/public mappings for every remaining reporter/control/report endpoint. Leak a genuine right and require actual exclusion failure.
    h. AC-85: Call through the actual retained M Child after a real consuming reap and distinguish cached Some from FIRST consuming Some under genuine parent/waiter history. A cache-only or fabricated-success mutant fails the consuming-operation predicate; no count or event proves it.
    i. AC-86: O alone, as actual same-M parent, establishes WNOWAIT waitability before FIRST uncached Child Some and same-child ECHILD after consuming reap, with original sole-waiter, SIGCHLD disposition/history. Independent verifier checks actual source/operation provenance, not its own non-parent ECHILD or copied history. Fabricated Some, cached Some, competing wait/auto-reap, live None and bare or unconsumed zombie status must independently fail. The restored genuine waitable-zombie control passes only with same-child prior WNOWAIT, actual FIRST uncached consuming Child Some and independent post-ECHILD under established original parent/sole-waiter/disposition/history premises. Safe method/storage/profile proof remains owed.
    j. AC-87: Independently observe actual I/M-before-report-seal and I-before-gate-close. Move real operations after seal or close gate before actual I confirmation and require failure. Stored ordinal/receipt time cannot repair order; unavailable construction is UNBACKED.
    k. AC-88: Inspect and exercise every due complete ordinary accounting sample, control/cutoff transition during normal and cancellation progress. A suppressed tick or wait for record permission fails independently.
    l. AC-89: Require genuine I Completed/original stop adoption before real original lease close. Move genuine reception/adoption after close and require failure; no copied carrier fill establishes order.
    m. AC-90: If a remaining optional fixture write exists, exercise real full/EPIPE/error under independently proven signal profile. It cannot terminate O, change controls/cancellation or rescue ignored EOF. No retired pipe is recreated to satisfy this check; absent write gives no signal Test credit.
    n. AC-91: Run actual ordinary matched feature-on execution without retired binding/endpoint and require unchanged availability. A required absent envelope or default identity fails.
    o. AC-92: Require genuine adopted-orphan ACK and release-dependent child ACK after an actual complete ordinary O sample, BOTH live identities/pins then BOTH dead before outer escalation. No announcement certificate/paused tick. Missing genuine coordination remains UNBACKED, not skipped-green.
    p. AC-93: Supply separate exact supported feature-on paired-artifact native workspace Analysis for every surviving named allocation and overlap, compare its independent finite charge. Remove terms only with actual allocation; missing proof remains UNPROVEN.
    q. AC-95: Independently substitute report backing under actual O collection/sealing/delivery and require the original-report identity predicate to fail. The restored actual retained backing and final descriptor identity pass without a new early field/right.
    r. AC-96: Exercise genuine missing/malformed/wrong original O/build/run/control authority and require refusal/no positive advance plus actual received-right cleanup under original cutoff; a default-identity admission or leaked-right mutant must fail.
    s. AC-97: Independently check actual original report backing/seal/delivery authority. Omitted actual equality or foreign report must fail; retired echo cannot substitute.
    t. AC-98: Compare actual feature-off consumer/helper phase bytes/rights and absent retired field/branches; ordinary feature-on execution requires no endpoint. Unknown artifact/source evidence blocks completion.
    u. AC-99: Apply the conditional-safety checks below only to a remaining existing optional O write; no writer activation or signal policy is allocated.
    v. AC-100: Apply the ineligible-profile no-write controls below, with pending/ordinary-flow preservation. Without genuine available control construction this Test remains UNRUN.
    Retain AC-51–56, AC-24, BOTH live/dead pins before escalation, separate actual outer confirmation
    and original FR-028-AC-21/FR-017-AC-24 assertion/mutant obligations. Each unavailable predicate
    remains UNBACKED with zero completion; no source/schema/method row replaces runtime proof.

### Charged-peak evidence constructors (FR-034-AC-32)

This additional Test and source-flow Analysis procedure is PLANNED/UNRUN. It does not
claim the current constructors or ledger integration satisfy the amended obligation.
Audit every emitted evidence constructor and both single-run and compatible batch callers;
require mandatory `charged_peak: ChargedPeakObservation` propagation without a default.
Exercise each evidence-producing case below through the real bounded execution flow and
compare against independently retained complete O charge observations. Use a genuine measured
sample as the positive control, and reject substitutions of zero, the ceiling, backing reserves,
a C-only RSS probe or a partial/failed observation. Preserve the existing outcome and settlement
oracles; an execution error must emit no `KaniExecutionEvidence`.

| Constructor/caller case | Required charged-peak result | Independent adverse oracle |
|---|---|---|
| Single-run timeout before L/O creation: zero/already-expired original deadline, or finite workdeadline elapsed with original T still live | TimedOut with `NotObserved { reason: PreRoleTimeout }` after applicable confirmed settlement | No new L/O or Dispatch after either cutoff; AC-20 original zero/expiry ordering unchanged; no manufactured observation or new reason |
| Single-run startup timeout after role creation, before Dispatch and before the first complete O sample | TimedOut with `NotObserved { reason: StartupTimeoutBeforeObservation }` after required settlement | A role-created startup timeout must not be mislabeled PreRoleTimeout |
| Single-run startup timeout before Dispatch with a prior complete actual O sample | `Observed { bytes }` using the actual complete peak | Reject NotObserved and any incomplete/proxy charge |
| Every post-Dispatch single-run evidence conclusion, including timeout, memory exhaustion and completed classification | `Observed { bytes }` from complete actual O observations | Missing complete measurement is execution error with no evidence, never a new absence reason |
| Compatible completed batch member evidence | The same authenticated whole-run `Observed { bytes }` propagated to each member | No per-member invented charge, optional field or default |
| C capability/availability failure; failed/expired ledger without an admissible measured conclusion; capture/refusal/settlement errors; whole-batch timeout/resource refusal | Existing execution error with no evidence | No invented NotObserved reason or evidence merely because an internal bounded-launch value exists |

The constructor audit shall distinguish startup admission from dispatched execution and trace
error conversion before evidence construction. A ledger deadline error alone does not establish
an evidence-emitting path or prove no prior sample existed. Retain an actual complete prior peak
when the existing timeout conclusion legitimately emits evidence; otherwise refuse as required.
No new public request field, observation schedule override, fallback or evidence kind is allocated.

Also exercise a pre-role workdeadline stop with a positively observed original deadline still live.
Require unchanged TimedOut candidate classification and confirmed settlement by original T;
unconfirmed settlement retains the existing CleanupUnconfirmed override with no evidence.
This case shall not require original expiry or role creation to select PreRoleTimeout.

The additional accounting Test and source-flow Analysis is PLANNED/UNRUN. Audit the complete charge
as all named formula terms: positively observed L/O/M/I and every owned descendant RSS, the declared
finite own caps of C per-run controls/captures (including bounded diagnostic detail), actual reserved
pipe capacity and pre-reserved memfd maximum. Require actual complete O setup/tick measurements and
the unchanged schedule. Independently omit L RSS, a named caller buffer, pipe reservation and memfd
reservation in separate mutants; each omission shall fail its actual charge/observation oracle.
Missing a named cap/quantity or owned-worker observation shall refuse, never become an incidental
allocation exclusion. Incidental caller-process allocations beyond the named terms, including
opaque Command/thread/native-runtime/TLS/guard/alternate-stack/allocator transients, are outside this
resource claim; no test shall claim their complete bound or use requested stack/whole-C RSS as its
proxy. The exclusion shall not remove L, named caller buffers or backing reservations, and shall
not relax ownership, writer closure, EOF, seals, ceilings, lease or original deadlines.

### Cross-namespace named accounting inputs (FR-034-AC-32)

This independent Test and source-flow Analysis is PLANNED/UNRUN. Positively establish that actual L
is not directly visible in O's fresh private proc, while O still obtains actual fresh L RSS at setup
and EACH original accounting tick from its selected authenticated source. Require original run
binding, actual owned L pidfd, recorded start/TGID identity and checked liveness. Independently
substitute a foreign/stale source, replay another run's observation, omit an input or break L
pin/identity/liveness binding; each must refuse before writer/Dispatch or cancel under existing
observation rules. Missing live L RSS shall never become zero or omission without actual
identity-matched MM-release proof. Compare against an independent real L observation through the
harness's retained owned identity; no backend-provided scalar or ambient PID lookup supplies the
positive control. The selected Safe observation implementation shall be feasibility-tested first;
this procedure allocates properties and does not mandate a particular transport or descriptor count.

Trace C's named per-run control/capture/diagnostic caps through authenticated C-origin authority
bound to the original run/ceiling/deadline. Require actual finite own caps and checked arithmetic;
a stack floor or C RSS alone does not establish the named buffer bound. O shall combine that bound
with fresh authenticated L/private-tree RSS and actual pipe/memfd reservations at setup and every
tick. Separate missing/underdeclared named-cap, wrong-run, omitted-L and missing-backing mutants
must fail their named oracle; complete within-ceiling setup sampling precedes writer exposure and
Dispatch. Unavailable L observation after admission shall yield existing MemoryObservationFailed
execution error without evidence, never a mid-run NotObserved; unavailable observation capability
before Dispatch retains typed unavailable admission. Verify no host observation/control authority
is reachable by arbitrary backend/descendants and O retains required private proc/PID isolation.
Runtime evidence must cover later ticks and final settlement; bootstrap success/one sample alone
is insufficient. All failure checks preserve existing classification, original timer and owned
cleanup obligations.

### Supported-build named native workspace (FR-034-AC-41 through FR-034-AC-50)

These Analysis procedures are PLANNED/UNRUN. Completed Analysis count is zero until
CG's CODE author supplies an operational receipt and the independent CODE reviewer
checks it. A computed method-without-symbol matrix row is not executed evidence.
The receipt shall give a completed or UNPROVEN disposition for each of AC-41..50;
IR-639's single CODE PR must satisfy this gate for its claimed initial configuration
before merge. Runtime observation/settlement Tests remain independently required.

1. Identify the actual final consumer executable input: consumer workspace manifest,
   package and binary target, matched helper, resolved workspace profile/features,
   target/ABI, selected compiler, flags/linker/panic strategy, allocator and actually
   loaded runtime configuration. The initial planned luna Linux x86_64 GNU input
   requires explicitly resolved release thin LTO/one codegen unit and feature-off
   production. Base `make build` builds a library, not this executable; dependency
   profiles/toolchain files cannot select the consumer workspace's settings.
2. Use GNU objdump or llvm-objdump on the delivered final linked executable and
   relevant loaded libraries after LTO/linking. Record actual tool/version in the
   operational receipt; reject nightly -Z/RUSTC_BOOTSTRAP and pre-link substitutes.
   Map named source operations to final address ranges using symbols/debug/inlining
   information; conservatively include a whole containing host frame when inlined.
   Unresolved final root attribution fails the claim.
3. Identify every thread entry root carrying named state, including indirect starts.
   Enumerate all reachable call targets through success/refusal/cancellation/error/
   cleanup paths. Unknown indirect targets and reachable recursive cycles leave the
   claim UNPROVEN. Derive per-thread live-frame maxima with actual tail replacements
   and stack adjustments, then sum all concurrently live instances. Unknown instance
   count/lifetime cannot be filled by one global maximum or a requested stack size.
4. Cover actual panic unwind/landing-pad/cleanup paths or prove absence under abort.
   Bound external frames using actual loaded machine code or authoritative finite
   bounds with matching ABI/runtime/configuration premises. Unknown dynamic stack,
   external bounds or loaded runtime selection leaves the claim UNPROVEN. Preserve
   unrelated incidental runtime/TLS/guard exclusions; named state cannot be excluded.
5. Inventory actual returned Rust Vec/String capacities times element size, named
   metadata and overlapping retained/temporary lifetimes. Larger returned capacity
   defeats requested-capacity accounting; allocator chunk overhead is excluded by
   the explicit FR-034 allocator-overhead boundary. Inspect the independently declared
   finite private native-workspace charge in actual caller_run_buffers before L/exposure,
   and prove the checked required bound <= declaration. Carrier implementation is an
   IR-639 CODE gate. Computed proof must not be written back into the analyzed binary;
   changing a declaration requires rebuilding and reanalyzing. Missing named charge
   uses the existing runtime refusal; missing proof only fails conformance, not runtime
   admission. No numeric reserve or new public carrier is allocated.
6. Invalidate the receipt after source, consumer configuration/compiler, executable,
   loaded runtime/allocator or external-bound premise changes. Require a new Analysis
   before claiming conformance; record unresolved facts and no completion credit.
   Keep exact build attribution private/operational, with no committed pin catalog.

Expected pass: complete current evidence accounts for roots, threads, every required
path and named quantity and proves the bound fits the independently declared charge.
Expected failure: a missing/stale premise, unresolved path or exceeded declaration
leaves the configuration UNPROVEN. Missing proof is not a new runtime refusal.
Other candidate native policies/configurations remain UNPROVEN until their own proofs.

### Pre-Armed negative capability checks

All procedures are PLANNED/UNRUN; they supply no prior-row execution credit.

1. Genuine pre-L-Armed O failure: inspect actual O→L zero-right frame and same L→C bytes with one
   actual retained clone. Authenticate actual ChildPID UID/GID0, L chain, build/run/source state and
   producer stamp independently. Substitutions, missing/extra rights and wrong state must refuse.

2. Prove actual open-file-description identity with admitted KCMP_FILE/equivalent kernel oracle;
   independently open a pidfd to the SAME O and to another positively owned actor, and require both
   clone negatives to fail. Inode equality never substitutes. Unavailable oracle is UNRUN/zero
   evidence credit, not passing skip or new runtime capability mandate.

3. Exercise L-local waiting arm versus fully accepted arm. A partial O→L Armed followed by O
   failure/EOF cannot authenticate Armed or splice a later negative; L retains actual Child/pin
   containment. After L acceptance, fail zero/partial direct O→C Armed: C closes a pidfd received
   with unauthenticated partial bytes and gains no signal or termination-observation authority from
   it. Require partial-as-positive mutant to fail. Original cause may be unavailable; no L post-
   Armed negative duty or renewed window.

4. While C AwaitArm and L still preArm, deliver actual directO EOF before full L-forwarded negative.
   C must preserve full authentic original cause/clock/capability if route completes by existing
   cutoff; EOF alone, malformed/partial/missing negative and deadline expiration never grant
   admission or extend window. Independent existing stream cursors/ancillary custody remain intact;
   no splice/new ACK/reset.

5. Inspect allowed cleanup capability operations versus prohibited
   pidfd_getfd/setns/process_madvise/numeric reopening/foreign-actor authority. Preserve actual
   Childwait at L. Delay wait under finite and absent work deadlines: original producer stamp
   adopted before wait; earliest original allowance never restarts. Ready pidfd without actual whole
   chain fails. Unconfirmed chain produces existing CleanupUnconfirmed/no evidence.

### Claimed-startup negative transaction checks

All procedures are PLANNED/UNRUN; they supply no prior-row execution credit.

1. Establish genuine typed producer failure first, then obtain Exhausted in the mandatory fresh
   complete pre-send tick BEFORE ANY emitted byte. Require retirement of offset-zero unpoisoned
   Failure into existing OwnerStop. Repeat exhaustion-before-failure, no resource-default/history
   substitute, and unchanged resource/deadline-over-malformed-report precedence. Resource
   establishment is complete named-input checked-sum evaluation, not sampling start/backdated
   receipt. Exercise eligible original work-deadline expiry and actual phase/site refusal beneath
   selected owner stop; a prior stop that ended work does not create a later timeout. Exercise
   successful nonblocking zero progress separately from offset-zero send/EOF/deadline/progress
   errors: only the former is unpoisoned and eligible for retirement; damaged sender cannot reset or
   renew the cutoff.

2. Exercise real zero-progress, partial and fully emitted terminal sends. First byte fixes
   candidate; partial/full frame cannot retire/splice/rewrite/append second terminal. Full
   authenticated negative receipt only provisions original cause/clock and closes real I lease, not
   evidence or settlement. Every due tick continues.

3. After first byte, produce a genuine fresh complete Exhausted tick while the real C lease remains
   positively open. Require actual retained claimed-I cancellation and separate retained-M
   cancellation/reap to begin on that tick under the original cutoff, before waiting for C lease
   closure. A wait-for-lease mutant must fail. Deliver the authentic O frame while AC-77-caused I death or
   lease EOF is observed: C must complete authentication by the original cutoff without transport
   preemption of the original cause; partial/invalid delivery and unconfirmed settlement still
   refuse with existing CleanupUnconfirmed precedence. Such termination supplies no lease-EOF
   witness. Missing ordinary actor/order witness remains owed; no
   fake tick or signal trace. Separately perform AC72 Analysis of OuterRunOwner.sampling and the
   actual ledger/history/owner_stop lifetime through true settlement; no runtime retention credit
   from holding settlement pending. AC73 independently requires actual positive I termination,
   separate M reap, writer EOF, final due accounting and whole chain before normal original-negative
   settlement, never public Success.

4. Separately produce actual later observation/collector/transport error. Require genuine owned
   later error through bounded cleanup and abnormal/absent normal O Code0 acceptance, selecting
   existing CleanupUnconfirmed rather than projecting original failure as final. No diagnostic
   parsing, fake cause, extra frame/field/window.

5. Establish actual complete observation history before failure, then fail a current tick. Absent
   FailureHeader peak field is neither measured zero nor no- history/current-validity. History does
   not repair failed current input. Source Analysis explicitly records missing claimed negative
   path, current guard, current partial exhaustion/error loss, and required Published-versus-Settled
   owner state; no implementation/coverage claim.

### Queued claimed-phase cleanup checks (FR-034-AC-94)

This standalone Test and source/custody Analysis is PLANNED/UNRUN. Use the genuine original C owner,
matching helper and original startup cursor. Establish the expected complete authenticated O
InnerClaimed with its actual received right before one committed OperationalFailure. Exercise both
I-first death/lease EOF and M-first cancellation with I still live, using existing retained validated
observations; raw unvalidated I rights and packet labels supply no death or cancellation authority.
Require the defined C-observable admission condition, close-only ownership before the next fallible
step, then SAME-cursor ordinary finite steps through partial-then-complete authentic negative
receipt. Preserve due accounting, scheduling, original C stop/cutoff and existing storage. Require concurrent
urgent owned cancellation/cleanup through existing authenticated authority; receipt cannot postpone
cleanup, add lease grace or grant raw-right signal authority. Never infer cancellation cause from dead I/M, and never accept a partial following frame.

Require actual raw-right close on every exit and no positive identity/phase, gate release, Dispatch
or signal authority. Independently inspect that lifetime and close. A leaked/never-closed raw-right
mutant must fail before later cleanup hides it. Positive live identity admission remains unchanged;
independently substitute wrong start/namespace/network/parent while original I/M remain live and
require existing identity refusal, not pending receipt. Wrong prior phase/sender/run, malformed or
partial claim and extra rights must refuse. An any-admission-failure-enters-pending mutant must fail.
Deliberately exercise excluded GateReleased and poisoned/ambiguous send under existing fail-closed
rules, without extending this Startup-only exception.

Preserve authentic partial delivery that becomes complete by the original cutoff. Independently
exercise absent following Failure with actual O EOF/death, expired cutoff, malformed/invalid or
unexpected following grammar: reject immediately on the defined observable EOF/death/grammar fault,
or at the original cutoff for silent O, with existing CleanupUnconfirmed/no evidence. I/M death
alone must not preempt later valid committed delivery. A complete negative still requires actual
I confirmation, separate M reap and real O/L/capture/creator/control-EOF settlement before exposing
its original cause; an unsettled-chain control keeps CleanupUnconfirmed/no evidence. Source Analysis
follows the one cursor, fixed pending records/raw-right close and original clock on every path;
phase-as-admitted and clock/cursor-reset mutants fail their named authority/deadline predicates.

Missing genuine schedule/observation construction leaves the Test owed through IR-655
SPEC-before-fixture-CODE. The CG CODE author supplies actual source/bounds and runtime receipts;
the independent CODE reviewer checks them before CODE acceptance. No synthetic actor, new fixture
hook, sampling pause or borrowed capacity proof is supplied. Analysis or a matrix method row never
completes this Test or binds the whole mixed criterion.

### Conditional signal-safety checks (FR-034-AC-99/100)

These checks remain PLANNED/UNRUN and apply only to a remaining existing optional O fixture write.
The retired certificate pipe/early-field/binding shall not be recreated or activated. With no such
write, actual retirement/absence is inspected but signal Test completion is not claimed.

a. Establish actual normal-helper O ignored/unblocked SIGPIPE, single-thread/no competing consumer,
   stable disposition/mask/tracing state, authenticated same-O complete proc identity/task/pending
   facts and independently checkable INITIAL untraced evidence across PID namespaces. Private
   TracerPid zero, inherited default, one snapshot or external probe alone cannot prove eligibility.
   Inspect complete kernel/syscall/runtime permissions and actual retained/transient/native overlap
   charges. Genuine live/broken-reader controls preserve pending signals and ordinary flow; omit
   actual O acquisition, use a foreign profile or snapshot-only proof and require rejection.
b. Genuine default/caught/blocked/traced/multithreaded/changing/unproved profiles, missing/malformed/
   duplicated/truncated identity/proc facts and query/permission failures permit no write. Include
   actual ancestor-PID-namespace tracer invisible to O's private TracerPid: refusal or UNPROVEN/
   no-write is required. Missing safe construction remains UNRUN; no hook/query/signal policy is
   granted. Write-on-unproved-profile and pending-drain mutants fail independently. No profile or
   write error may rescue ignored inner EOF via O exit/cancellation or later cleanup.

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
| FR-034-AC-23/24 | Real helper, genuine preclaim Bootstrap ownership limits distinguished from separate cleanup Analysis, positive claimed-INIT pins and production typed lease-close boundary before INIT escalation | Ignored EOF keeps worker or accepts closed-lease Dispatch; emergency teardown masks failure |
| FR-034-AC-27/28/29/30 | Opt-in single fixture with genuine independent pre-escalation kernel predicates, unconditional cleanup, same normal artifacts and separately owned driver exclusion; retired stage/ordinal/record certificates supply no proof | Default export; controller pause; changed production stage; false oracle after cleanup; cfg-test epoch override; feature enabled in production |
| FR-034-AC-31 | Actual outer pin and whole-tree termination at exact bwrap handoff failure; ordinary caller capabilities and creating-thread lifetime observed | Old295 orphaned unclaimed INIT; profile-only availability assumption; parent-thread exit misclassified as success |
| FR-034-AC-32 | Hard writer retention bound and defined conservative charge; actual Kani pipe roundtrip; slow/overflow cases stop under original deadline | Unmapped backing counted as zero; resizable pipe exceeds reservation; backend/collector deadlock; report cap becomes Failed or truncated pass |
| FR-034-AC-33 | O spawn writer closed, M and inner writers terminated, actual EOF, verified four seals and final descriptor read; final-close backing reclamation | Extra monitor writer prevents EOF; forged seal claim; report residue after all owners die |
| FR-034-AC-34 | Separate lease/report controls and live C retained outer ownership; ignored-EOF fails before outer escalation | Outer kill masks mandatory EOF mutant; deadline/ceiling reset or missing charge accepted |
| FR-034-AC-35 | PLANNED/UNRUN: safe same-PID backend-only installation, L/O/M allocation, real unconfined shared-prefix control and dynamic listener exclusion; actual Cargo/Kani compatibility; typed unavailable refusal and original build/resource classes | Policy bypass or inherited ring; peer socket created after Dispatch; unbound vacuous listener; unfiltered release or compatibility relaxation |
| FR-034-AC-36 | PLANNED/UNRUN: real socket stdin refuses, production fd1/fd2 capture inventory verified; explicit Closed and Open inspection error distinguished; mandatory typed cause and no fabricated result | Unreachable stdout socket test claimed; Open EBADF admitted as Closed; caller-fixable socket misreported as memory-only failure |
| FR-034-AC-37 | PLANNED/UNRUN: I non-dumpability/backend privilege exclusion block proc1fd/pidfd_getfd/ptrace; trusted channel lifetimes and unchanged EOF mutant retained; uniform authority fault domain | Ambient Yama masks absent protection; contained export excused as outside peer; final delivery closed early |
| FR-034-AC-38 | PLANNED/UNRUN: actual normal role/capture/spawner settlement; original-window typed CleanupUnconfirmed error with bounded diagnostic-only residuals, no new post-return custody; public precondition docs | Error carries authority; hidden cleanup worker; false joined/retired or kernel-cause claim; valid bytes accepted after unconfirmed settlement; grace resets T |
| FR-034-AC-39 | PLANNED/UNRUN: source/cfg Analysis of unsupported native support and unconditional native-x86_64 x32 rule; real available installation Tests, typed provenance/capability distinction, applicable process-kill/effect controls, omission mutant, original settlement/classification | Source Analysis called an executed Test; errno-only denial instead of process-kill; omitted x32 rule on a kernel without x32; unavailable fixture counted complete; emulator mistaken for guest syscall evidence; false pre-Dispatch label or unfiltered retry |
| FR-034-AC-40 | PLANNED/UNRUN: exhaustive actual sender/build named-kind source Analysis before delivery; genuine cases preserve site-first admission, actual private kind/errno, independently authenticated optional provenance and loss; typed public projection applies only to existing I/O-cause carriers, including valid original boxed declared operational categories exclusively outside required encoding/checking faults; fail-closed unexpected encoding faults never satisfy the supported producer gate or normalize an original kind; complete required-representation fault reports identify actual sender predicates with integrity type and no loss marker only in existing I/O-cause-bearing results, never invented undelivered reports; post-admission observation/settlement detail-only variants remain unchanged with private facts/loss and diagnostic-only text; authenticated negative-site unknown/malformed/incomplete cause metadata returns MemoryMechanismUnavailable with exact site context and actual local InvalidData/no-errno KaniCauseMetadataIntegrityError detectable by get_ref/downcast (no loss marker), never original replay; unauthenticated site uses its specified startup/protocol path; diagnostics cannot replace it; public rustdoc states carrier/detail-only scope, both typed detectors, sender encoding fault versus valid original cause and no original-source/stage-query promise for the projections; settlement remains required | Unproved/unwired sender domain; kind-driven Tool normalization; new emitted kind without encoder support; marker missing on opaque no-errno loss or attached to loss-free OS/payload-free/local/stronger AC-39 source; erased errno; fake replay; authenticated integrity fault sent to Tool; invented integrity admission context; false loss/integrity type; genuine original InvalidData misclassified from kind/source presence; missing actual local source; diagnostic replacement; misleading docs or false prior-test credit |
| FR-034-AC-41 | PLANNED/UNRUN Analysis: Exact consumer manifest/package/bin and resolved workspace configuration; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-42 | PLANNED/UNRUN Analysis: Final linked executable and loaded-library GNU/LLVM disassembly with stable inputs; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-43 | PLANNED/UNRUN Analysis: Final address mappings for operation and thread roots, all indirect targets; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-44 | PLANNED/UNRUN Analysis: No reachable recursive call cycle; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-45 | PLANNED/UNRUN Analysis: Per-thread live-frame maxima summed for all concurrent instances; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-46 | PLANNED/UNRUN Analysis: Actual panic unwind/cleanup edges or demonstrated abort absence; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-47 | PLANNED/UNRUN Analysis: Finite external bounds matching loaded runtime ABI/configuration; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-48 | PLANNED/UNRUN Analysis: Returned heap capacities, metadata and overlap inside declared charge; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-49 | PLANNED/UNRUN Analysis: Reanalysis after source/build/compiler/runtime/artifact/premise change; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |
| FR-034-AC-50 | PLANNED/UNRUN Analysis: Bound fits independent declaration; missing proof is UNPROVEN, missing charge uses existing refusal; actual per-criterion receipt required | Omitted or stale required fact, falsely completed method row or runtime proof-availability classification |

AC-31 through AC-34 and rewritten FR-017 AC-19 are UNRUN and must compute untagged until actual
production assertions are implemented. Scenario prose or research probes establish no executable
coverage. Native refinement remains planned until its actual typed entry is delivered. Host
destruction and uninterruptible tasks cannot justify fabricated teardown; live-caller unavailable
confirmation refuses. Caller-group signals and direct guardian death are included lifecycle cases,
not excluded double faults.


### Negative startup transaction expectations (PLANNED/UNRUN)

| Criterion | Expected result | Independent adverse result |
| --- | --- | --- |
| FR-034-AC-57 | PLANNED/UNRUN: New O→L pre-local-Armed negative uses zero rights | Misrepresented existing O→L route or extra right |
| FR-034-AC-58 | PLANNED/UNRUN: L authenticates actual O Child PID/UID/GID0/build/run/source/state/stamp | Packet/socket-creator label substitutes for actual sender |
| FR-034-AC-59 | PLANNED/UNRUN: Invalid negative rights refuse with actual owner/capability custody | Zero-right fallback or leaked received right |
| FR-034-AC-60 | PLANNED/UNRUN: Exact O-only cleanup/source capability scope; real Child stays at L | Fictional C Child or authority beyond owned O |
| FR-034-AC-61 | PLANNED/UNRUN: No positive Armed/namespace/phase/report/Dispatch admission | Partial Armed interpreted as positive; created M/backend |
| FR-034-AC-62 | PLANNED/UNRUN: Original authentic producer stamp adopted before wait, earliest cutoff preserved | Receipt-time stamp or restarted None settlement allowance |
| FR-034-AC-63 | PLANNED/UNRUN: Final original cause only after actual O/L/capture/creator/EOF settlement | Ready pin or receipt used as final result |
| FR-034-AC-64 | PLANNED/UNRUN: No new envelope type/ACK/window/cap/public category | Uncharged extra control/capability state |
| FR-034-AC-65 | PLANNED/UNRUN: Fresh exhausting pre-send tick retires zero-progress Failure into OwnerStop | Earlier genuine failure wrongly defeats prebyte precedence |
| FR-034-AC-66 | PLANNED/UNRUN: L-local preArm same-byte/one-clone route | Wrong bytes, rights count or L-local state accepted |
| FR-034-AC-67 | PLANNED/UNRUN: C authenticates full L negative only AwaitArm through actual chain | Direct O EOF hides full authentic L negative or waives startup integrity |
| FR-034-AC-68 | PLANNED/UNRUN: No pidfd_getfd/pidfd setns/process_madvise/numeric reopening | Capability operation exceeds negative cleanup scope |
| FR-034-AC-69 | PLANNED/UNRUN: Unconfirmed or abnormal settlement keeps CleanupUnconfirmed/no evidence | Provisional original failure escapes settlement override |
| FR-034-AC-70 | PLANNED/UNRUN: First emitted byte fixes immutable single candidate | Partial/full frame retired, rewritten or spliced |
| FR-034-AC-71 | PLANNED/UNRUN: All due pre-send ticks and actual fresh complete named inputs | History/default repairs failed current observation |
| FR-034-AC-72 | PLANNED/UNRUN Analysis only: Actual O ledger/history/owner_stop retained through settlement | Analysis finds first-byte retirement or early release of the actual owned fact; no vacuous Code0 runtime credit |
| FR-034-AC-73 | PLANNED/UNRUN: Claimed I termination plus separate real M reap/writerEOF/finaltick precede O normal return | Full published failure treated as settled |
| FR-034-AC-74 | PLANNED/UNRUN: Absent peak field means no peak transport, no inferred zero/history/current validity | Fabricated zero or historical-current substitution |
| FR-034-AC-75 | PLANNED/UNRUN: Later owned observation/collector/transport error blocks normal O Code0 | Original negative projected as final despite later genuine error |
| FR-034-AC-76 | PLANNED/UNRUN: Admitted open-file-description clone oracle | Same-O independently opened or foreign pidfd accepted as retained clone; inode proxy used |
| FR-034-AC-77 | PLANNED/UNRUN: Immediate actual I/M cancellation on postbyte complete Exhausted tick with C lease open | Cancellation waits for C lease closure or uses a fresh allowance |
| FR-034-AC-94 | PLANNED/UNRUN: Defined pending claim stays close-only and its raw right is actually closed; ordinary SAME-cursor partial delivery can complete by the original cutoff, with authentic settled original cause and unchanged live positive admission. | Leaked/never-closed raw right; any admission failure or live identity mismatch enters pending receipt; wrong/damaged/extra-right prior traffic waived; prior right grants phase/gate/Dispatch/signal; I/M death preempts later complete delivery; invalid/missing Failure or unsettled chain escapes CleanupUnconfirmed; clock/cursor reset. |

### Stage-2 replacement expectations (PLANNED/UNRUN)

| Criterion | Expected result | Independent adverse result |
| --- | --- | --- |
| FR-034-AC-8/10 (Test, step 28a) | BOTH positively live descendant pins become dead with actual I/M settlement before outer escalation. | Surviving pin, missing genuine settlement or observation only after escalation fails; source Analysis and later emergency cleanup supply no Test credit. |
| FR-034-AC-51 | Child ACK proves release-dependent birth after actual ordinary sample, with explicit genuine construction preconditions. | Membership-only proxy or missing precondition fails. |
| FR-034-AC-52 | O's actual I-IN confirmation precedes matching seal. | Fabricated IN with real I live/independent checks unchanged fails provenance; actual late poll fails order. |
| FR-034-AC-53 | O's genuine M Child reap precedes matching seal. | Fabricated reap with Child unreaped/independent checks unchanged fails provenance; actual late reap fails order. |
| FR-034-AC-54 | I confirmation while original gate owned precedes actual close. | Early close and omitted confirmation independently fail; marker absence gives no order. |
| FR-034-AC-55 | Independently accepted final-source authority-path Analysis excludes sampled-membership kill lists; actual INIT death remains descendant authority. | Any membership-authorized teardown path fails Analysis; AC-24 non-INIT-watcher Test stays independently owed/unrescued, with no source-only runtime credit. |
| FR-034-AC-56 | Independently checked published-source final accounting after actual I/M/EOF/seal, with normal ticks. | Earlier final sample or missing Analysis invalidates retirement; no runtime Test credit. |

No Implemented row supplies these criteria's backing. AC-55/56 Analysis completion is zero until
actual independently checked evidence, never inferred from a matrix method row. Distinct outer-confirm and confirmed whole
outer-tree termination remain owed. Per-predicate receipts and original trace obligations require
independent CODE-review acceptance; all runtime Tests remain UNRUN.

### Behavioral retirement expectations (PLANNED/UNRUN)

| Criterion | Required restored result | Independent adverse result |
|-----------|--------------------------|----------------------------|
| FR-034-AC-78 | C admits actual original O Armed only through genuine retained O/build/run authentication and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission. | Exercise genuine original O Armed/build/run admission and retained actual rights. Substitute a wrong actual sender/run or unvalidated capability and require refusal/no positive advance with owned cleanup. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-79 | Actual production control reception rejects malformed, partial or forbidden-right traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies a missing authenticated control. | Exercise actual existing production control malformed/partial/forbidden-right refusals; attempt positive advance from EOF or copied event metadata and require refusal without Dispatch/report authority. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-80 | Genuine original authenticated I Completed alone establishes completion reception; M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate is retired. | Receive genuine original authenticated I Completed. Substitute M exit, EOF, phase and constructed metadata independently; none supplies completion. No stored completion carrier. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-81 | Every surviving actual simultaneous named fixture/control storage and pipe/backing reservation is charged before exposure; retired certificate storage is removed from charges only after its actual allocation disappears. | Inventory all surviving actual simultaneous capacities/backing/native lifetimes before exposure; actual removed allocations alone lose their charges. Omit a required retained input and require existing refusal, not fabricated zero. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-82 | Actual supported-source/artifact Analysis and mechanical paired consumer/helper checks establish feature-off absence and unchanged production frames/rights after certificate retirement. | Inspect supported feature-off source/artifacts and mechanical paired consumer/helper checks for absent certificate items and unchanged production frames/rights. Public operation absence alone is insufficient. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-83 | The original exclusive caller lease closes independently of retained monitor/INIT ownership and does not leak into children; a leaked alias cannot preserve authorization after actual caller death. | Independently leak an original lease alias into a real child and require the caller-death/EOF authorization predicate to fail. Restored exclusion and actual original lease close must pass while genuine M/I ownership remains retained. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-84 | Every remaining fixture reporter/control/report endpoint is excluded from untrusted child/exec mappings and public control handles through actual descriptor checks. | Inspect actual child/exec/public mappings for every remaining reporter/control/report endpoint. Leak a genuine right and require actual exclusion failure. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-85 | Actual retained M consuming reap is not established by repeated or cached Child Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success. | Call through the actual retained M Child after a real consuming reap and distinguish cached Some from FIRST consuming Some under genuine parent/waiter history. A cache-only or fabricated-success mutant fails the consuming-operation predicate; no count or event proves it. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-86 | O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD provenance under the original sole-waiter/disposition/history; independent checks reject fabricated, cached or missing consuming-operation facts without stored history as proof. | O alone, as actual same-M parent, establishes WNOWAIT waitability before FIRST uncached Child Some and same-child ECHILD after consuming reap, with original sole-waiter, SIGCHLD disposition/history. Independent verifier checks actual source/operation provenance, not its own non-parent ECHILD or copied history. Fabricated Some, cached Some, competing wait/auto-reap, live None and bare or unconsumed zombie status must independently fail. The restored genuine waitable-zombie control passes only with same-child prior WNOWAIT, actual FIRST uncached consuming Child Some and independent post-ECHILD under established original parent/sole-waiter/disposition/history premises. Safe method/storage/profile proof remains owed. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-87 | Actual I confirmation/M consuming reap precede report seal and actual I confirmation precedes retained-gate close; independent late-operation/early-close mutants fail. Producer ordinal certificates are retired. | Independently observe actual I/M-before-report-seal and I-before-gate-close. Move real operations after seal or close gate before actual I confirmation and require failure. Stored ordinal/receipt time cannot repair order; unavailable construction is UNBACKED. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-88 | O preserves every due ordinary sample and existing control/cutoff transition without observation ACK, pause, stored publication permission or suppressed tick. | Inspect and exercise every due complete ordinary accounting sample, control/cutoff transition during normal and cancellation progress. A suppressed tick or wait for record permission fails independently. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-89 | Genuine original I Completed reception and original stop adoption precede actual original lease close, without a separate stored completion certificate. | Require genuine I Completed/original stop adoption before real original lease close. Move genuine reception/adoption after close and require failure; no copied carrier fill establishes order. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-90 | Any remaining optional fixture write failure does not terminate O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer is retired, not activated by this condition. | If a remaining optional fixture write exists, exercise real full/EPIPE/error under independently proven signal profile. It cannot terminate O, change controls/cancellation or rescue ignored EOF. No retired pipe is recreated to satisfy this check; absent write gives no signal Test credit. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-91 | Ordinary feature-on execution remains available without retired observation binding, endpoint, event emission or default identity. | Run actual ordinary matched feature-on execution without retired binding/endpoint and require unchanged availability. A required absent envelope or default identity fails. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-92 | Genuine adopted-orphan acknowledgement and release-dependent child birth after an actual complete ordinary O sample remain required without a stored sample-announcement certificate; missing real coordination is UNBACKED. | Require genuine adopted-orphan ACK and release-dependent child ACK after an actual complete ordinary O sample, BOTH live identities/pins then BOTH dead before outer escalation. No announcement certificate/paused tick. Missing genuine coordination remains UNBACKED, not skipped-green. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-93 | Every surviving feature-on named native workspace has separate supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose certificate workspace retires only with its actual allocation. | Supply separate exact supported feature-on paired-artifact native workspace Analysis for every surviving named allocation and overlap, compare its independent finite charge. Remove terms only with actual allocation; missing proof remains UNPROVEN. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-95 | Actual O retains the original report backing identity through collection, kernel sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate. | Independently substitute report backing under actual O collection/sealing/delivery and require the original-report identity predicate to fail. The restored actual retained backing and final descriptor identity pass without a new early field/right. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-96 | Missing, malformed, wrong or unvalidated original O/build/run/control authority refuses without positive admission, retaining actual received-right custody and original cutoff; no default identity supplies authority. | Exercise genuine missing/malformed/wrong original O/build/run/control authority and require refusal/no positive advance plus actual received-right cleanup under original cutoff; a default-identity admission or leaked-right mutant must fail. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-97 | Retired observation-binding echo creates no replacement comparison; actual original report backing equality and delivery/settlement authority remain required. | Independently check actual original report backing/seal/delivery authority. Omitted actual equality or foreign report must fail; retired echo cannot substitute. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-98 | Feature-off paired consumers/helpers retain unchanged phase bytes/rights with no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding requirement. | Compare actual feature-off consumer/helper phase bytes/rights and absent retired field/branches; ordinary feature-on execution requires no endpoint. Unknown artifact/source evidence blocks completion. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-99 | Any remaining optional O fixture write requires actual ignored/unblocked/untraced stable single-thread proof, including independent across-PID-namespace initial untraced evidence, pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement supplies no signal proof or activation. | Apply the conditional-safety checks below only to a remaining existing optional O write; no writer activation or signal policy is allocated. No absent capability or retirement row supplies executed credit. |
| FR-034-AC-100 | Unsupported, malformed, unavailable or unproved profiles permit no optional fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or private-proc zero alone repairs eligibility. | Apply the ineligible-profile no-write controls below, with pending/ordinary-flow preservation. Without genuine available control construction this Test remains UNRUN. No absent capability or retirement row supplies executed credit. |

## Settlement reserve research receipt

This research records the basis for SETTLE_RESERVE = 1 second. It supplies no executable coverage
of AC-35 through AC-38, whose production Tests remain PLANNED/UNRUN. Actual integrated product
whole-tree settlement must be remeasured before CODE delivery; this result is not a real Cargo/Kani
roundtrip, matched production helper/protocol evidence, runtime acceptance or proof for arbitrary
or kernel-unkillable tasks. No probe script, binary, schema or foreign artifact is copied here.

The measurement used 200 trials (100 baseline and 100 with a single CPU burner), with controller,
trees and burner sharing one CPU, reduced scheduling priority and a nonblocking machinewide lock.
The representative topology used nested bwrap PID namespaces and Python outer/inner INIT roles,
a backend-like child, workers, grandchild, detached session worker and adopted orphan. Positive
readiness/parent/start/namespace checks retained owned host pidfds before signalling only the actual
outer INIT pidfd with SIGKILL. Monotonic timing ran from just before that signal until every retained
pidfd exited, its original host/proc identity was gone and the direct launcher was reaped.
Nearest-rank quantiles were independently recomputed from raw monotonic samples; combined P99 is
sorted rank198 of200. This method measures representative kernel-topology settlement, not actual
product helper/protocol or Cargo/Kani behavior.

| Group | Trials | P50 nanoseconds | P99 nanoseconds | Maximum nanoseconds |
|---|---:|---:|---:|---:|
| Baseline | 100 | 3511211 | 4811362 | 5077398 |
| Single-CPU load | 100 | 6386183 | 8010711 | 8251330 |
| Combined | 200 | 3925047 | 7909234 | 8251330 |

The integer whole-second derivation is
R = ceil(max(1,000,000,000 ns, 10 * 7,909,234 ns) / 1,000,000,000 ns) seconds = 1 second.
For a 200ms whole-run T, R_eff=min(1s,100ms)=100ms, which exceeds ten combined P99
(79.09234ms) and ten loaded P99 (80.10711ms) in this measurement. This numerical example does not
assure settlement for every tree, host load or smaller T.

All 200 positive cleanup confirmations covered 1853 per-trial retained identity records; each had
pidfd exit and original host identity disappearance. No emergency cleanup signal was used.
All known-good inner-member pairs accepted and known-bad outer/inner pairs rejected. The burner
was confirmed by waitpid of the original unreaped fork child, but its PID/start/pidfd facts were not
serialized; no independent raw burner-identity witness is claimed. The trees' raw per-identity
confirmations and burner control-flow/waitpid confirmation have different evidence scopes.

One earlier attempt failed its own preflight and is recorded privately. It produced no measurement
samples or derived reserve; its failed membership oracle is not evidence of failed kernel teardown.
A separate known-good/known-bad preflight confirmed nine retained identities exited/disappeared and
launcher reaped via normal payload exit, without measurement or emergency SIGKILL. Neither
preliminary attempt counts among the 200 measurement samples or supplies product acceptance.
Full receipts and provenance are retained privately; this public note carries only the method,
quantile values, derivation and limits. No additional execution or assurance claim follows from it.
