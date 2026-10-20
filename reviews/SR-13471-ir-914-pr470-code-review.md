---
id: SR-13471
title: 'code-review review of IR-914 PR #470'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; spec/evidence/functional/FR-004-vacuity-evidence.md, spec/core/functional/interface-001-codegen-api.md, spec/evidence/matrix/TC-006-vacuity.md'
review_set: subset
---

## Summary

Reviewed PR #470 exact head `641b45004da50f318afb2fe387e960a4b0f4f156` against CG AGENTS.md/CLAUDE.md, FR-004, interface-001, TC-006 and FR-011 OQ-2. Ticket: IR-914.

## Examined

- FR-004-AC-10 (examined): PLANNED. CG's campaign runner issues its opaque receipt once inside the producing run while holding the exact decoded QSL result, execution record, generated source, source map, LLVM export, and LLVM tool/version bytes. The receipt retains those exact bytes in memory and over the wire without a digest; CG retains issuance authority independently of forwarded wire data, so decoding alone grants none. Binding first matches the receipt to CG's runner-held issuance state, then compares each association byte for byte with the supplied inputs from that run. The receipt matches requirement and revisi
- FR-004-AC-14 (examined): PLANNED. Only CG's in-run campaign runner can construct the opaque receipt; its fields are private and it has no public constructor. A valid `/2` clause-run result with absent receipt, omitted association, deserialized receipt without runner authority, or only caller-declared paths, QSL `package_id` or self-declared producer metadata yields a structured absent/unauthenticated binding diagnostic retaining the decoded outcome and available identities, with no qualification, classification or discharge. A runner-issued receipt with every exact association admits binding without changing the QSL o
- FR-004-AC-15 (examined): PLANNED. Independently replacing the decoded result, producer execution record, generated source, source map, LLVM export, or LLVM tool/version bytes after authenticated binding refuses with the mismatched association identified by byte-for-byte comparison, even when paths, QSL `package_id`, source/map and LLVM tool/version declarations match; the original decoded outcome remains distinct from the binding diagnostic. A valid typed command-error variant stays non-success even with a matching receipt, while an invalid `/2` or unsupported version refuses at QSL reading before receipt verification
- interface-001.analyze_coverage (examined): semantics: consume QSL's typed /2 reader result without a CG wire parser or /1 fallback; only CG's campaign runner issues the opaque private-field receipt inside the producing run while holding all exact associated bytes; the receipt has no public constructor and carries no digest, and wire decoding alone grants no runner authority; CG first matches it to runner-held issuance state independent of forwarded wire data, then compares each retained association byte for byte with the supplied result, execution record, generated source/map, LLVM export and tool/version before measured classification
- TC-006.receipt-authority (examined): For receipt authority, obtain a receipt issued once by CG's own campaign runner inside that run
while it holds the decoded QSL result, execution record, generated source, source map, LLVM export
and tool/version bytes. Assert that callers cannot construct its private fields and that decoding
forwarded wire bytes alone grants no runner authority: substitute a caller-built wire receipt with
identical self-declared associations but no matching runner-held issuance state and require refusal.
Compare every retained association against the supplied bytes independently; change only the LLVM
tool/vers
- FR-011-OQ-2 (context_only): OQ-2 (requirement and revision identity): the V1 `ClauseRef` supplied the requirement and revision identity for `construct_verdict`. The V2 request (selected clause id and claim occurrence) names no such source.

## Verdict

**PASS** — no findings in this method. FR-011 OQ-2 remains explicitly open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
