---
id: SR-15414
title: "IR-951 PR 521 CG gate acceptance gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@a78828a721be4f16750dd8663f5a9a2e50c95181; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md; spec/kani/matrix/TC-045-real-kani-lane-gate.md; Makefile; src/bin/cg_gates.rs; tests/it/kani_gate.rs; NFR-006-AC-1,4,5,6,7,8,9,10,11,12,14,18,19"
review_set: subset
---

## Summary

Ticket: IR-951. Diff-scoped acceptance chain for CG PR 521 at exact head a78828a721be4f16750dd8663f5a9a2e50c95181. The Rust gate controls have behavioral tests, including the three newly failing red cases before implementation. The existing lane-selection test hardcodes six filter names while the Make authority currently has eight, so it does not directly check the full current filter list.

## Verdict

**CONDITIONAL** — make the filter expectation read or explicitly enumerate all current Make filters and bind the new AC-1 negative case to its criterion. The one-run real Kani and final-head aggregate gates remain pending with the author; this review does not claim installed-backend proof.

## Coverage

- NFR-006-AC-1,2,3,14 (examined): scope glob membership, rename directions and stale-main refusal have focused tests; the newly added AC-1 red case needs its direct criterion tag.
- NFR-006-AC-4,11,18 (examined): Make's shared test command and the include closure are checked, but the hardcoded six-name expectation omits `kani_obligations_symbolic_domains` and `kani_stdout_capture` as distinct filters.
- NFR-006-AC-5,6,7,8,9,10,19 (examined): tests exercise lock, counts, launcher, changed tree and fake proof refusal; installed Kani remains reserved for the final gate.
- NFR-006-AC-12 (examined): the changed Makefile does not add the real Kani gate to `ci`.
- Plan completion: not assessed. Optional repository-wide semantic review: not requested. This is a PR-diff audit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The hardcoded lane filter assertion checks six names while `KANI_FILTERS` has eight. Removal of either omitted token is not caught by that assertion, so the shared lane selection test does not verify the complete current command. | tests/it/kani_gate.rs:579 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `EXPECTED_KANI_FILTERS` names all eight Make tokens; the dry-run test compares the complete suffix after `--test-threads=1` for exact equality, and the lane selection walk reuses that list. |

## Disposition verdict

**PASS** at exact head 7b1492dee014e41c0796de0fad5bb8f3776c2343 for this diff-scoped acceptance chain. Independently ran the exact-command and lane-selection focused tests; each passed 1/1. Installed Kani remains the author's final gate.
