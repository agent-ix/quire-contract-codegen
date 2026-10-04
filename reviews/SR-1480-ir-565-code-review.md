---
id: "SR-1480"
title: "CG PR 269 code review (with rust-review lane): lock move to QSL 47dd209 and IR 6fb6e97, flat wire, decimal-string integers, versionless dependency entries"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@17f779983e55e025fde18969ad08ee51f59fbf22; Cargo.lock, deny.toml, src/oracle/mod.rs, src/oracle/scalar/mod.rs, src/replay/function.rs, src/kani/generate/negotiate.rs, tests/common/withdraw_fixture.rs, tests/exact_scalar_support/package.rs, tests/it/{bound_generation,bound_strategy_generation,exact_scalar_generation,kani_obligations,kani_witness_join,skeleton_spine}.rs (gh pr diff 269, against main at merge base f3ece43)"
---

# SR-1480: CG PR 269 code review

## Summary

Ticket: IR-565 (primary), with IR-570 and IR-579 in the same PR. PR:
agent-ix/quire-contract-codegen#269 at 17f7799, a merge of `origin/main` f3ece43 into the WIP
commit aa71a53. `origin/main` has since moved to 67bc7b3 (#266). That commit touches only
`src/kani/run/{execute,launch}.rs` and a review file, and none of the PR's files. The
`rust-review` lane is in this file.

What I checked myself. I did not rely on the PR body.

- `Cargo.toml` is unchanged against f3ece43. Every first-party selector is still
  `branch = "main"`. No `rev`, `tag` or SHA was added outside `Cargo.lock`, and nothing was
  vendored or copied.
- `Cargo.lock`: QSL crates move 934e26f to 47dd209. IR and model move cbcd790 to 6fb6e97.
  quire-canonical moves b4bb97a to 5dc4e12 and QVC moves ec4563f to 1fc0ff6. quire-walk is now
  `git+https://github.com/agent-ix/quire-walk#89d05df`, still one entry, so `check_one_copy.awk`
  passes. The only third-party change is unicode-ident 1.0.26 to 1.0.24. IR's
  `crates/quire-contract-model/Cargo.toml` at 6fb6e97 pins `unicode-ident = "=1.0.24"`, so the
  downgrade is forced.
- `deny.toml`: the only change adds `https://github.com/agent-ix/quire-walk` to `allow-git`.
  This is needed because IR's model and QSL 47dd209 both take quire-walk from that repo.
  `make deny` passes: advisories, bans, licenses and sources are ok, and one-copy passes.
- Each adaptation is forced by the upstream change:
  - IR 6fb6e97 removed `CheckedPackageLimit::Depth` ("There is no depth limit",
    `checked_package/shared.rs`). The classifier arm and the `UNRECOGNISED_KINDS` entry had to
    go. The tests that used `Depth` as an arbitrary kind (`BodyIncomplete`,
    `refusal_variant_name`, the tc_025 negotiate mapping) were relabelled to `Nodes`/`"nodes"`.
    Each of those tests asserts independently of the kind, so nothing was weakened.
  - IR's `decimal.rs` names eight integer members that are decimal strings. The decoder
    "accepts a string and nothing else". Every edited fixture member is one of those eight:
    `IntegerType.minimum`/`maximum` and `IntegerLiteral.value`. Every remaining numeric member
    in a fixture (`multiplicity.lower/upper`) is outside the eight.
  - QSL #626, #627 and #621 (imports and lock selections bind by identity, with no version)
    remove `version` from `SuppliedLibrary` and `DependencyEntryWire`. CG's `DependencyLock.version`
    had no other reader. The spine import `import "test/units" as u;` follows the
    identity-only import grammar. The package_id binding is still asserted by
    `tc_026_a_lock_recording_another_dependency_identity_is_refused`, which is unchanged.
  - Flat wire (IR `flat_wire.rs`): the Member place admits a Leaf (`literal`, `reference`,
    `dependency_reference`), a Group (`aggregate`) or a `binding`, and never an `application`.
    So the EXPRESSION_OPERAND fixture's nested application had to become something else. An
    `aggregate` with one `binding` of a Leaf is a Group the reader admits. The fixture still
    exercises the `other =>` arm of `check_operand`.
- Test inventory: the count of `#[test]` attributes is 540 at f3ece43 and 540 at head, and
  `#[ignore]` is 27 at both. No test function was added, removed or renamed. One assertion
  line was removed (`entry.version == lock.version`, a field that no longer exists) and one
  was added (FND-001).
- Gates I reran at head in my own target dir: `cargo test --locked` gives 161 unit, 300 `it`
  (21 ignored) and 1 doctest, with 0 failed. `cargo clippy --locked --all-targets -D warnings`
  is clean. `cargo fmt --check` is clean. `make deny` passes and `make spec` exits 0. I did not
  rerun the real-Kani lane. Its 21 ignored tests match the claimed count of 21.
- Author's question, whether `ExactScalarRefusal::OperandUnsupported` is still reachable: yes,
  for any package that IR's V2 reader admits. It is not reachable from QSL's own emitter.
  - IR's flat grammar admits `aggregate`, `binding` and `dependency_reference` as application
    arguments. The EXPRESSION_OPERAND fixture passes the reader and reaches the arm, and the
    test asserts `OperandUnsupported { position: 0, term: "aggregate" }`.
  - QSL 47dd209's lowering (`qsl-semantics/src/check/lowering.rs`) puts a
    `dependency_reference` only as the callee of a `quire.op.function.call`. It puts an
    `aggregate` only as a node body, for a tuple or record. A sub-expression operand is always a
    `reference` to its own node. So a QSL-built package never reaches the arm.
  - CG's input contract is an admitted `CheckedPackageV2`, not QSL output. FR-014's Behavior
    section and FR-014-AC-10 require a typed refusal for "an operand that is neither a literal
    nor a reference".
  - The arm is therefore the total mapping of a closed set that the reader still admits. It is
    not dead code. Removing it would need either a spec change to FR-014-AC-10 or an
    `unreachable!`, and FR-014-AC-39 forbids `unreachable!`.
  - `term: "application"` is no longer reachable at all. No spec text names it.
- Next bump, QSL 02530e7 (read-only check of `git diff 47dd209 02530e7`): no line in this PR's
  diff would stop compiling. `DependencyEntryWire`, `SuppliedLibrary` and the import grammar
  are unchanged there. Lines outside this diff do break. 02530e7 makes
  `ReplayRequestWire.backend` a `String` and `Backend::new(identity)` takes no manifest digest.
  That breaks `src/replay/function.rs:380-384`, which builds the `(identity, domain, hex)`
  tuple from `backend_manifest`, and `tests/it/skeleton_spine.rs:361`, which reads
  `wire.backend.1`. The 02530e7 bump will have to change both, together with
  `ReplayInputs.backend_manifest`. Those changes belong to the next bump, not to this PR.

## Verdict

Approve. The PR is the minimal adaptation to the lock move. I found no smuggled behaviour
change, no weakened assertion, no `#[ignore]`, no deleted test, no pin and no shim. There are
two low findings, both about test oracles. Neither blocks the merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The assertion added in place of the removed `entry.version == lock.version` is `assert_eq!(entry.identity, lock.identity)`. The test already asserts `identities == ["test/aaa", "test/mmm", "test/units"]` a few lines above, and `entry` is `wire.dependencies[2]`, so the new line cannot fail independently. It adds no oracle strength. Deleting the version assertion with no replacement would be as strong and more honest | tests/it/skeleton_spine.rs:369 |
| FND-002 | low | `UNRECOGNISED_KINDS` is a hand-kept `[_; 4]` list. Nothing ties it to the variants of `CheckedPackageLimit`. If a lock move adds a kind, `classify_lowering_failure`'s exhaustive match stops compiling and gets fixed, but the three `tc_0xx_a_failed_record_is_refused_by_its_limit_kind` tests would still pass without asserting the new kind. An exhaustive `match` inside the test helper would make the compiler force the update. This lock move is exactly the moment the list is edited by hand | src/oracle/mod.rs:95-100 |

## Dispositions

Round 1, reviewed at 9b87323bfc33987e770bafa8177d159851c53b50.

- **Delta.** Since 17f7799 there are two changes: the fix commit 2645759 (5 files:
  `src/oracle/{mod,scalar/mod,equality/mod,function/mod}.rs` and `tests/it/skeleton_spine.rs`)
  and a clean merge of `origin/main` 67bc7b3 (#266: `src/kani/run/{execute,launch}.rs` and two
  review files). `git diff 17f7799 9b87323` contains nothing else.
- **Gates I reran at head in my own target dir.** fmt check clean; clippy `-D warnings` clean;
  `cargo test --locked` gives 161 unit, 300 `it` (21 ignored) and 1 doctest, 0 failed.
- **FND-001.** `wire.dependencies[2]` is now destructured as
  `DependencyEntryWire { identity, package_id, sources }` with no `..`. If the wire gains a
  member again (a `version`), the test stops compiling. That is a real check on the entry's
  shape, and the replacement had none.
  - The runtime line `assert_eq!(identity, "test/units")` still cannot fail on its own.
  - Probe P2 (each entry's identity replaced by its source path, `src/replay/function.rs:340`)
    is killed by the identities-list assertion at line 359. The literal is never reached.
  - It is harmless, and the finding as raised (no strength in the replacement) is resolved by
    the destructure.
- **FND-002.** `unrecognised_name` matches every `CheckedPackageLimit` variant with no wildcard,
  and `unrecognised_kinds()` derives the four kinds through it. All four callers were updated:
  the classifier test and the scalar, equality and function failed-record tests. The three
  `tc_0xx` tests still assert all four kinds.
  - Probe P1 (classifier gives `"edge"` for `Edges`): all three failed-record tests failed.
    Reverted.
  - Residual: the list of six variants inside `unrecognised_kinds()` is still written by hand.
    A new variant forces an edit to `unrecognised_name` 10 lines above it, but not to the list.
    IR's enum offers no way to iterate its variants, so nothing in CG can enforce this fully.
    This is the remedy the finding proposed. No new finding.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2645759 |
| FND-002 | fixed | 2645759 |
