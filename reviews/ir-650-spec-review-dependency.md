---
id: SR-6324
title: IR-650 spec-review/dependency review of PR 347
type: SpecReview
analysis: dependency
scope: agent-ix/quire-contract-codegen@61823c9c801aab6fdeaaf3ed2d01d4328d2e4589; spec/kani/functional/FR-017-kani-execution-evidence.md,
  spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
review_set: subset
---

## Summary

Ticket: IR-650. The change introduces no new prerequisite edge; FR-028 remains the resource-enforcement owner referenced by FR-017 and TC-043.

## Scope

- `FR-017` (examined), `spec/kani/functional/FR-017-kani-execution-evidence.md`: - The harness identity's `ProofCeilings`: `wall_clock` bounds one harness's run and
  `memory_bytes` bounds aggregate resident memory of its backend process tree (FR-028).
  `KaniExecutionRequest` carries the harness, installation and crate/target directories;
  it has no separate timeout field. The launcher's stdout and stderr are each bounded
  to 8 MiB per harness, and a stream over that is refused, never truncated.
- `FR-017-AC-21` (examined), `spec/kani/functional/FR-017-kani-execution-evidence.md`: N harnesses with equal option vectors (the harness selection removed) and equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector holds one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer than N processes; one harness starts one process with the single-run argument vector. Requests whose wall-clock or memory ceilings differ start separate processes. The group's proc
- `FR-028` (examined), `spec/kani/functional/FR-028-bounded-proof-ceilings.md`: FR-017 now groups on equal
identity `ProofCeilings`, comprising `wall_clock` and `memory_bytes`. One grouped process tree is
held to the shared aggregate memory
ceiling; its wall-clock outer bound is N times the shared `wall_clock` ceiling.
- `TC-043` (examined), `spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md`: 4. Build N = 1, 10 and 50 harnesses with equal option vectors and identity `ProofCeilings`
   (`wall_clock` T and `memory_bytes` M), and read the launch's argument vector and the number of
   launcher processes a stand-in counts; mix two option vectors, two wall-clock ceilings and two
   memory ceilings and count the processes. Drive one grouped process tree above M.
- `FR-028-AC-21` (context_only), `spec/kani/functional/FR-028-bounded-proof-ceilings.md`: The backend process tree of every run is held to the harness identity's memory ceiling by a mechanism that observes or limits the tree's memory (the choice is the code change's, Open Questions); every execution evidence records the mechanism and, where it observes memory, the tree's peak resident memory; a tree that exceeds the ceiling is killed whole and settles as FR-028-AC-3 states, with `KaniInconclusiveReason::MemoryExhausted`, serialized `memory_exhausted`, a variant distinct from every existing reason; a FR-017 batch's launcher process tree is held to the ceiling as one group, killed wh
- `FR-017-AC-26` (context_only), `spec/kani/functional/FR-017-kani-execution-evidence.md`: A generated FR-035 caller Text-admission harness enters the same production execution path in single and compatible batch runs; its evidence carries the exact typed caller identity (profile, bounds and finite payload class) supplied with that harness and `None` for contract obligation kind. A matching successful report with a satisfied cover and successful agreement assertion classifies `Verified`; an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement assertion with its own concrete playback classifies `Falsified` with that playback. A source-mismatched harness, mi

## Verdict

**PASS** — The change introduces no new prerequisite edge; FR-028 remains the resource-enforcement owner referenced by FR-017 and TC-043.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
