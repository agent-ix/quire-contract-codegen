---
id: SR-1820
title: "IR-631 follow-up code review: scalar identity and authority against merged QSL source"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/assurance/AD-003-evidence-chain.md, spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-047
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-1820: IR-631 follow-up code review

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only: base `5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60`, head `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8`, three files, 345 additions and 84 deletions. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af.

The diff changes no production or test code, so I ran no separate Rust review lane and no gap analysis. This pass checks the changed text against the measured upstream and CG source. One medium finding.

## Method

I read the full diff `5d3eaa2..f84c4fc`. I then measured the following read-only with `git show`, without a checkout:

- **Public QSL, merged `30d7beb7483b721bcb1ec98926e8b717e9a76fb7`:** QSL #645, which contains #650 at `6f8518414a88b48a380c54e4c3171bbf9968d0a1`; I confirmed that ancestry. Files read:
  - `spec/functional/FR-357-replay-scalar-parity-claims.md`
  - `qsl-replay/src/lib.rs`
  - `qsl-replay/src/scalar.rs`
  - `qsl-replay/src/execute/parity_identity.rs`
  - `qsl-replay/src/execute/operator_parity.rs`
  - `qsl-replay/src/execute/scalar_site.rs`
- **CG at the head:**
  - `src/kani/identity.rs` (`ScalarObligationIdentity`, `ScalarObligationArgument`, `ObligationKind`)
  - `src/kani/generate/scalar.rs`

I loaded the Rust review checklist before reading any Rust source. I ran no cargo, Kani, tests or builds. Private IR source is outside this review's payload, so the IR `CheckedPackageV2` claims were not re-measured. They are treated as the author's claims, which the IR-648 code gate already keeps open.

## Verdict

**PASS with one medium finding.** The following match the measured merged source:

- **Mint function:** `parity_obligation(&ParityPreimage) -> Result<ObligationIdentity, IdentityEncodeError>` is public and is the one shared encoder. `Domain::Range`/`Bounds` and `ParityArgument { identity, domain }` exist. Position is the enumerate index, not a struct field.
- **Canonical members:** the closed preimage is `arguments`, `node`, `obligation_kind`, `occurrence_key`.
- **Operand encodings:**
  - `graph_child` is `{node_id, tag}`.
  - `inline_literal` is `{node_id, occurrence_key, position, tag}`, using the application id.
  - `range` is `{lower, tag, upper}`, with decimal-string bounds.
- **Encoder failure:** it refuses typed; there is no zero sentinel.
- **Check order in `operator_parity.rs::settle`:** node, then operator, then enclosing function, then body, then occurrence, then operands (`OperandCount`/`OperandChild`/`NotInlineLiteral`/`LiteralValue`), then the obligation digest, then `compare`.
- **Native faults:** in `compare`, native `Incomplete`/`ExecutionFault` return `GeneratedFault` before operand admission.
- **Facade signature:** `replay_operator_parity(wire, claim, replay_limits)` takes `ReplayLimits` last. That is separate from `OperatorClaim.limits: ScalarLimits`, and `OperatorIdentity` has no `ReplayLimits` member.
- **Full claim:** `OperatorIdentity` holds obligation, node, content `DigestRecord`, operation (operands with identity and range), occurrence, kind, result range, limits and native outcome. It is carried on every outcome, including the decode refusal.
- **Terminal mapping:** `Diverged`/`GeneratedFault` give `Failed`. `Agrees` gives `Inconclusive(ScalarAgrees)`. `RefusedInput` gives `Inconclusive(ReplayRefused(invalid_runtime_input))`. Exact `Incomplete` gives `Incomplete(ResourceExhausted)`. `Refused` goes through `from_replay_refusal`. Nothing yields `Refuted`.
- **CG today:** `ScalarObligationArgument` has only `identifier`/`minimum`/`maximum`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-047 step 6 requires "corrupting the returned report while retaining the actual sent claim". The merged public API makes that impossible: `OperatorParityReport` has private fields, a `pub(crate)` constructor, no `Deserialize`, and `claim()` returns `&OperatorIdentity`. Only `replay_operator_parity` builds a report, and it copies the sent claim verbatim. The same step also forbids fabricating a full report, and FR-032 never defines the converter's input boundary (a report, a decoded form, or a separate `OperatorIdentity` plus result). The required mismatch therefore cannot be produced, and the check can only be met by testing claim B against claim A, which is the "another result/run" case it already lists. FR-032 should define the converter input where a mismatch is observable, and TC-047 should mutate the retained sent claim, or that boundary, instead of the report. | spec/replay/matrix/TC-047-routed-scalar-replay-binding.md:80-94, spec/replay/functional/FR-032-routed-scalar-replay-binding.md:135-142 |

## Dispositions

Round 1 re-check of fix commit `832633d7afa778e8a3688601595beb5d38917cb5` (run 733cf463-44a3-4ff9-83f7-a1a0a32f9463, model claude-opus-5-5). Each finding was verified against the spec text at that commit, not against the author's receipt. The reviewed content of `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8` was confirmed unchanged after the rebase onto main. The planned criteria remain unrun; no implementation, mutation coverage or settlement proof is claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: TC-047 step 6 now passes the immutable public report with the retained expected OperatorIdentity and mutates one retained member at a time; FR-032 adds a Converter Boundary section that never constructs, deserializes or alters the report. |
