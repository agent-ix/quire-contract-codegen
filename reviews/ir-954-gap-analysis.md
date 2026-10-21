---
id: SR-15352
title: "IR-954 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-954; spec/, src/, tests/"
review_set: subset
---

## Summary

Frozen PR509 head 59f879f3; authentic QSL source compilation, checked V2 admission, lowering, contract admission and public routed generation. Planner-scoped cases: plain add/equality/negate literals, plus bounded-peer equality numeric.narrow. No edited wire, patched node, local producer copy or replay-success stand-in. No applicable AssuranceProfile found.
Whole-repository computed matrix: 896 criteria; 524 tagged, 302 untagged, 18 tagged only by ignored tests, 52 method-without-symbol. Findings below inventory all 320 missing default-test bindings, inherited from main. Baseline-to-candidate change: FR-032-AC-9 becomes tagged by partial operand/generation controls; full gated replay remains unimplemented and is not claimed. No fresh run evidence is bound by quoin matrix; supplied focused logs are execution evidence only for their named controls.

## Verdict

FAIL for repository completeness: 302 untagged criteria and 18 ignored-only criteria. These 320 baseline gaps are not introduced by PR509. The scoped IR-954 operand controls are acceptable; this audit does not certify repository completion or complete FR-032 replay.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-005-AC-3: No test trace tag binds this criterion. | spec/core/functional/FR-005-cli-conformance.md:49 | correct-requirement-no-evidence |
| FND-002 | medium | FR-039-AC-4: No test trace tag binds this criterion. | spec/core/functional/FR-039-contract-package-consumption.md:213 | correct-requirement-no-evidence |
| FND-003 | medium | FR-043-AC-1: No test trace tag binds this criterion. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:31 | correct-requirement-no-evidence |
| FND-004 | medium | FR-043-AC-2: No test trace tag binds this criterion. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:32 | correct-requirement-no-evidence |
| FND-005 | medium | FR-043-AC-3: No test trace tag binds this criterion. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:33 | correct-requirement-no-evidence |
| FND-006 | medium | FR-043-AC-5: No test trace tag binds this criterion. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:35 | correct-requirement-no-evidence |
| FND-007 | medium | NFR-001-M-1: No test trace tag binds this criterion. | spec/core/non-functional/NFR-001-reproducible-atomic.md:1 | correct-requirement-no-evidence |
| FND-008 | medium | NFR-002-M-1: No test trace tag binds this criterion. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 | correct-requirement-no-evidence |
| FND-009 | medium | NFR-002-M-3: No test trace tag binds this criterion. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 | correct-requirement-no-evidence |
| FND-010 | medium | FR-004-AC-6: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:186 | correct-requirement-no-evidence |
| FND-011 | medium | FR-004-AC-8: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:188 | correct-requirement-no-evidence |
| FND-012 | medium | FR-004-AC-10: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:190 | correct-requirement-no-evidence |
| FND-013 | medium | FR-004-AC-11: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:191 | correct-requirement-no-evidence |
| FND-014 | medium | FR-004-AC-12: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:192 | correct-requirement-no-evidence |
| FND-015 | medium | FR-004-AC-13: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:193 | correct-requirement-no-evidence |
| FND-016 | medium | FR-004-AC-14: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:194 | correct-requirement-no-evidence |
| FND-017 | medium | FR-004-AC-15: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:195 | correct-requirement-no-evidence |
| FND-018 | medium | FR-004-AC-18: No test trace tag binds this criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:198 | correct-requirement-no-evidence |
| FND-019 | medium | FR-036-AC-1: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:140 | correct-requirement-no-evidence |
| FND-020 | medium | FR-036-AC-2: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:141 | correct-requirement-no-evidence |
| FND-021 | medium | FR-036-AC-3: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:142 | correct-requirement-no-evidence |
| FND-022 | medium | FR-036-AC-4: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:143 | correct-requirement-no-evidence |
| FND-023 | medium | FR-036-AC-5: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:144 | correct-requirement-no-evidence |
| FND-024 | medium | FR-036-AC-6: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:145 | correct-requirement-no-evidence |
| FND-025 | medium | FR-036-AC-7: No test trace tag binds this criterion. | spec/evidence/functional/FR-036-source-obligation-census.md:146 | correct-requirement-no-evidence |
| FND-026 | medium | FR-015-AC-19: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:763 | correct-requirement-no-evidence |
| FND-027 | medium | FR-015-AC-21: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:765 | correct-requirement-no-evidence |
| FND-028 | medium | FR-015-AC-22: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:39 | correct-requirement-no-evidence |
| FND-029 | medium | FR-015-AC-23: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:469 | correct-requirement-no-evidence |
| FND-030 | medium | FR-015-AC-24: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:768 | correct-requirement-no-evidence |
| FND-031 | medium | FR-015-AC-30: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:774 | correct-requirement-no-evidence |
| FND-032 | medium | FR-015-AC-31: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:775 | correct-requirement-no-evidence |
| FND-033 | medium | FR-015-AC-32: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:776 | correct-requirement-no-evidence |
| FND-034 | medium | FR-015-AC-38: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:38 | correct-requirement-no-evidence |
| FND-035 | medium | FR-015-AC-39: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:402 | correct-requirement-no-evidence |
| FND-036 | medium | FR-015-AC-40: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:406 | correct-requirement-no-evidence |
| FND-037 | medium | FR-015-AC-41: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:406 | correct-requirement-no-evidence |
| FND-038 | medium | FR-015-AC-42: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:413 | correct-requirement-no-evidence |
| FND-039 | medium | FR-015-AC-43: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:416 | correct-requirement-no-evidence |
| FND-040 | medium | FR-015-AC-44: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:41 | correct-requirement-no-evidence |
| FND-041 | medium | FR-015-AC-45: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:41 | correct-requirement-no-evidence |
| FND-042 | medium | FR-015-AC-46: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:424 | correct-requirement-no-evidence |
| FND-043 | medium | FR-015-AC-47: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:426 | correct-requirement-no-evidence |
| FND-044 | medium | FR-015-AC-48: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:429 | correct-requirement-no-evidence |
| FND-045 | medium | FR-015-AC-49: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:39 | correct-requirement-no-evidence |
| FND-046 | medium | FR-015-AC-51: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:442 | correct-requirement-no-evidence |
| FND-047 | medium | FR-015-AC-52: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:445 | correct-requirement-no-evidence |
| FND-048 | medium | FR-015-AC-56: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:453 | correct-requirement-no-evidence |
| FND-049 | medium | FR-015-AC-57: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:458 | correct-requirement-no-evidence |
| FND-050 | medium | FR-015-AC-69: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:55 | correct-requirement-no-evidence |
| FND-051 | medium | FR-015-AC-70: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:680 | correct-requirement-no-evidence |
| FND-052 | medium | FR-015-AC-71: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:683 | correct-requirement-no-evidence |
| FND-053 | medium | FR-015-AC-72: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:686 | correct-requirement-no-evidence |
| FND-054 | medium | FR-015-AC-73: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:58 | correct-requirement-no-evidence |
| FND-055 | medium | FR-015-AC-74: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:690 | correct-requirement-no-evidence |
| FND-056 | medium | FR-015-AC-75: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:692 | correct-requirement-no-evidence |
| FND-057 | medium | FR-015-AC-76: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:55 | correct-requirement-no-evidence |
| FND-058 | medium | FR-015-AC-80: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:824 | correct-requirement-no-evidence |
| FND-059 | medium | FR-015-AC-84: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:828 | correct-requirement-no-evidence |
| FND-060 | medium | FR-015-AC-85: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:829 | correct-requirement-no-evidence |
| FND-061 | medium | FR-015-AC-87: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:831 | correct-requirement-no-evidence |
| FND-062 | medium | FR-015-AC-89: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:833 | correct-requirement-no-evidence |
| FND-063 | medium | FR-015-AC-90: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:834 | correct-requirement-no-evidence |
| FND-064 | medium | FR-015-AC-91: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:835 | correct-requirement-no-evidence |
| FND-065 | medium | FR-015-AC-92: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:836 | correct-requirement-no-evidence |
| FND-066 | medium | FR-015-AC-93: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:837 | correct-requirement-no-evidence |
| FND-067 | medium | FR-015-AC-95: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:88 | correct-requirement-no-evidence |
| FND-068 | medium | FR-015-AC-98: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:138 | correct-requirement-no-evidence |
| FND-069 | medium | FR-015-AC-105: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:849 | correct-requirement-no-evidence |
| FND-070 | medium | FR-015-AC-106: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:104 | correct-requirement-no-evidence |
| FND-071 | medium | FR-015-AC-107: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:851 | correct-requirement-no-evidence |
| FND-072 | medium | FR-015-AC-108: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:852 | correct-requirement-no-evidence |
| FND-073 | medium | FR-015-AC-110: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:87 | correct-requirement-no-evidence |
| FND-074 | medium | FR-015-AC-111: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:87 | correct-requirement-no-evidence |
| FND-075 | medium | FR-015-AC-117: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-015-bounded-kani-obligations.md:839 | correct-requirement-no-evidence |
| FND-076 | medium | FR-015-AC-119: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:373 | correct-requirement-no-evidence |
| FND-077 | medium | FR-015-AC-120: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:864 | correct-requirement-no-evidence |
| FND-078 | medium | FR-015-AC-121: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:865 | correct-requirement-no-evidence |
| FND-079 | medium | FR-015-AC-122: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 | correct-requirement-no-evidence |
| FND-080 | medium | FR-015-AC-125: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:869 | correct-requirement-no-evidence |
| FND-081 | medium | FR-015-AC-126: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:870 | correct-requirement-no-evidence |
| FND-082 | medium | FR-015-AC-127: No test trace tag binds this criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:871 | correct-requirement-no-evidence |
| FND-083 | medium | FR-017-AC-7: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-017-kani-execution-evidence.md:859 | correct-requirement-no-evidence |
| FND-084 | medium | FR-017-AC-19: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:868 | correct-requirement-no-evidence |
| FND-085 | medium | FR-017-AC-26: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:875 | correct-requirement-no-evidence |
| FND-086 | medium | FR-017-AC-27: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-017-kani-execution-evidence.md:650 | correct-requirement-no-evidence |
| FND-087 | medium | FR-017-AC-30: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-017-kani-execution-evidence.md:879 | correct-requirement-no-evidence |
| FND-088 | medium | FR-017-AC-47: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-017-kani-execution-evidence.md:896 | correct-requirement-no-evidence |
| FND-089 | medium | FR-017-AC-52: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:901 | correct-requirement-no-evidence |
| FND-090 | medium | FR-017-AC-53: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:902 | correct-requirement-no-evidence |
| FND-091 | medium | FR-017-AC-54: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:903 | correct-requirement-no-evidence |
| FND-092 | medium | FR-017-AC-55: No test trace tag binds this criterion. | spec/kani/functional/FR-017-kani-execution-evidence.md:904 | correct-requirement-no-evidence |
| FND-093 | medium | FR-017-CON-1: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-017-kani-execution-evidence.md:848 | correct-requirement-no-evidence |
| FND-094 | medium | FR-025-AC-1: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:94 | correct-requirement-no-evidence |
| FND-095 | medium | FR-025-AC-2: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:95 | correct-requirement-no-evidence |
| FND-096 | medium | FR-025-AC-3: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:96 | correct-requirement-no-evidence |
| FND-097 | medium | FR-025-AC-4: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:97 | correct-requirement-no-evidence |
| FND-098 | medium | FR-025-AC-5: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:98 | correct-requirement-no-evidence |
| FND-099 | medium | FR-025-AC-6: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:99 | correct-requirement-no-evidence |
| FND-100 | medium | FR-025-AC-7: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:100 | correct-requirement-no-evidence |
| FND-101 | medium | FR-025-AC-8: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:101 | correct-requirement-no-evidence |
| FND-102 | medium | FR-025-AC-9: No test trace tag binds this criterion. | spec/kani/functional/FR-025-generated-subject-abi.md:102 | correct-requirement-no-evidence |
| FND-103 | medium | FR-028-AC-5: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:206 | correct-requirement-no-evidence |
| FND-104 | medium | FR-028-AC-6: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:207 | correct-requirement-no-evidence |
| FND-105 | medium | FR-028-AC-7: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:208 | correct-requirement-no-evidence |
| FND-106 | medium | FR-028-AC-8: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:209 | correct-requirement-no-evidence |
| FND-107 | medium | FR-028-AC-9: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:210 | correct-requirement-no-evidence |
| FND-108 | medium | FR-028-AC-13: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:33 | correct-requirement-no-evidence |
| FND-109 | medium | FR-028-AC-14: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:81 | correct-requirement-no-evidence |
| FND-110 | medium | FR-028-AC-15: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:143 | correct-requirement-no-evidence |
| FND-111 | medium | FR-028-AC-16: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:70 | correct-requirement-no-evidence |
| FND-112 | medium | FR-028-AC-17: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:81 | correct-requirement-no-evidence |
| FND-113 | medium | FR-028-AC-18: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:84 | correct-requirement-no-evidence |
| FND-114 | medium | FR-028-AC-19: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:163 | correct-requirement-no-evidence |
| FND-115 | medium | FR-028-AC-20: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:168 | correct-requirement-no-evidence |
| FND-116 | medium | FR-028-AC-22: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:221 | correct-requirement-no-evidence |
| FND-117 | medium | FR-028-AC-23: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:178 | correct-requirement-no-evidence |
| FND-118 | medium | FR-028-AC-24: No test trace tag binds this criterion. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:33 | correct-requirement-no-evidence |
| FND-119 | medium | FR-029-AC-17: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:377 | correct-requirement-no-evidence |
| FND-120 | medium | FR-029-AC-19: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:379 | correct-requirement-no-evidence |
| FND-121 | medium | FR-029-AC-21: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:381 | correct-requirement-no-evidence |
| FND-122 | medium | FR-029-AC-22: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382 | correct-requirement-no-evidence |
| FND-123 | medium | FR-029-AC-23: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:383 | correct-requirement-no-evidence |
| FND-124 | medium | FR-029-AC-24: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:384 | correct-requirement-no-evidence |
| FND-125 | medium | FR-029-AC-25: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:385 | correct-requirement-no-evidence |
| FND-126 | medium | FR-029-AC-26: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:386 | correct-requirement-no-evidence |
| FND-127 | medium | FR-029-AC-27: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:387 | correct-requirement-no-evidence |
| FND-128 | medium | FR-029-AC-30: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:207 | correct-requirement-no-evidence |
| FND-129 | medium | FR-029-AC-31: No test trace tag binds this criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:391 | correct-requirement-no-evidence |
| FND-130 | medium | FR-030-AC-19: No test trace tag binds this criterion. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:237 | correct-requirement-no-evidence |
| FND-131 | medium | FR-030-AC-20: No test trace tag binds this criterion. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:238 | correct-requirement-no-evidence |
| FND-132 | medium | FR-030-AC-21: No test trace tag binds this criterion. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:239 | correct-requirement-no-evidence |
| FND-133 | medium | FR-030-AC-22: No test trace tag binds this criterion. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:240 | correct-requirement-no-evidence |
| FND-134 | medium | FR-034-AC-1: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1844 | correct-requirement-no-evidence |
| FND-135 | medium | FR-034-AC-2: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1845 | correct-requirement-no-evidence |
| FND-136 | medium | FR-034-AC-3: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1846 | correct-requirement-no-evidence |
| FND-137 | medium | FR-034-AC-4: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1847 | correct-requirement-no-evidence |
| FND-138 | medium | FR-034-AC-5: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1848 | correct-requirement-no-evidence |
| FND-139 | medium | FR-034-AC-6: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1849 | correct-requirement-no-evidence |
| FND-140 | medium | FR-034-AC-7: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1850 | correct-requirement-no-evidence |
| FND-141 | medium | FR-034-AC-8: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1851 | correct-requirement-no-evidence |
| FND-142 | medium | FR-034-AC-9: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1852 | correct-requirement-no-evidence |
| FND-143 | medium | FR-034-AC-10: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1853 | correct-requirement-no-evidence |
| FND-144 | medium | FR-034-AC-11: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1854 | correct-requirement-no-evidence |
| FND-145 | medium | FR-034-AC-12: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1855 | correct-requirement-no-evidence |
| FND-146 | medium | FR-034-AC-13: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1856 | correct-requirement-no-evidence |
| FND-147 | medium | FR-034-AC-14: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1857 | correct-requirement-no-evidence |
| FND-148 | medium | FR-034-AC-15: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1858 | correct-requirement-no-evidence |
| FND-149 | medium | FR-034-AC-16: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1859 | correct-requirement-no-evidence |
| FND-150 | medium | FR-034-AC-17: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1860 | correct-requirement-no-evidence |
| FND-151 | medium | FR-034-AC-18: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1861 | correct-requirement-no-evidence |
| FND-152 | medium | FR-034-AC-19: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1862 | correct-requirement-no-evidence |
| FND-153 | medium | FR-034-AC-20: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1863 | correct-requirement-no-evidence |
| FND-154 | medium | FR-034-AC-21: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1864 | correct-requirement-no-evidence |
| FND-155 | medium | FR-034-AC-22: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1865 | correct-requirement-no-evidence |
| FND-156 | medium | FR-034-AC-23: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1866 | correct-requirement-no-evidence |
| FND-157 | medium | FR-034-AC-24: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1867 | correct-requirement-no-evidence |
| FND-158 | medium | FR-034-AC-25: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1868 | correct-requirement-no-evidence |
| FND-159 | medium | FR-034-AC-26: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1869 | correct-requirement-no-evidence |
| FND-160 | medium | FR-034-AC-27: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1870 | correct-requirement-no-evidence |
| FND-161 | medium | FR-034-AC-28: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1871 | correct-requirement-no-evidence |
| FND-162 | medium | FR-034-AC-29: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1872 | correct-requirement-no-evidence |
| FND-163 | medium | FR-034-AC-31: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1874 | correct-requirement-no-evidence |
| FND-164 | medium | FR-034-AC-32: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1875 | correct-requirement-no-evidence |
| FND-165 | medium | FR-034-AC-33: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1876 | correct-requirement-no-evidence |
| FND-166 | medium | FR-034-AC-34: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1877 | correct-requirement-no-evidence |
| FND-167 | medium | FR-034-AC-35: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1878 | correct-requirement-no-evidence |
| FND-168 | medium | FR-034-AC-36: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1879 | correct-requirement-no-evidence |
| FND-169 | medium | FR-034-AC-37: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1880 | correct-requirement-no-evidence |
| FND-170 | medium | FR-034-AC-38: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1881 | correct-requirement-no-evidence |
| FND-171 | medium | FR-034-AC-39: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1882 | correct-requirement-no-evidence |
| FND-172 | medium | FR-034-AC-40: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1883 | correct-requirement-no-evidence |
| FND-173 | medium | FR-034-AC-51: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1894 | correct-requirement-no-evidence |
| FND-174 | medium | FR-034-AC-52: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1895 | correct-requirement-no-evidence |
| FND-175 | medium | FR-034-AC-53: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1896 | correct-requirement-no-evidence |
| FND-176 | medium | FR-034-AC-54: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1897 | correct-requirement-no-evidence |
| FND-177 | medium | FR-034-AC-57: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1900 | correct-requirement-no-evidence |
| FND-178 | medium | FR-034-AC-58: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1901 | correct-requirement-no-evidence |
| FND-179 | medium | FR-034-AC-59: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1902 | correct-requirement-no-evidence |
| FND-180 | medium | FR-034-AC-60: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1903 | correct-requirement-no-evidence |
| FND-181 | medium | FR-034-AC-61: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1904 | correct-requirement-no-evidence |
| FND-182 | medium | FR-034-AC-62: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1905 | correct-requirement-no-evidence |
| FND-183 | medium | FR-034-AC-63: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1906 | correct-requirement-no-evidence |
| FND-184 | medium | FR-034-AC-65: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1908 | correct-requirement-no-evidence |
| FND-185 | medium | FR-034-AC-66: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1909 | correct-requirement-no-evidence |
| FND-186 | medium | FR-034-AC-67: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1910 | correct-requirement-no-evidence |
| FND-187 | medium | FR-034-AC-69: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1912 | correct-requirement-no-evidence |
| FND-188 | medium | FR-034-AC-70: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1913 | correct-requirement-no-evidence |
| FND-189 | medium | FR-034-AC-71: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1914 | correct-requirement-no-evidence |
| FND-190 | medium | FR-034-AC-73: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1916 | correct-requirement-no-evidence |
| FND-191 | medium | FR-034-AC-74: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1917 | correct-requirement-no-evidence |
| FND-192 | medium | FR-034-AC-75: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1918 | correct-requirement-no-evidence |
| FND-193 | medium | FR-034-AC-76: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1919 | correct-requirement-no-evidence |
| FND-194 | medium | FR-034-AC-77: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1920 | correct-requirement-no-evidence |
| FND-195 | medium | FR-034-AC-78: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1921 | correct-requirement-no-evidence |
| FND-196 | medium | FR-034-AC-79: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1922 | correct-requirement-no-evidence |
| FND-197 | medium | FR-034-AC-80: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1923 | correct-requirement-no-evidence |
| FND-198 | medium | FR-034-AC-81: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1924 | correct-requirement-no-evidence |
| FND-199 | medium | FR-034-AC-82: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1925 | correct-requirement-no-evidence |
| FND-200 | medium | FR-034-AC-83: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1926 | correct-requirement-no-evidence |
| FND-201 | medium | FR-034-AC-84: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1927 | correct-requirement-no-evidence |
| FND-202 | medium | FR-034-AC-85: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1928 | correct-requirement-no-evidence |
| FND-203 | medium | FR-034-AC-86: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1929 | correct-requirement-no-evidence |
| FND-204 | medium | FR-034-AC-87: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1930 | correct-requirement-no-evidence |
| FND-205 | medium | FR-034-AC-88: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1931 | correct-requirement-no-evidence |
| FND-206 | medium | FR-034-AC-89: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1932 | correct-requirement-no-evidence |
| FND-207 | medium | FR-034-AC-90: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1933 | correct-requirement-no-evidence |
| FND-208 | medium | FR-034-AC-91: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1934 | correct-requirement-no-evidence |
| FND-209 | medium | FR-034-AC-92: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1935 | correct-requirement-no-evidence |
| FND-210 | medium | FR-034-AC-94: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1937 | correct-requirement-no-evidence |
| FND-211 | medium | FR-034-AC-95: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1938 | correct-requirement-no-evidence |
| FND-212 | medium | FR-034-AC-96: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1939 | correct-requirement-no-evidence |
| FND-213 | medium | FR-034-AC-97: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1940 | correct-requirement-no-evidence |
| FND-214 | medium | FR-034-AC-98: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1941 | correct-requirement-no-evidence |
| FND-215 | medium | FR-034-AC-99: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1942 | correct-requirement-no-evidence |
| FND-216 | medium | FR-034-AC-100: No test trace tag binds this criterion. | spec/kani/functional/FR-034-caller-death-ownership.md:1943 | correct-requirement-no-evidence |
| FND-217 | medium | FR-041-AC-4: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:69 | correct-requirement-no-evidence |
| FND-218 | medium | NFR-006-M-1: No test trace tag binds this criterion. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 | correct-requirement-no-evidence |
| FND-219 | medium | NFR-006-M-2: No test trace tag binds this criterion. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 | correct-requirement-no-evidence |
| FND-220 | medium | NFR-006-M-3: No test trace tag binds this criterion. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 | correct-requirement-no-evidence |
| FND-221 | medium | FR-014-AC-52: No test trace tag binds this criterion. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:292 | correct-requirement-no-evidence |
| FND-222 | medium | FR-018-AC-21: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:584 | correct-requirement-no-evidence |
| FND-223 | medium | FR-018-AC-22: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:588 | correct-requirement-no-evidence |
| FND-224 | medium | FR-018-AC-23: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:595 | correct-requirement-no-evidence |
| FND-225 | medium | FR-018-AC-33: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:643 | correct-requirement-no-evidence |
| FND-226 | medium | FR-018-AC-34: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:644 | correct-requirement-no-evidence |
| FND-227 | medium | FR-018-AC-35: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:645 | correct-requirement-no-evidence |
| FND-228 | medium | FR-018-AC-36: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:646 | correct-requirement-no-evidence |
| FND-229 | medium | FR-018-AC-37: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:647 | correct-requirement-no-evidence |
| FND-230 | medium | FR-018-AC-38: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:648 | correct-requirement-no-evidence |
| FND-231 | medium | FR-018-AC-39: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:649 | correct-requirement-no-evidence |
| FND-232 | medium | FR-018-AC-40: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:650 | correct-requirement-no-evidence |
| FND-233 | medium | FR-018-AC-41: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:651 | correct-requirement-no-evidence |
| FND-234 | medium | FR-018-AC-42: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:652 | correct-requirement-no-evidence |
| FND-235 | medium | FR-018-AC-43: No test trace tag binds this criterion. | spec/oracle/functional/FR-018-composite-equality-oracles.md:653 | correct-requirement-no-evidence |
| FND-236 | medium | FR-021-AC-30: No test trace tag binds this criterion. | spec/oracle/functional/FR-021-function-application-oracles.md:277 | correct-requirement-no-evidence |
| FND-237 | medium | FR-031-AC-2: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:69 | correct-requirement-no-evidence |
| FND-238 | medium | FR-031-AC-6: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:481 | correct-requirement-no-evidence |
| FND-239 | medium | FR-031-AC-16: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:518 | correct-requirement-no-evidence |
| FND-240 | medium | FR-031-AC-17: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:189 | correct-requirement-no-evidence |
| FND-241 | medium | FR-031-AC-27: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:269 | correct-requirement-no-evidence |
| FND-242 | medium | FR-031-AC-20: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:524 | correct-requirement-no-evidence |
| FND-243 | medium | FR-031-AC-22: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:71 | correct-requirement-no-evidence |
| FND-244 | medium | FR-031-AC-26: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:527 | correct-requirement-no-evidence |
| FND-245 | medium | FR-031-AC-23: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:513 | correct-requirement-no-evidence |
| FND-246 | medium | FR-031-AC-24: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:529 | correct-requirement-no-evidence |
| FND-247 | medium | FR-031-AC-25: No test trace tag binds this criterion. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:166 | correct-requirement-no-evidence |
| FND-248 | medium | FR-035-AC-1: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:107 | correct-requirement-no-evidence |
| FND-249 | medium | FR-035-AC-2: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:108 | correct-requirement-no-evidence |
| FND-250 | medium | FR-035-AC-3: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:109 | correct-requirement-no-evidence |
| FND-251 | medium | FR-035-AC-4: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:110 | correct-requirement-no-evidence |
| FND-252 | medium | FR-035-AC-5: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:111 | correct-requirement-no-evidence |
| FND-253 | medium | FR-035-AC-6: No test trace tag binds this criterion. | spec/oracle/functional/FR-035-caller-text-admission.md:112 | correct-requirement-no-evidence |
| FND-254 | medium | FR-042-AC-1: No test trace tag binds this criterion. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:31 | correct-requirement-no-evidence |
| FND-255 | medium | FR-024-AC-1: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:75 | correct-requirement-no-evidence |
| FND-256 | medium | FR-024-AC-7: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:352 | correct-requirement-no-evidence |
| FND-257 | medium | FR-024-AC-8: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:353 | correct-requirement-no-evidence |
| FND-258 | medium | FR-024-AC-18: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:362 | correct-requirement-no-evidence |
| FND-259 | medium | FR-024-AC-34: This criterion is bound only to ignored tests; no default-lane verification is established. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:378 | correct-requirement-no-evidence |
| FND-260 | medium | FR-024-AC-35: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:379 | correct-requirement-no-evidence |
| FND-261 | medium | FR-024-AC-36: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:380 | correct-requirement-no-evidence |
| FND-262 | medium | FR-024-AC-37: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:381 | correct-requirement-no-evidence |
| FND-263 | medium | FR-024-AC-38: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:382 | correct-requirement-no-evidence |
| FND-264 | medium | FR-024-AC-39: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:383 | correct-requirement-no-evidence |
| FND-265 | medium | FR-024-AC-40: No test trace tag binds this criterion. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:384 | correct-requirement-no-evidence |
| FND-266 | medium | FR-032-AC-1: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:382 | correct-requirement-no-evidence |
| FND-267 | medium | FR-032-AC-4: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:385 | correct-requirement-no-evidence |
| FND-268 | medium | FR-032-AC-5: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:386 | correct-requirement-no-evidence |
| FND-269 | medium | FR-032-AC-6: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:387 | correct-requirement-no-evidence |
| FND-270 | medium | FR-032-AC-7: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:388 | correct-requirement-no-evidence |
| FND-271 | medium | FR-032-AC-8: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:389 | correct-requirement-no-evidence |
| FND-272 | medium | FR-032-AC-10: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:391 | correct-requirement-no-evidence |
| FND-273 | medium | FR-032-AC-11: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:392 | correct-requirement-no-evidence |
| FND-274 | medium | FR-032-AC-12: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:393 | correct-requirement-no-evidence |
| FND-275 | medium | FR-032-AC-13: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:394 | correct-requirement-no-evidence |
| FND-276 | medium | FR-032-AC-14: No test trace tag binds this criterion. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:395 | correct-requirement-no-evidence |
| FND-277 | medium | FR-033-AC-2: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:362 | correct-requirement-no-evidence |
| FND-278 | medium | FR-033-AC-3: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:363 | correct-requirement-no-evidence |
| FND-279 | medium | FR-033-AC-4: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:364 | correct-requirement-no-evidence |
| FND-280 | medium | FR-033-AC-5: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:365 | correct-requirement-no-evidence |
| FND-281 | medium | FR-033-AC-6: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:366 | correct-requirement-no-evidence |
| FND-282 | medium | FR-033-AC-8: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:368 | correct-requirement-no-evidence |
| FND-283 | medium | FR-033-AC-10: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:370 | correct-requirement-no-evidence |
| FND-284 | medium | FR-033-AC-14: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:374 | correct-requirement-no-evidence |
| FND-285 | medium | FR-033-AC-15: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:375 | correct-requirement-no-evidence |
| FND-286 | medium | FR-033-AC-17: No test trace tag binds this criterion. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:377 | correct-requirement-no-evidence |
| FND-287 | medium | FR-019-AC-3: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:519 | correct-requirement-no-evidence |
| FND-288 | medium | FR-019-AC-15: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:547 | correct-requirement-no-evidence |
| FND-289 | medium | FR-019-AC-24: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:548 | correct-requirement-no-evidence |
| FND-290 | medium | FR-019-AC-34: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:558 | correct-requirement-no-evidence |
| FND-291 | medium | FR-019-AC-35: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:559 | correct-requirement-no-evidence |
| FND-292 | medium | FR-019-AC-36: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:560 | correct-requirement-no-evidence |
| FND-293 | medium | FR-019-AC-37: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:561 | correct-requirement-no-evidence |
| FND-294 | medium | FR-019-AC-38: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:562 | correct-requirement-no-evidence |
| FND-295 | medium | FR-019-AC-39: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:563 | correct-requirement-no-evidence |
| FND-296 | medium | FR-019-AC-40: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:564 | correct-requirement-no-evidence |
| FND-297 | medium | FR-019-AC-41: No test trace tag binds this criterion. | spec/routed/functional/FR-019-capability-settlement.md:565 | correct-requirement-no-evidence |
| FND-298 | medium | FR-022-AC-26: No test trace tag binds this criterion. | spec/routed/functional/FR-022-routed-generation.md:361 | correct-requirement-no-evidence |
| FND-299 | medium | FR-022-AC-27: No test trace tag binds this criterion. | spec/routed/functional/FR-022-routed-generation.md:362 | correct-requirement-no-evidence |
| FND-300 | medium | FR-022-AC-28: No test trace tag binds this criterion. | spec/routed/functional/FR-022-routed-generation.md:363 | correct-requirement-no-evidence |
| FND-301 | medium | FR-026-AC-1: No test trace tag binds this criterion. | spec/routed/functional/FR-026-backend-adapter-contract.md:60 | correct-requirement-no-evidence |
| FND-302 | medium | FR-026-AC-4: No test trace tag binds this criterion. | spec/routed/functional/FR-026-backend-adapter-contract.md:61 | correct-requirement-no-evidence |
| FND-303 | medium | FR-002-AC-7: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:91 | correct-requirement-no-evidence |
| FND-304 | medium | FR-002-AC-8: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:92 | correct-requirement-no-evidence |
| FND-305 | medium | FR-002-AC-10: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:94 | correct-requirement-no-evidence |
| FND-306 | medium | FR-002-AC-11: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:95 | correct-requirement-no-evidence |
| FND-307 | medium | FR-002-AC-12: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:96 | correct-requirement-no-evidence |
| FND-308 | medium | FR-002-AC-13: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:97 | correct-requirement-no-evidence |
| FND-309 | medium | FR-002-AC-14: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:98 | correct-requirement-no-evidence |
| FND-310 | medium | FR-002-AC-15: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:99 | correct-requirement-no-evidence |
| FND-311 | medium | FR-002-AC-16: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:100 | correct-requirement-no-evidence |
| FND-312 | medium | FR-002-AC-17: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:101 | correct-requirement-no-evidence |
| FND-313 | medium | FR-002-AC-18: No test trace tag binds this criterion. | spec/strategy/functional/FR-002-tristate-proptest.md:102 | correct-requirement-no-evidence |
| FND-314 | medium | FR-008-AC-7: No test trace tag binds this criterion. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:123 | correct-requirement-no-evidence |
| FND-315 | medium | FR-008-AC-9: No test trace tag binds this criterion. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:125 | correct-requirement-no-evidence |
| FND-316 | medium | FR-009-AC-7: No test trace tag binds this criterion. | spec/strategy/functional/FR-009-constructive-correlated-populations.md:77 | correct-requirement-no-evidence |
| FND-317 | medium | NFR-004-M-1: No test trace tag binds this criterion. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 | correct-requirement-no-evidence |
| FND-318 | medium | NFR-004-M-2: No test trace tag binds this criterion. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 | correct-requirement-no-evidence |
| FND-319 | medium | NFR-004-M-3: No test trace tag binds this criterion. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 | correct-requirement-no-evidence |
| FND-320 | medium | NFR-004-M-4: No test trace tag binds this criterion. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 | correct-requirement-no-evidence |

## Coverage

Plan completion: not assessed

Rust checks: no CI/lint/coverage weakening; no unsafe, production panic, boundary cast, async/locking change, vendoring, host change, new public API or protected IR-948 file. Quantity explicitly follows the existing typed NonRangeType refusal. Arithmetic controls assert literal type, singleton operand range, authentic InlineLiteral node/occurrence/position and actual generated harness draws. Equality checks emitted inline/reference shape and successful oracle generation; narrowed equality checks actual generated checked_bounds empty. This does not inspect QSL private parity Domain::Bounds, mint O-09 identity, execute generated oracle or run scalar replay. Existing scalar_playback helper returns an arithmetic harness and requires a result conversion, so does not provide the new checked-package/equality fixture API. No duplicate public helper was added.

Supplied logs inspected: old producer red control (3 intended reference/literal failures), current scalar/playback (14 pass), frame (3 pass), focused Clippy and fmt pass. No reviewer Cargo/Kani/CBMC execution; one aggregate gate remains required on final reviewed head.
Whole-tree inventory: 94 Rust source files, 67 Rust test files; public surface inventory retained in scratch. Master spec maps core, publication, strategy, oracle, routed, Kani, replay and evidence subsystems. Reverse inspection found no newly unowned behavior: scalar seam is FR-014/FR-015/FR-022 with FR-032 positional context; frame refusal is existing range admission. Source TODO/unimplemented scan found no production placeholders; 9 test hits are generated-source forbidden-token assertions or adversarial parser inputs. Deep semantic review skipped (not opted in); full individual public-symbol semantic allocation was not performed, so no whole-tree semantic/reverse completeness claim.

## Examined scope

```yaml
clean: false
scope:
  - {id: FR-005-AC-3, path: &p0 "spec/core/functional/FR-005-cli-conformance.md", role: examined, excerpt: &e0 "For every shared-corpus case on which two backends"}
  - {id: FR-039-AC-4, path: &p1 "spec/core/functional/FR-039-contract-package-consumption.md", role: examined, excerpt: &e1 "For a genuinely admitted FR-330 package, CG binds "}
  - {id: FR-043-AC-1, path: &p5 "spec/core/functional/FR-043-owned-lock-coherence-gate.md", role: examined, excerpt: &e2 "The normal `make deny` recipe invokes the released"}
  - {id: FR-043-AC-2, path: *p5, role: examined, excerpt: &e3 "Through the same normal `make deny` recipe, a scra"}
  - {id: FR-043-AC-3, path: *p5, role: examined, excerpt: &e4 "A missing checker executable, malformed scratch lo"}
  - {id: FR-043-AC-5, path: *p5, role: examined, excerpt: &e5 "Adoption leaves the committed Cargo manifests and "}
  - {id: NFR-001-M-1, path: &p6 "spec/core/non-functional/NFR-001-reproducible-atomic.md", role: examined, excerpt: &e6 "Byte differences between repeated generations"}
  - {id: NFR-002-M-1, path: &p8 "spec/core/non-functional/NFR-002-provenance-boundary.md", role: examined, excerpt: &e7 "Generated Rust files missing generator or input-ca"}
  - {id: NFR-002-M-3, path: *p8, role: examined, excerpt: &e8 "Silent unsupported/inconclusive states"}
  - {id: FR-004-AC-6, path: &p17 "spec/evidence/functional/FR-004-vacuity-evidence.md", role: examined, excerpt: &e9 "Campaign counts and test outcome remain complete f"}
  - {id: FR-004-AC-8, path: *p17, role: examined, excerpt: &e10 "Adverse coverage and non-success native execution "}
  - {id: FR-004-AC-10, path: *p17, role: examined, excerpt: &e11 "PLANNED. CG binds coverage inside the same live ca"}
  - {id: FR-004-AC-11, path: *p17, role: examined, excerpt: &e12 "PLANNED. The consuming report retains vacuity and "}
  - {id: FR-004-AC-12, path: *p17, role: examined, excerpt: &e13 "PLANNED. CG consumes QSL's typed strict-reader cla"}
  - {id: FR-004-AC-13, path: *p17, role: examined, excerpt: &e14 "PLANNED. QSL `run_clause` success and a decisive w"}
  - {id: FR-004-AC-14, path: *p17, role: examined, excerpt: &e15 "PLANNED. Only CG's campaign runner constructs the "}
  - {id: FR-004-AC-15, path: *p17, role: examined, excerpt: &e16 "PLANNED. A controlled attempt to replace the stric"}
  - {id: FR-004-AC-18, path: *p17, role: examined, excerpt: &e17 "PLANNED (IR-815). A V2 clause whose generated sour"}
  - {id: FR-036-AC-1, path: &p24 "spec/evidence/functional/FR-036-source-obligation-census.md", role: examined, excerpt: &e18 "A genuine typed fixture with criteria C1/C2, oblig"}
  - {id: FR-036-AC-2, path: *p24, role: examined, excerpt: &e19 "For the unchanged AC-1 fixture, omitting a result,"}
  - {id: FR-036-AC-3, path: *p24, role: examined, excerpt: &e20 "Capability facts preserve unsupported capability a"}
  - {id: FR-036-AC-4, path: *p24, role: examined, excerpt: &e21 "A matching executed production fact retains its ac"}
  - {id: FR-036-AC-5, path: *p24, role: examined, excerpt: &e22 "Shadow-only evidence, a narrower effective bound a"}
  - {id: FR-036-AC-6, path: *p24, role: examined, excerpt: &e23 "A downstream consumer above CG and IR borrows the "}
  - {id: FR-036-AC-7, path: *p24, role: examined, excerpt: &e24 "Missing/incomplete authoritative inputs, a relatio"}
  - {id: FR-015-AC-19, path: &p81 "spec/kani/functional/FR-015-bounded-kani-obligations.md", role: examined, excerpt: &e25 "A zero-input transition over one bounded-integer s"}
  - {id: FR-015-AC-21, path: *p81, role: examined, excerpt: &e26 "A precondition that reads a post-state value is re"}
  - {id: FR-015-AC-22, path: *p81, role: examined, excerpt: &e27 "A declared proof-dependency census with an empty o"}
  - {id: FR-015-AC-23, path: *p81, role: examined, excerpt: &e28 "Every requested item receives exactly one `Obligat"}
  - {id: FR-015-AC-24, path: *p81, role: examined, excerpt: &e29 "A falsifying in-domain assignment of a plain bound"}
  - {id: FR-015-AC-30, path: *p81, role: examined, excerpt: &e30 "With the installed backend, the operation contract"}
  - {id: FR-015-AC-31, path: *p81, role: examined, excerpt: &e31 "With the installed backend, a frame-allowed effect"}
  - {id: FR-015-AC-32, path: *p81, role: examined, excerpt: &e32 "The forbidden frame counterexample, executed nativ"}
  - {id: FR-015-AC-38, path: *p81, role: examined, excerpt: &e33 "A V2 clause claim naming a `state`/`state_clause` "}
  - {id: FR-015-AC-39, path: *p81, role: examined, excerpt: &e34 "A V2 clause harness draws one nondeterministic val"}
  - {id: FR-015-AC-40, path: *p81, role: examined, excerpt: &e35 "A harness for a V2 clause whose condition, inline "}
  - {id: FR-015-AC-41, path: *p81, role: examined, excerpt: &e36 "A V2 clause whose body holds integer arithmetic (a"}
  - {id: FR-015-AC-42, path: *p81, role: examined, excerpt: &e37 "A V2 clause body holding an operator the V2 arm do"}
  - {id: FR-015-AC-43, path: *p81, role: examined, excerpt: &e38 "A V2 postcondition harness emits every `preconditi"}
  - {id: FR-015-AC-44, path: *p81, role: examined, excerpt: &e39 "A census declared on a V2 clause claim with an emp"}
  - {id: FR-015-AC-45, path: *p81, role: examined, excerpt: &e40 "A valid census declared on a V2 clause claim is fo"}
  - {id: FR-015-AC-46, path: *p81, role: examined, excerpt: &e41 "Every V2 clause harness contains exactly one non-v"}
  - {id: FR-015-AC-47, path: *p81, role: examined, excerpt: &e42 "Regeneration of a V2 clause harness from equal inp"}
  - {id: FR-015-AC-48, path: *p81, role: examined, excerpt: &e43 "The obligation identity of a V2 clause claim is fo"}
  - {id: FR-015-AC-49, path: *p81, role: examined, excerpt: &e44 "A V2 invariant claim yields a harness that draws i"}
  - {id: FR-015-AC-51, path: *p81, role: examined, excerpt: &e45 "Until the final retirement slice (FR-015-AC-111, A"}
  - {id: FR-015-AC-52, path: *p81, role: examined, excerpt: &e46 "Until the final retirement slice (FR-015-AC-111, A"}
  - {id: FR-015-AC-56, path: *p81, role: examined, excerpt: &e47 "With the installed backend, a V1 bundle harness wh"}
  - {id: FR-015-AC-57, path: *p81, role: examined, excerpt: &e48 "While its arm remains (FR-015-AC-109 to FR-015-AC-"}
  - {id: FR-015-AC-69, path: *p81, role: examined, excerpt: &e49 "A composite-equality claim over operands that are "}
  - {id: FR-015-AC-70, path: *p81, role: examined, excerpt: &e50 "The harness draws one nondeterministic value per l"}
  - {id: FR-015-AC-71, path: *p81, role: examined, excerpt: &e51 "Over every drawn pair the harness asserts that the"}
  - {id: FR-015-AC-72, path: *p81, role: examined, excerpt: &e52 "The emitted shadow and harness, parsed, contain no"}
  - {id: FR-015-AC-73, path: *p81, role: examined, excerpt: &e53 "Each row of the unsupported-shape table above is p"}
  - {id: FR-015-AC-74, path: *p81, role: examined, excerpt: &e54 "With the installed backend, the harness of the cor"}
  - {id: FR-015-AC-75, path: *p81, role: examined, excerpt: &e55 "With the installed backend each seeded mutant of t"}
  - {id: FR-015-AC-76, path: *p81, role: examined, excerpt: &e56 "The identity of a composite-equality harness recor"}
  - {id: FR-015-AC-80, path: *p81, role: examined, excerpt: &e57 "PLANNED (IR-624), IR-628 accessor merged; CG depen"}
  - {id: FR-015-AC-84, path: *p81, role: examined, excerpt: &e58 "Until the arithmetic slice (FR-015-AC-109) deletes"}
  - {id: FR-015-AC-85, path: *p81, role: examined, excerpt: &e59 "Until the arithmetic slice (FR-015-AC-109) deletes"}
  - {id: FR-015-AC-87, path: *p81, role: examined, excerpt: &e60 "Until the graph slice (FR-015-AC-111) deletes the "}
  - {id: FR-015-AC-89, path: *p81, role: examined, excerpt: &e61 "Until the collection slice (FR-015-AC-110) deletes"}
  - {id: FR-015-AC-90, path: *p81, role: examined, excerpt: &e62 "Until FR-015-AC-111 deletes the bounded-corpus emi"}
  - {id: FR-015-AC-91, path: *p81, role: examined, excerpt: &e63 "Until the final retirement slice (FR-015-AC-111, A"}
  - {id: FR-015-AC-92, path: *p81, role: examined, excerpt: &e64 "Until FR-015-AC-111 deletes the bounded-corpus emi"}
  - {id: FR-015-AC-93, path: *p81, role: examined, excerpt: &e65 "Until the final retirement slice (FR-015-AC-111, A"}
  - {id: FR-015-AC-95, path: *p81, role: examined, excerpt: &e66 "AD-004 step 4g retires the hand-built bounded-corp"}
  - {id: FR-015-AC-98, path: *p81, role: examined, excerpt: &e67 "A graph claim (one reachability application of eit"}
  - {id: FR-015-AC-105, path: *p81, role: examined, excerpt: &e68 "For a graph claim the harness draws an object coun"}
  - {id: FR-015-AC-106, path: *p81, role: examined, excerpt: &e69 "The independent native oracle replays the same aut"}
  - {id: FR-015-AC-107, path: *p81, role: examined, excerpt: &e70 "Graph claims are located by `state_clause` node id"}
  - {id: FR-015-AC-108, path: *p81, role: examined, excerpt: &e71 "A graph claim settles by the disposition sequence "}
  - {id: FR-015-AC-110, path: *p81, role: examined, excerpt: &e72 "PLANNED (IR-598 collection slice; gated on IR-858,"}
  - {id: FR-015-AC-111, path: *p81, role: examined, excerpt: &e73 "PLANNED (IR-598 graph and final slice; gated on IR"}
  - {id: FR-015-AC-117, path: *p81, role: examined, excerpt: &e74 "Implemented (IR-858). With the installed backend, "}
  - {id: FR-015-AC-119, path: *p81, role: examined, excerpt: &e75 "PLANNED (IR-917; depends on FR-015-AC-40 and FR-01"}
  - {id: FR-015-AC-120, path: *p81, role: examined, excerpt: &e76 "PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 "}
  - {id: FR-015-AC-121, path: *p81, role: examined, excerpt: &e77 "PLANNED (IR-942). A per-step `KaniObligationIdenti"}
  - {id: FR-015-AC-122, path: *p81, role: examined, excerpt: &e78 "PLANNED (IR-942). A selected proof-eligible refine"}
  - {id: FR-015-AC-125, path: *p81, role: examined, excerpt: &e79 "Authentic QSL-emitted, IR-admitted bounded integer"}
  - {id: FR-015-AC-126, path: *p81, role: examined, excerpt: &e80 "Genuine TC-027 healthy and bound mutant, TC-025 ar"}
  - {id: FR-015-AC-127, path: *p81, role: examined, excerpt: &e81 "End-to-end denied/cancelled admitted-native contro"}
  - {id: FR-017-AC-7, path: &p92 "spec/kani/functional/FR-017-kani-execution-evidence.md", role: examined, excerpt: &e82 "A crate whose library source does not contain the "}
  - {id: FR-017-AC-19, path: *p92, role: examined, excerpt: &e83 "The generator appends `-Z unstable-options --expor"}
  - {id: FR-017-AC-26, path: *p92, role: examined, excerpt: &e84 "A generated FR-035 caller Text-admission harness e"}
  - {id: FR-017-AC-27, path: *p92, role: examined, excerpt: &e85 "A genuine routed `x + 1` scalar harness from `gene"}
  - {id: FR-017-AC-30, path: *p92, role: examined, excerpt: &e86 "The public receipt exposes the measured identity, "}
  - {id: FR-017-AC-47, path: *p92, role: examined, excerpt: &e87 "Two receipts for the same package and harness with"}
  - {id: FR-017-AC-52, path: *p92, role: examined, excerpt: &e88 "Production emitted bytes and feature-free exact de"}
  - {id: FR-017-AC-53, path: *p92, role: examined, excerpt: &e89 "Both package locks are independently freshly resol"}
  - {id: FR-017-AC-54, path: *p92, role: examined, excerpt: &e90 "A successful claimed-init confirmation poll with n"}
  - {id: FR-017-AC-55, path: *p92, role: examined, excerpt: &e91 "Through the real public execution entry and real o"}
  - {id: FR-017-CON-1, path: *p92, role: examined, excerpt: &e92 "The generator SHALL NOT convert a non-verified out"}
  - {id: FR-025-AC-1, path: &p101 "spec/kani/functional/FR-025-generated-subject-abi.md", role: examined, excerpt: &e93 "For every harness kind, the identifiers of the emi"}
  - {id: FR-025-AC-2, path: *p101, role: examined, excerpt: &e94 "Every argument binding in a persisted identity nam"}
  - {id: FR-025-AC-3, path: *p101, role: examined, excerpt: &e95 "A Boolean argument is a `bool` that reaches the or"}
  - {id: FR-025-AC-4, path: *p101, role: examined, excerpt: &e96 "An argument whose domain has no declared finite bo"}
  - {id: FR-025-AC-5, path: *p101, role: examined, excerpt: &e97 "A frame obligation is accounted `unsupported` with"}
  - {id: FR-025-AC-6, path: *p101, role: examined, excerpt: &e98 "A binding that names no parameter of the selected "}
  - {id: FR-025-AC-7, path: *p101, role: examined, excerpt: &e99 "An argument of a family with no row in the Rust-ty"}
  - {id: FR-025-AC-8, path: *p101, role: examined, excerpt: &e100 "A harness over a state-reading obligation passes e"}
  - {id: FR-025-AC-9, path: *p101, role: examined, excerpt: &e101 "PLANNED (IR-806). Every emitted composite shadow l"}
  - {id: FR-028-AC-5, path: &p117 "spec/kani/functional/FR-028-bounded-proof-ceilings.md", role: examined, excerpt: &e102 "Every harness identity, generation refusal and exe"}
  - {id: FR-028-AC-6, path: *p117, role: examined, excerpt: &e103 "A harness of a family with no supplied shadow reco"}
  - {id: FR-028-AC-7, path: *p117, role: examined, excerpt: &e104 "A family with a supplied shadow yields a harness w"}
  - {id: FR-028-AC-8, path: *p117, role: examined, excerpt: &e105 "An argument narrowed inside its declared domain re"}
  - {id: FR-028-AC-9, path: *p117, role: examined, excerpt: &e106 "A narrowing outside the argument's declared domain"}
  - {id: FR-028-AC-13, path: *p117, role: examined, excerpt: &e107 "The source of a `bounded_shadow` harness, parsed, "}
  - {id: FR-028-AC-14, path: *p117, role: examined, excerpt: &e108 "The result of a `bounded_shadow` family carries a "}
  - {id: FR-028-AC-15, path: *p117, role: examined, excerpt: &e109 "For every case the refinement run executes, the pr"}
  - {id: FR-028-AC-16, path: *p117, role: examined, excerpt: &e110 "A refinement run over a domain of at most the requ"}
  - {id: FR-028-AC-17, path: *p117, role: examined, excerpt: &e111 "A verified `production` harness reports `productio"}
  - {id: FR-028-AC-18, path: *p117, role: examined, excerpt: &e112 "A harness identity of a shadow family lists each a"}
  - {id: FR-028-AC-19, path: *p117, role: examined, excerpt: &e113 "A refinement domain that does not contain every ca"}
  - {id: FR-028-AC-20, path: *p117, role: examined, excerpt: &e114 "An item whose shape lies outside its family's shad"}
  - {id: FR-028-AC-22, path: *p117, role: examined, excerpt: &e115 "A shadow family's machinery catches each seeded mu"}
  - {id: FR-028-AC-23, path: *p117, role: examined, excerpt: &e116 "Every execution evidence records the Kani version,"}
  - {id: FR-028-AC-24, path: *p117, role: examined, excerpt: &e117 "The refinement run is a native process launched th"}
  - {id: FR-029-AC-17, path: &p128 "spec/kani/functional/FR-029-run-outcome-terminal-record.md", role: examined, excerpt: &e118 "PLANNED (IR-806). IR-635's Eq-only verified constr"}
  - {id: FR-029-AC-19, path: *p128, role: examined, excerpt: &e119 "PLANNED/GATED (IR-806/QSL-640). After the cross-ou"}
  - {id: FR-029-AC-21, path: *p128, role: examined, excerpt: &e120 "PLANNED (IR-806): `run_terminal_value` has no proo"}
  - {id: FR-029-AC-22, path: *p128, role: examined, excerpt: &e121 "PLANNED (IR-806). The strength set enumerated by t"}
  - {id: FR-029-AC-23, path: *p128, role: examined, excerpt: &e122 "PLANNED/GATED (IR-806/QSL-640). A stopped backend "}
  - {id: FR-029-AC-24, path: *p128, role: examined, excerpt: &e123 "PLANNED/GATED (IR-806/QSL-640). After common claim"}
  - {id: FR-029-AC-25, path: *p128, role: examined, excerpt: &e124 "PLANNED (IR-806). With no retained refinement disa"}
  - {id: FR-029-AC-26, path: *p128, role: examined, excerpt: &e125 "PLANNED/GATED (IR-806/QSL-640). The converter pres"}
  - {id: FR-029-AC-27, path: *p128, role: examined, excerpt: &e126 "PLANNED/GATED (IR-806/QSL-640). The distinct typed"}
  - {id: FR-029-AC-30, path: *p128, role: examined, excerpt: &e127 "PLANNED (IR-806). A `bounded_shadow` outcome (veri"}
  - {id: FR-029-AC-31, path: *p128, role: examined, excerpt: &e128 "PLANNED (IR-806). The parity route fabricates no p"}
  - {id: FR-030-AC-19, path: &p132 "spec/kani/functional/FR-030-ir-outcome-terminal-map.md", role: examined, excerpt: &e129 "For both `BoundedComplete { depth }` and `Inductiv"}
  - {id: FR-030-AC-20, path: *p132, role: examined, excerpt: &e130 "For either SMT basis, a foreign query, wrong certi"}
  - {id: FR-030-AC-21, path: *p132, role: examined, excerpt: &e131 "SMT/cross-check disagreement without certificate r"}
  - {id: FR-030-AC-22, path: *p132, role: examined, excerpt: &e132 "Every expressible accepted SMT backend/checker/rep"}
  - {id: FR-034-AC-1, path: &p215 "spec/kani/functional/FR-034-caller-death-ownership.md", role: examined, excerpt: &e133 "Before the exclusive caller lease, matching helper"}
  - {id: FR-034-AC-2, path: *p215, role: examined, excerpt: &e134 "Before any M spawn, O is positively verified outer"}
  - {id: FR-034-AC-3, path: *p215, role: examined, excerpt: &e135 "In ClaimedBootstrap, InitReady or Dispatched, orig"}
  - {id: FR-034-AC-4, path: *p215, role: examined, excerpt: &e136 "Mutual authority is the specific anonymous connect"}
  - {id: FR-034-AC-5, path: *p215, role: examined, excerpt: &e137 "C retains actual unreaped Child L; L/O/M/I ownersh"}
  - {id: FR-034-AC-6, path: *p215, role: examined, excerpt: &e138 "The owned tree memory observer is ready before typ"}
  - {id: FR-034-AC-7, path: *p215, role: examined, excerpt: &e139 "Bounded inner startup/identity recovery is require"}
  - {id: FR-034-AC-8, path: *p215, role: examined, excerpt: &e140 "After Dispatch, original-caller death cancels doub"}
  - {id: FR-034-AC-9, path: *p215, role: examined, excerpt: &e141 "Cancelling one owned namespace leaves concurrent i"}
  - {id: FR-034-AC-10, path: *p215, role: examined, excerpt: &e142 "Normal backend completion, backend exec failure, e"}
  - {id: FR-034-AC-11, path: *p215, role: examined, excerpt: &e143 "Guardian SIGKILL, abort or OOM-equivalent death du"}
  - {id: FR-034-AC-12, path: *p215, role: examined, excerpt: &e144 "Every startup refusal, completion and cancellation"}
  - {id: FR-034-AC-13, path: *p215, role: examined, excerpt: &e145 "The bounded executor requires an explicit path to "}
  - {id: FR-034-AC-14, path: *p215, role: examined, excerpt: &e146 "Setup docs require the matched package helper role"}
  - {id: FR-034-AC-15, path: *p215, role: examined, excerpt: &e147 "Typed private controls reject malformed and unknow"}
  - {id: FR-034-AC-16, path: *p215, role: examined, excerpt: &e148 "Only the original caller holds the pair's executor"}
  - {id: FR-034-AC-17, path: *p215, role: examined, excerpt: &e149 "Backend receives unchanged raw argument bytes exce"}
  - {id: FR-034-AC-18, path: *p215, role: examined, excerpt: &e150 "Guardian diagnostics and control use separate chan"}
  - {id: FR-034-AC-19, path: *p215, role: examined, excerpt: &e151 "Original identity ceilings and existing outcome cl"}
  - {id: FR-034-AC-20, path: *p215, role: examined, excerpt: &e152 "Guardian connection, startup, identity verificatio"}
  - {id: FR-034-AC-21, path: *p215, role: examined, excerpt: &e153 "Guardian connection and handshake have a finite se"}
  - {id: FR-034-AC-22, path: *p215, role: examined, excerpt: &e154 "Control/capture shutdown and cleanup observation w"}
  - {id: FR-034-AC-23, path: *p215, role: examined, excerpt: &e155 "Caller-death fixtures use the actual matching norm"}
  - {id: FR-034-AC-24, path: *p215, role: examined, excerpt: &e156 "Actual lease-EOF cancellation passes independent o"}
  - {id: FR-034-AC-25, path: *p215, role: examined, excerpt: &e157 "Before backend Dispatch, the helper handshake matc"}
  - {id: FR-034-AC-26, path: *p215, role: examined, excerpt: &e158 "The guardian session/group is distinct from the or"}
  - {id: FR-034-AC-27, path: *p215, role: examined, excerpt: &e159 "guardian-test-support is off by default, absent fr"}
  - {id: FR-034-AC-28, path: *p215, role: examined, excerpt: &e160 "Fixture prefix selects unchanged production transi"}
  - {id: FR-034-AC-29, path: *p215, role: examined, excerpt: &e161 "The packaged caller fixture and real helper link t"}
  - {id: FR-034-AC-31, path: *p215, role: examined, excerpt: &e162 "Real production fixtures fail the merged PR #295 r"}
  - {id: FR-034-AC-32, path: *p215, role: examined, excerpt: &e163 "The bounded O event loop collects anonymous report"}
  - {id: FR-034-AC-33, path: *p215, role: examined, excerpt: &e164 "Authenticated Completed precedes original-lease cl"}
  - {id: FR-034-AC-34, path: *p215, role: examined, excerpt: &e165 "Live-C production close_lease_and_observe retains "}
  - {id: FR-034-AC-35, path: *p215, role: examined, excerpt: &e166 "PLANNED/UNRUN. L creates private network/root befo"}
  - {id: FR-034-AC-36, path: *p215, role: examined, excerpt: &e167 "PLANNED/UNRUN. C inventories actual backend fd0/fd"}
  - {id: FR-034-AC-37, path: *p215, role: examined, excerpt: &e168 "PLANNED/UNRUN. Trusted I retains its exclusive lea"}
  - {id: FR-034-AC-38, path: *p215, role: examined, excerpt: &e169 "PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is mea"}
  - {id: FR-034-AC-39, path: *p215, role: examined, excerpt: &e170 "PLANNED/UNRUN. Native guardian/backend-installer p"}
  - {id: FR-034-AC-40, path: *p215, role: examined, excerpt: &e171 "PLANNED/UNRUN. Only declared trusted role/operatio"}
  - {id: FR-034-AC-51, path: *p215, role: examined, excerpt: &e172 "PLANNED/UNRUN. Given genuine live-I construction p"}
  - {id: FR-034-AC-52, path: *p215, role: examined, excerpt: &e173 "PLANNED/UNRUN. Actual retained-I pidfd termination"}
  - {id: FR-034-AC-53, path: *p215, role: examined, excerpt: &e174 "PLANNED/UNRUN. Actual retained M consuming Child w"}
  - {id: FR-034-AC-54, path: *p215, role: examined, excerpt: &e175 "PLANNED/UNRUN. On the independently grounded suppo"}
  - {id: FR-034-AC-57, path: *p215, role: examined, excerpt: &e176 "PLANNED/UNRUN (IR-687). When L awaits its first O "}
  - {id: FR-034-AC-58, path: *p215, role: examined, excerpt: &e177 "PLANNED/UNRUN (IR-687). L accepts that negative on"}
  - {id: FR-034-AC-59, path: *p215, role: examined, excerpt: &e178 "PLANNED/UNRUN (IR-687). For L\u2192C AwaitArm negatives"}
  - {id: FR-034-AC-60, path: *p215, role: examined, excerpt: &e179 "PLANNED/UNRUN (IR-687). The negative O pidfd grant"}
  - {id: FR-034-AC-61, path: *p215, role: examined, excerpt: &e180 "PLANNED/UNRUN (IR-687). Negative capability custod"}
  - {id: FR-034-AC-62, path: *p215, role: examined, excerpt: &e181 "PLANNED/UNRUN (IR-687). C adopts the authentic O p"}
  - {id: FR-034-AC-63, path: *p215, role: examined, excerpt: &e182 "PLANNED/UNRUN (IR-687). C exposes a negative origi"}
  - {id: FR-034-AC-65, path: *p215, role: examined, excerpt: &e183 "PLANNED/UNRUN (IR-687). Before first-byte COMMIT, "}
  - {id: FR-034-AC-66, path: *p215, role: examined, excerpt: &e184 "PLANNED/UNRUN (IR-687). Only the allocated L-local"}
  - {id: FR-034-AC-67, path: *p215, role: examined, excerpt: &e185 "PLANNED/UNRUN (IR-687). C authenticates the L-forw"}
  - {id: FR-034-AC-69, path: *p215, role: examined, excerpt: &e186 "PLANNED/UNRUN (IR-687). Missing or abnormal whole-"}
  - {id: FR-034-AC-70, path: *p215, role: examined, excerpt: &e187 "PLANNED/UNRUN (IR-687). The first emitted terminal"}
  - {id: FR-034-AC-71, path: *p215, role: examined, excerpt: &e188 "PLANNED/UNRUN (IR-687). O includes every due obser"}
  - {id: FR-034-AC-73, path: *p215, role: examined, excerpt: &e189 "PLANNED/UNRUN (IR-687). A claimed-startup negative"}
  - {id: FR-034-AC-74, path: *p215, role: examined, excerpt: &e190 "PLANNED/UNRUN (IR-687). The negative\u2019s absent peak"}
  - {id: FR-034-AC-75, path: *p215, role: examined, excerpt: &e191 "PLANNED/UNRUN (IR-687). A later genuine observatio"}
  - {id: FR-034-AC-76, path: *p215, role: examined, excerpt: &e192 "PLANNED/UNRUN (IR-687). A genuine admitted open-fi"}
  - {id: FR-034-AC-77, path: *p215, role: examined, excerpt: &e193 "PLANNED/UNRUN (IR-687). On a fresh complete Exhaus"}
  - {id: FR-034-AC-78, path: *p215, role: examined, excerpt: &e194 "PLANNED/UNRUN. C admits actual original O Armed on"}
  - {id: FR-034-AC-79, path: *p215, role: examined, excerpt: &e195 "PLANNED/UNRUN. Actual production control reception"}
  - {id: FR-034-AC-80, path: *p215, role: examined, excerpt: &e196 "PLANNED/UNRUN. Genuine original authenticated I Co"}
  - {id: FR-034-AC-81, path: *p215, role: examined, excerpt: &e197 "PLANNED/UNRUN. Every surviving actual simultaneous"}
  - {id: FR-034-AC-82, path: *p215, role: examined, excerpt: &e198 "PLANNED/UNRUN. Actual supported-source/artifact An"}
  - {id: FR-034-AC-83, path: *p215, role: examined, excerpt: &e199 "PLANNED/UNRUN. The original exclusive caller lease"}
  - {id: FR-034-AC-84, path: *p215, role: examined, excerpt: &e200 "PLANNED/UNRUN. Every remaining fixture reporter/co"}
  - {id: FR-034-AC-85, path: *p215, role: examined, excerpt: &e201 "PLANNED/UNRUN. Actual retained M consuming reap is"}
  - {id: FR-034-AC-86, path: *p215, role: examined, excerpt: &e202 "PLANNED/UNRUN. O establishes same-M parent-only pr"}
  - {id: FR-034-AC-87, path: *p215, role: examined, excerpt: &e203 "PLANNED/UNRUN. Actual I confirmation/M consuming r"}
  - {id: FR-034-AC-88, path: *p215, role: examined, excerpt: &e204 "PLANNED/UNRUN. O preserves every due ordinary samp"}
  - {id: FR-034-AC-89, path: *p215, role: examined, excerpt: &e205 "PLANNED/UNRUN. Genuine original I Completed recept"}
  - {id: FR-034-AC-90, path: *p215, role: examined, excerpt: &e206 "PLANNED/UNRUN. Any remaining optional fixture writ"}
  - {id: FR-034-AC-91, path: *p215, role: examined, excerpt: &e207 "PLANNED/UNRUN. Ordinary feature-on execution remai"}
  - {id: FR-034-AC-92, path: *p215, role: examined, excerpt: &e208 "PLANNED/UNRUN. Genuine adopted-orphan acknowledgem"}
  - {id: FR-034-AC-94, path: *p215, role: examined, excerpt: &e209 "PLANNED/UNRUN. C disposes the defined pending clai"}
  - {id: FR-034-AC-95, path: *p215, role: examined, excerpt: &e210 "PLANNED/UNRUN. Actual O retains the original repor"}
  - {id: FR-034-AC-96, path: *p215, role: examined, excerpt: &e211 "PLANNED/UNRUN. Missing, malformed, wrong or unvali"}
  - {id: FR-034-AC-97, path: *p215, role: examined, excerpt: &e212 "PLANNED/UNRUN. Retired observation-binding echo cr"}
  - {id: FR-034-AC-98, path: *p215, role: examined, excerpt: &e213 "PLANNED/UNRUN. Feature-off paired consumers/helper"}
  - {id: FR-034-AC-99, path: *p215, role: examined, excerpt: &e214 "PLANNED/UNRUN. Any remaining optional O fixture wr"}
  - {id: FR-034-AC-100, path: *p215, role: examined, excerpt: &e215 "PLANNED/UNRUN. Unsupported, malformed, unavailable"}
  - {id: FR-041-AC-4, path: &p216 "spec/kani/functional/FR-041-claimed-module-proof-gate.md", role: examined, excerpt: &e216 "Each claimed module and computed shared helper has"}
  - {id: NFR-006-M-1, path: &p219 "spec/kani/non-functional/NFR-006-real-kani-lane-gating.md", role: examined, excerpt: &e217 "Merged pull requests touching the Kani-touching se"}
  - {id: NFR-006-M-2, path: *p219, role: examined, excerpt: &e218 "Lane runs reported `passed` with `ran` different f"}
  - {id: NFR-006-M-3, path: *p219, role: examined, excerpt: &e219 "`#[ignore = \"kani lane: ...\"]` tests the lane does"}
  - {id: FR-014-AC-52, path: &p220 "spec/oracle/functional/FR-014-exact-scalar-oracles.md", role: examined, excerpt: &e220 "Every generated quantity operand is a QSV `UnitQua"}
  - {id: FR-018-AC-21, path: &p234 "spec/oracle/functional/FR-018-composite-equality-oracles.md", role: examined, excerpt: &e221 "Each generated item's claim-map entry records its "}
  - {id: FR-018-AC-22, path: *p234, role: examined, excerpt: &e222 "For every pair of operand values in the item's ref"}
  - {id: FR-018-AC-23, path: *p234, role: examined, excerpt: &e223 "For every item of the TC-029 corpus whose leaves a"}
  - {id: FR-018-AC-33, path: *p234, role: examined, excerpt: &e224 "A genuine root-facade compilation of authentic ret"}
  - {id: FR-018-AC-34, path: *p234, role: examined, excerpt: &e225 "The actual generated ABI executes ordered and unor"}
  - {id: FR-018-AC-35, path: *p234, role: examined, excerpt: &e226 "Unknown declaration, unknown case and a directly o"}
  - {id: FR-018-AC-36, path: *p234, role: examined, excerpt: &e227 "Emitted source uses only WireNodeId and case ident"}
  - {id: FR-018-AC-37, path: *p234, role: examined, excerpt: &e228 "Equality's generated manifest and native scratch c"}
  - {id: FR-018-AC-38, path: *p234, role: examined, excerpt: &e229 "With `None`, standalone generation and emitted pre"}
  - {id: FR-018-AC-39, path: *p234, role: examined, excerpt: &e230 "`None` at the first reached nominal identity retur"}
  - {id: FR-018-AC-40, path: *p234, role: examined, excerpt: &e231 "Some(wrong-package context) fails the whole standa"}
  - {id: FR-018-AC-41, path: *p234, role: examined, excerpt: &e232 "A reached enum still reconstructs every owner-admi"}
  - {id: FR-018-AC-42, path: *p234, role: examined, excerpt: &e233 "Generated oracle invocations under Some or valid c"}
  - {id: FR-018-AC-43, path: *p234, role: examined, excerpt: &e234 "For genuine source with a: Int[0,9] and b: Integer"}
  - {id: FR-021-AC-30, path: &p235 "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: &e235 "A requested generation work limit of `u64::MAX` re"}
  - {id: FR-031-AC-2, path: &p246 "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md", role: examined, excerpt: &e236 "PLANNED (IR-602), GATED on the lock prerequisite. "}
  - {id: FR-031-AC-6, path: *p246, role: examined, excerpt: &e237 "PLANNED (IR-602), GATED on the lock prerequisite. "}
  - {id: FR-031-AC-16, path: *p246, role: examined, excerpt: &e238 "PLANNED (IR-602), GATED on the lock prerequisite. "}
  - {id: FR-031-AC-17, path: *p246, role: examined, excerpt: &e239 "PLANNED (IR-602), GATED on the lock prerequisite. "}
  - {id: FR-031-AC-27, path: *p246, role: examined, excerpt: &e240 "PLANNED (IR-602), GATED on the lock prerequisite ("}
  - {id: FR-031-AC-20, path: *p246, role: examined, excerpt: &e241 "IMPLEMENTED (IR-596). Under real Kani, a harness t"}
  - {id: FR-031-AC-22, path: *p246, role: examined, excerpt: &e242 "PLANNED (IR-602), CONTINGENT on OD-2 and on the lo"}
  - {id: FR-031-AC-26, path: *p246, role: examined, excerpt: &e243 "PLANNED (IR-602), CONTINGENT on OD-2 and on the lo"}
  - {id: FR-031-AC-23, path: *p246, role: examined, excerpt: &e244 "PLANNED (IR-602). The Kani bundle oracle renders D"}
  - {id: FR-031-AC-24, path: *p246, role: examined, excerpt: &e245 "PLANNED (IR-602). Under real Kani, a harness that "}
  - {id: FR-031-AC-25, path: *p246, role: examined, excerpt: &e246 "PLANNED (IR-602), GATED on the lock prerequisite ("}
  - {id: FR-035-AC-1, path: &p252 "spec/oracle/functional/FR-035-caller-text-admission.md", role: examined, excerpt: &e247 "A generated caller-ingress oracle accepts `&TextPa"}
  - {id: FR-035-AC-2, path: *p252, role: examined, excerpt: &e248 "Invalid UTF-8 bytes make `TextPayload::from_utf8` "}
  - {id: FR-035-AC-3, path: *p252, role: examined, excerpt: &e249 "For each profile, `Text[1,1]` admits or refuses ac"}
  - {id: FR-035-AC-4, path: *p252, role: examined, excerpt: &e250 "For each of the six profiles and both `Text[0,8]` "}
  - {id: FR-035-AC-5, path: *p252, role: examined, excerpt: &e251 "In a copy of the generated oracle with its admissi"}
  - {id: FR-035-AC-6, path: *p252, role: examined, excerpt: &e252 "A request outside the declared finite proof class "}
  - {id: FR-042-AC-1, path: &p253 "spec/oracle/functional/FR-042-generated-checked-invariant-causes.md", role: examined, excerpt: &e253 "CG's native Boolean, equality, function and pre-ch"}
  - {id: FR-024-AC-1, path: &p264 "spec/replay/functional/FR-024-counterexample-envelope-intake.md", role: examined, excerpt: &e254 "The envelope's obligation identity is QSL's `Oblig"}
  - {id: FR-024-AC-7, path: *p264, role: examined, excerpt: &e255 "A reduced `Witness`-arm candidate is retained only"}
  - {id: FR-024-AC-8, path: *p264, role: examined, excerpt: &e256 "A retained reduced `Witness`-arm envelope carries "}
  - {id: FR-024-AC-18, path: *p264, role: examined, excerpt: &e257 "With the installed backend, the falsified operatio"}
  - {id: FR-024-AC-34, path: *p264, role: examined, excerpt: &e258 "PLANNED (IR-624), IR-628 accessor merged; CG depen"}
  - {id: FR-024-AC-35, path: *p264, role: examined, excerpt: &e259 "PLANNED (IR-624), IR-628 accessor merged; CG depen"}
  - {id: FR-024-AC-36, path: *p264, role: examined, excerpt: &e260 "Two distinct test backend adapters each produce a "}
  - {id: FR-024-AC-37, path: *p264, role: examined, excerpt: &e261 "The common intake maps an adapter's decode-refusal"}
  - {id: FR-024-AC-38, path: *p264, role: examined, excerpt: &e262 "Starting from one admitted original failure, one v"}
  - {id: FR-024-AC-39, path: *p264, role: examined, excerpt: &e263 "From a retained failure revision, an out-of-domain"}
  - {id: FR-024-AC-40, path: *p264, role: examined, excerpt: &e264 "A retained reduced `Witness` envelope holds a tran"}
  - {id: FR-032-AC-1, path: &p277 "spec/replay/functional/FR-032-routed-scalar-replay-binding.md", role: examined, excerpt: &e265 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-3, path: *p277, role: examined, excerpt: &e266 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-4, path: *p277, role: examined, excerpt: &e267 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-5, path: *p277, role: examined, excerpt: &e268 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-6, path: *p277, role: examined, excerpt: &e269 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-7, path: *p277, role: examined, excerpt: &e270 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-8, path: *p277, role: examined, excerpt: &e271 "PLANNED (IR-631) / CODE-GATED (CG retention, obser"}
  - {id: FR-032-AC-9, path: *p277, role: examined, excerpt: &e272 "PLANNED (IR-631) / CODE-GATED (authoritative metad"}
  - {id: FR-032-AC-10, path: *p277, role: examined, excerpt: &e273 "PLANNED (IR-631) / CODE-GATED (typed scalar-family"}
  - {id: FR-032-AC-11, path: *p277, role: examined, excerpt: &e274 "PLANNED (IR-631) / CODE-GATED (typed scalar-family"}
  - {id: FR-032-AC-12, path: *p277, role: examined, excerpt: &e275 "PLANNED (IR-631) / CODE-GATED (typed scalar-family"}
  - {id: FR-032-AC-13, path: *p277, role: examined, excerpt: &e276 "PLANNED (IR-631) / CODE-GATED (typed scalar-family"}
  - {id: FR-032-AC-14, path: *p277, role: examined, excerpt: &e277 "PLANNED (IR-631) / CODE-GATED (actual CG scalar bu"}
  - {id: FR-033-AC-2, path: &p287 "spec/replay/functional/FR-033-composite-parity-replay-binding.md", role: examined, excerpt: &e278 "PLANNED/GATED. Canonical round trips preserve ever"}
  - {id: FR-033-AC-3, path: *p287, role: examined, excerpt: &e279 "PLANNED/GATED. Noncanonical JCS/tag/scalar/escape "}
  - {id: FR-033-AC-4, path: *p287, role: examined, excerpt: &e280 "PLANNED/GATED. Public caller input exactly at the "}
  - {id: FR-033-AC-5, path: *p287, role: examined, excerpt: &e281 "PLANNED/GATED. The retained actual assertion playb"}
  - {id: FR-033-AC-6, path: *p287, role: examined, excerpt: &e282 "PLANNED/GATED. For falsified replay the driver exe"}
  - {id: FR-033-AC-8, path: *p287, role: examined, excerpt: &e283 "PLANNED/GATED. CG submits the original claim's ope"}
  - {id: FR-033-AC-10, path: *p287, role: examined, excerpt: &e284 "PLANNED/GATED. Each lawful generated family exerci"}
  - {id: FR-033-AC-14, path: *p287, role: examined, excerpt: &e285 "PLANNED (IR-814; literal operands are admitted one"}
  - {id: FR-033-AC-15, path: *p287, role: examined, excerpt: &e286 "PLANNED (IR-814). When the original application no"}
  - {id: FR-033-AC-17, path: *p287, role: examined, excerpt: &e287 "PLANNED (IR-814). If the owning operand projection"}
  - {id: FR-019-AC-3, path: &p298 "spec/routed/functional/FR-019-capability-settlement.md", role: examined, excerpt: &e288 "On the Kani arm, an unbounded extent against a `bo"}
  - {id: FR-019-AC-15, path: *p298, role: examined, excerpt: &e289 "PLANNED (IR-633). CG's `BackendDescriptor` has a t"}
  - {id: FR-019-AC-24, path: *p298, role: examined, excerpt: &e290 "PLANNED (IR-338). Given QSL FR-335's `value-validi"}
  - {id: FR-019-AC-34, path: *p298, role: examined, excerpt: &e291 "PLANNED (IR-516). With one EN-1 candidate, an adve"}
  - {id: FR-019-AC-35, path: *p298, role: examined, excerpt: &e292 "PLANNED (IR-516). With one SMT temporal candidate "}
  - {id: FR-019-AC-36, path: *p298, role: examined, excerpt: &e293 "PLANNED (IR-516). A TP-5 whole-refinement `tempora"}
  - {id: FR-019-AC-37, path: *p298, role: examined, excerpt: &e294 "PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 "}
  - {id: FR-019-AC-38, path: *p298, role: examined, excerpt: &e295 "PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3"}
  - {id: FR-019-AC-39, path: *p298, role: examined, excerpt: &e296 "PLANNED (IR-516). A selected reduction reaches onl"}
  - {id: FR-019-AC-40, path: *p298, role: examined, excerpt: &e297 "PLANNED (IR-516). For an item over a weak `paralle"}
  - {id: FR-019-AC-41, path: *p298, role: examined, excerpt: &e298 "PLANNED (IR-516). With otherwise equal form and ki"}
  - {id: FR-022-AC-26, path: &p301 "spec/routed/functional/FR-022-routed-generation.md", role: examined, excerpt: &e299 "Given the checked QSL RR-16 package with two `x + "}
  - {id: FR-022-AC-27, path: *p301, role: examined, excerpt: &e300 "For `if p then x + 1 else x + 1`, each occurrence'"}
  - {id: FR-022-AC-28, path: *p301, role: examined, excerpt: &e301 "If a routed scalar item's occurrence has no entry "}
  - {id: FR-026-AC-1, path: &p303 "spec/routed/functional/FR-026-backend-adapter-contract.md", role: examined, excerpt: &e302 "The Kani adapter implements the adapter trait, and"}
  - {id: FR-026-AC-4, path: *p303, role: examined, excerpt: &e303 "The execution evidence type is an associated type "}
  - {id: FR-002-AC-7, path: &p314 "spec/strategy/functional/FR-002-tristate-proptest.md", role: examined, excerpt: &e304 "PLANNED (IR-815). For one precondition and one pos"}
  - {id: FR-002-AC-8, path: *p314, role: examined, excerpt: &e305 "PLANNED (IR-815). In the V2 harness, an instrument"}
  - {id: FR-002-AC-10, path: *p314, role: examined, excerpt: &e306 "PLANNED (IR-815). A selected clause identity that "}
  - {id: FR-002-AC-11, path: *p314, role: examined, excerpt: &e307 "PLANNED (IR-815). A selected clause body outside t"}
  - {id: FR-002-AC-12, path: *p314, role: examined, excerpt: &e308 "PLANNED (IR-815). Repeated V2 harness generation f"}
  - {id: FR-002-AC-13, path: *p314, role: examined, excerpt: &e309 "PLANNED (IR-815). A seeded V2 case on which both o"}
  - {id: FR-002-AC-14, path: *p314, role: examined, excerpt: &e310 "PLANNED (IR-815). A seeded V2 case on which the pr"}
  - {id: FR-002-AC-15, path: *p314, role: examined, excerpt: &e311 "PLANNED (IR-815). A seeded V2 case on which the pr"}
  - {id: FR-002-AC-16, path: *p314, role: examined, excerpt: &e312 "PLANNED (IR-815). A seeded V2 case on which the pr"}
  - {id: FR-002-AC-17, path: *p314, role: examined, excerpt: &e313 "PLANNED (IR-815). A seeded V2 case on which the pr"}
  - {id: FR-002-AC-18, path: *p314, role: examined, excerpt: &e314 "PLANNED (IR-815). A seeded V2 case on which the pr"}
  - {id: FR-008-AC-7, path: &p316 "spec/strategy/functional/FR-008-bound-domain-strategy-admission.md", role: examined, excerpt: &e315 "PLANNED. Beyond the per-operator agreement stated "}
  - {id: FR-008-AC-9, path: *p316, role: examined, excerpt: &e316 "PLANNED. A CG integration test calls IR's public c"}
  - {id: FR-009-AC-7, path: &p317 "spec/strategy/functional/FR-009-constructive-correlated-populations.md", role: examined, excerpt: &e317 "PLANNED. For each noncommutative operator, a compa"}
  - {id: NFR-004-M-1, path: &p321 "spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md", role: examined, excerpt: &e318 "Discarded / attempted on a fresh 10,000-case campa"}
  - {id: NFR-004-M-2, path: *p321, role: examined, excerpt: &e319 "Proptest global rejects on the same campaigns"}
  - {id: NFR-004-M-3, path: *p321, role: examined, excerpt: &e320 "Campaigns ending `Exhausted` on the same campaigns"}
  - {id: NFR-004-M-4, path: *p321, role: examined, excerpt: &e321 "In-domain plus out-of-domain census cases per admi"}
defaults: &defaults {severity: medium, confidence: high, method: gap-analysis, check: coverage, related_ids: []}
findings:
  - {<<: *defaults, fnd: FND-001, artifact_id: FR-005-AC-3, path: *p0, lines: "49", excerpt: *e0, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-002, artifact_id: FR-039-AC-4, path: *p1, lines: "213", excerpt: *e1, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-003, artifact_id: FR-043-AC-1, path: *p5, lines: "31", excerpt: *e2, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-004, artifact_id: FR-043-AC-2, path: *p5, lines: "32", excerpt: *e3, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-005, artifact_id: FR-043-AC-3, path: *p5, lines: "33", excerpt: *e4, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-006, artifact_id: FR-043-AC-5, path: *p5, lines: "35", excerpt: *e5, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-007, artifact_id: NFR-001-M-1, path: *p6, lines: "1", excerpt: *e6, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-008, artifact_id: NFR-002-M-1, path: *p8, lines: "1", excerpt: *e7, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-009, artifact_id: NFR-002-M-3, path: *p8, lines: "1", excerpt: *e8, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-010, artifact_id: FR-004-AC-6, path: *p17, lines: "186", excerpt: *e9, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-011, artifact_id: FR-004-AC-8, path: *p17, lines: "188", excerpt: *e10, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-012, artifact_id: FR-004-AC-10, path: *p17, lines: "190", excerpt: *e11, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-013, artifact_id: FR-004-AC-11, path: *p17, lines: "191", excerpt: *e12, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-014, artifact_id: FR-004-AC-12, path: *p17, lines: "192", excerpt: *e13, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-015, artifact_id: FR-004-AC-13, path: *p17, lines: "193", excerpt: *e14, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-016, artifact_id: FR-004-AC-14, path: *p17, lines: "194", excerpt: *e15, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-017, artifact_id: FR-004-AC-15, path: *p17, lines: "195", excerpt: *e16, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-018, artifact_id: FR-004-AC-18, path: *p17, lines: "198", excerpt: *e17, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-019, artifact_id: FR-036-AC-1, path: *p24, lines: "140", excerpt: *e18, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-020, artifact_id: FR-036-AC-2, path: *p24, lines: "141", excerpt: *e19, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-021, artifact_id: FR-036-AC-3, path: *p24, lines: "142", excerpt: *e20, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-022, artifact_id: FR-036-AC-4, path: *p24, lines: "143", excerpt: *e21, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-023, artifact_id: FR-036-AC-5, path: *p24, lines: "144", excerpt: *e22, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-024, artifact_id: FR-036-AC-6, path: *p24, lines: "145", excerpt: *e23, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-025, artifact_id: FR-036-AC-7, path: *p24, lines: "146", excerpt: *e24, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-026, artifact_id: FR-015-AC-19, path: *p81, lines: "763", excerpt: *e25, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-027, artifact_id: FR-015-AC-21, path: *p81, lines: "765", excerpt: *e26, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-028, artifact_id: FR-015-AC-22, path: *p81, lines: "39", excerpt: *e27, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-029, artifact_id: FR-015-AC-23, path: *p81, lines: "469", excerpt: *e28, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-030, artifact_id: FR-015-AC-24, path: *p81, lines: "768", excerpt: *e29, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-031, artifact_id: FR-015-AC-30, path: *p81, lines: "774", excerpt: *e30, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-032, artifact_id: FR-015-AC-31, path: *p81, lines: "775", excerpt: *e31, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-033, artifact_id: FR-015-AC-32, path: *p81, lines: "776", excerpt: *e32, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-034, artifact_id: FR-015-AC-38, path: *p81, lines: "38", excerpt: *e33, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-035, artifact_id: FR-015-AC-39, path: *p81, lines: "402", excerpt: *e34, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-036, artifact_id: FR-015-AC-40, path: *p81, lines: "406", excerpt: *e35, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-037, artifact_id: FR-015-AC-41, path: *p81, lines: "406", excerpt: *e36, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-038, artifact_id: FR-015-AC-42, path: *p81, lines: "413", excerpt: *e37, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-039, artifact_id: FR-015-AC-43, path: *p81, lines: "416", excerpt: *e38, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-040, artifact_id: FR-015-AC-44, path: *p81, lines: "41", excerpt: *e39, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-041, artifact_id: FR-015-AC-45, path: *p81, lines: "41", excerpt: *e40, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-042, artifact_id: FR-015-AC-46, path: *p81, lines: "424", excerpt: *e41, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-043, artifact_id: FR-015-AC-47, path: *p81, lines: "426", excerpt: *e42, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-044, artifact_id: FR-015-AC-48, path: *p81, lines: "429", excerpt: *e43, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-045, artifact_id: FR-015-AC-49, path: *p81, lines: "39", excerpt: *e44, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-046, artifact_id: FR-015-AC-51, path: *p81, lines: "442", excerpt: *e45, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-047, artifact_id: FR-015-AC-52, path: *p81, lines: "445", excerpt: *e46, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-048, artifact_id: FR-015-AC-56, path: *p81, lines: "453", excerpt: *e47, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-049, artifact_id: FR-015-AC-57, path: *p81, lines: "458", excerpt: *e48, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-050, artifact_id: FR-015-AC-69, path: *p81, lines: "55", excerpt: *e49, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-051, artifact_id: FR-015-AC-70, path: *p81, lines: "680", excerpt: *e50, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-052, artifact_id: FR-015-AC-71, path: *p81, lines: "683", excerpt: *e51, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-053, artifact_id: FR-015-AC-72, path: *p81, lines: "686", excerpt: *e52, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-054, artifact_id: FR-015-AC-73, path: *p81, lines: "58", excerpt: *e53, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-055, artifact_id: FR-015-AC-74, path: *p81, lines: "690", excerpt: *e54, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-056, artifact_id: FR-015-AC-75, path: *p81, lines: "692", excerpt: *e55, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-057, artifact_id: FR-015-AC-76, path: *p81, lines: "55", excerpt: *e56, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-058, artifact_id: FR-015-AC-80, path: *p81, lines: "824", excerpt: *e57, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-059, artifact_id: FR-015-AC-84, path: *p81, lines: "828", excerpt: *e58, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-060, artifact_id: FR-015-AC-85, path: *p81, lines: "829", excerpt: *e59, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-061, artifact_id: FR-015-AC-87, path: *p81, lines: "831", excerpt: *e60, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-062, artifact_id: FR-015-AC-89, path: *p81, lines: "833", excerpt: *e61, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-063, artifact_id: FR-015-AC-90, path: *p81, lines: "834", excerpt: *e62, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-064, artifact_id: FR-015-AC-91, path: *p81, lines: "835", excerpt: *e63, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-065, artifact_id: FR-015-AC-92, path: *p81, lines: "836", excerpt: *e64, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-066, artifact_id: FR-015-AC-93, path: *p81, lines: "837", excerpt: *e65, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-067, artifact_id: FR-015-AC-95, path: *p81, lines: "88", excerpt: *e66, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-068, artifact_id: FR-015-AC-98, path: *p81, lines: "138", excerpt: *e67, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-069, artifact_id: FR-015-AC-105, path: *p81, lines: "849", excerpt: *e68, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-070, artifact_id: FR-015-AC-106, path: *p81, lines: "104", excerpt: *e69, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-071, artifact_id: FR-015-AC-107, path: *p81, lines: "851", excerpt: *e70, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-072, artifact_id: FR-015-AC-108, path: *p81, lines: "852", excerpt: *e71, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-073, artifact_id: FR-015-AC-110, path: *p81, lines: "87", excerpt: *e72, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-074, artifact_id: FR-015-AC-111, path: *p81, lines: "87", excerpt: *e73, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-075, artifact_id: FR-015-AC-117, path: *p81, lines: "839", excerpt: *e74, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-076, artifact_id: FR-015-AC-119, path: *p81, lines: "373", excerpt: *e75, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-077, artifact_id: FR-015-AC-120, path: *p81, lines: "864", excerpt: *e76, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-078, artifact_id: FR-015-AC-121, path: *p81, lines: "865", excerpt: *e77, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-079, artifact_id: FR-015-AC-122, path: *p81, lines: "866", excerpt: *e78, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-080, artifact_id: FR-015-AC-125, path: *p81, lines: "869", excerpt: *e79, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-081, artifact_id: FR-015-AC-126, path: *p81, lines: "870", excerpt: *e80, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-082, artifact_id: FR-015-AC-127, path: *p81, lines: "871", excerpt: *e81, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-083, artifact_id: FR-017-AC-7, path: *p92, lines: "859", excerpt: *e82, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-084, artifact_id: FR-017-AC-19, path: *p92, lines: "868", excerpt: *e83, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-085, artifact_id: FR-017-AC-26, path: *p92, lines: "875", excerpt: *e84, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-086, artifact_id: FR-017-AC-27, path: *p92, lines: "650", excerpt: *e85, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-087, artifact_id: FR-017-AC-30, path: *p92, lines: "879", excerpt: *e86, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-088, artifact_id: FR-017-AC-47, path: *p92, lines: "896", excerpt: *e87, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-089, artifact_id: FR-017-AC-52, path: *p92, lines: "901", excerpt: *e88, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-090, artifact_id: FR-017-AC-53, path: *p92, lines: "902", excerpt: *e89, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-091, artifact_id: FR-017-AC-54, path: *p92, lines: "903", excerpt: *e90, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-092, artifact_id: FR-017-AC-55, path: *p92, lines: "904", excerpt: *e91, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-093, artifact_id: FR-017-CON-1, path: *p92, lines: "848", excerpt: *e92, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-094, artifact_id: FR-025-AC-1, path: *p101, lines: "94", excerpt: *e93, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-095, artifact_id: FR-025-AC-2, path: *p101, lines: "95", excerpt: *e94, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-096, artifact_id: FR-025-AC-3, path: *p101, lines: "96", excerpt: *e95, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-097, artifact_id: FR-025-AC-4, path: *p101, lines: "97", excerpt: *e96, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-098, artifact_id: FR-025-AC-5, path: *p101, lines: "98", excerpt: *e97, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-099, artifact_id: FR-025-AC-6, path: *p101, lines: "99", excerpt: *e98, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-100, artifact_id: FR-025-AC-7, path: *p101, lines: "100", excerpt: *e99, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-101, artifact_id: FR-025-AC-8, path: *p101, lines: "101", excerpt: *e100, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-102, artifact_id: FR-025-AC-9, path: *p101, lines: "102", excerpt: *e101, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-103, artifact_id: FR-028-AC-5, path: *p117, lines: "206", excerpt: *e102, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-104, artifact_id: FR-028-AC-6, path: *p117, lines: "207", excerpt: *e103, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-105, artifact_id: FR-028-AC-7, path: *p117, lines: "208", excerpt: *e104, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-106, artifact_id: FR-028-AC-8, path: *p117, lines: "209", excerpt: *e105, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-107, artifact_id: FR-028-AC-9, path: *p117, lines: "210", excerpt: *e106, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-108, artifact_id: FR-028-AC-13, path: *p117, lines: "33", excerpt: *e107, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-109, artifact_id: FR-028-AC-14, path: *p117, lines: "81", excerpt: *e108, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-110, artifact_id: FR-028-AC-15, path: *p117, lines: "143", excerpt: *e109, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-111, artifact_id: FR-028-AC-16, path: *p117, lines: "70", excerpt: *e110, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-112, artifact_id: FR-028-AC-17, path: *p117, lines: "81", excerpt: *e111, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-113, artifact_id: FR-028-AC-18, path: *p117, lines: "84", excerpt: *e112, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-114, artifact_id: FR-028-AC-19, path: *p117, lines: "163", excerpt: *e113, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-115, artifact_id: FR-028-AC-20, path: *p117, lines: "168", excerpt: *e114, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-116, artifact_id: FR-028-AC-22, path: *p117, lines: "221", excerpt: *e115, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-117, artifact_id: FR-028-AC-23, path: *p117, lines: "178", excerpt: *e116, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-118, artifact_id: FR-028-AC-24, path: *p117, lines: "33", excerpt: *e117, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-119, artifact_id: FR-029-AC-17, path: *p128, lines: "377", excerpt: *e118, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-120, artifact_id: FR-029-AC-19, path: *p128, lines: "379", excerpt: *e119, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-121, artifact_id: FR-029-AC-21, path: *p128, lines: "381", excerpt: *e120, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-122, artifact_id: FR-029-AC-22, path: *p128, lines: "382", excerpt: *e121, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-123, artifact_id: FR-029-AC-23, path: *p128, lines: "383", excerpt: *e122, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-124, artifact_id: FR-029-AC-24, path: *p128, lines: "384", excerpt: *e123, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-125, artifact_id: FR-029-AC-25, path: *p128, lines: "385", excerpt: *e124, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-126, artifact_id: FR-029-AC-26, path: *p128, lines: "386", excerpt: *e125, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-127, artifact_id: FR-029-AC-27, path: *p128, lines: "387", excerpt: *e126, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-128, artifact_id: FR-029-AC-30, path: *p128, lines: "207", excerpt: *e127, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-129, artifact_id: FR-029-AC-31, path: *p128, lines: "391", excerpt: *e128, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-130, artifact_id: FR-030-AC-19, path: *p132, lines: "237", excerpt: *e129, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-131, artifact_id: FR-030-AC-20, path: *p132, lines: "238", excerpt: *e130, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-132, artifact_id: FR-030-AC-21, path: *p132, lines: "239", excerpt: *e131, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-133, artifact_id: FR-030-AC-22, path: *p132, lines: "240", excerpt: *e132, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-134, artifact_id: FR-034-AC-1, path: *p215, lines: "1844", excerpt: *e133, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-135, artifact_id: FR-034-AC-2, path: *p215, lines: "1845", excerpt: *e134, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-136, artifact_id: FR-034-AC-3, path: *p215, lines: "1846", excerpt: *e135, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-137, artifact_id: FR-034-AC-4, path: *p215, lines: "1847", excerpt: *e136, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-138, artifact_id: FR-034-AC-5, path: *p215, lines: "1848", excerpt: *e137, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-139, artifact_id: FR-034-AC-6, path: *p215, lines: "1849", excerpt: *e138, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-140, artifact_id: FR-034-AC-7, path: *p215, lines: "1850", excerpt: *e139, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-141, artifact_id: FR-034-AC-8, path: *p215, lines: "1851", excerpt: *e140, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-142, artifact_id: FR-034-AC-9, path: *p215, lines: "1852", excerpt: *e141, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-143, artifact_id: FR-034-AC-10, path: *p215, lines: "1853", excerpt: *e142, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-144, artifact_id: FR-034-AC-11, path: *p215, lines: "1854", excerpt: *e143, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-145, artifact_id: FR-034-AC-12, path: *p215, lines: "1855", excerpt: *e144, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-146, artifact_id: FR-034-AC-13, path: *p215, lines: "1856", excerpt: *e145, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-147, artifact_id: FR-034-AC-14, path: *p215, lines: "1857", excerpt: *e146, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-148, artifact_id: FR-034-AC-15, path: *p215, lines: "1858", excerpt: *e147, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-149, artifact_id: FR-034-AC-16, path: *p215, lines: "1859", excerpt: *e148, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-150, artifact_id: FR-034-AC-17, path: *p215, lines: "1860", excerpt: *e149, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-151, artifact_id: FR-034-AC-18, path: *p215, lines: "1861", excerpt: *e150, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-152, artifact_id: FR-034-AC-19, path: *p215, lines: "1862", excerpt: *e151, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-153, artifact_id: FR-034-AC-20, path: *p215, lines: "1863", excerpt: *e152, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-154, artifact_id: FR-034-AC-21, path: *p215, lines: "1864", excerpt: *e153, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-155, artifact_id: FR-034-AC-22, path: *p215, lines: "1865", excerpt: *e154, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-156, artifact_id: FR-034-AC-23, path: *p215, lines: "1866", excerpt: *e155, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-157, artifact_id: FR-034-AC-24, path: *p215, lines: "1867", excerpt: *e156, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-158, artifact_id: FR-034-AC-25, path: *p215, lines: "1868", excerpt: *e157, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-159, artifact_id: FR-034-AC-26, path: *p215, lines: "1869", excerpt: *e158, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-160, artifact_id: FR-034-AC-27, path: *p215, lines: "1870", excerpt: *e159, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-161, artifact_id: FR-034-AC-28, path: *p215, lines: "1871", excerpt: *e160, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-162, artifact_id: FR-034-AC-29, path: *p215, lines: "1872", excerpt: *e161, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-163, artifact_id: FR-034-AC-31, path: *p215, lines: "1874", excerpt: *e162, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-164, artifact_id: FR-034-AC-32, path: *p215, lines: "1875", excerpt: *e163, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-165, artifact_id: FR-034-AC-33, path: *p215, lines: "1876", excerpt: *e164, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-166, artifact_id: FR-034-AC-34, path: *p215, lines: "1877", excerpt: *e165, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-167, artifact_id: FR-034-AC-35, path: *p215, lines: "1878", excerpt: *e166, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-168, artifact_id: FR-034-AC-36, path: *p215, lines: "1879", excerpt: *e167, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-169, artifact_id: FR-034-AC-37, path: *p215, lines: "1880", excerpt: *e168, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-170, artifact_id: FR-034-AC-38, path: *p215, lines: "1881", excerpt: *e169, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-171, artifact_id: FR-034-AC-39, path: *p215, lines: "1882", excerpt: *e170, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-172, artifact_id: FR-034-AC-40, path: *p215, lines: "1883", excerpt: *e171, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-173, artifact_id: FR-034-AC-51, path: *p215, lines: "1894", excerpt: *e172, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-174, artifact_id: FR-034-AC-52, path: *p215, lines: "1895", excerpt: *e173, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-175, artifact_id: FR-034-AC-53, path: *p215, lines: "1896", excerpt: *e174, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-176, artifact_id: FR-034-AC-54, path: *p215, lines: "1897", excerpt: *e175, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-177, artifact_id: FR-034-AC-57, path: *p215, lines: "1900", excerpt: *e176, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-178, artifact_id: FR-034-AC-58, path: *p215, lines: "1901", excerpt: *e177, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-179, artifact_id: FR-034-AC-59, path: *p215, lines: "1902", excerpt: *e178, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-180, artifact_id: FR-034-AC-60, path: *p215, lines: "1903", excerpt: *e179, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-181, artifact_id: FR-034-AC-61, path: *p215, lines: "1904", excerpt: *e180, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-182, artifact_id: FR-034-AC-62, path: *p215, lines: "1905", excerpt: *e181, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-183, artifact_id: FR-034-AC-63, path: *p215, lines: "1906", excerpt: *e182, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-184, artifact_id: FR-034-AC-65, path: *p215, lines: "1908", excerpt: *e183, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-185, artifact_id: FR-034-AC-66, path: *p215, lines: "1909", excerpt: *e184, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-186, artifact_id: FR-034-AC-67, path: *p215, lines: "1910", excerpt: *e185, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-187, artifact_id: FR-034-AC-69, path: *p215, lines: "1912", excerpt: *e186, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-188, artifact_id: FR-034-AC-70, path: *p215, lines: "1913", excerpt: *e187, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-189, artifact_id: FR-034-AC-71, path: *p215, lines: "1914", excerpt: *e188, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-190, artifact_id: FR-034-AC-73, path: *p215, lines: "1916", excerpt: *e189, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-191, artifact_id: FR-034-AC-74, path: *p215, lines: "1917", excerpt: *e190, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-192, artifact_id: FR-034-AC-75, path: *p215, lines: "1918", excerpt: *e191, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-193, artifact_id: FR-034-AC-76, path: *p215, lines: "1919", excerpt: *e192, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-194, artifact_id: FR-034-AC-77, path: *p215, lines: "1920", excerpt: *e193, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-195, artifact_id: FR-034-AC-78, path: *p215, lines: "1921", excerpt: *e194, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-196, artifact_id: FR-034-AC-79, path: *p215, lines: "1922", excerpt: *e195, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-197, artifact_id: FR-034-AC-80, path: *p215, lines: "1923", excerpt: *e196, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-198, artifact_id: FR-034-AC-81, path: *p215, lines: "1924", excerpt: *e197, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-199, artifact_id: FR-034-AC-82, path: *p215, lines: "1925", excerpt: *e198, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-200, artifact_id: FR-034-AC-83, path: *p215, lines: "1926", excerpt: *e199, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-201, artifact_id: FR-034-AC-84, path: *p215, lines: "1927", excerpt: *e200, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-202, artifact_id: FR-034-AC-85, path: *p215, lines: "1928", excerpt: *e201, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-203, artifact_id: FR-034-AC-86, path: *p215, lines: "1929", excerpt: *e202, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-204, artifact_id: FR-034-AC-87, path: *p215, lines: "1930", excerpt: *e203, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-205, artifact_id: FR-034-AC-88, path: *p215, lines: "1931", excerpt: *e204, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-206, artifact_id: FR-034-AC-89, path: *p215, lines: "1932", excerpt: *e205, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-207, artifact_id: FR-034-AC-90, path: *p215, lines: "1933", excerpt: *e206, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-208, artifact_id: FR-034-AC-91, path: *p215, lines: "1934", excerpt: *e207, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-209, artifact_id: FR-034-AC-92, path: *p215, lines: "1935", excerpt: *e208, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-210, artifact_id: FR-034-AC-94, path: *p215, lines: "1937", excerpt: *e209, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-211, artifact_id: FR-034-AC-95, path: *p215, lines: "1938", excerpt: *e210, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-212, artifact_id: FR-034-AC-96, path: *p215, lines: "1939", excerpt: *e211, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-213, artifact_id: FR-034-AC-97, path: *p215, lines: "1940", excerpt: *e212, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-214, artifact_id: FR-034-AC-98, path: *p215, lines: "1941", excerpt: *e213, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-215, artifact_id: FR-034-AC-99, path: *p215, lines: "1942", excerpt: *e214, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-216, artifact_id: FR-034-AC-100, path: *p215, lines: "1943", excerpt: *e215, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-217, artifact_id: FR-041-AC-4, path: *p216, lines: "69", excerpt: *e216, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-218, artifact_id: NFR-006-M-1, path: *p219, lines: "1", excerpt: *e217, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-219, artifact_id: NFR-006-M-2, path: *p219, lines: "1", excerpt: *e218, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-220, artifact_id: NFR-006-M-3, path: *p219, lines: "1", excerpt: *e219, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-221, artifact_id: FR-014-AC-52, path: *p220, lines: "292", excerpt: *e220, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-222, artifact_id: FR-018-AC-21, path: *p234, lines: "584", excerpt: *e221, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-223, artifact_id: FR-018-AC-22, path: *p234, lines: "588", excerpt: *e222, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-224, artifact_id: FR-018-AC-23, path: *p234, lines: "595", excerpt: *e223, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-225, artifact_id: FR-018-AC-33, path: *p234, lines: "643", excerpt: *e224, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-226, artifact_id: FR-018-AC-34, path: *p234, lines: "644", excerpt: *e225, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-227, artifact_id: FR-018-AC-35, path: *p234, lines: "645", excerpt: *e226, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-228, artifact_id: FR-018-AC-36, path: *p234, lines: "646", excerpt: *e227, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-229, artifact_id: FR-018-AC-37, path: *p234, lines: "647", excerpt: *e228, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-230, artifact_id: FR-018-AC-38, path: *p234, lines: "648", excerpt: *e229, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-231, artifact_id: FR-018-AC-39, path: *p234, lines: "649", excerpt: *e230, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-232, artifact_id: FR-018-AC-40, path: *p234, lines: "650", excerpt: *e231, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-233, artifact_id: FR-018-AC-41, path: *p234, lines: "651", excerpt: *e232, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-234, artifact_id: FR-018-AC-42, path: *p234, lines: "652", excerpt: *e233, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-235, artifact_id: FR-018-AC-43, path: *p234, lines: "653", excerpt: *e234, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-236, artifact_id: FR-021-AC-30, path: *p235, lines: "277", excerpt: *e235, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-237, artifact_id: FR-031-AC-2, path: *p246, lines: "69", excerpt: *e236, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-238, artifact_id: FR-031-AC-6, path: *p246, lines: "481", excerpt: *e237, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-239, artifact_id: FR-031-AC-16, path: *p246, lines: "518", excerpt: *e238, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-240, artifact_id: FR-031-AC-17, path: *p246, lines: "189", excerpt: *e239, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-241, artifact_id: FR-031-AC-27, path: *p246, lines: "269", excerpt: *e240, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-242, artifact_id: FR-031-AC-20, path: *p246, lines: "524", excerpt: *e241, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-243, artifact_id: FR-031-AC-22, path: *p246, lines: "71", excerpt: *e242, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-244, artifact_id: FR-031-AC-26, path: *p246, lines: "527", excerpt: *e243, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-245, artifact_id: FR-031-AC-23, path: *p246, lines: "513", excerpt: *e244, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-246, artifact_id: FR-031-AC-24, path: *p246, lines: "529", excerpt: *e245, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-247, artifact_id: FR-031-AC-25, path: *p246, lines: "166", excerpt: *e246, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-248, artifact_id: FR-035-AC-1, path: *p252, lines: "107", excerpt: *e247, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-249, artifact_id: FR-035-AC-2, path: *p252, lines: "108", excerpt: *e248, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-250, artifact_id: FR-035-AC-3, path: *p252, lines: "109", excerpt: *e249, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-251, artifact_id: FR-035-AC-4, path: *p252, lines: "110", excerpt: *e250, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-252, artifact_id: FR-035-AC-5, path: *p252, lines: "111", excerpt: *e251, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-253, artifact_id: FR-035-AC-6, path: *p252, lines: "112", excerpt: *e252, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-254, artifact_id: FR-042-AC-1, path: *p253, lines: "31", excerpt: *e253, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-255, artifact_id: FR-024-AC-1, path: *p264, lines: "75", excerpt: *e254, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-256, artifact_id: FR-024-AC-7, path: *p264, lines: "352", excerpt: *e255, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-257, artifact_id: FR-024-AC-8, path: *p264, lines: "353", excerpt: *e256, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-258, artifact_id: FR-024-AC-18, path: *p264, lines: "362", excerpt: *e257, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-259, artifact_id: FR-024-AC-34, path: *p264, lines: "378", excerpt: *e258, finding: "This criterion is bound only to ignored tests; no default-lane verification is established."}
  - {<<: *defaults, fnd: FND-260, artifact_id: FR-024-AC-35, path: *p264, lines: "379", excerpt: *e259, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-261, artifact_id: FR-024-AC-36, path: *p264, lines: "380", excerpt: *e260, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-262, artifact_id: FR-024-AC-37, path: *p264, lines: "381", excerpt: *e261, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-263, artifact_id: FR-024-AC-38, path: *p264, lines: "382", excerpt: *e262, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-264, artifact_id: FR-024-AC-39, path: *p264, lines: "383", excerpt: *e263, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-265, artifact_id: FR-024-AC-40, path: *p264, lines: "384", excerpt: *e264, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-266, artifact_id: FR-032-AC-1, path: *p277, lines: "382", excerpt: *e265, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-267, artifact_id: FR-032-AC-4, path: *p277, lines: "385", excerpt: *e267, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-268, artifact_id: FR-032-AC-5, path: *p277, lines: "386", excerpt: *e268, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-269, artifact_id: FR-032-AC-6, path: *p277, lines: "387", excerpt: *e269, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-270, artifact_id: FR-032-AC-7, path: *p277, lines: "388", excerpt: *e270, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-271, artifact_id: FR-032-AC-8, path: *p277, lines: "389", excerpt: *e271, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-272, artifact_id: FR-032-AC-10, path: *p277, lines: "391", excerpt: *e273, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-273, artifact_id: FR-032-AC-11, path: *p277, lines: "392", excerpt: *e274, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-274, artifact_id: FR-032-AC-12, path: *p277, lines: "393", excerpt: *e275, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-275, artifact_id: FR-032-AC-13, path: *p277, lines: "394", excerpt: *e276, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-276, artifact_id: FR-032-AC-14, path: *p277, lines: "395", excerpt: *e277, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-277, artifact_id: FR-033-AC-2, path: *p287, lines: "362", excerpt: *e278, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-278, artifact_id: FR-033-AC-3, path: *p287, lines: "363", excerpt: *e279, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-279, artifact_id: FR-033-AC-4, path: *p287, lines: "364", excerpt: *e280, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-280, artifact_id: FR-033-AC-5, path: *p287, lines: "365", excerpt: *e281, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-281, artifact_id: FR-033-AC-6, path: *p287, lines: "366", excerpt: *e282, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-282, artifact_id: FR-033-AC-8, path: *p287, lines: "368", excerpt: *e283, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-283, artifact_id: FR-033-AC-10, path: *p287, lines: "370", excerpt: *e284, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-284, artifact_id: FR-033-AC-14, path: *p287, lines: "374", excerpt: *e285, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-285, artifact_id: FR-033-AC-15, path: *p287, lines: "375", excerpt: *e286, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-286, artifact_id: FR-033-AC-17, path: *p287, lines: "377", excerpt: *e287, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-287, artifact_id: FR-019-AC-3, path: *p298, lines: "519", excerpt: *e288, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-288, artifact_id: FR-019-AC-15, path: *p298, lines: "547", excerpt: *e289, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-289, artifact_id: FR-019-AC-24, path: *p298, lines: "548", excerpt: *e290, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-290, artifact_id: FR-019-AC-34, path: *p298, lines: "558", excerpt: *e291, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-291, artifact_id: FR-019-AC-35, path: *p298, lines: "559", excerpt: *e292, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-292, artifact_id: FR-019-AC-36, path: *p298, lines: "560", excerpt: *e293, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-293, artifact_id: FR-019-AC-37, path: *p298, lines: "561", excerpt: *e294, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-294, artifact_id: FR-019-AC-38, path: *p298, lines: "562", excerpt: *e295, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-295, artifact_id: FR-019-AC-39, path: *p298, lines: "563", excerpt: *e296, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-296, artifact_id: FR-019-AC-40, path: *p298, lines: "564", excerpt: *e297, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-297, artifact_id: FR-019-AC-41, path: *p298, lines: "565", excerpt: *e298, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-298, artifact_id: FR-022-AC-26, path: *p301, lines: "361", excerpt: *e299, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-299, artifact_id: FR-022-AC-27, path: *p301, lines: "362", excerpt: *e300, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-300, artifact_id: FR-022-AC-28, path: *p301, lines: "363", excerpt: *e301, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-301, artifact_id: FR-026-AC-1, path: *p303, lines: "60", excerpt: *e302, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-302, artifact_id: FR-026-AC-4, path: *p303, lines: "61", excerpt: *e303, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-303, artifact_id: FR-002-AC-7, path: *p314, lines: "91", excerpt: *e304, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-304, artifact_id: FR-002-AC-8, path: *p314, lines: "92", excerpt: *e305, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-305, artifact_id: FR-002-AC-10, path: *p314, lines: "94", excerpt: *e306, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-306, artifact_id: FR-002-AC-11, path: *p314, lines: "95", excerpt: *e307, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-307, artifact_id: FR-002-AC-12, path: *p314, lines: "96", excerpt: *e308, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-308, artifact_id: FR-002-AC-13, path: *p314, lines: "97", excerpt: *e309, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-309, artifact_id: FR-002-AC-14, path: *p314, lines: "98", excerpt: *e310, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-310, artifact_id: FR-002-AC-15, path: *p314, lines: "99", excerpt: *e311, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-311, artifact_id: FR-002-AC-16, path: *p314, lines: "100", excerpt: *e312, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-312, artifact_id: FR-002-AC-17, path: *p314, lines: "101", excerpt: *e313, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-313, artifact_id: FR-002-AC-18, path: *p314, lines: "102", excerpt: *e314, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-314, artifact_id: FR-008-AC-7, path: *p316, lines: "123", excerpt: *e315, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-315, artifact_id: FR-008-AC-9, path: *p316, lines: "125", excerpt: *e316, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-316, artifact_id: FR-009-AC-7, path: *p317, lines: "77", excerpt: *e317, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-317, artifact_id: NFR-004-M-1, path: *p321, lines: "1", excerpt: *e318, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-318, artifact_id: NFR-004-M-2, path: *p321, lines: "1", excerpt: *e319, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-319, artifact_id: NFR-004-M-3, path: *p321, lines: "1", excerpt: *e320, finding: "No test trace tag binds this criterion."}
  - {<<: *defaults, fnd: FND-320, artifact_id: NFR-004-M-4, path: *p321, lines: "1", excerpt: *e321, finding: "No test trace tag binds this criterion."}
bindings: [{"test_id": "add_keeps_the_plain_literal_inline_through_scalar_generation", "ac_id": "FR-032-AC-3", "trace": "correct"}, {"test_id": "add_keeps_the_plain_literal_inline_through_scalar_generation", "ac_id": "FR-032-AC-9", "trace": "correct"}, {"test_id": "equality_with_a_plain_integer_peer_keeps_the_literal_inline", "ac_id": "FR-032-AC-9", "trace": "correct"}, {"test_id": "negate_keeps_the_plain_literal_inline_through_scalar_generation", "ac_id": "FR-032-AC-3", "trace": "correct"}, {"test_id": "negate_keeps_the_plain_literal_inline_through_scalar_generation", "ac_id": "FR-032-AC-9", "trace": "correct"}, {"test_id": "equality_with_a_bounded_peer_keeps_the_narrowed_literal_as_a_graph_child", "ac_id": "FR-032-AC-9", "trace": "correct"}]
```

## Dispositions

Round 1: no product fix commit; candidate remains 59f879f3. All 320 criterion statements and original matrix statuses are identical to baseline main; source changes remain limited to IR-954. 302 untagged gaps are deferred by the authorized planner scope. Of 18 ignored-only findings, 17 are rejected because the contract intentionally selects their backend tests in the real-Kani lane; one private library-only control is deferred. No finding is fixed, no baseline criterion is completed, no gate is waived, and no new issue is created.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-002 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-003 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-004 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-005 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-006 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-007 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-008 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-009 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-010 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-011 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-012 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-013 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-014 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-015 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-016 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-017 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-018 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-019 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-020 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-021 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-022 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-023 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-024 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-025 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-026 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-19 is bound to tests/it/kani_generation.rs::kani_proves_identity_and_prints_numeric_counterexamples, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-027 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-028 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-029 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-030 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-24 is bound to tests/it/kani_generation.rs::kani_proves_identity_and_prints_numeric_counterexamples, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-031 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-30 is bound to tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-032 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-31 is bound to tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-033 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-32 is bound to tests/it/kani_obligations_state_frame.rs::tc_035_real_kani_frame_counterexample_replays_through_qsl, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-034 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-035 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-036 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-037 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-038 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-039 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-040 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-041 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-042 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-043 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-044 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-045 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-046 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-047 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-048 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-56 is bound to tests/it/kani_generation.rs::tc_025_real_kani_classifies_the_v1_bundle_verified_vacuous_and_falsified, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-049 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-57 is bound to tests/it/bounded_kani_corpus.rs::tc_023_kani_executes_the_generated_graph_harness, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-050 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-051 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-052 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-053 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-054 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-055 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-056 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-057 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-058 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-015-AC-80 is bound to tests/it/kani_obligations_state_frame.rs::tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-059 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-060 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-061 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-062 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-063 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-064 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-065 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-066 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-067 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-068 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-069 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-070 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-071 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-072 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-073 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-074 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-075 | deferred | Inherited FR-015 ownership: the only binder is tests/it/kani_render_private.rs:17 in the library test target, explicitly ignored for a separate installed-Kani control. Makefile kani target selects only --test it, so its mandatory-lane inclusion is not established. It checks unsatisfiable Assume only, not all AC-117 positive/mutation cases. Defer owning spec/plan verification to IR planner via IR-954 SR-15352 inventory; outside literal-control scope. |
| FND-076 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-077 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-078 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-079 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-080 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-081 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-082 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-083 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-017-AC-7 is bound to tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-084 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-085 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-086 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-017-AC-27 is bound to tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-087 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-017-AC-30 is bound to tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-088 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-017-AC-47 is bound to tests/it/kani_obligations.rs::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-089 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-090 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-091 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-092 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-093 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-017-CON-1 is bound to tests/it/kani_obligations.rs::tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-094 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-095 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-096 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-097 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-098 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-099 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-100 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-101 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-102 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-103 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-104 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-105 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-106 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-107 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-108 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-109 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-110 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-111 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-112 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-113 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-114 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-115 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-116 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-117 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-118 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-119 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-120 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-121 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-122 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-123 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-124 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-125 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-126 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-127 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-128 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-129 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-130 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-131 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-132 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-133 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-134 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-135 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-136 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-137 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-138 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-139 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-140 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-141 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-142 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-143 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-144 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-145 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-146 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-147 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-148 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-149 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-150 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-151 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-152 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-153 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-154 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-155 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-156 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-157 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-158 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-159 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-160 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-161 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-162 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-163 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-164 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-165 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-166 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-167 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-168 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-169 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-170 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-171 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-172 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-173 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-174 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-175 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-176 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-177 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-178 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-179 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-180 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-181 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-182 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-183 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-184 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-185 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-186 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-187 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-188 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-189 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-190 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-191 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-192 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-193 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-194 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-195 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-196 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-197 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-198 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-199 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-200 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-201 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-202 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-203 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-204 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-205 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-206 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-207 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-208 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-209 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-210 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-211 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-212 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-213 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-214 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-215 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-216 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-217 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-041-AC-4 is bound to tests/it/kani_generation_claims.rs::real_kani_claims_have_discharge_and_falsifying_mutations, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-218 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-219 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-220 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-221 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-222 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-223 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-224 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-225 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-226 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-227 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-228 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-229 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-230 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-231 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-232 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-233 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-234 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-235 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-236 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-237 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-238 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-239 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-240 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-241 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-242 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-031-AC-20 is bound to tests/it/kani_generation.rs::kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-243 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-244 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-245 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-246 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-247 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-248 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-249 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-250 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-251 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-252 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-253 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-254 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-255 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-256 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-257 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-258 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-024-AC-18 is bound to tests/it/kani_obligations_state_clause_replay.rs::tc_035_real_kani_state_clause_counterexample_replays_through_qsl, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-259 | rejected | NFR-006 Scope and AC-11/12 deliberately place installed-backend controls in ignored real-Kani tests, selected by make kani; no default-lane obligation is stated. FR-024-AC-34 is bound to tests/it/kani_obligations_state_clause_replay.rs::tc_035_real_kani_state_clause_counterexample_replays_through_qsl, whose assertions exercise its backend proposition. Ignored status alone is not a missing-verification defect; this rejects only that original rationale, not a claim of fresh run evidence or exhaustive criterion completion. |
| FND-260 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-261 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-262 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-263 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-264 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-265 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-266 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-267 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-268 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-269 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-270 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-271 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-272 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-273 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-274 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-275 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-276 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-277 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-278 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-279 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-280 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-281 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-282 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-283 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-284 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-285 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-286 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-287 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-288 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-289 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-290 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-291 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-292 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-293 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-294 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-295 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-296 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-297 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-298 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-299 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-300 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-301 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-302 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-303 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-304 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-305 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-306 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-307 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-308 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-309 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-310 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-311 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-312 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-313 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-314 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-315 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-316 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-317 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-318 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-319 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |
| FND-320 | deferred | Baseline-main untagged criterion; owning spec/plan work is outside planner-authorized IR-954 literal-control scope. Retained by criterion and owner path in IR-954 SR-15352 inventory for the IR planner; no new issue, fix, or completion claimed. |

## Disposition evidence

NFR-006 defines the ignored installed-backend lane; its Scope and AC-11 require deliberate lane membership, and AC-12 excludes it from make ci. Makefile lines 107-108 select --test it --ignored and the named integration filters. FR-017-AC-7 also checks the no-backend preflight within genuine execution integration tests; the criterion does not require a duplicate default-lane test. FR-015-AC-117 differs: its only binder is the private library test target, excluded from the integration-lane command, and covers only its unsatisfiable-Assume case. That scope is deferred rather than rejected.

Code/Rust review remains PASS. The repository gap verdict remains FAIL for inherited untagged work. For the authorized IR-954 slice, no unresolved finding blocks merging after custody of these artifacts and the lead-owned final clean-head gates pass. Optional deep semantic audit and new backend execution remain unperformed.

Verified dedicated-lane bindings (source inspection, no execution):

```yaml
- ac_id: FR-015-AC-19
  test_id: kani_proves_identity_and_prints_numeric_counterexamples
  path: tests/it/kani_generation.rs
  line: 1051
  trace: correct
- ac_id: FR-015-AC-24
  test_id: kani_proves_identity_and_prints_numeric_counterexamples
  path: tests/it/kani_generation.rs
  line: 1051
  trace: correct
- ac_id: FR-015-AC-30
  test_id: tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  path: tests/it/kani_obligations_state_frame.rs
  line: 4319
  trace: correct
- ac_id: FR-015-AC-31
  test_id: tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  path: tests/it/kani_obligations_state_frame.rs
  line: 4387
  trace: correct
- ac_id: FR-015-AC-32
  test_id: tc_035_real_kani_frame_counterexample_replays_through_qsl
  path: tests/it/kani_obligations_state_frame.rs
  line: 4431
  trace: correct
- ac_id: FR-015-AC-32
  test_id: tc_025_real_kani_the_allowed_frame_effect_replays_as_a_respected_frame
  path: tests/it/kani_obligations_state_frame.rs
  line: 4486
  trace: correct
- ac_id: FR-015-AC-56
  test_id: tc_025_real_kani_classifies_the_v1_bundle_verified_vacuous_and_falsified
  path: tests/it/kani_generation.rs
  line: 450
  trace: correct
- ac_id: FR-015-AC-57
  test_id: tc_023_kani_executes_the_generated_graph_harness
  path: tests/it/bounded_kani_corpus.rs
  line: 193
  trace: correct
- ac_id: FR-015-AC-57
  test_id: tc_023_kani_verifies_a_true_collection_and_falsifies_a_false_graph_harness
  path: tests/it/bounded_kani_corpus.rs
  line: 343
  trace: correct
- ac_id: FR-015-AC-57
  test_id: tc_023_kani_falsifies_the_generated_false_collection_harness
  path: tests/it/bounded_kani_corpus.rs
  line: 425
  trace: correct
- ac_id: FR-015-AC-57
  test_id: tc_023_real_corpus_playback_replays_through_its_qsl_source
  path: tests/it/bounded_kani_corpus.rs
  line: 449
  trace: correct
- ac_id: FR-015-AC-80
  test_id: tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it
  path: tests/it/kani_obligations_state_frame.rs
  line: 4319
  trace: correct
- ac_id: FR-015-AC-80
  test_id: tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies
  path: tests/it/kani_obligations_state_frame.rs
  line: 4387
  trace: correct
- ac_id: FR-017-AC-7
  test_id: tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  path: tests/it/kani_obligations.rs
  line: 2737
  trace: correct
- ac_id: FR-017-AC-7
  test_id: tc_027_a_routed_scalar_harness_verifies
  path: tests/it/kani_obligations.rs
  line: 3008
  trace: correct
- ac_id: FR-017-AC-7
  test_id: tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified
  path: tests/it/kani_obligations.rs
  line: 3482
  trace: correct
- ac_id: FR-017-AC-27
  test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  trace: correct
- ac_id: FR-017-AC-30
  test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  trace: correct
- ac_id: FR-017-AC-47
  test_id: tc_027_attested_scalar_receipt_binds_real_proof_and_native_result
  path: tests/it/kani_obligations.rs
  line: 3054
  trace: correct
- ac_id: FR-017-AC-47
  test_id: tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders
  path: tests/it/kani_obligations.rs
  line: 3147
  trace: correct
- ac_id: FR-017-CON-1
  test_id: tc_025_real_kani_runs_verify_separate_obligations_and_falsify_a_seeded_defect
  path: tests/it/kani_obligations.rs
  line: 2737
  trace: correct
- ac_id: FR-041-AC-4
  test_id: real_kani_claims_have_discharge_and_falsifying_mutations
  path: tests/it/kani_generation_claims.rs
  line: 439
  trace: correct
- ac_id: FR-031-AC-20
  test_id: kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value
  path: tests/it/kani_generation.rs
  line: 1824
  trace: correct
- ac_id: FR-024-AC-18
  test_id: tc_035_real_kani_state_clause_counterexample_replays_through_qsl
  path: tests/it/kani_obligations_state_clause_replay.rs
  line: 1175
  trace: correct
- ac_id: FR-024-AC-34
  test_id: tc_035_real_kani_state_clause_counterexample_replays_through_qsl
  path: tests/it/kani_obligations_state_clause_replay.rs
  line: 1175
  trace: correct
- ac_id: FR-024-AC-34
  test_id: tc_035_real_kani_frame_counterexample_replays_through_qsl
  path: tests/it/kani_obligations_state_frame.rs
  line: 4431
  trace: correct
```
