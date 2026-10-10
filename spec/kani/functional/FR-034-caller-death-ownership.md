---
id: FR-034
title: "Retain backend ownership when the original caller dies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
---
# FR-034: Retain backend ownership when the original caller dies

## Description

If the original caller dies, then the namespace guardian shall cancel its backend descendants. The
guardian shall prevent startup EOF from authorizing a production backend instruction.

This requirement extends the Linux PID-namespace containment delivered by PR #295.
[FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 retains its resource authority. The preceding
containment code is merged; guardian ownership and its verification remain planned. This requirement
does not depend on completing all parent IR-241 work, which IR-639 itself blocks.

Planned CODE: the selected IR-652 allocation adds a dedicated single-thread launcher and a trusted
outer PID-namespace INIT around the inner bubblewrap/guardian namespace. The merged PR #295 monitor
can die after clone but before its internal `child_wait` handoff, orphaning blocked inner INIT; a
startup-info reader failure can trigger the same window. Outer INIT's kernel-owned namespace closes
that process-containment gap independently of inner INIT claim. Fatal inherited parent-death signals
cannot execute report unlink code; FR-017 therefore replaces its internal named report with bounded
unnamed kernel storage rather than relying on a surviving cleanup owner. Neither gap is repaired by
this SPEC-only amendment.

Nine safe scratch scenarios measured nested mapping/private proc, outer PID-1 parent-death
containment, pinned inner identities and bootstrap EOF. They do not establish actual original
caller death/exclusive-lease races, production accounting, exact bubblewrap pre-handoff failure,
real report export or all-owner storage reclamation. Those remain mandatory CODE gates below;
research success is not executable requirement coverage.

## Inputs

- The existing bounded execution request, original harness identity, backend command recipe and
  recorded ceilings, including the original monotonic wall deadline.
- A required explicit executable path to `quire-kani-guardian`, built with the library from the same
  Cargo package source and build inputs. A library consumer deliberately builds and supplies this
  artifact; Cargo does not build or install a dependency's binary for it.
- A private per-run anonymous connected Unix-stream pair carrying typed bounded control and the
  original caller's exclusive liveness lease, plus separate bootstrap/final-report controls and
  separately owned backend stdin.

## Outputs

- For a live original caller, the existing typed backend outcome or typed setup, observation or
  cleanup refusal, with existing evidence semantics.
- For a dead original caller, namespace cancellation and guardian cleanup; no fabricated run result
  or execution evidence attributed to that caller.

## Behavior

The **bounded executor C** is the original caller. Its actual unreaped Child is the dedicated
single-thread **launcher L**. L creates the outer user/PID/private-mount/private-network namespaces
and spawns the trusted **outer supervisor O**, actual outer PID 1. O owns the actual unreaped bubblewrap **monitor
M**; bubblewrap creates **guardian I**, actual inner namespace PID 1. C retains positively
authenticated host pidfds/identities through this C→L→O→M→I chain; I remains M's inner child, not
C's fictional direct Child. One ownership set covers the entire FR-017 batch, with two additional
process roles L/O, private proc/mount setup and bounded control/descriptor state. All roles,
startup, collection and kernel report backing remain inside the original FR-028 ceiling/deadline
allocation. M retains the existing `process_group(0)` separation. O and I establish distinct
sessions/groups outside C's terminal/session before production authorization.

### Outer containment and capability allocation

The original spawning C thread shall remain live until L is settled; synchronous execution shall
retain that thread, and an asynchronous adapter shall retain a joined dedicated spawner through the
run rather than return its thread to a pool. PR_SET_PDEATHSIG observes the actual creating thread,
not merely C process lifetime. Unexpected spawning-thread exit while C lives shall produce typed
ownership/setup failure with confirmed owned cleanup, never a proof from successful output. L shall
arm its actual C parent-death signal and positively verify original-exclusive bootstrap liveness
before namespace setup. Credential/mapping changes require rearm plus parent/liveness revalidation
before O creation. O shall receive only its bootstrap endpoint, become actual PID 1, arm and verify
parent-death protection, and positively confirm its original authority before ANY bwrap spawn.
Before that arm, there is no inner child: L loss closes the exclusive bootstrap writer, and bounded
EOF refusal ends O. Arm failure or absent/replayed authority cannot create M. After arm, L death
kills O and the kernel tears down every outer-namespace descendant, including an unclaimed I and
nested/escaped backend children. O death itself has the same kernel backstop; I death tears down its
inner descendants. A live C confirms positively owned pinned termination before conclusions;
unavailable confirmation refuses. No reused PID, host scan, monitor exit, signal success or inner
gate EOF supplies authority or teardown proof.

Setup requires caller-context permission for safe NEWUSER/NEWPID/NEWNS/NEWNET, verified UID/GID
mappings and setgroups policy, private mount identity, private propagation and fresh outer proc view, nested
installed bwrap, safe pidfds/descriptor mapping, and original resource-observer readiness. L/O/I
are first-party role entries of the explicitly supplied matching package helper; each role uses
that same verified source/build identity, with no extra PATH executable or copied helper. Before
ANY mount, L shall verify NEWNS succeeded and mount identity differs from the original host
namespace; only that private namespace may change propagation or mount proc. O's proc PID view must agree with its
namespace and bwrap's identity lookups. Namespace/mapping/profile or identity failure gives typed
pre-Dispatch refusal without host-policy/service/configuration mutations or weaker mode. The
compiled caller's inherited security profile is a capability prerequisite, not permission inferred
solely from the bwrap binary. This is an availability cost: a caller profile denying L
user/private-mount namespace creation refuses runs that previously succeeded when bwrap alone had
permission. Scratch success under `codex-node (unconfined)` does not prove ordinary-unconfined
caller availability on hosts with restricted unprivileged user namespaces. A real ordinary-caller
permission measurement is required before CODE, with actual inherited label and errno, no profile
workaround or policy mutation. All creator/L/O deaths and mapping/arm races require exact positive
barriers and bounded owned cleanup; no timing-only assertion proves liveness.

Before O or M creation, L shall establish the private network namespace and the confined outer root.
O shall verify the actual private network and root/mount allocation before spawning M; failed setup
or validation shall refuse before Dispatch and confirm owned cleanup. These are required startup
capabilities, not permission inferred from an executable or a namespace pathname. The nested
`--bind / /` below binds O's already-confined root into the inner namespace; it shall never bind the
original host root. The exact inner flags inherit L's private network and do not create a host
network connection. Continuous backend IPC enforcement on writable shared inputs is specified in
the report threat-boundary section; an initial socket scan cannot establish that enforcement.

Nested M shall execute installed bwrap with `--unshare-user --unshare-pid --as-pid-1` `--new-session
--bind / / --dev-bind /dev /dev --proc /proc --info-fd 3 --block-fd 4 --` followed by the actual
first-party I command/arguments. `--as-pid-1` makes I the actual inner PID 1 rather than bwrap PID 1
supervising a PID-2 command; `--new-session` establishes I session isolation before exec. These
correct the previously incomplete explicit argv without changing the selected roles or cancellation
authority. M keeps `process_group(0)`. The new recipe omits `--die-with-parent`: outer PID-1
teardown supplies M/I containment, while fatal inherited inner PDEATH must not bypass the live-C
lease oracle. The outer user mapping is only UID/GID 0 to actual caller UID/GID, with verified
setgroups policy; nested bwrap mapping/privilege behavior must pass installed-tool verification. O
owns the bounded info reader and exclusive block-gate writer; M receives only info writer fd3 and
gate reader fd4. They are CLOEXEC outside intended child mapping; O releases the gate only after
validated I/observer ownership. C authenticates the chain over separate bounded controls rather than
pretending to own M's direct Child. Flags allocate one inner user/PID namespace, inner proc mount,
bind/dev-bind view, one bounded info pipe and one gate pipe; all descriptors/setup remain charged. O
creates report pipe/memfd; C/L have no report writer. O child-only maps report writer N >= 5 into
I/M, distinct from info/gate/control mappings, and immediately closes its own spawn copy after
successful mapping/spawn. M can retain a writer: require actual M termination/reap as well as inner
teardown before final EOF drain. Reopened procfd and descendant writer copies are included. O
retains report reader/memfd; C receives only final sealed memfd authority over separate controls.

The report-writer entry ownership prerequisite shall use only safe APIs under forbid-unsafe. I may
open the authenticated exact `/proc/self/fd/N` slot as a NEW write-only, nonblocking CLOEXEC owned
File; this does not adopt the inherited raw descriptor. O shall obtain expected device/inode
identity from its originally created report pipe BEFORE child mapping, and deliver that expected
identity and chosen N through bounded authenticated control bound to the original run and verified O
authority. Before closing the original slot, I shall authenticate that O-origin expectation, bind N
to the intended child-only mapping, and validate the NEW owned File's pipe type, device/inode
identity and write access against O's independently supplied expected identity. I shall also verify
its CLOEXEC state. An identity derived only from reopening N, comparing N with its own proc link, or
trusting a caller-supplied expectation is not the required independent check. Neither a
caller-supplied integer nor a matching slot number alone grants ownership. During this single-thread
entry, no actor may close/rebind/reuse N between validation and its one-time close. The locked safe
nix close API may close only that positively validated original slot; no arbitrary integer closure,
unsafe raw adoption or unsafe inherited-FD initializer is permitted. Failure settles through bounded
owned cancellation, not a guessed close or a retry against a potentially reused descriptor number.

After original-slot closure, I shall retain the NEW owned writer as CLOEXEC and safely map it into
only the actual backend child's N >= 5 slot, with intended exec inheritance. Unrelated execs shall
inherit neither writer; C/L still hold none. Authentication/identity/access failure refuses before
backend Dispatch. This entry allocation reopens only the original report PIPE in I; C still
receives the final sealed memfd as actual OwnedFd and never reopens its proc symlink. It neither
maps the report into stdio, reuses the exclusive lease, nor changes the unnamed storage mode.
Safe source APIs make this ownership route a candidate, not measured compatibility: nested mapped
UID pipe access, original-slot/no-reuse closure, actual helper/bwrap/backend/Cargo inheritance and
unrelated-exec exclusion, and the real Kani export roundtrip remain UNRUN prerequisites. If this
concrete entry route cannot satisfy those gates, stop CODE and report the capability gap for a
measured SPEC revision; no unsafe exception or runtime fallback is authorized.

Separate bootstrap, original-exclusive guardian lease and final report/control channels have
separate owners and EOF meanings. Only C ever holds the original guardian-lease writer. L/O/M
cannot keep it alive or forge positive Dispatch. Closing that lease while C remains live retains
L/O ownership and the final report channel. During the mandatory pre-escalation lease observation,
no L/O kill or bootstrap-control EOF may substitute for I's own lease-EOF cancellation; AC-24's
ignored-EOF mutant must still fail before independent escalation. Urgent resource/deadline failure
retains bounded outer cancellation, without resetting the deadline.

- The bounded executor shall launch only trusted first-party O and I as their respective namespace
  PID 1.
- The bounded executor shall use one ownership set for the entire batch launcher.
- The guardian shall remain INIT while supervising and reaping its backend descendants.
- The guardian shall run in a new session and process group outside the original caller's.
- The guardian shall remain outside original-caller terminal job-control and hangup delivery.
- The bounded executor shall verify session isolation before authorizing backend Dispatch.
- The guardian shall authenticate the original caller through its exclusive connected lease.
- The guardian shall reject a creator UID outside the verified original-caller UID mapping.
- The bounded executor shall authenticate guardian Ready using kernel sender credentials.
- The guardian shall require positive typed Dispatch before creating a production backend.
- The guardian shall reject bootstrap-gate EOF as production backend authorization.
- The bounded executor shall verify the guardian peer against the owned monitor/INIT chain.
- The bounded executor shall establish memory-observer readiness before backend Dispatch.
- The bounded executor shall require a configured explicit helper path with no PATH discovery.
- If the helper is missing or unusable, then the bounded executor shall refuse setup.
- The bounded executor shall match the helper identity to the actual running library build.
- If helper identity mismatches, then the bounded executor shall refuse before backend Dispatch.
- The guardian shall enforce finite control-byte, pending-message and startup-work bounds.
- The guardian shall reject malformed controls and unknown fields.
- If caller-lease EOF occurs, then the guardian shall exit as inner namespace INIT.
- Explicit cancellation shall consume the private caller lease while retaining run ownership.
- The bounded executor shall record lease-close observation before independent INIT escalation.
- While Bootstrap is unclaimed, the bounded executor shall retain its gate during cancellation.
- When Bootstrap is cancelled, the bounded executor shall signal its pinned startup group.
- While INIT is claimed, the bounded executor shall confirm its pidfd termination on cleanup.
- The bounded executor shall keep the original caller's lease endpoint exclusive to the original
  caller.
- The bounded executor shall map CLOEXEC control descriptors only into their intended child.
- The guardian shall restore separately received original stdin as backend fd 0 before spawn.
- If lease EOF is observable, then the guardian shall reject pending Dispatch and cancel.
- The bounded executor shall use safe descriptor APIs without relaxing its unsafe prohibition.
- The guardian shall preserve the original backend argv, stdin, environment and cwd.
- The guardian shall separate control and diagnostics from backend captures and reports.
- The bounded executor shall retain existing capture, resource and verdict classifications.
- The bounded executor shall charge all setup and execution to the original identity deadline.
- If that deadline expires, then the bounded executor shall classify the run as FR-028 AC-2 states.
- The bounded executor shall apply a finite setup cap within the remaining identity deadline.
- If only the setup cap expires, then the bounded executor shall return a typed setup refusal.
- The bounded executor shall bound control, capture and cleanup observation waits.
- If guardian failure occurs, then the bounded executor shall return a typed refusal.
- If owned teardown is unconfirmed, then the bounded executor shall refuse any proof conclusion.
- The guardian shall restrict cancellation to its own namespace descendants.
- The bounded executor shall use an anonymous pair without a publicly bindable rendezvous.
- The bounded executor shall use unnamed kernel report storage.
- The surviving assigned actor shall clean other temporary artifacts within its explicit ownership.
- The live owner shall reap its actual direct child after confirmed owned cleanup while it remains
  live.
- Setup documentation shall explain explicit matched helper delivery for library consumers.
- Production verification shall invoke the real package helper and observable lifecycle barriers.
- Regression verification shall fail ownership mutants before separate emergency fixture cleanup.
- The CG library shall omit guardian-test-support from default features.
- Where guardian-test-support is enabled, the CG library shall export one fixture operation.
- The fixture operation shall perform independent original-process and lease checks before INIT
  escalation; stored values shall not certify current gate state.
- The live-caller fixture operation shall invoke unchanged production cleanup immediately after
  observation.
- Production and fixture execution shall select one shared private stage sequence as data.
- The intentional-death fixture shall use its selected prefix only to choose the shared transition
  boundary before self-killing; the prefix shall not certify current gate state.
- The intentional-death fixture shall transfer only its actual owned monitor or validated INIT pin
  to its harness.
- The fixture operation shall exclude pass/fail oracle evaluation from library execution.
- The CG verification harness shall judge supported predicates from independently observed
  original-process and lease behavior before emergency escalation; exact fatal-prefix gate retention
  remains unbacked.
- The private Dispatch sender shall queue the unchanged production frame without waiting for ACK.
- The bounded executor shall wait for Dispatch acknowledgement in a separate bounded transition.
- The fixture continuation shall resume only its deliberately stopped and pinned INIT.
- The stopped-INIT fixture shall require genuine bounded coordination before owned-pidfd SIGCONT;
  unavailable coordination shall fail rather than accept a retired stage publication.
- The fixture operation shall reject unavailable continuation as passing EOF-cancellation evidence.
- The fixture build shall select the actual package helper through the consumer manifest.
- The CG verification harness shall build feature-off and feature-on configurations separately.
- If helper and library feature identities differ, then the bounded executor shall refuse Dispatch.
- CG documentation shall identify the downstream production-feature exclusion owned by IR-649.
- Before O creation, L shall establish the private network namespace.
- Before O creation, L shall establish the confined private root.
- Before M creation, O shall validate the allocated network isolation.
- Before M creation, O shall validate the allocated private root and IPC enforcement.
- Before Dispatch, I shall require safe backend-only IPC policy installation.
- Before arbitrary backend recipe execution, I shall require backend-only privilege restriction.
- While backend code runs, the installed policy shall exclude addressable AF_UNIX socket creation.
- While backend code runs, the installed policy shall exclude AF_UNIX datagram socketpairs.
- While backend code runs, the installed policy shall exclude syscall/ABI/io_uring bypasses.
- The backend execution boundary shall preserve the positively owned PID and original recipe.
- If backend-only installation fails, then C shall refuse before Dispatch with owned cleanup.
- Before child/control descriptor allocation, C shall capture and pin original stdin ownership.
- The trusted installer shall start without the arbitrary backend's loader environment.
- After policy installation and positive Dispatch, the boundary shall restore original backend environment.
- If actual backend recipe exec fails after Dispatch, then I shall use existing bounded failure handling.
- Before Dispatch, C shall reject socket-backed OriginalStdin::Open through typed unavailable admission.
- If OriginalStdin::Open type inspection fails, then C shall refuse admission with the original cause.
- When C captures OriginalStdin::Closed at entry, I shall preserve its closed representation.
- While arbitrary backend code runs, I shall prevent acquisition or export of trusted owner endpoints.
- While awaiting or supervising Dispatch, I shall retain its exclusive guardian lease.
- After original lease closure, O shall retain the separate final-report delivery channel.
- If IPC admission capability is unavailable, then C shall confirm owned cleanup before refusal.

### Pre-Armed negative cleanup capability custody

When L awaits its first authenticated O arm result, L shall select between the existing Armed
envelope and a newly allocated zero-right Committed/OperationalFailure negative on its existing O
bootstrap channel. O shall send this negative to L with zero ancillary rights. This is a NEW
receive/publication route; current zero-right OperationalFailure is O→C after arm, not O→L. L shall
retain its real O Child and original pidfd independently of arm publication.

L shall authenticate the negative from its actual retained Child PID with kernel sender UID/GID0,
original build/run authority, genuine source/state and producer stamp. L shall forward the SAME
bytes and EXACTLY ONE clone of that retained pidfd over existing L→C bootstrap while L has not
accepted a complete positive Armed. C shall select this negative only in its own AwaitArm receive
state, authenticate the actual L chain and original O bindings, and adopt the original O stamp
before L wait. No packet label or socket creator replaces current sender identity.
Missing/extra/wrong/substituted rights shall refuse through retained owned cleanup; no zero-right
fallback.

If O emits only a partial Armed frame to L and then fails or closes its source, L shall retain that
partial frame and any received rights solely for disposal through owned cleanup. L shall neither
accept Armed nor append, splice or authenticate a later negative through that partial frame. This
remains startup-protocol failure with the original cause potentially unavailable; L's actual
retained Child/pidfd authority shall still settle its owned O. No new negative route or clock window
is allocated for that damaged frame.

The positive split is O→L Armed acceptance followed by direct O→C Armed publication, NOT L
forwarding Armed. Once L accepts a complete positive Armed, it shall not use this pre-arm negative
forwarding route. A partial direct O→C Armed shall remain partial with its actual ancillary custody;
no negative splice/replacement or startup admission follows. Zero progress shall not establish
positive arm acceptance. After L accepts Armed, a failed zero-progress or partial direct O→C Armed
publication shall remain startup-protocol failure, with no promise that the original producer cause
is available. C shall retain partial bytes without admission and close any pidfd received with an
unauthenticated partial Armed frame. That unvalidated descriptor grants C no signalling,
termination-observation or other authority; existing independently authenticated L/owner custody
shall perform containment. C shall not grant positive admission, splice or return unconfirmed normal
settlement. A genuine partial-Armed-as- positive mutant shall fail the no-admission/whole-chain
assertion. A later measured need for post-L- Armed negative routing requires a separate allocated
slice; no future duty is promised here.

While C remains AwaitArm, C shall service the L bootstrap as well as the direct O source. Direct O
EOF shall supply no original cause or admission; C shall retain it as a provisional source-closure
fact while the existing original startup/stop cutoff admits completion of the authenticated L
negative route. Only a full valid L negative grants original cause/cleanup custody and original-
clock adoption. Partial/malformed or missing negative at cutoff fails with unchanged bounded
cleanup, no evidence and no generic startup EOF waiver. Earlier valid negative shall not be
discarded merely because direct O EOF was observed first.

C shall use the transferred pidfd only for exact O termination observation and existing owned-
cancellation signalling; L shall retain its direct Child/wait obligation. C shall not obtain a
fictional Child, M/I/other-actor authority, reopen a numeric PID, call pidfd_getfd, enter namespaces
through setns on the pidfd or use process_madvise. This restricts the allocated negative capability;
it asserts no new escalation versus positive O custody. No
Armed/namespace/network/phase/report/Dispatch admission follows.

Test evidence shall prove the clone's actual open-file-description identity with L's retained
descriptor through an admitted genuine KCMP_FILE or equivalent identity oracle; st_dev/st_ino
equality is insufficient. Tests shall distinguish the genuine clone from an independent same-O
pidfd_open and foreign pidfd. Missing kernel permissions/safe facility yields unavailable Test
evidence, never an inode fallback or passing skip. This allocates no mandatory runtime KCMP
operation or new capability category; source-bound actual clone production and existing sender/Child
authentication remain CODE duties.

C shall expose the original failure only after real O Child wait at L, L/capture/creator/control-EOF
settlement under the original earliest producer cutoff. C shall not infer whole-chain settlement
from a ready pidfd. When settlement is unconfirmed, C shall retain CleanupUnconfirmed with no
evidence. The route changes no envelope type, ACK, window, cap, public field or category. New O→L
negative selection, L→C forwarding, dual-source AwaitArm handling and genuine clone identity remain
PLANNED CODE gates.

### Claimed-startup negative terminal transaction

COMMIT shall mean the FIRST emitted byte of the existing terminal frame, not producer error
creation, encoding, observation start, receipt or complete delivery. Before COMMIT, O shall apply
existing resource/deadline/refusal precedence. A successful fresh complete exhausting pre-send tick
shall retire a zero-progress OperationalFailure and select existing OwnerStop even when the genuine
failure occurred earlier. Only a known offset-zero sender with no failed or ambiguous prior progress
attempt shall retire. Here unpoisoned means the actual retained send owner has no unrecovered send,
EOF, deadline or progress error; successful nonblocking zero progress is distinct from such a
failure. O shall retain a poisoned sender through the bounded failure path, without reset or
substitute emission. No first-established-error override applies. The independently retained
earliest genuine producer stop/minimum cutoff shall not reset when selection changes.

Before COMMIT, O shall also retire an unsent Failure when an independently established candidate is
selected by the existing owner-stop or authenticated refusal rules. An eligible original work-
deadline expiry selects TimedOut; a fresh complete exhausting tick takes priority over simultaneous
eligible expiry. An earlier completion or stop already ending work shall not become a new work
timeout when settlement later crosses the work deadline. The original whole cutoff remains
mandatory; its expiry during unconfirmed settlement retains CleanupUnconfirmed, not a fresh timeout
allowance. A genuinely authenticated setup/policy refusal retains its existing actual site/phase
classification beneath a selected owner stop; unknown labels, damaged controls or arbitrary later
errors shall not manufacture such a refusal. These rules shall neither invent a total ordering among
unrelated error sites nor relax malformed-report or unavailable-confirmation precedence.

A producer failure is established only by the actual typed error/cause/state and original stamp.
Resource exhaustion is established only at completion/evaluation of every named observation and
checked charge sum; sampling start and history alone establish none. O shall include all due ticks
and the required fresh complete tick before initial emission. Failed or incomplete current
measurements shall refuse that send attempt, not supply historical/default peaks. Malformed report
resource/deadline precedence remains mandatory.

After COMMIT, O shall preserve that one candidate's bytes without rewrite/splice/second terminal
frame. When the selected candidate is genuine OperationalFailure, O shall publish this existing
frame before I/M settlement so C can close the actual original I lease. Full authenticated receipt
grants only provisional cause/clock custody. O shall positively confirm the claimed I termination
and separately reap its actual M Child before normal O return. C shall require actual
O/L/capture/creator/EOF whole-chain settlement under the original minimum cutoff before its final
result. C shall interpret normal O Code0 here only as a settled negative transaction, never public
Success.

Every due observation shall continue through cleanup after COMMIT. O shall retain later successful
complete exhaustion in actual accounting/history/owned-stop custody until actual settlement. These
private facts end with O exit; C shall not infer an after-exit diagnostic or new peak transport.
They shall not upgrade the committed failure to success, stop enforcement, omit any actor or permit
normal O Code0 while teardown is incomplete. On the completed exhausting tick, O shall initiate
cancellation of the still-live claimed I and retained M through the same actual owned authority,
before waiting for C lease closure or proceeding with another publication attempt. O shall use the
retained I pin for actual signal/confirmation and the retained real M Child for its separate
cancellation/reap, clipped to the original cutoff without a new allowance. Committed bytes shall
remain unchanged.

When AC-77 cancellation causes genuine claimed-I death or I-lease EOF during delivery, C shall
retain those facts without letting them preempt completion and full authentication of O's committed
frame within the original cutoff. C shall preserve the authentic original cause if that transaction
completes; I death or lease EOF alone shall neither replace that cause with a transport refusal nor
authorize an unsettled result. Missing, partial, malformed or unauthenticated O delivery shall not
gain acceptance. Original-cutoff expiry or unconfirmed whole-chain settlement shall retain existing
CleanupUnconfirmed precedence; no new window, renewed clock or EOF waiver is allocated.

Private retention is an Analysis obligation: CODE Analysis shall follow OuterRunOwner's retained
sampling through publication and actual settlement, including OuterSampling's ledger/history and
owned-stop state. No current ordinary runtime seam proves private retention; the formerly proposed
hold-settlement-open Test is not retention evidence. Applicable runtime evidence for the separate
cancellation and settlement obligations remains owed through the existing SPEC-before-fixture-CODE
allocation. Current NamespaceOwner::cleanup has a blocking five-second poll and does not reap M;
CODE shall not borrow it as a new finite-step guarantee. A bounded claimed cancellation step using
that same authority is still a CODE gate, not an implemented API or runtime witness.

A later genuine observation/collector/transport error after COMMIT shall remain owned through
cleanup and prevent normal O Code0 acceptance; the existing abnormal/absent O settlement path shall
preserve CleanupUnconfirmed and shall not expose the original candidate as a final result. No second
frame or reconstructed cause is allocated. The CODE boundary is
CallerBootstrap::finish_startup_negative_after_roles requiring real Reaped Code(0), with
caller_public's unsettled cancellation error path selecting CleanupUnconfirmed. Current propagation
can discard a later owned error; CODE shall provide actual retention and fail-closed normal exit.

An absent peak field shall mean no peak transport/evidence, never actual zero, no historical
complete observation or current validity. Complete historical maxima shall not repair a failed
current tick. No claimed-startup negative route exists today: unclaimed_failure_step excludes actual
I claims, retire-on-fresh-exhaustion is prebyte precedence, and current partial exhaustion can be
lost. Claimed publication-versus-settlement state, later successful exhaustion lifetime and later-
error custody/consequence are PLANNED/UNRUN CODE gates, not current behavior claims.

Source Analysis shall inspect the actual owning publication and receive paths in
`src/kani/run/launcher_owner.rs::confirm_arm` and `publish_outer_arm`,
`caller_bootstrap.rs::advance_startup` and `finish_startup_negative_after_roles`,
`outer_sampling.rs::unclaimed_failure_step`, and `helper_entry.rs::run_outer`. These references
identify CODE gates and the missing claimed path; they supply no runtime proof.

### Queued claimed-phase cleanup during negative receipt

InnerClaimed means O's outer phase reply carrying the claimed INIT pidfd for the stage table's
ClaimedGated admission. The startup cursor means C's single retained direct-O startup control
stream, including its actual incremental receive state and ancillary-right custody.

When C receives a COMPLETE expected InnerClaimed with valid required grammar, original O sender/run
binding and exact declared rights, but cannot complete live-child/parent admission because I or
its previously authenticated M is no longer live or the original lease has reached EOF, C shall
enter close-only pending-claim receipt without inferring why either actor died. C shall first
exclude any positively established start/namespace/network/parent identity mismatch while the
original I/M are live; such a defect shall retain the existing identity refusal and shall not
enter pending receipt. Successful full live identity admission shall take the unchanged positive
path. Authenticated M death is observable parent state only, never cancellation cause or raw-I
identity authority. No generic admission-error catch shall select pending receipt.

C shall own the pending raw I right before the next fallible step, retain it solely for actual
close through owned cleanup on every exit, and grant it no positive identity, phase, gate release,
Dispatch, signal, termination-observation or other authority. C shall not reopen a numeric PID,
substitute a validated I handle or use the raw right to establish actor death. C shall continue
only the SAME original startup cursor using ordinary finite actor steps and scheduling, with due
accounting/cutoff checks unchanged and existing receive storage reused. Partial following delivery
may complete within the original cutoff; no partial frame shall be accepted as a negative. C shall
preserve the original C stop/minimum cutoff without a new timer, cap, allowance or receipt-time reset.
C shall concurrently continue required urgent owned cancellation/cleanup under existing independently
authenticated owner authority. Pending receipt shall not postpone that cleanup, wait out a new lease
grace period or grant signal authority to the unvalidated I right.

C shall fully authenticate the following original OperationalFailure against actual retained O,
original build/run/source/state/producer stop before retaining provisional cause/clock custody.
C shall expose its original cause only after actual I/M/O/L/capture/creator/control-EOF settlement
under that original cutoff, and shall actually close the pending raw right through owned cleanup.
C shall immediately reject malformed, invalid or unexpected following grammar. When O control EOF
or actual O death leaves no complete authentic Failure, or the original cutoff expires, C shall
select existing CleanupUnconfirmed/no evidence through retained cleanup. A silent live O remains
bounded by that cutoff. I/M death or lease EOF alone shall not substitute a cause, preempt completion
of authentic committed delivery or authorize an unsettled result.

C shall refuse wrong-phase/sender/run, malformed, partial or extra-right prior traffic instead of
entering pending receipt. C shall deliberately exclude a queued GateReleased reply: fully successful
GateReleased enters Backend, outside this Startup-only exception, and failed/ambiguous emission
remains poisoned/refusal under existing rules. C shall neither skip damaged bytes nor splice, retry
with a new parser, infer a terminal from EOF or adopt a generic cancellation parser fallback. This
allocation changes no positive admission check, frame, right, outcome, cause, public field, ACK,
window, cap or cancellation authority. Actual fixed pending storage/custody and bounded same-cursor
integration remain PLANNED/UNRUN CODE duties under existing named accounting; no capacity or runtime
proof is supplied.

### Startup and termination observations

The stage boundaries and cancellation observations are cumulative only where explicitly stated:

| Stage | Authority and cancellation | Confirming observation |
|---|---|---|
| BeforeMonitor: no inner monitor or inner INIT spawned | The same shared sequence has not invoked monitor creation; the original caller owns only its actual setup/lease state. No absent inner child has a process identity or pidfd; any existing L/O retains its actual authority. | Typed NoInit witness, no backend marker and closed actual setup/lease ownership after caller death. |
| Bootstrap: M created; inner I unclaimed; no Dispatch | O is already verified outer PID 1 with armed parent-death protection before M spawn. C retains L/O authority and its original lease; O retains the actual bootstrap gate and M Child. Bounded exact inner claim remains required before Dispatch. On M failure, info-reader failure or C/L death, outer INIT teardown cancels even blocked unclaimed I. | Actual outer pidfd termination confirms outer namespace teardown; actual claimed inner pin separately confirms inner teardown where available. No fabricated inner pin, host scan or monitor/group exit proves either. |
| ClaimedGated: INIT pidfd/start/parent/namespace verified and observer bound; gate retained | The same unconditional claim transition retains the validated INIT pin before the separate gate-release transition. Caller/group death cannot authorize a backend; trusted guardian bootstrap still requires the exclusive lease and Dispatch. | Independent original claimed INIT pidfd termination and no production backend Dispatch/marker; cached prefix establishes neither gate presence nor absence at report time. Exact fatal-prefix early-release observation remains UNBACKED. |
| ClaimedBootstrap: INIT pidfd/start/parent/namespace verified; gate released only for trusted bootstrap; no backend Dispatch | bounded executor retains monitor and INIT handles while guardian enters its new session and authenticates the exclusive pair. Failed lease authentication, guardian death or caller loss never authorizes a backend. | Claimed INIT pidfd signals termination on cancellation; no production backend marker. |
| InitReady: guardian peer/build/session verified; observer ready; no backend Dispatch | bounded executor retains monitor and INIT handles. Lease EOF causes guardian exit; live bounded executor can cancel through the claimed INIT pidfd. Guardian death itself is namespace-INIT death. | Claimed INIT pidfd signals termination before a live caller accepts cleanup; no backend marker. |
| Dispatched: guardian received positive typed authorization with live lease | Guardian supervises the whole backend namespace. Caller loss causes guardian exit after owned cancellation; guardian death invokes kernel namespace teardown. Live bounded executor confirms claimed INIT termination on every completion or failure. Explicit cancellation first closes the lease and observes guardian exit; bounded escalation uses the claimed INIT pidfd. | Claimed INIT pidfd signals termination before any live-caller proof conclusion; descendants are cancelled through kernel INIT teardown, not an observed-PID list. |

Bootstrap cancellation uses the positively retained outer INIT authority even when inner INIT cannot
be recovered after monitor death/reparenting. Bounded startup information or the live owned
monitor's direct children may establish inner identity, validating parent/start/namespace before
pidfd claim; this is required before Dispatch, never replaced by a guessed PID. Inner recovery
failure refuses setup and cancels through outer authority while retaining the gate and ownership.
No normal or startup host-wide membership walk occurs. Confirmed outer INIT termination proves
kernel whole-outer-tree teardown; monitor reaping is separate. Missing outer authority is an earlier
capability/ownership failure, never permission to spawn M and later claim best-effort cleanup.

The bounded executor creates the anonymous connected Unix-stream pair before monitor spawn. Both
ends are CLOEXEC; only the original caller retains the executor end. Safe child-only mapping gives
the guardian end to fd 0 through bubblewrap. The guardian safely borrows stdin's owned descriptor
for control receive/send; it never adopts an arbitrary inherited raw fd. There is no pathname,
abstract name, nonce-derived secret, public listener or connect/rebind step. An unrelated process
cannot replace the original caller by binding a discovered address. Pair creation or mapping failure
is typed setup refusal, never a rendezvous fallback.

The guardian checks kernel peer credentials of the pair's creator against the actual mapped
original-caller UID. Its view of the outside creator PID can be 0 and is not used as a host PID. The
non-public authority is possession of this specific already-connected executor endpoint, which never
passes to a monitor, backend or unrelated exec. Neither argv, environment nor a public socket-name
listing confers that authority. The bounded executor enables kernel sender credentials and binds
Ready's actual sender PID to its claimed host INIT pidfd/start/parent/namespace chain: socketpair's
creator credentials alone cannot authenticate guardian messages. Controls from another pair,
mismatched mapped UID or stale/replayed run authority refuse.

The guardian receives the original backend stdin separately as a bounded typed control's owned
descriptor through safe ancillary-rights APIs. Received descriptors are CLOEXEC, with exactly the
declared count/type; unknown ancillary records, extra/missing descriptors and byte/control
truncation refuse and close every received descriptor. Safe conversion to backend stdio restores
that same stdin at fd 0; the closed-stdin case remains closed. The guardian's control fd is excluded
from backend inheritance, and stdout/stderr remain the original bounded captures. Invalid control,
EOF and peer/build mismatch refuse before Dispatch. Observable lease EOF takes precedence over
buffered or pending Dispatch; an earlier valid authorization remains owned by INIT, which cancels
when it observes EOF. No atomic prediction of future caller death is claimed.

### Production lease-close cancellation

Private typed ownership separates a non-clonable `CallerLease`, which owns only the executor stream,
from `RunOwner`, which retains the actual unreaped launcher Child, authenticated outer/monitor/inner
pins, captures and original deadline. Both stay in the original caller; this separation neither
exports the lease nor transfers ownership of the outer chain. Explicit cancellation in InitReady or
Dispatched invokes the production `close_lease_and_observe` operation: it consumes and closes
`CallerLease`, disables further caller Dispatch, and retains `RunOwner` while observing claimed INIT
termination. C shall send no independent INIT cancellation or escalation signal during this
LeaseClosing phase. This restriction on C shall not prevent O's separate post-COMMIT owned
cancellation required by AC-77. Termination caused by that cancellation shall never count as
lease-EOF evidence. Pending guardian authorization still obeys observable EOF precedence. The same operation
handles cancellation before and after Dispatch, not a test-only hook.

LeaseClosing has a finite observation cap clamped to the remaining original identity deadline. Its
private typed `LeaseCloseObservation` distinguishes confirmed guardian termination, escalation
required while INIT remains live, and unavailable termination observation. The operation records and
returns this observation before any independent INIT cancellation or escalation signal; it never
reports termination from stream EOF alone. The normal production cancellation driver immediately
uses the observation: confirmed termination proceeds to bounded capture settlement and monitor
reaping; the other cases signal the claimed INIT, perform bounded confirmation/cleanup and refuse
unconfirmed teardown. This operation boundary lets the opt-in fixture operation record the actual
pre-escalation observation while its live original caller retains monitor/INIT ownership, then
immediately execute the same production cleanup phase. It exports no lease or cancellation handle
and never waits for a test controller.

Resource breach, original-deadline expiry and observation/identity failure retain their urgent
claimed INIT cancellation authority; they do not wait out a new lease grace period or reset a
deadline. LeaseClosing expiry records escalation before signalling, never a successful EOF
cancellation. `RunOwner` remains an ownership guard throughout both phases: dropping or abandoning a
live owned run initiates pinned cancellation; only explicit bounded cleanup can confirm teardown,
reap and settle a result. Dropping the separate lease cannot drop or reap the monitor. No proof is
accepted during LeaseClosing or from an escalation-required/unavailable observation.

### Opt-in guardian fixture observation

The `guardian-test-support` Cargo feature shall be off by default and absent from default features.
It shall expose exactly one documented fixture operation for TC-049's private-boundary scenarios.
The operation shall invoke the same private production transitions selected by its typed scenario.
Its live-caller form shall execute unchanged lease-close and cleanup operations unconditionally. The
feature shall add no branch inside a production stage and no substitute namespace INIT, worker,
identity check or cleanup implementation. Its additional fixture-only authority is recording real
typed observations, arranging bounded positive coordination for pending Dispatch and reporting actual original identity/right delivery before early-stage caller death. Ordinary public execution and positive post-Dispatch caller-death
fixtures remain available without this feature.

The live-caller form shall retain genuine original caller ownership while performing the unchanged
lease-close, kernel termination observation and cleanup operations. The immediate private
`LeaseCloseObservation` result may guide production escalation; it shall not be treated as a stored
certificate of termination or EOF origin. The harness shall independently check the original I pin,
positively acknowledged worker and backend marker before escalation. A stored stage, boolean,
ordinal, snapshot or returned category shall establish none of those predicates. Unavailable kernel
observation shall fail the fixture, and unchanged cleanup shall run on success, refusal and error.
No lease, cleanup-deferring callback or public ownership handle shall be returned.

For early-stage caller deaths, production and fixture shall use the same functions/order with the
selected bounded prefix as fixture input only. Normal control phases remain operational admission
state, not certificates of live behavior. The former unconditional stage-publication mechanism and
exact-prefix snapshot acceptance are retired; no replacement record or publication is allocated.
The shared sequence shall still claim the gated INIT and bind its observer before separate actual
gate release. Actual local O gate ownership may guard an operation, but C's cached phase shall not
prove that O still retains the gate at C's reporting boundary. Concurrent O failure or fatal exit
may close the gate independently. The exact fatal-prefix early-release adverse class remains
UNBACKED until a genuinely independent observable construction exists; absence of a backend marker
or eventual dead I shall not satisfy it. No ACK, controller pause or C-owned gate redesign is
allocated.

The harness shall map its anonymous report socketpair only into the first-party fixture's startup
stdout. Before any process spawn, the fixture shall safely borrow stdout, duplicate it into an
OwnedFd auxiliary descriptor numbered at least 3 with CLOEXEC, and mark the original stdout CLOEXEC.
It shall then use only the auxiliary descriptor for witness reporting. This safe standard-descriptor
entry shall not adopt an arbitrary inherited raw fd. Every later spawn shall configure stdio
explicitly and exclude both report descriptors from its child mapping. Neither descriptor may enter
the monitor, INIT, guardian, backend or unrelated exec. No child stdio option may clear CLOEXEC or
reuse the reporter as a capture. Failure to establish this exclusion shall fail before monitor
spawn.

At the selected boundary, the intentional-death fixture shall transfer actual original
caller/monitor/claimed-INIT identity metadata and the actual owned descriptor identity through the
existing auxiliary reporter. This metadata shall describe only genuine operational custody; it
shall not certify stage, current gate retention or operation order. At ClaimedGated or later, the message shall carry
exactly one clone of the already-validated owned INIT pidfd through safe ancillary-rights APIs. At
Bootstrap immediately after monitor spawn but before any INIT claim or gate release, it shall
instead carry exactly one clone of the actual owned monitor pidfd and typed InitUnclaimed. INIT may
already exist; InitUnclaimed shall never mean NoInit, nor fabricate an INIT identity or pin. At
BeforeMonitor, typed NoInit shall record actual no-spawn/setup state and carry no descriptor. An
unavailable owned monitor pin or required claimed INIT pin shall fail, never establish a passing
behavioral fixture. The fixture shall never open a replacement pidfd from a reported PID after death. The
report channel shall never carry the original caller lease or become backend capture.

After the bounded complete send, the same fixture process shall immediately SIGKILL itself or its
positively owned dedicated caller group, before any next transition. No harness acknowledgement or
external controller permission shall delay that death. A partial send, unavailable actual identity,
missing pin or failed coordination shall fail the fixture and invoke unchanged owned cleanup while
the caller lives; no timed success or passing platform skip is permitted. All reporting work remains
within the original deadline. Reporting and self-kill exist only inside the opt-in fixture
operation; ordinary production transitions perform no report I/O or intentional death. This
operation does not return after successful self-kill and supplies no fabricated execution result for
its dead caller.

The surviving harness shall retain its actual original-caller Child unreaped and its start/pidfd
identity until authenticating the report's kernel sender credentials. It shall receive exactly one
CLOEXEC owned pidfd for Bootstrap or claimed INIT and match its actual descriptor identity and
reported pin kind to the actual original pin validated before caller death. It shall reject
malformed, truncated, wrong-sender,
wrong-type or extra/missing rights and close all rejected descriptors. NoInit shall carry no right.
For a live claimed INIT, the harness shall verify the transferred pin against independently observed
start/namespace
identity. After INIT death, that predeath validated pin remains termination authority; a reused
/proc PID cannot substitute. ClaimedGated and later require confirmed caller and INIT death plus no
pre-Dispatch backend marker. BeforeMonitor instead requires actual no-INIT setup observations,
closed actual lease/pair ownership and no backend marker, plus termination of any positively owned
setup child.

Bootstrap shall require actual caller and pinned monitor termination and no backend Dispatch or
marker under bounded observation after actual C/group death. These observations shall not infer
unclaimed INIT termination, current gate custody or guardian exit from stream EOF, monitor readiness
or a reported PID. Actual original I pins, where positively claimed, shall remain independently
usable after death without numeric-PID reopening. The no-unowned-backend and whole-tree cleanup
guarantees remain mandatory, including the internal map/setup handoff before the public gate and
actual parent-death installation order. Missing genuine unclaimed authority or exact native-window
construction shall remain an explicit AC-31/IR-652 verification gap, not a passing stage-only test.
Claimed cases shall require actual original I termination, no leaked owned inner processes and no
backend Dispatch/marker before separate emergency cleanup. Fatal O failure shall not be reported as
positive live gate custody.

Independent emergency cleanup of only fixture-owned processes shall follow recorded assertions
without masking them. Structural inspection shall establish shared stage functions/order, safe
reporter exclusion and feature item exposure only; mandatory AC-24 EOF mutants retain behavioral
parity evidence. Neither selected-prefix metadata nor structural inspection satisfies the complete
namespace-teardown oracle or supplies executable coverage for the Analysis obligation.

A Linux queued-descriptor probe measured one validated INIT pidfd surviving a separate reporter's
SIGKILL, received CLOEXEC both while INIT was live and after its death. This establishes only the
queued-pin facility: its host parent owned the actual monitor/INIT/control, not the production
original-caller Rust path. Actual original pin delivery, caller/group death, sender binding,
NoInit/InitUnclaimed observations and the production guardian oracle remain CODE acceptance tests.

A separate Python/kernel reporter probe measured the standard-stdout entry, non-stdio CLOEXEC
auxiliary duplication and original-stdout CLOEXEC marking before spawn. The actual socket identity
was absent from the unrelated exec, bubblewrap monitor, namespace PID-1 helper and backend. Removing
either mark failed its actual pre-spawn descriptor-flag assertion; removing the auxiliary mark also
leaked the socket into unrelated exec. Explicit stdout replacement prevented a leak in the
original-mark mutant, so flag and inheritance assertions remain distinct. Safe Rust descriptor APIs
were source-grounded; no arbitrary inherited-fd adoption is required. This facility probe is not
production Rust reporter acceptance, gated-INIT coverage or a passing Bootstrap cleanup Analysis.

For deterministic pending-Dispatch coordination, the feature-gated fixture owner shall stop its
verified InitReady INIT through that owned pidfd. The owner shall positively verify stopped state
`T` against the same live INIT/start/namespace identity before queueing Dispatch through the
unchanged private production frame-send step on the ordinary control stream. This step shall
serialize and send the actual production authorization and ancillary rights, not fixture-written
bytes. The send shall use bounded nonblocking transport within the original deadline. The send shall
return a typed pending-frame state only after the complete frame is queued. If the bounded send
fails or transport is unavailable, then the fixture operation shall record typed coordination
failure. The fixture operation shall invoke cleanup after that failure. The pending-frame state
proves neither guardian receipt nor backend creation. The normal executor shall await any guardian
acknowledgement in a separate bounded transition. The stopped-INIT fixture shall not enter that
acknowledgement wait before closing its lease. The owner shall keep `RunOwner` and the original
caller alive while invoking the unchanged synchronous `close_lease_and_observe` operation. After
actual `CallerLease` closure, production shall continue its existing bounded termination observation
and cancellation path with the original owner, stop and cutoff. Close-completion/publication
ordinals, immutable publication snapshots and their later acceptance are retired. The stopped-I
fixture must still resume or cancel its genuine pinned I and join every owned continuation during
unconditional cleanup; removal of the snapshot shall not strand I. A genuinely observable bounded
coordination construction is still owed before this Test can run. This amendment allocates no
replacement channel, record, timer, callback or grace. Missing coordination shall yield fixture
failure, never skip/pass. SIGCONT shall remain resumption only and shall not establish cancellation.
Urgent deadline/resource/identity cancellation shall retain its authority while I is stopped.

The external harness shall require actual original I termination before escalation and no backend
Dispatch/marker for pre-Dispatch EOF cancellation; post-Dispatch it shall additionally require the
formerly live positively acknowledged worker dead before escalation. It shall independently reject
ignored EOF and skipped actual lease close, not merely missing or reordered publication. Later
cleanup shall not repair a failed pre-escalation predicate. Immediate private observation results
remain legitimate control flow, but transported copies shall not replace independent kernel
observations or identify the cause of death. Restored controls and separate containment/session/
positive-Dispatch/startup-order mutants remain owed. All evidence remains PLANNED/UNRUN.

The packaged caller fixture, library and real helper shall use the same normal, non-`cfg(test)`
library artifact. The fixture build shall select the package helper from the consumer manifest using
`cargo -p`. The fixture build shall match target, profile, feature selection and compiler flags. The
bounded executor shall require actual artifact identity without an epoch, version or digest
override. If a feature-off helper is supplied to a feature-on fixture, then the bounded executor
shall refuse before Dispatch. If a feature-on helper is supplied to a feature-off production caller,
then the bounded executor shall refuse before Dispatch. Feature-off builds shall expose no fixture
operation. The CG verification harness shall check both mismatch refusals and feature-off export
absence using actual artifacts.

The CG verification harness shall use separate named Cargo invocations: `guardian-feature-off` for
normal feature-off library/helper/caller builds and AC-23/26 public positive post-Dispatch fixtures,
and `guardian-feature-on` for normal feature-enabled library/helper/caller builds, exact early-stage
AC-23/26 death witnesses and AC-24 observation.
These names identify verification configurations, not Cargo profiles or new build directories. The
harness shall not use a self dev-dependency that unifies guardian-test-support into the feature-off
configuration. Neither configuration uses a `cfg(test)` library to impersonate its normal helper.

CG documentation shall mark guardian-test-support as test-only. CG documentation shall identify
matched-artifact delivery and separate feature-off/on verification. CG documentation shall allocate
production-driver dependency-edge exclusion to the QSL production-driver work owned by
[IR-649](https://linear.app/agent-ix/issue/IR-649). That downstream work must reject the feature in
all driver production-build profiles, including transitive Cargo feature unification. IR-639 enables
that work; its downstream delivery is not a prerequisite for delivering the CG observation export.
CG inspection checks publication of this contract, not the driver's implementation or a CG-only
surrogate of it. The downstream dependency-edge assertion remains a separate planned gate.

### Report threat boundary and backend IPC confinement

The guaranteed ownership domain includes every contained L/O/M/I role and backend descendant,
including duplicated, reopened, reparented and late-born report writers. One fault domain applies to
report, original lease, bootstrap, ownership and final-control authority. This requirement makes no
channel exclusivity/authentication, report or lifecycle guarantee against a hostile or cooperating
same-UID host peer outside that tree independently obtaining such authority through host-side
access or SCM_RIGHTS. This does not assert that every host peer can perform those operations:
credentials, mappings, dumpability and kernel security policy can restrict them. Ordinary foreign
actors without independently stolen authority remain subject to the unchanged rejection criteria.
The exclusion does not excuse a contained backend obtaining or exporting any writer, lease or
trusted endpoint. I shall prevent that acquisition/export. Every existing contained-death, owned
writer closure, actual EOF, immutable seal, ceiling, deadline and exclusive-lease obligation remains
mandatory; no fault-domain clause repairs an existing contained-tree defect.

L shall allocate NEWNET and a private root before O creation, using the verified private mount
namespace and propagation. Required original cwd, crate/source, target, cached build inputs,
helper/backend/toolchain and loader paths shall retain their admitted path/byte semantics; required
shared writable paths shall remain writable. L shall exclude original host proc/root aliases,
retained host-directory/namespace descriptor escapes and unrelated host IPC rendezvous paths from
the root mapping. O shall validate actual network/root/proc identities and the admitted mapping
before M creation. The nested `--bind / /` binds this already-confined O root, never original host
root; inner M/I inherit the private network. Neither an initial socket scan nor a namespace label
proves continuous host-peer exclusion on writable shared paths.

I shall require the backend execution boundary to establish kernel seccomp IPC and privilege
restrictions before Dispatch and before executing any arbitrary backend recipe. That boundary shall
use safe code without unsafe or pre_exec, apply the restrictions only to the backend and descendants,
and keep INIT/I, O and M supervisors outside the backend-only filter. The eventual backend shall
retain the same positively owned PID/identity under I, with no extra surviving process, ownership
set or deadline. Actual argv0, non-report argv, environment, cwd and stdio shall remain unchanged.
No temporary trusted transport descriptor shall survive into arbitrary backend execution. If
installation fails, then C shall return typed pre-Dispatch unavailable refusal with owned cleanup;
there shall be no unfiltered Dispatch. The boundary shall separate trusted startup admission from
actual recipe exec, with these two transitions:

| Boundary | Required behavior |
| --- | --- |
| Before positive Dispatch | Safe policy/filter/privilege installation failure travels over the bounded authenticated startup channel as typed unavailable admission; C confirms owned cleanup. Initial trusted-helper spawn success or early channel EOF is not installation success. No arbitrary backend recipe executes. |
| After positive Dispatch | Installed policy permits execution of the exact original backend recipe. Its actual unsuccessful exec exit with no report, after confirmed owned teardown and original deadline/capture settlement, produces existing Inconclusive NoVerdict. No unfiltered retry, fabricated report/status/evidence, synthetic Failed or new kind is permitted. It is not retroactively classified as pre-Dispatch policy installation failure. |

Startup transport shall remain CLOEXEC through the successful exec boundary; partial actual recipe
exec failure shall settle through that existing post-Dispatch path without retry or fallback. An
actually unsuccessful backend exit with no report after confirmed owned settlement shall produce
the existing Inconclusive NoVerdict result; C/launcher boundary I/O failure retains its Tool Io
refusal. These conditions shall not be conflated or inferred from early transport EOF.
Initial trusted-helper spawn success or early EOF shall never prove successful recipe exec/handoff.
Positive Dispatch remains required before arbitrary backend creation/execution. The
mechanism is deliberately unspecified: a matched helper
exec-entry is a CODE-plan candidate only. A source-grounded safe-boundary feasibility audit is the
first CODE gate; inability to meet these properties stops CODE for SPEC revision.

The trusted installer shall start in a separate sanitized environment that cannot activate the
arbitrary backend's loader inputs (including LD_PRELOAD, LD_LIBRARY_PATH and equivalent loader
configuration). C shall carry the original raw backend environment only as authenticated bounded
recipe metadata; no trusted startup role shall activate those values before policy installation.
Only the safe recipe exec after installation and positive Dispatch shall restore the exact original
environment alongside original argv0, argv, cwd and stdio. Safe capture/transfer/restoration shall
retain existing resource/deadline bounds. Neither untrusted loader code before installation nor
rewriting the backend's final environment is permitted. This is a mechanism-neutral required
property, not a selected helper entry or implemented environment transport.

The policy shall prevent `socket(AF_UNIX, ...)` creation and
AF_UNIX datagram socketpairs. It shall allow anonymous connected SOCK_STREAM socketpairs for
contained-local IPC and shall not blanket-deny sendmsg. It shall cover legacy socketcall and every
supported syscall/ABI alias. Pre-Dispatch ABI admission establishes support for the actual native
ABI of the trusted guardian/backend-installer process and successful installation of the matching
policy; it does not establish the ABI or immutable identity of the eventual backend image.
No inherited socket/listener or io_uring descriptor shall reach arbitrary backend code. The policy
shall close io_uring socket/operation bypasses by excluding backend io_uring creation/control and
shall persist across fork, exec, reparenting and nested namespaces without a privilege-based escape.
C/L/O/M and intended trusted owner endpoints shall retain their separately allocated controls; no
backend IPC restriction shall close I's lease or O's final-delivery channel early. Dynamic host
pathname listeners created after Dispatch in admitted shared source/target/cwd paths remain
unconnectable throughout execution; explicit-address datagram export is also excluded. Private
network isolation excludes the host abstract address scope. I shall admit no arbitrary backend
host-peer endpoint; private proc/PID view shall exclude host descriptor/root aliases. Host ownership
pidfds and all other trusted descriptors shall remain CLOEXEC outside intended role mappings and
unavailable to arbitrary backend, sibling exec and backend descendants.

### Native policy admission and actual backend execution ABI

When the actual trusted guardian/backend-installer process ABI lacks an audited matching policy,
C shall refuse before Dispatch through the planned mandatory `CapabilityUnavailable` context
naming `BackendIpcExclusion`. This admission context is a merged SPEC API allocation that remains
CODE-gated, not an already implemented capability API.
When IPC policy preparation or seccomp filter installation fails, C shall retain its actual cause
through that BackendIpcExclusion admission context and confirm owned settlement.
When privilege installation or trusted-owner protection verification fails, C shall retain its
actual cause through the planned `TrustedOwnerProtection` capability context and confirm owned
settlement. NNP/CAP_SYS_PTRACE restriction failures remain owner-protection failures even when a
filter library reports them during installation; the actual typed operation decides the context,
not the outer wrapper, errno or diagnostic message.
Header inspection, a successful file open, initial helper spawn or startup EOF shall not establish
target-image ABI admission.

For semantic unsupported native policy support, C shall construct the mandatory admission
`io::Error` with `ErrorKind::Unsupported`, no raw OS errno and typed private support provenance
from the actual audited-support predicate. This is an admission error, not an invented OS failure.
For an actual OS installation error, C shall preserve the original public cause's errno and kind;
private wrapping shall not erase its raw OS errno. C shall retain the actual typed installation
site/origin in the authenticated private refusal custody independently of that public errno.
For a non-OS installation error, C shall retain only its finite actually reconstructible CG or
dependency variant/fields as typed private provenance and, where representable, typed public
io-error source. No Display/message parsing shall distinguish support, privilege or filter sites.
The private transport shall bind that provenance to the authenticated run/build, actual owned
pre-recipe PID/state before C classifies the refusal. A kind-only transport
projection does not satisfy typed-origin retention and remains a CODE gap. For these new ABI-admission errors, this allocation promises
neither arbitrary boxed-error/downcast-chain or object reconstruction nor an external post-return
stage query; any such original-chain fidelity loss remains explicit rather than forged. It adds no
public cause, stage field, catalog or budget to the planned capability plus mandatory io-error shape.
No other original-cause retention obligation is relaxed by this native-policy allocation; the separate
[cross-role refusal cause representation](#cross-role-refusal-cause-representation) qualifies only
its listed trusted role/operation/phase cases, preserving these native-policy typed-origin duties.

The installed backend-only policy shall enforce its actual syscall audit-architecture rule on
every backend and descendant syscall.
When a syscall has an unsupported audit architecture, the installed policy shall terminate the
calling process using the seccomp process-kill action before that syscall executes.
For every native x86_64 policy, the installed policy shall terminate the calling process on an x32
syscall-number/alias using the same process-kill action, regardless of host kernel x32 support.
The policy shall remain installed across exec and descendants.
When native entry issues a compat syscall, the installed policy shall apply the same architecture
and x32 process-kill rules. This is syscall-boundary enforcement, not a promise that
exec rejects every incompatible binary or that no incompatible image/instruction executes before
its first filtered syscall. Required IPC, privilege, descriptor and ownership protections remain
mandatory; an ABI mismatch permits no unfiltered retry or weaker policy.

The exact original recipe continues to use its admitted pathname, argv0, arguments, environment,
cwd and stdio. This contract imposes no executable/interpreter/loader stability precondition and
claims no binding of a prior file/header inspection to the eventual executed bytes. Path replacement,
content mutation, PATH/execvp resolution, script interpreter resolution, ELF PT_INTERP resolution,
loader selection and kernel binfmt_misc interpreter/handler resolution can differ between
observation and actual exec. A registered native emulator can translate a foreign image into native
syscalls; the filter enforces those actual native syscalls, not an inferred guest-image ABI.
Handler/emulator resolution grants no exception to private-root, descriptor, IPC, privilege or
contained-descendant requirements. Those check-to-exec residuals
apply before the first filtered syscall; they do not authorize recipe rewriting, a new inherited
execution descriptor, changed shared-writable-path semantics or relaxation of installed policy.
The trusted installer still uses sanitized loader inputs, and only filtered post-Dispatch recipe
exec restores the original backend environment.

An actual post-Dispatch exec failure or syscall-policy ABI termination shall follow the existing
bounded backend-result path, rather than retroactive pre-Dispatch unavailable admission. When the
actual exit is unsuccessful with no report, C shall classify existing Inconclusive
NoVerdict only after positive whole-chain settlement and original capture/deadline handling. Existing
memory/deadline candidate precedence and CleanupUnconfirmed override remain unchanged. Present,
malformed or missing-success reports retain their existing report/classification rules; an ABI
assumption never fabricates a status, report, evidence, new cause or Failed verdict. No rule here
promotes an observed-only file check into proof of the actual backend image.

Before backend creation, I shall set and positively confirm PR_SET_DUMPABLE 0 after final
credential/mapping transitions. I shall remove backend CAP_SYS_PTRACE authority in I's owning user
namespace and prevent regain through exec, file capabilities, credentials or nested user namespaces.
I shall retain non-dumpability while it owns its endpoints. These restrictions shall prevent real
backend /proc/1/fd reopening, pidfd_getfd and ptrace acquisition of I's report/lease/control endpoints
independently of ambient Yama. If safe protection setup or verification fails, then C shall refuse
admission with confirmed owned cleanup; a host profile denying an attack is not the protection proof.

The allowlisted trusted channels are C/I's exclusive original lease, C/L/O bootstrap and authenticated
ownership controls, and O/C final report/control delivery. I shall retain its lease through Dispatch;
O shall retain final delivery after original lease close. They remain owned and unavailable to
arbitrary backend/descendants/sibling exec, with existing bounded SCM_RIGHTS and EOF meanings.
The AC-27 opt-in reporter is separately trusted, excluded from every production child; it supplies
no backend host-peer endpoint. First-party controls shall not be closed early to satisfy admission.

Caller stdio stability is a trusted-caller precondition outside the arbitrary-backend fault domain.
While C performs launch setup, the embedding caller and its other threads shall not concurrently
close, rebind or replace the caller process's fd0/fd1/fd2. Setup here runs from the ordinary C entry
through positive Dispatch or settled setup refusal. Violating this precondition is a caller contract
breach, not a guardian containment fault. C shall refuse through existing typed admission handling
when capture actually observes inconsistent presence, followed-target identity or original flags, an
unexpected EBADF after an Open observation, or a nonmatching expected file type. No obligation
claims detection of every ambient mutation, atomicity across separate observations or prevention of caller-induced races. Authoritative
initial absence remains the distinct Closed case below; this clarification waives no contained
backend/owned-writer/death obligation.

Public rustdoc for execute_kani_obligation, execute_kani_obligations and any other public bounded
entry shall document this trusted-caller fd0..2 stability precondition, its
entry-through-positive-Dispatch-or-settled-refusal
setup window, and that violation is a caller contract breach. Documentation shall state that only
observed inconsistency refuses; it shall not promise atomic capture or detection of every mutation.

An authenticated self-proc lstat of /proc/self/fd/N is only a presence/magic-link probe: its symlink
type, proc inode and mode are not the target's metadata and shall not be compared with fstat of the
owned pin. For an Open capture, followed stat of that same self-proc entry and fstat of the captured
pin shall agree on target file type (st_mode & S_IFMT), st_dev and st_ino at the capture observations.
Only original F_GETFL & O_ACCMODE and F_GETFD & FD_CLOEXEC shall be compared across
original-descriptor capture observations; the pin's intentionally set CLOEXEC is excluded
from original exec-flag comparison. Link absence followed by an Open observation, differing target
identity/type, differing original access mode/exec flag or unexpected EBADF after Open is observed inconsistency
and shall refuse. Mutable shared-open-file-description status flags, including O_NONBLOCK and
O_APPEND, shall not be compared for instability admission: another process sharing that description
can change them without a caller descriptor-table contract breach. Capture shall preserve actual
original input/recipe semantics, neither rewriting those flags nor restoring an earlier snapshot.
Initial authoritative absence retains the Closed rule below. These separate
observations are not atomic and do not prove identity of an open file description from inode alone.
Linux [stat](https://man7.org/linux/man-pages/man2/stat.2.html) distinguishes link and target
metadata; [fcntl](https://man7.org/linux/man-pages/man2/fcntl.2.html) defines the separate flag queries.

OriginalStdin is a planned internal bootstrap ownership value captured by C, not a new public
KaniExecutionRequest field or a caller-supplied request parameter. Its variants are Open(OwnedFd)
and Closed. Current CG's request has no stdin field and BackendCommand preserves inherited stdin. At C's authoritative ordinary-exec boundary,
before child/control descriptor allocation or fd0 reuse, C shall capture original descriptor/exec
flags and pin the actual open file description using safe owned-descriptor APIs. OriginalStdin::Open
owns that pin when the original stdin is open for inherited exec; OriginalStdin::Closed records an
originally absent fd0 or an original CLOEXEC source that ordinary exec would leave closed. The
transport pin's own CLOEXEC flag shall not turn an originally Open input into Closed. Initial
absence established authoritatively at that C boundary may use EBADF as Closed; failed capture for
any other cause shall refuse. Capture relies on the trusted-caller stability precondition; observed
inconsistency shall refuse, while unobserved caller mutation is not a detection guarantee. Unavailable
safe absent-fd handling shall refuse, never guess from later child descriptor numbers or adopt raw
numbers as owned descriptors. Internal capture/pin, authenticated self-proc observations and safe
absent-fd handling remain UNRUN implementation gates, not claims of current support.

C shall establish actual backend fd0/fd1/fd2 inventory before Dispatch. It shall inspect fstat type
on the captured OriginalStdin::Open pin. Production fd1/fd2 are owned capture pipes, verified by
mapping/inventory Analysis, not caller-selectable socket positions. C-only reporter/control sockets
are not backend stdio. If an admitted descriptor has S_IFSOCK, then C shall refuse before arbitrary
backend creation regardless of socket family/peer state. Pipe, regular-file, terminal and /dev/null
inputs shall retain original semantics. I shall preserve the captured Closed tag without fstat on
an absent descriptor. If later inspection of an already captured Open pin fails, including EBADF,
then C shall refuse; neither C nor a descendant shall reinterpret that failure as Closed or re-probe
a reused fd0. C/I shall not substitute, reopen or rewrite original input, raw non-report argv,
environment or cwd to obtain admission.

Socket stdin, private network isolation and loss of addressable AF_UNIX rendezvous are explicit
caller availability limits. Anonymous local stream socketpair IPC remains admitted. Cargo builds
with locally present source/toolchain/cache inputs may run; host registry/git fetches and host Unix
services are unavailable. C shall supply no prefetch, unmetered external resolution, recipe rewrite
or weaker network/IPC mode. If missing inputs or denied rendezvous cause an unsuccessful build
without a report after admitted Dispatch, then C shall preserve FR-017 Inconclusive NoVerdict for
single runs and every compatible batch member. Existing memory/deadline classifications take their
existing precedence; build incompatibility does not become a fabricated setup failure. Genuine
installed Cargo plus Kani 0.68 under the actual filter is a decisive PLANNED/UNRUN gate. If that
roundtrip is incompatible, then CODE delivery shall stop for measured SPEC revision, never relax
this policy or supply fallback.

If stdio inspection, private network/root/proc, IPC enforcement or owner protection cannot be
established, then C shall classify the admission failure as BoundedLaunchError::Unavailable at its
site regardless of original errno/io::ErrorKind, retain the original cause and confirm owned cleanup.
For listed cross-role causes only, original-cause retention has the explicit representation/loss
meaning in [cross-role refusal cause representation](#cross-role-refusal-cause-representation);
local causes retain their original io::Error and source.
The generic run-stage Unsupported/NotFound classifier is insufficient for PermissionDenied, EIO
or EOVERFLOW admission failure. The public top-level refusal remains
KaniExecutionRefusal::MemoryMechanismUnavailable with its original `cause: std::io::Error`; planned
CODE shall broaden its documentation and Display from memory-only enforcement to bounded startup
prerequisites. Planned CODE shall attach mandatory `admission: KaniStartupAdmissionCause` typed
context alongside that unchanged cause. Its exact variants are `MemoryEnforcement` for existing
memory checks, `BackendStdioSocket { descriptor }`, `BackendStdioInspectionFailed { descriptor }`,
and `CapabilityUnavailable { capability: KaniStartupCapability }`. BackendStdioDescriptor names
Stdin, Stdout and Stderr; ordinary caller socket rejection identifies Stdin. KaniStartupCapability
names PrivateNetwork, PrivateRoot, PrivateProc, BackendIpcExclusion and TrustedOwnerProtection.
Original syscall errno/error remains in cause; input-type rejection retains actual descriptor/type
facts in admission context, not message parsing. Every unavailable site shall select its mandatory
variant, with no optional None, default, string discriminant, compatibility layer or new top-level
refusal. This is a planned API amendment, not a claim about current code.

MemoryMechanismUnavailable has code()==None and returns no KaniExecutionEvidence, evidence kind or
KaniRunOutcome. Existing run_terminal_value accepts KaniRunOutcome, not an execution refusal; C shall
fabricate no terminal value or Failed outcome. Original deadline expiry retains its classification
precedence. All mechanisms, compatibility, capability and confinement Tests here remain PLANNED/UNRUN.

### Cross-role refusal cause representation

This PLANNED/CODE-gated allocation covers only I/O causes emitted by matched trusted helper roles
and carried to C through their existing authenticated negative controls. Private finite cause,
provenance and loss custody applies to the listed roles/operations/phases below. Public io::Error
projection and public loss/integrity get_ref/downcast duties apply ONLY where the existing public
refusal actually carries an io::Error, such as MemoryMechanismUnavailable or Tool(KaniToolError::Io).
They add no carrier to detail-only results. Its role/site/phase scope is:

| Actual producer role | Allocated operations and phases |
| --- | --- |
| L | Trusted role bootstrap/identity/protection and private control preparation during setup; actual O spawn, ownership/control and child wait/retirement during work or settlement. |
| O | Trusted namespace/proc/identity/owner-protection setup; actual L/tree observation and checked resource accounting; bounded private control/context preparation, encoding and reception during setup, work and settlement; owned anonymous report pipe/memfd creation, collection, sealing and final delivery; actual M spawn/claim/stop/reap and final O settlement control. |
| I and its matched trusted same-PID backend installer | Trusted bootstrap/identity/owner protection, original exclusive-lease and private control/context preparation or reception; safe native policy preparation/installation before Dispatch; genuine backend supervision/completion and owned settlement controls after Dispatch. Policy-site distinctions and stronger native-policy origin duties remain required. |

A Rust error from creating, reading or waiting for the external bwrap M is produced by its actual
trusted owner; this allocation invents no M-origin Rust payload. C's own report-read/capture/control
preparation errors remain local-role errors, even when the same preparation type is used remotely.
Arbitrary backend code, recipe exec/result/report-content errors and foreign/custom payloads supplied
by that backend are outside this loss permission. Their existing result/report/refusal rules remain.
A new role, operation or phase outside the table requires an explicit SPEC allocation before CODE
uses this permission; a renamed implementation function alone does not expand it.

When a listed operation emits a cross-role I/O cause, the sending owner shall capture its actual
producer ErrorKind, optional raw OS errno and available typed payload facts before projection.
For valid original-cause replay, C shall authenticate the originating role and failing operation
before selecting that cause's existing mapping. Integrity/fault reports retain only independently
authenticated optional facts and use the explicit mapping rules below; a packet label supplies no
missing role/site authority. C shall bind admitted cause facts to the existing run/build, actual retained process identity,
state, descriptor and applicable original-stop authority. Decoding cause metadata grants none of
that authority.

For these listed cross-role operations only, this section qualifies the original-cause wording in
[admission refusal](#report-threat-boundary-and-backend-ipc-confinement) and the
[failure mapping](#settlement-confirmation-and-kernel-fault-boundary) below: the public cause retains
the original facts declared here where that result has an I/O cause, with explicit opaque loss, rather
than an identical boxed object. A detail-only result retains its existing public shape and no original
public cause/downcast promise is allocated.
Local-role original io::Error/source retention and the native-policy typed-origin duties remain
unchanged. C shall retain the authenticated finite cause representation and its loss facts before
publication. No public refusal variant/code, errno-independent admission, candidate precedence,
actual settlement or original deadline is changed.

| Actual originating cause | Sending-owner obligation | C receiving/projection obligation (public projection only for existing I/O-cause-bearing results) |
| --- | --- | --- |
| I/O error with a raw OS errno | The sending owner shall retain the exact errno and actual ErrorKind. | C shall reconstruct the public I/O cause directly from that errno and verify its kind on the same running platform; C shall reject a kind mismatch. No wrapper shall erase the public raw_os_error or invent an errno. |
| I/O error with no raw errno and no custom payload | The sending owner shall retain the actual named ErrorKind and absence of errno/payload. | C shall reconstruct that kind without a custom source or loss marker. The supported producer/build domain below supplies complete kind coverage. |
| Valid original positively observed reservation payload or Reservation wrapper, excluding required cause-representation faults | The sending owner shall retain actual outer I/O facts and its observed reservation category, wrapper and operation. | C shall preserve the actual no-errno kind with the public KaniCrossRoleCauseLoss marker; C shall infer neither allocation-versus-capacity kind nor layout from diagnostics and shall not manufacture an identical Reservation payload. |
| Other valid original trusted custom payload, excluding required cause-representation faults and stronger existing public-source duties | The sending owner shall retain actual outer I/O facts and the typed listed operation with explicit unrepresented-payload/chain status. | C shall preserve the actual no-errno kind with KaniCrossRoleCauseLoss; C shall invent no original source, downcast identity or diagnostic-derived category. |

Planned CODE shall expose `KaniCrossRoleCauseLoss` alongside KaniExecutionRefusal at the public
crate boundary. It is an opaque marker implementing Debug, Display and std::error::Error, with
Send + Sync; its source() is None. Its private fields shall retain the authenticated actual producer
role and originating operation from the finite role/site table above, as typed non-source provenance.
Its constructor and fields remain private; C shall initialize them from the authenticated typed
control, never diagnostics. It carries no original payload/chain. For a listed no-errno cause whose
original custom payload/chain is lost, C shall construct io::Error::new(actual_original_kind, marker). A caller
shall detect loss through cause.get_ref() and downcast_ref::<KaniCrossRoleCauseLoss>(), without
parsing Display. The marker denotes projection loss, not the original source or a reconstruction
of its type, value or chain. Its presence grants no role/site authority and supplies no classification.
Only typed loss detection is public; private role/site custody adds no public getter or stage query.
Debug/Display are diagnostic, not provenance selection, classification or a semantic parsing API.
C shall not attach it to a loss-free original errno or payload-free cause; an OS cause remains
directly reconstructed, preserving public raw_os_error and kind. io::Error::new discards a raw errno,
so this marker construction is permitted only when the original raw_os_error is None. Local original causes and AC-39's
finite representable typed public sources shall not be replaced by this generic marker allocation.
Private finite provenance remains independently authenticated and is not a public stage-query API.
This adds a public loss-marker type, not a refusal variant, error code or classification catalog.

The cause model has exactly two disjoint domains. ValidOriginal contains only successfully captured
original kind/errno and payload-presence/loss facts for a listed operation. Its finite dependency
facts remain actual EmptyFilter, Prctl and Seccomp with nested I/O facts, and ThreadSync with its
actual PID; an observed Reservation payload supplies no allocation-kind/layout inference. A boxed
first-party operational cause receives the same original-payload loss treatment as other original
custom causes unless a stronger existing public-source duty applies. It shall not be reconstructed
from a representation-error name. CauseIntegrity contains failures of required capture/checking/
representation instead of a valid original cause; a representable Other wrapper for such a failure
shall not move it into ValidOriginal. The sending/checking owner and C shall select the domain from
the actual typed operation and failure predicate, never kind, source presence, diagnostics or a
similarly named CODE variant. Adding a CODE variant shall not enlarge either declared domain.

The following single finite inventory governs every CauseIntegrity disposition in this subsection,
including its public carrier/detail-only rules and TC-049 oracle. Checker provenance identifies only
the independently known role/site that actually observed the predicate: sender capture and C-side
projection are distinct, and neither shall be inferred from an unverified packet label.

| CauseIntegrity predicate | Actual observed failure |
| --- | --- |
| RequiredRepresentationExceededBound | Required cause representation exceeds its existing bound. |
| RequiredRepresentationFormattingFailed | Formatting required cause representation fails. Optional diagnostic formatting after a valid original cause is retained is excluded. |
| UnnameableOriginalKind | The capturing owner cannot encode an actual original no-errno kind in the established producer domain; this is not a normalized original cause or support-gate acceptance. |
| OriginalOsKindMismatch | Actual capture/checking finds the kind and raw-errno representation inconsistent with actual OS conversion; no alleged original kind is replayed. |
| NonInstallationDependencyCause | Actual installation-cause capture/checking receives a dependency cause outside the allocated installation domain. |
| PolicySiteCauseMismatch | Actual checking/projection finds the cause domain inconsistent with the independently authenticated policy site. |
| UnknownKindMetadata | The checking receiver observes unknown kind metadata, without claiming that this was the producer's original kind. |
| MalformedCauseMetadata | The checking receiver observes malformed required cause/envelope metadata; optional diagnostic payload decoding alone is excluded. |
| IncompleteCauseMetadata | The checking receiver observes incomplete required cause/envelope custody; an incomplete UTF-8 scalar in otherwise complete optional diagnostic bytes is excluded. |

A complete authenticated sender fault report supplies only its actual observed predicate and known
provenance. A C-observed checking failure supplies C's actual predicate and independently known
origin facts; it shall not be described as a delivered sender fault. Neither supplies an original
remote boxed source. Missing delivery supplies no inferred sender predicate. Unlisted failure
predicates require a reviewed SPEC allocation, not a kind-only replay, diagnostic-derived category
or substitution of another predicate. ValidOriginal unlisted custom sources use opaque loss only
within an already listed operation and the representable outer I/O domain.

The supported no-errno producer/build domain is closed to these named ErrorKinds: NotFound,
PermissionDenied, ConnectionRefused, ConnectionReset, HostUnreachable, NetworkUnreachable,
ConnectionAborted, NotConnected, AddrInUse, AddrNotAvailable, NetworkDown, BrokenPipe, AlreadyExists,
WouldBlock, NotADirectory, IsADirectory, DirectoryNotEmpty, ReadOnlyFilesystem, StaleNetworkFileHandle,
InvalidInput, InvalidData, TimedOut, WriteZero, StorageFull, NotSeekable, QuotaExceeded, FileTooLarge,
ResourceBusy, ExecutableFileBusy, Deadlock, CrossesDevices, TooManyLinks, InvalidFilename,
ArgumentListTooLong, Interrupted, Unsupported, UnexpectedEof, OutOfMemory and Other. This finite
encoding domain does not assert that every listed kind is emitted by every producer. Before CODE
delivery, source-flow Analysis shall exhaustively establish that each actual declared sender operation
in the supported build emits only these no-errno kinds or retains an actual OS errno from which C
can verify its actual kind. A producer, dependency or toolchain change introducing another emitted
no-errno kind shall update the producer-side finite encoding under CODE review before delivery.
Current generic sender integration and this exhaustive CODE gate remain PLANNED/UNRUN; source
inspection of available producers does not prove future unwired paths or universal toolchain support.
No supported genuine admission cause shall become Tool or another refusal merely because of kind:
C shall retain the existing authenticated site-first Unavailable mapping regardless of its actual
kind/errno. C shall not normalize or replay an original cause outside the established producer
domain, invent its kind/errno/source, or accept it through a compatibility fallback. The separate
fail-closed integrity disposition for an unexpected required encoding fault, including
UnnameableOriginalKind, reports the actual encoding failure rather than a normalized original cause.
It neither admits that emitted original kind into the supported domain nor satisfies or waives the
producer-side source/encoding gate before CODE delivery. A genuinely emitted unsupported original
kind leaves that CODE gate unsatisfied until a reviewed producer encoding update.

Unknown kind metadata, incomplete cause custody or malformed cause representation is an actual
cause-metadata integrity fault, not a valid original producer cause. Planned CODE shall expose
`KaniCauseMetadataIntegrityError` alongside KaniExecutionRefusal at the public crate boundary.
This opaque error implements Debug, Display and std::error::Error with Send + Sync. Its constructor,
actual typed predicate and finite provenance fields remain private. Each role/site provenance field
shall be a typed Option containing only a fact independently authenticated by C; a role may be
present while the site is absent, and both may be absent. C shall preserve every actually known
field and shall never fill an absent field from an unverified packet label. A reported sender identity
shall remain only a claim until independently authenticated. Absence shall not prevent
construction or public integrity detection, nor supply admission-site authority. Its
private predicates are exactly those in the CauseIntegrity inventory above;
C shall select the predicate from the actual observed failure, never prose. It shall retain any
actual local decoder/control error, exposed by its source() when present, without an invented
original producer payload. A caller shall identify integrity failure only by
cause.get_ref().and_then(|error| error.downcast_ref::<KaniCauseMetadataIntegrityError>()).
Presence of this typed error identifies the actual local integrity fault; neither InvalidData,
get_ref().is_some(), source absence/presence, source type-name text nor Display alone distinguishes
it from an original producer/local cause. Public predicate/role/site getters or stage queries are
not allocated. Debug/Display remain diagnostic and shall not select a classification.
For this fault ONLY, when the originating admission site and negative startup state are independently
authenticated from retained run/build/role/identity/control authority, C shall use
BoundedLaunchError::Unavailable with that site's actual mandatory KaniStartupAdmissionCause and
io::Error::new(ErrorKind::InvalidData, actual_integrity_error). After confirmed settlement its public
result is MemoryMechanismUnavailable { admission, cause }, code()==None, with no evidence/outcome.
Here cause is explicitly the actual C-observed integrity error: its kind is InvalidData and its raw
errno is None; it is NOT the original producer cause, whose kind/source cannot be replayed from the
faulty metadata. This narrowly qualifies the original-cause promise for this integrity case only.
C shall attach no KaniCrossRoleCauseLoss marker: there is no established original custom source
projection to mark, and the local integrity source shall remain the actual
KaniCauseMetadataIntegrityError. No fabricated original kind, errno, source or original-cause replay
is allowed.

If the independent originating admission site/state cannot be authenticated, C shall not invent
admission context. It shall report the actual startup/control failure under the table's existing
startup/protocol path. If an actual C-side ownership/protection check cannot establish the required
ownership/protection, that check's real Unavailable cause and known C-side admission context remain
required; C shall not substitute or label it as a cause-metadata integrity error merely because an
invalid packet was also received. Otherwise C shall report the actual metadata-integrity error as
Tool(KaniToolError::Io { tool: KaniTool::Launcher, path, error }) with the same
io::Error::new(InvalidData, KaniCauseMetadataIntegrityError). Public get_ref/downcast therefore
identifies integrity in either mapping where it is the actual reported cause. Actual unrelated
C/launcher transport I/O retains its existing Tool path. These are distinct from an authenticated admission site's cause-metadata integrity fault;
that fault shall never become Tool solely because the producer kind metadata is invalid.
C shall retain already authenticated facts and actual owners through original bounded settlement.
Neither invalid metadata nor a typed integrity error authorizes Dispatch or evidence.
CleanupUnconfirmed retains its existing candidate override. This allocates no new public refusal
variant, field, code or integrity catalog. The public opaque error retains actual C-observed integrity
provenance or an authenticated finite sender representation-fault report, not an original boxed
producer error or a promise to transport an unnamed original ErrorKind.

When an actual post-admission observation/accounting failure is reported, including a required cause
representation or metadata integrity fault that prevents the actual observation, C shall retain the
existing MemoryObservationFailed { detail } result with no evidence/outcome after required settlement.
The actual finite producer cause/kind/errno facts, independently authenticated provenance, explicit
source loss and any representation/integrity predicate remain private custody. Since this public
variant has no io::Error field, C shall attach no public marker/error, promise no public get_ref/
downcast or original source access, and add no error carrier. This states the existing public-shape
limit, not removal of an existing observation cause API. detail remains diagnostic; its presence,
absence, wording or truncation shall not select a cause or mapping. C shall not change observation
failure to Tool or admission Unavailable just to expose a public cause. A malformed observation
cause report supplies only its actually observed private integrity facts, not fake original cause
replay. Genuine independent protocol/ownership/settlement failures retain their separately specified
actual paths and candidate precedence; CleanupUnconfirmed still overrides when settlement fails.
The same scope rule applies during settlement: this allocation changes no existing final failure
variant to create an I/O carrier, and all finite facts/loss remain private when that variant is detail-only.

When the actual producer uses io::Error::other with a reservation payload, C shall preserve its
observed Other kind. When the producer uses the standard From<TryReserveError> conversion, C shall
preserve its actual resulting kind and observed source presence/absence, according to the table.
A standard conversion that yields OutOfMemory establishes neither allocator failure nor allocation
layout. C shall not assume a source is absent merely from the conversion's name: source-retaining
and source-dropping implementations use their actual observed case. A changed producer conversion
requires CODE review and never authorizes transport normalization.

Required cause representation can itself fail at the sending owner before a valid original cause
representation exists; a representable outer wrapper for the encoding failure is not such a
representation. The sender shall retain that actual typed representation failure and select only its
observed predicate from the single CauseIntegrity inventory, including an actual non-installation
dependency cause or policy-site/cause mismatch when observed by that sender.
KaniCauseMetadataIntegrityError retains the independently known checker role/site; C shall not relabel them as
original producer I/O kinds, optional diagnostic failure or generic custom-source loss. When the
existing bounded authenticated negative control can carry that finite fault report, the sender shall
send it using the same reserved control storage and limits, omitting original-cause replay. C shall
construct io::Error::new(InvalidData, actual_integrity_error) for an existing I/O-cause-bearing public
result, from the authenticated fault predicate and independently known provenance. For that public
projection its kind/raw errno are InvalidData/None, the public
integrity downcast is Some and the loss-marker downcast is None. C shall use the same explicit
site-authenticated Unavailable versus non-site startup/protocol mapping specified above for cause-bearing
results; a work/settlement observation failure instead retains the explicit detail-only private-custody
rule. No public cause carrier is added, and evidence/Dispatch remain forbidden for these error paths,
with unchanged settlement precedence. The cause denotes the actual required
representation failure, not an alleged original remote I/O error or original boxed representation
object. It retains actual finite predicate/origin facts, not an invented original payload.

When C itself detects NonInstallationDependencyCause or PolicySiteCauseMismatch during actual
capture/checking/projection, it shall retain that actual C-observed predicate with independently
known role/site facts and use the same cause-bearing integrity or detail-only rule. No sender fault
report is required for a positively observed C-side failure, and no sender provenance is invented.

If no complete fault report is actually delivered, C shall not claim that a sender predicate was
received or infer it from EOF/timeouts/diagnostics. An actual incomplete, malformed or unknown cause
packet follows the C-observed metadata-integrity rule above; other actual transport/ownership errors
retain their explicit existing paths. The sender retains its real failure/owners through the original
bounded settlement; this rule creates no alternate control, acknowledgment, capacity or deadline.

When a listed refusal's typed cause is representable, the sending owner shall retain that cause
before attempting optional diagnostic capture. If diagnostic capture exceeds its existing bound or
fails formatting, then the sending owner shall omit the diagnostic detail and send the unchanged
retained typed cause; it shall not substitute a context/formatting error. C shall select its mapping
from that original authenticated cause. The required-representation-failure case above applies only
without a previously representable original cause; it shall not override retained original cause
custody because optional detail failed. Optional detail shall remain bounded UTF-8 diagnostic data
and shall supply no cause, role, operation or public classification.

Required cause metadata includes actual kind/errno facts and mandatory envelope identity, authority,
site/state, applicable stop and field-presence/type/framing obligations. A mandatory context field
remains required envelope structure: absence, duplicate fields, a non-byte-array value, non-u8
members or broken/incomplete enclosing framing cannot be accepted as an omitted diagnostic.
Omitting diagnostic text shall preserve that structure with empty context bytes. Within a complete
bounded structurally valid context byte array, the bytes are optional diagnostic payload, not
kind/errno or authority metadata. C shall discard diagnostic text that exceeds its retention bound,
is invalid UTF-8 or ends in an incomplete UTF-8 scalar, without changing an independently validated
and authenticated original cause, its public errno/kind or mapping. Such text supplies no integrity
predicate and no classification duty; public docs shall permit absent/lossy diagnostics. Existing
whole-control encoded-byte limits and complete-frame/ancillary/authentication checks remain
mandatory: whole-packet truncation or invalid required metadata uses the actual CauseIntegrity
predicate and existing carrier/detail-only mapping, never a claimed valid original replay.
A decoder shall distinguish those structural/mandatory failures from optional payload rendering
failure; failure of the current combined context decoder is not implementation evidence that both
are cause-metadata faults. This distinction remains CODE-gated/UNRUN and allocates no new field,
frame, capacity, acknowledgment or deadline.

The sending helper shall charge its actual encode/context storage to its measured owned-role
accounting and existing backing terms. C shall charge actual receive/context/metadata storage to
named caller_run_buffers before L creation. Both sides shall enforce the existing complete-control
encoded-byte/ancillary/pending-work bounds and whole-run ceilings; neither side's charge replaces
the other's. No arbitrary source-chain serialization, unstable feature, new acknowledgment, phase
cap or resource allowance is allocated.

C's bounded-execution API and MemoryMechanismUnavailable rustdoc shall explain this listed cross-role
projection limit, typed public loss-marker detection, observed source presence/loss and unchanged
local original-source retention.
Public rustdoc shall state which existing refusals carry io::Error and permit these typed public
projections, versus detail-only MemoryObservationFailed and other existing detail-only results, whose
finite facts/loss are private and whose diagnostics promise no original-cause or downcast access.
Public rustdoc shall distinguish an authenticated admission cause-metadata integrity fault or
complete finite sender required-representation-fault report in a cause-bearing result through
get_ref()/downcast_ref::<KaniCauseMetadataIntegrityError>(): its Unavailable cause is the actual
local InvalidData/no-errno integrity error, not replay of the original
producer cause, and it has no projection-loss marker. It shall state the independent site-authentication
precondition, existing unauthenticated-control path and lack of a known original kind/source in that
case; callers shall not read this integrity cause as an original producer error.
The documentation shall promise no unavailable public role/stage query. The representation and
public consumer flow remain CODE-gated. These obligations allocate no NotObserved reason and weaken
no report, capability, ownership, refusal precedence or cleanup requirement.

### Stage-2 live-birth and producer-operation observations

This PLANNED/UNRUN allocation conditionally retires only TC-049's former literal child birth
AFTER the FINAL whole-run sample oracle. For successful report Commit, O shall complete the final
accounting observation only after actual claimed I termination confirmation, retained M Child
reap, writer EOF and immutable seal. O shall preserve every later due ordinary observation through
final delivery; a final sample before inner settlement shall invalidate this retirement. The CG
CODE author shall supply Analysis of the published integrated source and its actual transition
order; the independent CODE reviewer shall check it before permitting the replacement. A claimed-I
descendant cannot fork after confirmed I teardown. This stronger construction does not assert
universal kernel success: failed/unconfirmed settlement remains refusal. AC-8, AC-10 and the
separate confirmed whole-outer-tree guarantee remain required, including distinct O-origin
inner-confirm/M-reap/outer-confirm observations. Inner confirmation never supplies outer confirmation.

Where guardian-test-support is enabled, the single live-caller fixture operation shall select
bounded typed scenarios through the unchanged production transitions. The live-birth scenario
shall positively acknowledge a real adopted orphan while its owned I remains live. After an actual
completed ordinary O observation of the complete accounting formula, the controller shall release
that same worker to fork exactly one child. The child's own positive acknowledgement shall prove
release-dependent birth. The operation shall validate BOTH actual identities and retain BOTH live
pins and genuine authenticated I Completed before unchanged cleanup; M exit or a stage token
shall not substitute. These are construction preconditions, separately checked and reported, not
inferred from sample-membership absence. Missing any precondition shall yield typed fixture failure.

The sampled-membership-only teardown regression class shall be verified by source Analysis,
not an unproved runtime child-absence window. Closing C's original lease starts a request for I
termination; it is not a last-sample boundary. O shall continue ordinary samples while I observes
EOF and while inner settlement/report EOF/delivery progress. No existing bound or event record
is claimed to make the child absent from every completed sample up to an actual control kill.
The fixture shall still retain BOTH live then dead pins before outer escalation and actual I/M
settlement, without claiming that the child stayed unsampled until death.

The CG CODE author shall supply exhaustive Analysis of the actual contained teardown authority
paths, including claimed-I lease EOF/INIT exit and retained-INIT signalling, unclaimed bootstrap
recovery/cancellation and L/O outer cancellation/settlement. The independent CODE reviewer shall
check that sample membership is used only for accounting/observation, never as a descendant kill
list or cancellation authority. Any path using observed PID membership for teardown shall fail
this Analysis; absence of such a path in an earlier snapshot shall not complete final-source
Analysis. Claimed namespace descendants shall remain cancelled through actual INIT death.
This allocation preserves AC-8's absent-from-previous-samples guarantee. The independent AC-24
non-INIT-watcher runtime mutant and its restored control shall remain mandatory and unrescued by
outer cleanup. That Test remains owed until genuinely executed; source Analysis shall provide
neither its completion nor sampled-membership-mutant runtime credit. No sampling pause, new
control-kill seam or impossible child-absence acceptance precondition is allocated.

The harness shall independently verify actual retained-I pidfd termination confirmation and actual
retained M Child consuming wait/reap strictly before the matching kernel report seal. Actual original
I stays INIT after authenticated Completed until lease EOF or existing owned cancellation; Completed,
phase, a token, stored category or eventual dead pin shall not prove the earlier operation. I
termination authority remains the original C lease EOF and existing owned cancellation.

Fabricated live-I IN and fabricated M Some(status) with actual M unreaped shall independently fail
while the underlying call/result alone is mutated. Any independent observation/checker shall remain
unchanged; a test that deletes its evidence together with the operation shall not count. Real I poll
or M reap moved after report sealing shall fail strict order, not pass because an ordinal says so.
No emitted record subsystem or synthetic settlement token is allocated to establish these facts.
If independent operation/order observation cannot be constructed, that Test remains UNBACKED.

The retained-gate case shall separately require actual owned I termination confirmation while the
original O gate is retained, followed by actual gate close. Early-close and omitted-confirmation
mutants remain independent and shall not pass from marker absence or later teardown. A cached C
phase or serialized gate-retained flag cannot establish this predicate. Genuine bounded production
signal/confirmation/close integration remains owed, as does an independent witness of that actual
order. No feature-only kill authority, pause or replacement gate transport is allocated.
The ignored-inner-EOF mutant shall remain separate and unrescued by outer cancellation. AC-77's
post-COMMIT cancellation remains required but its termination shall never supply lease-EOF evidence.

Every original assertion, adverse patch and FR-028-AC-21/FR-017-AC-24 trace obligation shall remain
until measured replacement parity. Parity requires restored controls to pass and each named
independent adverse case to fail its specific predicate before emergency cleanup: fabricated live-I IN, fabricated M reap, I poll after seal, M reap after seal, gate close
before I confirmation, omitted gate-path I confirmation, and ignored inner EOF. The CG CODE author
shall supply per-predicate runtime and source/bounds receipts; the independent CODE reviewer shall
check each before accepting the single lifecycle CODE PR. The sampled-membership regression class
requires independently accepted final-source AC-55 Analysis plus the genuine AC-24 non-INIT-watcher
adverse/restored Test; neither is inferred from a matrix row, and no sampled-membership runtime
control is claimed. Replacement trace bindings shall transfer
only with full genuine original assertions; no partial test may bind an entire mixed criterion.

The fixture shall immediately invoke unchanged owned cleanup after observation success, refusal
or error, and return no cleanup-deferring callback/process handle. Independent behavioral checks
shall observe the actual unconditional operations; fixture selection shall not change authorization,
cancellation, ordinary ticks or production stages. No controller pause or production callback is
allocated. Before fixture CODE, actual kernel observation sources, mutation boundaries, integrated schedule,
coordination/storage bounds and simultaneous charged lifetimes shall be grounded. No current
operation token or stage ordinal is claimed to provide those capabilities. A dedicated off-default,
noncaller-tunable coordination bound shall be justified by real one-child birth/identity/live-pin
measurement and observable load margin, clamped to the original remaining monotonic deadline.
No numeric cap is selected here; existing settlement or post-close allowances shall not fund extra
coordination. Missing genuine authority/bounds or insufficient remaining budget shall block fixture
implementation or yield typed failure, never skip/pass or raised headroom. No DTO, hook, new right,
public role field or runtime implementation is delivered by this allocation.

### Run artifact and report lifetime

FR-017's internal named report allocation is replaced explicitly by a child-only anonymous pipe
writer and trusted O collector into initially empty sealing-capable memfd. The actual backend argv
ends with `-Z unstable-options --export-json /proc/self/fd/N`; N is its actual mapped writer >=5.
The consumer holds actual descriptor authority, never reopens a proc symlink with pathname-based
NOFOLLOW rules. Original non-report argv bytes, stdin/environment/cwd and captures are preserved.
Kani 0.68's export uses `std::fs::write`; this source fact makes the pipe plausible but does not
prove cargo inheritance or absence of a seek/reread expectation. A failed real roundtrip stops CODE
and requires a measured spec revision, never a runtime fallback or named compatibility mode.

O's normal bounded event loop concurrently handles report readiness, controls, deadline and
accounting while the backend runs. It never blocks waiting for backend completion while that backend
waits for pipe capacity. Retained memfd has a hard 16 MiB plus one detection-byte maximum; Safe
descriptor ownership excludes leaked writable memfd aliases; unavailable sealing, allocation or
conservative backing accounting causes typed refusal and bounded owned cancellation. The measured
pipe capacity is finite and included even when unused; backing is never assumed free. Actual bounded
pipe capacity and all kernel memfd backing are concurrently charged to the original whole-run
ceiling, including unmapped shmem that process RSS misses. Over-limit input cancels owned execution
and yields single-run `KaniRunOutcome::Inconclusive` with `KaniInconclusiveReason::MemoryExhausted`;
evidence names the report cap independently of the identity ceiling. Batches keep whole-batch
memory-exhausted refusal with no member classification. FR-029 maps the reason to
`Incomplete(ResourceExhausted)`, never `Failed`. Independently established resource/deadline stops
take precedence over malformed partial content with existing single-run or whole-batch behavior;
otherwise malformed content remains refusal, never truncated acceptance. Slow collection and
backpressure must terminate within the original deadline, not deadlock.

The conservative charge oracle is `owned_RSS + caller_run_buffers + pipe_reserve + memfd_reserve`.
`owned_RSS` sums positively observed RSS of L/O/M/I and every owned backend descendant without
omitting unsampled live workers; ambiguous observation refuses under FR-028. `caller_run_buffers`
uses the declared finite upper allocation bound of C's per-run controls/captures, not unrelated C
memory. `pipe_reserve` is page-rounded actual F_GETPIPE_SZ measured before writer exposure; resizing
authority must be excluded or a conservative finite maximum capacity pre-reserved before Dispatch. A
writer able to grow capacity beyond that reservation invalidates accounting and requires refusal; a
runtime test attempts that actual resize. `memfd_reserve` is page-rounded 16 MiB + 1 reserved BEFORE
writing, rather than racy i_size or st_blocks. It conservatively includes mapped memfd pages again;
no subtraction/deduplication may create an undercount. Page size and checked arithmetic are
verified; absent quantity, failed bound or overflow refuses setup/observation. Compare the checked
sum to the original memory ceiling at setup and every original observer tick; crossing cancels under
existing memory-exhausted rules. The hard collector bound ensures backing never exceeds its
pre-reserved maximum between samples. Evidence distinguishes this conservative total from actually
observed tree peak RSS. Capability, backing-reservation validity and sampling coverage must be
tested, never presumed from read size.

O's fresh private proc shall observe O/M/I and their owned descendants; it shall not be assumed
to expose parent launcher L. O shall obtain an actual fresh L RSS observation at setup and EACH
original accounting tick from an authenticated source valid while that private proc view is in
place. The source shall be bound to the original run, actual owned L pidfd, recorded start/TGID
identity and checked liveness. Missing, stale, replayed, unbound or unavailable L observations shall
not be replaced by an omitted term, proxy or a self-reported value reachable from the contained
backend tree. Zero shall require the existing positively observed identity-matched address-space
release rules, never missing live-L RSS. This requirement allocates the observation properties,
not a particular descriptor handoff or host-view sampling implementation; the selected Safe
implementation shall prove those properties before CODE delivery.

Trusted C shall derive `caller_run_buffers` from real finite own allocation caps of its named
per-run controls/captures, including bounded diagnostics. O shall receive that bound from an
authenticated C-origin source bound to the same original run, ceiling and deadline, with checked
arithmetic. A requested stack alone, C RSS proxy or opaque incidental-runtime estimate shall not
establish those named caps. O shall combine the fresh authenticated L observation, fresh private
owned-tree observation, authenticated named caller-buffer bound and actual pipe/memfd reservations
into one checked complete charge at setup and every original observer tick. Before any report
writer exposure or Dispatch, an actual complete within-ceiling setup sample shall be required.
Missing, stale, unbound or unavailable named input shall refuse under existing setup/observation
rules, never omit L, a caller cap or backing. After admission, unavailable actual L observation
shall follow existing `MemoryUnobserved` to `MemoryObservationFailed { detail }` execution error
with no evidence; pre-Dispatch inability to establish the required observation capability shall
retain the existing typed unavailable admission path. No mid-run `NotObserved` reason is allocated.

Any trusted L observation and caller-cap authority shall stay owned/CLOEXEC outside intended
trusted mappings and unreachable from arbitrary backend/descendants. The source shall preserve O's
required private proc/PID view and shall not expose host observation authority to the backend,
create a public hook or relax ownership/control lifetimes. Current-tick completeness, original
schedule, ceilings/deadlines and confirmed final settlement shall remain mandatory; neither a
successful bootstrap nor one setup sample establishes later coverage or an evidence conclusion.

The resource accounting fault domain excludes incidental caller-process allocations outside the
named terms. Opaque Command implementation storage, caller thread/native runtime, TLS,
guard/alternate-stack and allocator transient allocations beyond the named per-run controls/captures
are not charged, bounded or claimed by this requirement. Allocator bookkeeping, chunk overhead
and rounding of a named allocation are also excluded from its named bound. Its actual returned
Rust capacity multiplied by element size, and its named metadata, shall still be charged; this
explicit allocator-overhead exclusion shall not omit either quantity. This exclusion shall not omit
positively observed launcher L RSS, any other owned role/descendant, named `caller_run_buffers` or actual
pipe/memfd reservations. Named controls/captures shall retain real finite allocation caps and their
charge; an already allocated named quantity shall not be reclassified as incidental to omit it.
A complete conservative-charge observation shall cover every named formula term and the actual
owned tree; it is not whole-C memory or a complete caller runtime allocation bound. No whole-C RSS
proxy, requested-stack proxy, raised budget or fallback shall supply missing named accounting.

### Retirement of stored lifecycle certificates

The former Stage2ObservationRecord, five-event slots/codec, producer ordinals, separate copied
completion carrier, observation pipe and Stage2ObservationBinding writer transfer are retired.
Their sole-purpose early MonitorSpawned report-identity field, decoder branches, endpoint mappings,
profile-query storage and native workspace shall be removed with them. No replacement record,
DTO, right, channel, ACK, callback, timer, cap or certificate is allocated. AC-78 through AC-100
retain their identifiers below with explicit amended obligations; this retirement grants no CODE
or runtime completion. Historical review findings/dispositions remain historical data.

The actual original O/build/run admission, genuine I Completed reception before original lease
close, retained report PipeIdentity and report delivery authority remain unchanged. C shall not
infer genuine I completion from M exit, EOF, phase or constructed metadata. AC-94's close-only
queued-claim receipt and original-cause/settlement precedence remain unchanged. Ordinary feature-on
execution shall require no retired binding or early report field. Feature-off paired consumer/helper
frames, bytes, rights and production control/sampling behavior shall remain unchanged, with actual
source/artifact absence Analysis and mechanical checks required for retired items.

For M provenance, only the actual O parent may perform same-original-M nonblocking WNOWAIT and
post-wait checks around its FIRST uncached retained Child Some(status). Exact original parent,
sole-waiter history, SIGCHLD neither ignored nor SA_NOCLDWAIT and no prior consuming wait shall be
established. The post-query ECHILD alone cannot distinguish competing waiter, auto-reap or wrong
parent; the independent verifier shall check the actual operation and those premises. Fabricated
Some, cached Some, competing wait, auto-reap, live None and a bare or unconsumed zombie status
shall not satisfy genuine consuming-reap provenance. A waitable zombie positively established by
same-child WNOWAIT shall pass the restored control when the actual parent's FIRST uncached Child
Some performs its consuming reap and the independent post-query returns ECHILD under the established
sole-waiter/disposition/history premises. No stored event or copied history provides this proof. Safe method and
production-profile feasibility remain PLANNED/UNRUN; the controlled standalone probe supplies no
production or mutation credit.

Every due complete ordinary sample, original control transition and cutoff shall continue without
observation permission or suppression. Real adopted-orphan acknowledgement, actual release-dependent
birth after a complete ordinary sample and BOTH live/dead pinned identities remain independently
owed under AC-51/92 and AC-8/10. This amendment does not provide a new sample-announcement mechanism
or coordination capability. Missing construction remains UNBACKED; it never permits a synthetic
sample or paused tick. Actual I/M/report/gate operations and all named adverse/restored controls
remain required before measured replacement parity and transfer of old trace bindings.

Removing a sole-purpose allocation shall remove its charge only when its actual allocation and
all overlapping lifetimes disappear. All surviving reporter, control/capture, identity, parser,
query and native storage shall remain charged before exposure through the existing named formula,
actual capacities and independently declared native-workspace proof. C's named storage shall remain
caller_run_buffers, O's storage owned_RSS, and every retained pipe/backing reservation shall remain
actual and additive. No logical length proves capacity; no feature-off proof backs retained
feature-on workspace. Missing initializer/highwater, capacity or simultaneous-lifetime proof leaves
conformance UNPROVEN. No new cap, charge term or allowance is allocated.

### Conditional signal safety without certificate writes

This amendment removes the sole-purpose observation writer, not the safety condition on any
remaining optional fixture write. No new write or signal policy is authorized. If such an existing
write is exercised, actual original O shall be proven SIGPIPE ignored AND unblocked, independently
untraced across PID namespaces, single-threaded and free of competing signal consumers, with stable
disposition/mask/tracing state through each write. Authenticated same-O proc identity, SigIgn,
SigBlk, SigPnd, ShdPnd, TracerPid and task/thread facts shall be completely checked. Private-proc
TracerPid zero may hide an ancestor tracer; independently checkable initial-state evidence valid
across PID namespaces is required. Inherited/default disposition, one snapshot, source absence of
thread creation or socket NOSIGNAL alone shall not establish eligibility.

Actual normal-helper live-reader/broken-reader controls, complete supported kernel/syscall/runtime
configuration, permission and full accounting/native-overlap proof shall precede positive writing.
Default, caught, blocked, traced, multithreaded, changing, malformed/missing/unavailable or unproved
profiles shall permit no write and leave ordinary sampling/control/settlement and cutoff unchanged.
No handler installation, mask change/restoration, pending drain/recreation, global policy or new
transport is allocated. Preexisting foreign-origin pending signals shall remain unchanged. An
optional write fault shall not terminate O, change cancellation or rescue the ignored-EOF mutant;
missing independent signal proof leaves eligibility UNPROVEN/no-write. These conditional obligations
remain PLANNED/UNRUN and confer no activation or fixture acceptance credit.

### Supported-build proof of named native workspace

For each exact consumer build configuration claimed to conform to the named
`caller_run_buffers` accounting above, CG's CODE verification shall establish a
finite conservative bound on the actual native workspace used by its named per-run
control/capture operations. Missing, incomplete or stale proof shall leave that
configuration UNPROVEN and fail its conformance claim. Neither C nor O determines
proof availability at runtime. This gate allocates no new runtime refusal, public
cause, request field, timeout or budget.

The analysis input shall be the actual final consumer executable, identified by its
consumer workspace manifest, selected package and binary target, and the matched
normal helper artifact required above. A library archive is not that input: this
package's `make build` at the specification baseline builds a library, not the
consumer executable. Dependency release profiles and dependency toolchain files do
not select the consumer workspace's profile or compiler. The analysis shall record
the actual resolved consumer profile, target triple/ABI, enabled features, compiler
selected by that consumer's build environment, codegen/linker/CPU flags, panic
strategy, allocator, and loaded runtime libraries and their configuration. Feature-off
production and feature-on verification are separate configurations. The exact
source/build/compiler/artifact attribution belongs in operational evidence, not a
committed source pin or checksum catalog.

The initial planned evidence input is a consumer final executable on luna for Linux
`x86_64-unknown-linux-gnu`, with its consumer workspace explicitly selecting release
thin LTO and one codegen unit, no verification feature, and the compiler selected by
the existing build environment. These are required resolved inputs, not a claim that
`make build` supplies such an executable or that any native workspace is proved.
Other audited native policies permitted by the existing contract remain candidates,
including Linux little-endian aarch64/riscv64, and remain UNPROVEN until their own
current exact-configuration evidence exists. This allocates no implementation claim
for those policies and does not prohibit other consumer configurations.

CG's CODE author shall produce the operational Analysis receipt, and the independent
CODE reviewer shall check its completeness and correspondence to the delivered
configuration. The single IR-639 CODE PR shall not merge with a native-accounting
conformance claim until this gate is satisfied for its claimed initial configuration.
Matching caller/helper artifacts, prior Tests or a computed matrix method row cannot
replace an executed Analysis receipt. Each criterion below shall have an explicit
completed or UNPROVEN disposition with its evidence; all remain PLANNED/UNRUN until
that receipt exists.

The verification shall use GNU `objdump` or `llvm-objdump` on the actual final linked
executable after LTO and linking, and on relevant actually loaded external libraries.
Its operational receipt shall name the tool used and record its actual version.
Stable compiler/tool inputs are required; nightly `-Z` facilities and
`RUSTC_BOOTSTRAP` shall not supply this proof. Pre-link emit-asm/emit-obj does not
substitute for the final executable. The analysis shall map every named control/
capture source operation to its final machine-code address ranges using available
symbol and debug/inlining information. If inlining removes a standalone symbol, the
analysis shall cover the entire containing host frame and its relevant reachable
paths conservatively. An unresolved root/address mapping leaves the claim UNPROVEN;
a source function name alone does not identify a final root.

For every thread carrying named per-run control/capture state, the analysis shall
identify its entry roots, including indirect thread-start roots, and all relevant
success, refusal, cancellation, error and cleanup paths. Every reachable indirect
call target shall be enumerated or the site proved unreachable. Reachable recursive
cycles, unresolved targets or unknown concurrently live thread counts/lifetimes leave
the bound UNPROVEN. The analysis shall derive a per-thread maximum of simultaneously
live frames, including actual tail-call replacement and stack adjustments, then sum
the maxima for all concurrently live thread instances. One global maximum or a sum
of nested call frames in only one thread is insufficient.

The resolved panic strategy shall govern the actual unwind/landing-pad and cleanup
edges included, or their demonstrated absence under abort. All frames reached while
carrying named state, including runtime/libc/allocator frames, shall have an explicit
finite bound source applicable to the actual loaded ABI/configuration: analysis of
that loaded machine code or an authoritative finite bound with matching premises.
A dynamically loaded runtime is not accounted for by disassembling only the main
executable. Unknown dynamic stack adjustments, external bounds, runtime selection or
unwind edges leave the claim UNPROVEN, without a guessed reserve. Unrelated incidental
runtime allocations, TLS, guard pages and other previously excluded terms remain
excluded; no frame or allocation holding named state may be relabelled incidental.

For each named retained or temporary heap allocation, the inventory shall use its
actual returned Rust capacity multiplied by element size, plus named metadata and
checked simultaneous-lifetime overlap. For `Vec` and `String`, requested constructor
capacity alone is insufficient if the returned capacity is larger. Allocator usable
chunk size, rounding and overhead are not this capacity term and are excluded by the
explicit allocator-overhead boundary above. Temporary decoding, formatting and
collection overlap shall be included; conservative duplicate counting is allowed.
Subtracting a named live term requires a justified lifetime proof.

The runtime carrier shall be an independently declared finite private native-workspace
charge included in the existing `caller_run_buffers` sum before L creation or named
storage exposure, with checked arithmetic and the existing named heap terms. Its
implementation and actual accounting path are IR-639 CODE gates, not implemented
facts asserted here. The final-artifact Analysis shall prove the required bound is
no greater than that declaration. It shall not write the computed bound back into
the executable it just analyzed: a declaration/source change requires a new final
build and new analysis. A missing actual named charge retains the existing runtime
missing-input refusal; a missing proof instead fails the conformance CODE gate and
leaves the configuration UNPROVEN. C/O do not convert proof availability into runtime
admission. No numerical reserve, raised product cap or new transient exclusion is
allocated here.

Any change to source, resolved consumer configuration/compiler, final executable,
loaded runtime/allocator or the premises of an external bound shall require analysis
again before claiming conformance. The receipt shall state roots/address mappings,
thread instances/lifetimes, complete edges/targets, frame/heap bounds, declared charge
and checked comparison, and every unresolved fact. Any required path or quantity
without a finite applicable bound leaves the entire claim UNPROVEN.

Every emitted `KaniExecutionEvidence` shall carry the mandatory field
`charged_peak: ChargedPeakObservation`. The planned typed variants are
`Observed { bytes: u64 }` and `NotObserved { reason: ChargedPeakNotObservedReason }`.
`Observed` shall be used if and only if O obtained an actual complete conservative-charge
observation; `bytes` shall be the maximum of those complete actual observations, distinct from
observed tree peak RSS. A configured ceiling, backing reservation alone, partial observation,
C-only availability RSS probe, manufactured zero or absent/default field shall not substitute
for that measurement. `ChargedPeakNotObservedReason` shall contain only `PreRoleTimeout` and
`StartupTimeoutBeforeObservation` for the allocated evidence-producing cases.

When a timeout stop occurs before L/O creation, whether the original deadline has expired
(including zero) or the finite workdeadline T minus R_eff has elapsed while the original deadline
remains live, the existing single-run `TimedOut` evidence shall carry
`NotObserved { reason: PreRoleTimeout }`. Workdeadline exhaustion shall retain its existing
cancellation/settlement and candidate-classification rules. No L/O shall be newly created or Dispatch
authorized after the applicable cutoff. AC-20 original zero/expiry timeout/capability ordering shall
remain unchanged; this stage-based reason allocates no new outcome. When startup times out after role
creation but before Dispatch and before any complete O observation, its existing single-run
`TimedOut` evidence shall carry
`NotObserved { reason: StartupTimeoutBeforeObservation }`. If startup timeout follows an actual
complete O observation, it shall instead carry `Observed` with the actual peak. Before Dispatch,
O shall establish the complete charge observation required above. Every post-Dispatch conclusion
that emits evidence shall carry `Observed`; missing actual complete measurement shall produce an
execution error with no evidence. Errors that already emit no evidence, including C capability
probe, failed ledger observation, capture, batch timeout/resource and settlement refusals, shall
not acquire invented `NotObserved` reasons. These metadata obligations shall not change existing
outcomes, stop precedence, ownership, sampling schedule, deadlines or cleanup obligations.

Authenticated Completed → original lease close → confirmed inner teardown, M termination/reap and
all writer handles closed → bounded drain to actual EOF → WRITE/GROW/SHRINK/SEAL seals → consumer
F_GET_SEALS verification → consumer OwnedFd read is mandatory. Reopened procfd writers and
descendant copies count as writers. A quiet pipe is not EOF. The final control/report handoff
survives original lease closure; it is not that lease endpoint. C shall inspect F_GET_SEALS on the
actual received descriptor and require WRITE, GROW, SHRINK and SEAL_SEAL; missing/unavailable seals
cause typed refusal and owned cleanup before read. Reads retain the 16 MiB content cap, stable
descriptor identity and original deadline. Classification follows confirmed cleanup. Reader failure
before Completed cancels O/I and releases kernel storage within bounded cleanup. Killing every
storage owner closes kernel-held pipe/memfd references with the contained tree; final close reclaims
backing without any dead actor executing Drop/unlink. Other temporary artifacts retain explicit
surviving-owner cleanup; all-owner death may leave those files, never this unnamed report, and never
waives kernel descendant cancellation.

### Settlement confirmation and kernel fault boundary

Normal returns, including ordinary startup refusal, completion and cancellation, shall positively
confirm actual owned L/O/M/I role termination/reaping, capture settlement and the existing creator
thread's required join before a conclusion. The spawning-thread retention/join rule remains
mandatory for these returns. Explicit cleanup, guard Drop and join-error handling shall settle or
report inside the execution call. No returned io::Error, refusal or other Result payload shall own
cleanup authority; dropping an error or guard after return shall not be a cleanup mechanism. CODE
shall introduce no post-return cleanup thread, daemon or hidden custodian to achieve bounded return.

The request's original whole-run bound T includes work and settlement. CODE shall use one named,
non-caller-tunable SETTLE_RESERVE R = 1 second, with the effective finite reserve allocated inside T
below. The [TC-049 settlement reserve research receipt](../matrix/TC-049-caller-death-ownership.md#settlement-reserve-research-receipt)
derives R from max(1 second, ten times the measured P99 representative kernel-topology settlement),
rounded to the least whole-second duration greater than or equal to that maximum; an integral
boundary is unchanged. This recorded research selects the SPEC allocation, not product acceptance.
Actual integrated production whole-tree settlement shall be remeasured as a mandatory CODE gate;
this scratch result cannot back AC-35 through AC-38 or substitute for their UNRUN Tests. That reserve
is not new wall-clock or memory headroom, a fixture-coordination cap or a post-expiry allowance.

Short finite ceilings remain admitted; this amendment allocates no minimum-time admission refusal
or CeilingBelowSettlementReserve cause. For a finite representable original monotonic deadline,
R_eff=min(R,T/2), with T/2 evaluated as duration division without rounding up. The workdeadline is
the original deadline minus R_eff. Here T is the original whole-run bound: a compatible batch retains
its existing member timeout and N-times-member-timeout outer-bound calculation, and reserves once
from that outer bound, never from each member separately. Checked arithmetic shall neither
underflow/wrap nor reset the deadline. An already-expired original deadline, including the current
zero-ceiling case, permits no Dispatch and retains existing timeout/capability ordering; reserve
arithmetic shall not create an admission error or a later deadline. A workdeadline already passed
while original T remains live shall immediately cancel/settle without Dispatch.

Setup, Dispatch and backend work shall not proceed beyond a finite workdeadline. If admitted work
reaches it without completion, then C shall stop work, cancel and settle, selecting the existing
TimedOut classification for the exhausted whole-run work allocation. Evidence and batch metadata
still name their original ceilings; recipe argv/environment/cwd and existing backend timeout flags
shall not be silently rewritten. R_eff is reserved once per whole run and never restarted after
failure. At small T this provides a proportionately smaller settlement window without changing the
existing ability to run short budgets or inventing a minimum-budget refusal.

The existing never-elapsing large-timeout cases remain admitted exactly: an outer-bound product
that does not fit remains Duration::MAX; original checked_add returning None means no elapsing
original deadline, not Tool/InvalidInput refusal. When the original deadline is None, there is no
workdeadline subtraction or timeout-driven stop. Member T above 4294967295 rounded whole seconds
still omits --harness-timeout, and a fitting finite original outer bound retains its normal expiry.
At the FIRST actual stop trigger for a None deadline (completion, cancellation, a resource ceiling
or observed caller death), the settlement deadline shall be that trigger's monotonic instant plus R.
That single stop-trigger-relative allowance does not extend an existing expiry, because this case
has none. It shall not restart on later triggers or per role/phase. No artificial wall-clock cutoff
or new refusal replaces this existing never-elapsing work behavior; all memory/capture/ownership
obligations remain live. A checked-add failure of the original deadline must not be confused with
an already-expired Some deadline.

For a finite deadline, settlement begins no later than workdeadline, with actual owned roles retained
through confirmation. Every settlement, Drop and join observation shall complete or report by the
same original deadline T; the None case instead uses its one FIRST-stop-trigger-plus-R deadline;
phase caps cannot accumulate additional allowances. No post-expiry grace, reset, new observation gap
or post-return cleanup extends T. If actual whole-chain/capture/creator settlement is positively
confirmed by the applicable settlement deadline, then C shall preserve its candidate classification: workdeadline expiry gives
single-run KaniRunOutcome::Inconclusive with KaniInconclusiveReason::TimedOut or the existing
BatchTimedOut refusal, as FR-028 AC-2/FR-034 AC-20 require; other candidates retain existing resource,
report and refusal precedence. No classification is published before that actual settlement.

The lifecycle guarantee has the explicit kernel fault precondition that SIGKILL delivery and
PID-namespace teardown permit this whole settlement to be confirmed by the applicable settlement
deadline. If confirmation is unavailable by that deadline, then CleanupUnconfirmed overrides every candidate, including timed-out, and
execution shall return only a typed settlement-unconfirmed Err, with no KaniExecutionEvidence,
verdict, outcome or confirmed-cleanup claim, even beside retained valid report bytes. This is an
unavailable-confirmation observation, not a positive diagnosis of D-state, a kernel bug or any
particular cause. Ordinary timeouts with confirmed settlement retain their timed-out classification;
this exceptional observation is not their normal replacement. This allocation reconciles original
AC-10/20/34: work stops with its reserved in-T settlement window, actual owned teardown precedes
conclusions and every phase still uses the original whole-run ceilings/deadline without reset.

This amendment allocates a new public variant in KaniExecutionRefusal, which merged source does not
yet provide: `Guardian { kind: GuardianFailureKind, detail: String }`. The planned public
GuardianFailureKind shall be #[non_exhaustive] and this amendment allocates exactly one current
variant, CleanupUnconfirmed. External exhaustive matches shall follow that non-exhaustive API;
this slice allocates no other kind or adoption of an unmerged WIP catalog. Meaning comes from the
typed value, never detail parsing. It adds no compatibility layer, public Result handle or authority
inside io::Error. CleanupUnconfirmed has code()==None and remains an execution error, never a
serialized evidence kind or synthetic Failed/inconclusive verdict.

After positive settlement, every other guardian failure retains its existing refusal/evidence path:

| Failure or candidate | Existing public mapping retained by this amendment |
|---|---|
| Tool executable precheck or I/O at the C/launcher boundary (not an unsuccessful backend exec exit) | Tool(KaniToolError::Io { tool: KaniTool::Launcher, path, error }); execute.rs::start retains BoundedLaunchError::Io mapping. No synthetic Kani evidence. |
| Unavailable setup/memory mechanism, namespace/proc capability, authenticated owner protection or confinement admission | MemoryMechanismUnavailable with original io::Error (listed cross-role copies use [the explicit representation/loss rule](#cross-role-refusal-cause-representation); local sources remain original) and mandatory KaniStartupAdmissionCause: existing MemoryEnforcement or the separately allocated BackendStdioSocket, BackendStdioInspectionFailed, CapabilityUnavailable. Admission Unavailable is independent of errno; existing run-stage Unsupported/NotFound unavailable mapping is retained for non-admission I/O. |
| Cause-metadata integrity fault at an independently authenticated negative admission site before Dispatch | MemoryMechanismUnavailable with that site's mandatory admission context and actual local io::Error::new(InvalidData, KaniCauseMetadataIntegrityError), raw errno None; this cause is the integrity failure, not original producer replay, and has no KaniCrossRoleCauseLoss marker. No evidence/outcome; confirmed settlement required and CleanupUnconfirmed precedence retained. |
| Authenticated startup/identity/protocol failure before Dispatch | Existing unavailable startup path when ownership/protection cannot be established; otherwise ordinary I/O Tool refusal. No additional Guardian kind or Dispatch. |
| Actual memory observation unavailable after admission | MemoryObservationFailed { detail }; no classified report/evidence. |
| Observed whole-tree memory ceiling exceeded | Single-run Inconclusive with MemoryExhausted; compatible batch BatchMemoryExhausted { members, memory_bytes, memory }. |
| Short finite or zero ceiling | Existing admitted short-budget resource paths; R_eff=min(R,T/2). Zero/already-expired retains current no-Dispatch timeout/capability ordering, no minimum-budget cause. |
| Original checked_add None or overflowing outer product | Existing admitted never-elapsing work semantics, no workdeadline stop or arithmetic refusal; FIRST actual stop starts one R settlement allowance. Existing above-backend-maximum member flag omission remains. |
| Admitted workdeadline expiry | Single-run Inconclusive with TimedOut; compatible batch BatchTimedOut { members, timeout }, naming original request ceiling. |
| Capture overflow or failed capture | OutputOverLimit { stream, limit, harnesses } or OutputUnread { stream, detail }, retaining existing stable codes and no outcome. |
| Unusable exported report, nonmember playback or crate/harness mismatch | Report(KaniReportRefusal), PlaybackForNonMember { harness } or HarnessNotInCrate { harness_path }, respectively. |
| Admitted backend unsuccessful/no-report completion, including original recipe exec failure after Dispatch, unavailable cache or denied rendezvous | After confirmed owned settlement, existing single-run and batch-member KaniRunOutcome::Inconclusive { reason: KaniInconclusiveReason::NoVerdict }, as classify_kani_run and execute.rs batch classification define. Actual unsuccessful exit/no report supports that observed result; it is not Tool Io, a synthetic Failed/evidence or a pre-Dispatch admission error. Existing memory/deadline and CleanupUnconfirmed precedence remains. |

The diagnostic detail shall contain at most 4096 UTF-8 bytes, charged to existing caller run buffers
and whole-run ceilings. This small bound permits fixed known L/O/M/I and creator/capture role labels,
already observed PID/start/namespace identity values, the last observed stage and missing
confirmation facts; it does not authorize an unbounded descendant list, host scan, after-death
identity reopen or raised resource budget. It contains data only: no pidfd, Child, JoinHandle,
namespace/control descriptor or other authority. Formatting shall enforce the byte bound at UTF-8
boundaries, mark omitted diagnostic facts explicitly and never use omission/truncation to select
the kind or claim settlement. The fixed role inventory and scalar identity fields justify this
finite diagnostic bound independently of backend output size.

An exceptional existing creator role itself stuck in the kernel may remain unjoined at error return.
The guardian shall truthfully report that observed residual and relinquishes its in-call ownership
at return; it shall not claim the role joined, retired or returned to a pool. This unavoidable
residual lies outside the kernel fault precondition, is not guaranteed cleaned up, and supplies no
new post-return custody mechanism. Ordinary missing confirmation must not be relabelled as a
positively diagnosed kernel failure. Caller/group death and all killable contained descendants
remain guaranteed under the fault precondition; no exception excuses ordinary teardown defects,
contained writer export, report/lease lifetime failure or lost ownership.

Public rustdoc for execute_kani_obligation, execute_kani_obligations and any other public bounded
entry shall document both the trusted-caller stdio stability precondition/setup window and this
kernel settlement precondition, the typed unconfirmed error, diagnostic-only residuals and absence
of a cleanup guarantee outside that kernel precondition. All new implementation/oracle checks are
PLANNED/UNRUN; existing bounded teardown defects are not thereby fixed or tests accepted.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-034-AC-1 | Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer, observer readiness and positive typed Dispatch are established, no production backend instruction is created. Caller SIGKILL, abort or OOM-equivalent forced death during initialization may start only bounded guardian bootstrap, which exits without backend on absent lease or authorization. | Test |
| FR-034-AC-2 | Before any M spawn, O is positively verified outer PID 1 with armed parent-death protection and original-exclusive bootstrap authority. Unclaimed inner I is contained by O even if M dies before internal child_wait handoff or startup-info reader loss aborts M. Owned outer pidfd termination confirms teardown; missing authority refuses before spawn. No host scan, monitor/group exit or successful signal proves cleanup. | Test |
| FR-034-AC-3 | In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants are torn down by the kernel. The test controller observes dead-caller cleanup without manufacturing that caller's outcome or evidence. | Test |
| FR-034-AC-4 | Mutual authority is the specific anonymous connected pair: guardian checks the actual mapped creator UID and only the original caller holds its executor endpoint. The bounded executor binds kernel credentials of Ready's actual sender to claimed host INIT/start/pidfd/parent/namespace, not socketpair creator credentials alone. Guardian's creator PID0 is not a host identity. A foreign actor cannot rebind a public name or supply replacement lease/Dispatch; another pair, mismatched UID or replayed authority refuses. | Test |
| FR-034-AC-5 | C retains actual unreaped Child L; L/O/M/I ownership is verified through host pidfds, start/parent/namespace identities and actual outer/inner PID 1. O owns actual M Child and I is M's inner child. Reused, stale, incomplete, translated incorrectly or dead identity refuses before Dispatch. | Test |
| FR-034-AC-6 | The owned tree memory observer is ready before typed Dispatch; failed observer preparation releases no backend instruction. | Test |
| FR-034-AC-7 | Bounded inner startup/identity recovery is required before Dispatch. M death/reparenting or malformed/missing info cancels through retained outer INIT authority and yields typed setup refusal; complete outer termination is independently confirmed. No host scan or gate EOF authorizes a backend. | Test |
| FR-034-AC-8 | After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority. | Test |
| FR-034-AC-9 | Cancelling one owned namespace leaves concurrent independent runs and unrelated host children unaffected. | Test |
| FR-034-AC-10 | Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess, capture failure and observation failure each require confirmed owned teardown before any proof conclusion. Backend exit or valid success output alone never authorizes acceptance. | Test |
| FR-034-AC-11 | Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady or immediately after Dispatch cannot release an unowned production backend: the guardian is INIT, and its death tears down that namespace. With a live original caller, guardian failure always yields a typed refusal and never verified/falsified evidence, even after confirmed teardown and beside a valid success report. | Test |
| FR-034-AC-12 | Every startup refusal, completion and cancellation settles actual owned roles before conclusions. Anonymous controls and report pipe/memfd have no persistent names and end at final close. All report-owner death reclaims kernel report backing without Drop/unlink; other assigned artifacts retain surviving-owner cleanup and truthful all-owner-death limits. No dead actor is claimed to reap or unlink. | Test |
| FR-034-AC-13 | The bounded executor requires an explicit path to this package's actual Cargo executable quire-kani-guardian, built and supplied with the library from the same package source/build inputs. A missing, non-executable or unusable helper gives a typed setup refusal before backend Dispatch, with no PATH/global discovery, copied executable, shell substitute or alternate launcher. Cargo library dependency resolution alone is not helper delivery. | Test |
| FR-034-AC-14 | Setup docs require the matched package helper roles, safe caller-context user/PID/private-mount permissions, verified mapping/setgroups/private proc, nested installed bwrap and pidfd/observer facilities. The compiled caller security profile is checked, not inferred from binary mode. Caller-profile restriction can refuse runs formerly available through bwrap-only permission; ordinary-caller availability requires actual measurement. Missing capability gives typed pre-Dispatch refusal without host-policy mutation or weaker mode. Added L/O roles and bounded controls/storage are charged to existing ceilings. | Test, Inspection |
| FR-034-AC-15 | Typed private controls reject malformed and unknown fields and enforce finite encoded-byte, pending-message and startup-work bounds. Ancillary descriptor count/type is exact, received descriptors are CLOEXEC, and unknown/extra/truncated ancillary data refuses while closing all received descriptors. Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch. | Test |
| FR-034-AC-16 | Only the original caller holds the pair's executor endpoint. Guardian receives only its control end as fd0 through safe CLOEXEC child-only mapping, borrows it with a safe descriptor API and receives actual backend stdin separately as OwnedFd through safe ancillary rights. Backend fd0 restores that original stdin or closed state, while the control/lease descriptor is excluded from backend and unrelated exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs. | Test |
| FR-034-AC-17 | Backend receives unchanged raw argument bytes except the explicitly allocated internal report locator: harness options then -Z unstable-options --export-json /proc/self/fd/N with actual mapped N >=5. Evidence records that exact argv. Inherited stdin, inherited/overridden environment and cwd remain unchanged. | Test |
| FR-034-AC-18 | Guardian diagnostics and control use separate channels from backend stdout/stderr captures and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative. | Test |
| FR-034-AC-19 | Original identity ceilings and existing outcome classifications remain authoritative after confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence rules. | Test |
| FR-034-AC-20 | Guardian connection, startup, identity verification and Dispatch use the original monotonic identity deadline without resetting it. An already expired deadline causes no Dispatch and settles inconclusive with the timed-out reason as FR-028 AC-2 states; actual identity-deadline expiry during setup or execution uses the same classification. | Test |
| FR-034-AC-21 | Guardian connection and handshake have a finite setup cap within the remaining original deadline. Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline timeout. | Test |
| FR-034-AC-22 | Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination refuses with a live caller rather than accepting a proof or claiming physical disappearance of an uninterruptible task. | Test |
| FR-034-AC-23 | Caller-death fixtures use the actual matching normal helper and shared production functions/order with bounded prefix selection as input only. Actual caller/group death is followed by independent original owned M/I pidfd termination, no backend Dispatch/marker and no leaked owned inner processes before emergency cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit; AC-31 remains independently owed. Cached prefix or gate-retained metadata proves no live gate. Feature-off positive post-Dispatch controls remain required. | Test, Analysis |
| FR-034-AC-24 | Actual lease-EOF cancellation passes independent original I termination and no backend Dispatch/marker before escalation, plus death of a positively acknowledged pinned worker after Dispatch. Ignored EOF, skipped actual lease close, removed positive Dispatch, non-INIT watcher, broken session isolation and startup-order mutants independently fail their genuine predicates; later cleanup cannot rescue them. The immediate private LeaseCloseObservation remains control flow, not a stored certificate. Stopped-I pending-authorization coordination remains owed without close/publication snapshots or ordinals; unavailable construction is UNBACKED and cleanup must not strand I. Fixture-written production frames remain forbidden. Restored controls pass. | Test |
| FR-034-AC-25 | Before backend Dispatch, the helper handshake matches the actual running CG library's build, protocol and lifecycle-capability identity against the actual invoked first-party executable. A stale helper or changed lifecycle implementation refuses, even if a caller supplies a matching version label or digest. Expected identity derives from actual library/helper build artifacts, not caller assertions or manually maintained tracking pins. | Test |
| FR-034-AC-26 | The guardian session/group is distinct from the original caller before Ready/Dispatch. Real feature-on caller-group deaths and feature-off positive post-Dispatch death controls require independent owned termination/no backend/no leaked processes. Unclaimed Bootstrap cannot infer I death or held gate from monitor death or cached phase; AC-31 remains separately mandatory. Claimed I termination, operational lease cleanup and directly killed guardian kernel teardown remain required. | Test |
| FR-034-AC-27 | guardian-test-support is off by default, absent from default features, and exposes exactly one documented fixture operation only when explicitly enabled. A feature-off consumer cannot use that operation. No public lease, process-ownership handle, cancellation entry or cleanup-deferring callback is exported. The private initialized death witness transfers exactly one actual owned monitor or validated INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup stdout is safely duplicated into a non-stdio CLOEXEC OwnedFd and itself marked CLOEXEC before any spawn; every child stdio/mapping excludes both report descriptors. | Test, Inspection |
| FR-034-AC-28 | Fixture prefix selects unchanged production transitions, never certifies live state. Actual original identity/right custody, independent pre-escalation I/worker/marker checks and immediate unchanged cleanup on success/refusal/error replace stored stage/close/publication certificates. Intentional C/group self-death follows complete bounded operational pin delivery without ACK/pause or fabricated result. Missing identity, observation or genuine coordination fails; unavailable exact early-release observation remains UNBACKED. No stored record or later cleanup repairs an oracle. | Test, Inspection |
| FR-034-AC-29 | The packaged caller fixture and real helper link the same normal library artifact through consumer-manifest package selection with matching target/profile/features/compiler flags. Separate named feature-off/on invocations avoid self dev-dependency feature unification. No cfg-test library or identity override is accepted. The bounded executor refuses feature mismatch in both directions before Dispatch; feature-off consumer compilation verifies absence of the fixture operation. | Test |
| FR-034-AC-30 | CG publishes the test-only feature contract and allocates production-driver dependency-edge exclusion to IR-649's QSL driver work. The contract requires all downstream production-build profiles to reject direct or transitively unified guardian-test-support. CG inspection verifies the published allocation and checks; downstream assertion evidence is owned by IR-649. | Inspection |
| FR-034-AC-31 | Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait caller death and natural M/info-reader failure, and pass only with confirmed actual outer INIT and unclaimed inner/escaped descendant termination. Exercise C/group, L, O and I death before/after parent-death arm with positive barriers; pre-arm has no inner child and exclusive EOF gives bounded refusal. Original-exclusive-lease race, actual host/outer identity translation, mapping/private-proc capability refusal, real observer, ordinary-caller capability measurement, spawning-thread lifetime and original-deadline assertions are mandatory CODE gates, required before CODE delivery. Existing stage-only witnesses and nine scratch cases do not cover them. | Test |
| FR-034-AC-32 | The bounded O event loop collects anonymous report pipe into memfd without a backend/collector completion wait cycle. A hard 16 MiB plus one detection-byte retention bound and the defined owned_RSS + caller_run_buffers + page-rounded F_GETPIPE_SZ + page-rounded pre-reserved memfd maximum comparison applies while writing against the original ceiling; unmapped shmem is not zero. Beyond cap yields owned cancellation and single-run KaniRunOutcome::Inconclusive with MemoryExhausted; batches keep whole-batch memory-exhausted refusal, with no member classified. Evidence names the report cap separately; FR-029 maps ResourceExhausted, never Failed or truncated acceptance. Slow/over-cap writers finish or refuse within the original deadline. Genuine installed cargo/Kani 0.68 roundtrip validates mapped FD inheritance and no seek/reread dependency; failure stops CODE pending measured spec revision, with no runtime fallback. O obtains fresh L RSS each tick from an authenticated source valid under private proc, bound to actual L pidfd/start/TGID/liveness and run authority, and receives real finite named C buffer caps from authenticated C-origin authority; setup/every-tick combination requires every input before writer/Dispatch, and missing/stale/unbound inputs refuse. The complete charge covers every named formula term and all actually observed owned roles/descendants, including L; only incidental caller-process allocations outside named terms are excluded, with named caller controls/captures still capped and charged. PreRoleTimeout covers both original-expiry and workdeadline pre-role timeout stops without changing outcomes or settlement. Every emitted evidence carries mandatory `charged_peak: ChargedPeakObservation`: `Observed { bytes }` only from the actual complete O conservative-charge peak, otherwise `NotObserved { reason }` only for `PreRoleTimeout` or `StartupTimeoutBeforeObservation` as allocated above. Startup timeout with an actual complete sample uses Observed; post-Dispatch evidence requires Observed, and absent measurement is execution error with no evidence. No zero/default/cap/C-RSS proxy or changed AC-20 classification is permitted. | Test |
| FR-034-AC-33 | Authenticated Completed precedes original-lease close, confirmed inner teardown and M termination/reap closing ALL pipe writers, including reopened procfd and descendant copies. Bounded actual-EOF drain precedes immutable WRITE/GROW/SHRINK/SEAL seals, consumer F_GET_SEALS verification and stable actual OwnedFd reads, all under the original deadline. Separate final control/report delivery remains live after lease close. Pre-Completed reader failure cancels O/I. All storage-owner death reclaims backing at final close without persistent report residue or surviving-owner dependence. Inheritance, seal race, concurrent accounting and all-owner-death gates are required before CODE delivery. | Test |
| FR-034-AC-34 | Live-C production close_lease_and_observe retains L/O/M/I ownership and final report control independently of consumed original lease. No outer kill, bootstrap EOF or parent-death cascade masks AC-24's independent pre-escalation EOF and termination checks; ignored-EOF still fails its named actual-behavior predicate before cleanup. Missing conservative backing accounting or immutable kernel report sealing gives typed refusal and owned cancellation. All roles, private namespace/proc setup, controls, report backing and observation use existing whole-run ceilings and original deadline, with no reset or observation gap. | Test |
| FR-034-AC-35 | PLANNED/UNRUN. L creates private network/root before O, O validates before M, and nested bind / / refers to confined O root. Safe backend-only seccomp/privilege installation before Dispatch preserves the same positively owned PID and original recipe, with no unfiltered Dispatch or extra surviving process/ownership/deadline; policy installation failure is pre-Dispatch, actual recipe exec/failure is post-Dispatch with existing bounded handling/no fabricated evidence; trusted installer starts in sanitized loader environment and restores original backend environment only at filtered recipe exec; I/O/M remain outside that filter. Continuous AF_UNIX socket/datagram-socketpair, legacy syscall/ABI/io_uring and inherited-endpoint exclusion prevents host-peer acquisition/export throughout writable shared source/target/cwd paths, including listeners created after Dispatch. AC authority requires the same real backend unconfined positive control to connect/export at that visible shared prefix; confined real attempts and a genuine omission mutant distinguish protection from absent listeners. Anonymous local stream socketpair IPC remains admitted. Safe-boundary feasibility and genuine installed Cargo/Kani under the actual filter are decisive UNRUN gates; incompatibility stops CODE for SPEC revision, no relaxation. Required capabilities fail through typed pre-Dispatch unavailable admission regardless errno with confirmed owned cleanup; successful admission followed by missing cache/denied rendezvous build failure retains FR-017 NoVerdict and resource precedence. All old contained-death/writer/EOF/seal/deadline/lease obligations remain mandatory. | Test, Analysis |
| FR-034-AC-36 | PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open socket input and failed fstat inspection refuse before Dispatch; actual production fd1/fd2 are capture pipes verified by mapping/inventory Analysis, not caller socket cases. Caller fd0..2 stability throughout setup is a trusted precondition; observed capture inconsistency refuses, with no claim to detect every ambient mutation. C internally captures/pins OriginalStdin before child/control fd reuse, without a public request field; authoritative initial absence/CLOEXEC yields Closed, while later Open inspection EBADF refuses and never creates Closed admission. Pipes/files/terminal/devnull input, original argv0/non-report argv/environment/cwd and captures remain unchanged; C-only AC-27 reporter is excluded from backend stdio. Admission routes BoundedLaunchError::Unavailable regardless errno to the same MemoryMechanismUnavailable with original io::Error cause and mandatory KaniStartupAdmissionCause, distinguishing BackendStdioSocket, BackendStdioInspectionFailed and CapabilityUnavailable from MemoryEnforcement. Planned public rustdoc documents the caller stability precondition/setup window and observed-only capture refusal; target type/device/inode, original O_ACCMODE and FD_CLOEXEC are compared separately from proc-link presence; mutable shared-OFD status flags neither trigger instability refusal nor get rewritten. Planned docs/Display cover bounded startup/input prerequisites, not false missing-memory diagnosis. code()==None; no execution evidence/kind, outcome or fabricated terminal/Failed. Original expiry retains its classification. | Test, Analysis |
| FR-034-AC-37 | PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch; O retains separate final delivery after original lease close. All bootstrap/ownership/report/reporter controls remain owned/CLOEXEC outside intended mappings and unavailable to arbitrary backend, sibling exec and descendants. I confirms non-dumpability after final credentials and backend cannot hold or regain CAP_SYS_PTRACE in I owning user namespace; real backend /proc/1/fd, pidfd_getfd and ptrace gates prove protection independently of host Yama. Actual leaked-control/protection mutants fail before emergency cleanup; restored protection passes. A uniform outside-host independent-authority-theft exclusion applies to all channels without excusing contained acquisition/export or dynamic shared-path peers. No early owner-channel closure or blanket sendmsg denial replaces actual final EOF, kernel report sealing and delivery, or the independent pre-escalation lease checks. | Test |
| FR-034-AC-38 | PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite effective reserve stays inside original whole T; short finite ceilings stay admitted with R_eff=min(R,T/2), no minimum-budget cause. At finite workdeadline=T-R_eff stop/cancel; normal returns positively settle/reap all owned roles, captures and existing creator thread by original T before conclusions; original None/overflow never-elapsing work remains admitted and FIRST actual stop starts one R settlement deadline. Confirmed workdeadline expiry preserves existing TimedOut classification naming T; unconfirmed settlement by the applicable deadline overrides every candidate with Err(Guardian { kind: CleanupUnconfirmed, detail }), code()==None, no evidence/verdict/outcome/cleanup claim. No phase/reset/post-expiry grace extends T or whole-run ceilings. Detail is at most 4096 UTF-8 diagnostic-only bytes, never parsed/authority. GuardianFailureKind is non-exhaustive with only CleanupUnconfirmed allocated here; every other failure retains the table's existing public mapping. No new post-return custodian or error-owned cleanup; an exceptional existing kernel-stuck unjoined creator role is truthfully reported/relinquished, never claimed joined/retired. Kernel signal/namespace teardown permitting confirmation by T is an explicit fault precondition, not diagnosed from timeout. Public bounded rustdoc explains both fault preconditions, finite R_eff/None stop-trigger settlement deadlines and classifications. Independent positive timeout/settlement and unavailable-confirmation adverse gates remain UNRUN. | Test, Analysis |
| FR-034-AC-39 | PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg Analysis; unreachable unsupported-native support is not a fabricated runtime Test. Actual installation failures are separate production Tests when available, otherwise unavailable/UNRUN with no Test completion credit. Planned mandatory admission context distinguishes BackendIpcExclusion capability support/filter installation from TrustedOwnerProtection privilege installation, retaining typed cause provenance without message parsing or fabricated errno. The policy terminates the calling process on an unsupported syscall audit architecture; every native x86_64 policy also terminates it for x32 syscall-number/aliases regardless of kernel x32 support. Native entry, exec and descendants retain these rules. Real applicable compat/x32 tests require an unconfined actual-syscall effect control and genuine omission mutant; unavailable workload/tooling/kernel controls yield zero whole-criterion completion credit, not assertion-skipping success. This establishes no immutable target-image identity or universal incompatible-image exec/entry rejection. Actual unsuccessful/no-report termination retains NoVerdict after whole-chain settlement and existing deadline/capture/resource precedence; CleanupUnconfirmed and all other report rules remain unchanged. Path/content/PATH/interpreter/loader/binfmt_misc residuals add no stability precondition, recipe change, descriptor leak or unfiltered retry. | Test, Analysis |
| FR-034-AC-40 | PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the cross-role representation. Actual kind/errno, independently authenticated optional finite provenance and loss survive privately; public projection applies only to existing io::Error-cause-bearing refusals. Post-admission observation/settlement detail-only results keep their existing shape/mapping, private finite facts/loss and diagnostic-only text, with no public marker/carrier/downcast promise or Tool reclassification; generic original custom sources/chains are lost explicitly, including valid original boxed declared operational categories but excluding required encoding/checking faults, with typed public KaniCrossRoleCauseLoss detectable by get_ref/downcast on no-errno opaque projections. Loss-free OS and payload-free causes remain unmarked; OS projection preserves public errno directly. The closed named-kind domain requires exhaustive actual sender/build source Analysis before CODE delivery; adding an emitted no-errno kind requires producer-side reviewed encoding support first. Genuine admission errors keep site-first Unavailable regardless of kind/errno; a complete bounded authenticated required-representation fault report with no prior representable original cause gives actual InvalidData/no-errno integrity in an existing I/O-cause-bearing result (no loss marker), retaining the actual sender predicate, not original replay; absent reports supply no invented received fault; unknown/malformed/incomplete cause metadata at an independently authenticated negative admission site yields Unavailable with that site's admission context and actual local InvalidData/no-errno KaniCauseMetadataIntegrityError recognized by get_ref/downcast, not original producer replay or a loss marker. Genuine original payload-free/local custom InvalidData must not be misidentified as integrity by kind or source-presence heuristics. Unauthenticated site faults retain the specified startup/protocol path; no Dispatch or evidence. Optional diagnostic failure preserves the retained original cause. Actual producer conversions determine kind/source presence without normalization or allocation-kind/layout inference. Local original sources, stronger AC-39 public-source/provenance duties, existing charges, cutoffs and settlement precedence remain. Public API/refusal rustdoc states the existing I/O-cause-carrier scope versus private-only detail results, get_ref/downcast detection of both KaniCrossRoleCauseLoss and KaniCauseMetadataIntegrityError, required sender-representation fault recognition versus valid original causes, and absence of an original-source/stage-query promise for those projected types. Standalone Analysis and genuine available transport/consumer Tests remain PLANNED/UNRUN with no prior-row credit, forced allocation failure, new refusal catalog, acknowledgment, budget or NotObserved reason. | Test, Analysis |
| FR-034-AC-41 | PLANNED/UNRUN (IR-682). For a named native-accounting conformance claim, the analysis identifies the exact consumer build configuration and compiler actually used, including resolved profile/features/target/flags/runtime/allocator. A proof for a different configuration cannot establish that claim. | Analysis |
| FR-034-AC-42 | PLANNED/UNRUN (IR-682). The proof is derived from GNU objdump or llvm-objdump disassembly using stable compiler inputs of the actual final linked executable after LTO/linking. A crate-level or pre-link emit-asm/emit-obj result cannot pass as that final-artifact proof. | Analysis |
| FR-034-AC-43 | PLANNED/UNRUN (IR-682). Every named operation and thread-entry root maps to final address ranges or conservatively covered host frames, and every reachable direct or indirect call target is accounted for. An unresolved reachable indirect target leaves the native-workspace bound UNPROVEN. | Analysis |
| FR-034-AC-44 | PLANNED/UNRUN (IR-682). A reachable recursive call cycle, including one through an external callee, leaves the native-workspace bound UNPROVEN under this nonrecursive method. | Analysis |
| FR-034-AC-45 | PLANNED/UNRUN (IR-682). The native-stack bound sums per-thread maxima for every concurrently live thread instance carrying named state, accounting for thread-entry roots, actual tail-call replacement and dynamic adjustments. Unknown instance counts or lifetimes fail the claim. An unresolved frame or adjustment cannot be filled by a guessed reserve. | Analysis |
| FR-034-AC-46 | PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy: reachable unwind/landing-pad and cleanup paths contribute to the stack bound, or the final artifact establishes their absence under abort. Assuming the strategy from a source profile default fails. | Analysis |
| FR-034-AC-47 | PLANNED/UNRUN (IR-682). Each relevant external library/runtime/libc/allocator frame has an explicit finite bound source matching the actual loaded runtime ABI/configuration. A missing source leaves the whole named-workspace claim UNPROVEN without changing the existing incidental-allocation exclusions. | Analysis |
| FR-034-AC-48 | PLANNED/UNRUN (IR-682). The named heap inventory covers actual returned Rust capacities times element size, metadata and simultaneous lifetimes, excluding incidental allocator chunk overhead. A larger actual capacity or overlapping temporary omitted from the checked caller-buffer sum fails the conformance claim. | Analysis |
| FR-034-AC-49 | PLANNED/UNRUN (IR-682). A change to the source head, resolved consumer configuration/compiler, resulting executable or loaded runtime/bound premises invalidates the former proof and requires analysis again. A stale proof or merely matching caller/helper artifact does not satisfy native-accounting conformance. | Analysis |
| FR-034-AC-50 | PLANNED/UNRUN (IR-682). Missing or incomplete proof, or a proven bound exceeding the independently declared runtime charge, leaves the configuration UNPROVEN and fails the conformance gate; a missing actual named charge retains the existing runtime refusal. This build gate introduces no runtime refusal/public capability or evidence of completed Analysis; the initial release configuration and other eligible configurations remain UNPROVEN until their own proofs exist. | Analysis |
| FR-034-AC-51 | PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's own acknowledgement establishes release-dependent birth after the actual completed ordinary O sample. | Test, Analysis |
| FR-034-AC-52 | PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly precedes matching kernel report seal; independently observed fabricated-live-I and late-poll controls fail without stored-record or ordinal proof. | Test, Analysis |
| FR-034-AC-53 | PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes matching kernel report seal; independent fabricated-reap and late-reap controls fail without cached Some, copied history or ordinal proof. | Test, Analysis |
| FR-034-AC-54 | PLANNED/UNRUN. During cancellation with the original O gate retained, actual I confirmation precedes actual gate close; independent early-close and omitted-confirmation controls fail. Cached C stage or gate-retained flag establishes neither fact; unavailable genuine operation/order construction remains UNBACKED. | Test, Analysis |
| FR-034-AC-55 | PLANNED/UNRUN. Independently checked final-source Analysis establishes contained teardown authority through actual namespace INIT death and retained owner settlement, with no sampled-PID membership kill path; the separate AC-24 non-INIT-watcher runtime adverse/restored control remains owed. | Analysis |
| FR-034-AC-56 | PLANNED/UNRUN. Published-source Analysis establishes O's final complete accounting after actual I confirmation, retained M reap, writer EOF and immutable seal for successful report Commit, with all later due ticks preserved; earlier final accounting invalidates literal-FINAL-oracle retirement. | Analysis |
| FR-034-AC-57 | PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly allocated authenticated O→L OperationalFailure carries zero ancillary rights. | Test |
| FR-034-AC-58 | PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained O Child PID with kernel sender UID/GID0 and the original build/run/source/state/stamp authority. | Test |
| FR-034-AC-59 | PLANNED/UNRUN (IR-687). For L→C AwaitArm negatives, C rejects missing, extra, wrong-type or substituted rights through owned cleanup, without a zero-right fallback. | Test |
| FR-034-AC-60 | PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination observation and existing owned-cancellation signalling, while L retains its direct Child/wait obligation. | Test, Inspection |
| FR-034-AC-61 | PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network, phase, report or Dispatch admission. | Test |
| FR-034-AC-62 | PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L wait and preserves the original earliest cutoff, including absent work deadline, without receipt-time reset. | Test |
| FR-034-AC-63 | PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only after actual O Child wait at L and required L/capture/creator/control-EOF settlement under that cutoff. | Test |
| FR-034-AC-64 | PLANNED/UNRUN (IR-687). The new negative routes reuse existing charged controls and ceilings without a new envelope type, ACK, window, cap or public category. | Inspection |
| FR-034-AC-65 | PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting pre-send tick retires an unsent genuine OperationalFailure into existing OwnerStop under existing precedence, even when the failure occurred earlier. | Test |
| FR-034-AC-66 | PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route forwards the same bytes to C with exactly one clone of L’s retained O pidfd. | Test |
| FR-034-AC-67 | PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in AwaitArm through the actual L chain and original O/run/build bindings. | Test |
| FR-034-AC-68 | PLANNED/UNRUN (IR-687). C does not use the negative pidfd for pidfd_getfd, pidfd setns, process_madvise, other-actor authority or numeric-PID reopening. | Inspection |
| FR-034-AC-69 | PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides the provisional negative with existing CleanupUnconfirmed and no execution evidence. | Test |
| FR-034-AC-70 | PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes the candidate; partial/full publication cannot be rewritten, spliced or followed by a second terminal frame. | Test |
| FR-034-AC-71 | PLANNED/UNRUN (IR-687). O includes every due observation and a genuine fresh complete named-input tick before initial terminal emission; failed current input never borrows historical/default peaks. | Test |
| FR-034-AC-72 | PLANNED/UNRUN (IR-687). After COMMIT, later successful complete exhaustion stays in actual private O accounting until genuine settlement; retention is an Analysis obligation with no current runtime seam or Test credit. | Analysis |
| FR-034-AC-73 | PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains provisional until positive actual I termination and separate real M Child reap, with required writer EOF/final due accounting before normal O return. | Test |
| FR-034-AC-74 | PLANNED/UNRUN (IR-687). The negative’s absent peak field supplies no zero measurement, no-history claim or current-validity substitute. | Test |
| FR-034-AC-75 | PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport error after COMMIT remains owned through bounded cleanup and prevents normal O Code0 acceptance through existing CleanupUnconfirmed. | Test |
| FR-034-AC-76 | PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle distinguishes L's retained-pidfd clone from independently opened same-O and foreign pidfds; inode equality supplies no clone evidence. | Test |
| FR-034-AC-77 | PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte COMMIT, O immediately initiates the same actual owned cancellation of live claimed I and retained M without waiting for C lease closure, preserving the original cutoff and committed bytes. | Test |
| FR-034-AC-78 | PLANNED/UNRUN. C admits actual original O Armed only through genuine retained O/build/run authentication and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission. | Test, Analysis |
| FR-034-AC-79 | PLANNED/UNRUN. Actual production control reception rejects malformed, partial or forbidden-right traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies a missing authenticated control. | Test |
| FR-034-AC-80 | PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes completion reception; M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate is retired. | Test |
| FR-034-AC-81 | PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control storage and pipe/backing reservation is charged before exposure; retired certificate storage is removed from charges only after its actual allocation disappears. | Test, Analysis |
| FR-034-AC-82 | PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical paired consumer/helper checks establish feature-off absence and unchanged production frames/rights after certificate retirement. | Test, Analysis |
| FR-034-AC-83 | PLANNED/UNRUN. The original exclusive caller lease closes independently of retained monitor/INIT ownership and does not leak into children; a leaked alias cannot preserve authorization after actual caller death. | Test |
| FR-034-AC-84 | PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint is excluded from untrusted child/exec mappings and public control handles through actual descriptor checks. | Test, Analysis |
| FR-034-AC-85 | PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated or cached Child Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success. | Test |
| FR-034-AC-86 | PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD provenance under the original sole-waiter/disposition/history; independent checks reject fabricated, cached or missing consuming-operation facts without stored history as proof. | Test, Analysis |
| FR-034-AC-87 | PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal and actual I confirmation precedes retained-gate close; independent late-operation/early-close mutants fail. Producer ordinal certificates are retired. | Test |
| FR-034-AC-88 | PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff transition without observation ACK, pause, stored publication permission or suppressed tick. | Test, Analysis |
| FR-034-AC-89 | PLANNED/UNRUN. Genuine original I Completed reception and original stop adoption precede actual original lease close, without a separate stored completion certificate. | Test |
| FR-034-AC-90 | PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer is retired, not activated by this condition. | Test, Analysis |
| FR-034-AC-91 | PLANNED/UNRUN. Ordinary feature-on execution remains available without retired observation binding, endpoint, event emission or default identity. | Test |
| FR-034-AC-92 | PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent child birth after an actual complete ordinary O sample remain required without a stored sample-announcement certificate; missing real coordination is UNBACKED. | Test |
| FR-034-AC-93 | PLANNED/UNRUN. Every surviving feature-on named native workspace has separate supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose certificate workspace retires only with its actual allocation. | Analysis |
| FR-034-AC-94 | PLANNED/UNRUN. C disposes the defined pending claim through actual close of its raw I right without positive admission or capability; invalid prior traffic and live identity mismatches refuse, while the defined same-cursor negative receipt preserves the original cause only after authenticated delivery and whole-chain settlement. | Test, Analysis |
| FR-034-AC-95 | PLANNED/UNRUN. Actual O retains the original report backing identity through collection, kernel sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate. | Test |
| FR-034-AC-96 | PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control authority refuses without positive admission, retaining actual received-right custody and original cutoff; no default identity supplies authority. | Test |
| FR-034-AC-97 | PLANNED/UNRUN. Retired observation-binding echo creates no replacement comparison; actual original report backing equality and delivery/settlement authority remain required. | Test |
| FR-034-AC-98 | PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase bytes/rights with no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding requirement. | Test, Analysis |
| FR-034-AC-99 | PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced stable single-thread proof, including independent across-PID-namespace initial untraced evidence, pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement supplies no signal proof or activation. | Test, Analysis |
| FR-034-AC-100 | PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles permit no optional fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or private-proc zero alone repairs eligibility. | Test |

## Dependencies

The AC-39 x86_64 Test facility uses repository-authored standalone assembly source, built by the
host's existing GNU `as` and `ld` ELF modes: `as --64` / `ld -m elf_x86_64` for native entry with
compat `int 0x80` and x32-number syscall instructions; `as --32` / `ld -m elf_i386` and `as --x32` /
`ld -m elf32_x86_64` for actual compat/x32 exec workloads where the kernel admits them. These tiny
fixtures use an explicit entry, direct syscalls and no libc or interpreter. Safe Rust test code
only invokes the tools and owned workloads; it adds no unsafe Rust, inline/global assembly, Cargo
dependency, multilib runtime, Rust target, copied executable or toolchain fallback. Missing tools,
ELF emulations or successful actual-syscall controls are unavailable/UNRUN Test evidence, never a
reason to relax production policy. Source Analysis of every native x86_64 x32 rule remains required
independently. Other audited native policies retain architecture-specific source Analysis and real
Tests only where an applicable genuine workload exists; the x86_64 fixture does not cover them.
This fixture plan establishes source feasibility only; assembly/linking and actual runtime controls
remain CODE-gated and UNRUN.

- [FR-028](./FR-028-bounded-proof-ceilings.md) AC-21 owns the every-run tree memory mechanism;
  AC-2/3 own resource outcomes, and AC-24 owns bounded native refinement. This requirement adds
  independent original-caller lifecycle ownership, not another resource model.
- [FR-017](./FR-017-kani-execution-evidence.md) owns backend reports, capture and batching.
- Downstream [IR-649](https://linear.app/agent-ix/issue/IR-649) owns QSL production-driver
  dependency-edge exclusion after IR-639 delivers this contract; it is not a CG acceptance test or
  an upstream prerequisite for the observation export.
- [IR-655](https://linear.app/agent-ix/issue/IR-655) owns the fixture SPEC gate: it may be grounded
  on actual unmerged integrated O source, but must merge before its fixture CODE and before the
  single lifecycle CODE PR merges. Final frozen CODE source must revalidate schedule/cap evidence;
  relevant source divergence requires reassessment and any amended SPEC before changed fixture
  implementation. This is a SPEC-before-fixture-CODE edge, not a separate stage-1 CODE merge gate.
- [IR-652](https://linear.app/agent-ix/issue/IR-652) owns this outer containment/unnamed storage
  specification and subsequent CODE repair.
  Stage-only fixture evidence does not complete the mandatory AC-31 through AC-34 gates.
- [TC-049](../matrix/TC-049-caller-death-ownership.md) describes planned production verification.
  The preceding containment code slice (PR #295) is merged; guardian CODE remains planned,
  independently of completion of all parent IR-241 work. Criterion-level implementation order is
  FR-017 launcher → FR-028 AC-21 containment slice → FR-034 guardian ownership → FR-028 AC-24 native
  refinement; no whole-requirement or parent-ticket cycle is introduced.

Evidence is staged explicitly in [TC-049](../matrix/TC-049-caller-death-ownership.md)'s evidence
delivery allocation: ordinary production seams without fixture extension first, IR-655's owed exact
O-origin/internal-bwrap/live-I birth and positive-operation ordering, and independent role witnesses second. This is an
internal evidence/commit-order delta in ONE lifecycle CODE PR, not separate CODE merges or a
guarantee reduction. IR-655 fixture SPEC may use real integrated O source on the unmerged first
stage, but shall merge before fixture CODE and before that single CODE PR merges. All internal
stages, genuine old-test adaptation or explicit stronger-guarantee retirement, and full gates are
required before CODE acceptance; pre-PR gates must pass before opening the CODE PR. Mixed whole
criteria remain untagged until all obligations have actual Test evidence; partial first-slice
readiness is not complete IR-639 or Kani MVP acceptance. Unavailable ordinary-seam predicates are
explicitly transferred as owed, never waived. Original compile/adaptation gaps, assertion and
mutant-retirement rules remain mandatory, alongside the unchanged independent AC-24 pre-escalation
EOF oracle.

Primary source grounding (Analysis, not production Test): Linux
[PID namespaces](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html) defines INIT-death
namespace teardown; [parent-death signals](https://man7.org/linux/man-pages/man2/PR_SET_PDEATHSIG.2const.html)
and [user namespaces](https://man7.org/linux/man-pages/man7/user_namespaces.7.html) define arming and
mapping constraints. Bubblewrap's
[launch implementation](https://github.com/containers/bubblewrap/blob/main/bubblewrap.c) places
monitor setup/info output before internal child_wait release and inner exec. Kani 0.68.0's
[JSON export](https://github.com/model-checking/kani/blob/kani-0.68.0/kani-driver/src/frontend/json_handler.rs)
uses std::fs::write of the argument-selected path; safe descriptor plumbing and the genuine pipe
roundtrip remain mandatory implementation gates. No copied source or fixed product dependency pin
is introduced by these research references.

The same bwrap source's setsid branch and conditional PID-1 fork ground the required
`--new-session` and `--as-pid-1` flags; its monitor closes extra descriptors only after setup,
so actual M settlement still precedes report EOF. Linux
[proc descriptor documentation](https://man7.org/linux/man-pages/man5/proc_pid_fd.5.html) identifies
separate pipe-inode access checks on reopen; inherited descriptor possession does not prove mapped
UID reopen permission. Safe std File ownership and the existing nix close signature supply source
prerequisites only; unsafe command-fds inherited initialization remains excluded. These sources
establish no actual helper/backend roundtrip or executable coverage.

Linux [AF_UNIX](https://man7.org/linux/man-pages/man7/unix.7.html) distinguishes pathname and
abstract sockets and SCM_RIGHTS descriptor transfer;
[network namespaces](https://man7.org/linux/man-pages/man7/network_namespaces.7.html) isolate the
abstract address scope, not inherited socket references. The private proc/PID-view allocation
requires actual absence of host aliases, not a pathname label. CG's existing typed unavailable map
is in src/kani/run/execute.rs::start and KaniExecutionRefusal::code; its terminal API is
src/kani/terminal.rs::run_terminal_value. These are source-grounded allocation facts, not executed
confinement/admission Tests or a new serialized kind.

Linux [seccomp](https://man7.org/linux/man-pages/man2/seccomp.2.html) defines inherited syscall
filters, while [non-dumpability](https://man7.org/linux/man-pages/man2/PR_SET_DUMPABLE.2const.html)
and [ptrace access checks](https://man7.org/linux/man-pages/man2/ptrace.2.html) ground the required
trusted-I protection independently of ambient Yama. These Analysis references select no helper
entry mechanism and supply no runtime evidence. Safe-boundary installation/exec feasibility,
architecture/alias/io_uring closure and the actual filtered Cargo/Kani roundtrip remain UNRUN.

The lifecycle claim includes signals directed to the original caller's process group/session and
direct guardian death. Host failure or loss of the kernel's namespace facilities cannot be turned
into a confirmed cleanup observation. Such unavailable observation with a live caller refuses; it
does not waive caller-group or guardian startup death protection.
