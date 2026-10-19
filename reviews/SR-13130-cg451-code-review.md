---
id: "SR-13130"
title: "CG PR 451 code review (Rust lane): repeated backend identity refusal"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@e2703b15779aac1585db8d4928c666a3d928d63b; src/routed/capability.rs, tests/it/capability_settlement.rs, spec/routed/matrix/tests.md, reviews/26-10-19-ir-887-repeated-backend-identity.md (diff origin/main...HEAD, base 87db2f7e); context: spec/routed/functional/FR-019-capability-settlement.md, quire-specification@a2230e13 FR-290, FR-272, FR-331, TC-282; quire-driver origin/main src/refusal.rs"
---

# SR-13130: CG PR 451 code review (Rust lane)

## Summary

Ticket: IR-887 (parent IR-878). PR: agent-ix/quire-contract-codegen#451 at e2703b15. Reviewer is
not the author. The code-review and rust-review methods share this one artifact.

The PR adds a check to `negotiate_backend_provider`. Before any item settles, it refuses an
envelope whose manifest repeats an exact backend identity. The refusal is
`EnvelopeRefusal::InvalidCapability { cause: "duplicate-backend", member: "manifest", received:
<smallest repeated identity> }`. Three TC-030 tests are added, one of them tagged FR-019-AC-10.

What the review checked:

- FR-019, read in full. Behavior lines 116-192, Outputs lines 98-109, the IR-633 origin section
  and AC-1 to AC-44. FR-019 names no refusal, cause or locus for a repeated manifest identity.
  The only envelope-level refusal it states is `invalid_capability`/`unsupported-version` for the
  vocabulary (AC-7).
- QSpec FR-290 lines 246-317 and FR-272 line 94. These define `duplicate-backend` as a registration
  cause with three properties. Byte-equal repeats are held once and not refused. The cause refuses
  only that identity and "never refuses a registration of another identity". It retains the
  repeated identity and the refused registration's manifest. QSpec FR-331 line 57 says manifest
  identities are unique, and line 337 says a malformed carrier refuses before consumption, but it
  names no code for this case.
- Identity comparison is exact and bytewise: `BTreeSet<&str>` over `backend.identity`. That
  matches FR-290 "Key: the exact label" and "compared bytewise as UTF-8". A case-folding mutant is
  killed.
- The check runs before item settlement and on every path. `negotiate_contract_package` calls
  `negotiate_backend_provider`, and `descriptor()` has no other caller.
- API shape: no change. `EnvelopeRefusal` keeps its single variant, with a new `cause` string
  value. It is not `#[non_exhaustive]`, but no variant was added. quire-driver (`src/refusal.rs`
  on origin/main) formats `invalid_capability/{cause}` generically, so it still compiles. No
  generated or routed artifacts change.
- Gates, run by the reviewer at the PR head: `make fmt-check` passed. `make lint` (locked clippy,
  `-D warnings`) passed. The `capability_settlement` filter passed 47 of 47 (44 existing tests,
  including FR-019-AC-4 and AC-7 from #450, plus 3 new). The full `it` binary had 614 passed and
  2 failed. The 2 failures (`kani_obligations::cbmc_failure_terminal_value_reads_the_retained_cause`
  and `tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics`) fail identically
  on origin/main. In `--lib`, 5 bounded-Kani tests fail with `MemoryMechanismUnavailable`
  (namespace startup refused), which is the known sandbox limitation in `src/kani/run`.
- Mutation probes, run by the reviewer in a throwaway worktree against the capability_settlement
  filter:
  - M1, revert to first-match with no check: killed (2 tests fail).
  - M2, flag only adjacent duplicates: **survived**.
  - M3, ignore the second manifest entry: killed.
  - M4, run the check after item settlement: survived. Settlement is pure, so the two are
    observationally equivalent; this is not a defect.
  - M5, case-fold identity: killed.
  - M6, report `.max()` instead of `.min()`: **survived**.

## Verdict

**FAIL.** The mechanism is correct for its own definition: exact identity, independent of order,
before settlement, on every entry point. But the refusal it emits is one the CG spec does not name.
It reuses the FR-290 code `duplicate-backend` with a different scope and different semantics
(FND-001). That needs a spec decision before the code is fixed in place. The other findings are
medium and low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The refusal is not one FR-019 names. FR-019 (Behavior, Outputs, AC-10) states no cause or locus for a repeated manifest identity. The PR picks `invalid_capability`/`duplicate-backend` at envelope scope, but FR-290 defines that code with other semantics. FR-290 holds byte-equal repeats once and does not refuse them (FR-290:258-260). It refuses only the repeated identity and "never refuses a registration of another identity" (FR-290:274, 315-316). FR-272:94 says it retains the refused registration's manifest. The PR instead refuses byte-equal descriptors (pinned by `tc_030_identical_repeated_descriptors_refuse_the_envelope`), refuses every item of every identity, and retains no manifest. quire-driver reports CG's refusal as `invalid_capability/duplicate-backend`, the same code as QSL's registration-stage refusal, but with a different scope and payload. FR-019 needs an amendment naming the envelope refusal (cause, member, retained fields, and whether identical repeats refuse), or CG needs to follow FR-290's semantics | src/routed/capability.rs:1302-1315, spec/routed/functional/FR-019-capability-settlement.md:397, tests/it/capability_settlement.rs:1190-1209 |
| FND-002 | medium | `EnvelopeRefusal::InvalidCapability` now carries three distinct refusals in one variant: two members under `unsupported-version`, plus `duplicate-backend`. They are told apart only by the `&'static str` fields `cause` and `member`, so a caller must compare strings to tell a foreign vocabulary from a repeated manifest identity. The field doc "Both members refuse with one cause" is now false. Per the Rust idiom "the variant set is the API", a repeated identity should be its own typed variant, for example `DuplicateBackend { identity }` | src/routed/capability.rs:951-974 |
| FND-003 | medium | The tests fix only two-entry, adjacent duplicates with one repeated identity. Two mutants survive: an adjacent-only check (`windows(2)`) and reporting `.max()` instead of `.min()`. The tests therefore back neither "fires on every registration order" for non-adjacent repeats nor the claimed stable choice when two identities repeat. Fix: add a three-entry manifest with a non-adjacent repeat, and one with two repeated identities, asserting the reported identity | tests/it/capability_settlement.rs:1148-1234, src/routed/capability.rs:1308 |
| FND-004 | low | The comment on the `many` arm with a named backend still says reaching it "needs a manifest that repeats a backend identity, which the registry refuses ... before a request is built". After this PR, CG refuses such a manifest itself, and the arm is reachable only through a repeated candidate entry. The rationale no longer matches the code | src/routed/capability.rs:1464-1468 |
| FND-005 | low | The PR commits the author's own review artifact `reviews/26-10-19-ir-887-repeated-backend-identity.md` (SR-13102). Its scope lists `.github/workflows/ci.yml`, which the diff does not touch. It records PASS "against ... the referenced FR-290 refusal vocabulary" without noting the FR-290 semantic differences in FND-001. Its file name carries an agent-typed date prefix | reviews/26-10-19-ir-887-repeated-backend-identity.md:6-16 |

## Dispositions

Round 1, against head 5c07c891b630056da636f5925de496def23b7f75. The FR-019 amendment (#453, AC-45, AC-46,
AC-47 and the Behavior note) is merged on CG main.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-019 now names the refusal: AC-45, AC-46, Outputs and the Behavior note. It is a CG-owned input-precondition defense distinct from FR-290 registration semantics. The code returns the new typed `EnvelopeRefusal::ManifestRepeatsBackendIdentity { identity }` and no longer uses `InvalidCapability`/`duplicate-backend` (capability.rs:972-976, 1305-1316) |
| FND-002 | fixed | A repeated identity is its own typed variant. `InvalidCapability` again carries only the two `unsupported-version` members, so the doc "Both carrier members refuse with `unsupported-version`" is now true (capability.rs:961) |
| FND-003 | fixed | The new tests cover [A,B,A] non-adjacent repeats, every permutation of [A1,B,A2], and bytewise-first selection among the repeated identities A and B in four orders. The adjacent-only and `.max()` mutants are both killed |
| FND-004 | still-open | The comment at capability.rs:1465-1470 still says reaching the arm "needs a manifest that repeats a backend identity", and in the same sentence says CG refuses such a manifest before settling any item. The arm is in fact reached with a unique manifest and repeated candidate entries (candidates [kani, kani], named kani): `inconsistent_candidates` flags neither. The rationale is still false |
| FND-005 | fixed | `reviews/26-10-19-ir-887-repeated-backend-identity.md` is absent at the head and in `git diff origin/main...HEAD` |

Round 2, against head 29e8f650b36869d4b3769b637247ed0d6e9a0bc1.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-004 | fixed | capability.rs:1465-1468 now says a repeated candidate entry reaches the arm even with a unique manifest. A reviewer probe confirms it: manifest [kani] and candidates [kani, kani] named kani settle `InconsistentCandidates` with both candidates |
