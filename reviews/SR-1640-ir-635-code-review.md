---
id: SR-1640
title: "IR-635 code-review review: composite parity replay binding spec"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1640: IR-635 code-review review

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. The diff changes six Markdown files and no code, tests, schemas, scripts or CI workflows. It adds no vendored file, compatibility layer, hash, pin or checksum catalogue. The only identity it names is the canonical proof-content identity that binds a proof to the content it proved, which the repository CLAUDE.md permits. One low layout finding.

## Method

I read `git diff origin/main...HEAD` in full and both new files end to end. I checked the diff for code or test changes, vendored or copied upstream artifacts, new tracking pins, compatibility or fallback wording, and Markdown layout against the surrounding files. Correctness of the requirement content is covered by the spec-review artifacts SR-1641 to SR-1646.

## Verdict

**PASS with one low finding.** Clean: no source, test or workflow edits; no vendored schema or binary; no new pin, digest or SHA; "No compatibility layer is specified" (FR-029 Implementation Gate) and "no ... compatibility fallback" (FR-033) are explicit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Added prose does not follow the files' existing 100-column wrapping. FR-033 lines 114-117 break mid-sentence ("original limits and canonical" then "proved-content identity"), and several added prose lines run past 100 columns as single unwrapped lines: FR-033 lines 49, 66, 78 and 151, FR-029 lines 60, 81 and 242, FR-028 lines 298, 313, 357 and 358, and TC-048 line 73. The files already hold some over-width lines, so this is a layout nit, not a gate. Reflow the added lines to the surrounding width. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:114, spec/kani/functional/FR-029-run-outcome-terminal-record.md:60, spec/kani/functional/FR-028-bounded-proof-ceilings.md:357, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:73 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The fix round reflowed text by splitting tokens at line ends. Two Markdown link targets are now broken across a newline: FR-029 line 259 `[AD-003](../../assurance/AD-003-evidence-` with `chain.md)` on the next line, and TC-048 line 19 `[FR-033](../functional/FR-033-composite-parity-replay-` with `binding.md)` on the next. A link destination cannot contain a line break, so both render as plain text and the cross-reference is lost. Hyphenated words are split the same way (FR-029 lines 193, 220 and 221; FR-033 lines 47 and 112), so "missing-cover- summary", "out-of- range", "node- selected" and "decoded- size" render with a stray space. Rejoin each link target on one line and wrap at whitespace rather than inside a token. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:259, spec/replay/matrix/TC-048-composite-parity-replay-binding.md:19, spec/kani/functional/FR-029-run-outcome-terminal-record.md:193, spec/replay/functional/FR-033-composite-parity-replay-binding.md:47 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #298, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | The mid-sentence break at FR-033 lines 114-117 is fixed: the sentence now reads "limits and canonical proved-content identity". But most of the added over-width prose lines the finding lists are still single lines past 100 columns: FR-033 lines 69, 81 and 155; FR-029 lines 60 and 82; FR-028 lines 297, 312, 356 and 357; and TC-048 line 75. The fix also adds a new one at TC-048 line 69 (123 columns). Reflow these lines; FND-002 records the separate token-splitting regression. |
