---
id: "SR-13101"
title: "CG PR 450 gap analysis: FR-019-AC-4 and FR-019-AC-7 backing, the two criteria left untagged, and the strict-coverage delta"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5cb8ab212e360682ceb695637086b5bf2705f7b8; spec/routed/functional/FR-019-capability-settlement.md (AC-4, AC-7, AC-10), spec/core/functional/FR-039-contract-package-consumption.md (AC-5), spec/routed/matrix/tests.md, tests/it/capability_settlement.rs, src/routed/capability.rs, src/routed/generate.rs (diff origin/main...HEAD, base 47f9abf)"
---

# SR-13101: CG PR 450 gap analysis

## Summary

Ticket: IR-878. PR: agent-ix/quire-contract-codegen#450 at 5cb8ab2. Plan completion: not
assessed. Tools: quire 0.36.2 (engine 0.50.2), quoin 0.28.3.

Criteria against tests and code:

- FR-019-AC-4. Backed by `tc_030_two_candidates_with_no_named_backend_settle_ambiguous`. The
  code settles `many if item.named_backend.is_none()` to `AmbiguousBackend { candidates:
  many.to_vec() }` from the request's own candidate list, and never reads manifest order on
  that path. Order-dependent mutants are killed. Breadth gap recorded once in SR-13100 FND-002.
- FR-019-AC-7. Backed by `tc_030_a_foreign_capability_vocabulary_refuses_the_carrier`. The
  check is the second statement of `negotiate_backend_provider`, before any item is read.
  "Exactly" gap recorded once in SR-13100 FND-001.
- FR-019-AC-10, left untagged by the author. See FND-001.
- FR-039-AC-5, left untagged by the author. See FND-002.

Strict coverage, measured with `quire coverage --strict --json` on `origin/main` (47f9abf) and
on the PR head:

- `totals.backed` 546 to 548 of 906. The only changed group is FR-019's
  `acceptance-criterion` group, 26 to 28 of 42. `quire matrix` agrees: FR-019-AC-4 and
  FR-019-AC-7 go from `untagged` to `tagged`, and nothing else changes (tagged 479 to 481,
  untagged 309 to 307, tagged-by-ignored-test 18, method-without-symbol 48).
- `unbacked_rows` 229 to 229, `status_lies` 0 to 0. The set of unbacked rows is identical.
  This footer counts rows of the TestMatrix documents (`functional-coverage` rows of
  `spec/*/matrix/tests.md`), not criteria. A row is backed when any id in it has a tagged
  symbol. The old FR-019 row and both rows it is split into carry `TC-030`, and many tests are
  tagged `TC-030`, so the row was already backed on `main`. That is the sense in which quire
  "already counted" AC-4 and AC-7 through TC-030: at row level only. At criterion level they
  were untagged until this PR.
- Nothing regressed. The delta (+2 backed criteria, footer unchanged, 0 contradicted) is the
  correct and complete measurable effect of tagging two criteria whose matrix row already
  carried a tagged TC id. It is acceptable for this PR.

## Verdict

The two tags close two real criterion-level gaps, and the coverage numbers move exactly as they
should. The author was right to leave FR-019-AC-10 and FR-039-AC-5 untagged, but for different
reasons. FR-039-AC-5 is a real owner-boundary and verification-method defect. FR-019-AC-10 is
not only an upstream concern: half of it is testable in CG today, and CG fails it. A tag on it
would be false. Both are pre-existing and neither blocks this PR. Each needs its own follow-up.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-10 says "a registry repeating a backend identity routes nothing rather than choosing between its entries". This is testable in CG, and CG chooses. `descriptor()` takes the first manifest entry whose identity matches (`.find`). A probe gave a manifest two linked `kani` descriptors (bounded-only, unbounded) and one `kani` candidate with an unbounded extent. It settled `Unsupported(UnboundedExtent)` in one registration order and `Supported` in the other. QSL ADR-029 PV-1 withdraws duplicates upstream, but `negotiate_backend_provider` is public and accepts such an envelope. CG should refuse a repeated identity, or the spec should restate AC-10 as a QSL precondition. The first half ("only an item settled supported routes") is partly testable through `route_records` joining harnesses to `Supported` records only. Pre-existing, not introduced by this PR, and correctly left untagged | src/routed/capability.rs:1506-1513, spec/routed/functional/FR-019-capability-settlement.md:397 |
| FND-002 | low | FR-039-AC-5 is a type-level and upstream property, not a CG behaviour a CG test can observe. Every CG consumer takes an admitted `ContractPackageV1`, so a malformed or noncanonical wire "reaches no CG consumer" by signature. The typed refusal it must "preserve" is produced by `ContractPackageWireV1::read` in the model crate, and CG has no API that carries it. Its verification `Test` cannot be met by a CG test that is not really testing the reader. Re-verify it as Analysis over the consumer signatures, or move the reader half to the reader's owner. Pre-existing; correctly left untagged | spec/core/functional/FR-039-contract-package-consumption.md:188, src/routed/capability.rs:1349-1353 |

## Dispositions

- FND-001 — deferred: IR-887 owns repeated backend identity handling; this pre-existing gap remains untagged.
- FND-002 — deferred: IR-888 owns the FR-039-AC-5 reader/consumer boundary; this pre-existing gap remains untagged.
