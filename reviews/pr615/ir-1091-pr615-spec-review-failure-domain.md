---
id: SR-615003
title: IR-1091 PR 615 unique collection capacity refusal — failure domain
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; IR-1091; PR 615 head 18ebb99edf790cd9546ad2fa8d3329a2bf758fbe vs main 92ebb4e2943a47cc172a4237c82077c8b7da9859; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
---

## Summary

Ticket: IR-1091. Failure-domain review of the new capacity rule in
FR-015-AC-96 at head 18ebb99e. The rule handles the main failure modes
correctly. It names a typed reason with both values and emits no harness, so
there is no silent vacuous `Verified` for an uninhabited
`min > capacity` field. It does not reclassify an admitted field as an
invalid model. It forbids tightening the domain, and it keeps the boundary
`max == capacity` feasible.

Three edge orderings are left open:

- the resource ceilings of FR-015-AC-101
- an empty element interval
- capacity arithmetic at the full `i64` width

## Verdict

PASS WITH FINDINGS. FND-001 is medium because two implementations would
return different typed reasons for the same request. FND-002 and FND-003 are
low precision gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The rule orders the capacity check only against representability ("applies only after the finite endpoints are representable"). It does not order it against FR-015-AC-101's resource ceilings, which `render_collection` checks first: the unwind check (`UnwindBelowCollectionBound`, requires unwind >= max+1) and the memory check (`SymbolicDomainExceedsMemoryCeiling`). Example: a unique Boolean field with max 8 and unwind 4 could settle with either reason. TC-025's "instead of the capacity reason" implies resource ceilings win, but AC-96 does not say so. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846; src/kani/generate/collection.rs:242-285 |
| FND-002 | low | For an empty `IntRange` element interval (lower > upper), the formula `upper - lower + 1` gives a capacity of 0 or less. The rule does not say that the existing `UnsatisfiableElementDomain` refusal (`CollectionDomainRefusal::UnsatisfiableElementDomain`, raised at lowering) takes precedence over `UniqueElementCapacityExceeded`, and does not limit "representable finite element domain" to a non-empty one. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846; src/kani/generate/collection.rs:83-84 |
| FND-003 | low | With representable `i64` endpoints, the exact capacity `upper - lower + 1` can reach 2^64, which overflows both `i64` and `u64`. The rule says "exact" but does not say the comparison must be overflow-free, for example by saturating, given that `declared_maximum` is a `u64`. A wrapping or panicking implementation would misreport a field that is in fact feasible. The reported `capacity` is always below `declared_maximum`, so it fits in a `u64`. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846; src/kani/generate/collection.rs:256-263 |

## Coverage

Checked these failure domains against AC-96's new text and current code:

- uninhabited unique field (min > capacity): covered
- boundary max == capacity and max == capacity + 1: covered
- Boolean capacity 2; single-point IntRange capacity 1 by formula: covered
- representability before capacity: covered
- resource-ceiling order: FND-001
- empty interval: FND-002
- arithmetic width: FND-003
- identity confusion with malformed or invalid package: covered by "not
  reclassified"
- vacuous proof or harness emission: covered

Purity and determinism are unaffected: the refusal emits no artifact and the
AC-100 identity is untouched.

## Dispositions

Round 1, exact fix head 00c799c99c82626ab55109d7e6750baaba5f4d73 (fix delta 18ebb99e..00c799c9; merge-base with main 92ebb4e2). Checked against the fix diff and the current spec/code context; `quire validate --scope . "spec/**/*.md"` exits 0. The PR remains a draft for the owner decision; AC-96 is not complete.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-96 now states that FR-015-AC-101 representational, unwind and memory refusals precede the capacity reason in their existing order. This matches `render_collection` (src/kani/generate/collection.rs:239-285), and TC-025:94-103 requires `UnwindBelowCollectionBound` and `SymbolicDomainExceedsMemoryCeiling` for the paired cases. |
| FND-002 | fixed | The rule now requires a nonempty domain and gives `UnsatisfiableElementDomain` precedence; TC-025:96-99 adds an empty `IntRange` with positive maximum as its own case. |
| FND-003 | fixed | AC-96 now requires overflow-free, non-saturating comparison wide enough for the 2^64-value full interval and explains why a reported capacity fits u64; TC-025:84-92 covers the full `i64::MIN..=i64::MAX` interval. |
