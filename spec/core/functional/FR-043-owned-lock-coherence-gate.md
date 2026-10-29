---
id: FR-043
title: "Retired owned lock coherence gate"
type: FR
---
# FR-043: Retired owned lock coherence gate

## Description

RETIRED under IR-1105. The planned IR-641 consumer would have replaced the local
`scripts/check_one_copy.awk` check with `quire check-lock Cargo.lock`. Both checks
compare pinned Git revisions. They add a second dependency identity gate beyond
Cargo's manifest and lock, without a concrete failure that the Cargo resolution
and normal dependency policy checks cannot expose. The local script and its
`make deny` / `make ci` invocation are removed without a replacement checker,
computed digest assertion, compatibility gate, or revision allowance. The
unrelated `cargo deny` policy checks remain in `make deny`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-043-AC-1 | RETIRED under IR-1105: no owned `quire check-lock Cargo.lock` invocation is required in `make deny`. | Inspection |
| FR-043-AC-2 | RETIRED under IR-1105: no duplicate-package or split-revision checker controls are required. | Inspection |
| FR-043-AC-3 | RETIRED under IR-1105: no missing-checker or checker-refusal Make gate is required. | Inspection |
| FR-043-AC-4 | RETIRED under IR-1105: the planned replacement-checker assertion does not apply. The local `scripts/check_one_copy.awk` implementation is removed as part of the same retirement. | Inspection |
| FR-043-AC-5 | RETIRED under IR-1105: no lock-coherence checker qualification or receipt is required. | Inspection |

## Qualification Status

The planned gate was never adopted. Its criteria are retired, so they carry no
qualification or implementation claim. The removal is complete when the local
script and its Make invocations are absent while the unrelated `cargo deny`
policy checks remain.

## Dependencies

None. The retired consumer has no dependency on CLI FR-027.
