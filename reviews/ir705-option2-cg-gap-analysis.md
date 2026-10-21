---
id: SR-15332
title: "IR-705 option 2 CG gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-705; spec/assurance/AD-006-admitted-native-integer-oracles.md; spec/kani/functional/FR-015-bounded-kani-obligations.md; spec/kani/matrix/TC-027-kani-execution-evidence.md; existing oracle and clause generation seams"
review_set: subset
---

## Summary

Independent reviewer-only review of CG PR504 for IR-705 option 2. Reviewed every new AD-006 paragraph/decision/risk, the FR-015 description and three proposed criteria, TC-027 qualification addendum, related kernel FR-384 and existing CG generation seams. No implementation or proof-fit acceptance is claimed.

## Verdict

FAIL — Inherited assurance gaps remain, plus three omitted prospective obligations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-005-cli-conformance.md: (FR-005-AC-3) |
| FND-002 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-039-contract-package-consumption.md: (FR-039-AC-4) |
| FND-003 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md: (FR-043-AC-1) |
| FND-004 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md: (FR-043-AC-2) |
| FND-005 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md: (FR-043-AC-3) |
| FND-006 | medium | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md: (FR-043-AC-5) |
| FND-007 | medium | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-001-reproducible-atomic.md: (NFR-001-M-1) |
| FND-008 | medium | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-002-provenance-boundary.md: (NFR-002-M-1) |
| FND-009 | medium | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-002-provenance-boundary.md: (NFR-002-M-3) |
| FND-010 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-6) |
| FND-011 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-8) |
| FND-012 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-10) |
| FND-013 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-11) |
| FND-014 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-12) |
| FND-015 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-13) |
| FND-016 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-14) |
| FND-017 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-15) |
| FND-018 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md: (FR-004-AC-18) |
| FND-019 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-1) |
| FND-020 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-2) |
| FND-021 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-3) |
| FND-022 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-4) |
| FND-023 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-5) |
| FND-024 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-6) |
| FND-025 | medium | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md: (FR-036-AC-7) |
| FND-026 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-19) |
| FND-027 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-21) |
| FND-028 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-22) |
| FND-029 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-23) |
| FND-030 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-24) |
| FND-031 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-30) |
| FND-032 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-31) |
| FND-033 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-32) |
| FND-034 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-38) |
| FND-035 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-39) |
| FND-036 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-40) |
| FND-037 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-41) |
| FND-038 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-42) |
| FND-039 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-43) |
| FND-040 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-44) |
| FND-041 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-45) |
| FND-042 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-46) |
| FND-043 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-47) |
| FND-044 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-48) |
| FND-045 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-49) |
| FND-046 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-51) |
| FND-047 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-52) |
| FND-048 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-56) |
| FND-049 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-57) |
| FND-050 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-69) |
| FND-051 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-70) |
| FND-052 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-71) |
| FND-053 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-72) |
| FND-054 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-73) |
| FND-055 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-74) |
| FND-056 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-75) |
| FND-057 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-76) |
| FND-058 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-80) |
| FND-059 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-84) |
| FND-060 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-85) |
| FND-061 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-87) |
| FND-062 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-89) |
| FND-063 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-90) |
| FND-064 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-91) |
| FND-065 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-92) |
| FND-066 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-93) |
| FND-067 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-95) |
| FND-068 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-98) |
| FND-069 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-105) |
| FND-070 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-106) |
| FND-071 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-107) |
| FND-072 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-108) |
| FND-073 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-110) |
| FND-074 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-111) |
| FND-075 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-117) |
| FND-076 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-119) |
| FND-077 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-120) |
| FND-078 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-121) |
| FND-079 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md: (FR-015-AC-122) |
| FND-080 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-7) |
| FND-081 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-19) |
| FND-082 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-26) |
| FND-083 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-27) |
| FND-084 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-30) |
| FND-085 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-47) |
| FND-086 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-52) |
| FND-087 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-53) |
| FND-088 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-54) |
| FND-089 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-AC-55) |
| FND-090 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md: (FR-017-CON-1) |
| FND-091 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-1) |
| FND-092 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-2) |
| FND-093 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-3) |
| FND-094 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-4) |
| FND-095 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-5) |
| FND-096 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-6) |
| FND-097 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-7) |
| FND-098 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-8) |
| FND-099 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md: (FR-025-AC-9) |
| FND-100 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-5) |
| FND-101 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-6) |
| FND-102 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-7) |
| FND-103 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-8) |
| FND-104 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-9) |
| FND-105 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-13) |
| FND-106 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-14) |
| FND-107 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-15) |
| FND-108 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-16) |
| FND-109 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-17) |
| FND-110 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-18) |
| FND-111 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-19) |
| FND-112 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-20) |
| FND-113 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-22) |
| FND-114 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-23) |
| FND-115 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md: (FR-028-AC-24) |
| FND-116 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-17) |
| FND-117 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-19) |
| FND-118 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-21) |
| FND-119 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-22) |
| FND-120 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-23) |
| FND-121 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-24) |
| FND-122 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-25) |
| FND-123 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-26) |
| FND-124 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-27) |
| FND-125 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-30) |
| FND-126 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md: (FR-029-AC-31) |
| FND-127 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md: (FR-030-AC-19) |
| FND-128 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md: (FR-030-AC-20) |
| FND-129 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md: (FR-030-AC-21) |
| FND-130 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md: (FR-030-AC-22) |
| FND-131 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-1) |
| FND-132 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-2) |
| FND-133 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-3) |
| FND-134 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-4) |
| FND-135 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-5) |
| FND-136 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-6) |
| FND-137 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-7) |
| FND-138 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-8) |
| FND-139 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-9) |
| FND-140 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-10) |
| FND-141 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-11) |
| FND-142 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-12) |
| FND-143 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-13) |
| FND-144 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-14) |
| FND-145 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-15) |
| FND-146 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-16) |
| FND-147 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-17) |
| FND-148 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-18) |
| FND-149 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-19) |
| FND-150 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-20) |
| FND-151 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-21) |
| FND-152 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-22) |
| FND-153 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-23) |
| FND-154 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-24) |
| FND-155 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-25) |
| FND-156 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-26) |
| FND-157 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-27) |
| FND-158 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-28) |
| FND-159 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-29) |
| FND-160 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-31) |
| FND-161 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-32) |
| FND-162 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-33) |
| FND-163 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-34) |
| FND-164 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-35) |
| FND-165 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-36) |
| FND-166 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-37) |
| FND-167 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-38) |
| FND-168 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-39) |
| FND-169 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-40) |
| FND-170 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-51) |
| FND-171 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-52) |
| FND-172 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-53) |
| FND-173 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-54) |
| FND-174 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-57) |
| FND-175 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-58) |
| FND-176 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-59) |
| FND-177 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-60) |
| FND-178 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-61) |
| FND-179 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-62) |
| FND-180 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-63) |
| FND-181 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-65) |
| FND-182 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-66) |
| FND-183 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-67) |
| FND-184 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-69) |
| FND-185 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-70) |
| FND-186 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-71) |
| FND-187 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-73) |
| FND-188 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-74) |
| FND-189 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-75) |
| FND-190 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-76) |
| FND-191 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-77) |
| FND-192 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-78) |
| FND-193 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-79) |
| FND-194 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-80) |
| FND-195 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-81) |
| FND-196 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-82) |
| FND-197 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-83) |
| FND-198 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-84) |
| FND-199 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-85) |
| FND-200 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-86) |
| FND-201 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-87) |
| FND-202 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-88) |
| FND-203 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-89) |
| FND-204 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-90) |
| FND-205 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-91) |
| FND-206 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-92) |
| FND-207 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-94) |
| FND-208 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-95) |
| FND-209 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-96) |
| FND-210 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-97) |
| FND-211 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-98) |
| FND-212 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-99) |
| FND-213 | medium | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md: (FR-034-AC-100) |
| FND-214 | medium | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-041-claimed-module-proof-gate.md: (FR-041-AC-4) |
| FND-215 | medium | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md: (NFR-006-M-1) |
| FND-216 | medium | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md: (NFR-006-M-2) |
| FND-217 | medium | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md: (NFR-006-M-3) |
| FND-218 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-21) |
| FND-219 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-22) |
| FND-220 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-23) |
| FND-221 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-33) |
| FND-222 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-34) |
| FND-223 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-35) |
| FND-224 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-36) |
| FND-225 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-37) |
| FND-226 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-38) |
| FND-227 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-39) |
| FND-228 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-40) |
| FND-229 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-41) |
| FND-230 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-42) |
| FND-231 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md: (FR-018-AC-43) |
| FND-232 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-021-function-application-oracles.md: (FR-021-AC-30) |
| FND-233 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-2) |
| FND-234 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-6) |
| FND-235 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-16) |
| FND-236 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-17) |
| FND-237 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-27) |
| FND-238 | medium | Inherited criterion has only ignored-test bindings. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-20) |
| FND-239 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-22) |
| FND-240 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-26) |
| FND-241 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-23) |
| FND-242 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-24) |
| FND-243 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md: (FR-031-AC-25) |
| FND-244 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-1) |
| FND-245 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-2) |
| FND-246 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-3) |
| FND-247 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-4) |
| FND-248 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-5) |
| FND-249 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md: (FR-035-AC-6) |
| FND-250 | medium | Inherited criterion has no tagged test. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md: (FR-042-AC-1) |
| FND-251 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-1) |
| FND-252 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-7) |
| FND-253 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-8) |
| FND-254 | medium | Inherited criterion has only ignored-test bindings. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-18) |
| FND-255 | medium | Inherited criterion has only ignored-test bindings. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-34) |
| FND-256 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-35) |
| FND-257 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-36) |
| FND-258 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-37) |
| FND-259 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-38) |
| FND-260 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-39) |
| FND-261 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md: (FR-024-AC-40) |
| FND-262 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-1) |
| FND-263 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-4) |
| FND-264 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-5) |
| FND-265 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-6) |
| FND-266 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-7) |
| FND-267 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-8) |
| FND-268 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-9) |
| FND-269 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-10) |
| FND-270 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-11) |
| FND-271 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-12) |
| FND-272 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-13) |
| FND-273 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md: (FR-032-AC-14) |
| FND-274 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-2) |
| FND-275 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-3) |
| FND-276 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-4) |
| FND-277 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-5) |
| FND-278 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-6) |
| FND-279 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-8) |
| FND-280 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-10) |
| FND-281 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-14) |
| FND-282 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-15) |
| FND-283 | medium | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md: (FR-033-AC-17) |
| FND-284 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-3) |
| FND-285 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-15) |
| FND-286 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-24) |
| FND-287 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-34) |
| FND-288 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-35) |
| FND-289 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-36) |
| FND-290 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-37) |
| FND-291 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-38) |
| FND-292 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-39) |
| FND-293 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-40) |
| FND-294 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md: (FR-019-AC-41) |
| FND-295 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md: (FR-022-AC-26) |
| FND-296 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md: (FR-022-AC-27) |
| FND-297 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md: (FR-022-AC-28) |
| FND-298 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-026-backend-adapter-contract.md: (FR-026-AC-1) |
| FND-299 | medium | Inherited criterion has no tagged test. | spec/routed/functional/FR-026-backend-adapter-contract.md: (FR-026-AC-4) |
| FND-300 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-7) |
| FND-301 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-8) |
| FND-302 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-10) |
| FND-303 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-11) |
| FND-304 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-12) |
| FND-305 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-13) |
| FND-306 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-14) |
| FND-307 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-15) |
| FND-308 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-16) |
| FND-309 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-17) |
| FND-310 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md: (FR-002-AC-18) |
| FND-311 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md: (FR-008-AC-7) |
| FND-312 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md: (FR-008-AC-9) |
| FND-313 | medium | Inherited criterion has no tagged test. | spec/strategy/functional/FR-009-constructive-correlated-populations.md: (FR-009-AC-7) |
| FND-314 | medium | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md: (NFR-004-M-1) |
| FND-315 | medium | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md: (NFR-004-M-2) |
| FND-316 | medium | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md: (NFR-004-M-3) |
| FND-317 | medium | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md: (NFR-004-M-4) |
| FND-318 | high | FR-015-AC-125 through AC-127 sit inside the two-column mutation table instead of the Acceptance Criteria table; the computed matrix omits all three obligations. | spec/kani/functional/FR-015-bounded-kani-obligations.md:888-890 (FR-015-AC-125) |

## Coverage

Full repository computed matrix: 892 criteria; 523 tagged, 299 untagged, 52 method-without-symbol, 18 ignored-only. Base and candidate row IDs/statuses are identical; proposed AC125..127 are absent. Full source ownership/stub scan found no introduced production code, hollow tests or reverse gaps. Existing test-module panic assertions are test controls, not production stubs. Plan completion: not assessed

## Examined scope

- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-1` (examined): "Peter approved IR-705 option 2: a kernel-owned admitted bounded-native API with a proven refinement to BigInt, followed by genuine IR-705, IR-917 and IR-948 proof qualification. CG selects and renders that production API from authentic IR-admitted CheckedPackageV2 data. quire-exact owns native value/operation admission, arithmetic and Meter semantics. This decision does not authorize a private CG evaluator or replace existing general Integer and as_big contracts."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-2` (examined): "The views below separate authentic graph admission, production native execution and independent proof qualification. Each retains its own owner and refusal boundary."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-3` (examined): "CG reads operand intervals and every reachable arithmetic operation from the admitted graph through the existing IR-owned seams. It derives conservative interval bounds in expression order, preserving source identities and parameter relationships. The native capacity request includes intermediate results, not only the final declared domain. The kernel checks Add/Subtract/Multiply/Negate interval capacity and creates a checked operation plan; per-call membership creates an opaque admitted operation. A plan and a value from unrelated domains cannot be combined without that check."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-4` (examined): "Host admission derives canonical native endpoints from the genuine IR domains. Generated production/proof code reconstructs the selected interval plan from those native constant endpoints through the same checked kernel constructor, then admits actual payloads before evaluation. Both constructors and operand admission remain bounded-native on their successful paths. The host does not serialize an unchecked capability and the emitted proof does not repeat general Integer/BigInt admission or cache construction. This boundary must be evidenced in the actual generated call graph."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-5` (examined): "CG records the selected kernel capability and its input/result interval decisions in the generated artifacts and identity inputs using the repository's existing canonical identity mechanism. It does not invent checked node ids or equate a caller descriptor with package admission. Any public struct/serialized identity change follows the owning schema's compatibility rules rather than silently changing a digest interpretation."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-6` (examined): "Generated production native and Kani clients call the same kernel-owned admitted operations with the real public Meter. Chained values remain native i128 carriers, without promotion into general Integer/cache on the successful arithmetic path. Comparison and Boolean operations remain kernel-owned; subjects and independent expected-value expressions preserve their existing distinct roles."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-7` (examined): "A kernel capacity refusal is a typed generation/capability disposition carrying the offending source and capacity reason, before artifact success. It is distinct from IntegerOutOfDomain for a completed mathematical result outside a declared semantic domain. The general exact route remains available under its existing capability rules; selection failure is not proof success. No added assumption may shrink the authored population to make a native plan admissible."
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-8` (examined): "Kernel qualification supplies machine-checked full-capacity refinement against an independent mathematical abstraction of actual BigInt semantics; sampled differential tests alone are not a universal proof. Kernel qualification proves native admission and every operation against BigInt semantics over the full accepted capacity, including real Meter/refusal refinement. CG qualification separately obtains authentic QSL-emitted, IR-admitted inputs through the public generator and installed backend. No generated fixture is hand-edited into a replacement evaluator, and no healthy proof becomes igno"
- `spec/assurance/AD-006-admitted-native-integer-oracles.md#unit-9` (examined): "The route can change generated oracle source bytes and native ABI carriers. Those changes are recorded and reviewed. It preserves the original proposition, selected clause, input/domain bounds, compiled subject intent, independent oracle/expectation, assertions, final completion cover and substantive mutants. The general-only route's source bytes are not an acceptance oracle for this architecture change."
- `spec/kani/functional/FR-015-bounded-kani-obligations.md#unit-1` (examined): "When selecting the additive kernel facility of [quire-exact FR-384](ix://agent-ix/quire-exact/FR-384), CG SHALL obtain finite operand domains and operation structure from the authentic CheckedPackageV2 and existing IR-owned accessors, preserve every original input valuation, and request kernel capacity admission for every reachable intermediate. CG SHALL render the same production kernel API in native and proof clients, retain the real Meter and semantic result-domain refusals, and return a typed source-associated capability/generation refusal if capacity or value admission fails. It SHALL NOT"
- `FR-015-AC-125` (examined): "Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs generate kernel-owned native carriers and checked operation plans for all intermediates, with original graph identities/domains retained. Reversed/oversized capacities and operands outside a selected plan yield typed source-associated refusal, never narrowed assumptions or a completed value. Native production and proof artifacts use the same kernel implementation and real Meter; inspection finds no copied arithmetic or verification-only evaluator."
- `FR-015-AC-126` (examined): "Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917 clause implication and IR-948 V2 pre/post/invariant profiles regenerate through public CG with the reviewed admitted kernel API. Original semantic domains, selected propositions/clauses, independent expected values, subject controls, assertions, final completion covers and existing solver/unwind/time settings remain. Each healthy profile verifies at 16 GiB; each required substantive mutant falsifies with original-domain native playback. The IR-948 impossible-precondition control must complete as CoverUnsatisfied at the s"
- `FR-015-AC-127` (examined): "End-to-end denied/cancelled admitted-native controls retain all real Meter counters, charge order, complete incomplete/refusal fields and admitted prefixes at every reached charge point, compared with independent normative expectations and the general exact route. A semantic result-domain failure remains after arithmetic charge and before result-retain, separately from generation-time native-capacity failure."
- `spec/kani/matrix/TC-027-kani-execution-evidence.md#unit-1` (examined): "After reviewed [kernel FR-384](ix://agent-ix/quire-exact/FR-384) refinement evidence, regenerate the authentic TC-027 healthy/bound-mutant controls, TC-025 arithmetic mutant, IR-917 clause implication and IR-948 V2 contract pre/post/invariant profiles through the public generator under [AD-006](../../assurance/AD-006-admitted-native-integer-oracles.md). Record the reviewed kernel identity and native-capability selection without copying runtime source or mutating historical fixtures. Preserve original populations, semantic bounds, clauses, independent oracle/expectation, compiled subject contro"
- `spec/kani/matrix/TC-027-kani-execution-evidence.md#unit-2` (examined): "Run one backend job at a time with the original per-profile solver, unwind and time settings and a measured 16 GiB memory envelope. Require healthy Verified with cover satisfied and every specified mutant Falsified with concrete original-domain native playback; a successful compiler or generation Boolean is insufficient. Record actual terminal outcome, peak and confirmed cleanup per proof. Diagnostic runs with larger authorized ceilings are attributed separately and cannot establish 16 GiB fit. Exercise real cumulative Meter exact/one-less limits, cancellation, each named injected denial and d"
- `spec/kani/matrix/TC-027-kani-execution-evidence.md#unit-3` (examined): "For IR-948, retain the authentic impossible-precondition package and require CoverUnsatisfied at the same 16 GiB ceiling and original unwind8. This is an adverse cover control rather than an arithmetic falsification witness. Healthy Verified alone does not complete the profile pair."
- `src/oracle/boolean_v1.rs` (context_only): "//! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.\n\nuse std::collections::BTreeMap;\n\nuse crate::{\n    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},\n    core::canonical::artifact_json,\n    core::diagnostic::{GenerationDiagnostic, GenerationErrorCode},\n    core::naming::{observation_name, oracle_symbol, reference_identifier},\n    core::source_map::{SourceProbe, SourceRegion},\n};\nuse quire_contract_model::{\n    BooleanOperator, ClauseId, ComparisonOperator, DefinednessObligationKind, DependencyIdentity,\n    DependencyKind, Expression, Expression"
- `src/kani/generate/clause.rs` (context_only): "//! The V1 clause lowering that the precondition and contract families share: the lowered clause\n//! form, the clause oracle, the subject ABI and its slots.\n\nuse std::collections::BTreeMap;\n\nuse quire_contract_model::{\n    BoundClause, BoundPackage, ClauseKind, ClauseRef, DependencyIdentity, DependencyKind,\n    ExecutionPoint, StateObservation,\n};\n\nuse crate::{\n    core::diagnostic::{GenerationDiagnostic, GenerationErrorCode},\n    core::naming::{oracle_symbol, reference_identifier},\n    kani::abi::{\n        i64_literal, readable_component, KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType,"
- `src/kani/generate/contract.rs` (context_only): "//! The contract family: rendering the Kani contract harness of a V1 postcondition or invariant\n//! clause (FR-015).\n\nuse crate::{\n    kani::abi::i64_literal,\n    kani::generate::clause::{call, symbolic_arguments, Abi, LoweredClause, SlotContext},\n    kani::identity::ObligationKind,\n};\n\n/// The non-vacuity cover every contract harness ends with. It is reachable only on a path where\n/// the IR argument bounds and every `requires` hold together, so an unsatisfied cover means the\n/// `ensures` was never checked.\nconst CONTRACT_COVER: &str = \"contract requires and IR bounds are jointly satisfiable"
- `src/oracle/scalar/mod.rs` (context_only): "//! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).\n//!\n//! The V2 transport names only an operator class (`binary`, `unary`,\n//! `convert`) for a scalar expression, not the operation law. Each request\n//! item therefore pairs a checked node with a typed [`ExactScalarOperation`]\n//! built from Contract Runtime `exact` values, and this module cross-checks\n//! that descriptor against the lowered node before emitting anything.\n//!\n//! Every parameter the node can carry is checked against the IR: the\n//! descriptor's integer, rational and decimal domains, IEEE rounding "

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-002 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-003 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-004 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-005 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-006 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-007 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-008 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-009 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-010 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-011 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-012 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-013 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-014 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-015 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-016 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-017 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-018 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-019 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-020 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-021 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-022 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-023 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-024 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-025 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-026 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-027 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-028 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-029 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-030 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-031 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-032 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-033 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-034 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-035 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-036 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-037 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-038 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-039 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-040 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-041 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-042 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-043 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-044 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-045 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-046 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-047 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-048 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-049 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-050 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-051 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-052 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-053 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-054 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-055 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-056 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-057 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-058 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-059 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-060 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-061 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-062 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-063 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-064 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-065 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-066 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-067 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-068 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-069 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-070 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-071 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-072 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-073 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-074 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-075 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-076 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-077 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-078 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-079 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-080 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-081 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-082 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-083 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-084 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-085 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-086 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-087 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-088 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-089 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-090 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-091 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-092 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-093 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-094 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-095 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-096 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-097 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-098 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-099 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-100 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-101 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-102 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-103 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-104 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-105 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-106 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-107 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-108 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-109 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-110 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-111 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-112 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-113 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-114 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-115 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-116 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-117 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-118 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-119 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-120 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-121 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-122 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-123 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-124 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-125 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-126 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-127 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-128 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-129 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-130 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-131 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-132 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-133 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-134 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-135 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-136 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-137 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-138 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-139 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-140 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-141 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-142 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-143 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-144 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-145 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-146 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-147 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-148 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-149 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-150 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-151 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-152 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-153 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-154 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-155 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-156 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-157 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-158 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-159 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-160 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-161 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-162 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-163 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-164 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-165 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-166 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-167 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-168 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-169 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-170 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-171 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-172 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-173 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-174 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-175 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-176 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-177 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-178 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-179 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-180 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-181 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-182 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-183 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-184 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-185 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-186 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-187 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-188 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-189 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-190 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-191 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-192 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-193 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-194 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-195 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-196 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-197 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-198 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-199 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-200 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-201 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-202 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-203 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-204 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-205 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-206 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-207 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-208 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-209 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-210 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-211 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-212 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-213 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-214 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-215 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-216 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-217 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-218 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-219 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-220 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-221 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-222 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-223 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-224 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-225 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-226 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-227 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-228 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-229 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-230 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-231 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-232 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-233 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-234 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-235 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-236 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-237 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-238 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-239 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-240 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-241 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-242 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-243 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-244 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-245 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-246 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-247 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-248 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-249 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-250 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-251 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-252 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-253 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-254 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-255 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-256 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-257 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-258 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-259 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-260 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-261 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-262 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-263 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-264 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-265 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-266 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-267 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-268 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-269 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-270 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-271 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-272 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-273 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-274 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-275 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-276 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-277 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-278 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-279 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-280 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-281 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-282 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-283 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-284 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-285 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-286 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-287 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-288 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-289 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-290 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-291 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-292 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-293 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-294 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-295 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-296 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-297 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-298 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-299 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-300 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-301 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-302 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-303 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-304 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-305 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-306 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-307 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-308 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-309 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-310 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-311 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-312 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-313 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-314 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-315 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-316 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |
| FND-317 | deferred | IR-878 owns inherited repository assurance closure; not delivered or waived by the IR-705 specification PR. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- |
| FND-319 | medium | New prospective native-capability criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:865 (FR-015-AC-125) |
| FND-320 | medium | New prospective native-capability criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 (FR-015-AC-126) |
| FND-321 | medium | New prospective native-capability criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:867 (FR-015-AC-127) |

### Disposition round 1

| FND | outcome | reason |
| --- | --- | --- |
| FND-318 | fixed | The three byte-identical AC rows now appear in the main acceptance table and real matrix. |
| FND-319 | deferred | IR-705 implementation and genuine qualification stage owns this prospective criterion; no completed or waived proof credit. |
| FND-320 | deferred | IR-705 implementation and genuine qualification stage owns this prospective criterion; no completed or waived proof credit. |
| FND-321 | deferred | IR-705 implementation and genuine qualification stage owns this prospective criterion; no completed or waived proof credit. |

## Narrow rebase and fix review

Incoming main changes remove untyped review transcripts and add the typed IR-953 review only; no implementation, Cargo/lock, AD-006 or TC-027 content changed. The three AC rows move byte-for-byte after AC-124, before mutation headings. Independent matrix recomputation now finds 895 criteria (523 tagged, 302 untagged, 52 method-without-symbol, 18 ignored-only). All 892 original criterion statements/statuses are unchanged. AC-125, AC-126 and AC-127 each compute Test / untagged / no binders. This is spec acceptance only: IR-705 implementation and actual healthy/mutant/profile qualification remain required.
