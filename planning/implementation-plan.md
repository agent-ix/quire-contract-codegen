---
id: Plan-001
title: "Contract codegen v0.1 implementation plan"
type: Plan
status: active
---

# Contract codegen v0.1 implementation plan

## Dependency DAG

```text
PGM-01 ---------------------> foundation (#1)
IR schema/corpus (#10) -----/       |
runtime helpers (#3 / PR #5) -------+-> oracles/manifests (#4)
                                         |--> tri-state/proptest (#3) --+
                                         |--> Kani/proof graph (#2) ----+-> CLI/parity (#6)
                                         +--> vacuity/coverage (#5) ----+
```

## Foundation plan bundle

1. Validate the requirements, interface, test matrix, and five assurance artifacts with pinned Quire
   and engineering-assurance modules.
2. Retain foundation CI, license, branch-protection, provenance, and draft-dependency evidence.
3. Keep semantic children in Backlog while PGM-01, IR corpus, and runtime are draft.
4. Rebase or rebranch onto reviewed upstream revisions; reconcile schemas, names, evidence envelopes,
   feature contracts, and expected diagnostics before moving issue #4 into Specify.
5. Implement one dependency-ready child at a time with requirement-tagged tests and plan deltas.
6. Close with reproducibility, compile, strategy, Kani, coverage, differential, parity, and gap evidence.
7. Present the exact source/dependency candidate to an independent reviewer and human release owner;
   do not publish to crates.io.
