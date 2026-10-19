---
id: "SR-13100"
title: "CG PR 450 code review (Rust lane): FR-019-AC-4 and FR-019-AC-7 trace tags in the capability settlement tests"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@5cb8ab212e360682ceb695637086b5bf2705f7b8; tests/it/capability_settlement.rs, spec/routed/matrix/tests.md (diff origin/main...HEAD, base 47f9abf); read for context: src/routed/capability.rs, spec/routed/functional/FR-019-capability-settlement.md"
---

# SR-13100: CG PR 450 code review (Rust lane)

## Summary

Ticket: IR-878. PR: agent-ix/quire-contract-codegen#450 at 5cb8ab2, non-author review.
The diff tags two existing TC-030 tests with `#[trace("FR-019-AC-4", "TC-030")]` and
`#[trace("FR-019-AC-7", "TC-030")]`, adds a typed-code assertion to each, changes the
vocabulary test's item kind to `RequestedKind::Unknown`, and splits the FR-019 row of
`spec/routed/matrix/tests.md`. Both tags use `#[trace]` only; no `/// Trace:` line is added.

Gates run in a throwaway worktree (own `CARGO_TARGET_DIR`, `--jobs 2`, `nice -n 10`, under
`flock /tmp/luna-ir-heavy-3.lock`): `make fmt-check lint` passed (clippy `--all-targets -D
warnings`); `cargo test --locked --test it capability_settlement` with `QSPEC_DIR` set to a
quire-specification worktree at `origin/main` a2230e1: 43 passed, 0 failed.

Mutation checks on `src/routed/capability.rs`, each run against the `capability_settlement`
filter and reverted:

| Mutant | Result |
| --- | --- |
| Vocabulary check skipped when the value is empty | killed |
| Vocabulary check removed | killed |
| Refusal cause `unknown-vocabulary` in place of `unsupported-version` | killed |
| Serialized refusal code renamed to `unsupported_projection` | killed |
| Vocabulary accepted by `starts_with(CAPABILITY_VOCABULARY)` | survived |
| Vocabulary accepted by `trim().eq_ignore_ascii_case(..)` | survived |
| Ambiguous candidates reversed | killed |
| Ambiguous candidates listed in manifest (registration) order | killed |
| Ambiguity broken by settling the first registered candidate | killed |
| `Cause::AmbiguousBackend` coded `unsupported_projection` | killed |
| Ambiguous candidates sorted bytewise by identity | survived |
| Ambiguity only when exactly two candidates (`many.len() == 2`) | survived |

Moving the vocabulary check after item settlement is an equivalent mutant: the function is pure
and returns only the refusal, so "none of its kinds read" is observable only as the absence of
any settlement, which the test asserts.

## Verdict

Both tags are bound to the right criterion and each test fails for the main wrong behaviours. The
AC-4 test covers both registration orders of its two-backend manifest, which is every order of
that manifest, and asserts the cause, the candidate list and the `invalid_capability` code. The
AC-7 test covers the absent case (the empty string, which is how the typed envelope and
`EnvelopeRefusal::received` represent an absent member) and a foreign version, and asserts the
whole refusal and its serialized code. The new `Unknown` item kind is a good change: a kind
settlement that ran first would now settle an unknown-kind item rather than pass silently.

One medium gap: AC-7 says "not exactly", and the test only covers values far from the selected
identity, so a prefix or case-folding comparison survives. One low gap on AC-4's breadth, and one
stale Test Case Summary row. Not mergeable as-is: FND-001 is a small test addition in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-7 refuses any `capability_vocabulary` that is "not exactly" `quire.capability-kind/v1`, but the tagged test supplies only `""` and `quire.capability-kind/v2`. Mutants that accept by `starts_with(CAPABILITY_VOCABULARY)` or by `trim().eq_ignore_ascii_case(..)` pass all 43 capability_settlement tests. Add near-miss values such as `quire.capability-kind/v1 `, `QUIRE.CAPABILITY-KIND/V1`, `quire.capability-kind/v10` and `quire.capability-kind/v1.0` to the loop | tests/it/capability_settlement.rs:2328, src/routed/capability.rs:1294 |
| FND-002 | low | The FR-019-AC-4 test uses one two-candidate set already in bytewise order. Mutants that sort the candidates by identity, or that settle ambiguity only when there are exactly two candidates, survive. A three-backend manifest over all six registration orders, with the candidates in an order that is not bytewise, would back "more than one candidate" and "in candidate order" in full | tests/it/capability_settlement.rs:1046-1085, src/routed/capability.rs:1443-1447 |
| FND-003 | low | The PR marks FR-019-AC-4 and FR-019-AC-7 `✅ Covered` but leaves the TC-030 row of the Test Case Summary saying "generic tests have no criterion-level tags", which is no longer true. The summary defers to the coverage tables, so nothing is wrong in effect, but the row now contradicts the row above it | spec/routed/matrix/tests.md:55 |

## Dispositions

- FND-001 — fixed: added exact-identity near misses and killed the prefix and trim/case-fold mutants.
- FND-002 — fixed: added three candidates in non-bytewise order across all six manifest orders and killed the sorting and exactly-two mutants.
- FND-003 — fixed: corrected the TC-030 Test Case Summary wording.
