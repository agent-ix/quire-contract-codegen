---
id: "SR-037"
title: "IR-21 code review (incl. rust-review lane): skeleton spine through Kani to QSL replay"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@d2987efc51b1d34aabda0dbd909663e9b884feef; Cargo.toml, Cargo.lock, Makefile, src/kani_execution.rs, src/kani_module_gate.rs, src/kani_transcript.rs, src/lib.rs, src/spine_replay.rs, tests/fixtures/skeleton_spine/claimed-modules.txt, tests/it/kani_obligations.rs, tests/it/main.rs, tests/it/skeleton_spine.rs"
relationships: []
---

# SR-037: IR-21 code review

## Summary

Ticket: IR-21. PR: agent-ix/quire-contract-codegen#184, head `d2987ef`, base `origin/main` `f2fd369`.

The PR adds `replay_falsification` (src/spine_replay.rs), a thin adapter that builds a
backend-witness transcript and calls `qsl_replay::replay`. It adds `claimed_module_gate`
(src/kani_module_gate.rs), which reads Kani's per-check `RESULTS:` listing through a new
`KaniTranscript::checks` parser. It adds `execute_kani_obligation_with_transcript`, and a spine test
that proves one postcondition, runs two mutation controls, decodes the counterexample and replays it
through QSL. It moves the QSL pin to `20ba521` and adds two QSL dev-deps at the same rev.

## Method

I read the full diff. The rust-review lane covered idioms, error envelopes, panics, trait seams,
stubs and tautologies. I traced the replay path to confirm that no verdict comes from CG. I checked
QSL `origin/main` for the T12-A and FB-05 rules (`tools/arch-lint/api_surface.rs`,
`spec/functional/FR-060-*`, `qsl-replay/src/lib.rs` at `20ba521`). I compared the `write_crate`
manifest with the generator's own `manifest()` functions. Then I ran the gates below in my own
worktree.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The replay request is built only in the test, and the test reaches `qsl_replay::spine::compile` (forbidden to CG by FB-05/T12-A) plus two extra QSL crates. CG's src has no way to build a replay request with qsl-replay alone. | src/spine_replay.rs:85-91, tests/it/skeleton_spine.rs:16-24, tests/it/skeleton_spine.rs:69-96, tests/it/skeleton_spine.rs:120-173, Cargo.toml:40-44 |
| FND-002 | medium | The spine overstates its chain. The input starts at a hand-built Contract IR JSON projection, not a contract. The "proving run's package identity" comes from a hand-mirrored QSL twin that Kani never proved. The limits are UNLIMITED, not the run's. | tests/it/skeleton_spine.rs:1-9, tests/it/skeleton_spine.rs:62-69, tests/it/skeleton_spine.rs:118-173, tests/it/skeleton_spine.rs:292 |
| FND-003 | low | `mod kani_module_gate` is tagged `// Implements: FR-017`; the owning requirement is FR-023. | src/lib.rs:27-28 |
| FND-004 | low | The replay oracle is weaker than it looks. The default-lane witness (1,0) breaks the proved precondition `amount-within-balance`. Neither twin depends on the value beyond amount>0, so no test pins that QSL evaluated at the witness point. The Kani-lane healthy replay checks only the settlement. | tests/it/skeleton_spine.rs:198-242, tests/it/skeleton_spine.rs:377-388 |
| FND-005 | low | `FieldDelimiter`, `Transcript`, `Refused` and `WrongArm` are untested. The Boolean-as-0/1 encoding that FR-016/interface-001 claim is never exercised: the only Boolean in a test is refused before encoding. | src/spine_replay.rs:88-122, tests/it/skeleton_spine.rs:247-257 |
| FND-006 | low | "Green in CI" (IR-21 AC-1) holds only in the manual `make kani` lane. The prover spine test is `#[ignore]`, and `make ci` does not include `kani`. | tests/it/skeleton_spine.rs:289-290, Makefile:272 |

### FND-001 detail

`replay_falsification` takes `request: impl FnOnce(ReplaySource) -> ReplayRequestWire`. The package
id, source digests, byte provision, backend digest and every limit come from the caller. The only
caller is `tests/it/skeleton_spine.rs::request`. It needs `qsl_foundation::digest::{DigestRecord,
DigestDomain, ByteDigest}`, `qsl_foundation::SourceIdentity` and `quire_exact::{ScalarLimits,
Identifier}`, which are the new dev-deps. It learns the package id and parameter node ids from
`qsl_replay::spine::compile`.

At `20ba521`, `qsl-replay/src/lib.rs` says "`spine` is public only for `command`", and "CG reaches
this crate's public API and nothing else in QSL (ADR-011 FB-05)". FR-060's T12-A says any reference
to `qsl_replay::spine` is a violation from any module. The arch-lint only scans `src/`
(`Role::Cg => vec![scan_root.join("src")]`), so the test passes the lint because of the lint's
scope, not because an exemption exists.

Failure scenario: the first production caller (the IR-217 widening, or FR-014-AC-6) has to build
this request in `src/`. It must then either call `spine::compile` (T12-A red), or promote
`qsl-foundation` and `quire-exact` to normal deps (breaking "qsl-replay is the one QSL edge" in
FR-016's own Dependencies). A QSL change that narrows `spine` breaks this test too. Recommend
either: (a) a QSL facade ticket that exposes the package id and parameter node ids plus a request
builder through qsl-replay, cited in the test's module doc; or (b) a documented, time-boxed
test-only exception with a ticket and an expiry condition.

### FND-002 detail

`bound_package(1000)` is a hand-written JSON projection (tests/it/kani_obligations.rs:293-332).
Nothing compiles a contract, so the contract to Contract IR arrow is not exercised.
`prove_identity` compiles `native_source(...)`, a QSL function hand-written to mirror the Rust
subject. It is unrelated to the proved `PACKAGE`, and only argument names join them. The module doc
("goes from contract to Contract IR") and the `request` doc ("the package reference and limits are
the run's") say more than the code does, and the limits are `UNLIMITED`. IR-21 AC-4 limits evidence
to the stages the input passes through. Failure scenario: a G1/G3 gate reader takes TC-026/TC-034
"Covered" as evidence for stages the input never passed through. Fix: rename `prove_identity`, say
in the module doc and in TC-026/TC-034 that the input is a hand-built IR projection and the native
twin is hand-mirrored, and name which ADR-011 stages the spine covers.

## Non-findings checked (clean)

- **FB-07 / no stub or predetermined verdict.** src calls `qsl_replay::replay` directly, with no
  injection seam. The healthy twin settles `Inconclusive` while the violating twin settles
  `ReproducedWithEvaluatedWitness`/`Violation` with `work_units > 0`, so a constant verdict fails
  one of the two. This oracle is meaningful: it proves QSL evaluated the source.
- **Mutation controls hit each claimed module.** The generated-module control tightens the
  ensures post-state bound (`*post_state >= 0_i64` becomes `>= 1_i64`, asserted to occur once). The
  subject control swaps `-` for `+`. Both falsify on a real run and turn the gate red via
  `NotVerified`. The gate is red for any falsified run, so the controls prove the proof depends on
  each module, not that the per-module counter sees them. The `unreached` path is exercised on the
  real transcript by claiming `dead`.
- **Vacuity is not a loophole.** An incidental `SUCCESS` check could discharge a module whose clause
  assertion is unreachable, but `Verified` already requires the harness cover to be satisfied (FR-017
  classification).
- **Path-segment matching** (`module` or `module::`) and the RESULTS parser are unit-tested, and
  `tc_027_no_other_source_file_contains_kani_prose_literals` still passes.
- **Pins.** `qsl-replay`, `qsl-foundation` and `quire-exact` are all at `20ba521`, an ancestor of QSL
  main (main is now `30f0358`). The old `daa0178` is gone from Cargo.lock. The `9395be4` QSL rev
  that stays comes in through quire-contract-ir and predates this PR. No QSL file is copied or
  vendored.
- **`write_crate` `features = ["exact"]`.** This matches the generator's own manifests
  (src/exact_function.rs:1421, src/composite_equality.rs:1573). The harness uses
  `quire_contract_runtime::exact`, and the prover run passes with it. The change is test-only and
  correct.
- **Makefile `kani` target.** Both libtest filters after `--` select exactly the two modules.
- **Coder's red-on-main claims.** `make spec` exits 2 on `f2fd369` too, with the same two documents
  (AP-001, MP-001), and the PR adds no spec validation error. `cargo deny check` exits 12 on
  `f2fd369` too (AGPL license rejections plus unlisted git sources). The PR adds one more AGPL
  rejection of the same kind (`qsl-attrs@20ba521`). I did not re-measure tc_025.

## Gates run by this review

| Gate | Command | Exit | Log |
| --- | --- | --- | --- |
| Format | `cargo fmt --check` | 0 | logs/fmt.log |
| Clippy | `cargo clippy --locked --all-targets -- -D warnings` | 0 | logs/clippy.log |
| Unit (gate, transcript) | `cargo test --locked --lib -- kani_module_gate kani_transcript` | 0 (15 passed) | logs/unit.log |
| Spine default lane | `cargo test --locked --test it -- skeleton_spine` | 0 (3 passed, 1 ignored) | logs/it-spine-default.log |
| Spine prover lane | `cargo test --locked -j 4 --test it -- --ignored --test-threads=1 skeleton_spine` (stable toolchain, not +1.98.1) | 0 (1 passed, 107.94s) | logs/kani-spine.log |
| interface-001 | `it interface_001` | 0 (6 passed) | logs/interface001.log |
| Upstream identity | `make upstream-identity` | 0 | logs/upstream-identity.log |
| Spec (head / main) | `make spec` | 2 / 2, same pre-existing docs | logs/spec-head.log, logs/spec-main.log |
| Deny (head / main) | `cargo deny check` | 12 / 12, pre-existing classes | logs/deny-head.log, logs/deny-main.log |

Logs are under `/tmp/claude-1000/-home-peter-dev/7460db9c-2787-438d-92ba-cee3b3bbc775/scratchpad/ir21-review/`.

## Verdict

No HIGH findings, and the functional spine works on a real prover run. Two MEDIUM findings (FND-001
and FND-002) concern whether the spine is described honestly and whether the replay request can be
built from src. Fix both, or disposition them with a ticket, before merge.

## Dispositions

Round 1, reviewed at `728e4312b8f2be6ff70a39c568ad549bc69a1546` (fix commit `728e431`).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | The test-only T12-A/FB-05 exception and its expiry condition are now written down (Cargo.toml:40-44, tests/it/skeleton_spine.rs:14-20, FR-016 Dependencies). No ticket tracks the expiry, though: none is cited, and Linear search finds no QSL/IR ticket for the qsl-replay re-exports or for exposing a compiled unit's ids. File one and cite its id in Cargo.toml and the test's module doc. |
| FND-002 | fixed 728e431 | The module doc names the stages the input passes through (hand-built BoundPackage; E1-E4 and contract-to-IR do not run; E7, E8, E9 do). `prove_identity` is renamed `compile_native_twin`. The limits are documented as unlimited stand-ins. The test is renamed `..._from_a_bound_package_...`. TC-026/TC-034 say the same. |
| FND-003 | fixed 728e431 | src/lib.rs:27 now reads `// Implements: FR-023`. |
| FND-004 | fixed 728e431 | The witness is now (1,5), which satisfies amount<=balance. A new test replays (0,5) against the violating twin and expects inconclusive with proved=violation and replayed=success. Both healthy-twin replays (default and prover lane) now assert the named verdicts. The prover lane asserts that the decoded counterexample satisfies the precondition. |
| FND-005 | fixed 728e431 | `tc_026_each_adapter_refusal_is_its_own_typed_error` covers FieldDelimiter, Transcript, Refused (stale package_id) and WrongArm. `tc_026_a_boolean_value_replays_as_zero_or_one` replays false (reproduced) and true (inconclusive). |
| FND-006 | fixed 728e431 | The skeleton_spine module doc, FR-023 and TC-034 Status now state that the prover spine is an ignored `make kani` test outside `make ci`. |

Round 2, reviewed at `9e1787611dac0bf29bacec38dff4804e2f3c86ce` (fix commit `9e17876`).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 9e17876 | IR-309 now tracks the exception and names the expiry condition. It is cited in Cargo.toml:40, the tests/it/skeleton_spine.rs module doc (line 15) and FR-016 Dependencies. |
