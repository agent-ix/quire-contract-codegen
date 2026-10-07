---
id: "SR-3040"
title: "IR-629 PR 322 spec review: FR-290-AC-13 domain-kind rule follow-up"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 322, branch spec/ir-629-fr290-domain-kind (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md"
---

# SR-3040: IR-629 PR 322 spec review (base)

## Summary

Ticket: IR-629. Spec-only change to three Markdown files in one commit over CG main.

I re-read these sources by path on their main branches instead of trusting the brief or the
ticket text:

- QSpec `spec/objects/protocol/FR-290-protocol-claim-kind.md`: "Advertised mode", the
  "Single-candidate arm" steps 1 to 3, the advertised-mode table and FR-290-AC-13.
- QSpec `spec/test-cases/TC-271-capability-kind-candidate-negotiation.md`, rows NG-05, NG-06
  and NG-16 to NG-20.
- QSpec `spec/objects/interfaces/FR-331-backend-provider-envelope.md`, FR-331-AC-21 and AC-22.
- QSpec `proposals/backend-provider-v1/schema.json`: `ProofBound` (requires `kind` and
  admits only boundable kinds), `Extent` (the unbounded arm carries `domains[].kind` over all
  seven `DomainKind` values plus `finite_bound_available`), and `BackendDescriptor.domains`.
- QSL `spec/decisions/ADR-014-temporal-trace-and-boundedness-architecture.md` section 4 (the
  boundable table and the "Available finite bound" paragraph) and
  `spec/decisions/ADR-029-lifecycle-cli-provider-plugin-cache-codegen-boundaries.md` PV-4.
- Linear QSL-654, read as data.

What the change gets right:

- The routing rule matches QSpec FR-290 step 2 and AC-13. On a `bounded`-only candidate, CG
  compares only the boundable kinds of an unbounded item. An unadvertised boundable kind
  settles `unsupported-requested-capability` whatever `finite_bound_available` says. Otherwise
  the mode table decides, giving `requires-bound` or `unbounded-extent`. An `unbounded`
  advertisement skips the comparison.
- The boundable set {collection, population, integer, recursive} matches ADR-014 section 4 and
  the schema.
- The stale merged text is gone. The FR-019 rule that every `extent.domains[].kind` must be
  advertised has been replaced, AC-17, AC-21 and AC-22 have been rewritten, and the old TC-046
  rows have been replaced. One of those rows was unreachable: `finite_bound_available=false`
  with every domain advertised and no non-boundable kind, a combination ADR-014 rules out.
- No numeric comparison and no missing-`domains` fallback appear anywhere.
- No SHAs, local paths or conflict markers appear.

Units examined: FR-019 Inputs (extent reading), the bounded and unbounded process-provider
bullets, the admission bullet, follow-on decision 1, FR-019-AC-12, AC-16, AC-17, AC-21, AC-22,
Dependencies, the TC-046 Description, step 2 rows, step 3 admission note, Expected Results 2,
seeded mutants and Status, and the routed matrix row for FR-019.

## Verdict

Not merge-ready on the spec-only gate yet. The rule itself is faithful to QSpec FR-290-AC-13.
The remaining defects are about ownership and attribution: CG restates admission text that QSL
owns, and that text already disagrees with the AC it cites. CG also credits FR-290-AC-13 with
a disposition that FR-290 leaves to the arm. And the claim that no bounded-capable descriptor
without `domains` reaches CG is gated only on the producer, not on the QSL admit change that
makes it true.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019 and TC-046 restate the QSL/QSpec registration-refusal rule (absent, empty, non-boundable or repeated `domains` refused as `invalid_capability`/`invalid-domains`, keyed by backend identity) instead of only stating CG's consumption. The copy already drifts from the cited owner: FR-290-AC-13 refuses only a registration that advertises a `bounded` pair, while CG's text, following the FR-290 prose and FR-331-AC-22, refuses any registration with bad `domains`. | spec/routed/functional/FR-019-capability-settlement.md:164-169; spec/routed/matrix/TC-046-process-provider-settlement.md:65-69 |
| FND-002 | low | FR-019-AC-12 cites QSpec FR-290-AC-13 for its whole "Otherwise" clause. That clause includes a bounded item on an `unbounded`-only descriptor settling `unsupported-requested-capability`, which FR-290's advertised-mode table leaves to "the negotiation arm's own disposition". AC-13 does not mandate it. | spec/routed/functional/FR-019-capability-settlement.md:257 |
| FND-003 | low | FR-019 says the process arm "receives no such bounded-capable descriptor". That holds only once QSL admit enforces `invalid-domains`. The status rows gate the PLANNED criteria only on QSL-654's producer of `ProofBound.kind`, not on the admit-side check. | spec/routed/functional/FR-019-capability-settlement.md:168-169; spec/routed/matrix/TC-046-process-provider-settlement.md:124-126; spec/routed/matrix/tests.md:20 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker. QSpec FR-290 and FR-331 on main were re-read and are unchanged since the review pass.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): CG no longer states an admission rule. FR-019 lines 178-182 and TC-046 step 3 keep only "QSL supplies admitted descriptors (QSpec FR-290 'Advertised mode'; FR-331-AC-22)", no missing-domains fallback, and QSL-654 owning admit-side enforcement. |
| FND-002 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): FR-019-AC-12 cites FR-290-AC-13 only for the uncovered-kind decline. The mode decline is CG's own and carries no owner citation. |
| FND-003 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): AC-12, the TC-046 Status, the routed matrix row and FR-019 Dependencies all gate on QSL-654's admit-side invalid-domains enforcement as well as its producer. |
