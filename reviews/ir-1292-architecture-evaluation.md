---
id: SR-2026110283821
title: 'IR-1292 architecture evaluation of CG AD-002 through AD-007'
type: SpecReview
analysis: architecture-evaluation
scope: 'agent-ix/quire-contract-codegen; spec/assurance/AD-002-cg-qsl-replay-seam.md through AD-007-v2-symbolic-width-tractability.md; CLAUDE.md deny command'
review_set: subset
---

## Summary

Ticket: IR-1292. Read-only architecture review of `origin/main` at `9b5256fc090c8b5d7468f6b8a128de6275384f41` and the proposed one-line `CLAUDE.md` correction. Examined AD-002, AD-003, AD-004, AD-005, AD-006 and AD-007 against the current CG dependency list, replay and terminal seams, native generation entry point, and explicit proof limits. The decision is to accept AD-005 and AD-006 as proposed directions with their documented upstream and proof qualifications; accept AD-002, AD-003 and AD-004 after narrow current-state corrections below; keep AD-007 an unapproved, open decision. None of this review grants proof credit.

For a successful function counterexample, QSL selects the site and issues the structural obligation identity, CG checks the retained claim and reconstructs a QSL `WitnessEnvelope`, and QSL replays the request. For failure and recovery, a packet with mismatched target, occurrence, subject, limits, domains, backend, source, or obligation is refused before replay (`src/replay/function.rs:879-931`); frame replay still declares empty domains (`src/replay/frame.rs:849`), an open gap that AD-002 explicitly records. For a producer change, AD-005 keeps source inventories and authored correspondence upstream, CG facts in CG, and the IR accounting policy behind an adapter above both crates; its missing IR-759 input and compile conformance remain gates. For the native proof change, AD-006 keeps admission and arithmetic in the kernel and the public real Meter, while AD-007 requires a versioned encoding record and clean 16 GiB proof and mutation evidence before any representation change earns credit.

The `CLAUDE.md` correction accurately describes `make deny`: `origin/main:Makefile:146-147` invokes only `$(CARGO) deny check`; `origin/main:scripts/check_one_copy.awk` does not exist. This conclusion is about the command in this exact main revision.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AD-002 assigns CG ownership of a SHA-256 obligation digest made from node, occurrence, kind and arguments. Current code receives a QSL producer-issued `ObligationIdentity` and returns it unchanged after separate argument checks. AD-003 E-1 already describes the structural node/model/ordinal identity. In a replay join, the AD-002 account would send a future implementer to mint a different identity and merge semantic claim data into its key. Replace the current-state identity row and preimage explanation with producer-issued structural identity; retain separate kind, occurrence and domain comparisons. | spec/assurance/AD-002-cg-qsl-replay-seam.md:57,77-82,151-167,170,221; spec/assurance/AD-003-evidence-chain.md:168-170; src/replay/obligation.rs:1-4,168-175,186-195; src/replay/function.rs:986-997 |
| FND-002 | high | AD-002 says the function replay path builds no `WitnessEnvelope` and calls this an open FR-024 gap. Current `request_from_packet` reconstructs a QSL envelope, compares the selected claim and passes it to `request_from_envelope`; `backend_witness_packet` is the public packet construction path. A recovery design based on the AD would duplicate or bypass the existing admission and claim checks. Update the function path current state and remove the closed missing-envelope gap while retaining any independently open FR-024 criteria. | spec/assurance/AD-002-cg-qsl-replay-seam.md:151-155,184-193; src/replay/function.rs:864-876,879-931,934-948 |
| FND-003 | medium | AD-003's Current state and gaps says there is no terminal map and no direct model dependency. CG now exports `run_terminal_value` and `ir_outcome_terminal_value`, and `Cargo.toml` declares `quire-contract-model` directly. A terminal failure or import-path change planned from these statements would target work already present and could misassign the owner. Refresh those current-state bullets and the open-question rows that repeat them, or explicitly label the measured base historical, then preserve only genuinely open terminal and ownership questions. | spec/assurance/AD-003-evidence-chain.md:399-421,428,463; src/kani/terminal.rs:108-115,192-197; Cargo.toml:33-42 |
| FND-004 | medium | AD-004 still presents an obligation preimage encoded through `core::canonical` as a current architecture decision in its dependency and Decisions sections. The current replay obligation function clones QSL's producer identity. This contradicts the corrected structural-identity direction and AD-003 E-1 during a future identity or canonical-encoding change. Mark those obligation-preimage portions historical or update them to the present separation between CG content digests and QSL obligation identity. | spec/assurance/AD-004-cg-crate-layout.md:509,875,886,1780; spec/assurance/AD-003-evidence-chain.md:168-170; src/replay/obligation.rs:168-175 |

## Verdict

AD-002: accept with FND-001 and FND-002 corrections. Its QSL-to-CG site selection and CG-to-QSL replay request direction is consistent with `Cargo.toml` and `src/replay/function.rs`; the frame empty-domain gap remains explicitly open.

AD-003: accept with FND-003 correction. Its E-1 structural identity and separation of producer, observation, and terminal value are coherent with the current replay and terminal code; the lower historical/current status bullets need reconciliation.

AD-004: accept with FND-004 correction. It explicitly labels its original inventory and migration tables historical, and the current `src/` has the described core, oracle, strategy, evidence, kani, replay, routed and publication partitions. The stale obligation-preimage decision should not remain current.

AD-005: accept as a planned boundary, not an implemented feature. It explicitly names the absent IR-759 authored correspondence, retains source and production owners, and places the consuming adapter above CG and IR to avoid a reverse IR-to-CG dependency (`AD-005:24-29,56-66,118-129,146-159`; `Cargo.toml:33-42`).

AD-006: accept its kernel-owned native-admission boundary with genuine proof qualification still required. The public route exists at `src/routed/generate.rs:321`; the AD explicitly distinguishes capacity refusal from semantic out-of-domain, requires real Meter and original population, and treats timeout or memory exhaustion as inconclusive (`AD-006:28-46,48-64`). The review did not run proofs or assess full-capacity refinement.

AD-007: open decision. Its own `System Boundary` calls it unapproved, and the only isolated experiment ended with `MemoryObservationFailed`, no Kani verdict, cover or measured peak RSS (`AD-007:22-43`). The primary i64 draw/rebind proposal and versioned encoding record are a candidate, not proof credit; the physical u16 carrier needs a separate decision against AD-006. Do not approve a representation amendment or close the 16 GiB criterion from the recorded diagnostic.

## Dispositions

Round 1 rechecked PR #722 at `3119f7938eca3d63d49bf82f6bdb2a3b16dc5cc6` against the source cited above. No Cargo or Kani gate was run for this spec and documentation review.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-002 now names QSL as structural identity producer and CG as carrier, explicitly separates kind, occurrence and domains, and updates R-Q7. AD-004's associated digest claims were corrected separately under FND-004. |
| FND-002 | fixed | AD-002 now names `request_from_packet` and `WitnessEnvelope::reconstruct`, describes its claim comparisons, and marks the former missing function envelope gap resolved. The separate frame empty-domain gap remains visible. |
| FND-003 | fixed | AD-003 now identifies both terminal functions and the direct model dependency, marks the IR provider map and import migration historical/resolved, and updates R-I1/R-I3 and the open-question rows. |
| FND-004 | fixed | AD-004 now distinguishes QSL structural obligation identity from CG content artifact digests at the migration, canonical module and Decisions locations; its original inventory remains explicitly historical. |
