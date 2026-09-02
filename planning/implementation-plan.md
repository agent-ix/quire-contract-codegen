---
id: REV-005
title: "Contract codegen v0.1 plan-bundle migration"
type: Review
---

# Contract codegen v0.1 plan-bundle migration

The mechanically checkable plan is retained at `plan/PLAN-001-codegen-v01/plan.md`. Its task files
separate completed foundation work, dependency reconciliation in progress, semantic work that must
not start without the IR candidate, and the human-owned release decision.

## Dependency DAG

```text
PGM-01 ---------------------> foundation (#1)
IR schema/corpus (#10) -----/       |
runtime helpers (#3 / PR #5) -------+-> oracles/manifests (#4)
                                         |--> tri-state/proptest (#3) --+
                                         |--> Kani/proof graph (#2) ----+-> CLI/parity (#6)
                                         +--> vacuity/coverage (#5) ----+
```

## Historical step mapping

1. Validate the requirements, interface, test matrix, and five assurance artifacts with pinned Quire
   and engineering-assurance modules.
2. Retain foundation CI, license, branch-protection, provenance, and draft-dependency evidence.
3. Keep semantic children in Backlog while the IR corpus candidate and runtime release decision are
   under review.
4. Rebase or rebranch onto reviewed upstream revisions; reconcile schemas, names, evidence envelopes,
   feature contracts, and expected diagnostics before moving issue #4 into Specify.
5. Implement one dependency-ready child at a time with requirement-tagged tests and plan deltas.
6. Close with reproducibility, compile, strategy, Kani, coverage, differential, parity, and gap evidence.
7. Present the exact source/dependency candidate to an independent reviewer and human release owner;
   do not publish to crates.io.
