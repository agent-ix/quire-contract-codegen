---
id: "SR-1114"
title: "CG PR 245 code review with Rust lane: QSL replay API, IR main, byte-limit lowering refusal"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@5d60e06c836b96a3f803b53a267326d98656b57d; Cargo.lock, deny.toml, src/oracle/mod.rs, src/oracle/scalar/mod.rs, src/oracle/equality/mod.rs, src/oracle/function/mod.rs, src/kani/generate/negotiate.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/replay/frame.rs, src/replay/function.rs, tests/**; quire-spec-language@6f314877 (read only); quire-contract-ir@cbcd790 (read only); quire-driver@fc53a75 (read only); diff origin/main...HEAD, base 27153ba"
---

# SR-1114: CG PR 245 code review with Rust lane: QSL replay API, IR main, byte-limit lowering refusal

## Summary

Ticket: IR-548 (also IR-547, IR-536, IR-537). PR: agent-ix/quire-contract-codegen#245 at 5d60e06, three commits on main 27153ba. The PR body and the author's report were treated as claims. Each was measured.

What was measured:

- **Lock.** The diff touches only first-party entries. qsl-* and quire-exact move a28a5578 to 6f314877. quire-contract-ir and quire-contract-model move 968ba9b8 to cbcd790a. quire-verification-contracts moves ead78f3f to ec4563ff. quire-canonical moves from tag v0.3.0 to branch main b4bb97a5. The PR adds quire-canonical-derive, quire-semantic-value and quire-walk. No third-party package entry changed. syn 3.0.6 was already locked. Cargo.toml is unchanged: every first-party dependency is still `branch = "main"`, there is no rev or tag pin, and there is no direct qsl-foundation dependency. `Category` comes from `qsl_replay`'s re-export. deny.toml allow-lists the three new crates. No vendored or copied files.
- **QSL adaptation.** Checked against QSL origin/main 6f31487:
  - `selection()` has 6 sites. They are mechanical.
  - `Category` is `qsl_foundation::diagnostic::Category`, re-exported by qsl-replay (lib.rs:96). It now has more variants than Success and Violation. `verdict_of` still reproduces only `(ReproducedWithEvaluatedWitness, Violation)`, so the meaning is preserved.
  - `ReplayRequestWire.obligation_identity` replaces `originating_counterexample_identity`, and the three version and vocabulary members are gone. The PR matches QSL request.rs:236-263. QSL reads the slot only to label `ReplayRefusal::Witness` (execute.rs:813-862).
  - The frame path now sends `FrameReplayInputs::obligation_identity`, which matches the slot's documented meaning (ADR-013 O-09).
  - The function path still passes the transcript `ByteDigest`. The comment at function.rs:435-437 says this is a placeholder and that no code in CG computes the O-09 identity. That is true: AD-003 E-1 is open.
- **IR adaptation.** Checked against IR cbcd790:
  - `document_pointer: None` is added to the struct literal.
  - `NominalIdentityPreimage::digest(CheckedPackageReadLimits::bounded().bytes)` uses the right limit. IR's own validator digests under the reader's byte limit (identity.rs:453-483), and the fixtures are read under `bounded()`.
  - `CheckedPackageLimit` (7 variants) is not `#[non_exhaustive]`, so the classifier's exhaustive match makes a new IR kind a compile error.
- **Byte-limit refusal.**
  - `classify_lowering_failure` is the only reader of `limit_kind`. Each generator has exactly one `Failed { .. }` arm, and that arm calls the classifier.
  - negotiate sends both new variants to `OracleRefused` unchanged, in a match with no wildcard.
  - `LoweringWorkExhausted` is produced only for `Work`.
  - There are no other `Failed` sites in src/.
- **Public API.** The three public refusal enums gain new variants, and `FrameReplayInputs::counterexample_identity` is removed. quire-driver at fc53a75 names none of the three enums and does not use `FrameReplayInputs`, so nothing there breaks.
- **Gates (run by this reviewer on 5d60e06, each stage's own rc).**
  - lint 0, test 0 (122 lib + 272 it + 1; 15 ignored), deny 0, audit-unsafe 0, rustdoc 0, fmt-check 0.
  - `make kani` rc 0: all 15 real-prover tests passed in 969 s, including the bounded corpus, the skeleton spine, the frame replay and the witness join.
- **Mutation probes (throwaway worktree).** Killed:
  - classifier bytes mapped to work;
  - equality arm reverted to work exhaustion;
  - function `limit` and `consumed` swapped;
  - negotiate byte arm mapped to work;
  - E_SELF dropped from the golden items;
  - `verdict_of` ignoring the category;
  - unrecognised name swapped;
  - scalar unrecognised kind mapped to work.

  Survived: the frame-path request carrying `[0; 32]` instead of the obligation identity (FND-003).

## Verdict

Approve with low findings. No finding blocks the merge.

The adaptations preserve meaning, not just compilation. The classifier's exhaustive, wildcard-free match is stricter than the spec's "any other kind" wording, and it follows the repo's own compile-error precedent (FR-019). I judge it correct, not a deviation. Restoring E_SELF matches the vectors that were removed in baab597, verbatim. IR #256 (1a59efc) is an ancestor of cbcd790, so the deleted refusal pin is obsolete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The three `Failed { .. }` arms fall back to `InvalidInput` when `classify_lowering_failure` returns `None`. That branch is unreachable, because the arm has already matched `Failed`. If it were ever reached, it would silently report a lowering failure as invalid input. | src/oracle/scalar/mod.rs:912-913, src/oracle/equality/mod.rs:893-896, src/oracle/function/mod.rs:691-695, src/oracle/mod.rs:379-390 |
| FND-002 | low | The rebuilt TEMPORAL fixtures say "CG still refuses it on its temporal tag alone". The scalar vector now expects `unsupported_node_id` to be `TEMPORAL_FORMULA`: the refusal comes from the clause's formula dependency, which is also `temporal`-tagged. So no vector shows that the `temporal_clause` node's own tag is refused, and the comment overstates what the test observes. | tests/it/exact_scalar_generation.rs:463-465, tests/exact_scalar_support/package.rs:2754-2757, tests/composite_equality_support/package.rs:1272-1276 |
| FND-003 | low | No test asserts that the frame-path replay request carries `FrameReplayInputs::obligation_identity`. A probe that replaced it with `[0; 32]` in `frame.rs` survived the frame tests. | src/replay/frame.rs:149-154 |
| FND-004 | low | The new comment in `corpus_package` that wraps to a line starting `FR-370), so` is read by `quire coverage` as a trace tag on a non-binding symbol (CR-061). It is a new coverage warning that main does not have. | tests/composite_equality_support/package.rs:1274 (bound to `corpus_package` at :983) |

## Dispositions

Round 1, reviewed at b8cf39ddc15e33d730d1dbc1d0deb349343f0d11 (one commit fast-forwarded on 5d60e06). Gates rerun by the reviewer: lint 0; test 0 (122 lib + 273 it + 1); kani 0 (15 passed, 920 s); quire validate 0; quire coverage --strict 66 unbacked, the same as main. The lock moves only the 11 QSL entries, 6f31487 to 934e26f, and nothing else. QSL 934e26f adds `function` and `declaration` to `FunctionSite`. CG never constructs a `FunctionSite`; it reads only `site.parameters` and `site.package_id` (function.rs:408-410, :440), so CG needs no change.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | accepted-no-change | The `InvalidInput` fallback stays, and each of the three arms now has a comment saying it cannot be reached and why. FR-014-AC-42 requires `Failed { .. }` to bind nothing, so any classifier the arm calls takes the whole record and must say something about non-`Failed` input. Removing the branch would need a fourth outcome or an AC-42 rewording, and neither would buy any behavior. The comment is accurate. |
| FND-002 | fixed b8cf39d | Both corpus comments now say CG refuses the clause "as an unsupported temporal family". The scalar comment adds that "the refusal names `TEMPORAL_FORMULA`, the first unsupported node IR reaches from the clause", which matches the vector at tests/it/exact_scalar_generation.rs:465. |
| FND-003 | fixed b8cf39d | New test `tc_025_the_frame_replay_request_and_envelope_carry_the_supplied_obligation_identity` asserts `wire.obligation_identity == [1; 32]` and `packet.obligation_identity == Some([1; 32])`. The reviewer reran the surviving mutant (request gets `[0; 32]`) and a sibling (envelope gets `Some([0; 32])`), and the new test kills both. |
| FND-004 | fixed b8cf39d | The comment no longer has a line starting with `FR-370`. `quire coverage --strict` prints zero CR-061 / tag-on-non-binding-symbol lines. |
