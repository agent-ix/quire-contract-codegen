---
id: "SR-690"
title: "CG PR 217 code review: AD-004 steps 1b-1d (typed symbols, RUNTIME_REVISION, model dependency)"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@757efc45d63ec97dddb83bbada83dacaeb908a76; src/identity.rs, src/profile.rs, src/routed_generation.rs, src/kani_obligations.rs, src/state_frame.rs, src/kani_execution.rs, src/spine_replay.rs, src/lib.rs, src/oracle.rs, src/exact_scalar.rs, src/composite_equality.rs, src/exact_function.rs, Cargo.toml, Cargo.lock, tests/**"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-690: CG PR 217 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#217 at 757efc4, base main fda2316 (main
has since moved to b2aa4e9, a doc-only AD-003 change; `git merge-tree` with it is clean). 46
files, +558 -260. Methods: code-review with the rust-review lane, run against the repository's
own CLAUDE.md (the hash and pin rule) and AD-004 (steps 1b to 1d, L-10, L-11, Shared core).

Measured:

- `make ci TRUSTED_HOME=<scratchpad home>` exit 0: fmt-check, spec (3 baseline warnings),
  clippy `-D warnings`, msrv (1.98.1) and stable test (105 unit, 245 integration passed with 9
  ignored, 1 doctest), deny (advisories, bans, licenses, sources ok; one-copy check passes),
  audit-unsafe, rustdoc `-Dwarnings`.
- `make kani` exit 0: 9 passed, 0 failed (the real-Kani lane: kani_obligations,
  kani_obligations_state_frame, kani_witness_join, skeleton_spine; 1056 s), with the test
  fixtures resolving the runtime as `branch = "main"`.
- Identity records and harness text are byte-identical between base and head. A scratch test,
  appended to both checkouts and then removed, dumped the three contract harnesses (V1 arm), the
  scalar harness for corpus node 1001, its records, both state-frame harnesses (healthy shape)
  and one corpus arithmetic case: rust source, record artifact and `serde_json` identity. `cmp`
  reported no difference: 26,028, 6,343 and 1,965 bytes. `KaniSolver` is
  `rename_all = "snake_case"`, so it serializes as `"cadical"` as the old `String` did.
- Newtypes: the tuple field is private. There is no `Deserialize`, `From<String>` or `Deref`, so
  the only way to build one is `TryFrom`, which validates. Validation is an ASCII
  first-character and rest check, then `syn::Ident`, which refuses keywords, `_` and
  `self`/`Self`/`super`/`crate`. Mutation reasoning: dropping the `syn` check admits
  `fn`/`self`/`_`, and dropping the ASCII check admits `naïve`. The unit test fails in both
  cases. The `compile_fail` doctest runs under `make test` (it is in the doctest count) and
  stops a swapped `HarnessPath`. No length bound, which is right for Rust identifiers. The stems
  come from bounded readable components.
- DuplicateHarness: if `index_harnesses` reverts to an overwriting `collect`, the
  `expect_err` fails. If the error names the first harness instead of the second, the
  `b_module::x_proof` assertion fails.
- 1d: `quire_contract_ir` is named only as `quire_contract_ir::kani::` in src and tests. No glob
  import remains. Cargo.lock gains one dependency line, and `quire-contract-model` and
  `quire-contract-ir` each have one entry at 0a889f9.
- No `#[allow]`, `unwrap`, `expect` or `panic!` was added outside tests and doctests. No CI
  workflow changed. `RUNTIME_REVISION` is gone from `lib.rs` with no alias kept.
- The test assertions are not weakened. The TC-029/TC-031 manifest tests now assert the whole
  dependency line and `!contains("rev =")`, which is stronger than `contains(RUNTIME_REVISION)`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Every test that compiles a generated crate (about 20 manifests in tests/it, all `cargo ... --offline`, with no lockfile except in exact_scalar_agreement) now names the runtime as `branch = "main"`. Cargo resolves that from whatever `refs/remotes/origin/main` the shared `~/.cargo/git/db` holds, not from CG's Cargo.lock. Measured on this machine: the db's main is 9e07f7a (fetched 2026-10-01 04:18), and CG's lock pins ccc722b. So `make ci` compiles CG-generated code against a runtime commit that CG itself never built against, and which commit that is depends on when any other repository on the machine last fetched RT. Before this PR the stale `rev = ed0a04b` was at least the same on every machine. Fix in the tests, not the emitted manifest: seed CG's own Cargo.lock into each compiled test crate as exact_scalar_agreement already does (`tests/it/exact_scalar_agreement.rs:63`). Then the `branch = "main"` spelling resolves to the locked ccc722b | tests/it/oracle_generation.rs:340, tests/it/kani_generation.rs:377, tests/it/strategy_generation.rs:205, tests/it/composite_equality_generation.rs:969, tests/it/exact_scalar_generation.rs:1415 |
| FND-002 | low | The runtime source spelling (`git = "https://github.com/agent-ix/quire-contract-runtime", branch = "main"`) is now a literal in about 24 test manifests (tests/it plus four support files). They used to share one public constant. `RUNTIME_DEPENDENCY_SOURCE` is private to `src/profile.rs`, so if the source changes the emitted manifest moves and the test fixtures do not. Nothing would catch that, because each fixture compiles against its own manifest. L-11 asks for one place. One test helper, or a `pub(crate)`/test-visible accessor, would make it one edit | src/profile.rs:9-10, tests/it/bound_strategy_generation.rs, tests/it/harness_generation.rs |
| FND-003 | low | Stale doc comment: the test's doc still says the manifest "pins the runtime revision", but its assertions now require `branch = "main"` and no `rev` | tests/it/exact_function_generation.rs:661-662 |

## Verdict

The code is sound. 1b's newtypes cannot be built without validation, they serialize bare, and
the records and harness text are byte-identical by measurement. DuplicateHarness replaces a
silent overwrite and is mutation-tested. 1c writes the manifest template once, and the
`branch = "main"` spelling is what AD-004 (Shared core, and Decisions taken from the planner,
`RUNTIME_REVISION`) and the repository's CLAUDE.md hash and pin rule prescribe. The floating
emitted dependency is the decided design, not a defect. 1d is complete, and `make deny` passes
with one copy.

FND-001 is the one finding to fix before merge. It concerns test hermeticity, not shipped
behaviour, and seeding the lock is a small change. FND-002 and FND-003 are small cleanups.
Mergeable after FND-001, or with FND-001 explicitly accepted.
