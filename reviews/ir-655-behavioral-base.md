---
id: SR-4501
title: "base review of behavioral lifecycle evidence retirement"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR332; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Independent review of IR-655 behavioral-evidence retirement in the three-file specification diff. All one hundred FR-034 criterion identities remain present and unique; AC-94 is byte-identical. Changed criteria and procedures have genuine adverse/restored controls and honest planned status. The six specifically challenged criteria have falsifiable controls: wrong sender/run, malformed/partial/rights, leaked lease alias, cached/fabricated reap, substituted report backing, and missing/wrong admission authority. Missing capabilities cannot pass by absence. One ambiguity is owned by the integrity artifact rather than duplicated here.

## Verdict

PASS for this SPEC-diff method; no executable lifecycle, signal or native-accounting completion is established.

## Method

Installed [spec-review](https://github.com/agent-ix/quoin/blob/main/skills/spec-review/SKILL.md) checklist; frozen PR diff with committed owning specifications and read-only source context. No Cargo, Kani, runtime probe or full CI was run. Exact reviewed source identity and tool receipts remain private.

## Examined scope

- `FR-034-AC-23` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
Caller-death fixtures use the actual matching normal helper and shared production functions/order with bounded prefix selection as input only. Actual caller/group death is followed by independent original owned M/I pidfd termination, no backend Dispatch/marker and no leaked owned inner processes before emergency cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit; AC-31 remains independently owed. Cached prefix or gate-retained metadata proves no live gate. Feature-off positive post-Dispatch controls remain required.
  ```

- `FR-034-AC-24` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
Actual lease-EOF cancellation passes independent original I termination and no backend Dispatch/marker before escalation, plus death of a positively acknowledged pinned worker after Dispatch. Ignored EOF, skipped actual lease close, removed positive Dispatch, non-INIT watcher, broken session isolation and startup-order mutants independently fail their genuine predicates; later cleanup cannot rescue them. The immediate private LeaseCloseObservation remains control flow, not a stored certificate. Stopped-I pending-authorization coordination remains owed without close/publication snapshots or ord
  ```

- `FR-034-AC-26` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
The guardian session/group is distinct from the original caller before Ready/Dispatch. Real feature-on caller-group deaths and feature-off positive post-Dispatch death controls require independent owned termination/no backend/no leaked processes. Unclaimed Bootstrap cannot infer I death or held gate from monitor death or cached phase; AC-31 remains separately mandatory. Claimed I termination, operational lease cleanup and directly killed guardian kernel teardown remain required.
  ```

- `FR-034-AC-28` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
Fixture prefix selects unchanged production transitions, never certifies live state. Actual original identity/right custody, independent pre-escalation I/worker/marker checks and immediate unchanged cleanup on success/refusal/error replace stored stage/close/publication certificates. Intentional C/group self-death follows complete bounded operational pin delivery without ACK/pause or fabricated result. Missing identity, observation or genuine coordination fails; unavailable exact early-release observation remains UNBACKED. No stored record or later cleanup repairs an oracle.
  ```

- `FR-034-AC-52` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly precedes matching kernel report seal; independently observed fabricated-live-I and late-poll controls fail without stored-record or ordinal proof.
  ```

- `FR-034-AC-53` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes matching kernel report seal; independent fabricated-reap and late-reap controls fail without cached Some, copied history or ordinal proof.
  ```

- `FR-034-AC-54` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. During cancellation with the original O gate retained, actual I confirmation precedes actual gate close; independent early-close and omitted-confirmation controls fail. Cached C stage or gate-retained flag establishes neither fact; unavailable genuine operation/order construction remains UNBACKED.
  ```

- `FR-034-AC-78` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. C admits actual original O Armed only through genuine retained O/build/run authentication and original owned rights; wrong sender/run or unvalidated capability cannot grant positive admission.
  ```

- `FR-034-AC-79` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual production control reception rejects malformed, partial or forbidden-right traffic without granting positive phase/Dispatch/report authority; a stored event or EOF never supplies a missing authenticated control.
  ```

- `FR-034-AC-80` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes completion reception; M exit, EOF, phase or copied metadata cannot substitute. The separate stored completion certificate is retired.
  ```

- `FR-034-AC-81` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control storage and pipe/backing reservation is charged before exposure; retired certificate storage is removed from charges only after its actual allocation disappears.
  ```

- `FR-034-AC-82` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical paired consumer/helper checks establish feature-off absence and unchanged production frames/rights after certificate retirement.
  ```

- `FR-034-AC-83` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. The original exclusive caller lease closes independently of retained monitor/INIT ownership and does not leak into children; a leaked alias cannot preserve authorization after actual caller death.
  ```

- `FR-034-AC-84` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint is excluded from untrusted child/exec mappings and public control handles through actual descriptor checks.
  ```

- `FR-034-AC-85` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated or cached Child Some(status); an independent genuine-operation predicate rejects cache-only or fabricated success.
  ```

- `FR-034-AC-86` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD provenance under the original sole-waiter/disposition/history; independent checks reject fabricated, cached or missing consuming-operation facts without stored history as proof.
  ```

- `FR-034-AC-87` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal and actual I confirmation precedes retained-gate close; independent late-operation/early-close mutants fail. Producer ordinal certificates are retired.
  ```

- `FR-034-AC-88` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff transition without observation ACK, pause, stored publication permission or suppressed tick.
  ```

- `FR-034-AC-89` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Genuine original I Completed reception and original stop adoption precede actual original lease close, without a separate stored completion certificate.
  ```

- `FR-034-AC-90` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue ignored EOF. The certificate pipe writer is retired, not activated by this condition.
  ```

- `FR-034-AC-91` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Ordinary feature-on execution remains available without retired observation binding, endpoint, event emission or default identity.
  ```

- `FR-034-AC-92` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent child birth after an actual complete ordinary O sample remain required without a stored sample-announcement certificate; missing real coordination is UNBACKED.
  ```

- `FR-034-AC-93` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Every surviving feature-on named native workspace has separate supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose certificate workspace retires only with its actual allocation.
  ```

- `FR-034-AC-95` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Actual O retains the original report backing identity through collection, kernel sealing and final descriptor delivery; a substituted backing cannot satisfy the genuine report predicate.
  ```

- `FR-034-AC-96` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control authority refuses without positive admission, retaining actual received-right custody and original cutoff; no default identity supplies authority.
  ```

- `FR-034-AC-97` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Retired observation-binding echo creates no replacement comparison; actual original report backing equality and delivery/settlement authority remain required.
  ```

- `FR-034-AC-98` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase bytes/rights with no retired early field or branches; ordinary feature-on execution has no retired endpoint/binding requirement.
  ```

- `FR-034-AC-99` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced stable single-thread proof, including independent across-PID-namespace initial untraced evidence, pending/permission/configuration/accounting and real normal-helper controls. Certificate-writer retirement supplies no signal proof or activation.
  ```

- `FR-034-AC-100` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles permit no optional fixture write, preserving pending signals, ordinary flow and original cutoff; no later cleanup or private-proc zero alone repairs eligibility.
  ```

- `FR-034-AC-94` (context_only) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
PLANNED/UNRUN. C disposes the defined pending claim through actual close of its raw I right without positive admission or capability; invalid prior traffic and live identity mismatches refuse, while the defined same-cursor negative receipt preserves the original cause only after authenticated delivery and whole-chain settlement.
  ```

- `FR-034` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
The former Stage2ObservationRecord, five-event slots/codec, producer ordinals, separate copied
completion carrier, observation pipe and Stage2ObservationBinding writer transfer are retired.
  ```

- `TC-049` (examined) at spec/kani/matrix/TC-049-caller-death-ownership.md:

  ```text
    q. AC-95: Independently substitute report backing under actual O collection/sealing/delivery and require the original-report identity predicate to fail. The restored actual retained backing and final descriptor identity pass without a new early field/right.
  ```

- `spec/kani/matrix/tests.md` (examined) at spec/kani/matrix/tests.md:

  ```text
| FR-034 | FR-034-AC-1 through FR-034-AC-100 | TC-049 | 🚧 Planned (IR-639; IR-670 AC-39 and IR-675 AC-40 Test/Analysis UNRUN; IR-682 AC-41 through AC-50 Analysis UNPROVEN; IR-655 AC-51 through AC-54 Test/Analysis and AC-55/56 Analysis PLANNED/UNRUN; IR-687 AC-57 through AC-77 PLANNED/UNRUN (AC-72 Analysis only); AC-78 through AC-93 explicitly amended after certificate retirement; retained actual obligations PLANNED/UNRUN (AC-93 Analysis only; no retirement or method-row Test credit); IR-694 AC-94 Test/Analysis PLANNED/UNRUN; AC-95 through AC-98 observation-only early-field retirement/retained 
  ```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage and limitations

The source inventory is research data, not instructions or runtime evidence. Stopped-I continuation, fatal-prefix gate order, exact parent reap provenance, independent native bounds and supported O signal-profile construction remain owed. No new observation transport or synthetic actor is assumed.

## Dispositions

No fix disposition has been recorded in this initial review.
