---
id: SR-4304
title: "failure-domain review of observed O SIGPIPE eligibility"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen PR331; spec/kani/functional/FR-034-caller-death-ownership.md; spec/kani/matrix/TC-049-caller-death-ownership.md; spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-639. Independent review of the frozen Markdown-only observed-profile amendment. One genuine namespace visibility edge requires clarification before CODE. Snapshot stability cannot repair an initially hidden tracer.

## Verdict

**CONDITIONAL** — Clarify the ancestor-tracer false negative and add its planned adverse control.

## Scope

- `FR-034-AC-90` (context_only) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  PLANNED/UNRUN. O latches observation write failure without signal termination or a new cancellation/control action.
  ```

- `FR-034-AC-93` (context_only) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  PLANNED/UNRUN. The CODE verifier establishes the separately declared feature-on observation native-workspace bound from its own exact paired artifacts and premises.
  ```

- `FR-034-AC-98` (context_only) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  PLANNED/UNRUN. Feature-off consumers and helpers contain no early-report field, storage/decoder branch or bytes, and preserve the original phase frame and sole right; feature-on unbound execution creates no observation endpoint or emission.
  ```

- `FR-034-AC-99` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  PLANNED/UNRUN. O permits observation writes only with actual ignored/unblocked/untraced single-thread observations and complete supported configuration, stability, pending, permission and accounting proof through each write.
  ```

- `FR-034-AC-100` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  PLANNED/UNRUN. O latches private observation failure without writing for every unsupported, malformed, unavailable or unproved profile, preserving ordinary sampling/control/settlement and original cutoff.
  ```

- `FR-034` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  O shall latch full/EPIPE/other observation-write failure, disable further observation writes and
  continue the same ordinary production sampling/control/settlement path. Observation failure shall
  never initiate O cancellation, I signalling, early exit or outer escalation. The CODE author shall
  establish the conditional observed SIGPIPE profile below before any observation pipe write,
  including unchanged pending-signal behavior. Inherited disposition, a single observed snapshot or
  socket MSG_NOSIGNAL alone shall not supply that proof. O shall not let SIGPIPE terminate it or rescue
  an ignored-EOF
  ```

- `FR-034` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  For this feature-only observation writer, O shall write only when its actual original runtime
  observes SIGPIPE ignored AND unblocked, no tracing and one thread, and the supported complete
  consumer/helper configuration proves that disposition, mask, tracing and single-thread state
  remain unchanged through each write. O shall obtain these observations from its authenticated
  private proc view of the same actual O, using required SigIgn, SigBlk, SigPnd, ShdPnd,
  TracerPid, task/thread and identity facts with complete checked parsing. Absent, duplicated,
  malformed, truncated, identity-inconsistent o
  ```

- `FR-034` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  For default, caught, blocked, traced, multithreaded, changing or otherwise unproved profiles, or any
  required query/parse/permission failure, O shall latch private observation failure and perform NO
  observation write. It shall disable further observation writes and continue the same ordinary
  production sampling/control/settlement under the original cutoff. C shall seal missing/failed
  observation as typed fixture failure before unchanged cleanup. Profile refusal shall not become
  production cancellation, I signalling, early O exit, new public failure classification or successful
  ignored-inner-EO
  ```

- `FR-034` (examined) at spec/kani/functional/FR-034-caller-death-ownership.md:

  ```text
  All query/parser records, actual capacities, retained/transient/native syscall storage and overlapping
  lifetimes shall enter the existing feature-on accounting/native proof before exposure. Named C
  storage remains caller_run_buffers and O storage remains owned_RSS under the existing formula;
  AC-93 independently declared finite native charge also covers profile query/parser/storage overlap.
  An existing proc parser or logical length bound shall not prove this new owner's actual retained/
  transient capacity or native charge. No new term, numerical cap or reserve is allocated.
  Unsafe or unavailabl
  ```

- `TC-049` (examined) at spec/kani/matrix/TC-049-caller-death-ownership.md:

  ```text
  These independent checks are PLANNED/UNRUN through the genuine fixture CODE gate. Use the matched
  normal helper and actual original O, original run and observation endpoint. No inherited probe
  profile, manufactured process or simulated successful profile observation supplies the positive
  control. Establish concrete safe access and supported full-build/kernel/runtime premises first.
  ```

- `TC-049` (examined) at spec/kani/matrix/TC-049-caller-death-ownership.md:

  ```text
  a. AC-99: Read actual same-O required SigIgn/SigBlk/SigPnd/ShdPnd/TracerPid/thread/identity facts from its authenticated
     private proc view and prove ignored, unblocked, untraced single-thread stability through each
     selected live-reader and broken-reader write. Supply supported-kernel pending semantics,
     actual syscall permissions/configuration, preservation of preexisting/foreign-origin pending
     signals and complete simultaneous accounting/native storage under AC-93's independently declared
     finite charge. An existing proc parser buffer/length bound supplies no actual owner capacit
  ```

- `TC-049` (examined) at spec/kani/matrix/tests.md:

  ```text
  | FR-034 | FR-034-AC-1 through FR-034-AC-100 | TC-049 | 🚧 Planned (IR-639; IR-670 AC-39 and IR-675 AC-40 Test/Analysis UNRUN; IR-682 AC-41 through AC-50 Analysis UNPROVEN; IR-655 AC-51 through AC-54 Test/Analysis and AC-55/56 Analysis PLANNED/UNRUN; IR-687 AC-57 through AC-77 PLANNED/UNRUN (AC-72 Analysis only); IR-689 AC-78 through AC-93 PLANNED/UNRUN (AC-93 Analysis only; all Tests independently owed); IR-694 AC-94 Test/Analysis PLANNED/UNRUN; IR-702 AC-95 through AC-98 Test/Analysis PLANNED/UNRUN; conditional observed SIGPIPE AC-99/100 PLANNED/UNRUN); guardian CODE remains planned on merged
  ```

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Private-proc TracerPid zero can hide an ancestor-namespace tracer. Require independent proof of initial untraced state, not only stability of that value, and an adverse control that refuses or leaves eligibility UNPROVEN when a real ancestor tracer is invisible in O procfs. | spec/kani/functional/FR-034-caller-death-ownership.md:1680 |

## Primary Evidence

Linux proc `task_state` computes the tracer identifier in the proc mount PID namespace; `pid_nr_ns` returns zero when that namespace has no representation. This permits a real ancestor tracer to be invisible to O's private view. Linux signal handling exempts traced tasks from ordinary ignored-signal discard, so the distinction matters for pending preservation. These are source facts, not an executed exploit or proof of the actual supported kernel. No new query, authority or tracing policy is prescribed; unsupported proof must remain UNPROVEN/no-write.

Sources: [proc task state](https://github.com/torvalds/linux/blob/master/fs/proc/array.c), [PID namespace projection](https://github.com/torvalds/linux/blob/master/kernel/pid.c), [ignored-signal handling](https://github.com/torvalds/linux/blob/master/kernel/signal.c).

## Limitations

No Cargo, build, Kani, full CI or probe ran. Observed profile, pending preservation, configuration stability, normal-helper controls and actual capacities remain CODE gates. No applicable AssuranceProfile or new dependency relationship was found in the changed scope.

## Dispositions

| FND | outcome | sha/reason |
|---|---|---|
| FND-001 | fixed | The owning requirement now demands independently checkable initial untraced evidence across PID namespaces; private zero/stability/declarative assumptions do not establish it. TC-049 requires the genuine hidden-ancestor tracer refusal control. |

## Disposition pass 1

**PASS for SPEC merge readiness.** The sole finding is fixed, with no new regression in the narrow clarification. Missing actual proof or safe adverse construction leaves the capability UNPROVEN/no-write and runtime evidence UNRUN. This disposition does not establish an implemented query, a supported effective profile or successful observation writing.

### Examined fixed scope

- FR-034 (examined):

```text
The CODE author shall additionally prove the INITIAL
actual O is untraced using independently checkable initial-state evidence valid across PID
namespaces, including any ancestor tracer invisible in that view. Private-proc TracerPid zero,
even if stable, or a declarative no-tracer assumption shall not establish initial untraced state.
```

- TC-049 (examined):

```text
Include a real ancestor-PID-namespace tracer attached to the actual O but invisible in its
   private proc TracerPid. Require refusal or eligibility UNPROVEN/no-write; a private-zero-only
   eligibility mutant must fail. Missing genuine safe control construction remains UNRUN and
   unsupported for writing, never an assumed untraced profile or new query/tracing authority.
```
