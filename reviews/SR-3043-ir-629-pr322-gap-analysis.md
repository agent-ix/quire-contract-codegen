---
id: "SR-3043"
title: "IR-629 PR 322 gap analysis: process domain-kind truth table and matrix"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 322, branch spec/ir-629-fr290-domain-kind (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md"
---

# SR-3043: IR-629 PR 322 gap analysis

## Summary

Ticket: IR-629. This analysis is planless, so plan completion was not assessed. The change is
spec only, and the gate set for it is make spec plus spec review. I ran no cargo, tests or
Kani.

Computed matrix (`quire matrix --format tsv`, base main against the PR head):

- Both runs have 589 records.
- Only the text of FR-019-AC-12, AC-17, AC-21 and AC-22 changed. All four were and remain
  `untagged` (PLANNED).
- No criterion gained or lost a binder.
- `--strict` exits 1 on both base and head, so this PR does not make it worse.

Truth table, unbounded item on a `bounded`-only descriptor. ADR-014 makes
`finite_bound_available` true exactly when every domain kind is boundable, so a cell is
reachable only if its flag agrees with its kinds. The reachable cells:

- every boundable kind advertised, no non-boundable kind, flag true: `requires-bound` (row 53)
- some non-boundable kind and every boundable kind advertised, flag false: `unbounded-extent`
  (row 54)
- an unadvertised boundable kind, flag true: `unsupported-requested-capability` (row 55)
- an unadvertised boundable kind with a non-boundable kind, flag false:
  `unsupported-requested-capability` (row 56)
- only non-boundable kinds, flag false: `unbounded-extent`, with no row

The unbounded items on `unbounded` and on both-mode advertisements (rows 51-52) and the nine
bounded rows (42-50) are unchanged. The previous unreachable row has been removed (flag false,
every domain advertised, no non-boundable kind). So has the row applying the check to
non-boundable kinds.

Seeded mutants checked against the rows:

- comparing a non-boundable kind is caught by row 54
- letting the flag mask an unadvertised kind is caught by row 56
- skipping the unbounded domain check is caught by row 55
- applying the check under an `unbounded` advertisement is caught by row 52

## Verdict

The rewritten rows cover the rule's decision points, apart from one reachable cell: an
unbounded item whose domains are all non-boundable. That is the ordinary shape of an
infinite-trace or quantity claim on a `bounded`-only provider, and no row exercises it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-046 has no row for an unbounded item whose `extent.domains` hold only non-boundable kinds (for example only `infinite-trace` or only `quantity`) on a `bounded`-only descriptor. The rule settles it `unbounded-extent`, because the set of compared kinds is empty. Row 54 always includes `integer`, so a mutant that treats an empty compared set as a domain-check failure (`unsupported-requested-capability`) survives every row. | spec/routed/matrix/TC-046-process-provider-settlement.md:53-56 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | One admitted cell has neither a stated outcome nor a row: an unbounded-only descriptor that omits domains (FR-331-AC-22 admits it) with a bounded item whose bounds are non-empty. FR-019 compares bounds kinds "with manifest domains" but never says that absent domains hold no kind, and row 50 presumes domains are present. An arm that skips the check when domains are absent then settles the unnamed mode decline instead of the FR-290-AC-13 decline that names the kind, and no row catches it. | spec/routed/functional/FR-019-capability-settlement.md:145-152; spec/routed/matrix/TC-046-process-provider-settlement.md:49-50 |
| FND-003 | low | TC-046 rows 44 and 49 (the bounded mode decline on an unbounded-only advertisement) expect unsupported without "warned", while FR-019 lines 153-156 require that decline to be warned. A mutant that drops the warning on that path passes every row. | spec/routed/matrix/TC-046-process-provider-settlement.md:44, 49; spec/routed/functional/FR-019-capability-settlement.md:153-156 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker. The computed matrix still has 589 records. Only the text of FR-019-AC-12, AC-17, AC-19, AC-21 and AC-22 differs from main, all five stay untagged, and no binder was lost. --strict exits 1, as it does on main.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): TC-046 row 55 covers an unbounded item with only quantity or only infinite-trace, finite_bound_available=false, on a bounded-only descriptor, expecting unbounded-extent, and the mutant table lists "Reject an empty compared-kind set". |
