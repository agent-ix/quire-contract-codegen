---
id: SR-17940486709564847047977
title: "IR-805 PR #831 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #831 at 284ced6c, git diff origin/main...HEAD; IR-805 slice C; FR-015-AC-106 (IR-939 domain controls), TC-025; tests/it/kani_graph_domain_private.rs, src/kani/generate/graph.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805 slice C. Plan completion was not assessed. I checked the AC-106 domain-oracle
clarification (FR-015, the "FR-015-AC-106" section) and the TC-025 domain-control paragraph against
the tests, clause by clause.

`quire trace --id FR-015-AC-106` lists 6 new native tests as verifying claims. The 10 installed-Kani
controls are not bound; see SR-17940486709564847047976 FND-001.

I re-ran the installed controls at the head: 16/16 passed, Kani 0.68.0, 110 s of test time.

## Verdict

**PASS with findings (no high).** These TC-025 clauses are met:

- the profile (N=4, a `{0,3}` end, unique and non-unique);
- a 5x4 carrier;
- an expectation checked against the projections before mutation;
- an observer over the emitted ranges, arrays and guards, including capacity rejection without
  out-of-bounds indexing;
- a healthy `Verified` run with the `D and G and count=4` cover;
- degree 0 and degree 3 probes, each with one final cover;
- three mutants, each `Falsified` at the named assertion with a typed witness and natively checked
  `D`/`G`;
- a per-mutant probe with no membership assumption;
- the three concrete boundary witnesses;
- no replay credit for witnesses that exist only in a mutant.

AC-106 correctly stays PARTIAL. The wrong-role clause is stated as NOT evidenced, and the field-edge
form is left to slice D. Two gaps remain: the evidence-retention requirement, and the
carrier-representation evidence (SR-17940486709564847047976 FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-025 requires "Retain source, actual installed-backend evidence, failed property identity, non-vacuity/boundary cover results and native domain-witness checks for each profile and run". The AC-106 clarification says "Every run retains generated source, the observed domain definition, backend property identity and FR-017 execution evidence". The ignored tests run in a `Scratch` dir that is removed on `Drop` and write no artifact. The only record is prose timings in the AC-106 row, and TC-025 and tests.md call slice C "delivered" without naming this gap. Scenario: an auditor asks for the N-1 run's generated source or FR-017 evidence and has nothing but a rerun. Fix: retain the artifacts, or state the gap in the AC-106 status. | tests/it/kani_graph_domain_private.rs:811-834; spec/kani/matrix/TC-025-bounded-kani-obligations.md:304-306 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-015-AC-106 | examined | "Separately, the domain-conformance/completeness oracle defined below verifies the healthy generated domain and falsifies each pure authored-bound mutant: population extent raised or lowered by one and edge upper bound raised by one." |
| FR-015 AC-106 clarification (D, G, carrier, mutants) | examined | "Neither D nor its expected endpoints may be obtained from the generated harness, the mutated bound, or the generator's mutated lowering result." |
| TC-025 domain-control paragraph | examined | "Run three separate mutants against the unchanged original expected domain, candidate carrier and clause evaluation rules" |
| FR-015-AC-99 | context_only | status edit only: "graph proof controls are AC-106 (PARTIAL, IR-805)" |

## Dispositions

Round 1, fix commit 5f550fc3.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The artifacts are still not retained. The gap is now stated plainly and consistently in three places: FR-015-AC-106 ("run artifacts ... are not retained, so only the verdicts, walls and memory above are recorded and each run must be repeated to inspect them"), TC-025 ("that retention is an open gap") and the tests.md row. AC-106 stays PARTIAL, so no credit is overclaimed. Accepted as an honestly stated open gap within the PARTIAL status. |
