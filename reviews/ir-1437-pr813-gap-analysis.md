---
id: SR-17940486709564847047754
title: "Gap analysis — CG PR 813 FR-055 kani-mutations target"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-055-kani-mutations-target.md, spec/kani/matrix/TC-069-kani-mutations-target.md, scripts/kani_mutations.rs, scripts/kani-mutants.json, Makefile, tests/it/kani_mutations.rs; ticket IR-1437"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-055", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-069", type: references }
---

## Summary

Planless gap analysis, scoped to the diff, of the FR-055 chain from requirement to test to code. Plan completion: not assessed. Ticket: IR-1437. The ticket says the target is part of `make ci`; FR-055 (authoritative) says it is not, and the code follows FR-055.

## Verdict

**GAPS.** FR-055-AC-2 to AC-6, AC-9 and AC-10 each have a tagged test that can fail. Those tests use a stand-in runner that also confirms the replacement reached the scratch copy. AC-3 is backed by `make -n ci` and AC-8 by an inspection-style test. Two gaps remain. AC-1 is tagged only on a stand-in test, and code-review FND-001 and FND-002 show the real half cannot pass. AC-7, the ticket's only Verification item, has no test. The FR-055 Status line still says IMPLEMENTED.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-055-AC-1 ("runs the owning proof through Kani, and exits zero only when every mutant is `pass`") is tagged on a stand-in test whose runner prints the marker itself. It cannot catch the real gap: the marker never reaches the owning proofs' output, and mutant 3 is test-only code. The Status line ("IMPLEMENTED") overstates a target that cannot exit 0 on the shipped declaration | tests/it/kani_mutations.rs:127-156, spec/kani/functional/FR-055-kani-mutations-target.md:130-135 |
| FND-002 | medium | FR-055-AC-7 (a weakened harness makes its mutant `fail`; restoring returns 0) has no test and no recorded run. It is the ticket's only Verification criterion. The Status line discloses this, but the PR ships the target without that evidence | spec/kani/functional/FR-055-kani-mutations-target.md:116 |

## Coverage

- Reconciliation: trace tags read directly from tests/it/kani_mutations.rs. FR-055-AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-8, AC-9 and AC-10 are tagged; AC-7 is untagged.
- Plan completion: not assessed
- Untraced behaviour in the diff: none. Every branch of scripts/kani_mutations.rs maps to an FR-055 outcome rule.
- Stubs or coverage inflation: none, apart from AC-1's tag on stand-in coverage (FND-001).
