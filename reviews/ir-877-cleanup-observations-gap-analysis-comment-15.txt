<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 15/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-253 | high | coverage | Inherited FR-034-AC-65 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-254 | high | coverage | Inherited FR-034-AC-66 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-255 | high | coverage | Inherited FR-034-AC-67 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-256 | high | coverage | Inherited FR-034-AC-69 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-257 | high | coverage | Inherited FR-034-AC-70 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-258 | high | coverage | Inherited FR-034-AC-71 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-259 | high | coverage | Inherited FR-034-AC-73 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-260 | high | coverage | Inherited FR-034-AC-74 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-261 | high | coverage | Inherited FR-034-AC-75 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-262 | high | coverage | Inherited FR-034-AC-76 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-263 | high | coverage | Inherited FR-034-AC-77 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-264 | high | coverage | Inherited FR-034-AC-78 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-265 | high | coverage | Inherited FR-034-AC-79 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-266 | high | coverage | Inherited FR-034-AC-80 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-267 | high | coverage | Inherited FR-034-AC-81 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-268 | high | coverage | Inherited FR-034-AC-82 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-269 | high | coverage | Inherited FR-034-AC-83 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-270 | high | coverage | Inherited FR-034-AC-84 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-034-AC-13
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded executor requires an explicit path to this package's actual Cargo executable quire-kani-guardian,
    built and supplied with the library from the same package source/build inputs. A missing, non-executable
    or unusable helper gives a typed setup refusal before backend Dispatch, with no PATH/global discovery,
    copied executable, shell substitute or alternate launcher. Cargo library dependency resolution alone
    is not helper delivery.
- id: FR-034-AC-14
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Setup docs require the matched package helper roles, safe caller-context user/PID/private-mount
    permissions, verified mapping/setgroups/private proc, nested installed bwrap and pidfd/observer facilities.
    The compiled caller security profile is checked, not inferred from binary mode. Caller-profile restriction
    can refuse runs formerly available through bwrap-only permission; ordinary-caller availability requires
    actual measurement. Missing capability gives typed pre-Dispatch refusal without host-policy mutation
    or weaker mode. Added L/O roles and bounded controls/storage are charged to existing
- id: FR-034-AC-15
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Typed private controls reject malformed and unknown fields and enforce finite encoded-byte,
    pending-message and startup-work bounds. Ancillary descriptor count/type is exact, received descriptors
    are CLOEXEC, and unknown/extra/truncated ancillary data refuses while closing all received descriptors.
    Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch.
- id: FR-034-AC-16
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Only the original caller holds the pair's executor endpoint. Guardian receives only its control
    end as fd0 through safe CLOEXEC child-only mapping, borrows it with a safe descriptor API and receives
    actual backend stdin separately as OwnedFd through safe ancillary rights. Backend fd0 restores that
    original stdin or closed state, while the control/lease descriptor is excluded from backend and unrelated
    exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs.
- id: FR-034-AC-17
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Backend receives unchanged raw argument bytes except the explicitly allocated internal report
    locator: harness options then -Z unstable-options --export-json /proc/self/fd/N with actual mapped
    N >=5. Evidence records that exact argv. Inherited stdin, inherited/overridden environment and cwd
    remain unchanged.'
- id: FR-034-AC-18
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr captures
    and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative.
- id: FR-034-AC-19
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Original identity ceilings and existing outcome classifications remain authoritative after
    confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses,
    and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this
    ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence
    rules.'
- id: FR-034-AC-20
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection, startup, identity verification and Dispatch use the original monotonic
    identity deadline without resetting it. An already expired deadline causes no Dispatch and settles
    inconclusive with the timed-out reason as FR-028 AC-2 states; actual identity-deadline expiry during
    setup or execution uses the same classification.
- id: FR-034-AC-21
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining original deadline.
    Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline
    timeout.
- id: FR-034-AC-22
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination
    refuses with a live caller rather than accepting a proof or claiming physical disappearance of an
    uninterruptible task.
- id: FR-034-AC-23
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Caller-death fixtures use the actual matching normal helper and shared production functions/order
    with bounded prefix selection as input only. Actual caller/group death is followed by independent
    original owned M/I pidfd termination, no backend Dispatch/marker and no leaked owned inner processes
    before emergency cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit; AC-31
    remains independently owed. Cached prefix or gate-retained metadata proves no live gate. Feature-off
    positive post-Dispatch controls remain required.
- id: FR-034-AC-24
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Actual lease-EOF cancellation passes independent original I termination and no backend Dispatch/marker
    before escalation, plus death of a positively acknowledged pinned worker after Dispatch. Ignored EOF,
    skipped actual lease close, removed positive Dispatch, non-INIT watcher, broken session isolation
    and startup-order mutants independently fail their genuine predicates; later cleanup cannot rescue
    them. The immediate private LeaseCloseObservation remains control flow, not a stored certificate.
    Stopped-I pending-authorization coordination remains owed without close/publication snapshots or ord
- id: FR-034-AC-25
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before backend Dispatch, the helper handshake matches the actual running CG library's build,
    protocol and lifecycle-capability identity against the actual invoked first-party executable. A stale
    helper or changed lifecycle implementation refuses, even if a caller supplies a matching version label
    or digest. Expected identity derives from actual library/helper build artifacts, not caller assertions
    or manually maintained tracking pins.
- id: FR-034-AC-26
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The guardian session/group is distinct from the original caller before Ready/Dispatch. Real
    feature-on caller-group deaths and feature-off positive post-Dispatch death controls require independent
    owned termination/no backend/no leaked processes. Unclaimed Bootstrap cannot infer I death or held
    gate from monitor death or cached phase; AC-31 remains separately mandatory. Claimed I termination,
    operational lease cleanup and directly killed guardian kernel teardown remain required.
- id: FR-034-AC-27
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: guardian-test-support is off by default, absent from default features, and exposes exactly
    one documented fixture operation only when explicitly enabled. A feature-off consumer cannot use that
    operation. No public lease, process-ownership handle, cancellation entry or cleanup-deferring callback
    is exported. The private initialized death witness transfers exactly one actual owned monitor or validated
    INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup stdout is safely
    duplicated into a non-stdio CLOEXEC OwnedFd and itself marked CLOEXEC before any spawn; every chi
- id: FR-034-AC-28
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Fixture prefix selects unchanged production transitions, never certifies live state. Actual
    original identity/right custody, independent pre-escalation I/worker/marker checks and immediate unchanged
    cleanup on success/refusal/error replace stored stage/close/publication certificates. Intentional
    C/group self-death follows complete bounded operational pin delivery without ACK/pause or fabricated
    result. Missing identity, observation or genuine coordination fails; unavailable exact early-release
    observation remains UNBACKED. No stored record or later cleanup repairs an oracle.
- id: FR-034-AC-29
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The packaged caller fixture and real helper link the same normal library artifact through consumer-manifest
    package selection with matching target/profile/features/compiler flags. Separate named feature-off/on
    invocations avoid self dev-dependency feature unification. No cfg-test library or identity override
    is accepted. The bounded executor refuses feature mismatch in both directions before Dispatch; feature-off
    consumer compilation verifies absence of the fixture operation.
- id: FR-034-AC-30
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: CG publishes the test-only feature contract and allocates production-driver dependency-edge
    exclusion to IR-649's QSL driver work. The contract requires all downstream production-build profiles
    to reject direct or transitively unified guardian-test-support. CG inspection verifies the published
    allocation and checks; downstream assertion evidence is owned by IR-649.
- id: FR-034-AC-31
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait
    caller death and natural M/info-reader failure, and pass only with confirmed actual outer INIT and
    unclaimed inner/escaped descendant termination. Exercise C/group, L, O and I death before/after parent-death
    arm with positive barriers; pre-arm has no inner child and exclusive EOF gives bounded refusal. Original-exclusive-lease
    race, actual host/outer identity translation, mapping/private-proc capability refusal, real observer,
    ordinary-caller capability measurement, spawning-thread lifetime and '
- id: FR-034-AC-32
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without a backend/collector
    completion wait cycle. A hard 16 MiB plus one detection-byte retention bound and the defined owned_RSS
    + caller_run_buffers + page-rounded F_GETPIPE_SZ + page-rounded pre-reserved memfd maximum comparison
    applies while writing against the original ceiling; unmapped shmem is not zero. Beyond cap yields
    owned cancellation and single-run KaniRunOutcome::Inconclusive with MemoryExhausted; batches keep
    whole-batch memory-exhausted refusal, with no member classified. Evidence names the report cap separatel
- id: FR-034-AC-33
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner teardown and M termination/reap
    closing ALL pipe writers, including reopened procfd and descendant copies. Bounded actual-EOF drain
    precedes immutable WRITE/GROW/SHRINK/SEAL seals, consumer F_GET_SEALS verification and stable actual
    OwnedFd reads, all under the original deadline. Separate final control/report delivery remains live
    after lease close. Pre-Completed reader failure cancels O/I. All storage-owner death reclaims backing
    at final close without persistent report residue or surviving-owner dependence. Inheritance, s
- id: FR-034-AC-34
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Live-C production close_lease_and_observe retains L/O/M/I ownership and final report control
    independently of consumed original lease. No outer kill, bootstrap EOF or parent-death cascade masks
    AC-24's independent pre-escalation EOF and termination checks; ignored-EOF still fails its named actual-behavior
    predicate before cleanup. Missing conservative backing accounting or immutable kernel report sealing
    gives typed refusal and owned cancellation. All roles, private namespace/proc setup, controls, report
    backing and observation use existing whole-run ceilings and original deadline, with no res
- id: FR-034-AC-35
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before M, and nested bind
    / / refers to confined O root. Safe backend-only seccomp/privilege installation before Dispatch preserves
    the same positively owned PID and original recipe, with no unfiltered Dispatch or extra surviving
    process/ownership/deadline; policy installation failure is pre-Dispatch, actual recipe exec/failure
    is post-Dispatch with existing bounded handling/no fabricated evidence; trusted installer starts in
    sanitized loader environment and restores original backend environment only at filtered recipe exec;
    I
- id: FR-034-AC-36
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open socket input
    and failed fstat inspection refuse before Dispatch; actual production fd1/fd2 are capture pipes verified
    by mapping/inventory Analysis, not caller socket cases. Caller fd0..2 stability throughout setup is
    a trusted precondition; observed capture inconsistency refuses, with no claim to detect every ambient
    mutation. C internally captures/pins OriginalStdin before child/control fd reuse, without a public
    request field; authoritative initial absence/CLOEXEC yields Closed, while later Open inspection EBA
- id: FR-034-AC-37
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch; O retains separate final
    delivery after original lease close. All bootstrap/ownership/report/reporter controls remain owned/CLOEXEC
    outside intended mappings and unavailable to arbitrary backend, sibling exec and descendants. I confirms
    non-dumpability after final credentials and backend cannot hold or regain CAP_SYS_PTRACE in I owning
    user namespace; real backend /proc/1/fd, pidfd_getfd and ptrace gates prove protection independently
    of host Yama. Actual leaked-control/protection mutants fail before emergency cleanup; rest
- id: FR-034-AC-38
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite effective reserve
    stays inside original whole T; short finite ceilings stay admitted with R_eff=min(R,T/2), no minimum-budget
    cause. At finite workdeadline=T-R_eff stop/cancel; normal returns positively settle/reap all owned
    roles, captures and existing creator thread by original T before conclusions; original None/overflow
    never-elapsing work remains admitted and FIRST actual stop starts one R settlement deadline. Confirmed
    workdeadline expiry preserves existing TimedOut classification naming T; unconfirmed settlement
- id: FR-034-AC-39
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg Analysis; unreachable
    unsupported-native support is not a fabricated runtime Test. Actual installation failures are separate
    production Tests when available, otherwise unavailable/UNRUN with no Test completion credit. Planned
    mandatory admission context distinguishes BackendIpcExclusion capability support/filter installation
    from TrustedOwnerProtection privilege installation, retaining typed cause provenance without message
    parsing or fabricated errno. The policy terminates the calling process on an unsupported sysca
- id: FR-034-AC-40
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the cross-role representation.
    Actual kind/errno, independently authenticated optional finite provenance and loss survive privately;
    public projection applies only to existing io::Error-cause-bearing refusals. Post-admission observation/settlement
    detail-only results keep their existing shape/mapping, private finite facts/loss and diagnostic-only
    text, with no public marker/carrier/downcast promise or Tool reclassification; generic original custom
    sources/chains are lost explicitly, including valid original boxed declared ope
- id: FR-034-AC-41
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). For a named native-accounting conformance claim, the analysis identifies
    the exact consumer build configuration and compiler actually used, including resolved profile/features/target/flags/runtime/allocator.
    A proof for a different configuration cannot establish that claim.
- id: FR-034-AC-42
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The proof is derived from GNU objdump or llvm-objdump disassembly using
    stable compiler inputs of the actual final linked executable after LTO/linking. A crate-level or pre-link
    emit-asm/emit-obj result cannot pass as that final-artifact proof.
- id: FR-034-AC-43
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Every named operation and thread-entry root maps to final address ranges
    or conservatively covered host frames, and every reachable direct or indirect call target is accounted
    for. An unresolved reachable indirect target leaves the native-workspace bound UNPROVEN.
- id: FR-034-AC-44
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A reachable recursive call cycle, including one through an external
    callee, leaves the native-workspace bound UNPROVEN under this nonrecursive method.
- id: FR-034-AC-45
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The native-stack bound sums per-thread maxima for every concurrently
    live thread instance carrying named state, accounting for thread-entry roots, actual tail-call replacement
    and dynamic adjustments. Unknown instance counts or lifetimes fail the claim. An unresolved frame
    or adjustment cannot be filled by a guessed reserve.
- id: FR-034-AC-46
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy: reachable unwind/landing-pad
    and cleanup paths contribute to the stack bound, or the final artifact establishes their absence under
    abort. Assuming the strategy from a source profile default fails.'
- id: FR-034-AC-47
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Each relevant external library/runtime/libc/allocator frame has an
    explicit finite bound source matching the actual loaded runtime ABI/configuration. A missing source
    leaves the whole named-workspace claim UNPROVEN without changing the existing incidental-allocation
    exclusions.
- id: FR-034-AC-48
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The named heap inventory covers actual returned Rust capacities times
    element size, metadata and simultaneous lifetimes, excluding incidental allocator chunk overhead.
    A larger actual capacity or overlapping temporary omitted from the checked caller-buffer sum fails
    the conformance claim.
- id: FR-034-AC-49
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A change to the source head, resolved consumer configuration/compiler,
    resulting executable or loaded runtime/bound premises invalidates the former proof and requires analysis
    again. A stale proof or merely matching caller/helper artifact does not satisfy native-accounting
    conformance.
- id: FR-034-AC-50
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Missing or incomplete proof, or a proven bound exceeding the independently
    declared runtime charge, leaves the configuration UNPROVEN and fails the conformance gate; a missing
    actual named charge retains the existing runtime refusal. This build gate introduces no runtime refusal/public
    capability or evidence of completed Analysis; the initial release configuration and other eligible
    configurations remain UNPROVEN until their own proofs exist.
- id: FR-034-AC-51
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's own acknowledgement
    establishes release-dependent birth after the actual completed ordinary O sample.
- id: FR-034-AC-52
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly precedes matching
    kernel report seal; independently observed fabricated-live-I and late-poll controls fail without stored-record
    or ordinal proof.
- id: FR-034-AC-53
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes matching kernel
    report seal; independent fabricated-reap and late-reap controls fail without cached Some, copied history
    or ordinal proof.
- id: FR-034-AC-54
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation path, the actual
    original O gate owner retains the gate through actual I confirmation before actual close; early-close
    and omitted-confirmation controls fail without marker, stage or refusal-label proof. Genuine unavailable
    confirmation at the original cutoff follows AC-38 CleanupUnconfirmed with explicit actual-owner abandonment/right
    disposal and no ordering or cleanup credit, never a renamed fatal close or diagnosis of kernel fault.
    Healthy restored order and unavailable-confirmation refusal are separate control
- id: FR-034-AC-55
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Independently checked final-source Analysis establishes contained teardown authority
    through actual namespace INIT death and retained owner settlement, with no sampled-PID membership
    kill path; the separate AC-24 non-INIT-watcher runtime adverse/restored control remains owed.
- id: FR-034-AC-56
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Published-source Analysis establishes O's final complete accounting after actual
    I confirmation, retained M reap, writer EOF and immutable seal for successful report Commit, with
    all later due ticks preserved; earlier final accounting invalidates literal-FINAL-oracle retirement.
- id: FR-034-AC-57
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly allocated authenticated
    O→L OperationalFailure carries zero ancillary rights.
- id: FR-034-AC-58
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained O Child PID with
    kernel sender UID/GID0 and the original build/run/source/state/stamp authority.
- id: FR-034-AC-59
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). For L→C AwaitArm negatives, C rejects missing, extra, wrong-type or
    substituted rights through owned cleanup, without a zero-right fallback.
- id: FR-034-AC-60
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination observation
    and existing owned-cancellation signalling, while L retains its direct Child/wait obligation.
- id: FR-034-AC-61
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network, phase,
    report or Dispatch admission.
- id: FR-034-AC-62
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L wait and preserves
    the original earliest cutoff, including absent work deadline, without receipt-time reset.
findings:
- fnd: FND-253
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-65
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1908'
  excerpt: PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting pre-send tick
    retires an unsent genuine OperationalFailure into existing OwnerStop under existing precedence, even
    when the failure occurred earlier.
  finding: 'Inherited FR-034-AC-65 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-254
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-66
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1909'
  excerpt: PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route forwards the same
    bytes to C with exactly one clone of L’s retained O pidfd.
  finding: 'Inherited FR-034-AC-66 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-255
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-67
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1910'
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in AwaitArm through the
    actual L chain and original O/run/build bindings.
  finding: 'Inherited FR-034-AC-67 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-256
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-69
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1912'
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides the provisional
    negative with existing CleanupUnconfirmed and no execution evidence.
  finding: 'Inherited FR-034-AC-69 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-257
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-70
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1913'
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes the candidate; partial/full
    publication cannot be rewritten, spliced or followed by a second terminal frame.
  finding: 'Inherited FR-034-AC-70 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-258
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-71
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1914'
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine fresh complete named-input
    tick before initial terminal emission; failed current input never borrows historical/default peaks.
  finding: 'Inherited FR-034-AC-71 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-259
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-73
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1916'
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains provisional until positive
    actual I termination and separate real M Child reap, with required writer EOF/final due accounting
    before normal O return.
  finding: 'Inherited FR-034-AC-73 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-260
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-74
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1917'
  excerpt: PLANNED/UNRUN (IR-687). The negative’s absent peak field supplies no zero measurement, no-history
    claim or current-validity substitute.
  finding: 'Inherited FR-034-AC-74 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-261
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-75
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1918'
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport error after COMMIT
    remains owned through bounded cleanup and prevents normal O Code0 acceptance through existing CleanupUnconfirmed.
  finding: 'Inherited FR-034-AC-75 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-262
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-76
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1919'
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle distinguishes
    L's retained-pidfd clone from independently opened same-O and foreign pidfds; inode equality supplies
    no clone evidence.
  finding: 'Inherited FR-034-AC-76 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-263
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-77
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1920'
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte COMMIT, O immediately
    initiates the same actual owned cancellation of live claimed I and retained M without waiting for
    C lease closure, preserving the original cutoff and committed bytes.
  finding: 'Inherited FR-034-AC-77 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-264
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-78
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1921'
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained O/build/run authentication
    and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission.
  finding: 'Inherited FR-034-AC-78 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-265
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-79
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1922'
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial or forbidden-right
    traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies
    a missing authenticated control.
  finding: 'Inherited FR-034-AC-79 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-266
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-80
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1923'
  excerpt: PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes completion reception;
    M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate
    is retired.
  finding: 'Inherited FR-034-AC-80 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-267
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-81
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1924'
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control storage and pipe/backing
    reservation is charged before exposure; retired certificate storage is removed from charges only after
    its actual allocation disappears.
  finding: 'Inherited FR-034-AC-81 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-268
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-82
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1925'
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical paired consumer/helper
    checks establish feature-off absence and unchanged production frames/rights after certificate retirement.
  finding: 'Inherited FR-034-AC-82 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-269
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-83
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1926'
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently of retained monitor/INIT
    ownership and does not leak into children; a leaked alias cannot preserve authorization after actual
    caller death.
  finding: 'Inherited FR-034-AC-83 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-270
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-84
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1927'
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint is excluded from untrusted
    child/exec mappings and public control handles through actual descriptor checks.
  finding: 'Inherited FR-034-AC-84 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 15
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++