---
id: SR-15415
title: "IR-951 PR 521 TC-045 base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@a78828a721be4f16750dd8663f5a9a2e50c95181; spec/kani/matrix/TC-045-real-kani-lane-gate.md; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md; Makefile; tests/it/kani_gate.rs; NFR-006-AC-4,5,6,7,11; TC-045"
review_set: subset
---

## Summary

Ticket: IR-951. Reviewed the changed TC-045 procedure and its NFR-006 links at exact PR head a78828a721be4f16750dd8663f5a9a2e50c95181. The edited implementation pointer to `cg_gates` resolves and the document validates. Two existing procedure claims remain inconsistent with the current Make command and the gate's trusted-Cargo rule.

## Verdict

**CONDITIONAL** — correct the filter count and the stand-in success oracle in TC-045 so its procedure can be run as written against the Rust gate. The changed requirement IDs and relationships themselves remain intact.

## Coverage

- TC-045 steps 1-3 and 8-13 (examined): scope cases, rename handling, stale main, tree checks, selection, CI exclusion and inspection remain stated and linked to NFR-006.
- TC-045 steps 4-5 (examined): text says six filters but Make's `KANI_FILTERS` has eight.
- TC-045 steps 6-7 (examined): text expects a fake Cargo run with 27/27 to return `passed`, whereas the real gate requires the trusted Cargo path and the existing test correctly asserts stand-ins fail.
- NFR-006-AC-4,5,6,7,11 (examined): the criteria describe shared command, lock, count, result fields and selection; the test case should reflect the current command and identity prerequisite.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-045 step 6 expects a stand-in Cargo listing and passing 27/27 to produce `result=passed` and exit 0. Both the shell gate and Rust gate refuse a stand-in regardless of its printed count, and the test asserts that refusal; the positive oracle in this procedure is impossible. | spec/kani/matrix/TC-045-real-kani-lane-gate.md:44 |
| FND-002 | medium | TC-045 steps 4-5 say to verify six lane filters, but `KANI_FILTERS` currently has eight. Following this procedure can miss the two added filters. | spec/kani/matrix/TC-045-real-kani-lane-gate.md:35 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-045 steps 6-7 now state that a stand-in with matching nonzero counts still fails trusted-Cargo identity and point to the installed-Kani step 13 for the positive oracle. |
| FND-002 | fixed | TC-045 steps 4, 5 and 11 now refer to all eight current filters, matching Make and the test's exact token list. |

## Disposition verdict

**PASS** at exact head 7b1492dee014e41c0796de0fad5bb8f3776c2343. The changed TC-045 file validates under Quire.
