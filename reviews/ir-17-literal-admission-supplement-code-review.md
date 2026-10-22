---
id: SR-15512
title: "IR-17 literal admission correction code-review supplement"
type: SpecReview
analysis: code-review
scope: "CG #527 FR-014-AC-53 admission sentence and TC-024 literal boundary/provenance correction; original SR-15427 FND-002 as context"
review_set: subset
---
# SR-15512: IR-17 literal admission correction supplement

## Summary

Reviewed the exact frozen correction distinguishing owner admission from runtime canonical integer parsing. The corrective specification resolves the original public-reader counterexample without changing constructor eligibility, existing refusal behavior or proof semantics.

## Verdict

**PASS** — no new finding in this bounded corrective specification. Original SR-15427 FND-002 is fixed by the inspected correction; no literal implementation or memory-fit credit follows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

The patch removes an incorrect universal reader-refusal assumption rather than adding a workaround to executable code. It preserves the original public admitted input and existing canonical parser/typed stop. No pre-filter, spelling normalization, fixture-only constructor exception, local owner copy, source-hash/scan feature or weakening of the original proof is introduced. TC-024 now names source-emitted and independently owner-admitted/rekeyed input routes separately, avoiding a fake claim that QSL accepts every substituted spelling/value. Scope is spec only; there is no executable Rust delta to test or production source gap closure to credit.

## Review boundary and preserved history

Same independent reviewer of #524, now reviewing only corrective spec #527 at published/frozen `b8ead73e114b53d7df37d05203fb0ca9489b16c9`, sole parent/base `534729ab1705d5abcf278f9856c8edd09e6f7893`. The actual delta is only the AC-53 admission sentence and TC-024 admission/fallback/provenance prose. The canonical complete-spelling/i64-fit iff constructor policy, MIN distinction, full mathematical/meters/denials/stops/laziness/BigInt obligations and original public-regenerated #513 proof/one-cover settings remain unchanged. No production source, executable tests, kernel API, lock, FR-015, scalar-bound emitter or gate changed in this PR.

All four original repository SR-15426..15429 files were compared byte-for-byte with original reviewer custody and remain unchanged. The prior PASS decisions and the later SR-15427 FND-002 review escape remain historical evidence. This NEW supplement has a distinct ID and path; it does not overwrite, duplicate the ID of, or retrospectively rewrite the original SR. Original finding FND-002 is closed by its separately posted same-reviewer disposition and the correction inspected here, not by erasing the finding.

## Evidence and actual checks

Inspected actual `literal-admission-evidence.md`, raw `code-red/public-reader-facts.log`, `public-large-intake-2.log` and the corresponding tests-first public fixture/helper source. The eight concrete general-leaf spellings +1, 01, -0, leading-space 1, trailing-space 1, 1a, empty and --1 each report StaleNodeKey under stale literal identity, then Admitted through the existing owner-candidate/rekey/final-reader route and original parse/InvalidConstant generated fallback. Actual IR/model lock is d0737774. These are specific retained author test results, not a new reviewer rerun or a universal claim about all malformed spelling. The facts run reports 9 PASS and 2 expected constructor-policy FAIL on unchanged old production code; it is not an all-green implementation gate.

Those retained general-leaf controls assert the emitted parse/InvalidConstant source route. They do not execute a newly changed runtime path, prove a Kani cover or qualify the future constructor implementation. Kernel canonical parsing rejects the listed noncanonical spellings; checked identity/admission is not canonical runtime parsing. Separately, the tested QSL comparison source accepts positive i128 MAX and refuses positive 2^127 as IllTyped before CG; owner-candidate/reader-admitted beyond-i128 controls establish a different input boundary. The correction carefully keeps those claims separate.

Actual full `quire validate --scope <frozen-worktree> "spec/**/*.md" --summary` exit 0 and `git diff --check <base>...HEAD` exit 0. Only the same three inherited EARS advisories appear (FR-017 line 267 unclassifiable/missing-subject; FR-014 line 269 non-canonical-trigger). No changed requirement-bearing Description/Behavior/Constraints sentence exists, so no new EARS artifact was applicable; full grammar validation still ran. Quire 0.36.2 / engine 0.50.2; Quoin 0.28.4. This supplement validates at its explicit reviewer-custody path.

## Assurance Context

No applicable repository AssuranceProfile or required review_selection was found. Active exceptions for this scoped change: none. Unavailable evidence: no literal production implementation or new compiled evaluation/whole-domain proof is delivered here. AC-53 remains planned/untagged and inherited strict-matrix debt remains open; this review neither reruns nor claims a green matrix. No Rust execution review, Cargo/build, namespace, aggregate, engine, Kani or branch write was performed. This is a spec correction PASS, not implementation, fit, full-gap or merge acceptance.
