<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 16/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-271 | high | coverage | Inherited FR-034-AC-85 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-272 | high | coverage | Inherited FR-034-AC-86 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-273 | high | coverage | Inherited FR-034-AC-87 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-274 | high | coverage | Inherited FR-034-AC-88 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-275 | high | coverage | Inherited FR-034-AC-89 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-276 | high | coverage | Inherited FR-034-AC-90 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-277 | high | coverage | Inherited FR-034-AC-91 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-278 | high | coverage | Inherited FR-034-AC-92 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-279 | high | coverage | Inherited FR-034-AC-94 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-280 | high | coverage | Inherited FR-034-AC-95 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-281 | high | coverage | Inherited FR-034-AC-96 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-282 | high | coverage | Inherited FR-034-AC-97 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-283 | high | coverage | Inherited FR-034-AC-98 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-284 | high | coverage | Inherited FR-034-AC-99 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-285 | high | coverage | Inherited FR-034-AC-100 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-286 | high | coverage | Inherited FR-035-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-287 | high | coverage | Inherited FR-035-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-288 | high | coverage | Inherited FR-035-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-034-AC-63
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only after actual O Child
    wait at L and required L/capture/creator/control-EOF settlement under that cutoff.
- id: FR-034-AC-64
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The new negative routes reuse existing charged controls and ceilings
    without a new envelope type, ACK, window, cap or public category.
- id: FR-034-AC-65
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting pre-send tick
    retires an unsent genuine OperationalFailure into existing OwnerStop under existing precedence, even
    when the failure occurred earlier.
- id: FR-034-AC-66
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route forwards the same
    bytes to C with exactly one clone of L’s retained O pidfd.
- id: FR-034-AC-67
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in AwaitArm through the
    actual L chain and original O/run/build bindings.
- id: FR-034-AC-68
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C does not use the negative pidfd for pidfd_getfd, pidfd setns, process_madvise,
    other-actor authority or numeric-PID reopening.
- id: FR-034-AC-69
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides the provisional
    negative with existing CleanupUnconfirmed and no execution evidence.
- id: FR-034-AC-70
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes the candidate; partial/full
    publication cannot be rewritten, spliced or followed by a second terminal frame.
- id: FR-034-AC-71
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine fresh complete named-input
    tick before initial terminal emission; failed current input never borrows historical/default peaks.
- id: FR-034-AC-72
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). After COMMIT, later successful complete exhaustion stays in actual
    private O accounting until genuine settlement; retention is an Analysis obligation with no current
    runtime seam or Test credit.
- id: FR-034-AC-73
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains provisional until positive
    actual I termination and separate real M Child reap, with required writer EOF/final due accounting
    before normal O return.
- id: FR-034-AC-74
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative’s absent peak field supplies no zero measurement, no-history
    claim or current-validity substitute.
- id: FR-034-AC-75
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport error after COMMIT
    remains owned through bounded cleanup and prevents normal O Code0 acceptance through existing CleanupUnconfirmed.
- id: FR-034-AC-76
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle distinguishes
    L's retained-pidfd clone from independently opened same-O and foreign pidfds; inode equality supplies
    no clone evidence.
- id: FR-034-AC-77
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte COMMIT, O immediately
    initiates the same actual owned cancellation of live claimed I and retained M without waiting for
    C lease closure, preserving the original cutoff and committed bytes.
- id: FR-034-AC-78
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained O/build/run authentication
    and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission.
- id: FR-034-AC-79
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial or forbidden-right
    traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies
    a missing authenticated control.
- id: FR-034-AC-80
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes completion reception;
    M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate
    is retired.
- id: FR-034-AC-81
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control storage and pipe/backing
    reservation is charged before exposure; retired certificate storage is removed from charges only after
    its actual allocation disappears.
- id: FR-034-AC-82
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical paired consumer/helper
    checks establish feature-off absence and unchanged production frames/rights after certificate retirement.
- id: FR-034-AC-83
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently of retained monitor/INIT
    ownership and does not leak into children; a leaked alias cannot preserve authorization after actual
    caller death.
- id: FR-034-AC-84
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint is excluded from untrusted
    child/exec mappings and public control handles through actual descriptor checks.
- id: FR-034-AC-85
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated or cached Child
    Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success.
- id: FR-034-AC-86
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    provenance under the original sole-waiter/disposition/history; independent checks reject fabricated,
    cached or missing consuming-operation facts without stored history as proof.
- id: FR-034-AC-87
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal. On AC-54's independently
    grounded supported cooperative path, actual I confirmation precedes retained-gate close; late-operation/early-close
    mutants fail. AC-38 unavailable-confirmation abandonment at the original cutoff grants no seal, order
    or cleanup credit; a skipped poll, lost pin or owner defect is not a kernel-fault diagnosis. Producer
    ordinal certificates are retired.
- id: FR-034-AC-88
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff transition
    without observation ACK, pause, stored publication permission or suppressed tick.
- id: FR-034-AC-89
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop adoption precede actual
    original lease close, without a separate stored completion certificate.
- id: FR-034-AC-90
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate O or alter ordinary
    sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer
    is retired, not activated by this condition.
- id: FR-034-AC-91
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Ordinary feature-on execution remains available without retired observation
    binding, endpoint, event emission or default identity.
- id: FR-034-AC-92
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent child birth after
    an actual complete ordinary O sample remain required without a stored sample-announcement certificate;
    missing real coordination is UNBACKED.
- id: FR-034-AC-93
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving feature-on named native workspace has separate supported paired-artifact
    finite-bound Analysis and an independent charge; sole-purpose certificate workspace retires only with
    its actual allocation.
- id: FR-034-AC-94
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C disposes the defined pending claim through actual close of its raw I right
    without positive admission or capability; invalid prior traffic and live identity mismatches refuse,
    while the defined same-cursor negative receipt preserves the original cause only after authenticated
    delivery and whole-chain settlement.
- id: FR-034-AC-95
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through collection, kernel
    sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate.
- id: FR-034-AC-96
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control authority
    refuses without positive admission, retaining actual received-right custody and original cutoff; no
    default identity supplies authority.
- id: FR-034-AC-97
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement comparison; actual original
    report backing equality and delivery/settlement authority remain required.
- id: FR-034-AC-98
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase bytes/rights with
    no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding
    requirement.
- id: FR-034-AC-99
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    stable single-thread proof, including independent across-PID-namespace initial untraced evidence,
    pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement
    supplies no signal proof or activation.
- id: FR-034-AC-100
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles permit no optional
    fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or
    private-proc zero alone repairs eligibility.
- id: FR-035-AC-1
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut
    Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime
    provenance, charge sequence and consumed counters unchanged for all six profiles over representative
    empty, ASCII, combining, composed and supplementary payloads.
- id: FR-035-AC-2
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and
    leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes.
- id: FR-035-AC-3
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained
    sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`,
    and denial of each reached charge point yields the direct runtime stop, charges and consumed counters.
- id: FR-035-AC-4
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically
    ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each
    `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime
    call including outcome and metering, and ends with one satisfied cover after its assertions. The proof
    identity names this finite class and its type.
- id: FR-035-AC-5
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType` altered so
    the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement
    assertion with concrete playback; restoring the call verifies over the same symbolic class.
- id: FR-035-AC-6
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A request outside the declared finite proof class has a typed unsupported or requires-bound
    disposition and no harness, and no caller-ingress claim is marked `ir_confirmed` or assigned a checked-package
    node id. The six attempted text-operand `numeric.convert` nodes remain Contract IR refusals and do
    not count as passing caller-admission coverage.
- id: FR-036-AC-1
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: 'A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source identities S1/S2,
    domain D and two production functions in one module exports three obligation inventory entries and
    exactly four candidate keys for authored tuples (C1,O1,S1,D), (C2,O1,S1,D), (C1,O2,S1,D), (C1,O2,S2,D).
    O3 has no authored criterion correspondence: it remains an inventory/gap record with no candidate.
    Missing production correspondence and authored no-function remain distinct gaps on complete candidate
    keys; a zero-mapping source symbol remains an inventory/gap record. Removing only C2''s relation leaves
    C'
- id: FR-036-AC-2
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported or pre-run refusal,
    retrying and changing method/profile/subject/artifact/run leave the complete obligation inventory,
    all four criterion-obligation-source-domain keys and mapping-gap records unchanged; enumeration precedes
    producer invocation. Removing O1 and its authored relations from the authoritative source removes
    its inventory entry and exactly the two O1 candidate tuples without editing an auxiliary catalog.
- id: FR-036-AC-3
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome; pre-run
    refusal facts preserve producer/stage/cause separately from actual execution outcomes. An absent decision
    or observation and an unfamiliar producer state remain distinct, and missing, counterexample, invalid/incomplete,
    unavailable, timeout, exhaustion, cancellation and inconclusive evidence states survive export.
- id: FR-036-AC-4
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A matching executed production fact retains its actual typed artifact and run identities separately
    from attempt identity, exact source/profile/subject/domain/bounds and explicitly bound load-bearing
    check plus production reachability. Wrong, missing and mismatched artifact/run identity controls preserve
    the received typed identity or explicit missing reason for IR joining, without inventing or repairing
    identities. Zero-check, check-location-only, tag-only, LLVM-only and incidental-reachability controls
    never acquire positive observations by export.
- id: FR-036-AC-5
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed sampled native refinement
    retain distinct subject/scope/assumption observations. Missing refinement stays missing; adding a
    refinement observation does not change the census or create an unqualified-production-proof flag.
- id: FR-036-AC-6
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types over the reviewed
    neutral correspondence contract without source/transcript/name parsing or a copied schema. Duplicate
    and artifact/run/source/profile/subject/domain-mismatched result controls survive as separate inspectable
    records without changing inventories or candidate tuples; repeated identical inputs produce identical
    ordering.
findings:
- fnd: FND-271
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-85
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1928'
  excerpt: PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated or cached Child
    Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success.
  finding: 'Inherited FR-034-AC-85 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-272
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-86
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1929'
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    provenance under the original sole-waiter/disposition/history; independent checks reject fabricated,
    cached or missing consuming-operation facts without stored history as proof.
  finding: 'Inherited FR-034-AC-86 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-273
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-87
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1930'
  excerpt: PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal. On AC-54's independently
    grounded supported cooperative path, actual I confirmation precedes retained-gate close; late-operation/early-close
    mutants fail. AC-38 unavailable-confirmation abandonment at the original cutoff grants no seal, order
    or cleanup credit; a skipped poll, lost pin or owner defect is not a kernel-fault diagnosis. Producer
    ordinal certificates are retired.
  finding: 'Inherited FR-034-AC-87 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-274
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-88
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1931'
  excerpt: PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff transition
    without observation ACK, pause, stored publication permission or suppressed tick.
  finding: 'Inherited FR-034-AC-88 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-275
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-89
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1932'
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop adoption precede actual
    original lease close, without a separate stored completion certificate.
  finding: 'Inherited FR-034-AC-89 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-276
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-90
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1933'
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate O or alter ordinary
    sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer
    is retired, not activated by this condition.
  finding: 'Inherited FR-034-AC-90 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-277
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-91
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1934'
  excerpt: PLANNED/UNRUN. Ordinary feature-on execution remains available without retired observation
    binding, endpoint, event emission or default identity.
  finding: 'Inherited FR-034-AC-91 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-278
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-92
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1935'
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent child birth after
    an actual complete ordinary O sample remain required without a stored sample-announcement certificate;
    missing real coordination is UNBACKED.
  finding: 'Inherited FR-034-AC-92 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-279
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-94
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1937'
  excerpt: PLANNED/UNRUN. C disposes the defined pending claim through actual close of its raw I right
    without positive admission or capability; invalid prior traffic and live identity mismatches refuse,
    while the defined same-cursor negative receipt preserves the original cause only after authenticated
    delivery and whole-chain settlement.
  finding: 'Inherited FR-034-AC-94 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-280
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-95
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1938'
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through collection, kernel
    sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate.
  finding: 'Inherited FR-034-AC-95 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-281
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-96
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1939'
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control authority
    refuses without positive admission, retaining actual received-right custody and original cutoff; no
    default identity supplies authority.
  finding: 'Inherited FR-034-AC-96 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-282
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-97
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1940'
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement comparison; actual original
    report backing equality and delivery/settlement authority remain required.
  finding: 'Inherited FR-034-AC-97 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-283
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-98
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1941'
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase bytes/rights with
    no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding
    requirement.
  finding: 'Inherited FR-034-AC-98 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-284
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-99
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1942'
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    stable single-thread proof, including independent across-PID-namespace initial untraced evidence,
    pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement
    supplies no signal proof or activation.
  finding: 'Inherited FR-034-AC-99 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-285
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-100
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1943'
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles permit no optional
    fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or
    private-proc zero alone repairs eligibility.
  finding: 'Inherited FR-034-AC-100 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-286
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-1
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '107'
  excerpt: A generated caller-ingress oracle accepts `&TextPayload`, the declared `TextType` and `&mut
    Meter`, calls `rt::admit_text` once, and returns the runtime outcome, retained text, original runtime
    provenance, charge sequence and consumed counters unchanged for all six profiles over representative
    empty, ASCII, combining, composed and supplementary payloads.
  finding: 'Inherited FR-035-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-287
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-2
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '108'
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8` before admission and
    leave the admission meter unchanged; `admit_text` itself is never claimed to accept those bytes.
  finding: 'Inherited FR-035-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-288
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-035-AC-3
  related_ids: []
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  lines: '109'
  excerpt: For each profile, `Text[1,1]` admits or refuses according to the profile length of the retained
    sequence, including `e` plus U+0301 under NFC and NFD; a length refusal occurs before `text.result-retain`,
    and denial of each reached charge point yields the direct runtime stop, charges and consumed counters.
  finding: 'Inherited FR-035-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 16
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++