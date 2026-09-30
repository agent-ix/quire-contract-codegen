---
id: REV-015
title: "Vacuity primitive recovery and remaining binding gaps"
type: SpecReview
analysis: gap-analysis
scope: "PR #23 bounded source probes, LLVM parser and classification recovery"
review_set: subset
---

# Vacuity primitive recovery and remaining binding gaps

## Summary

PR #23 was specification-only. The recovery
reproduces/adjudicates its review rather than treating every proposed fix as implementation authority.
Only source probes, bounded LLVM reading, measured classification, native fixtures and unambiguous
spec repairs are implemented. Bound aggregate analysis remains dependent on IR #50 and native
producer/run-result ownership. Quire owns static facts; attributed
human decisions remain outside automatic coverage classification.

The old implementation was inspected but not cherry-picked: it mixed a superseded local
evidence framework with seven-state classification, suffix path matching, loose tuple shape,
intersection/max-count inference and unbounded final spans. No old evidence is revived.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-23001 | high | S23-01: Spec repaired: default deny for adverse/unavailable coverage or execution. Aggregate outcome schema and consuming obligation gate remain open. | FR-004-AC-8 |
| FND-23002 | high | S23-02: Repaired measured partition; implication-free positive entry is exercised, zero entry is unexecuted, unavailable observation has no classification. Native and synthetic controls exercise the distinction. | FR-004-AC-3 |
| FND-23003 | high | S23-03: Generator independently counts typed implications on the clause envelope; primitive rejects census mismatch. Complete trusted package/clause population awaits IR-owned BoundPackage; public primitive arguments are not an authenticated census. | FR-004-AC-7 |
| FND-23004 | high | S23-04: FR-001 now owns one function-entry token probe and every consequent entry probe, preserving exact expression line ranges. Actual generated Rust is instrumented in the native test. | FR-001-AC-5 |
| FND-23005 | high | S23-05: Self-declared export version/manifest are checked but expressly do not authenticate a run. Native command/binary/profile/source/campaign/candidate binding remains open; no primitive claims it. | FR-004-AC-4 |
| FND-23006 | high | S23-06: Spec requires structured non-success outcomes retaining available identities. Primitive diagnostics exist; aggregate outcome/report producer is not implemented. | FR-004-AC-5 |
| FND-23007 | high | S23-07: Repaired: one count-bearing non-gap active span must contain the entire entry-token probe. Partial, gap, non-count and unterminated-span controls refuse observation, not invent zero. | FR-004-AC-3 |
| FND-23008 | high | S23-08: Native generated controls plus independent malformed tuple/version/path/population/span tests are executable. Aggregate tampering/discharge controls remain planned and explicitly listed in TC-006. | TC-006 |

Additional review dispositions: exact u64-to-canonical-decimal runtime revision normalization,
outcome axes, native toolchain/target/profile and accounting consistency are now specified but await
aggregate implementation. FR-004 gains its
stakeholder relationship and one matrix row per criterion; these rows remain planned. The live
source-map schema changes because this slice now implements the producer fields, not spec alone.
Package identity is still absent from the historical generated source map and must be added when
joining the public bound population; requirement/revision/clause alone is not cross-package binding.

## Measured profile and implemented controls

Native fixtures call generated oracles for vacuous, implication-free exercised, sibling partial,
unexecuted, nested partial and fully exercised cases. Synthetic controls cover exact tuple arity and
types, coordinate order/zero, source-root boundary and normalized duplicates, traversal/backslash,
missing segments, unavailable probes, partial overlap, gap/non-count and final-span refusal, census
mismatch, contradictory entry/consequent observations and the raw 16 MiB size limit. Decoder limits
also cap files and total segments with independent over-limit controls. The fixture preflights installed
LLVM tools; it does not install missing tools.

## Remaining work and verdict

Implement and independently review the IR-owned complete population join, full artifact/map semantic
validation, native runtime report binding,
versioned success/non-success analysis outcomes and the producer/consumer obligation gate. Tests must
prove removing all clauses, one clause, an implication or evaluator cannot pass, and serialization of
an adverse/unavailable result cannot discharge coverage. Failures must retain diagnostics rather than
turn unavailable evaluation into implication-free success. No automatic human sufficiency decision.

**PARTIAL; NOT FULL FR-004 COMPLETION.** No full matrix row, native campaign producer integration,
independent review, or complete local CI pass is claimed by this document.
