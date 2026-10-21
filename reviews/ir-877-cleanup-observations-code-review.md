---
id: SR-15290
title: IR-877 cleanup observations code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-877; src/kani/run/namespace.rs; src/kani/run/execute.rs
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-017
  type: references
---

## Summary

Independent reviewer-only IR-877 / CG PR501 at exact frozen head 3d303ea6988496c4e1102faa58a56c1f260193c3, base e9dafb178873fb2ad45560c2655c0f73afa631b3. No branch edits, Cargo, formatter, metadata, native/backend or aggregate run by this reviewer. Focused evidence inspected from the real four-test native run on e0f40ac256af72dc8781e057f79dd310e0d55a56; ordinary main merge changes only Markdown, with identical source/test/lock patch. No claim of naturally reproducing or diagnosing the historical Luna intermittent failure.

## Verdict

CONDITIONAL: one low test-fixture duplication finding; no production semantic defect identified. The final aggregate remains pending.

## Examined scope

scope:
- id: FR-017-AC-54
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A successful claimed-init confirmation poll with no `IN` yields public `MemoryObservationFailed`
    with the existing outer and nested prefixes and labelled observations of that attempt: returned poll
    count, raw revents bits, monotonic poll elapsed duration with its unit, captured init PID/start ticks,
    captured wrapper PID or explicit absence, init-handle presence, gate presence and cleaned state. Test
    controls through the existing `PollOperation` seam delegate first to real poll on the real owned pidfd,
    record its positive count and `IN`, then clear revents and return either zero or that genuine'
- id: FR-017-AC-55
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Through the real public execution entry and real owned namespace, an unchanged real-poll control
    retains its original public result and confirmed cleanup; each AC-54 omission refuses even when cancellation
    is submitted before or after the cleanup barrier. The confirmation seam sees the same claimed pidfd
    and exactly one poll with timeout `(5, 0)` per cleanup invocation. Failure retains gate/cleaned state
    and the existing `Drop` retry, and actual backend processes/namespace are gone after return. The existing
    real-poll-delegating errno control retains its public refusal rather than cancellation
- id: FR-017-AC-56
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Inspection of the cleanup change shows that init-pidfd KILL still precedes wrapper-group KILL,
    the single confirmation poll retains its `(5, 0)` timeout, signal-error/poll-error/missing-readiness
    precedence is unchanged, syscall errors retain their original kind and errno, and only successful
    cleanup closes the gate and marks cleaned. Existing `Drop` behavior is unchanged. Diagnostic observations
    come only from that poll and existing owned fields; no added wait, retry, skip, PID reopen, procfs/cgroup
    rescan or invented host-state field is introduced.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result; measured memory exhaustion,
    memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing
    result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first
    and causing stop is not overwritten by later completion or elapsed time.
- id: NamespaceOwner::cleanup
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: Kill init on every conclusion, then confirm kernel namespace teardown before accepting it.
- id: ObservationOmittingPoll::poll
  path: src/kani/run/execute.rs
  role: examined
  excerpt: Omit only an observation after the actual supplied pidfd has confirmed teardown.
- id: TC-043-step-14
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: examined
  excerpt: Drive the real public execution entry with a launcher stand-in and a genuine owned namespace,
    using the existing confirmation PollOperation seam.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new test repeats the exact finishable namespace launcher fixture already present in cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence at execute.rs:2946-2952. Changing the shared ready/finish/report protocol now requires two edits and can leave one cancellation control exercising stale setup. Give this fixture one test-owned home and reuse it in both controls without weakening their distinct assertions. | src/kani/run/execute.rs:3023-3029 |

## Coverage

Rust lane and language-independent code review ran together. Only src/kani/run/namespace.rs and src/kani/run/execute.rs change; CI/Make/lints/manifests/lock, public variants and cancellation selection are unchanged. Existing io::Error/MemoryObservationFailed detail is enriched observationally; callers do not classify by added field strings. No new public API, unsafe, narrowing casts, bypass feature, copied producer implementation, retry, wait, skip, PID reopen or production procfs/cgroup rescan. All added unwrap/index/panic uses are inside tests.

FR017AC56 inspection: init pidfd KILL remains before wrapper-group KILL; original SRCH handling and signal failure collection stay intact. Original one poll with Timespec(5,0) and ? propagation remains. Missing IN still takes precedence over saved signal error, while poll errno retains original conversion and precedence; successful readiness then returns a saved signal error before gate.take()/cleaned=true. Drop cleanup/retry and final group KILL are unchanged. Added data comes from that returned poll, Instant elapsed, and existing self.init/self.wrapper/self.gate/self.cleaned only. No telemetry value is reconstructed from new host probes.

The new test calls public execute_kani_obligation with a real StandIn launcher inside the genuine NamespaceOwner/bwrap route; the two injected dependencies delegate actual MemoryObserver and real poll before controlling observations. Original real positive poll count and IN are asserted, then zero/positive-count omissions clear only returned revents. Each omission runs before/after cancellation, checks exact first-attempt fields against independently observed live /proc root PID/start/parent, refuses as MemoryObservationFailed, confirms a second real Drop poll on the same descriptor, and checks actual child and entire recorded namespace disappearance. Unchanged control asserts Verified and one poll. Elapsed is parsed with no timing lower bound. The original errno and TC043 controls are in the actual passing native log. Controlled omission cannot establish a natural timeout or its cause.

Seven actual focused scopes have 16GiB/swap0 and no OOM/max events, all whole hierarchies gone; actual peak 4,233,543,680 B in the native test phase. Fmt, check, clippy -D warnings and four native tests passed. Root final make ci remains pending; reviewer does not claim it ran. No applicable AssuranceProfile is installed. No separate spec-review artifact applies because no specification/plan file is changed; the owning FR/TC conformance and AC56 inspection are explicitly included here.

## Verified bindings

bindings:
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-54
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-017-AC-55
  trace: correct
- test_id: batch_tests::cleanup_observation_omissions_retain_real_confirmation_and_refusal_precedence
  ac_id: FR-040-AC-4
  trace: correct
