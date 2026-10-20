---
id: SR-13474
title: 'failure-domain review of IR-914 PR #470'
type: SpecReview
analysis: failure-domain
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

**NOT MERGEABLE** — 1 finding(s) in this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Runner-held issuance state is required but its owner, lifetime, run key and replay rule are unspecified across the stated wire-forwarding boundary. A genuine receipt copied from an earlier run could remain in an issuance set and satisfy byte equality, while a later process may have no state at all. Define how binding selects live state for the producing run and rejects a valid receipt replayed outside it; test both cases. | spec/evidence/functional/FR-004-vacuity-evidence.md:164 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | An owned producer channel can still return a stale or misrouted LLVM export from another run. A private invocation lifetime proves custody of the channel, not the origin of its bytes. AC-10 and TC-006 treat that export as unconstructible, so the stated controls cannot detect this cross-run substitution; require a verifiable producer-run association at the channel boundary and a test injecting foreign export bytes. | spec/evidence/functional/FR-004-vacuity-evidence.md:176; spec/evidence/matrix/TC-006-vacuity.md:65-71 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The new private context has one invocation lifetime, no registry or serialized run key, expires on return, and cannot qualify a later process or replayed report. |
| FND-002 | still-open | The revised requirement and test still assume invocation ownership authenticates the bytes returned by the producer channel. |
| FND-002 | fixed | The runner now uses fresh private campaign profile output, exports only that profile through its linked LLVM child, and captures bytes from that child’s direct stream. Detached bytes, foreign child/stream/profile, and pre-existing output refuse as producer_run_mismatch; TC-006 injects detached, foreign-child and foreign-profile cases. This uses transient run inputs rather than a separate tracking record. |

Reviewed fix head: `4d001cdca91aeb7e194f4bdf78e677ad11ab2d4d`.

Reviewed fix head: `707154cfb55d9b4a7fc8a3201e1b6bd1ecde1ba5`. Latest FND-002 disposition: fixed; no open findings.
