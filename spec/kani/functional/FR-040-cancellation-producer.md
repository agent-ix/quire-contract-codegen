---
id: FR-040
title: "Observe caller cancellation in bounded Kani execution and retain its source"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-030
    type: references
---
# FR-040: Observe caller cancellation in bounded Kani execution and retain its source

## Description

When a caller requests cancellation of a bounded Kani execution, the generator SHALL observe the
caller-owned `quire_exact::Cancel` handle at the real launcher and retain the cause of the first
cancellation event that stops that launcher. The caller owns the handle and any timer: a timer can
call `cancel(CancelCause::Deadline)`, while an elapsed [FR-028](./FR-028-bounded-proof-ceilings.md)
execution ceiling without a recorded cancellation event remains `TimedOut`. This requirement adds
one producer path to [FR-017](./FR-017-kani-execution-evidence.md); it does not treat a Contract IR
outcome received from outside CG as an executed `KaniRunOutcome`.

## Inputs

- A required `&quire_exact::Cancel` on each public `KaniExecutionRequest`, live or already
  cancelled. The caller retains the owning handle across the synchronous call and can give clones
  to its request or timer threads. One handle belongs to one concurrent execution group; requests
  with distinct handles cannot share a launcher process. Reuse after a cancelled execution does not
  reset the handle.
- The existing harness, installation, directories, ceilings and report/capture controls. CG takes
  no new duration or source string from the caller.

## Outputs

- For a stopped single run, `KaniExecutionEvidence.outcome` is
  `KaniRunOutcome::Cancelled { source: KaniCancellationSource }`, carrying the source read by the
  launcher. The run has no backend verdict, checks, playback or exit code. The evidence retains its
  invocation, identity, ceiling and actual resource observation as [FR-017](./FR-017-kani-execution-evidence.md)
  requires; no separate, possibly inconsistent cancellation field is added.
- For a stopped group of multiple harnesses, a typed group refusal
  `KaniExecutionRefusal::BatchCancelled { source: KaniCancellationSource, members }` with no
  member evidence. Other groups in the batch retain their own results. The refusal's source is the
  source observed for that group's launcher, not a later reading of a member's handle.
- For a stopped single run, one Contract IR
  `KaniOutcomeKind::Cancelled { source: KaniCancellationSource }` constructed from that execution
  evidence through `KaniOutcome::non_success`, and its QSL terminal
  `Incomplete(IncompleteCause::Cancelled { source: quire_exact::CancelCause })` through
  [FR-030](./FR-030-ir-outcome-terminal-map.md). The IR `source` is mapped exhaustively to QSL's
  exact cause; `code`, `source_id`, `context` and elapsed time never supply it.

## Behavior

- The public single and batch execution entries SHALL pass the request's handle to the bounded
  launcher. When execution is requested, the launcher SHALL check the handle before spawning the
  backend. If the handle is already cancelled, then the launcher SHALL return a typed cancellation
  with the handle's recorded cause and start no backend. While a
  backend runs, the launcher SHALL read the handle during its bounded observation loop and stop
  owned execution when it first observes a cause before completion. The handle's atomic first-cause
  rule retains the first `Requested` or `Deadline` event; a later `cancel` call cannot replace it.
  `Cancel::tripped` is a meter-charge observation and is not required for an external process run.
- The launcher SHALL retain the cause at the stop decision, before teardown, report/capture
  settlement or a later handle read. An event submitted after completion or after a different
  stop has been selected SHALL NOT relabel the run. The existing monotonic deadline and
  work/settlement reserve remain authoritative: elapsed time alone yields the existing timed-out
  outcome or group refusal, even if the caller later records `Deadline`.
- The launcher SHALL preserve existing ownership and cleanup before returning any cancellation
  evidence. An unconfirmed cleanup or memory observation failure SHALL refuse classification;
  measured memory exhaustion SHALL retain its resource outcome or group refusal. Output over-limit
  or unread capture SHALL retain its existing refusal, because incomplete output cannot support an
  execution record. For simultaneously observed conditions, the existing memory, capture and
  elapsed-ceiling checks SHALL retain their existing priority before cancellation; completion
  accepted before a cancellation observation SHALL retain its report-derived result. A recorded
  cancellation that is observed first and actually triggers stop SHALL win over later completion
  or elapsed timeout, subject to the preceding cleanup/evidence refusals. Existing report rules
  apply to every non-cancelled completion.
- The batch entry SHALL group only requests that already meet [FR-017](./FR-017-kani-execution-evidence.md)
  grouping rules **and** share the same `Cancel` handle identity. A cancellation observed for one
  multi-member process SHALL stop the whole group, return `BatchCancelled` with the retained source
  and member count, and classify none of its members. It SHALL NOT ascribe that event to another
  group, or read each member after the fact to classify it independently. A singleton group uses
  the single-run evidence path.
- The single-run producer SHALL construct the source-bearing IR cancellation outcome only from
  its settled `KaniRunOutcome::Cancelled` evidence and pass that outcome to the FR-030 terminal map.
  Other CG-executed run outcomes retain [FR-029](./FR-029-run-outcome-terminal-record.md)'s map;
  external IR outcomes retain FR-030's existing input path. CG SHALL depend directly on
  `quire-exact` for `Cancel` and `CancelCause` at the same coherent Cargo resolution used by
  `qsl-replay`, and SHALL continue to obtain every QSL type from `qsl-replay`. No CG copy of either
  cancellation enum or QSL facade change is required.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-040-AC-1 | A caller retains a live `Cancel`, sends its clone through a public single-run request, and the bounded launcher observes it before spawn and during an actual running backend. Pre-spawn cancellation starts no backend; running cancellation confirms owned teardown before evidence returns. | Test |
| FR-040-AC-2 | In two live single runs, issue `Requested` then `Deadline`, and reverse the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }` retain the first cause; a later event cannot change them. A hand-built IR outcome alone does not satisfy this criterion. | Test |
| FR-040-AC-3 | A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline` event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that result into `Cancelled`. | Test |
| FR-040-AC-4 | Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not overwritten by later completion or elapsed time. | Test |
| FR-040-AC-5 | A multi-member group sharing one handle stops wholly on its observed cause, returns `BatchCancelled { source, members }`, and classifies no member; another group using a distinct handle retains its own result. Reversing the two cause calls changes the cancelled group's source and not the other group's evidence. Distinct handles never share one launcher. | Test |
| FR-040-AC-6 | The single-run producer constructs an IR `KaniOutcomeKind::Cancelled { source }` from settled execution evidence, and its real call path reaches the FR-030 map. Mapping both IR source variants exhaustively yields the identical `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither metadata nor a copied enum determines the source. | Test |
| FR-040-AC-7 | A direct `quire-exact` dependency resolves to the same package revision as `qsl-replay`'s exact cause, while all QSL imports remain through `qsl-replay`; no implicit cancellation control, source fallback, or compatibility overload is present. | Inspection |

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md), [FR-028](./FR-028-bounded-proof-ceilings.md), Contract IR `ix://agent-ix/quire-contract-ir/FR-030`.
- **Verification**: [TC-055](../matrix/TC-055-cancellation-producer.md) is planned; no implementation or passing test is claimed here.
