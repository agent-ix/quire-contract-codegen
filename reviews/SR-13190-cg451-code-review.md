---
id: "SR-13190"
title: "CG PR 451 re-review code review (Rust lane): typed repeated-identity envelope refusal"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@5c07c891b630056da636f5925de496def23b7f75; src/routed/capability.rs, tests/it/capability_settlement.rs, spec/routed/matrix/tests.md, reviews/SR-13130-cg451-code-review.md, reviews/SR-13131-cg451-gap-analysis.md (diff origin/main...HEAD, merge base 35f37aaf, origin/main 661ef492); context: spec/routed/functional/FR-019-capability-settlement.md at origin/main (AC-45, AC-46, AC-47, Outputs, Behavior note, paired-change paragraph); quire-driver origin/main src/refusal.rs and Cargo.toml"
---

# SR-13190: CG PR 451 re-review code review (Rust lane)

## Summary

Ticket: IR-887 (parent IR-878). PR: agent-ix/quire-contract-codegen#451 at 5c07c891. This is a
fresh pass over the reworked diff. The earlier findings are disposed in SR-13130 and SR-13131.
The code-review and rust-review methods share this one artifact.

What the review checked:

- The variant matches FR-019 exactly. It is `EnvelopeRefusal::ManifestRepeatsBackendIdentity {
  identity: String }`, on the existing enum with the existing derives (`Clone, Debug, Eq,
  PartialEq, Serialize`). It has no string cause and no manifest or digest payload. No code in the
  diff still uses `InvalidCapability`/`duplicate-backend`.
- Comparison. FR-019 says to treat the whole CG `BackendDescriptor` (identity, advertised, origin,
  domains, bounds) as the descriptor, so "both identical and differing descriptors repeat an
  identity". Keying detection on `identity` alone is the faithful implementation: any second
  descriptor of an identity refuses, whatever the other fields hold. Adding more fields to the key
  would be the defect.
  - Matching is exact and case-sensitive (`BTreeSet<&str>`).
  - The reported identity is the bytewise-first repeated one (`.min()` over `&str`).
  - The version check runs first, then the vocabulary check, then repeat detection, then item
    settlement.
- API. `negotiate_backend_provider` still returns `Result<Vec<ItemSettlement>, EnvelopeRefusal>`.
  CG has no exhaustive match on `EnvelopeRefusal`, so nothing inside CG needs updating.
- quire-driver is not edited here. Its `src/refusal.rs:250` and `:545` still match only
  `InvalidCapability`, which is the paired break FR-019 records. The driver pins CG at `rev =
  edade593`, so it does not break until that pin moves.
- Comments. The doc "Both carrier members refuse with `unsupported-version`" is true. The comment
  at capability.rs:1465-1470 is still false; that is SR-13130 FND-004 (still-open) and is not
  repeated here.
- Gates, run by the reviewer at the head: `make fmt-check` passed. `make lint` (locked clippy,
  `-D warnings`) passed. The `capability_settlement` filter passed 48 of 48 (44 existing plus 4
  new), with an explicit QSPEC_DIR pointing at a quire-specification worktree at origin/main
  6fd431e2. Kani execution tests were not run.
- Mutation probes, run in a throwaway worktree:
  - first-match revert: killed
  - adjacent-only check: killed
  - ignore second entry: killed
  - `.max()` in place of `.min()`: killed
  - case-fold identity: killed
  - repeat check before version/vocabulary: killed
  - key includes origin: killed
  - key includes domains: survived (SR-13191 FND-001)

## Verdict

**PASS** for the code-review and Rust lanes on the new diff. The implementation, variant and check
order match the merged FR-019. There are no new code findings. Open items from earlier rounds are
tracked in SR-13130 (FND-004 is still-open).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Dispositions
