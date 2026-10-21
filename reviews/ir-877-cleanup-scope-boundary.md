---
id: SR-15241
title: IR-877 cleanup diagnostic scope-boundary review
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-contract-codegen; IR-877; spec/kani/functional/FR-017-kani-execution-evidence.md;
  spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
review_set: subset
relationships:
- type: references
  target: ix://agent-ix/quire-contract-codegen/FR-017
---

## Summary

Ticket: IR-877. Frozen head 33f3f906997de4134df3a552bea56d6cae0d294c; exact base bc4aea2bdc4635cd6fb76e5a7cf9adb9ac528301. Independent source-only review of PR497.

NamespaceOwner owns bounded observation capture (infrastructure); launch maps existing cleanup error detail to public MemoryObservationFailed (cross-cutting). Kernel readiness is an external observation, not an asserted causal guarantee. Real pidfd readiness/teardown and public refusal are checked through the existing controlled poll seam, not a replacement owner. Host AppArmor, cgroup/procfs failure-state diagnosis, natural Luna recurrence and Guardian caller-death supervision remain outside this slice. No FR034 implementation or IR877 closure is claimed.

## Verdict

PASS — No introduced defect found in this analysis scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

No applicable AssuranceProfile or required review-selection found. Rust review and native gates are inapplicable to this Markdown-only diff; no Cargo/fmt/metadata/build/test/backend commands ran. Dependency analysis skipped: no relationships edge changed. Object review skipped: no domain object changed. Criterion-strength/Jev not selected. Full-repository semantic acceptance not assessed.

### Examined units

- FR-017-AC-54; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "A successful claimed-init confirmation poll with no `IN` yields public `MemoryObservationFailed` with the existing outer and nested prefixes and labelled observations of that attempt: returned poll count, raw revents bits, monotonic poll elapsed duration with its unit, captured init PID/start ticks, captured wrapper PID or explicit absence, init-handle presence, gate presence and cleaned state. Test controls through the existing `PollOperation` seam delegate first to real poll on the real owned pidfd, record its positive count and `IN`, then clear revents and return either zero or that genuine"
- FR-017-AC-55; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "Through the real public execution entry and real owned namespace, an unchanged real-poll control retains its original public result and confirmed cleanup; each AC-54 omission refuses even when cancellation is submitted before or after the cleanup barrier. The confirmation seam sees the same claimed pidfd and exactly one poll with timeout `(5, 0)` per cleanup invocation. Failure retains gate/cleaned state and the existing `Drop` retry, and actual backend processes/namespace are gone after return. The existing real-poll-delegating errno control retains its public refusal rather than cancellation"
- FR-017-AC-56; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "Inspection of the cleanup change shows that init-pidfd KILL still precedes wrapper-group KILL, the single confirmation poll retains its `(5, 0)` timeout, signal-error/poll-error/missing-readiness precedence is unchanged, syscall errors retain their original kind and errno, and only successful cleanup closes the gate and marks cleaned. Existing `Drop` behavior is unchanged. Diagnostic observations come only from that poll and existing owned fields; no added wait, retry, skip, PID reopen, procfs/cgroup rescan or invented host-state field is introduced."
- FR-017-owned-nonconfirmation-paragraph-1; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "When the existing confirmation poll on a claimed init pidfd succeeds but its returned readiness\nlacks `IN`, the namespace owner SHALL retain the observed return count, raw readiness bits and\nmonotonic elapsed duration of that single poll in its cleanup error detail. The owner SHALL also\nretain its already captured init host PID and start ticks, captured wrapper PID or explicit absence,\ninit-handle presence, gate presence and cleaned state at that failure. Field labels SHALL distinguish\npoll duration from the duration of the whole cleanup. These are observations of the failed attempt,\nnot a dia"
- FR-017-owned-nonconfirmation-paragraph-2; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "The diagnostic SHALL propagate through the existing `namespace cleanup failed:` detail of public\n`MemoryObservationFailed`, retaining the `namespace teardown was not confirmed` prefix of the\nnested cleanup error. It introduces no verdict, receipt field or new refusal discriminant. The\nowner SHALL derive identity and state fields only from its existing owned values, without reopening\na PID, rescanning procfs/cgroups, replacing the claimed pidfd, or inventing wrapper start ticks,\nnamespace identity or failure-time host state."
- FR-017-owned-nonconfirmation-paragraph-3; examined; spec/kani/functional/FR-017-kani-execution-evidence.md: "The owner SHALL preserve the existing init-pidfd KILL then wrapper-group KILL order, the single\nconfirmation poll with timeout `(5 seconds, 0 nanoseconds)` per cleanup invocation, and the existing\nsignal-error, poll-error and missing-readiness precedence. A syscall error SHALL retain its existing\nerror kind and errno. Only successful confirmed cleanup may close the gate and mark the owner\ncleaned; failure retains the existing state and `Drop` behavior, including its cleanup retry. Public\nrefusal over competing cancellation remains as specified by\n[FR-040](./FR-040-cancellation-producer.md) AC-"
- TC-043-test-procedure-14; examined; spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md: "14. (FR-017-AC-54/55, planned under IR-877.) Drive the real public execution entry with a\n    launcher stand-in and a genuine owned namespace, using the existing confirmation\n    `PollOperation` seam. First run an unchanged control delegating to real poll. In two adverse\n    modes, delegate the first confirmation to real poll on the supplied pidfd and timeout, record\n    its positive count and `IN`, then clear revents and return respectively zero or the genuine\n    positive count. These modes are injected observation omissions. Delegate the subsequent\n    `Drop` confirmation unchanged to real "
- TC-043-expected-results-14; examined; spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md: "14. The normal control preserves its original public result and real confirmed cleanup. Both\n    controlled omissions return typed `MemoryObservationFailed`, including when cancellation\n    competes on either side of the barrier. The detail retains `namespace cleanup failed:` and\n    `namespace teardown was not confirmed` and decodable labelled fields for count, raw revents,\n    poll elapsed duration/unit, captured init PID/start ticks, wrapper PID or explicit absence,\n    init-handle presence, gate presence and cleaned state. The fields match the exact first failed\n    attempt: count zero or "
- TC-043-implementation-14; examined; spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md: "Step 14 is planned under IR-877; its new observation criteria are not implemented or qualified by\nthis specification amendment. Existing cleanup and cancellation controls remain regressions, not\nevidence that the new successful-poll/no-`IN` branch is already covered.\n"
- FR-040-AC-4; context_only; spec/kani/functional/FR-040-cancellation-producer.md: "Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not overwritten by later completion or elapsed time."

## Narrow current-base disposition round 2

Same-reviewer narrow rebase disposition for PR497. Exact current head 3f1e7668ff49abaa439998bcb98572b0d4ae1b98; base 76dea88f2d22f6fe15208552a24bfdb44fe2038c. Independent binary git diff and every one of 16 owned files are byte-identical across the ordinary rebase (two commits range-diff equal). PR494 changed family/corpus outcome lowering and tests; PR496 changed composite admission stage allocation. Neither alters namespace owner/launch/poll/cancellation source, FR017 diagnostic semantics or TC043 planned controls. The diagnostic remains observational only; no timing, retries, KILL ordering, pidfd identity, errno priority or Drop behavior changes are introduced. No Cargo/fmt/metadata/native/test/proof command ran in this review.

Eight non-gap analyses SR15233/15234/15236–15241 remain PASS; no introduced specification defect.
