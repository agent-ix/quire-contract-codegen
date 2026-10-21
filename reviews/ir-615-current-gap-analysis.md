---
id: SR-14861
title: gap-analysis current-cohort review of IR615 CG
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; IR615 current main-to-candidate eight-file
  diff; whole repository matrix
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/NFR-006
  type: references
---

## Summary

Ticket: IR-615; CG PR485. Fresh complete review of current main 8ac09bc85f2a34b7b91d36bd59ce8e92e6ced59d through current candidate 2f24b78989f550f3e68e31323cf33782a96caf5d, all eight changed files, including historical review custody. Old SR14500/14501/14502 are historical and supply no fresh acceptance. Reviewer GPT-6 (deployment SKU unavailable; marker gpt-6). No applicable AssuranceProfile exists. No product edit or Cargo/fmt/metadata/rustc/native/Kani/CBMC/goto/aggregate command was run. Scope excerpts longer than600characters are verbatim prefixes.

## Verdict

FAIL —291inherited untagged and18ignored-only criteria; current309findings await explicit dispositions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-005-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/core/functional/FR-005-cli-conformance.md:49 |
| FND-002 | medium | FR-039-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/core/functional/FR-039-contract-package-consumption.md:213 |
| FND-003 | medium | NFR-001-M-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/core/non-functional/NFR-001-reproducible-atomic.md:1 |
| FND-004 | medium | NFR-002-M-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 |
| FND-005 | medium | NFR-002-M-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/core/non-functional/NFR-002-provenance-boundary.md:1 |
| FND-006 | medium | FR-004-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:186 |
| FND-007 | medium | FR-004-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:188 |
| FND-008 | medium | FR-004-AC-10 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:190 |
| FND-009 | medium | FR-004-AC-11 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:191 |
| FND-010 | medium | FR-004-AC-12 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:192 |
| FND-011 | medium | FR-004-AC-13 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:193 |
| FND-012 | medium | FR-004-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:194 |
| FND-013 | medium | FR-004-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:195 |
| FND-014 | medium | FR-004-AC-18 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-004-vacuity-evidence.md:198 |
| FND-015 | medium | FR-036-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:140 |
| FND-016 | medium | FR-036-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:141 |
| FND-017 | medium | FR-036-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:142 |
| FND-018 | medium | FR-036-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:143 |
| FND-019 | medium | FR-036-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:144 |
| FND-020 | medium | FR-036-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:145 |
| FND-021 | medium | FR-036-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/evidence/functional/FR-036-source-obligation-census.md:146 |
| FND-022 | medium | FR-015-AC-19 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:741 |
| FND-023 | medium | FR-015-AC-21 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:743 |
| FND-024 | medium | FR-015-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:744 |
| FND-025 | medium | FR-015-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:745 |
| FND-026 | medium | FR-015-AC-24 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:746 |
| FND-027 | medium | FR-015-AC-30 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:752 |
| FND-028 | medium | FR-015-AC-31 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:753 |
| FND-029 | medium | FR-015-AC-32 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:754 |
| FND-030 | medium | FR-015-AC-38 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:760 |
| FND-031 | medium | FR-015-AC-39 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:761 |
| FND-032 | medium | FR-015-AC-40 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:762 |
| FND-033 | medium | FR-015-AC-41 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:763 |
| FND-034 | medium | FR-015-AC-42 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:764 |
| FND-035 | medium | FR-015-AC-43 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:765 |
| FND-036 | medium | FR-015-AC-44 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |
| FND-037 | medium | FR-015-AC-45 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 |
| FND-038 | medium | FR-015-AC-46 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:768 |
| FND-039 | medium | FR-015-AC-47 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:769 |
| FND-040 | medium | FR-015-AC-48 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:770 |
| FND-041 | medium | FR-015-AC-49 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:771 |
| FND-042 | medium | FR-015-AC-51 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:773 |
| FND-043 | medium | FR-015-AC-52 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:774 |
| FND-044 | medium | FR-015-AC-56 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:778 |
| FND-045 | medium | FR-015-AC-57 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:779 |
| FND-046 | medium | FR-015-AC-69 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:791 |
| FND-047 | medium | FR-015-AC-70 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:792 |
| FND-048 | medium | FR-015-AC-71 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:793 |
| FND-049 | medium | FR-015-AC-72 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:794 |
| FND-050 | medium | FR-015-AC-73 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:795 |
| FND-051 | medium | FR-015-AC-74 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:796 |
| FND-052 | medium | FR-015-AC-75 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:797 |
| FND-053 | medium | FR-015-AC-76 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:798 |
| FND-054 | medium | FR-015-AC-80 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:802 |
| FND-055 | medium | FR-015-AC-84 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:806 |
| FND-056 | medium | FR-015-AC-85 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:807 |
| FND-057 | medium | FR-015-AC-87 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:809 |
| FND-058 | medium | FR-015-AC-89 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:811 |
| FND-059 | medium | FR-015-AC-90 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:812 |
| FND-060 | medium | FR-015-AC-91 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:813 |
| FND-061 | medium | FR-015-AC-92 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:814 |
| FND-062 | medium | FR-015-AC-93 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:815 |
| FND-063 | medium | FR-015-AC-95 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:817 |
| FND-064 | medium | FR-015-AC-98 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:820 |
| FND-065 | medium | FR-015-AC-105 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:827 |
| FND-066 | medium | FR-015-AC-106 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:828 |
| FND-067 | medium | FR-015-AC-107 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:829 |
| FND-068 | medium | FR-015-AC-108 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:830 |
| FND-069 | medium | FR-015-AC-110 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:832 |
| FND-070 | medium | FR-015-AC-111 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:833 |
| FND-071 | medium | FR-015-AC-117 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:839 |
| FND-072 | medium | FR-015-AC-119 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-015-bounded-kani-obligations.md:841 |
| FND-073 | medium | FR-017-AC-7 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-017-kani-execution-evidence.md:833 |
| FND-074 | medium | FR-017-AC-19 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-017-kani-execution-evidence.md:842 |
| FND-075 | medium | FR-017-AC-26 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-017-kani-execution-evidence.md:849 |
| FND-076 | medium | FR-017-AC-27 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-017-kani-execution-evidence.md:850 |
| FND-077 | medium | FR-017-AC-30 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-017-kani-execution-evidence.md:853 |
| FND-078 | medium | FR-017-AC-47 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-017-kani-execution-evidence.md:870 |
| FND-079 | medium | FR-017-AC-52 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-017-kani-execution-evidence.md:875 |
| FND-080 | medium | FR-017-AC-53 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-017-kani-execution-evidence.md:876 |
| FND-081 | medium | FR-017-CON-1 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-017-kani-execution-evidence.md:822 |
| FND-082 | medium | FR-025-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:94 |
| FND-083 | medium | FR-025-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:95 |
| FND-084 | medium | FR-025-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:96 |
| FND-085 | medium | FR-025-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:97 |
| FND-086 | medium | FR-025-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:98 |
| FND-087 | medium | FR-025-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:99 |
| FND-088 | medium | FR-025-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:100 |
| FND-089 | medium | FR-025-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:101 |
| FND-090 | medium | FR-025-AC-9 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-025-generated-subject-abi.md:102 |
| FND-091 | medium | FR-028-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:206 |
| FND-092 | medium | FR-028-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:207 |
| FND-093 | medium | FR-028-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:208 |
| FND-094 | medium | FR-028-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:209 |
| FND-095 | medium | FR-028-AC-9 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:210 |
| FND-096 | medium | FR-028-AC-13 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:212 |
| FND-097 | medium | FR-028-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:213 |
| FND-098 | medium | FR-028-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:214 |
| FND-099 | medium | FR-028-AC-16 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:215 |
| FND-100 | medium | FR-028-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:216 |
| FND-101 | medium | FR-028-AC-18 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:217 |
| FND-102 | medium | FR-028-AC-19 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:218 |
| FND-103 | medium | FR-028-AC-20 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:219 |
| FND-104 | medium | FR-028-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:221 |
| FND-105 | medium | FR-028-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:222 |
| FND-106 | medium | FR-028-AC-24 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:223 |
| FND-107 | medium | FR-029-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:377 |
| FND-108 | medium | FR-029-AC-19 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:379 |
| FND-109 | medium | FR-029-AC-21 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:381 |
| FND-110 | medium | FR-029-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382 |
| FND-111 | medium | FR-029-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:383 |
| FND-112 | medium | FR-029-AC-24 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:384 |
| FND-113 | medium | FR-029-AC-25 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:385 |
| FND-114 | medium | FR-029-AC-26 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:386 |
| FND-115 | medium | FR-029-AC-27 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:387 |
| FND-116 | medium | FR-029-AC-30 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:390 |
| FND-117 | medium | FR-029-AC-31 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:391 |
| FND-118 | medium | FR-030-AC-19 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:237 |
| FND-119 | medium | FR-030-AC-20 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:238 |
| FND-120 | medium | FR-030-AC-21 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:239 |
| FND-121 | medium | FR-030-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:240 |
| FND-122 | medium | FR-034-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1844 |
| FND-123 | medium | FR-034-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1845 |
| FND-124 | medium | FR-034-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1846 |
| FND-125 | medium | FR-034-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1847 |
| FND-126 | medium | FR-034-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1848 |
| FND-127 | medium | FR-034-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1849 |
| FND-128 | medium | FR-034-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1850 |
| FND-129 | medium | FR-034-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1851 |
| FND-130 | medium | FR-034-AC-9 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1852 |
| FND-131 | medium | FR-034-AC-10 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1853 |
| FND-132 | medium | FR-034-AC-11 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1854 |
| FND-133 | medium | FR-034-AC-12 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1855 |
| FND-134 | medium | FR-034-AC-13 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1856 |
| FND-135 | medium | FR-034-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1857 |
| FND-136 | medium | FR-034-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1858 |
| FND-137 | medium | FR-034-AC-16 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1859 |
| FND-138 | medium | FR-034-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1860 |
| FND-139 | medium | FR-034-AC-18 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1861 |
| FND-140 | medium | FR-034-AC-19 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1862 |
| FND-141 | medium | FR-034-AC-20 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1863 |
| FND-142 | medium | FR-034-AC-21 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1864 |
| FND-143 | medium | FR-034-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1865 |
| FND-144 | medium | FR-034-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1866 |
| FND-145 | medium | FR-034-AC-24 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1867 |
| FND-146 | medium | FR-034-AC-25 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1868 |
| FND-147 | medium | FR-034-AC-26 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1869 |
| FND-148 | medium | FR-034-AC-27 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1870 |
| FND-149 | medium | FR-034-AC-28 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1871 |
| FND-150 | medium | FR-034-AC-29 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1872 |
| FND-151 | medium | FR-034-AC-31 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1874 |
| FND-152 | medium | FR-034-AC-32 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1875 |
| FND-153 | medium | FR-034-AC-33 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1876 |
| FND-154 | medium | FR-034-AC-34 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1877 |
| FND-155 | medium | FR-034-AC-35 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1878 |
| FND-156 | medium | FR-034-AC-36 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1879 |
| FND-157 | medium | FR-034-AC-37 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1880 |
| FND-158 | medium | FR-034-AC-38 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1881 |
| FND-159 | medium | FR-034-AC-39 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1882 |
| FND-160 | medium | FR-034-AC-40 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1883 |
| FND-161 | medium | FR-034-AC-51 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1894 |
| FND-162 | medium | FR-034-AC-52 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1895 |
| FND-163 | medium | FR-034-AC-53 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1896 |
| FND-164 | medium | FR-034-AC-54 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1897 |
| FND-165 | medium | FR-034-AC-57 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1900 |
| FND-166 | medium | FR-034-AC-58 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1901 |
| FND-167 | medium | FR-034-AC-59 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1902 |
| FND-168 | medium | FR-034-AC-60 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1903 |
| FND-169 | medium | FR-034-AC-61 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1904 |
| FND-170 | medium | FR-034-AC-62 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1905 |
| FND-171 | medium | FR-034-AC-63 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1906 |
| FND-172 | medium | FR-034-AC-65 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1908 |
| FND-173 | medium | FR-034-AC-66 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1909 |
| FND-174 | medium | FR-034-AC-67 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1910 |
| FND-175 | medium | FR-034-AC-69 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1912 |
| FND-176 | medium | FR-034-AC-70 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1913 |
| FND-177 | medium | FR-034-AC-71 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1914 |
| FND-178 | medium | FR-034-AC-73 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1916 |
| FND-179 | medium | FR-034-AC-74 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1917 |
| FND-180 | medium | FR-034-AC-75 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1918 |
| FND-181 | medium | FR-034-AC-76 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1919 |
| FND-182 | medium | FR-034-AC-77 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1920 |
| FND-183 | medium | FR-034-AC-78 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1921 |
| FND-184 | medium | FR-034-AC-79 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1922 |
| FND-185 | medium | FR-034-AC-80 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1923 |
| FND-186 | medium | FR-034-AC-81 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1924 |
| FND-187 | medium | FR-034-AC-82 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1925 |
| FND-188 | medium | FR-034-AC-83 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1926 |
| FND-189 | medium | FR-034-AC-84 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1927 |
| FND-190 | medium | FR-034-AC-85 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1928 |
| FND-191 | medium | FR-034-AC-86 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1929 |
| FND-192 | medium | FR-034-AC-87 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1930 |
| FND-193 | medium | FR-034-AC-88 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1931 |
| FND-194 | medium | FR-034-AC-89 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1932 |
| FND-195 | medium | FR-034-AC-90 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1933 |
| FND-196 | medium | FR-034-AC-91 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1934 |
| FND-197 | medium | FR-034-AC-92 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1935 |
| FND-198 | medium | FR-034-AC-94 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1937 |
| FND-199 | medium | FR-034-AC-95 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1938 |
| FND-200 | medium | FR-034-AC-96 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1939 |
| FND-201 | medium | FR-034-AC-97 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1940 |
| FND-202 | medium | FR-034-AC-98 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1941 |
| FND-203 | medium | FR-034-AC-99 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1942 |
| FND-204 | medium | FR-034-AC-100 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/functional/FR-034-caller-death-ownership.md:1943 |
| FND-205 | medium | FR-041-AC-4 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:69 |
| FND-206 | medium | NFR-006-M-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-207 | medium | NFR-006-M-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-208 | medium | NFR-006-M-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:1 |
| FND-209 | medium | FR-018-AC-21 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:590 |
| FND-210 | medium | FR-018-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:591 |
| FND-211 | medium | FR-018-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:592 |
| FND-212 | medium | FR-018-AC-33 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:602 |
| FND-213 | medium | FR-018-AC-34 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:603 |
| FND-214 | medium | FR-018-AC-35 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:604 |
| FND-215 | medium | FR-018-AC-36 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:605 |
| FND-216 | medium | FR-018-AC-37 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:606 |
| FND-217 | medium | FR-018-AC-38 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:607 |
| FND-218 | medium | FR-018-AC-39 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:608 |
| FND-219 | medium | FR-018-AC-40 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:609 |
| FND-220 | medium | FR-018-AC-41 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:610 |
| FND-221 | medium | FR-018-AC-42 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:611 |
| FND-222 | medium | FR-018-AC-43 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-018-composite-equality-oracles.md:612 |
| FND-223 | medium | FR-021-AC-30 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-021-function-application-oracles.md:277 |
| FND-224 | medium | FR-031-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:504 |
| FND-225 | medium | FR-031-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:508 |
| FND-226 | medium | FR-031-AC-16 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:518 |
| FND-227 | medium | FR-031-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:519 |
| FND-228 | medium | FR-031-AC-27 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:521 |
| FND-229 | medium | FR-031-AC-20 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:524 |
| FND-230 | medium | FR-031-AC-22 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:526 |
| FND-231 | medium | FR-031-AC-26 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:527 |
| FND-232 | medium | FR-031-AC-23 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:528 |
| FND-233 | medium | FR-031-AC-24 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:529 |
| FND-234 | medium | FR-031-AC-25 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:530 |
| FND-235 | medium | FR-035-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:107 |
| FND-236 | medium | FR-035-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:108 |
| FND-237 | medium | FR-035-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:109 |
| FND-238 | medium | FR-035-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:110 |
| FND-239 | medium | FR-035-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:111 |
| FND-240 | medium | FR-035-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-035-caller-text-admission.md:112 |
| FND-241 | medium | FR-042-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:31 |
| FND-242 | medium | FR-016-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-016-witness-native-replay.md:134 |
| FND-243 | medium | FR-024-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:347 |
| FND-244 | medium | FR-024-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:352 |
| FND-245 | medium | FR-024-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:353 |
| FND-246 | medium | FR-024-AC-18 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:362 |
| FND-247 | medium | FR-024-AC-34 is bound only by ignored tests; default native execution does not qualify it, and no current full backend gate has run. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:378 |
| FND-248 | medium | FR-024-AC-35 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:379 |
| FND-249 | medium | FR-024-AC-36 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:380 |
| FND-250 | medium | FR-024-AC-37 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:381 |
| FND-251 | medium | FR-024-AC-38 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:382 |
| FND-252 | medium | FR-024-AC-39 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:383 |
| FND-253 | medium | FR-024-AC-40 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:384 |
| FND-254 | medium | FR-032-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:382 |
| FND-255 | medium | FR-032-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:385 |
| FND-256 | medium | FR-032-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:386 |
| FND-257 | medium | FR-032-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:387 |
| FND-258 | medium | FR-032-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:388 |
| FND-259 | medium | FR-032-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:389 |
| FND-260 | medium | FR-032-AC-9 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:390 |
| FND-261 | medium | FR-032-AC-10 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:391 |
| FND-262 | medium | FR-032-AC-11 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:392 |
| FND-263 | medium | FR-032-AC-12 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:393 |
| FND-264 | medium | FR-032-AC-13 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:394 |
| FND-265 | medium | FR-032-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:395 |
| FND-266 | medium | FR-033-AC-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:362 |
| FND-267 | medium | FR-033-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:363 |
| FND-268 | medium | FR-033-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:364 |
| FND-269 | medium | FR-033-AC-5 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:365 |
| FND-270 | medium | FR-033-AC-6 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:366 |
| FND-271 | medium | FR-033-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:368 |
| FND-272 | medium | FR-033-AC-10 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:370 |
| FND-273 | medium | FR-033-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:374 |
| FND-274 | medium | FR-033-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:375 |
| FND-275 | medium | FR-033-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:377 |
| FND-276 | medium | FR-019-AC-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:519 |
| FND-277 | medium | FR-019-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:547 |
| FND-278 | medium | FR-019-AC-24 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:548 |
| FND-279 | medium | FR-019-AC-34 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:558 |
| FND-280 | medium | FR-019-AC-35 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:559 |
| FND-281 | medium | FR-019-AC-36 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:560 |
| FND-282 | medium | FR-019-AC-37 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:561 |
| FND-283 | medium | FR-019-AC-38 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:562 |
| FND-284 | medium | FR-019-AC-39 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:563 |
| FND-285 | medium | FR-019-AC-40 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:564 |
| FND-286 | medium | FR-019-AC-41 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-019-capability-settlement.md:565 |
| FND-287 | medium | FR-022-AC-26 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-022-routed-generation.md:361 |
| FND-288 | medium | FR-022-AC-27 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-022-routed-generation.md:362 |
| FND-289 | medium | FR-022-AC-28 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-022-routed-generation.md:363 |
| FND-290 | medium | FR-026-AC-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-026-backend-adapter-contract.md:60 |
| FND-291 | medium | FR-026-AC-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/routed/functional/FR-026-backend-adapter-contract.md:61 |
| FND-292 | medium | FR-002-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:91 |
| FND-293 | medium | FR-002-AC-8 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:92 |
| FND-294 | medium | FR-002-AC-10 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:94 |
| FND-295 | medium | FR-002-AC-11 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:95 |
| FND-296 | medium | FR-002-AC-12 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:96 |
| FND-297 | medium | FR-002-AC-13 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:97 |
| FND-298 | medium | FR-002-AC-14 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:98 |
| FND-299 | medium | FR-002-AC-15 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:99 |
| FND-300 | medium | FR-002-AC-16 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:100 |
| FND-301 | medium | FR-002-AC-17 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:101 |
| FND-302 | medium | FR-002-AC-18 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-002-tristate-proptest.md:102 |
| FND-303 | medium | FR-008-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:123 |
| FND-304 | medium | FR-008-AC-9 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:125 |
| FND-305 | medium | FR-009-AC-7 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/functional/FR-009-constructive-correlated-populations.md:77 |
| FND-306 | medium | NFR-004-M-1 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-307 | medium | NFR-004-M-2 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-308 | medium | NFR-004-M-3 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |
| FND-309 | medium | NFR-004-M-4 has no test trace binder in the computed matrix; inherited from current main and unqualified. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:1 |

## Coverage

Plan completion: not assessed

Fresh quire matrix was executed by this reviewer on the current frozen candidate:880rows,521tagged,291untagged,18ignored-only,50method-without-symbol (quire0.36.2/engine0.50.2). Compared supplied current-base matrix against this independently generated head: all880IDs/statements/methods identical; only AC82/83/86/88 become structurally tagged; zero new untagged/ignored-only. Every291untagged and18ignored-only row gets its own finding,309total. Method-without-symbol is separate and not automatically Test coverage. Matrix tagged status is structural, not full intent or dynamic proof evidence; code-review identifies introduced partial criterion coverage. Original old IR878 handoff has305rows and was verified as context only; it does not automatically dispose this fresh309finding set. Root must explicitly hand off the current inventory before a disposition pass.

Whole-tree reverse audit: public crate exports and master spec registry cover seven implemented behavior groups (core, strategy, oracle, Kani, routed, replay, evidence);208public-function/re-export declaration lines inventoried. Family classify/collection/graph APIs expressly map to FR015AC82-89; remaining groups have their actual owning FRs in master registry. No newly unowned public behavior found. Whole source/test placeholder scan found no executable source stub; test hits are deliberate malformed generated-source/parser fixtures. No hollow source/test implementation or unrequested API was found. Counts are source declarations/behavior groups, not independent full dynamic coverage of every branch. No plan supplied, no optional semantic fanout requested, no line coverage measured. Ordinary code-review intent checks still apply to changed assertions.

All inherited gaps remain unqualified even if their criteria say PLANNED. Existing ownership recorded by IR878 is luna:IR:plan for non-Kani and titan:kani:plan/assigned proof queue for Kani. This current review does not grant that deferral itself. Full repository verdict is FAIL, independently of a source slice being correct.

Independently read current resume/focused.log: actual asserted memory.max17179869184/swap0, fmt-check and locked all-target Clippy PASS,9profile/family+4syntheticclassification+1recordedzero-SUCCESS+2terminal=16actual native tests PASS,0failed/ignored, peak4,935,614,464bytes/events0. Those passing partial controls do not resolve the criterion gaps. Recorded-real-Kani capture classification is not a live backend proof. NFR006 explicitly applies to src/kani and Cargo.lock; required full make kani-gate is PENDING on the final clean/stable candidate and cannot be waived by these reviews or native logs. Root aggregate is also PENDING. No merge-ready or backend-verified conclusion is given.

## Examined Scope

```yaml
scope:
- id: FR-005-AC-1
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Failed generation leaves no partial published bundle or edited developer
    region.
- id: FR-005-AC-2
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Supported platforms reproduce byte-identical files.
- id: FR-005-AC-3
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Differential Disposition Register holds exactly one row naming
    the case, the backends, and either a regression fixture that exists in the test
    tree or a documented profile/backend distinction; a differing case with no row,
    a row naming a fixture that does not exist, or a row with neither disposition
    fails the check.
- id: FR-005-AC-4
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: Executable, proptest, Kani, and coverage semantics agree or retain an explicit
    difference.
- id: FR-005-AC-5
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: A bundle naming more than 4096 artifacts, an artifact whose contents exceed
    16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes
    is refused at construction as a bounded-resource `InvalidBundle` diagnostic before
    any staging or destination I/O begins, so no partial bundle is ever published.
- id: FR-005-AC-6
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When an artifact path is absolute, or contains a `..` segment, a `.` segment,
    an empty segment, a trailing separator, a backslash or a control character, or
    is empty, bundle construction is refused with `UnsafeArtifactPath` naming the
    offending `bundle.artifacts[i].path`, before any destination or staging I/O begins.
- id: FR-005-AC-7
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When publication fails before the destination is replaced, or fails after
    replacement began and the prior destination is restored, the publisher reports
    destination state `unchanged`, the destination holds its prior bytes or remains
    absent, an adjacent developer-owned file is byte-identical, and no staging or
    backup sibling remains.
- id: FR-005-AC-8
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When removal of the replaced bundle's backup fails after the staged bundle
    has replaced the destination, the publisher reports destination state `published`,
    the destination holds every artifact of the new bundle, and the backup sibling
    remains as visible residue.
- id: FR-005-AC-9
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When replacement fails and restoring the prior destination also fails,
    the publisher reports destination state `unknown`, the destination path is absent,
    and the complete prior bundle and the complete staged bundle remain in their sibling
    directories for recovery.
- id: FR-005-AC-10
  path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  excerpt: When two artifacts of one bundle share the same path, bundle construction
    is refused with `DuplicateArtifactPath` naming the later `bundle.artifacts[i].path`,
    before any destination or staging I/O begins.
- id: FR-037-AC-1
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Both `tests/exact_scalar_support/package.rs` and `tests/composite_equality_support/package.rs`
    derive every candidate application node identity through `derive_application_node_id`
    over the complete draft node; neither retains an application preimage encoder,
    application version constant, or SHA call for that identity. An ungrouped fixture's
    candidate equals the checked reader's admitted node ID.
- id: FR-037-AC-2
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A fixture containing an application in a recursion group supplies the complete
    ordered draft graph to `ApplicationIdentityContext`; its candidate agrees with
    the reader after group-reference rewriting. Missing or mismatched context and
    a canonical byte-limit refusal retain Contract IR's typed error and produce no
    candidate.
- id: FR-037-AC-3
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: A canonical enum-member fixture identity is derived through `NominalIdentityPreimage::EnumMember(...).digest`,
    agrees with the checked reader, and changes when the member case changes. In-process
    agreement keys that are never compared with a checked-package identity remain
    classified as fixture-only keys.
- id: FR-037-AC-4
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: Positive exact-scalar and composite-equality packages remain admitted.
    A stale candidate key or a body mutation without rekeying is refused by the checked
    reader; the fixture builder does not repair those negative controls before the
    assertion.
- id: FR-037-AC-5
  path: spec/core/functional/FR-037-fixture-identity.md
  role: examined
  excerpt: The consumer change leaves deterministic fixture-only keys distinct from
    application identities and uses `quire-canonical` for ordinary canonical-value
    hashes where those hashes are needed. Package identity refresh uses Contract IR's
    typed package identity preimage and `quire-canonical`; no independent application
    or package preimage rule is added.
- id: FR-038-AC-1
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: 'A source inventory compares every identity in [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)''s
    specified derivable set with the implemented scalar, scalar Kani and state/frame
    selections, recording the unimplemented scalar integer `eq/ne` and Boolean variants
    as open FR-014 work. For the four integer arithmetic identities shared by scalar
    oracle and scalar Kani, and the four integer ordering identities shared by scalar
    oracle and state/frame Kani, the implemented consumers resolve the same typed
    identity, family, arity and ordered operands; no second string-to-operator match '
- id: FR-038-AC-2
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The typed source distinguishes implemented integer, rational, decimal,
    IEEE, text, enum and quantity operations and their applicable result and mode
    forms. When FR-014's specified Boolean scalar variants are implemented, they use
    distinct Boolean entries in this same source before the scalar emitter selects
    them. A valid caller-supplied scalar descriptor with a present but disagreeing
    node identity, law or mode still generates its descriptor-named oracle as `caller_declared`
    with a typed blocked item under FR-014-AC-12 and AC-13. A missing node identity
    retains its typed refusal; an underivabl
- id: FR-038-AC-3
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: Focused V2 oracle, scalar Kani and state/frame tests distinguish the wrong
    identity, family, operand arity or order, comparison symbol and applicable runtime
    callee from the correct rendering. With the correct identity and arity, a wrong
    fixed result form or wrong member-selected result target, including a conversion
    target, is refused under FR-014's result/parameter checks rather than rendered
    as the right operator with the wrong result. Existing admission and generated-artifact
    behavior for supported identities remains equivalent.
- id: FR-038-AC-4
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The four scalar Kani harnesses still compare their embedded oracle result
    with a separately rendered native `i128` arithmetic expression. A mutation of
    the production oracle's selected operation or callee makes the real Kani proof
    fail under an in-domain non-vacuous control; routing the native expression through
    that callee fails review.
- id: FR-038-AC-5
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: The V1 `strategy/bound` relation evaluator remains outside this V2 catalog
    migration until the V2 strategy replacement specified by [AD-004](../../assurance/AD-004-cg-crate-layout.md)
    and [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md).
    The hand-built bounded corpus is also excluded, and its duplicate `checked_method`
    mapping is removed only by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    and AD-004 step 4g retirement after that step's proof controls pass. Neither path
    gains a compatibility forwarding layer.
- id: FR-038-AC-6
  path: spec/core/functional/FR-038-typed-operator-catalog.md
  role: examined
  excerpt: While the V1 `strategy/bound` relation evaluator remains live, focused
    tests discriminate all six comparison operators and a primary read on the left
    versus on the right of a noncommutative comparison; its host evaluation preserves
    the relation as authored.
- id: FR-039-AC-1
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Each of two genuine QSL-admitted source graphs with a different clause
    or pure-function operation lowers through IR to its own admitted FR-330 package.
    CG's production settlement and generation use each package's typed graph and accounting
    for its exact request index and node ID; supplying the two packages with each
    one's corresponding node ID at the same request position changes the generated
    bytes to reflect the represented operation while backend and generation context
    stay fixed. If the production API still accepts an original checked graph, holding
    that legacy input fixed across the two c
- id: FR-039-AC-2
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A mixed request with a representable clause or function and an independent
    `unsupported`, `requires_bound` or `invalid_input` sibling retains one typed decision
    per bound driver item, including duplicate requested IDs, and emits only the representable
    sibling's artifact. An interleaved context-only root retains its own exact accounting
    position and key without creating a driver decision. For two QSL scalar items
    sharing one expression key but selecting different checked application/conversion
    pairs, CG retains each pair's own admitted FR-049 result bound; a conversion outside
    its driver's cont
- id: FR-039-AC-3
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A selected dependency function reference resolves only through the reader-validated
    selected package identity and target key. A missing, wrong-identity or stale selected
    dependency refuses before CG; an identically keyed local node or extra unselected
    dependency cannot redirect generation.
- id: FR-039-AC-4
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference`
    carrier's exact `body.target` to the local `protocol` node, retaining its typed
    key, requested identity and source occurrences; changing that local target and
    recomputing the admitted carrier and package identities changes the selected target
    and package identity. A matching key only in a selected dependency does not bind
    locally. An absent local target, a present non-`protocol` target and a target
    with unequal `semantic_type` are refused by the FR-322 reader before CG with the
    respective `missing_declaration`/
- id: FR-039-AC-5
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: A malformed or noncanonical FR-330 wire, wrong version, invalid accounting/source
    map, stale semantic identity or incomplete reader result reaches no CG consumer
    and emits no artifact. CG preserves the reader's typed refusal or limit result
    and performs no second parse or checked-graph fallback.
- id: FR-039-AC-6
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: An item whose admitted package accounting is `failed`, `invalid_body` or
    `body_incomplete`, or whose `lowered` form is unrepresentable by the selected
    backend, emits no artifact and retains a distinct typed cause; the package byte-ceiling
    case emits no artifact for any sibling.
- id: FR-039-AC-7
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Generated scalar, composite equality and exact function claims each serialize
    `contract_package_identity` equal to the admitted FR-330 `identity` and `node_id`
    equal to the requested `CheckedNodeId`, with no generated-claim `ir_id` key. Each
    retains exact `semantic_type`, admitted `source_map`, family-specific provenance,
    and `package_id` denoting the originating checked package; each serializes typed
    `claims`, `bounds` and `dependencies`. For the same requested node in two genuinely
    admitted packages with different identities, the claim identity differs; for two
    different requested nodes in o
- id: FR-039-AC-8
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: 'For each admitted `lowered` bound driver item, Contract IR''s authoritative
    traversal is total over the required root, bounds and selected dependencies, and
    the generated claim''s reached claim, bound and dependency keys and dependent
    declaration closure equal its output. An unresolved required reference cannot
    reach CG as an admitted `lowered` item: Contract IR records a typed item refusal
    or its strict reader refuses the package; a refused or incomplete reader result
    yields no item claim or artifact. Inspection finds no per-node ID from a second
    lowering or wire-fragment hash.'
- id: FR-039-AC-9
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: In an admitted package with a bound driver item, an excluded accounting-only
    position and another bound driver item, CG matches all three positions and exact
    `node_id` values to `feature_accounting`; the excluded position keeps its driver
    index but has no claim or generated artifact, and the following bound item keeps
    its original driver index. An interleaved context-only root has accounting but
    advances neither driver nor envelope item indexes. Removing the excluded position,
    adding an extra one or changing its `node_id` causes a typed whole-request correspondence
    refusal before any artifact.
- id: FR-039-AC-10
  path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  excerpt: Given a bound driver item followed by an excluded accounting-only position,
    a selected conversion context root outside the preceding item's contiguous span
    yields typed `IncompleteContext` for that item when the complete root sequence
    is otherwise valid. A context-only root directly after the excluded position and
    before the next bound driver item is orphaned and causes a typed whole-request
    correspondence refusal before any artifact, even when its accounting position
    and `node_id` match. A conversion root immediately before the excluded position
    remains in the preceding item's span.
- id: NFR-001-AC-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Identical inputs and configuration produce byte-identical bundles.
- id: NFR-001-AC-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: An injected generation failure leaves no partially published bundle.
- id: NFR-001-AC-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Generation does not modify developer-owned regions or files.
- id: NFR-001-M-1
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Byte differences between repeated generations
- id: NFR-001-M-2
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Partial published bundles after injected failure
- id: NFR-001-M-3
  path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  excerpt: Developer-owned files modified
- id: NFR-002-AC-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file in a family named in Scope has a header naming
    `quire-contract-codegen`, every input-carried subject identity it renders as defined
    in Scope, and every requirement, revision or clause identifier present in the
    generation input.
- id: NFR-002-AC-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity.
- id: NFR-002-AC-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Unsupported and inconclusive states remain explicit rather than silently
    succeeding.
- id: NFR-002-AC-4
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated output makes no project-specific validation, accreditation, certification,
    or release-approval claim.
- id: NFR-002-M-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated Rust files missing generator or input-carried subject identity
- id: NFR-002-M-2
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Generated files missing dual-license SPDX header
- id: NFR-002-M-3
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  excerpt: Silent unsupported/inconclusive states
- id: NFR-005-AC-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Every `.rs` file under `src/`, except a file its parent module declares
    under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments,
    every `#[cfg(test)]` item and every string and character literal removed (`non_test_code_outside_literals`,
    a variant of `non_test_code` that `tests/common/panic_scan.rs` gains), including
    the body of `fn digest` of `impl CaseIdentity` in `src/kani/generate/corpus/bounded_kani_corpus.rs`;
    no `expect` in that file, `render_artifacts` included, is allowed. The tokens
    are the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `
- id: NFR-005-AC-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)`
    for a claim whose `checked_bounds` is empty and for a claim whose first checked
    bound has no `IntegerRange` among the derived domains, each built directly against
    the function, and `classify_claim` reports each as `UnsupportedObligation::OperationNotRendered`.'
- id: NFR-005-AC-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`ItemSettlement::warning` returns `None` for a `Disposition::Unsupported`
    carrying each of `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`,
    `InconsistentCandidates` and `AmbiguousBackend`, and still returns its warning
    for each `unsupported_projection` cause.'
- id: NFR-005-AC-4
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: With a coverage export supplied, `observe_clause` given a map whose oracle-evaluation
    region has no probe returns a row with a `MapMismatch` diagnostic with the message
    `missing semantic probe`, no classification and an `evaluation_count` of `None`,
    and given one whose consequent region has no probe returns the same diagnostic
    with no classification. With no coverage export it returns `UnavailableObservation`
    for the same maps.
- id: NFR-005-AC-5
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The step of `generate_kani` that pairs FR-015's records with the Kani group,
    given hand-built inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniRecordCountMismatch
    { records, items }` carrying both counts when the counts differ, in both directions,
    and pairs every item with its record when they are equal.
- id: NFR-005-AC-6
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`rewrite_duplicate_position`, a function of `src/routed/generate.rs` that
    `route_records` (which `generate_kani` calls to pair, rewrite and join FR-015''s
    records) calls to rewrite a `DuplicateItem` record''s `first_index` into the driver''s
    request index, given hand-built records and a group in a `#[cfg(test)]` module,
    returns `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index,
    items }` carrying the position and the group''s length when `first_index` equals
    or exceeds the group''s length, with nothing generated, and returns the request
    index of the earlier item when `first_index'
- id: NFR-005-AC-7
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: '`observe_clause`, given a map of zero regions and a map of one region,
    each with and without a coverage export, returns a row with a `MapMismatch` diagnostic
    with the message `typed implication census differs from map`, no classification,
    an `evaluation_count` of `None` and no consequents; given a map of two probed
    regions and a coverage export it returns a row with no consequents and no `MapMismatch`
    diagnostic.'
- id: NFR-005-AC-8
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: The scan locates the body of `generate_kani`, `route_records`, `rewrite_duplicate_position`,
    `observe_clause` and `generate_boolean_oracle_inner` in the literal-free non-test
    code of its file, asserts that it found each of the five and that each holds an
    identifier of its own, and asserts that none contains an index token or a subtraction
    token, so that a later change cannot bring back one of the IR-577 sites. An operand-ending
    token is `)`, `]`, `}`, `?`, a numeric literal (so the `0` of a tuple field `t.0`
    counts), or an identifier that is not a Rust keyword (`in`, `let`, `mut`, `ref`,
    `retu
- id: NFR-005-M-1
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Panic tokens in the non-test, literal-free code of `src/`
- id: NFR-005-M-2
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Measured sites a test can reach that abort instead of returning a typed
    value
- id: NFR-005-M-3
  path: spec/core/non-functional/NFR-005-no-generation-panics.md
  role: examined
  excerpt: Index, slice and unchecked subtraction expressions in the bodies of the
    five IR-577 functions
- id: FR-004-AC-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: An evaluated implication whose consequent is unobserved yields a vacuity
    finding even when every oracle return was true; the consuming coverage obligation
    denies success for that clause.
- id: FR-004-AC-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: A clause whose oracle-evaluation region was not observed is unexecuted,
    not vacuous.
- id: FR-004-AC-3
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: The four measured clause classifications form a total partition; an implication-bearing
    clause is exercised only when every expected consequent entry probe is observed,
    and implication-free clauses require observed evaluation.
- id: FR-004-AC-4
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Successful analysis verifies the bound population, source/map, and native
    execution bindings.
- id: FR-004-AC-5
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Malformed, summary-only, identity-mismatched, path-ambiguous, unsupported,
    or missing observation inputs retain structured non-success outcomes without invented
    classifications.
- id: FR-004-AC-6
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- id: FR-004-AC-7
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Removing an expected clause, consequent, evaluation probe, or all clauses
    prevents successful analysis; the expected census comes from bound typed IR.
- id: FR-004-AC-8
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Adverse coverage and non-success native execution cannot discharge the
    coverage obligation merely because a report serialized successfully.
- id: FR-004-AC-9
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: Complete bound observations without a native campaign run are reported
    as observations only and never as a passed coverage result. Valid informational-only
    populations retain their references as no executable work.
- id: FR-004-AC-10
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. CG binds coverage inside the same live campaign-runner invocation,
    using the original `/2` wire bytes and their QSL strict-reader typed result directly,
    the producer execution record, and the same generated source/map buffers used
    by that campaign. The runner starts the campaign with fresh private profile output,
    performs any required LLVM merge over only that output, starts LLVM export over
    only that profile, and captures exact export bytes and tool/version from that
    LLVM child's direct output stream while retaining the linked child handles. Binding
    verifies this live campaign/profil
- id: FR-004-AC-11
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence
    for IR FR-045; an oracle-success run with an unobserved consequent denies the
    coverage obligation, and LLVM probe observations alone produce no completed-proof
    credit or separate eligible denominator.
- id: FR-004-AC-12
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. CG consumes QSL''s typed strict-reader clause-run or command-error
    variant for exact `native-run-result/2`: a decisive basis retains its witness
    with assigned members, while `closed-scope` or `unavailable` retains none. Missing
    or other versions, missing/unknown basis, malformed witness, contradictory basis/witness
    and mixed/invalid variant shape are located wire refusals before binding; no `/1`
    fallback or CG copy of the wire reader exists.'
- id: FR-004-AC-13
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause
    facts and do not count as generated-campaign execution or LLVM coverage; QSL violation,
    refusal, undefined, incomplete, cancelled, unsupported and internal failure remain
    distinct typed adverse/non-success facts. A bound live run does not upgrade any
    QSL disposition or itself grant campaign success, measured classification, proof
    credit or coverage discharge. The same reader's command-error variant retains
    stage/code/optional cause/message/details and unavailable basis as non-success,
    with no campaign qualification o
- id: FR-004-AC-14
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable,
    non-serializable live run context; callers cannot construct or restore it from
    a `/2` document, paths, QSL `package_id`, metadata, or a forwarded report. Absent,
    foreign or expired context, or an omitted association yields structured binding
    non-success retaining the typed QSL outcome and available identities, with no
    qualification, classification or discharge. A complete same-invocation context
    admits binding without changing the QSL outcome. The caller supplies maximum encoded-input
    and decoded-working-byte limits; CG '
- id: FR-004-AC-15
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result,
    execution record, generated source, source map, LLVM export stream or LLVM tool/version
    with a foreign-run value cannot construct a valid same-invocation context, even
    when paths, QSL `package_id`, source/map and tool/version declarations match.
    In particular, foreign export bytes offered through a detached buffer or a different
    producer stream/child handle refuse as `producer_run_mismatch` before classification
    because neither is the current runner-owned campaign/profile/export chain. A malformed
    association observable i
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2`
    clause bodies.
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2`
    body only through the `core/ir/` layer of AD-004, and names no `BoundPackage`
    or `ClauseRef`, both of which today's `analyze_bound_coverage` names.
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication
    count unequal to the count derived from its body yields a structured binding non-success
    naming the clause and no classification, and is never treated as implication-free.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected consequent
    probe was observed is `vacuous`.
- id: FR-004-AC-20
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation
    region has no positive execution count is `unexecuted`.
- id: FR-004-AC-21
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause
    for which some but not all expected consequent probes were observed is `partially_exercised`.
- id: FR-004-AC-22
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause
    with every expected consequent probe observed, and an implication-free clause
    with its evaluation region observed, is `exercised`.
- id: FR-004-AC-23
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, removing one clause, one
    consequent probe, one evaluation probe, or every clause from the generated artifacts,
    each alone, prevents successful analysis.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-036-AC-1
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: 'A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source
    identities S1/S2, domain D and two production functions in one module exports
    three obligation inventory entries and exactly four candidate keys for authored
    tuples (C1,O1,S1,D), (C2,O1,S1,D), (C1,O2,S1,D), (C1,O2,S2,D). O3 has no authored
    criterion correspondence: it remains an inventory/gap record with no candidate.
    Missing production correspondence and authored no-function remain distinct gaps
    on complete candidate keys; a zero-mapping source symbol remains an inventory/gap
    record. Removing only C2''s relation leaves C'
- id: FR-036-AC-2
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported
    or pre-run refusal, retrying and changing method/profile/subject/artifact/run
    leave the complete obligation inventory, all four criterion-obligation-source-domain
    keys and mapping-gap records unchanged; enumeration precedes producer invocation.
    Removing O1 and its authored relations from the authoritative source removes its
    inventory entry and exactly the two O1 candidate tuples without editing an auxiliary
    catalog.
- id: FR-036-AC-3
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome;
    pre-run refusal facts preserve producer/stage/cause separately from actual execution
    outcomes. An absent decision or observation and an unfamiliar producer state remain
    distinct, and missing, counterexample, invalid/incomplete, unavailable, timeout,
    exhaustion, cancellation and inconclusive evidence states survive export.
- id: FR-036-AC-4
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A matching executed production fact retains its actual typed artifact and
    run identities separately from attempt identity, exact source/profile/subject/domain/bounds
    and explicitly bound load-bearing check plus production reachability. Wrong, missing
    and mismatched artifact/run identity controls preserve the received typed identity
    or explicit missing reason for IR joining, without inventing or repairing identities.
    Zero-check, check-location-only, tag-only, LLVM-only and incidental-reachability
    controls never acquire positive observations by export.
- id: FR-036-AC-5
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed
    sampled native refinement retain distinct subject/scope/assumption observations.
    Missing refinement stays missing; adding a refinement observation does not change
    the census or create an unqualified-production-proof flag.
- id: FR-036-AC-6
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types
    over the reviewed neutral correspondence contract without source/transcript/name
    parsing or a copied schema. Duplicate and artifact/run/source/profile/subject/domain-mismatched
    result controls survive as separate inspectable records without changing inventories
    or candidate tuples; repeated identical inputs produce identical ordering.
- id: FR-036-AC-7
  path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign
    selection and a dangling relation endpoint produce distinct typed census refusals
    with their offending input/relation and no complete census. A valid absent correspondence
    remains an explicit obligation/criterion/source-symbol gap, or a production gap
    on an existing complete candidate key, instead of that global refusal.
- id: FR-015-AC-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Pre, post, invariant and frame obligations of one contract produce separate
    harnesses with distinct identities.
- id: FR-015-AC-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every harness identity records its model-domain bounds, taken only from
    IR `bounded_domain` nodes, and its solver and options and unwind bound.
- id: FR-015-AC-3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An unbounded or non-finite obligation is refused with a typed reason and
    no harness.
- id: FR-015-AC-4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: No harness assumption excludes an undefined, refused or incomplete runtime
    outcome.
- id: FR-015-AC-5
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation whose bounds are unsatisfiable is refused with a typed reason
    and no harness.
- id: FR-015-AC-6
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An obligation over an oracle whose operation identity is `caller_declared`
    is refused with a typed reason and no harness.
- id: FR-015-AC-7
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every generated harness contains exactly one non-vacuity cover; a precondition
    harness covers that the precondition holds within the IR bounds, and a contract
    harness''s cover stands after the contract call, so a run that satisfies every
    check without satisfying the cover is not a proof. The cover is the last statement
    of the harness, after every assertion the harness carries: the state-clause and
    frame-effect harnesses and the scalar harness place it after their assertions,
    and a contract harness''s checks are inside the contract call its cover follows.
    A cover that preceded an assertion would '
- id: FR-015-AC-8
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A postcondition or invariant harness emits every package precondition sharing
    its anchor operation as a `requires` on its generated contract and records each
    in its identity's embedded oracles; it embeds no other obligation's oracle; and
    an obligation whose sibling precondition is not a supported item of the same request
    is refused with a typed reason naming that precondition and no harness.
- id: FR-015-AC-9
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every harness identity records solver `cadical` and the complete ordered
    option vector — function contracts, concrete playback, the exact fully qualified
    harness, `--exact`, the explicit unwind, the explicit solver, `--output-format
    regular`, and `--concrete-playback print` — and no option enabling stubbing is
    emitted.
- id: FR-015-AC-10
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and changing the unwind
    bound or the customer subject changes the harness identity.
- id: FR-015-AC-11
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every symbolic argument carries an inclusive assumption equal to its IR
    `bounded_domain` or a literal operand's own value (FR-015-AC-16); for a V2 scalar
    claim, an operand with the FR-014 inferred application interval uses that interval
    (FR-015-AC-103). A bounded-integer post-state result is required to lie in the
    same domain; no generated source carries a `#[kani::unwind]`.
- id: FR-015-AC-12
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request naming no items, more than 256 items, an unparsable subject path,
    or an unwind bound outside `1..=1024` is refused whole, with no item accounted
    and no harness exposed.
- id: FR-015-AC-13
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An otherwise-supported obligation whose generated harness source exceeds
    the bounded-resource ceiling is refused with a distinct resource-limit reason
    naming the generated size, and one that fits the ceiling but fails to parse as
    Rust is refused with a distinct syntax reason naming the parse error; neither
    is reported as the internal-invariant render-assembly fallback.
- id: FR-015-AC-14
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose graph node carries a tag/form pair this generator
    does not model as a contract role is refused with a typed reason naming that tag
    and form, and no harness is emitted; a claim naming a node id absent from the
    graph entirely is refused rather than accounted as supported with no contract
    role recorded.
- id: FR-015-AC-15
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields
    `unsupported` with the `no_derivable_claim` reason naming the node and the derivation
    refusal, emits no harness, and does not reject the group.
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar harness constrains a literal operand, inline or a reference to
    a `value` node whose body is a literal, to exactly its own value: `x + 1` over
    `x: Int[0, 9]` into `Int[0, 10]` has arguments `[0, 9]` and `[1, 1]`.'
- id: FR-015-AC-17
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is
    `unsupported` as `domain_not_representable_in_i64` naming the literal as both
    endpoints, with no harness.
- id: FR-015-AC-18
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar claim whose operation over its operand ranges yields no result
    inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable
    `[100, 109]`) is `unsupported` as `result_bound_unreachable` naming both ranges,
    with no harness.'
- id: FR-015-AC-19
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A zero-input transition over one bounded-integer state value verifies for
    an identity subject and is falsified with a concrete counterexample for a subject
    that changes the value.
- id: FR-015-AC-20
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a Boolean-connective or bounded-integer comparison claim
    (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds
    its FR-014 oracle byte-identical to the oracle crate's function.
- id: FR-015-AC-21
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A precondition that reads a post-state value is refused with a typed reason
    naming the node, and no harness is emitted.
- id: FR-015-AC-22
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A declared proof-dependency census with an empty or duplicate identity,
    an inconsistent kind, state and path combination, or a non-`Required` kind is
    refused as a typed invalid input with no harness.
- id: FR-015-AC-23
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`,
    `requires-bound`, `unsupported` or `invalid-request`), and an item that is not
    `supported` keeps its source identity and typed reason with no harness and no
    assumption emitted for it.
- id: FR-015-AC-24
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison
    claim is reported through Kani's concrete playback.
- id: FR-015-AC-25
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid declared census is folded into the harness identity, readiness
    is `ready` only when every dependency passed and `incomplete` while any is missing
    or failed, and proof execution is recorded `not_run`.
- id: FR-015-AC-26
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: One `postcondition` `state_clause` of an admitted package, whose condition
    is one integer comparison of the pre and post reads of one field through its first
    parameter `self`, yields a separate operation-contract harness and frame-effect
    harness for the operation its anchor names. Each has exactly one non-vacuity cover,
    and its identity is scoped to the operation, anchor, frame and framed object;
    the two harnesses of two clauses of one operation are written to different paths.
- id: FR-015-AC-27
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The operation-contract harness assumes each ranged state field within the
    inclusive `i64` endpoints returned by `model_object_fields` for the QSL-emitted
    selected model declaration (FR-015-AC-77). It draws a present non-clause field
    with no usable range without an assumption (FR-015-AC-78), asserts the clause's
    comparison between pre-state and post-state, and refuses a clause whose own field
    has no usable range. A non-model object or admitted unselected model/object_type
    body is refused rather than used as a range source (FR-015-AC-78). PLANNED (IR-624).
- id: FR-015-AC-28
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-effect harness takes the granted fields from the frame node's
    `modifies` entries and asserts every other state field the caller names unchanged.
- id: FR-015-AC-29
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A state clause that is not a postcondition, whose condition is not one
    comparison of the pre and post reads of one field through `self` (a literal operand,
    a negation, two fields, two reads of one side, or a read through another parameter),
    whose frame creates, deletes or grants a relationship or a foreign field, whose
    frame grants every state field, whose field the caller's state lacks, or whose
    clause field declares no integer range, is refused with a typed reason and no
    harness; a malformed request is refused first.
- id: FR-015-AC-30
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the operation contract verifies for a healthy
    subject and is falsified, naming the postcondition, for a subject mutated to debit.
- id: FR-015-AC-31
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden
    effect is falsified naming the forbidden field, and regenerating the frame from
    a package whose `modifies` is mutated to grant nothing falsifies the allowed subject.
- id: FR-015-AC-32
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a
    frame violation of that field through QSL's `replay_frame`, and the allowed run
    replays as a respected frame.
- id: FR-015-AC-33
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay payload's anchor, frame and frame occurrence are the values
    `qsl_replay::call_site` names for the operation's name.
- id: FR-015-AC-34
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay envelope's `clause_node` is the payload's frame node and
    its `occurrence_key` is the payload's frame occurrence.
- id: FR-015-AC-35
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An operation the unit names no frame for, whether the domain package declares
    it or not, is refused by the call site when the frame-replay request is built.
- id: FR-015-AC-36
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`FrameReplay::replay` returns QSL''s `replay_frame` result: a forbidden
    write settles a reproduced violation naming the written field, and a write the
    frame grants settles `inconclusive` with no frame witness.'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar function-application harness asserts the oracle''s outcome against
    the clause''s operation evaluated natively in `i128` over the same symbolic operands,
    independently of the oracle: `Completed` with exactly that value when it lies
    in the result bound, `Refused` when it does not, and any other outcome fails.
    With the installed backend, the healthy oracle verifies and an oracle whose arithmetic
    is mutated is falsified on that assertion.'
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `invariant`
    yields one harness of that obligation kind, separate from the harness of every
    other clause of the package; the kind is read from the node and no caller value
    changes it. PLANNED (IR-489).
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state field of `self` the body reads, whether bare or under
    `pre(...)` (the subject''s input state; how it reaches the subject is FR-025''s),
    and each operation parameter (for a precondition or invariant, each bare state
    field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected
    model declaration state field (FR-015-AC-77), and the IR `bounded_domain` of a
    parameter''s type. A non-model or admitted unsel'
- id: FR-015-AC-40
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression node, contains Boolean connectives and bounded-integer
    comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons)
    embeds an FR-014 oracle of that condition byte-identical to the oracle crate's
    clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a comparison lowers, and its harness keeps the independent
    native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle''s outcome
    against the operation evaluated natively in `i128` over the same operands, `Refused`
    assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000`
    over `amount: Int[0, 1000]` yields one contract harness. With the installed backend
    the unmutated clause verifies with its cover satisfied, and the same clause with
    `+` mutated to return `left'
- id: FR-015-AC-42
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause body holding an operator the V2 arm does not support (division,
    modulo, absolute value, and every operator outside connectives, the six integer
    comparisons and add, subtract, multiply, negate; those operators are planned scope
    for a later criterion) is `unsupported`, with a typed reason naming the node and
    the operator and no harness; a node that is not a modelled clause role, or a node
    id absent from the package, is refused as FR-015-AC-14 states; no item is skipped
    silently, and a supported item of the same request is unaffected. PLANNED (IR-489).
- id: FR-015-AC-43
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing
    its anchor node as a `requires` and records each in its identity's embedded oracles,
    embeds no other obligation's oracle, and a postcondition whose sibling precondition
    is not a supported item of the same request is refused naming that precondition
    with no harness; a V2 invariant is anchored at an object type, binds `self` alone,
    and so has no operation precondition to assume. PLANNED (IR-489).
- id: FR-015-AC-44
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency
    identity, an inconsistent kind, state and path combination, or a non-`Required`
    kind is refused as a typed invalid input with no harness, by the same rules as
    FR-015-AC-22; the census request and readiness types it uses are not `ProofDependencyGraph`.
    PLANNED (IR-489).
- id: FR-015-AC-45
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's
    identity record, readiness is `ready` only when every dependency passed and `incomplete`
    while any is missing or failed, and proof execution is recorded `not_run`, by
    the same rules as FR-015-AC-25; in addition, the folded census is independent
    of the order the caller lists its dependencies. PLANNED (IR-489).
- id: FR-015-AC-46
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every V2 clause harness contains exactly one non-vacuity cover, placed
    after every assumption, after the subject call and after every assertion the harness
    carries (the last statement, as FR-015-AC-7 states), that states the family''s
    reachability, as FR-015-AC-7 does: a precondition harness covers that the precondition
    holds within the bounds; a postcondition harness covers, after the subject call
    returns, that the requires and the bounds are jointly satisfiable; an invariant
    harness covers that the drawn state satisfies its domain assumptions. The subject
    call is the call of the customer subj'
- id: FR-015-AC-47
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical,
    and changing the unwind bound or the customer subject changes the harness identity
    record (as FR-015-AC-10 states). PLANNED (IR-489).
- id: FR-015-AC-48
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from
    the clause node id, its occurrence key, the obligation kind, and the arguments
    each as parameter node id and declared domain for an operation parameter, as E-1
    states (the domain FR-015-AC-39 defines). A state field read is named by its declaring
    node id and field name, never by a node of its own; that E-1 lists parameters
    only is an open question for the AD-003 owner and E-1 is not widened here. The
    source span is excluded; changing any included member changes it, and changing
    the span, the unwind bound or the subject d
- id: FR-015-AC-49
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters
    within their declared domains (FR-015-AC-39) and asserts the invariant clause;
    its subject is the clause itself, with no subject call. Preservation of the invariant
    under an operation is not specified by this criterion. PLANNED (IR-489).
- id: FR-015-AC-50
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded`
    or `LoweringLimitUnrecognised` as `Outcome::Unsupported(UnsupportedObligation::OracleRefused
    { refusal })` with `refusal` equal to the scalar refusal, field for field, and
    neither is mapped to another `UnsupportedObligation` variant nor to `LoweringWorkExhausted`.'
- id: FR-015-AC-51
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a finite input validated under a profile selection
    other than the offered profile's is refused as a typed `InvalidInput` `kani_profile_input_mismatch`
    result naming the request's source id, with no artifact and no case identity recorded.
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, the context of every outcome and refusal that
    path returns (generated-case `Proved` or `Counterexample`, `kani_corpus_dependency_invalid`,
    `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`)
    is the profile selection's revision. The generated-case values are not Kani run
    verdicts.
- id: FR-015-AC-53
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Each of the seven harness kinds the generator emits (precondition, V1
    contract, scalar, state-clause, frame-effect, V1 bundle and corpus) is subject
    to the cover-last rule of FR-015-AC-7: exactly one `kani::cover!`, the last statement
    of the body. What the cover witnesses, by kind: a precondition harness, that the
    precondition holds within the IR bounds (it asserts nothing); a V1 contract harness
    (postcondition or invariant), that the requires clause and the IR bounds are jointly
    satisfiable, after the contract call whose `ensures` is the harness''s check;
    a scalar harness, that the oracle''s `C'
- id: FR-015-AC-54
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract` harness
    it emits for a requires clause, an ensures clause and the bundle's bounded arguments)
    ends with exactly one `kani::cover!` after the call of its contract, which witnesses
    that the requires clause and the argument and result bounds are jointly satisfiable,
    so a run whose requires clause no bounded argument satisfies never reaches it;
    the bundle's `ensures` is checked at the contract call, so the cover follows every
    check the harness carries.
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic,
    FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness
    of that family ends with exactly one `kani::cover!` after its `assert!` of the
    case's oracle, which witnesses that the harness runs to its end past that assertion.
- id: FR-015-AC-56
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a V1 bundle harness whose requires clause some
    bounded argument satisfies and whose `ensures` holds for every such argument classifies
    `Verified` (FR-017-AC-4); the same bundle with a requires clause no bounded argument
    satisfies classifies `CoverUnsatisfied` with its satisfied and total cover counts,
    never `Verified` and never `Falsified`.
- id: FR-015-AC-57
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed
    backend, a bounded-corpus harness of each of the arithmetic, graph and collection
    families whose oracle is true classifies `Verified`, and a graph or collection
    harness whose oracle is false (an arithmetic case's oracle is always true) classifies
    `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes
    the corpus replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`),
    never `Inconclusive` for lack of a counterexample.
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test generates a harness through every emitting entry point (the precondition,
    V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bundle`,
    and the corpus generator for each of its families whose arm remains, per FR-015-AC-109
    to FR-015-AC-111), parses each emitted source, and fails for any function attributed
    `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain
    exactly one `kani::cover!`, which must be the last statement, with no assertion
    after it and no other cover; and a scan of the non-test string literals of `src/`
    fails when a file othe
- id: FR-015-AC-59
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request of `StateFrame` items in which the first item is refused and
    later items are supported, requires-bound, unsupported or invalid yields exactly
    one `ObligationRecord` per item in request order, `kind` `postcondition` for a
    `contract` item and `frame` for a `frame` item; and each later item's record (disposition
    and reason) equals the record the same item has in a request holding that item
    only; a request of three refused items yields three records. The requires-bound
    item comes from a lowering `RequiresBound` record in that same admitted package,
    not an object-body member target; remov
- id: FR-015-AC-60
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` item that yields is recorded `supported` carrying its harness
    symbol, and its harness, returned in the `state_frame_harnesses` field of `KaniObligationOutcome::Emitted`
    in request order, is byte-identical to the harness `generate_state_frame_role`
    returns for the same clause and role, and for a clause both of whose roles succeed
    to the corresponding harness `generate_state_frame_obligations` returns. COVERED
    (IR-461): a crate-internal test uses an admitted V2 package to compare direct
    role generation with each negotiated supported item, its symbol and exact harness
    bytes, and th'
- id: FR-015-AC-61
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is a requires-bound record is recorded
    `requires_bound` with that record's `unbounded_type` and no harness. A selected
    model declaration's clause read of a native `Integer` that makes lowering return
    `RequiresBound` keeps that lowering refusal; it is not remapped to value-based
    `ModelMemberNotI64Range`. The latter applies to a present listed `Integer` field
    only if range resolution is reached, such as an unread field. A non-model or admitted
    unselected model/object_type body is refused as `ModelFieldsUnavailable`, without
    inventing an `unbounded_type` node or
- id: FR-015-AC-62
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose lowering is an unsupported-family record is recorded
    `unsupported` with reason `NoFiniteEncoding` naming the record's node and family,
    and an item whose node is not a `state_clause` is recorded `unsupported` with
    `UnknownNodeKind`, each with no harness.
- id: FR-015-AC-63
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item whose condition is a negation, compares to a literal,
    uses an operator other than the six integer comparisons, reads through a parameter
    other than `self`, compares two fields or two reads of one side, or whose frame
    creates or deletes an object or grants a relationship or foreign field (naming
    the effect), is recorded `unsupported` with `StateFrameRefused` and no harness.
    The same result holds for a clause that is not a postcondition, is malformed,
    whose frame grants every field, whose lowering is an invalid-body, incomplete
    or over-budget record, or whose object is non-mo
- id: FR-015-AC-64
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A `StateFrame` item with an unparsable state path or an unparsable item
    subject path, an invalid or lacking state field, an absent node, a repeat of an
    earlier item or a package other than the first item's is recorded `invalid_request`
    (`InvalidStatePath`, `InvalidStateField`, `UnknownNode`, `DuplicateItem`, `MixedStatePackages`),
    and the outcome is `Rejected` with every item's record and no harness bytes.
- id: FR-015-AC-65
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`,
    an unparsable request subject path or an unwind bound outside `1..=MAX_OBLIGATION_UNWIND`
    is refused whole with the existing `KaniObligationError` variant and no record.
- id: FR-015-AC-66
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test builds every retained `StateFrameRefusal` variant (including `ModelFieldsUnavailable`,
    `MemberAbsent`, `ModelMemberNotI64Range` with each cause, and each of the six
    refusal arms of `NotLowered`) and calls the mapping on it; each yields the disposition
    and reason of the table. It does not construct retired body-member `BoundNotResolvedCause`
    values. The variants a request reaches only with a defective generator or a source
    over 1 MiB (`UnwindOutOfRange`, `InvalidGeneratedSyntax`, `RecordSerialization`)
    are built directly, and `ResourceLimitExceeded` is built directly or reached with
    a su
- id: FR-015-AC-67
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`generate_state_frame_obligations` refuses with `MalformedClause` at the
    node, not `InvalidField` or `ConditionNotSupported`, for a clause whose graph
    field name is not a Rust identifier; a field name the caller supplies that is
    not an identifier is still `InvalidField`.'
- id: FR-015-AC-68
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A `StateFrame` request whose frame grants every state field records its
    `frame` item `unsupported` (`StateFrameRefused`) and its `contract` item `supported`
    with the harness `generate_state_frame_role` returns for the `contract` role;
    a request whose condition is a negation records its `contract` item `unsupported`
    and its `frame` item `supported` with the harness `generate_state_frame_role`
    returns for the `frame` role; in both, `generate_state_frame_obligations` returns
    the refusal of the failing role; a request whose lowering is refused records both
    items alike. COVERED (IR-461): a crate-in'
- id: FR-015-AC-69
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A composite-equality claim over operands that are parameters or literals
    of one record, tuple or option type whose leaves are Boolean or bounded integers,
    with the descriptor FR-018 generated, yields one harness whose identity records
    family `composite_equality`, proof subject `bounded_shadow` and a refinement obligation
    in the same result (FR-028-AC-7), separate from the harness of every other item;
    both operators (`Equal`, `NotEqual`) yield one; and the operator and operand types
    are read from the descriptor while no bound is. PLANNED (IR-264).
- id: FR-015-AC-70
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The harness draws one nondeterministic value per leaf of each parameter
    operand and no other value: a `bool` per Boolean leaf, an `i64` per bounded-integer
    leaf assumed inside its `integer_range` (FR-015-AC-11), a presence `bool` per
    option and a payload only for a present option, and for a record field of optional
    presence one of `present`, `absent` or `null`; a literal operand''s leaves are
    constrained to exactly its value (FR-015-AC-16); the drawn arguments ascend by
    parameter node id then by leaf path, each bound to its node id and path (FR-025-AC-1,
    FR-025-AC-2), and no composite is itself'
- id: FR-015-AC-71
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Over every drawn pair the harness asserts that the shadow's verdict equals
    the expectation's and that the shadow's pair count equals the expectation's, where
    the expectation is computed from the drawn values by QSpec FR-149's rules (same
    declaration and pairwise-equal fields or positions in order; two `none` equal;
    `none` and present unequal; `absent` equals `absent`, `null` equals `null`, `absent`
    and `null` unequal; the pair count is the node count of the occurrence-pair tree
    with no early exit after an unequal pair), the verdict of `NotEqual` is the negation
    of `Equal`'s with the same count
- id: FR-015-AC-72
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The emitted shadow and harness, parsed, contain no loop (`while`, `loop`,
    `for`), no recursive call, no `Box`, `Vec`, `Rc`, `String` or map, and no integer
    type wider than `i64`; and a closure whose pair-node count (the node count of
    the largest occurrence-pair tree its type admits) exceeds `SHADOW_PAIR_NODE_BUDGET`
    is refused as `ShadowShapeOverBudget` naming the count and the budget, one at
    the budget generates, and the budget and the count are recorded in the identity
    and the evidence. The value of the budget is set by the measurement of the code
    change and recorded in TC-025's status. PLAN
- id: FR-015-AC-73
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Each row of the unsupported-shape table above is produced by an item reaching
    that shape and yields its disposition and reason with no harness: an unbounded
    integer leaf `requires_bound`; a bound outside `i64` `domain_not_representable_in_i64`;
    a recursive declaration `requires_bound` naming the declaration; one leaf of each
    family the table lists `ShadowFamilyNotBuilt` naming that family; a `reference`
    composite, as FR-018-AC-7; an operand that is neither a parameter nor a literal,
    and a `convert<T>` operand, `OperandNotSupported`; an over-budget closure `ShadowShapeOverBudget`;
    an item FR-01'
- id: FR-015-AC-74
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the harness of the corpus case (two record-typed
    and two tuple-typed values compared with `=` and `!=`, a record with an optional
    field and a nested option, Boolean and bounded-integer leaves) is classified `Verified`
    with its cover satisfied within the memory and wall-clock ceilings its identity
    records, and its execution evidence records the versions and options of FR-028-AC-23;
    the refinement run of the same case agrees on every case it runs, and the case's
    refinement domain is at most the case cap, so the run is classified `exhaustive`
    and records its case count
- id: FR-015-AC-75
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'With the installed backend each seeded mutant of the shadow is `Falsified`
    with a concrete playback: an `absent` slot read equal to a `null` slot; one field
    of the left operand compared with the next field of the right; the pair count
    omitting the payload pair of two present options; the count stopping after the
    first unequal field; the operator `NotEqual` rendered as `Equal`; and a field
    left out of the comparison. Run natively, each seeded mutant of the production
    oracle settles `refinement_failed` naming the first disagreeing case: the opposite
    operator emitted; an `absent` slot compared eq'
- id: FR-015-AC-76
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The identity of a composite-equality harness records family `composite_equality`,
    proof subject `bounded_shadow`, the closure's declaration keys each with its leaves'
    declared domains, the operator, the size budget and the closure's pair-node count,
    the abstractions `representation`, `integer_widening`, `absent_payload` and `unrolled_walk`
    each with its discharging evidence, and the behaviours the proof does not exercise
    (`meter_limits`, `ill_typed_operands`, `charge_amounts_other_than_pairs`, `foreign_reference`,
    `declaration_reconstruction`) each with the native criterion that covers it (FR-
- id: FR-015-AC-77
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For the QSL-emitted twin package, `field_range` calls `model_object_fields`
    on the framed object''s selected model declaration and obtains both `balance`
    and unread `audit` with `IntRange { lower: 0, upper: 1000 }`; it converts their
    `i128` endpoints to `i64` and returns 0 to 1000 for each. A second admitted unit
    with the same declaration but no read of `audit` returns the same range for `audit`;
    IR FR-038-AC-140 independently shows a read''s range-node body does not change
    the accessor''s bound. The genera'
- id: FR-015-AC-78
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG implementation pending. `field_range`
    returns `MemberAbsent` for a requested name absent from the selected model declaration's
    accessor, including a clause's own field; an accessor `UnknownNode`, `NotModelObjectType`
    or `AmbiguousField(name)` returns `ModelFieldsUnavailable` with that error and
    object id, with no body or read fallback. A non-model object and an admitted unselected
    model/object_type with a nonempty body both yield `NotModelObjectType` and refuse
    even if body members claim valid ranges. A selected model/object_type whose nonempty
    body
- id: FR-015-AC-79
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. From the package QSL emits for the twin's unit, both roles of each clause
    (`BalanceNeverDrops`, `AuditNeverDrops`) are generated; each identity's `state_fields`
    equals the request's list in its order, its `domains` equal the accessor ranges
    of FR-015-AC-77, and its `scope.anchor` and `scope.frame` equal the ids `qsl_replay::call_site`
    names, asserted before replay. The accessor's ascending-name order does not reorder
    `state_fields`. A request naming a field absent from the accessor is refused;
    one omitti
- id: FR-015-AC-80
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31
    run over harnesses generated from the package QSL emits: the healthy subject verifies,
    the subject mutated to debit is falsified naming the postcondition, a granted
    write verifies and a write to an ungranted field is falsified naming that field.'
- id: FR-015-AC-81
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A present listed model field with `member_type()` of `None`, a non-`IntRange`
    variant, or `IntRange` outside `i64` is recorded as unranged with `TypeNotRange`
    in `StateFrameIdentity` and its persisted record, even when no read names it;
    a present unread `IntRange` within `i64` is recorded in `domains` and is absent
    from the unranged list. Equal inputs produce byte-identical records; a record
    naming a field twice is invalid. The model declaration path records no `NoRead`
    reason. QSL-emission gaps for `Non
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for
    distinct named constructs returns one `CapabilityEntry` per requested construct
    in request order, preserving each admitted `Supported` module or `Refused` cause
    code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied
    code becomes an `Inconclusive` run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate
    or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition
    census.
- id: FR-015-AC-84
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder
    returns the exact checked `i128` result only inside the request's inclusive result
    range.
- id: FR-015-AC-85
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted result range is `InvalidInput`; zero division, checked overflow
    and a result outside the range are typed refusals, with no arithmetic result or
    generated artifact.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a profile/dispatch-admitted request follows only the selected reference field,
    finds a positive-length path in sorted depth-first order, and checks the expansion
    bound before entering each new object; a target edge is found without expanding
    its target.
- id: FR-015-AC-87
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the
    bound is `ResourceExhausted`, with no graph result or generated artifact.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query
    retains input order and duplicates and returns its truth value and examined-item
    count; a decisive item ends that count, otherwise every item counts.
- id: FR-015-AC-89
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, an input longer than `max_items` is `ResourceExhausted`, with no query
    result or generated artifact.
- id: FR-015-AC-90
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic,
    graph or collection case emits all four artifacts (oracle, finite strategy, Kani
    harness and proof-dependency graph) from the same selected profile, validated
    finite input and family lowering.
- id: FR-015-AC-91
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, that path returns a generation-time case classification
    of `Proved` when the family lowering's Boolean oracle is true and `Counterexample`
    when false; neither value is an observed Kani run or native replay verdict. This
    interim classification ends when FR-015-AC-111 deletes the path.
- id: FR-015-AC-92
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), a declared dependency
    census with an empty or repeated identity, inconsistent kind/state/path or a kind
    other than `Required` refuses as typed `InvalidInput` with no artifact or case-identity
    claim; a valid census is normalized into the emitted proof graph.
- id: FR-015-AC-93
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corpus path, a profile/input mismatch, family-lowering
    refusal, proof-graph serialization refusal or duplicate case identity returns
    its typed refusal with the selected profile revision, emits no artifact and claims
    no new case identity.
- id: FR-015-AC-94
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce
    equal artifact paths and contents regardless of emission order, and reordering
    the same finite graph's objects or references does not change them; changing a
    request field, selected profile, finite input, ceilings or normalized dependency
    census changes the case identity and artifact paths.
- id: FR-015-AC-95
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111
    (graph, which deletes the whole public corpus API and the generation-time `KaniOutcome`);
    this criterion is the end state they reach together and adds no deletion of its
    own. The end state is: QSL-emitted packages admitted through Contract IR feed
    the one V2 generator for arithmetic, collection and graph cases; the collection
    and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived
    symbolic bounds, and each'
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall`
    or `quire.op.collection.exists` operation over a selected model collection field
    is a collection claim. For a field with typed inclusive cardinality `[minimum,
    maximum]` and supported finite elements, the generator emits a harness with symbolic
    length and symbolic elements. Its assumptions admit both cardinality endpoints
    and every length between them; for `IntRange` elements they admit both inclusive
    integer endpoints and every integer between them, and for `Boolean` elements they
    admit both `false` and `true
- id: FR-015-AC-97
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted collection claim whose selected field has multiplicity
    lower bound `0` and no finite upper bound, QSL admits an unbounded collection
    type; when its element type is supported, this generator settles `requires_bound`
    naming the selected field or unbounded collection type and emits no harness. An
    admitted field with no finite element domain likewise settles `requires_bound`
    naming that element type and emits no harness. A field with multiplicity lower
    bound greater than `0` and no finite upper bound has no QSL kernel type ([QSL
    FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-05
- id: FR-015-AC-98
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim (one reachability application of either form above, at its
    `state_clause` node id and argument path) generates a harness when the admitted
    package gives its covering object type `E` a finite population `[0, N]` through
    `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field`
    the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches`
    the destination end's finite multiplicity). Every symbolic object and edge is
    bounded inclusively by those declarations, including zero and maximum where admitted.
    Reacha
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The collection harness asserts the admitted clause/function body over symbolic
    inputs drawn from the selected model field's typed domain, ends with the FR-015-AC-7
    cover after its assertions, and never replaces those inputs with the finite-input
    corpus's concrete `Vec<i128>`. The independent native oracle replays the same
    authored function through `qsl_replay::replay`; it does not substitute a separately
    authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani
    `Verified` with its cover satisfied; a seeded claim or subject mutant yields `Falsified`
    with a concrete in-dom
- id: FR-015-AC-100
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For the collection family, changing the selected field's authored cardinality
    or element bound, admitted body claim, customer subject or unwind changes the
    harness identity; equal admitted inputs regenerate byte-identically. The identity
    records every symbolic collection domain and the solver and complete ordered option
    vector, and FR-017 run evidence records the actual Kani, Rust and solver versions,
    unwind and every option used by the installed backend. For the graph family (AC-98),
    changing the claim's clause id or argument path, the reachability form, relationship
    or field, its role or nam
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A request whose symbolic collection domain exceeds the generator''s representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing or malformed selected declaration likewise refuses
    with its typed reason and no harness. For a graph claim, a population or edge
    domain whose symbolic object or edge count exceeds the ceiling likewise settles
    `unsupported` naming the domain, with no harness and no reduced population or
    multiplicity. PARTIAL (IR-268, CG #396 fe2'
- id: FR-015-AC-102
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'An admitted collection body claim is settled through the FR-015 V2 obligation
    path, one disposition per requested item; supported items alone emit harnesses.
    The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply
    the symbolic bounds nor classify the proof verdict. Graph items take the same
    V2 path (IR-805), one disposition per requested item, and are not implementation
    or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG
    #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_negotiation_accounts_for_each_supported_or_refus'
- id: FR-015-AC-103
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0,
    20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x:
    Int[0, 9]` into `Int[-10, -1]` each settle `Supported` with an IR-confirmed outer
    oracle and a harness. Their respective argument ranges are `[1, 10]` and `[1,
    1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the
    outer declared result bounds. Each inferred range carries its source application
    id in the harness identity.'
- id: FR-015-AC-104
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A plain-Integer parameter as any nested leaf keeps `requires_bound` with
    the Integer `unbounded_type` and no harness; a direct reference to a raw application
    never borrows a separate narrowing conversion's domain, and an inferred endpoint
    outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming
    both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch`
    and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact
    scalar refusal, including its node, position or limit fields, and no harness.
    Refused items leave a suppor
- id: FR-015-AC-105
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and symbolic objects within it; for every object, the
    selected edge as its shape states: a `Reference` field has exactly one target,
    an `Option` field an absent or one target, a `Collection` field or finite multi-valued
    relationship end a count in `[lower, upper]` (a unique end admits no repeated
    target, a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at
    least `lower` targets) and every target is an object of the population. Start
    and target are drawn from that population and may b'
- id: FR-015-AC-106
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'The independent native oracle replays the same authored clause over each
    original-domain concrete graph through QSL''s replay (`qsl_replay`), and does
    not substitute a separately authored reachability. With the installed backend,
    a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied
    for one field-edge claim and one relationship-end claim over a `{0,3}` unique
    and a `{0,3}` non-unique end. Each semantic mutant yields `Falsified` at the clause
    assertion with an original-domain playback and native replay of that same authored
    clause: reachability that allows a zero-len'
- id: FR-015-AC-107
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice in one condition yields two items at positions
    0 and 1; one node under two clauses yields two items with different identities;
    a field-edge and a relationship-end reachability yield separate items; a collection
    claim, a clause with no reachability and a path that does not name a reachability
    application each settle `unsupported` naming the missing claim with no harness.
    A mixed request settles one disposition per item and a supported item''s harness
    is unchanged by a refused sibling. PLANNED (IR'
- id: FR-015-AC-108
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction
    or caller `max_expansions` substitute. While the population selection for the
    claim is `Unavailable` (no available population selection for this admitted claim),
    the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`,
    that is a claim whose population has no finite authored bound (once a selection
    is available), settles `requires_bound` naming the population; an unbounded `Collection`
    edg
- id: FR-015-AC-109
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004
    step 4b). After this slice no source under `src/` contains the corpus `Arithmetic`
    request variant and arm, `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`,
    `checked_method`, `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and
    their re-exports, and no test in `tests/` drives them. The slice deletes only
    what the arithmetic family owns: the corpus replay (`BoundedCorpusCase::replay_counterexample`,
    `compile_corpus_qsl_package`, `BoundedCorpusReplayError` and the replay-harness
    identit'
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because the collection family renders through `HarnessSpec`
    and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend
    controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate`
    run on the slice PR's head that is green
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims
    need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package
    population emission (after QSL-819)). Graph claims are served by FR-015-AC-98
    and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay
    is the replacement for the corpus graph replay; Evidence for a real-Kani precondition
    is a recorded `make kani` or `make kani-gate` run on the slice PR's head that
    is green, cited in the PR (the real-Kani
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness:
    a spec with no cover or more than one cover; a cover whose condition is the literal
    `true`; no `Call` or more than one; an `Input` or `Assume` after the `Call`; an
    `Assert` before it; a binding name bound twice (other than an input''s own rebinding);
    a name, `result` or `callee` that is not a valid Rust identifier; an input domain
    with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`;
    a mess'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). For every spec that constructs, `render` emits the
    header, the oracle source, the module wrapper and the proof attribute, then the
    spec''s statements in spec order, then its one cover as the last statement of
    the function: after every assumption, the subject call and every assertion, whatever
    order a family listed them in. A hand-built spec in each of several statement
    orders renders its cover last; a cover is not a field of `body` and no statement
    order moves it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7).'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). `render` of one spec returns equal bytes on repeated
    calls and in a second process, and a spec whose `Input` list is reordered renders
    the reordered source and not the same bytes (AD-004 "The HarnessSpec and render
    contract (step 4b)", R-1).
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). After the scalar port, for each claim of a fixed
    set (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a
    literal operand; a claim with an inferred operand; a claim with each operand at
    an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path
    and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`,
    and the same digests derived from them, as a test-only reference copy of the pre-port
    `render_scalar` computes in the same test. '
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). `render` holds no family-specific template text:
    for hand-built specs whose label, expressions and messages are not the scalar
    family''s, the rendered source consists of the fixed header, the spec''s `oracle_source`,
    the module wrapper, the proof attribute, the spec''s statements rendered from
    their typed fields, and the one cover, and contains no other text; and the ported
    scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives
    `render.rs` for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1
    item 3).'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port,
    the unmutated scalar harness classifies `Verified` with its cover satisfied and
    the existing scalar mutation control (the oracle's `Add` swapped for `Subtract`,
    FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable
    `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run is
    a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3).
- id: FR-015-AC-118
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: For an admitted V2 scalar `implies` claim over two Boolean operands, the
    generated harness embeds its FR-014 oracle as FR-015-AC-20 requires and draws
    both operands symbolically. It calls `quire_contract_runtime::operators::implies_short_circuit`
    with `left` and a closure returning `right` as the subject expression, asserts
    that subject result equals an independent native `!left || right` expectation
    over the same operands, and separately asserts that the oracle's `Outcome<bool>`
    is `Completed` with that expectation; neither subject nor oracle computes the
    native expectation. With installed Ka
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source
    authors an operation with one parameter `amount: Int[0, 1000]` and its V2 postcondition
    `amount < 1000 implies amount + 1 <= 1000`; both occurrences of `amount` read
    that operation parameter, not a state field or subject-produced post-state value.
    The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle
    and an FR-015 postcondition harness for that same clause, not a scalar implication
    harness. Under FR-015-AC-39 the harness draws the parameter once within its declared
    finite domain and passes that same'
- id: FR-015-CON-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Clause ABI construction SHALL check both endpoints of a bounded integer
    dependency before conversion to i64 and refuse one outside i64 as `UnsupportedDependency`
    in the clause-lowering detail, without producing a binding.
- id: FR-015-CON-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Subject binding SHALL check both endpoints of a bounded integer dependency
    before conversion to i64 and refuse one outside i64 as `UnsupportedBinding`, without
    producing a binding or harness.
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before
    anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report
    states success, and every cover property is reported satisfied; a successful run
    with an unsatisfied cover is cover-unsatisfied with its satisfied and total counts;
    a report of success from an unsuccessfully exited process, a report with no cover
    property and a report with no successful check are each inconclusive with their
    own reason; console banners never decide the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying
    that playback verbatim and never the playback of a satisfied cover; a failure
    with no playback is inconclusive for that reason; a failed unwinding assertion
    is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding
    check in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the
    harness path, the launcher path, the complete argument vector, the unwind bound,
    the solver, the exit code and the outcome.
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for
    byte is refused, and no backend runs.
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`)
    runs through `execute_kani_obligation` and `kani_launch_command` the same way
    an FR-015 contract harness does: a crate whose library source lacks its generated
    source byte for byte is `HarnessNotInCrate`, its covers classify a run identically
    (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed
    is inconclusive), and its evidence carries `None` for obligation kind, since an
    exact-scalar claim carries no contract role.'
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Real Kani captures of a verified run, a falsified run with a playback,
    an exhausted unwind bound, a run whose only check is unreachable, a partly satisfied
    cover and a run with no cover each parse into the expected typed report of harness
    status, successful checks, cover counts and failed checks, and classify to the
    expected outcome; the falsifying playback block passes through verbatim.
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run whose process exited successfully and whose backend reported success
    with zero successful checks is inconclusive with the vacuous-proof reason, never
    verified, except that a precondition harness, which asserts nothing beyond its
    cover, is decided by its cover summary.
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher whose stdout or stderr carries more than 8 MiB times its harness
    count is stopped, its process group is killed, and the run is refused with `kani_output_over_limit`
    naming the stream, the limit and the count, with no outcome and no retained text;
    a launcher whose stream is exactly the limit completes with its real exit status
    and the whole stream retained. No stream is ever truncated and then classified.
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A timeout too large to add to the current instant never elapses and does
    not panic.
- id: FR-017-AC-16
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The launcher''s capture threads are stopped and joined on every outcome:
    they return what the launcher wrote before it ended, stop while a write end is
    still held open, and stop within their drain limit while a straggler keeps writing,
    so a process holding a pipe open does not delay the return.'
- id: FR-017-AC-17
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run that times out has the whole group the launcher leads killed, a real
    grandchild included.
- id: FR-017-AC-18
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A report that is malformed, of an unknown check or harness status, of another
    schema version, that holds other than one harness result in a single run, or whose
    harness states success while it lists a failed, errored, undetermined or unknown
    check (of any class, including an unwinding assertion) is refused with its own
    typed cause and never classified, for every obligation kind; a run that exited
    successfully and exported no report is refused, and one that exited unsuccessfully
    and exported none is `NoVerdict`.
- id: FR-017-AC-19
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator appends `-Z unstable-options --export-json /proc/self/fd/N`
    after unchanged harness options, with actual child-only anonymous pipe writer
    N >= 5, and records the actual argv bytes. It uses per-run unnamed report authority,
    never a named target-directory report or fallback. FR-034 AC-32/33 own collector
    bounds, resource-stop classification, kernel lifetime and immutable final handoff.
    Zero-byte EOF means no exported report; nonempty partial/malformed content is
    refused under AC-18 unless an independently established resource/deadline stop
    takes precedence with the existing single-
- id: FR-017-AC-20
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The evidence and the classified run list every check of a real run with
    its id, class, source file and line and status, a line stated as unknown is absent,
    and a non-numeric line or a check with no location is a refused report; the view
    serializes as `id`, `class`, `location { file, line }` and `status` and is not
    deserializable.
- id: FR-017-AC-21
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: N harnesses with equal option vectors (the harness selection removed) and
    equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly
    one launcher process whose argument vector holds one `--harness <module::harness>
    --exact` pair per member in request order, `--harness-timeout` T, then the shared
    options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses
    start fewer than N processes; one harness starts one process with the single-run
    argument vector. Requests whose wall-clock or memory ceilings differ start separate
    processes. The group's proc
- id: FR-017-AC-22
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over a real or captured batch report, members are matched by `module::harness`
    path (two members with the bare symbol `check` stay distinct), and each member''s
    outcome, checks and SUCCESS count come from its own entry: a falsified member
    beside a verified member leaves the verified member verified; a Success member
    in a non-zero exit with no Failure entry is inconclusive, and with one Failure
    entry is verified; a Failure entry with no checks and exit status `timeout` is
    inconclusive timed-out naming T while the others keep their results; a Failure
    entry with no checks and no timeout is inconcl'
- id: FR-017-AC-23
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A group's report that lacks a requested harness, holds one twice or holds
    an unrequested one refuses the group with a typed cause and classifies no member;
    a member whose source is not in the crate refuses the whole batch with `HarnessNotInCrate`
    and launches nothing; a falsified member's playback is the first counterexample
    block headed for its own path and never another member's block, a member that
    fails two property checks (two counterexample blocks under its path, as real Kani
    prints) takes the first, as the same harness alone does, and neither it nor its
    neighbours lose their evidence, a
- id: FR-017-AC-24
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher that exits on its own after leaving a real grandchild in its
    process group has that grandchild killed by the time the run returns, the same
    as on a timeout.
- id: FR-017-AC-25
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A capture thread that panics, or whose poll or read of the stream errs,
    refuses the run with `kani_output_unread`; the run is never classified from empty
    text.
- id: FR-017-AC-26
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A generated FR-035 caller Text-admission harness enters the same production
    execution path in single and compatible batch runs; its evidence carries the exact
    typed caller identity (profile, bounds and finite payload class) supplied with
    that harness and `None` for contract obligation kind. A matching successful report
    with a satisfied cover and successful agreement assertion classifies `Verified`;
    an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement
    assertion with its own concrete playback classifies `Falsified` with that playback.
    A source-mismatched harness, mi
- id: FR-017-AC-27
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A genuine routed `x + 1` scalar harness from `generate_routed`, its complete
    emitted oracle package and one in-range concrete operand are passed to the paired
    runner. One real Kani run reports `Verified`, and the runner invokes the public
    oracle from the same staged library source at that operand with the request's
    exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed
    runtime result of that invocation. A Kani verdict alone or a result reconstructed
    by the caller is insufficient.
- id: FR-017-AC-28
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Before either child starts, a package or harness whose path or bytes differ
    from the retained routed output, a selected oracle symbol or argument order that
    differs from the retained scalar identity, or an operand count/value outside the
    retained ordered ranges is refused with a typed cause and no receipt. For an accepted
    request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`,
    used to initialize the native meter, and returned in the receipt; no hidden or
    substituted limit value is permitted. CG attests these supplied, used and returned
    values are equal, bu
- id: FR-017-AC-29
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The runner measures one canonical proof-content identity over the exact
    paths and bytes of every staged execution input: the complete generated package,
    retained harness, canonical native invocation adapter, observer manifest and both
    independently generated dependency locks. This identity binds the content actually
    proved and executed; it is not a tool-version or file-version tracking identity,
    and no separate identity is added for any staged item. The Kani and native invocations
    cannot write the staged source tree. A changed, replaced or additional staged
    file before or after either invocati'
- id: FR-017-AC-30
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The public receipt exposes the measured identity, scalar identity, ordered
    operands, exact supplied and meter-used `ScalarLimits`, verified Kani evidence
    and actual typed native outcome through read-only access. Callers cannot construct,
    deserialize or mutate a receipt, set its identity or outcome, or turn any non-verified,
    refused, inconclusive, faulted or malformed run into a receipt.
- id: FR-017-AC-31
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The paired runner executes an order-sensitive routed scalar operation with
    two symbolic operands in two concrete operand orders. Each real Kani run is verified;
    the native invocation returns the result for that order; and each receipt binds
    its exact ordered operand vector and typed native result. Swapping the operands
    changes the observed result and receipt context while retaining the same generated
    proof-content identity.
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'In the `make kani` lane, a crate whose library source holds the generated
    harness byte for byte and, beside it, an item that does not compile (a reference
    to an undefined name) is run through `execute_kani_obligation`: the run exits
    unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit
    code, and its evidence carries a diagnostic record whose phase is `compile`, whose
    stdout excerpt holds the rustc error code and Kani''s `error: Failed to execute
    cargo` line and whose stderr excerpt holds Cargo''s `error: could not compile`
    line with the crate path as `<CRATE_DIR>`. A '
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside
    the unsuccessful exit is `unknown` even when its stdout holds `Solving with CaDiCaL
    3.0.0`; with no report, stdout holding the Kani banner, `Checking harness proofs::check...`
    and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and the `Checking harness`
    line alone is `backend`; the banner and `error: Failed to execute cargo (exit
    status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error:
    could not compile` alone with the banner; a usage error on stderr and nothing
    on stdout is `launcher`; the ba'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes`
    0; one of 8193 bytes whose final line starts within the window keeps the longest
    line-aligned suffix of at most 8192 bytes with `omittedBytes` the byte count before
    it; a stream whose final line alone exceeds 8192 bytes keeps at most 8192 bytes
    ending at its last byte, starting on a character boundary; a stream that is one
    10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`,
    never an empty excerpt; a stream of a 10000-byte line, `\n`, `error: last\n` keeps
    exactly `error: last\n`; a 4-byte ch'
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target`
    and home directory `/h/u` supplied in the request, a stand-in whose streams print
    `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`, `/h/u/ws/crate/target/debug/x.out`,
    `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`,
    `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`,
    `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242`
    and `thread ''main'' (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`,
    `<TARGET_'
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The diagnostic record never feeds classification: two runs with the same
    report and exit and different stream text classify to the same outcome, checks,
    SUCCESS count and playback, and two `NoVerdict` runs with equal exit and no report
    whose streams differ have equal evidence in every field but `diagnostics`, `memory`
    (the observed peak resident bytes vary run to run) and the run''s report-path
    argument (the last argument, unique to each run); the FR-029 terminal value of
    a `NoVerdict` evidence carrying a record is `Failed`, the value FR-029-AC-4 gives
    the no-verdict reason, equal to the termin'
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness
    proofs::check...` and `CBMC failed with status 64` on stdout and `Unknown option:
    --no-such-option` on stderr, and export a report whose one result `proofs::check`
    is `Failure` with no checks and whose one `error_details` entry for `proofs::check`
    has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics`
    record of phase `unknown` whose stderr excerpt holds `Unknown option: --no-such-option`;
    `exit_code_15` has `cod'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing
    a failed assertion check with no playback on the console, whose entry for the
    harness has exit status `properties_failed`, is `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics` member.
    The same report with no `error_details` member, with an entry that names another
    harness only, with two entries naming the harness, with an entry whose `exit_status`
    is null, or whose one result names no `harness_id`, is `FailedWithoutCounterexample`
    with neither a `failureCause` nor a '
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check`
    and `e::check` whose process exits 1 and whose report has `a::check` `Success`
    with a passing assertion, every cover satisfied and no exit status, `b::check`
    `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure`
    with no checks and exit status `timeout`, `d::check` `Failure` listing a failed
    assertion with no playback block headed for its path and exit status `timeout`,
    and `e::check` `Failure` listing a failed assertion with no playback block headed
    for its path and exit status `properties_failed
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with
    the same `Failure` report, no checks and the same exit, whose entries give `exit_code_64`,
    `out_of_memory` and `cbmc_exploded` and whose streams differ, classify to the
    same outcome, checks, SUCCESS count and playback, and their evidence is equal
    in every field but `failureCause`, `diagnostics`, `memory` and the report-path
    argument; the cause is the only input of the FR-029 terminal value that differs:
    `exit_code_64` and `cbmc_exploded` give `Failed`, equal to the terminal value
    of the same evidence with `failureCause` and '
- id: FR-017-AC-41
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Every `KaniScalarNativeError` variant has a `code()` that is a total `&''static
    str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`,
    `kani_execution`, `proof_not_verified`, `oracle_stop`, `native_launch`, `dependency_lock`,
    `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`. The set
    is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]`
    (a rustdoc `compile_fail` exhaustive match without a wildcard arm, beside a compiling
    control with one), and `class()` returns a closed `KaniScalarNativeErrorClass`
    (a compiling exhaustive '
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`
    and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`,
    `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and
    `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence
    outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`,
    `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no
    `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`,
    `Inconclusive` `TimedOut`, `MemoryExhausted` or'
- id: FR-017-AC-43
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode`
    and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult`
    is produced only for the helper's malformed, missing or unsupported structured
    result, and a forced identity-encode failure through an in-crate test seam returns
    `IdentityEncode`, not `NativeResult`.
- id: FR-017-AC-44
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The code table and the mapping to class are frozen: a test lists every
    variant with its expected code and its mapping to class (one class for a fixed
    variant; for `proof_not_verified` the class of each `KaniRunOutcome`, `KaniInconclusiveReason`
    and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal),
    fails if a variant, outcome, reason, cause or nested refusal exists that the list
    lacks or if a code value or a mapped class differs, and fails if two variants
    share a code. Adding a variant, code, class or mapping entry is a spec change
    to the "Scalar native error dis'
- id: FR-017-AC-45
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The receipt exposes its proof-content identity as a `DigestRecord`, the
    `qsl_replay::DigestRecord` re-exported at the crate root (no CG-defined twin type;
    a compile test names it by the CG path and by the `qsl_replay` path as the same
    type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`.
    An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema":
    "agent-ix/cg-kani-scalar-proof-content/v3", "artifacts": [...]}` over the entries
    of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`.'
- id: FR-017-AC-46
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts,
    in UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted
    record `kani-obligations/<module>.json`, `observer/Cargo.lock`, `observer/Cargo.toml`,
    `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs` (harness), and
    `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts,
    harness and record compare byte-identically under the rename map. Removing, renaming,
    reordering, duplicating or changing one byte of any entry changes the digest.
    Non-UTF-8 runner-re'
- id: FR-017-AC-47
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Two receipts for the same package and harness with different operands,
    different `ScalarLimits` and different native outcomes have equal `DigestRecord`s;
    operand order, limits, ranges, outcome and Kani evidence appear nowhere in the
    preimage. This holds for the swapped-operand runs of FR-017-AC-31.
- id: FR-017-AC-48
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The receipt exposes read-only the ordered staged `(path, bytes)` entries
    the digest was built from, including both manifests, both independently generated
    locks, the source map and the canonical `src/bin/ir791_native.rs`. All ten entries
    remain measured, sealed and compared across both execution boundaries. Recomputing
    the digest from only those exposed entries equals the receipt's `DigestRecord`.
    A recomputation that substitutes one byte in the package, harness or harness record
    (which a driver holding different bytes would do) differs from the receipt's digest.
    The exposed entries are exactl
- id: FR-017-AC-49
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A change to the preimage schema (artifact set, path names, ordering, encoding,
    canonicalization or hash) is made only by changing the schema string, with the
    golden digest of AC-46 updated in the same change; the golden is computed over
    fixed literal entries, so dependency bumps do not move it, and is asserted together
    with the schema string in one assertion; a golden change without a schema bump
    is rejected in review (a test cannot enforce it). A change to either manifest,
    either lock or adapter bytes changes the digest with the schema string unchanged.
    The domain remains `sha256-jcs` across '
- id: FR-017-AC-50
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits`
    the native meter was constructed with. They are read back from the native invocation''s
    own output, not copied from the request: a test seam that makes the adapter report
    a well-formed but different limit value makes the helper''s structured result
    unsupported in FR-017-AC-43''s sense and yields `NativeResult` / `native_result`
    / `EnvironmentFault` and no receipt. For an accepted request the request limits,
    the meter-observed limits and `scalar_limits()` are equal, and the meter-observed
    limits are unchanged after the '
- id: FR-017-AC-51
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits`
    using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits`
    compiles with that same return type, and assignment to the owning `quire_exact::ScalarLimits`
    compiles without conversion, wrapper or compatibility shim. The CG public name
    and field shape remain unchanged. Planned with IR-17 kernel adoption.
- id: FR-017-AC-52
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged.
    The isolated private observer uses resolver 2 and its sole example compiles the
    single canonical adapter in place through `../src/bin/ir791_native.rs`, with runtime
    `exact` and kernel `test-support` exposed only by dev-dependencies; no wrapper,
    copied source, symlink, normal/build logging edge or CLI feature override is allowed.
    Native invocation explicitly names `observer/Cargo.toml` and the example; the
    real production proof retains its harness, ordered options, claims, covers, bounds
    and backend gates. A control re
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/update workflow, without copying a parent/other lock
    or substituting a cached resolution. Each resolves both owning runtime and exact
    identities to the build's authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently
    of the renderer's list. Both manifests, locks and all other AC-46 entries are
    sealed and compared as a complete path/byte set before proof, after proof and
    after native execution; mutation/replacemen
- id: FR-017-CON-1
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim.
- id: FR-017-CON-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The generator SHALL NOT report a generation-time classification as an execution
    outcome.
- id: FR-025-AC-1
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: For every harness kind, the identifiers of the emitted `kani::any()` bindings,
    in source order, equal the identity's `arguments` identifiers in order, and those
    ascend by identifier.
- id: FR-025-AC-2
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: Every argument binding in a persisted identity names its parameter node
    id and declared domain.
- id: FR-025-AC-3
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A Boolean argument is a `bool` that reaches the oracle unchanged, and a
    bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from`
    of that value. The values `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` each reach
    the oracle equal to their mathematical value.
- id: FR-025-AC-4
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument whose domain has no declared finite bound yields no harness
    and is never narrowed to a machine range.
- id: FR-025-AC-5
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no
    harness.
- id: FR-025-AC-6
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A binding that names no parameter of the selected function, and a parameter
    of that function with no binding, are each refused at generation with a typed
    reason and no harness.
- id: FR-025-AC-7
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal,
    IEEE, text, enum, composite, collection or function) is accounted `unsupported`
    with a typed reason naming the family, and no harness is emitted.
- id: FR-025-AC-8
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A harness over a state-reading obligation passes each state value to the
    subject as `&mut` to a harness-owned value and copies it before the call; the
    pre-state the harness asserts over is the copy, and the post-state is the value
    after the call.
- id: FR-025-AC-9
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its
    original operand/parameter node, typed path, declared domain and primitive draw
    type in symbolic call order; controls distinguish absent/null/present slots and
    option/union members. A missing, duplicate, unknown or domain/type-conflicting
    leaf refuses with a typed reason and no harness; parameter/literal and literal/literal
    operands retain their literal singleton identity/value with no unconstrained draw.
    Public reconstruction preserves these facts through FR-033.
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock
    ceiling, and changing either ceiling changes the identity.
- id: FR-028-AC-2
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its wall-clock ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the timed-out reason naming the ceiling, never
    verified or falsified. FR-029 maps this source outcome to final QSL Incomplete(TimedOut);
    the backend classification is not the terminal category.
- id: FR-028-AC-3
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its memory ceiling is stopped and classified
    KaniRunOutcome::Inconclusive with the memory-exhausted reason naming the ceiling,
    distinct from timed-out and never verified or falsified. FR-029 maps it to final
    QSL Incomplete(ResourceExhausted); FR-028-AC-24 governs the separate refinement
    run.
- id: FR-028-AC-4
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the bounds of each symbolic argument and
    the two ceilings the run was held to.
- id: FR-028-AC-5
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity, generation refusal and execution evidence names
    its family.
- id: FR-028-AC-6
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
- id: FR-028-AC-7
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow`
    and its refinement obligation in the same result, and never the shadow harness
    alone.
- id: FR-028-AC-8
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: An argument narrowed inside its declared domain records the declared domain,
    the narrowed bound and that the harness covers only the narrowed bound, in the
    harness identity and in the evidence.
- id: FR-028-AC-9
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A narrowing outside the argument's declared domain is refused with a typed
    reason and no harness.
- id: FR-028-AC-12
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: In a group of N harnesses (one launcher process of a FR-017 batch) sharing
    a wall-clock budget T, a member the backend cuts off (entry status Failure, no
    checks, exit status `timeout`) is `inconclusive` timed-out naming T and never
    falsified, while members that finished keep their verified or falsified results;
    a group still running at N times T is killed, refused as timed out, and no member
    is classified; an N times T too large to represent never elapses and does not
    panic; a T above 4294967295 whole seconds launches the group without `--harness-timeout`
    and the group still runs, its outer bo
- id: FR-028-AC-13
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The source of a `bounded_shadow` harness, parsed, names no crate but `core`,
    `alloc`, `std` and `kani` (so no path into `quire_contract_runtime`, `quire_exact`
    or any other crate the production oracle's crate depends on), no `kani::stub`
    or `kani::stub_verified`, and no `kani::assume` outside the argument-domain assumptions
    of FR-015-AC-11; a mutated generator that imports a runtime type, emits a stub
    or assumes a relation between two drawn values fails each check. PLANNED (IR-241).
- id: FR-028-AC-14
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The result of a `bounded_shadow` family carries a refinement obligation,
    as generated Rust source in the same result, that names the production subject
    by its symbol, the shadow, the bounded domain as one inclusive bound per drawn
    argument, the abstraction function from a shadow value to a production value,
    and the observables the family's comparison reads (FR-028-AC-15); the abstraction
    function lives in one module that the shadow, the expectation and the Kani harness
    do not import, and it builds each production value through the production oracle's
    own environment, the one its generated envi
- id: FR-028-AC-15
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: For every case the refinement run executes, the production subject's complete
    outcome under an unlimited meter agrees with the shadow's result on the observables
    its family names; for composite equality (FR-015-AC-71) that is `Completed` with
    the shadow's verdict and an admitted `equality.pair` count equal to the shadow's
    count. A case whose production outcome is `Refused` or `Incomplete`, a case whose
    production value the oracle's own environment refuses to construct (a construction
    refusal), or a case that differs on a named observable, is recorded as a disagreement
    carrying the case's drawn
- id: FR-028-AC-16
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement run over a domain of at most the request's case cap cases
    executes every case and is classified `exhaustive`; a run over a larger domain,
    or one whose case count overflows `u128`, is classified `sampled`, runs the boundary
    cases and then exactly the request's sample size of seeded cases drawn uniformly
    over the whole domain from the request's seed, and records the class, the case
    count or that it overflowed, the cases run and the seed; two runs with equal inputs
    run the same cases. The boundary cases are each leaf's boundary values (each integer
    bound, its neighbour inside the bou
- id: FR-028-AC-17
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A verified `production` harness reports `production_proved`; a verified
    `bounded_shadow` harness reports `shadow_proved_refinement_exhaustive`, `shadow_proved_refinement_sampled`,
    `shadow_proved_refinement_not_run` or `shadow_proved_refinement_inconclusive`
    by its refinement run's class (`not_run` when none ran, `inconclusive` when the
    run reached a ceiling, FR-028-AC-24); a refinement disagreement reports `refinement_failed`
    whatever the shadow harness settled, including a verified one; and no result of
    a `bounded_shadow` harness reports `production_proved`. PLANNED (IR-241).
- id: FR-028-AC-18
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness identity of a shadow family lists each abstraction the shadow
    makes by name with the evidence that discharges it (a Kani check of the harness
    or a refinement comparison), each behaviour of the production subject the proof
    does not exercise, and each helper the shadow shares with its expectation, marked
    `value_bearing` (it returns a bound, a presence, a member set or a count the comparison
    reads) or `order_only` (it returns only an order the compared observables do not
    read, as FR-149's conjunction and node count do not read field order), with the
    reason for each `order_only`; a shado
- id: FR-028-AC-19
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A refinement domain that does not contain every case the shadow harness's
    assumptions admit (a bound narrower on any argument) is refused with a typed reason
    naming the argument, with no harness; and the domain a shadow harness draws from
    equals the declared domain, or the narrowed bound FR-028-AC-8 records. PLANNED
    (IR-241).
- id: FR-028-AC-20
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'An item whose shape lies outside its family''s shadow support is refused:
    a missing bound as `requires_bound` naming the unbounded node, a shape no shadow
    covers as `unsupported` naming the family and the capability, a shape over the
    family''s size budget as `unsupported` naming the size and the budget; none emits
    a harness, none invents a bound, and none falls back to a `production` harness;
    siblings of a refused item settle as they do alone. PLANNED (IR-241).'
- id: FR-028-AC-21
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: The backend process tree of every run is held to the harness identity's
    memory ceiling by a mechanism that observes or limits the tree's memory (the choice
    is the code change's, Open Questions); every execution evidence records the mechanism
    and, where it observes memory, the tree's peak resident memory; a tree that exceeds
    the ceiling is killed whole and settles as FR-028-AC-3 states, with `KaniInconclusiveReason::MemoryExhausted`,
    serialized `memory_exhausted`, a variant distinct from every existing reason;
    a FR-017 batch's launcher process tree is held to the ceiling as one group, killed
    wh
- id: FR-028-AC-22
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'A shadow family''s machinery catches each seeded mutant: a shadow harness
    over a mutated shadow is `Falsified` with a concrete playback, a refinement run
    over a mutated production subject settles `refinement_failed` naming the first
    disagreeing case, a mutant of a `value_bearing` shared helper that the production
    subject reads independently settles `refinement_failed`, one that it reads through
    the same helper is caught by the family''s independent check of that helper (FR-018-AC-23
    for the composite closure), and the unmutated family verifies with its cover satisfied
    and a refinement run that a'
- id: FR-028-AC-23
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the Kani version, the Rust compiler version,
    the solver name and version, the unwind bound and the complete option vector the
    run used, for a `production` harness and a `bounded_shadow` harness alike, and
    a refinement evidence records the Rust compiler version and the production subject's
    crate version. PLANNED (IR-241).
- id: FR-028-AC-24
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: 'The refinement run is a native process launched through one typed refinement
    execution entry that reuses FR-017''s launcher and FR-028''s ceilings: it is held
    to the memory and wall-clock ceilings its harness identity records, its evidence
    (class, case count, cases run, seed, both ceilings, mechanism, peak where observed,
    outcome, versions) is recorded beside the shadow run''s evidence, and a run that
    reaches either ceiling is stopped, records the ceiling, and yields strength `shadow_proved_refinement_inconclusive`,
    never `exhaustive`, `sampled`, `not_run` or `refinement_failed`. PLANNED (IR-241)'
- id: FR-029-AC-1
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`verified` with three SUCCESS checks maps to `Proved { success_checks:
    3 }`, and `falsified` with a reproduced replay maps to `Refuted`.'
- id: FR-029-AC-2
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each
    map to `Proved { success_checks: 0 }`, whose QSL category is `inconclusive` with
    cause `KaniVacuousProof`.'
- id: FR-029-AC-3
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted
    and exhausted-unwind-bound reasons each map to `Incomplete(ResourceExhausted)`.
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The no-verdict reason maps to `Failed`.
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The missing-cover-summary reason maps to `Failed`. The failure-without-counterexample
    reason maps by its cause (FR-029-AC-32 to FR-029-AC-35).
- id: FR-029-AC-6
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The ordinary production/source-predicate map returns no `Tested`.
- id: FR-029-AC-8
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`,
    `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`.'
- id: FR-029-AC-9
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)`
    carrying `ReplayRefusal::code()` of that refusal.'
- id: FR-029-AC-10
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a fault maps to `Failed` for each of: the call-site fault''s
    own reading (`ReplaySettlement::Fault`, which QSL''s replay result does not carry:
    QSL reports a fault as an error); `ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)`,
    each bare and as the cause of `SpineReplayError::Refused`, `FrameReplayError::Refused`
    and `StateClauseReplayError::Refused`; and `CallSiteRefusal::Fault` bare and wrapped
    in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`.'
- id: FR-029-AC-11
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with each CG-raised failure that carries no QSL code maps
    to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`,
    `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope`, `Name`,
    `NotAFrame`, `FieldSetMismatch`, `Decode`, `OutOfDomain`, `PreState` (a pre state
    the generator cannot read from the driver''s documents, or one that differs from
    the playback; a document whose bytes do not match its digest is QSL''s `Refused`,
    read by its code), `ScopeMismatch` and `Identity`; `ReplayPackageError::InvalidFunction`;
    a playback outside the harness proof'
- id: FR-029-AC-12
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `falsified` maps
    to a value other than `Refuted`.
- id: FR-029-AC-13
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to
    `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()`
    of that refusal, and never to `Declined`.'
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity`
    refusal (code `invalid_package`), as a lock whose only defect is a repeated library
    identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying
    `invalid_package`.'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified
    outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other outcome (`verified`, `cover-unsatisfied`, `cancelled` and every
    inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement
    input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- id: FR-029-AC-16
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'The state-clause replay path (FR-024) settles as the other replay paths
    do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness`
    in category `violation` is a reproduction and an `inconclusive` one is a disagreement
    carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`;
    `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal`
    do for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code;
    their fault reading is FR-029-AC-10''s), and `Dependencies` as `DependencyLockError`
    doe'
- id: FR-029-AC-17
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). IR-635''s Eq-only verified constructor invokes QSL with
    supplied evidence; the closed FR-028 strength producer/projection remains gated.
    The strength projection enumerates every FR-028-AC-17 strength: production retains
    its ordinary map; exhaustive projects to Exhausted, sampled/not_run to NotExhausted,
    ceiling-inconclusive to CeilingReached, and refinement_failed to Disagreed. Backend-inconclusive
    outcomes acquire no invented verified proof strength.'
- id: FR-029-AC-18
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`,
    whose post-state value (-1) lies outside `balance`'s declared range, this repository
    builds the post snapshot with that exact unclamped value and does not refuse it
    itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect),
    and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)`
    or `Failed`. QSL settles the exact post-state observation as a reproduced violation
    (QSL-634).
- id: FR-029-AC-19
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule,
    verified shadow CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhausted
    with QSL-derived complete coverage yields Proved n; remaining completed nonzero
    verification yields Tested. Tested is excluded for disagreement, ceilings and
    zero checks; tightening or missing coverage cannot promote it to Proved.
- id: FR-029-AC-20
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct F-1 to F-7 report consumption covered; IR-806 must
    reach these rows from a CG-built request over the original proving context). An
    identity-valid falsified composite shadow consumes the shared Refinement and ordered
    F-1 to F-7 result: Disagreed/Failed/CgDefect; native Incomplete or ExecutionFault
    as GeneratedFault/Failed with NativeCause, winning over an invalid operand and
    every later limit; RefusedInput/ReplayRefused with QSL code and operand index,
    winning over CeilingReached; admission accounting exhaustion as Incomplete(ResourceExhausted)/Admission
    with counter; exa'
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength
    input and `TerminalPairError` has no variant for a bounded-shadow outcome; adding
    either to the map is a deliberate, tested change rather than a wildcard arm.'
- id: FR-029-AC-22
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17's
    closed set; adding a strength without its projection and outcome row fails the
    build or test rather than entering a wildcard mapping.
- id: FR-029-AC-23
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary
    timeout/memory inconclusive classification; a stopped refinement retains FR-028-AC-24's
    CeilingReached projection and recorded ceiling. Neither stop, nor QSL settlement
    resource refusal, becomes completed NotExhausted evidence or Tested.
- id: FR-029-AC-24
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09
    validation, retained refinement_failed/Disagreed evidence for the same valid claim
    yields Failed/CgDefect for verified, falsified, inconclusive and cover-unsatisfied
    backend outcomes, including zero checks, an operand field x declared Int[0, 9]
    but supplied as x: 12, missing/fault native observation and a falsified replay
    agreeing on its retained case; no early operand/native setup refusal loses this
    disagreement, which precedes resource, vacuity and agreement rows. Missing observations
    are never fabricated.'
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow
    inconclusive reason maps through the ordinary rows: vacuous proof and cover-unsatisfied
    are Proved0; timeout is Incomplete(TimedOut); memory and unwind exhaustion are
    Incomplete(ResourceExhausted); no-verdict and missing-summary are Failed; failure-without-counterexample
    takes the cause rows (FR-029-AC-32 to FR-029-AC-35). No vacuous outcome is coerced
    into verified evidence.'
- id: FR-029-AC-26
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL
    record category/cause: Proved0 remains inconclusive/KaniVacuousProof and is never
    promoted by its payload. Zero-count backend cases use AC-25, not an unreachable
    verified-count-zero row.'
- id: FR-029-AC-27
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement
    input refuses a missing settlement or another claim's node/run/operation/operand/domain/limits/content
    binding with no value; ordinary source-predicate AC-15 does not consume or reject
    a valid verified-parity settlement as UnexpectedSettlement.
- id: FR-029-AC-28
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'IR-666 public converter delivered. For any full-identity-bound QSL `CompositeParityResult::Refused`
    report, CG preserves `report.terminal_value()`: non-fault refusal is Inconclusive(ReplayRefused)
    with QSL''s code, while `Fault` and `Admission(Fault)` are Failed. QSL `prepare`
    refusal precedes F-1; operand-admission and exact-comparison refusals follow F-1,
    so Disagreed wins over those later faults. CG verifies a reachable non-fault report
    by invoking QSL''s public facade with a valid wire request and a replay input-byte
    limit below that request''s size, then passing the actual report through its'
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants
    to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same
    exhaustive conversion as FR-030; supplying a replay settlement returns `UnexpectedSettlement`.
    The production cancellation path publishes only FR-030's result.
- id: FR-029-AC-30
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive
    or cover-unsatisfied) reaches only the composite parity route on the production
    path and never `run_terminal_value`, because `KaniRunOutcome` cannot tell a shadow
    from a production proof and a verified shadow outcome there would read as `Proved`,
    which FR-028-AC-17 forbids.
- id: FR-029-AC-31
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). The parity route fabricates no predicate replay and
    no terminal cause for a bounded-shadow outcome: its terminal value is the one
    QSL''s report carries under AC-17 and AC-19 to AC-27.'
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `out_of_memory`
    maps to `Incomplete(ResourceExhausted)`, and the same reason with cause `properties_failed`,
    `exit_code` (any code), `unknown` or no cause maps to `Failed`. Over a single-run
    stand-in that exits 1 and exports a `Failure` report for `proofs::check` with
    no checks and one `error_details` entry with exit status `out_of_memory`, the
    evidence passed to `run_terminal_value` gives `Incomplete(ResourceExhausted)`;
    the same report with exit status `exit_code_64`, `cbmc_exploded`, `properties_failed`,
    a null status or no `e
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `timeout`
    maps to `Incomplete(TimedOut)`: the single-run stand-in of FR-029-AC-32 with exit
    status `timeout` gives `Incomplete(TimedOut)`. The batch member `c::check` of
    FR-017-AC-39 (a `Failure` with no checks and exit status `timeout`, classified
    `TimedOut`) keeps `Incomplete(TimedOut)` through the timed-out row, and the member
    `d::check` (a `Failure` listing a failed check with no playback, exit status `timeout`,
    classified `FailedWithoutCounterexample` with cause `timeout`) gives `Incomplete(TimedOut)`
    through the cause.'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). CG''s own memory stop (the memory-exhausted reason)
    and a `FailedWithoutCounterexample` run with cause `out_of_memory` give the same
    terminal value, `Incomplete(ResourceExhausted)`, so one exhaustion has one value
    whoever observed it. The cause is read for the failure-without-counterexample
    reason only: every other outcome and reason maps identically whatever cause is
    passed, including a cause passed with `NoVerdict` or `MissingCoverSummary`.'
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). The cause never creates a verdict. Over every combination
    of outcome, reason and cause (absent, `properties_failed`, `timeout`, `out_of_memory`,
    `exit_code`, `unknown`), and with the replay settlement each outcome takes: a
    `falsified` outcome with a reproduced replay is `Refuted` whatever the cause;
    the cause never turns a non-`Refuted` result into `Refuted`, and never turns any
    result into `Proved` (`Proved` appears only for the rows FR-029-AC-1 and FR-029-AC-2
    give). The `NoVerdict` and `MissingCoverSummary` reasons map to `Failed` whatever
    the cause. Classification is unch'
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The rationale of this requirement states that Kani reports
    any `SIGKILL` of CBMC as `out_of_memory`, that an `out_of_memory` cause is accepted
    as `Incomplete(ResourceExhausted)` without corroboration, and that the raw cause
    stays in the evidence.
- id: FR-030-AC-1
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every pair (`KaniOutcomeKind`, replay settlement) that the map's input
    can express maps to exactly one `TerminalValue`.
- id: FR-030-AC-2
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with
    `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively,
    so no refusal cause is lost, and each carries the outcome''s `Std001Code` unchanged
    as `DeclineCode::Std001`, including a code STD-001 does not register.'
- id: FR-030-AC-3
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`TimedOut` and `ResourceExhausted` map to their distinct `Incomplete`
    causes. `Cancelled { source: Requested }` and `Cancelled { source: Deadline }`
    map to `IncompleteCause::Cancelled` carrying the corresponding `quire_exact::CancelCause`,
    with no source inferred from `code`, `source_id`, `context` or elapsed time.'
- id: FR-030-AC-4
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Proved` with three SUCCESS checks maps to `Proved { basis: Checks { success_checks:
    3 }, certification: Certified }`, zero checks maps to `Proved { basis: Checks
    { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous),
    and `Counterexample` with reproduced replay maps to `Refuted`.'
- id: FR-030-AC-5
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis:
    Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous),
    and `Inconclusive` with any other cause maps to `Failed`.'
- id: FR-030-AC-6
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: No outcome maps to `Tested`.
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement)
    with no wildcard arm.
- id: FR-030-AC-8
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`;
    with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.'
- id: FR-030-AC-9
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`,
    and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying
    `ReplayRefusal::code()` of that refusal.'
- id: FR-030-AC-10
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a fault, walked through every wrapper FR-029-AC-10
    lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`.'
- id: FR-030-AC-11
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `Counterexample`
    maps to a value other than `Refuted`.
- id: FR-030-AC-12
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal''s
    QSL catalog code, never to `Declined`.'
- id: FR-030-AC-13
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a `DependencyLockError::Input` that carries QSL''s
    `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect
    is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)`
    carrying `invalid_package`.'
- id: FR-030-AC-14
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value.
- id: FR-030-AC-15
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). For an identity-valid
    IR `Counterexample` composite claim, F-1 Disagreed yields Failed/CgDefect even
    with a native fault or invalid operand; without Disagreed, native Incomplete and
    ExecutionFault each yield a binding-checked GeneratedFault/Failed report with
    the actual NativeCause even when an operand would refuse, admission or exact limits
    would stop, or refinement is CeilingReached. CG asserts the typed result and terminal
    value; QSL FR-358 owns the inte
- id: FR-030-AC-16
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same
    rows from a CG-built request over the original proving context). With valid identity,
    no Disagreed and Completed native, an out-of-domain operand yields F-3 RefusedInput/ReplayRefused
    with its index and QSL code even with CeilingReached; a request accounting limit
    reached during admission yields F-4 Incomplete(ResourceExhausted)/Admission with
    the counter; an exact limit reached after admission yields F-5 Incomplete(ResourceExhausted)/ExactEvaluation
    even with CeilingReached; with sufficient limits, CeilingReached yields
- id: FR-030-AC-17
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 F-7 and binding controls; IR-806 original-artifact path
    planned). With valid identity and all earlier rows absent, changing only the retained
    shadow verdict or pair count yields F-7 Diverged/Failed/CgDefect; agreement yields
    Inconclusive(ScalarAgrees) with the CompositeEquality claim and Equality outcome.
    A binding-valid QSL `prepare` non-fault `Refused` report precedes Disagreed and
    yields Inconclusive(ReplayRefused) with QSL's code. `Refused` from later admission
    or exact comparison follows Disagreed; if QSL returns `Fault` or `Admission(Fault)`,
    its bound report yields Faile
- id: FR-030-AC-18
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every Kani `Proved` row carries `ProofBasis::Checks` with the transcript
    count and `Certification::Certified`, including the zero-check row; zero checks
    remain category inconclusive/vacuous. No Kani non-`Proved` row carries basis or
    certification.
- id: FR-030-AC-19
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT
    proof whose certificate FR-314 verifies maps to `Proved` with the same basis/depth
    and `Certified`; no certificate or an `Unverifiable` unchecked rule maps to `Proved`
    with the same basis/depth and `Uncertified`, retaining the unverifiable locus
    when present. No SMT proof maps to `Trusted`.
- id: FR-030-AC-20
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid
    checked proof step or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected
    { rule, at })` retaining the exact rejection; none produces `Proved`, basis or
    certification. When cvc5 and Z3 also disagree, `CertificateRejected` remains the
    one terminal cause while the parity failure and both verdicts are retained alongside
    it.
- id: FR-030-AC-21
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts
    and settles inconclusive; agreement does not change certification; missing Z3
    retains its warning and cvc5 settlement. An SMT counterexample becomes `Refuted`
    only after FR-197 replay reproduces it; replay disagreement, non-fault refusal
    and fault map to their distinct non-proof results.
- id: FR-030-AC-22
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces
    one terminal result, never `Tested`; no non-`Proved` result carries basis or certification.
    Negotiation refusals produce no terminal from this map, and neither SMT nor Kani
    map consumes the other backend's outcome.
- id: FR-034-AC-1
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer,
    observer readiness and positive typed Dispatch are established, no production
    backend instruction is created. Caller SIGKILL, abort or OOM-equivalent forced
    death during initialization may start only bounded guardian bootstrap, which exits
    without backend on absent lease or authorization.
- id: FR-034-AC-2
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death
    protection and original-exclusive bootstrap authority. Unclaimed inner I is contained
    by O even if M dies before internal child_wait handoff or startup-info reader
    loss aborts M. Owned outer pidfd termination confirms teardown; missing authority
    refuses before spawn. No host scan, monitor/group exit or successful signal proves
    cleanup.
- id: FR-034-AC-3
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF
    causes guardian cancellation/exit and confirmed INIT termination; Dispatched descendants
    are torn down by the kernel. The test controller observes dead-caller cleanup
    without manufacturing that caller's outcome or evidence.
- id: FR-034-AC-4
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks
    the actual mapped creator UID and only the original caller holds its executor
    endpoint. The bounded executor binds kernel credentials of Ready''s actual sender
    to claimed host INIT/start/pidfd/parent/namespace, not socketpair creator credentials
    alone. Guardian''s creator PID0 is not a host identity. A foreign actor cannot
    rebind a public name or supply replacement lease/Dispatch; another pair, mismatched
    UID or replayed authority refuses.'
- id: FR-034-AC-5
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through
    host pidfds, start/parent/namespace identities and actual outer/inner PID 1. O
    owns actual M Child and I is M's inner child. Reused, stale, incomplete, translated
    incorrectly or dead identity refuses before Dispatch.
- id: FR-034-AC-6
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The owned tree memory observer is ready before typed Dispatch; failed observer
    preparation releases no backend instruction.
- id: FR-034-AC-7
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M
    death/reparenting or malformed/missing info cancels through retained outer INIT
    authority and yields typed setup refusal; complete outer termination is independently
    confirmed. No host scan or gate EOF authorizes a backend.
- id: FR-034-AC-8
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid,
    late-born and nested-PID-namespace descendants through kernel-owned INIT teardown,
    including descendants absent from previous samples. Observed-PID lists or normal-operation
    host scans are not cancellation authority.
- id: FR-034-AC-9
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated
    host children unaffected.
- id: FR-034-AC-10
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Normal backend completion, backend exec failure, explicit cancellation,
    timeout, memory excess, capture failure and observation failure each require confirmed
    owned teardown before any proof conclusion. Backend exit or valid success output
    alone never authorizes acceptance.
- id: FR-034-AC-11
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady
    or immediately after Dispatch cannot release an unowned production backend: the
    guardian is INIT, and its death tears down that namespace. With a live original
    caller, guardian failure always yields a typed refusal and never verified/falsified
    evidence, even after confirmed teardown and beside a valid success report.'
- id: FR-034-AC-12
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Every startup refusal, completion and cancellation settles actual owned
    roles before conclusions. Anonymous controls and report pipe/memfd have no persistent
    names and end at final close. All report-owner death reclaims kernel report backing
    without Drop/unlink; other assigned artifacts retain surviving-owner cleanup and
    truthful all-owner-death limits. No dead actor is claimed to reap or unlink.
- id: FR-034-AC-13
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded executor requires an explicit path to this package's actual
    Cargo executable quire-kani-guardian, built and supplied with the library from
    the same package source/build inputs. A missing, non-executable or unusable helper
    gives a typed setup refusal before backend Dispatch, with no PATH/global discovery,
    copied executable, shell substitute or alternate launcher. Cargo library dependency
    resolution alone is not helper delivery.
- id: FR-034-AC-14
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Setup docs require the matched package helper roles, safe caller-context
    user/PID/private-mount permissions, verified mapping/setgroups/private proc, nested
    installed bwrap and pidfd/observer facilities. The compiled caller security profile
    is checked, not inferred from binary mode. Caller-profile restriction can refuse
    runs formerly available through bwrap-only permission; ordinary-caller availability
    requires actual measurement. Missing capability gives typed pre-Dispatch refusal
    without host-policy mutation or weaker mode. Added L/O roles and bounded controls/storage
    are charged to existing
- id: FR-034-AC-15
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Typed private controls reject malformed and unknown fields and enforce
    finite encoded-byte, pending-message and startup-work bounds. Ancillary descriptor
    count/type is exact, received descriptors are CLOEXEC, and unknown/extra/truncated
    ancillary data refuses while closing all received descriptors. Invalid/overlimit
    control or EOF cancels or refuses rather than authorizing Dispatch.
- id: FR-034-AC-16
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Only the original caller holds the pair's executor endpoint. Guardian receives
    only its control end as fd0 through safe CLOEXEC child-only mapping, borrows it
    with a safe descriptor API and receives actual backend stdin separately as OwnedFd
    through safe ancillary rights. Backend fd0 restores that original stdin or closed
    state, while the control/lease descriptor is excluded from backend and unrelated
    exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs.
- id: FR-034-AC-17
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Backend receives unchanged raw argument bytes except the explicitly allocated
    internal report locator: harness options then -Z unstable-options --export-json
    /proc/self/fd/N with actual mapped N >=5. Evidence records that exact argv. Inherited
    stdin, inherited/overridden environment and cwd remain unchanged.'
- id: FR-034-AC-18
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr
    captures and cannot contaminate reports; existing bounded capture and capture-failure
    behavior remain authoritative.
- id: FR-034-AC-19
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Original identity ceilings and existing outcome classifications remain
    authoritative after confirmed cleanup: memory excess and wall expiry stay distinct,
    ambiguous live-worker RSS refuses, and ordinary completed/refused/falsified reports
    retain their meanings. Native refinement uses this ownership path when its FR-028
    AC-24 typed entry is implemented, with unchanged refinement class/evidence rules.'
- id: FR-034-AC-20
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection, startup, identity verification and Dispatch use the
    original monotonic identity deadline without resetting it. An already expired
    deadline causes no Dispatch and settles inconclusive with the timed-out reason
    as FR-028 AC-2 states; actual identity-deadline expiry during setup or execution
    uses the same classification.
- id: FR-034-AC-21
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining
    original deadline. Cap expiry while that deadline remains live is a typed setup
    refusal, distinct from identity-deadline timeout.
- id: FR-034-AC-22
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds.
    Unconfirmed termination refuses with a live caller rather than accepting a proof
    or claiming physical disappearance of an uninterruptible task.
- id: FR-034-AC-23
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Caller-death fixtures use the actual matching normal helper and shared
    production functions/order with bounded prefix selection as input only. Actual
    caller/group death is followed by independent original owned M/I pidfd termination,
    no backend Dispatch/marker and no leaked owned inner processes before emergency
    cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit;
    AC-31 remains independently owed. Cached prefix or gate-retained metadata proves
    no live gate. Feature-off positive post-Dispatch controls remain required.
- id: FR-034-AC-24
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Actual lease-EOF cancellation passes independent original I termination
    and no backend Dispatch/marker before escalation, plus death of a positively acknowledged
    pinned worker after Dispatch. Ignored EOF, skipped actual lease close, removed
    positive Dispatch, non-INIT watcher, broken session isolation and startup-order
    mutants independently fail their genuine predicates; later cleanup cannot rescue
    them. The immediate private LeaseCloseObservation remains control flow, not a
    stored certificate. Stopped-I pending-authorization coordination remains owed
    without close/publication snapshots or ord
- id: FR-034-AC-25
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Before backend Dispatch, the helper handshake matches the actual running
    CG library's build, protocol and lifecycle-capability identity against the actual
    invoked first-party executable. A stale helper or changed lifecycle implementation
    refuses, even if a caller supplies a matching version label or digest. Expected
    identity derives from actual library/helper build artifacts, not caller assertions
    or manually maintained tracking pins.
- id: FR-034-AC-26
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The guardian session/group is distinct from the original caller before
    Ready/Dispatch. Real feature-on caller-group deaths and feature-off positive post-Dispatch
    death controls require independent owned termination/no backend/no leaked processes.
    Unclaimed Bootstrap cannot infer I death or held gate from monitor death or cached
    phase; AC-31 remains separately mandatory. Claimed I termination, operational
    lease cleanup and directly killed guardian kernel teardown remain required.
- id: FR-034-AC-27
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: guardian-test-support is off by default, absent from default features,
    and exposes exactly one documented fixture operation only when explicitly enabled.
    A feature-off consumer cannot use that operation. No public lease, process-ownership
    handle, cancellation entry or cleanup-deferring callback is exported. The private
    initialized death witness transfers exactly one actual owned monitor or validated
    INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup
    stdout is safely duplicated into a non-stdio CLOEXEC OwnedFd and itself marked
    CLOEXEC before any spawn; every chi
- id: FR-034-AC-28
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Fixture prefix selects unchanged production transitions, never certifies
    live state. Actual original identity/right custody, independent pre-escalation
    I/worker/marker checks and immediate unchanged cleanup on success/refusal/error
    replace stored stage/close/publication certificates. Intentional C/group self-death
    follows complete bounded operational pin delivery without ACK/pause or fabricated
    result. Missing identity, observation or genuine coordination fails; unavailable
    exact early-release observation remains UNBACKED. No stored record or later cleanup
    repairs an oracle.
- id: FR-034-AC-29
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The packaged caller fixture and real helper link the same normal library
    artifact through consumer-manifest package selection with matching target/profile/features/compiler
    flags. Separate named feature-off/on invocations avoid self dev-dependency feature
    unification. No cfg-test library or identity override is accepted. The bounded
    executor refuses feature mismatch in both directions before Dispatch; feature-off
    consumer compilation verifies absence of the fixture operation.
- id: FR-034-AC-30
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: CG publishes the test-only feature contract and allocates production-driver
    dependency-edge exclusion to IR-649's QSL driver work. The contract requires all
    downstream production-build profiles to reject direct or transitively unified
    guardian-test-support. CG inspection verifies the published allocation and checks;
    downstream assertion evidence is owned by IR-649.
- id: FR-034-AC-31
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait
    caller death and natural M/info-reader failure, and pass only with confirmed actual
    outer INIT and unclaimed inner/escaped descendant termination. Exercise C/group,
    L, O and I death before/after parent-death arm with positive barriers; pre-arm
    has no inner child and exclusive EOF gives bounded refusal. Original-exclusive-lease
    race, actual host/outer identity translation, mapping/private-proc capability
    refusal, real observer, ordinary-caller capability measurement, spawning-thread
    lifetime and '
- id: FR-034-AC-32
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without
    a backend/collector completion wait cycle. A hard 16 MiB plus one detection-byte
    retention bound and the defined owned_RSS + caller_run_buffers + page-rounded
    F_GETPIPE_SZ + page-rounded pre-reserved memfd maximum comparison applies while
    writing against the original ceiling; unmapped shmem is not zero. Beyond cap yields
    owned cancellation and single-run KaniRunOutcome::Inconclusive with MemoryExhausted;
    batches keep whole-batch memory-exhausted refusal, with no member classified.
    Evidence names the report cap separatel
- id: FR-034-AC-33
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner
    teardown and M termination/reap closing ALL pipe writers, including reopened procfd
    and descendant copies. Bounded actual-EOF drain precedes immutable WRITE/GROW/SHRINK/SEAL
    seals, consumer F_GET_SEALS verification and stable actual OwnedFd reads, all
    under the original deadline. Separate final control/report delivery remains live
    after lease close. Pre-Completed reader failure cancels O/I. All storage-owner
    death reclaims backing at final close without persistent report residue or surviving-owner
    dependence. Inheritance, s
- id: FR-034-AC-34
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: Live-C production close_lease_and_observe retains L/O/M/I ownership and
    final report control independently of consumed original lease. No outer kill,
    bootstrap EOF or parent-death cascade masks AC-24's independent pre-escalation
    EOF and termination checks; ignored-EOF still fails its named actual-behavior
    predicate before cleanup. Missing conservative backing accounting or immutable
    kernel report sealing gives typed refusal and owned cancellation. All roles, private
    namespace/proc setup, controls, report backing and observation use existing whole-run
    ceilings and original deadline, with no res
- id: FR-034-AC-35
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before
    M, and nested bind / / refers to confined O root. Safe backend-only seccomp/privilege
    installation before Dispatch preserves the same positively owned PID and original
    recipe, with no unfiltered Dispatch or extra surviving process/ownership/deadline;
    policy installation failure is pre-Dispatch, actual recipe exec/failure is post-Dispatch
    with existing bounded handling/no fabricated evidence; trusted installer starts
    in sanitized loader environment and restores original backend environment only
    at filtered recipe exec; I
- id: FR-034-AC-36
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open
    socket input and failed fstat inspection refuse before Dispatch; actual production
    fd1/fd2 are capture pipes verified by mapping/inventory Analysis, not caller socket
    cases. Caller fd0..2 stability throughout setup is a trusted precondition; observed
    capture inconsistency refuses, with no claim to detect every ambient mutation.
    C internally captures/pins OriginalStdin before child/control fd reuse, without
    a public request field; authoritative initial absence/CLOEXEC yields Closed, while
    later Open inspection EBA
- id: FR-034-AC-37
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch;
    O retains separate final delivery after original lease close. All bootstrap/ownership/report/reporter
    controls remain owned/CLOEXEC outside intended mappings and unavailable to arbitrary
    backend, sibling exec and descendants. I confirms non-dumpability after final
    credentials and backend cannot hold or regain CAP_SYS_PTRACE in I owning user
    namespace; real backend /proc/1/fd, pidfd_getfd and ptrace gates prove protection
    independently of host Yama. Actual leaked-control/protection mutants fail before
    emergency cleanup; rest
- id: FR-034-AC-38
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite
    effective reserve stays inside original whole T; short finite ceilings stay admitted
    with R_eff=min(R,T/2), no minimum-budget cause. At finite workdeadline=T-R_eff
    stop/cancel; normal returns positively settle/reap all owned roles, captures and
    existing creator thread by original T before conclusions; original None/overflow
    never-elapsing work remains admitted and FIRST actual stop starts one R settlement
    deadline. Confirmed workdeadline expiry preserves existing TimedOut classification
    naming T; unconfirmed settlement
- id: FR-034-AC-39
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg
    Analysis; unreachable unsupported-native support is not a fabricated runtime Test.
    Actual installation failures are separate production Tests when available, otherwise
    unavailable/UNRUN with no Test completion credit. Planned mandatory admission
    context distinguishes BackendIpcExclusion capability support/filter installation
    from TrustedOwnerProtection privilege installation, retaining typed cause provenance
    without message parsing or fabricated errno. The policy terminates the calling
    process on an unsupported sysca
- id: FR-034-AC-40
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the
    cross-role representation. Actual kind/errno, independently authenticated optional
    finite provenance and loss survive privately; public projection applies only to
    existing io::Error-cause-bearing refusals. Post-admission observation/settlement
    detail-only results keep their existing shape/mapping, private finite facts/loss
    and diagnostic-only text, with no public marker/carrier/downcast promise or Tool
    reclassification; generic original custom sources/chains are lost explicitly,
    including valid original boxed declared ope
- id: FR-034-AC-41
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). For a named native-accounting conformance claim,
    the analysis identifies the exact consumer build configuration and compiler actually
    used, including resolved profile/features/target/flags/runtime/allocator. A proof
    for a different configuration cannot establish that claim.
- id: FR-034-AC-42
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The proof is derived from GNU objdump or llvm-objdump
    disassembly using stable compiler inputs of the actual final linked executable
    after LTO/linking. A crate-level or pre-link emit-asm/emit-obj result cannot pass
    as that final-artifact proof.
- id: FR-034-AC-43
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Every named operation and thread-entry root maps
    to final address ranges or conservatively covered host frames, and every reachable
    direct or indirect call target is accounted for. An unresolved reachable indirect
    target leaves the native-workspace bound UNPROVEN.
- id: FR-034-AC-44
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A reachable recursive call cycle, including one
    through an external callee, leaves the native-workspace bound UNPROVEN under this
    nonrecursive method.
- id: FR-034-AC-45
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The native-stack bound sums per-thread maxima for
    every concurrently live thread instance carrying named state, accounting for thread-entry
    roots, actual tail-call replacement and dynamic adjustments. Unknown instance
    counts or lifetimes fail the claim. An unresolved frame or adjustment cannot be
    filled by a guessed reserve.
- id: FR-034-AC-46
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: 'PLANNED/UNRUN (IR-682). The analysis covers the actual linked panic strategy:
    reachable unwind/landing-pad and cleanup paths contribute to the stack bound,
    or the final artifact establishes their absence under abort. Assuming the strategy
    from a source profile default fails.'
- id: FR-034-AC-47
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Each relevant external library/runtime/libc/allocator
    frame has an explicit finite bound source matching the actual loaded runtime ABI/configuration.
    A missing source leaves the whole named-workspace claim UNPROVEN without changing
    the existing incidental-allocation exclusions.
- id: FR-034-AC-48
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). The named heap inventory covers actual returned
    Rust capacities times element size, metadata and simultaneous lifetimes, excluding
    incidental allocator chunk overhead. A larger actual capacity or overlapping temporary
    omitted from the checked caller-buffer sum fails the conformance claim.
- id: FR-034-AC-49
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). A change to the source head, resolved consumer
    configuration/compiler, resulting executable or loaded runtime/bound premises
    invalidates the former proof and requires analysis again. A stale proof or merely
    matching caller/helper artifact does not satisfy native-accounting conformance.
- id: FR-034-AC-50
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-682). Missing or incomplete proof, or a proven bound
    exceeding the independently declared runtime charge, leaves the configuration
    UNPROVEN and fails the conformance gate; a missing actual named charge retains
    the existing runtime refusal. This build gate introduces no runtime refusal/public
    capability or evidence of completed Analysis; the initial release configuration
    and other eligible configurations remain UNPROVEN until their own proofs exist.
- id: FR-034-AC-51
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's
    own acknowledgement establishes release-dependent birth after the actual completed
    ordinary O sample.
- id: FR-034-AC-52
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly
    precedes matching kernel report seal; independently observed fabricated-live-I
    and late-poll controls fail without stored-record or ordinal proof.
- id: FR-034-AC-53
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes
    matching kernel report seal; independent fabricated-reap and late-reap controls
    fail without cached Some, copied history or ordinal proof.
- id: FR-034-AC-54
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation
    path, the actual original O gate owner retains the gate through actual I confirmation
    before actual close; early-close and omitted-confirmation controls fail without
    marker, stage or refusal-label proof. Genuine unavailable confirmation at the
    original cutoff follows AC-38 CleanupUnconfirmed with explicit actual-owner abandonment/right
    disposal and no ordering or cleanup credit, never a renamed fatal close or diagnosis
    of kernel fault. Healthy restored order and unavailable-confirmation refusal are
    separate control
- id: FR-034-AC-55
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Independently checked final-source Analysis establishes
    contained teardown authority through actual namespace INIT death and retained
    owner settlement, with no sampled-PID membership kill path; the separate AC-24
    non-INIT-watcher runtime adverse/restored control remains owed.
- id: FR-034-AC-56
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Published-source Analysis establishes O's final complete
    accounting after actual I confirmation, retained M reap, writer EOF and immutable
    seal for successful report Commit, with all later due ticks preserved; earlier
    final accounting invalidates literal-FINAL-oracle retirement.
- id: FR-034-AC-57
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly
    allocated authenticated O→L OperationalFailure carries zero ancillary rights.
- id: FR-034-AC-58
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained
    O Child PID with kernel sender UID/GID0 and the original build/run/source/state/stamp
    authority.
- id: FR-034-AC-59
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). For L→C AwaitArm negatives, C rejects missing,
    extra, wrong-type or substituted rights through owned cleanup, without a zero-right
    fallback.
- id: FR-034-AC-60
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination
    observation and existing owned-cancellation signalling, while L retains its direct
    Child/wait obligation.
- id: FR-034-AC-61
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network,
    phase, report or Dispatch admission.
- id: FR-034-AC-62
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L
    wait and preserves the original earliest cutoff, including absent work deadline,
    without receipt-time reset.
- id: FR-034-AC-63
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only
    after actual O Child wait at L and required L/capture/creator/control-EOF settlement
    under that cutoff.
- id: FR-034-AC-64
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The new negative routes reuse existing charged
    controls and ceilings without a new envelope type, ACK, window, cap or public
    category.
- id: FR-034-AC-65
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting
    pre-send tick retires an unsent genuine OperationalFailure into existing OwnerStop
    under existing precedence, even when the failure occurred earlier.
- id: FR-034-AC-66
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route
    forwards the same bytes to C with exactly one clone of L’s retained O pidfd.
- id: FR-034-AC-67
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in
    AwaitArm through the actual L chain and original O/run/build bindings.
- id: FR-034-AC-68
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). C does not use the negative pidfd for pidfd_getfd,
    pidfd setns, process_madvise, other-actor authority or numeric-PID reopening.
- id: FR-034-AC-69
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides
    the provisional negative with existing CleanupUnconfirmed and no execution evidence.
- id: FR-034-AC-70
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes
    the candidate; partial/full publication cannot be rewritten, spliced or followed
    by a second terminal frame.
- id: FR-034-AC-71
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine
    fresh complete named-input tick before initial terminal emission; failed current
    input never borrows historical/default peaks.
- id: FR-034-AC-72
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). After COMMIT, later successful complete exhaustion
    stays in actual private O accounting until genuine settlement; retention is an
    Analysis obligation with no current runtime seam or Test credit.
- id: FR-034-AC-73
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains
    provisional until positive actual I termination and separate real M Child reap,
    with required writer EOF/final due accounting before normal O return.
- id: FR-034-AC-74
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). The negative’s absent peak field supplies no zero
    measurement, no-history claim or current-validity substitute.
- id: FR-034-AC-75
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport
    error after COMMIT remains owned through bounded cleanup and prevents normal O
    Code0 acceptance through existing CleanupUnconfirmed.
- id: FR-034-AC-76
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle
    distinguishes L's retained-pidfd clone from independently opened same-O and foreign
    pidfds; inode equality supplies no clone evidence.
- id: FR-034-AC-77
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte
    COMMIT, O immediately initiates the same actual owned cancellation of live claimed
    I and retained M without waiting for C lease closure, preserving the original
    cutoff and committed bytes.
- id: FR-034-AC-78
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained
    O/build/run authentication and original owned rights; wrong sender/run or unvalidated
    capability cannot grant positive admission.
- id: FR-034-AC-79
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial
    or forbidden-right traffic without granting positive phase/Dispatch/report authority;
    a stored event or EOF never supplies a missing authenticated control.
- id: FR-034-AC-80
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes
    completion reception; M exit, EOF, phase or copied metadata cannot substitute.
    The separate stored completion certificate is retired.
- id: FR-034-AC-81
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control
    storage and pipe/backing reservation is charged before exposure; retired certificate
    storage is removed from charges only after its actual allocation disappears.
- id: FR-034-AC-82
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical
    paired consumer/helper checks establish feature-off absence and unchanged production
    frames/rights after certificate retirement.
- id: FR-034-AC-83
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently
    of retained monitor/INIT ownership and does not leak into children; a leaked alias
    cannot preserve authorization after actual caller death.
- id: FR-034-AC-84
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint
    is excluded from untrusted child/exec mappings and public control handles through
    actual descriptor checks.
- id: FR-034-AC-85
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated
    or cached Child Some(status); an independent genuine-operation predicate rejects
    cache-only or fabricated success.
- id: FR-034-AC-86
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    provenance under the original sole-waiter/disposition/history; independent checks
    reject fabricated, cached or missing consuming-operation facts without stored
    history as proof.
- id: FR-034-AC-87
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal.
    On AC-54's independently grounded supported cooperative path, actual I confirmation
    precedes retained-gate close; late-operation/early-close mutants fail. AC-38 unavailable-confirmation
    abandonment at the original cutoff grants no seal, order or cleanup credit; a
    skipped poll, lost pin or owner defect is not a kernel-fault diagnosis. Producer
    ordinal certificates are retired.
- id: FR-034-AC-88
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff
    transition without observation ACK, pause, stored publication permission or suppressed
    tick.
- id: FR-034-AC-89
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop
    adoption precede actual original lease close, without a separate stored completion
    certificate.
- id: FR-034-AC-90
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate
    O or alter ordinary sampling/control/settlement/cancellation; it cannot rescue
    ignored EOF. The certificate pipe writer is retired, not activated by this condition.
- id: FR-034-AC-91
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Ordinary feature-on execution remains available without
    retired observation binding, endpoint, event emission or default identity.
- id: FR-034-AC-92
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent
    child birth after an actual complete ordinary O sample remain required without
    a stored sample-announcement certificate; missing real coordination is UNBACKED.
- id: FR-034-AC-93
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Every surviving feature-on named native workspace has separate
    supported paired-artifact finite-bound Analysis and an independent charge; sole-purpose
    certificate workspace retires only with its actual allocation.
- id: FR-034-AC-94
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. C disposes the defined pending claim through actual close
    of its raw I right without positive admission or capability; invalid prior traffic
    and live identity mismatches refuse, while the defined same-cursor negative receipt
    preserves the original cause only after authenticated delivery and whole-chain
    settlement.
- id: FR-034-AC-95
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through
    collection, kernel sealing and final descriptor delivery; a substituted backing
    cannot satisfy the genuine report predicate.
- id: FR-034-AC-96
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control
    authority refuses without positive admission, retaining actual received-right
    custody and original cutoff; no default identity supplies authority.
- id: FR-034-AC-97
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement
    comparison; actual original report backing equality and delivery/settlement authority
    remain required.
- id: FR-034-AC-98
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase
    bytes/rights with no retired early field or branches; ordinary feature-on execution
    has no retired endpoint/binding requirement.
- id: FR-034-AC-99
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    stable single-thread proof, including independent across-PID-namespace initial
    untraced evidence, pending/permission/configuration/accounting and real normal-helper
    controls. Certificate-writer retirement supplies no signal proof or activation.
- id: FR-034-AC-100
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles
    permit no optional fixture write, preserving pending signals, ordinary flow and
    original cutoff; no later cleanup or private-proc zero alone repairs eligibility.
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run
    request, and the bounded launcher observes it before spawn and during an actual
    running backend. Pre-spawn cancellation starts no backend; running cancellation
    confirms owned teardown before evidence returns.
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse
    the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed
    IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }` retain
    the first cause; a later event cannot change them. A hand-built IR outcome alone
    does not satisfy this criterion.
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains
    `TimedOut`. A `Deadline` event recorded only after completion, memory/capture
    stop or elapsed-ceiling stop does not turn that result into `Cancelled`.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result;
    measured memory exhaustion, memory observation failure, output over-limit, unread
    capture and unconfirmed cleanup keep their existing result/refusal and confirmed
    teardown rules when cancellation competes. A cancellation observed first and causing
    stop is not overwritten by later completion or elapsed time.
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause,
    returns `BatchCancelled { source, members }`, and classifies no member; another
    group using a distinct handle retains its own result. Reversing the two cause
    calls changes the cancelled group's source and not the other group's evidence.
    Distinct handles never share one launcher.
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled {
    source }` from settled execution evidence, and its real call path reaches the
    FR-030 map. Mapping both IR source variants exhaustively yields the identical
    `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither
    metadata nor a copied enum determines the source.
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled`
    through a type-checked exhaustive conversion, while all QSL imports remain through
    `qsl-replay`; no implicit cancellation control, source fallback, or compatibility
    overload is present.
- id: FR-041-AC-1
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A nonempty reviewed declaration is checked against the current compiled-module
    census and the merge-base declaration. An absent compiled module, a removed claim,
    and an empty list each name the claim or condition and fail; adding a compiled
    claim does not erase an older one.
- id: FR-041-AC-2
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: With two claimed modules sharing one `src/lib.rs`, a SUCCESS location inside
    only the first and an UNREACHABLE check inside the second reports the second `unreached`
    and fails. A SUCCESS check inside each passes this census; a SATISFIED cover,
    unknown location or non-SUCCESS status cannot supply the missing check.
- id: FR-041-AC-3
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: The current build's transitive expectation/subject reachability yields
    the shared-helper intersection; the gate fails when the checked-in generated evidence
    omits or adds a helper, and records an explicit empty set when none is shared.
    For an exact operation, the expectation uses QSpec vectors or a model with no
    `quire-exact` call.
- id: FR-041-AC-4
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: Each claimed module and computed shared helper has a positive unmodified
    real-Kani proof and a source mutation that compiles, executes, and reports a failed
    proposition check. Deleting a control or making its mutant verify, fail to compile,
    time out, skip execution, or merely trip the dirty-tree guard fails the gate;
    evidence names the exact diff, command and failed check from this run.
- id: FR-041-AC-5
  path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  excerpt: A stubbed executor, predetermined outcome or build-only run produces no
    qualifying module evidence and cannot pass the gate.
- id: NFR-006-AC-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `required` and the path for each of `src/kani/generate/negotiate.rs`,
    `src/oracle/boolean_v1.rs`, `src/routed/generate.rs`, `src/replay/witness.rs`,
    `src/core/identity.rs`, `src/publication/mod.rs`, `tests/it/kani_batching.rs`,
    `tests/it/skeleton_spine.rs`, `tests/it/bounded_kani_corpus.rs`, `tests/it/kani_obligations_state_frame.rs`,
    `tests/it/scratch_crate.rs`, `tests/exact_scalar_support/package.rs`, `tests/checked_package_support/base.rs`,
    `tests/checked_package_support/rekey.rs`, `tests/state_frame_support/model.rs`,
    `schemas/kani-proof-graph-v2.schema.json`, `sch'
- id: NFR-006-AC-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-scope` prints `not required` for each of `spec/kani/functional/FR-017-kani-execution-evidence.md`,
    `reviews/REV-018-bound-coverage-observations.md`, `src/strategy/mod.rs`, `src/evidence/mod.rs`,
    `Makefile` and `.github/workflows/ci.yml`, each fed alone.'
- id: NFR-006-AC-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A rename of `src/kani/old.rs` to `src/strategy/new.rs`, and a rename of
    `src/strategy/old.rs` to `src/kani/new.rs`, each print `required`.
- id: NFR-006-AC-4
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n kani` and `make -n kani-gate` expand to the same `cargo test`
    command line, and that line holds each of the six filters `kani_obligations`,
    `skeleton_spine`, `kani_witness_join`, `bounded_kani_corpus`, `kani_generation`
    and `kani_batching`.'
- id: NFR-006-AC-5
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` starts the lane''s `cargo test` once with `--test-threads=1`,
    holds the lock named by `KANI_LOCK` while it runs, and does not start it while
    another process holds that lock.'
- id: NFR-006-AC-6
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make kani-gate` exits non-zero and prints `result=failed` when the lane''s
    filters list zero tests, when fewer tests run than are listed, and when any test
    fails; it exits zero and prints `result=passed` only when every listed test ran
    and passed and the count is above zero.'
- id: NFR-006-AC-7
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: The passing run-result line has the fields `result`, `ran`, `expected`,
    `elapsed`, `kani`, `tree` in that order, with `ran` equal to `expected`, a non-empty
    `kani` equal to the version the launcher reports, and `tree=clean`; it has no
    commit token. A run whose launcher reports no version prints `result=failed`.
- id: NFR-006-AC-8
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: When the tree state or the head commit read after the run differs from
    the one read before the build, `make kani-gate` prints `result=failed` and exits
    non-zero.
- id: NFR-006-AC-9
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: With a dirty working tree `make kani-gate` prints `tree=dirty`, never `result=passed`,
    and exits non-zero.
- id: NFR-006-AC-10
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'With no `cargo-kani` launcher `make kani-gate` starts no test, prints
    `kani-gate: not run: launcher absent` and exits non-zero.'
- id: NFR-006-AC-11
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: 'The tests marked `#[ignore = "kani lane: ..."]` in `tests/it` are exactly
    the `#[ignore]`d tests of `tests/it` whose path holds one of the `make kani` filters.'
- id: NFR-006-AC-12
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`make -n ci` expands to no `kani` or `kani-gate` command.'
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: For a Kani-touching pull request, the author runs the full gate on its
    clean, stable candidate before opening it, and the merger runs it again on the
    clean, stable current candidate before merge; both actual processes exit successfully
    with all expected tests run. A copied body line alone cannot satisfy either run.
- id: NFR-006-AC-14
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Given Kani-reaching paths changed on `origin/main` since the candidate's
    merge base and main absent from the candidate, `make kani-gate` refuses before
    running tests and names those paths. Given only non-touching main changes, it
    may run. The merger updates a stale candidate from current main and the full gate
    passes on the updated clean candidate before merge.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A Kani-touching pull request whose required lane did not run or failed
    remains unmerged and does not claim real-Kani verification; a written explanation
    does not waive the gate.
- id: NFR-006-AC-16
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: A pull request whose scope is `not required` has no mandatory real-Kani
    run under this requirement.
- id: NFR-006-AC-17
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Before a version tag is pushed, the full gate actually passes on the clean,
    stable candidate to be tagged; a copied release-ticket line cannot substitute
    for that run.
- id: NFR-006-AC-18
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Every `include!`, `include_str!`, `include_bytes!` and `#[path]` target
    named in a file of `tests/it` that holds a lane test, and in each file those name
    in turn, matches a pattern of the Kani-touching set.
- id: NFR-006-AC-19
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: On a clean current candidate, `make kani-gate` runs FR-041's module census
    and real mutation controls in its one required lane, and a failure of any claim
    or shared-helper control makes the gate exit non-zero. An optional mutation target
    or saved earlier result cannot substitute.
- id: NFR-006-M-1
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Merged pull requests touching the Kani-touching set without an actual full
    gate passing on the current candidate immediately before merge
- id: NFR-006-M-2
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran`
    of 0
- id: NFR-006-M-3
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
- id: FR-014-AC-1
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition,
    and a refused item contributes no generated function while its siblings are generated
    unchanged.
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching runtime `exact` operator.
    Entry-point dispositions for a `quire.op.quantity.convert` node: derivation and
    generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and capability settlement '
- id: FR-014-AC-3
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose arity, result or operand scalar type disagrees with
    the lowered node, a node that is not a lowered scalar expression, a node that
    is unbounded, has an invalid or incomplete body, or exhausts lowering work, or
    a node id requested more than once, is refused with a typed reason.
- id: FR-014-AC-4
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order; the generated crate is exactly `Cargo.toml`, `src/lib.rs`
    and `claim-map.json`, its `claim-map.json` equals the returned claim map, every
    generated claim's oracle symbol is defined in `src/lib.rs`, and claim-map entries
    are ordered by node id.
- id: FR-014-AC-5
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, bounds and operation identity of its item, and, for a `quire.op.quantity.convert`
    item, the `type_argument` type node id (FR-014-AC-48).
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers), U14 (th'
- id: FR-014-AC-7
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Composite, collection, function, model, relation, state, temporal and protocol
    nodes are refused with their blocked or unsupported reason and never reported
    as generated.
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: FR-014-AC-9
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Generated oracle functions do not panic: an invalid generated constant,
    including a decimal target, stops as `InvalidConstant`, an operand of the wrong
    width stops before any charge, and generated source over its ceiling is a typed
    error with no output.'
- id: FR-014-AC-10
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor parameter whose admitted bound is missing, repeated or unequal
    to the reachable `bounded_domain` node, an operand that is neither a literal nor
    a reference, and a literal quantity operand, are each refused by CG with a typed
    reason. A malformed bound with a noncanonical integer endpoint such as `05` is
    refused by Contract IR's strict checked-package reader before CG receives an admitted
    item; CG retains its defensive typed `UnreadableBound` refusal for an admitted
    bound it cannot decode. Contract IR's FR-038 "Mode" reader rule also refuses a
    bare Float32 addition authored with the
- id: FR-014-AC-11
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose descriptor passes every check this requirement states, and
    whose descriptor agrees with the node's catalogued `operation.identity`, law definition
    and mode value, marks its operation `ir_confirmed`. An item refused by any of
    those checks is `caller_declared` even where its operation agrees.
- id: FR-014-AC-12
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor naming a different catalogued operation than the node's own
    is not confirmed, including where the two share one operand shape and one checked
    bound; nor is a descriptor naming an operation the catalogue has no entry for.
- id: FR-014-AC-13
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A descriptor whose implied identity matches the node's but whose law definition
    is absent from that node's `operation.laws`, or whose mode value disagrees with
    that node's `operation.mode`, is not confirmed; the item still lowers and still
    generates the oracle its descriptor names, marked `caller_declared` with a typed
    blocked item.
- id: FR-014-AC-14
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A claim whose node this generator never inspected — a duplicate copy, or
    a record that never lowered — or inspected and refused with a typed reason, marks
    its operation `caller_declared` with a typed blocked item and reports the request
    item's own descriptor-derived identity.
- id: FR-014-AC-15
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A requested item whose node closure exceeds the 65,536-unit lowering work
    ceiling is refused as `LoweringWorkExhausted`, naming the ceiling and the consumed
    counter, contributes no generated function, and leaves every other item's disposition
    unaffected.
- id: FR-014-AC-16
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A bound is read from `binding` members looked up by name in any order (`min`/`max`,
    `numerator_min` through `denominator_max`, `coefficient_min` through `rounding`,
    `rounding`, `min`/`max`/`text_profile`); a bare literal member, a missing, duplicate
    or unlisted name is refused as an unreadable bound.
- id: FR-014-AC-17
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand whose target is typed by a `bounded_domain` node
    is classified by that domain's base scalar type, so `x + 1` over a parameter `x`
    of type `Int[0, 9]` generates, and a bounded text parameter is still a text operand.
- id: FR-014-AC-18
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`derive_exact_scalar_items` returns, per node id in input order, the descriptor
    determined by the node''s `operation.identity`, `operation.mode`, `operation.laws`,
    operand forms and the one bound of the needed form on its result type; a node
    that is not an application, has no identity, names an identity outside the derivable
    set, has operand forms, a law or a mode that select no parameter, is refused with
    the matching `ClaimDerivationRefusal` and no descriptor; a node that is absent,
    does not lower or has a refused bound (missing, repeated or unreadable) is refused
    with the `ExactScalarRefusal`'
- id: FR-014-AC-19
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A derived item generates an oracle whose operation is `ir_confirmed`, and
    for every item of the corpus the derived descriptor equals the descriptor the
    fixture declares.
- id: FR-014-AC-20
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An integer node over two distinct bounded parameters, `Int[0, 9]` and `Int[10,
    20]`, each typed by its own `integer_range` `bounded_domain`, with its result
    typed by a third `integer_range` `[0, 29]`, generates an `ir_confirmed` oracle
    whose descriptor is compared with the result bound only; the claim's checked bounds
    are the result bound, then each operand's own bound; derivation returns the descriptor
    over `[0, 29]`.
- id: FR-014-AC-21
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose reachable bounds
    are its operands' own and exactly one other takes that other as its result bound.
- id: FR-014-AC-22
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node whose result is typed by a scalar type and whose two or more reachable
    bounds of the form are not exactly one outside the operands' own bounds is refused
    as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-23
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An operand, or the result, typed by a `bounded_domain` of a form other
    than the descriptor's is refused as `MissingBound`.
- id: FR-014-AC-24
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'An operand''s own bound need not lie inside the result bound: `a + wide`
    over `[0, 9]` and `[0, 50]` with a `[0, 29]` result, `-e` over `[1, 9]` with a
    `[-9, -1]` result and `e * f` over `[1, 9]` and `[100, 200]` with a `[100, 1800]`
    result each generate, derive the same descriptor, and record each operand''s own
    bound.'
- id: FR-014-AC-25
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A `reference` operand to a plain scalar parameter or a non-literal `value`
    node is refused as `RequiresBound` naming its type, in generation and in derivation,
    where another operand owns a bound or the result is typed by one (`x + y`, `y
    + y`, and `x + y` over a scalar-typed result); a literal operand is never refused
    as `RequiresBound`. An eligible application reference follows FR-014-AC-45 instead.
- id: FR-014-AC-26
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `reference` operand whose target is a `value` node with a literal body
    is a literal operand: `x + 1` over `x: Int[0, 9]` narrowed to `Int[0, 10]`, with
    the literal as a reference to its own plain-Integer-typed `value` node, generates
    with checked bounds `[0, 10]` then `[0, 9]` and derives the descriptor over `[0,
    10]`.'
- id: FR-014-AC-27
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A node whose result is typed by a scalar type and that only narrowing
    conversions (`quire.op.numeric.narrow`, exactly one argument referencing the node)
    consume takes the bound they narrow it to as its result bound: `x + y` over `Int[0,
    9]` and `Int[10, 20]` narrowed to `Int[10, 29]` generates with checked bounds
    `[10, 29]`, `[0, 9]`, `[10, 20]` and derives the descriptor over `[10, 29]`, a
    descriptor over `[0, 9]` is `BoundMismatch` against `[10, 29]`, and `-z` over
    `Int[0, 9]` narrowed to `Int[-9, 0]` checks `[-9, 0]` then `[0, 9]`.'
- id: FR-014-AC-28
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The referenced node''s body, not its form or type, makes an operand a
    literal: `x + n`, where `n` is a plain-Integer-typed `value` node of form `literal`
    whose body is not a literal, is refused as `RequiresBound` naming Integer, in
    generation and in derivation.'
- id: FR-014-AC-29
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`x + p` narrowed to `Int[0, 29]`, where `p` is a parameter typed by the
    plain Integer type, is refused as `RequiresBound` naming Integer, in generation
    and in derivation.'
- id: FR-014-AC-30
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: '`q + q` narrowed to `Int[0, 29]`, where no operand owns a bound and `q`''s
    node reaches `[0, 29]`, is refused as `RequiresBound` naming Integer, in generation
    and in derivation: the narrowing bounds the result as a `bounded_domain` result
    type does.'
- id: FR-014-AC-31
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A literal operand of an integer operation that is not a canonical `integer`
    literal is refused as `OperandTypeMismatch` naming its position and literal kind,
    whatever the referenced node''s type: `x + "a"` with the text literal in a plain-Integer-typed
    `value` node is refused at position 1 with kind `text`, in generation and in derivation.'
- id: FR-014-AC-32
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that narrowing conversions narrow to two distinct bounds (`[10,
    29]` and `[0, 40]`) is refused as `AmbiguousBound`, in generation and in derivation.
- id: FR-014-AC-33
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node narrowed to a `text_bounds` bound over Integer, or to an `integer_range`
    over Rational, is refused as `MissingBound` naming Integer and `integer_range`,
    in generation and in derivation.
- id: FR-014-AC-34
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A node that a narrowing conversion and a node that is not a narrowing conversion
    both consume (`x + 2` narrowed to `Int[0, 11]` and an operand of `(x + 2) + x`)
    is refused as `AmbiguousBound`, in generation and in derivation, while `x + 1`,
    consumed only by its narrowing, generates.
- id: FR-014-AC-35
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Boolean `and`, `or`, `not`, `implies`, `eq` and `ne` nodes and integer
    `eq`, `ne`, `lt`, `le`, `gt` and `ge` nodes each derive a descriptor and generate
    an `ir_confirmed` oracle; each connective's oracle calls `evaluate_boolean` or
    `evaluate_boolean_short_circuit`, and each `eq`/`ne` oracle calls `check_equality`
    and `CheckedEquality::evaluate`, each returning `Outcome<bool>`.
- id: FR-014-AC-36
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every oracle's source map declares the node's implication-consequent count,
    one evaluation-entry probe on the function-entry line disjoint from every consequent
    region, and one entry-token probe inside the exact region of each consequent;
    a dropped or duplicated region does not change the declared count.
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles
    against the runtime alone, and each oracle''s outcome, admitted charges and consumed
    counters equal direct runtime execution. This criterion claims no agreement with
    the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle
    against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming it.'
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
    When the condition is a reference to an admitted Boolean expression node, the
    oracle uses that node's expression, not the reference as a Boolean variable; an
    inline condition uses its own expression. In both cases the claim and generated
    oracle retain the clause node identity, not
- id: FR-014-AC-39
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'The generated `src/lib.rs` of every crate the TC-024 corpus generates,
    and the non-test code of `src/oracle/scalar/mod.rs` (comments and every `#[cfg(test)]`
    item removed wherever the item sits, string literals the generator emits counted),
    contain zero panic tokens: the identifiers `unwrap`, `expect`, `unwrap_unchecked`,
    `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind`
    however written (called, with whitespace before the paren, named on a path such
    as `Option::unwrap`, or imported); the macros `panic`, `unreachable`, `todo`,
    `unimplemented`, `assert`, `assert'
- id: FR-014-AC-40
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A package whose lowering returns `failed` for the `bytes` limit (Contract
    IR FR-038-AC-95) is refused per item as `ExactScalarRefusal::LoweringByteLimitExceeded
    { limit, consumed }` with `limit` and `consumed` equal to the record''s, never
    as `LoweringWorkExhausted`, and contributes no generated function. Through a whole
    call of `generate_exact_scalar_oracles` whose input is read under a byte ceiling
    that admits the checked package and is one byte below the canonical length of
    the lowered contract package of that call (the package `lower` returns for the
    call''s requested nodes, not the checked '
- id: FR-014-AC-41
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `failed` record whose `limit_kind` is `work` is still refused as `LoweringWorkExhausted`
    with the record''s `limit` and `consumed` (FR-014-AC-15 unchanged), and one whose
    `limit_kind` is neither `work` nor `bytes` (each of `nodes`, `edges`, `occurrences`
    and `diagnostics`) is refused as `ExactScalarRefusal::LoweringLimitUnrecognised`
    with `limit_kind` equal to that snake_case name and the record''s `limit` and
    `consumed`, without a panic and without a `LoweringWorkExhausted` or `LoweringByteLimitExceeded`.
    Each of the two is asserted on hand-built `Failed` records given to the scalar
    `Failed` '
- id: FR-014-AC-42
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`,
    `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one
    shared classifier, `classify_lowering_failure` of the shared `oracle` module:
    the non-test code of each of the three files, as FR-014-AC-39 defines it (comments
    and every `#[cfg(test)]` item removed wherever the item sits, taken with `non_test_code`
    and `comments_stripped` of `tests/common/panic_scan.rs`), names no `CheckedPackageLimit`
    and accesses no `.limit_kind`, has exactly one such direct-lowering `Failed` arm
    in `src/oracle/scalar/m'
- id: FR-014-AC-44
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: For each IR-admitted direct request naming a `temporal`/`formula`, `temporal`/`fairness`
    or `expression`/`case` node, the scalar claim map has exactly one entry with that
    requested node id and `ExactScalarRefusal::Unsupported`. For an IR `Unsupported`
    lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that
    record's first unsupported reachable node and tag, even if that node is not the
    request root; for a lowered `case` root they name the root and `expression`. A
    lowered `case` is refused as `Unsupported`, not `FormMismatch`. Each refused item
    has no generated function or
- id: FR-014-AC-45
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'For a QSL-shaped outer integer operation narrowed to an `integer_range`,
    a direct reference to a plain-Integer inner application over independently bounded
    operands supplies a conservative inclusive operand interval: `(x + 1) + 1` for
    `x: Int[0, 9]` supplies `[1, 10]` for its inner operand, and `-(x + 1)` supplies
    `[1, 10]` to the outer negate; `x * -1` for `x: Int[1, 9]` supplies `[-1, -1]`
    from a negate application over the canonical literal `1`. Each outer node derives
    and generates an `ir_confirmed` oracle, records the inferred range by argument
    position without inventing a checked bound i'
- id: FR-014-AC-46
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: An inferred interval uses only the four named integer operations and canonical
    literal or `integer_range`-typed leaves. A direct reference to a raw application
    consumed by a separate narrowing conversion never takes that conversion's domain
    as its operand interval; where the raw application has another consumer, requesting
    that application itself still yields FR-014-AC-34's `AmbiguousBound`. A plain-Integer
    parameter at any depth remains `RequiresBound` naming Integer. A missing, cyclic
    or unsupported application, wrong-kind or noncanonical literal, and an inference
    depth, work or endpoint-bit
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement
    corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and TC-193
    vectors, the FR-014-AC-37 corpus, the composite-equality corpus and the function-application
    corpus), no TC-024, TC-029 or TC-031 step and no oracle-matrix row for those cases
    or corpora claims, or presents as evidence, agreement with the QSL value authority;
    text that states this boundary is permitted.
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`,
    a unit-free variant: the target unit is a call-time argument `unit: &QuantityUnit`,
    and the generator does not check it against the `type_argument`, which the node
    carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with
    no checked bound, and the claim-map entry records the `type_argument` type node
    id (FR-014-'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When the runtime refuses a `quire.op.quantity.convert` with `IllTyped`
    before any charge, the oracle stops as `ill_typed` with zero admitted charges
    and no change to any consumed counter, equal to direct `convert_quantity` execution
    (Contract Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key),
    `1 N_m` to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts
    and is not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit
    is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar:
    its result is always a `Quantity` in the supplied target unit, and a conversion
    node whose `operation.mode` is not `exact` is refused with `ClaimDerivationRefusal::ModeNotDerivable`,
    checked first, so a scalar `type_argument` with a non-`exact` mode is `ModeNotDerivable`;
    one whose mode is `exact` and whose `type_argument` is a scalar type (`Decimal`
    or `Integer`, QSpec TC-187 U07, U08, U16, U17, U18 and U29) or a compound unit
    (U25''s first step; IR-831) is refused with `OperandFormsNotDerivable`, each with
    no descriptor, no'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269), because the Kani scalar renderer builds no quantity harness; the conversion
    is credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-018-AC-1
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every requested item receives exactly one generated or typed-refused disposition;
    a refused item contributes no function, no environment constructor and no symbol
    to the generated crate, while its siblings are generated unchanged.
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records the descriptor the request
    supplied — operator, both operand source types and both conversion targets — and
    it is equal to the request's, so the independent native run of AC-2 is driven
    from the request and never read back out of the generated crate. An authentic
    asymmetric source control preserves the actual bounded-left/unbounded-Integer-right
    source IDs and independent caller target; a source conversion erased from the
    emitted graph is not evidence of a retained conversion-reference branch.
- id: FR-018-AC-4
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records an `EqualitySchedule` equal
    to `CheckedEquality::schedule()` of the `CheckedEquality` its descriptor admits,
    for a top-level text, enum and quantity pair and for each `Plan`-schedule composite
    and collection shape.
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A descriptor with a `convert<T>` operand outside `admits_equality_conversion`,
    distinct text profiles, distinct enum declarations, incompatible dimensions, distinct
    units, or no common type is refused at generation time with its `IllTypedCause`,
    emits no code, and admits no charge on any `Meter`.
- id: FR-018-AC-6
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: An operand type bearing an IEEE value at any depth — including a `float64`
    field of a record nested inside a `sequence` — is refused as `OperatorIneligible`,
    and the refusal agrees with `TypeEnvironment::contains_ieee` on the reconstructed
    type at every depth in the corpus.
- id: FR-018-AC-7
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: On the FR-330 package path, model, function, state and temporal roots outside
    the equality lowering profile retain their exact typed `Unsupported` feature-accounting
    record and source occurrences in `PackageAccounting`; each has its own requested
    key and node tag, no symbol or oracle source, and does not change an independent
    admitted sibling. A `reference` composite rejected by Contract IR's strict reader
    is a reader-stage `InvalidPackage/InvalidValue` control, not a CG blocker. A lowered
    expression of form `call` is refused as blocked on quire-contract-runtime#34 with
    no oracle.
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes and a duplicate
    record field; and each generated oracle calls `TypeEnvironment::check_type` on
    both operand comparison types before `check_equality`, so calling it with an empty
    environment, one omitting a key its operand types reach, or one declaring that
    closure under a different key, yields `Outcome::Refused(Refusal::CheckedInvariant)`
    with no charge admitted, rather than a panic or a completed Boolean.
- id: FR-018-AC-9
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Every generated oracle function returns `Outcome<bool>`, never `bool`:
    with a denial injected at each of `equality.plan-form`, `equality.plan`, `equality.pair`,
    `equality.result-retain` and each conversion charge point in turn, the oracle
    yields `Outcome::Incomplete` naming that point, the denied charge is not applied
    — every counter equals those of the same run stopped immediately before that point
    — and never a completed Boolean.'
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order, including a request carrying two descriptors over one node
    id, and claim-map entries are ordered by the descriptor key — expression node
    id, operator rank, then each operand's source-type and conversion-target node
    ids, every node id compared in node-id order; the generated crate is compiled
    and executed under AC-2 against a native run driven from the request's own descriptor
    (AC-3), so a generated crate with a changed emitted operator, operand order or
    descriptor fails AC-2.
- id: FR-018-AC-11
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim marks its operation `caller_declared`, the claim map carries
    the blocked item "operation identity not consumed by codegen's generators", and
    two items over one node differing only in `EqualityOperator` both generate with
    that mark, under distinct symbols, each built from its own operator's stem, and
    produce complementary outcomes on a vector whose operands differ.
- id: FR-018-AC-12
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names direct exact/QSV
    and root replay dependencies through the one manifest authority, contains no charge
    amount and no planned pair count (every charge and every pair comes from runtime
    metering), and compiles.
- id: FR-018-AC-13
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Every claim-map entry carries the node id, IR id, package id, source map,
    claims, reconstructed declaration keys and selected schedule of its item, and
    its genuine owner-issued declaration keys equal the exact domain and digest of
    the V2 node ids its operand types reach, selected through owning lookup without
    CG/emitted key construction.
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact
    named bound values FR-014 specifies. For `integer_range` and `collection_bounds`,
    Contract IR FR-038 admission requires exactly `min` then `max` bindings with their
    canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted
    binding control is an IR-intake refusal before CG, with no admitted package or
    public CG `UnreadableBound` result. CG private parser tests retain their precise
    malformed-bound defenses separately and do not count as public-generator acceptance
    for these reader-ineligible inputs
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested conversions to the first node that is not a conversion, and a reference
    to any other node, including a non-`convert` application, as that node''s own
    `semantic_type`. A descriptor whose `source_type` is the read type generates,
    and any other source is refused as an operand-type disagreement at that position,
    reporting the read type as found. A genuine source fixture with no emitted convert
    node exercises its actual parameter source types'
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A tuple position whose type node is a `text_bounds` `bounded_domain` node
    generates the declaration `Text(min, max, profile)` read from that node''s own
    members, an integer, decimal or rational `bounded_domain` member reads as the
    same type as a member naming its base scalar, and a second bound over the same
    base is never read: naming either of two `text_bounds` nodes over one text scalar
    reads that node''s own `min` and `max`. A `bounded_domain` member whose form does
    not fit its base scalar is refused as missing the form the base reads, and one
    over a boolean scalar, over QSL''s enum declaratio'
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(`
    anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`,
    and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`,
    `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into
    exactly the mat
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every
    `#[cfg(test)]` item removed wherever the item sits, string literals the generator
    emits counted) contains zero panic tokens: the identifiers `unwrap`, `expect`,
    `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any`
    and `resume_unwind` however written (called, with whitespace before the paren,
    named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`,
    `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq`
    and `debug_assert_ne'
- id: FR-018-AC-20
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: When an admitted FR-330 package contains a `Failed` feature-accounting
    record, its equality claim is refused as `CompositeEqualityRefusal::PackageAccounting`
    with the exact `limit_kind`, `limit`, `consumed` and source occurrences from that
    record; it is never relabelled as work exhaustion. When a checked package is admitted
    under a byte ceiling one byte below its otherwise canonical lowered contract package
    length, Contract IR FR-038-AC-95 returns `Failed(Bytes)` for every requested root
    with that ceiling as `limit` and one shared `consumed` above it, and exposes neither
    canonical package byte
- id: FR-018-AC-21
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each generated item's claim-map entry records its declaration closure as
    every declaration's key, form (record, tuple or option), members in declaration
    order with each member's name, presence and type, and each leaf's family and declared
    bound, and the closure equals member by member (name, presence, value type, order)
    the declarations of the `TypeEnvironment` the item's generated environment constructor
    returns, for every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers. PLANNED (IR-264).
- id: FR-018-AC-22
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16:
    every case where the domain fits the case cap, the boundary and seeded cases otherwise;
    every presence state of an option and of an optional field included) of an item
    whose leaves are Boolean or bounded integers, the generated oracle called with
    its own environment and a `Meter` whose every limit is `u64::MAX` returns `Outcome::Completed`
    with the verdict of QSpec FR-149 for the item''s operator and admits exactly `equality.plan-form`,
    `equality.plan`, one `equality.pair` per node of the occurrence-pair tree, then '
- id: FR-018-AC-23
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers, the claim-map closure's member names, presence and each integer leaf's
    inclusive bounds equal those of an independent read of the checked package (the
    V2 composite and `bounded_domain` nodes read through Contract IR's reader, never
    through the generator's reconstruction); a closure that dropped a member, narrowed
    a bound by one or read an optional member as required fails the check. PLANNED
    (IR-264).
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Type reconstruction of QSpec's `positive-recursive-records.json` List (through
    `Option<List>`) and Tree (through `Sequence<Tree>` bounded `[0,3]`) closes each
    back-edge at the in-progress record key, generates equality items built over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence`
    self-cycle each refuse as `TypeResolutionCycle` naming the repeated type node,
    without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`,
    with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`;
    removing or renaming `optional`, adding a second nested member, or targeting a
    non-option node refuses only that item as malformed, never as a required field
    or a generated oracle. PLANNED (IR-643).
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds`
    follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element
    reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that
    bound node's named `min` and `max`; changing the semantic type to a non-sequence,
    dropping or duplicating either bound, or making either value non-integer refuses
    only that item with a typed cause and emits no code, never silently yielding an
    unbounded collection. PLANNED (IR-643).
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-29
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A cycle detected after a charged repeated entry reports `TypeResolutionCycle`
    without following a child; if that repeated entry is the first denied work entry,
    `TypeResolutionWorkExhausted` wins and reports the effective limit and first denied
    count. An unaffected sibling keeps its original disposition in either case. PLANNED
    (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
- id: FR-018-AC-31
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A caller-selected type-resolution work limit of `u64::MAX` refuses as `InvalidTypeResolutionWorkLimit`
    before lowering or charging; with `u64::MAX - 1`, the first denied count fits
    in the `u64` `consumed` field and is reported without wraparound. PLANNED (IR-511).
- id: FR-018-AC-32
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Each IR-admitted direct `temporal`/`formula`, `temporal`/`fairness` and
    `expression`/`case` request has one claim-map entry keyed by its requested node
    id and refused as `CompositeEqualityRefusal::Unsupported`. For an IR `Unsupported`
    lowering record, the refusal's `unsupported_node_id` and `node_tag` equal that
    record's first unsupported reachable node and tag, which need not be the requested
    root; for a lowered `case` root they name the root and `"expression"`. The request
    contributes no generated symbol or source, never panics or disappears, and leaves
    a supported sibling's claim and source
- id: FR-018-AC-33
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted request limits has equal compiled/context/original package
    identities and exact full recompiled/original CheckedPackageV2 equality before
    its context is exposed. An explicitly supplied different context package refuses
    generation and emitted environment construction before classification or lookup,
    including primitive-only requests, publishes no prepared value, and cannot evaluate
    with a completed Boolean or meter charge.
- id: FR-018-AC-34
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons
    over all declared cases, including a case absent from every selected literal and
    an enum parameter with no member literals. Exact shape variants and canonical
    positions come from genuine facade/QSV values. Same-spelled cases from distinct
    declarations retain distinct identities; matching and mismatching declaration
    comparisons preserve the owning equality disposition.
- id: FR-018-AC-35
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the original typed Unknown/Foreign details at consumed and
    generated lookup boundaries. Malformed source/dependency input, QSL stage or above-reader
    resource limits, and internal Fault remain Package, Resource and Fault at the
    source-retaining boundary; a subsequent IR read stop remains the separate PackageReadLimit,
    with no partial context or successful prepared wrapper. The current ImportedContextUnsupported
    guard remains until original admitted dependency packages are retained.
- id: FR-018-AC-36
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and
    contains no NodeKey constructor/decoder or generated nominal preimage/hash. Only
    the prepared wrapper's private genuine index reaches check_equality. A private
    environment-only fault fixture still satisfies AC-8, every original charge-stop
    case still satisfies AC-9, and a wrong item/descriptor prepared wrapper refuses
    before charging. Record/tuple/recursive/collection cases remain required by AC-2
    and AC-21 through AC-30; an absent owner lookup capability is recorded as unresolved
    migration work, not as accepted Unsupporte
- id: FR-018-AC-37
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Equality's generated manifest and native scratch consumers use the one
    profile authority's direct exact/QSV/replay dependencies, while ordinary Boolean/integer
    FR-014 scalar proof manifests and oracle bodies acquire no replay source-compilation/context
    leg. Native generated enum acceptance is reported separately from original TC025/TC027
    proof qualification; neither result substitutes for the other.
- id: FR-018-AC-38
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nominal primitive and nested Option/Collection closures; their
    index is empty only after complete zero-enum resolution. Existing admitted-IR
    conversion-form and quantity-family negatives retain respectively exact FormMismatch("conversion")
    and Unsupported("quantity"), with no replacement ContextPackageMismatch or MissingAdmittedContext,
    no symbols for the refused item and unchanged independent siblings. This does
    not claim matching QSL source emission or source authentication for those IR fixtures.
- id: FR-018-AC-39
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declaration. Cover left and right source operands, left and right
    conversion targets from primitive sources, nested Option and Collection enum types,
    and eventual record/tuple field and recursive closures through their genuine owner
    capability. No key, default/partial index, omitted nominal declaration or Unsupported
    substitute is produced. Retain the existing charged-entry/cycle/error precedence
    and unaffected sibling results; the separate missing record/tuple owner lookup
    remains outstanding.'
- id: FR-018-AC-40
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Some(wrong-package context) fails the whole standalone call after existing
    setting/accounting checks and before item classification, for primitive-only,
    nominal and empty requests. Emitted context-free and nominal constructors both
    reject the same supplied mismatch before reconstruction. None and Some(wrong)
    have distinct outcomes; no failure publishes partial artifacts or a prepared wrapper.
- id: FR-018-AC-41
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A reached enum still reconstructs every owner-admitted case, including
    unused cases and an enum parameter with no member literal, using the supplied
    genuine keys and canonical positions. Failed nominal admission/lookup/traversal
    cannot yield an empty-index success. A successfully resolved zero-enum closure
    may use an empty index; merely inspecting an outer primitive/Option/Collection
    tag does not establish zero enums.
- id: FR-018-AC-42
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/descriptor/index binding, both check_type guards, same-index
    check_equality and evaluate. Private wrong-binding, present-context mismatch,
    missing nominal context and environment-only faults refuse without charge. The
    native OriginalCompositeEqContext route retains every mandatory authentic source/lock/package/whole-IR
    comparison and cannot use None to bypass them. No scalar proof manifest/body acquires
    replay compilation.
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_range and the right parameter names canonical base
    Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded
    Integer despite the sibling range: a caller descriptor converting the left to
    Integer compares with the unbounded right under the original equality rule, including
    right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing
    conversion by borrowing that sibling interval. Claim/native descriptor checks
    remain independent of generated output. If so'
- id: FR-021-AC-1
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Every requested `call` item receives exactly one scheduled or typed-refused
    disposition; a refused item contributes no instruction root, concrete symbol or
    location entry, while an unrelated admitted item is unchanged. Concrete generated
    disposition is outside CG's boundary (IR-845).
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'For each function-application corpus vector, the oracle''s outcome, admitted
    charge sequence and consumed counters equal a direct application independently
    assembled from the request with a fresh meter; today that application is the Contract
    Runtime `exact` port (historical), and CG names no QSL application API as its
    replacement (IR-845). This criterion claims no agreement with the QSL value authority:
    that boundary is owned by quire-integration IR-842 (FR-014-AC-47, FR-021-AC-18).
    PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes the `exact` port,
    this criterion''s comparison ta'
- id: FR-021-AC-3
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled item records the requested function name and declaring V2
    function node id, plus its request-order function index; any independent application
    a test builds for comparison is constructed from the request, never read back
    from generated code. The origin mapping to a QSL-owned API is outside CG's boundary
    (IR-845).
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The oracle returns the arity, wrong-value-kind, dangling-reference and
    unknown-function input refusals unchanged to the caller before any execution charge;
    the schedule records ordered arguments without inventing a refusal result. That
    the refusals equal QSL-owned ones is outside CG''s boundary (IR-842, FR-014-AC-47).
    PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes the `exact` port,
    the refusal comparison''s target is decided by IR-853.'
- id: FR-021-AC-5
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The schedule orders arity validation before per-argument validation, validates
    arguments in declared parameter order, then places `function.call` before body
    work; owner-path execution of that order is outside CG''s boundary (IR-845). The
    corpus half is PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes
    the `exact` port, its target is decided by IR-853.'
- id: FR-021-AC-6
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A function whose declared operator requirements name a capability no registered
    backend can discharge is marked `unsupported` at generation time, naming the capability,
    before any item naming it is applied; no oracle is emitted for such an item, and
    the disposition is never an `Outcome` variant, never an `InputRefusal`, and takes
    no `Meter`.
- id: FR-021-AC-7
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A tree-shaped 100,000-node V2 `expression`/`call` graph whose nested children
    are separate admitted nodes reached by `reference` produces a flat SSA instruction
    table on a 512 KiB stack when caller work and source-byte limits fit it; no CG
    analysis or emission depth cap participates. The schedule crate compiles. Running
    a concrete oracle against a QSL evaluator is outside CG's boundary (IR-845); no
    Contract Runtime `MAX_CALL_DEPTH` behavior is required.
- id: FR-021-AC-8
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule marks only linked-package admission as applicable and never
    marks a kernel-mode package applicable. Concrete mode admission through a QSL
    application API is outside CG's boundary (IR-845).
- id: FR-021-AC-9
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The schedule preserves a `function.call` charge point and an incomplete
    stop before body work, and the denied charge is not applied; owner-path execution
    of it is outside CG''s boundary (IR-845). The agreement vectors are PLANNED, blocked
    on IR-853: once Contract Runtime FR-275 deletes the `exact` port, their target
    is decided by IR-853.'
- id: FR-021-AC-10
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A declared parameter or result type reaching a `reference` composite form
    is refused as blocked on quire-spec-language#120, with no schedule root or concrete
    code for that function. No meter charge is admitted for it; a runtime charge assertion
    against a QSL API is outside CG's boundary (IR-845).
- id: FR-021-AC-11
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Model and relation lowering records map to the quire-spec-language#120
    blocker; state, temporal nodes other than directly requested `formula` and `fairness`
    roots, and protocol lowering records map to quire-spec-language#121; the blockers
    remain distinct. A requested temporal clause keeps that blocker even if its first
    unsupported reachable node is a formula or fairness child. TC-031''s defensive
    Model/State classifier case pairs those request roots with call items bound to
    a different checked source function, so it does not establish applied-call behavior.
    The source-bound applied-call leg is '
- id: FR-021-AC-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A declared function whose own body fails to lower — an unlowerable node,
    a form none of this generator's classifiers admits, or a nested `call` naming
    a function absent from the request — refuses every item bound to that function
    with a typed reason, without changing an unrelated package's items.
- id: FR-021-AC-13
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Schedule source and claim-map bytes are identical across runs and request
    permutations; entries order by call-node id, declaring function-node id, then
    source argument-node ids, with the name tie-break AC-24 specifies.
- id: FR-021-AC-14
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The schedule crate declares `publish = false`, compiles without a Contract
    Runtime `exact` dependency, contains no charge amount or literal outcome/value
    standing in for owner execution, and stays within caller-selected source bytes
    and publication artifact limits. Concrete owner-path emission is outside CG's
    boundary (IR-845).
- id: FR-021-AC-15
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled call point has one constant-size location record with function
    origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction
    slot. A charged query reconstructs the same path as an independent walk of the
    request V2 reference graph; no record embeds the entire root-to-point path. A
    100,000-link chain uses O(100,000) map storage and generation work.
- id: FR-021-AC-16
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The schedule and its claim/location maps neither read nor depend on dynamic
    invocation `location` or `losses` fields. No oracle makes a claim that those fields
    are populated: CG claims no QSL application API (IR-845).'
- id: FR-021-AC-17
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Each location record uses the request function binding and function-body
    ordering for its origin; the cross-check against an origin published by a QSL-owned
    checker is outside CG''s boundary (IR-845). The runtime check is PLANNED, blocked
    on IR-853: once Contract Runtime FR-275 deletes the `exact` port, its target is
    decided by IR-853.'
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the function-application
    corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    agreement in outcome, refusal, charge order or consumed counters with the QSL-owned
    application authority; text that states this boundary is permitted. That agreement
    is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- id: FR-021-AC-19
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The emitted schedule crate for scalar, equality and nested-call corpus
    cases contains zero `.unwrap(`, `.expect(`, `unreachable!`, `panic!`, `todo!`
    and `unimplemented!` tokens; no concrete owner-path oracle exists to check (IR-845).
- id: FR-021-AC-20
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For each scalar and equality instruction, the target-neutral schedule preserves
    completed, undefined, refused, incomplete and checked-invariant result distinctions
    without folding one into another. Forwarding a QSL application API's owner result
    variants is outside CG's boundary (IR-845).
- id: FR-021-AC-21
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'A scalar `Negate` body is refused as `UnsupportedOperator` with no schedule
    root, whether or not it reaches a `quire.op.quantity.convert` node (a conversion
    is not refused by name, because FR-021 has no quantity body, operand or argument
    support: the generator refuses a body that is one as `UnsupportedOperator` and
    the schedules as `ScheduleRefusal::UnsupportedOperation`; an earlier check that
    fires first is the accepted result: `Unsupported` for a direct unsupported form
    of the root, `NotExpression` and the other root checks, `UnsupportedOperandType`
    for a quantity-typed parameter or result, '
- id: FR-021-AC-22
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'When two or more declarations share one declaring node id, none of them
    appears in the instruction table or in `location-map.json`, and every item naming
    one of them is refused with `ExactFunctionRefusal::DuplicateDeclaringNode { node_id
    }` (the shared node id; the smallest in node-id order when the item''s name is
    held by duplicate groups on several node ids) before Stage 1 classification: never
    `UnknownFunction`, never `AmbiguousFunctionName` (the node-id refusal takes precedence
    when the declarations also share a name), never `DuplicateRequest` (two items
    on one `call` node naming different '
- id: FR-021-AC-23
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: For an admitted FR-330 package carrying a `Failed` feature-accounting record,
    a function item retains that exact typed record, its `limit_kind`, `limit`, `consumed`
    and source occurrences in `PackageAccounting`, without relabelling bytes as work
    exhaustion. Under a ceiling one byte below the shorter of the separately measured
    body-root and call-root package lengths, Contract IR FR-038-AC-95 returns `Failed(Bytes)`
    for every root in each request, one shared `consumed` per request above the ceiling,
    and neither request exposes canonical package bytes or `wire()`. The two measured
    `consumed` valu
- id: FR-021-AC-24
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Items on one `call` node, with equal argument operands, that name functions
    absent from the request''s declarations each get their own claim-map entry. Requesting
    `zz_unknown` and `aa_unknown` on one node yields two entries, in that order: `UnknownFunction
    { name: "aa_unknown" }`, then `UnknownFunction { name: "zz_unknown" }` (function
    name compared byte-wise over its UTF-8 bytes, case-sensitively), and no `DuplicateRequest`;
    the entries are identical under both request orders. Requesting `Zz_unknown` and
    `aa_unknown` yields `Zz_unknown` first, because `Z` (0x5A) sorts before `a` (0x61).
    Reques'
- id: FR-021-AC-25
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Across one body, the counter charges each node entry, child edge, emitted
    instruction and location record once in source traversal order. A setting equal
    to the required units emits a complete schedule; one less refuses that function
    as `GenerationWorkExhausted { limit, consumed: limit + 1 }` before the denied
    action, with no partial root or location record and no change to an unrelated
    function. A failed Contract IR lowering record keeps its lowering refusal and
    incurs no generation charge. PLANNED (IR-511).'
- id: FR-021-AC-26
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The target-neutral schedule's branch/stop edges order a denied left-operand
    charge to stop before the right operand charge, and, after the left completes,
    the right charge before the parent operation. Owner-path execution matching an
    equivalent shallow body is outside CG's boundary (IR-845).
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected
    only by `reference` leaves under QSpec FR-322, with each expression node reached
    from one parent in this CG subset; callee argument 0 binds an exact V2 function
    node, parameter references bind the declared parameter slots, and remaining arguments
    preserve source order. Missing, cyclic, shared-expression, unbound-parameter,
    ill-typed or wrong-operation edges refuse with a typed cause before schedule emission.
    PLANNED (IR-511).
- id: FR-021-AC-28
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Under the default 1,048,576-byte source limit, a 100,000-instruction body
    that needs more bytes returns `SourceTooLarge` naming the default; raising the
    caller limit to its measured flat source bytes emits a complete compilable schedule
    crate, with deterministic ordinal-range modules if one artifact would exceed its
    publication limit. One byte below the measured count refuses before append with
    no partial output. PLANNED (IR-511).
- id: FR-021-AC-29
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A 100,000-call linear body produces 100,000 constant-size parent-linked
    location records; requesting its deepest path under sufficient query limits returns
    exactly 100,000 child ordinals, and a one-unit-below query limit refuses without
    a partial path. Generation does not materialize all paths. PLANNED (IR-511).
- id: FR-021-AC-30
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit`
    before lowering or charging; at `u64::MAX - 1` the first denied count is representable
    in the `u64` refusal and never wraps or panics. PLANNED (IR-511).
- id: FR-021-AC-31
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the 100,000-node
    case, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    that a generated oracle compiled, ran on a 512 KiB stack or agreed with a shallow
    equivalent against a QSL application API; schedule-source compilation (AC-7) is
    not execution evidence. CG claims no QSL application API (IR-845, FR-014-AC-47).
- id: FR-021-AC-32
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Neither the schedule crate nor any concrete oracle requires `quire_contract_runtime::exact`,
    an alias, wrapper or compatibility layer; CG emits no owner-path oracle (IR-845).
    PLANNED, blocked on IR-853: Contract Runtime FR-275 expects the generator to name
    QSL''s evaluation leaf, which conflicts with IR-845 and QSL arch-lint T12-A; IR-853
    resolves that conflict and decides who closes this criterion once the `exact`
    port is deleted.'
- id: FR-021-AC-33
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'With an IR-admitted `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case`
    node used directly as an item''s call-node id, the claim-map entry retains that
    id and is `Refused { refusal: ExactFunctionRefusal::Unsupported }`. For the separate
    declared-body-root case, an item names that function through a distinct, IR-admitted
    `expression`/`call` node that lowers successfully and has no duplicate, capability,
    or earlier item refusal; its claim retains the call-node id and carries the body''s
    `ExactFunctionRefusal::Unsupported`. For an IR `Unsupported` lowering record the
    refusal''s `unsup'
- id: FR-021-AC-34
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Given one authentic strict-reader-admitted `CheckedPackageV2` of less than
    1,048,576 bytes containing 129 distinct functions with two bound Integer parameters
    each, 128 acyclic `quire.op.function.call` body-root edges forming one chain and
    a terminating admitted Scalar body, the public function generator schedules all
    129 functions and all 128 call edges from that same checked graph. Use the default
    1,048,576-byte Contract IR reader/lowering ceilings and default 1,048,576-byte
    generated-source ceiling; only the caller-selected generation work limit may be
    raised to the measured required units.
- id: FR-031-AC-1
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of Add, Subtract and Multiply over a `reject`
    integer type, the generated source of the native oracle calls `exact::evaluate_integer_arithmetic`
    with the matching `IntegerArithmetic` variant and the type's interval as the bound,
    and the oracle's body holds no Rust arithmetic operator between operand expressions
    and none of `checked_`, `wrapping_`, `saturating_` or `overflowing_` method calls.
- id: FR-031-AC-2
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder
    over a `reject` integer type, the generated source of the native oracle calls
    `exact::divide` with `DivisionProfile::Truncating`, with `DivisionMember::Quotient`
    for a divide and `DivisionMember::Remainder` for a remainder, and with `IntegerDomain::Bounded`
    over the node's interval as the domain, and the oracle's body holds no Rust `/`
    or `%` and none of `checked_div`, `checked_rem`, `wrapping_rem`, `div_euclid`,
    `rem_euclid` or `modulo`, and no division of a quotient and remainder pair.
- id: FR-031-AC-3
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns `Outcome<bool>`
    and takes a trailing `&mut Meter`; an oracle with none returns `bool` and takes
    no meter.
- id: FR-031-AC-4
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free
    operands is emitted as the one native operator the table names, and over the seven
    values `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`,
    with two interior values added, taken pairwise in both orders, agrees with the
    exact kernel's own evaluation, as the test takes it from Contract Runtime today
    (`order_numbers` for the four orderings, `check_equality` then `CheckedEquality::evaluate`
    for equal and not-equal; after the migration of FR-031-AC-22, `quire_exact`'s
    `order_numbers` and `planned_equ
- id: FR-031-AC-5
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract
    or multiply result outside the type''s interval evaluates to `Refused(IntegerOutOfDomain)`,
    and the oracle neither panics nor returns a wrapped, saturated or clamped value.
    The cases are O-1 `i64::MAX + 1`, O-2 `i64::MIN - 1` as a subtract, and O-4 `i64::MIN
    * 2`, each built as TC-044 step 4 gives. On the tree before the change each fails:
    a debug build panics with an overflow message, a release build returns a wrapped
    value, and no case returns the refusal.'
- id: FR-031-AC-6
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type, a zero divisor evaluates to `Undefined(DivisionByZero)` for both Divide
    and Remainder, for the cases Z-1 `x / 0`, Z-2 `0 / 0`, Z-3 `x % 0` and Z-4 `0
    % 0`, each built as TC-044 step 5 gives (Z-3 and Z-4 reach the zero through the
    add `y + 11`, which completes with 0), and the oracle does not panic and does
    not return `Refused`. On the tree before the change each case is refused at generation
    with the interim code, and a raw operator would panic with a division-by-zero
    message.
- id: FR-031-AC-7
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer
    type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with
    no artifact, for the rows `x + 1` at `x = i64::MAX` and `x - 1` at `x = i64::MIN`
    over an integer type of `i64::MIN..=i64::MAX`, and `x * 2` at `x = i64::MAX` over
    `0..=10` (where an in-domain `x = 10` also gives the wrong value `20 <= 10` against
    the policy's `10 <= 10`); the refusal span is the first such node in authored
    preorder when a negation also occurs later, and the diagnostic message names the
    missing runtime saturating operation
- id: FR-031-AC-8
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). In a generated crate compiled and executed against
    `quire-contract-runtime`, the outcome of a native add, subtract or multiply oracle
    equals a plain-integer model of IR FR-015''s member-range semantics, computed
    in the test with `i128` independently of the emitter and of any `exact::` call:
    `Completed` with the exact result when it lies in the type''s interval and `Refused(IntegerOutOfDomain)`
    when it does not. The grid covers operand values inside the declared domain, at
    its edges and outside it, and the outcome kind is compared, so no vector returns
    `Completed` where the '
- id: FR-031-AC-9
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order
    is the oracle''s outcome: an arithmetic stop in the left operand of a short-circuit
    connective is returned, a stop in a right operand the left decided is never reached,
    a total connective returns the first stop of its two operands, and a stop is never
    returned as `Completed(true)` or `Completed(false)`.'
- id: FR-031-AC-10
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The tri-state harness generator, the bound strategy
    generators and the Kani obligation clause lowering each refuse a clause holding
    an arithmetic node with the refusal the Consumers section names for it, emit no
    raw arithmetic operator, and never read an `Outcome<bool>` as `bool`.
- id: FR-031-AC-11
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause
    that runs QSL's IT-011 and IT-010-SC-05 against this branch is pending the quire-integration
    run; CG's in-repo run of the same bundle shape under real Kani is the closest
    equivalent and does not stand for it. `generate_kani_bundle` over the postcondition
    `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000`
    under `reject`, generates a bundle whose postcondition oracle returns `bool`,
    holds exactly one syntactic addition (`syn::ExprBinary` with `BinOp::Add`) and
    no `exact::` call, and, run wit
- id: FR-031-AC-12
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Apart from the generated Rust header lines required
    by NFR-002, the oracle generated for an expression with no arithmetic node is
    byte-identical to the oracle the generator produced for it before this requirement.
    This criterion passes on the tree before the change and is a held agreement, not
    a defect proof.
- id: FR-031-AC-13
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic
    node twice, and from a permuted request, yields identical bytes. This criterion
    passes on the tree before the change and is a held agreement, not a defect proof.
- id: FR-031-AC-14
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic
    node contains no `unwrap`, `expect` or panic macro. This criterion passes on the
    tree before the change and is a held agreement, not a defect proof.
- id: FR-031-AC-15
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons
    that has an arithmetic operand is emitted as the exact-kernel call the table names
    (for Equal and NotEqual, until the migration of FR-031-AC-22, the checked equality
    of Contract Runtime that the Comparison section records), with no Rust comparison
    operator between its two operands.
- id: FR-031-AC-16
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type of `-5..=5`, the minimum divided by negative one is out of domain and the
    minimum remainder negative one is not, and neither panics: O-3 `i64::MIN / -1`
    evaluates to `Refused(DivisionOutOfDomain)` whose member is `Quotient` and whose
    domain is `-5..=5`, with the refusal''s code `division_out_of_domain` and cause
    `quotient-outside-domain`; O-5 `i64::MIN % -1` evaluates to `Completed(true)`,
    the remainder being the exact 0 and `0 <= 5` true. Each is built as TC-044 step
    4 gives. On the tree before the change each cons'
- id: FR-031-AC-17
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate,
    the outcome of a divide or remainder oracle equals a plain-integer model of IR''s
    member-only semantics under the truncating law, computed with `i128` independently
    of the emitter and of any `exact::` call (zero divisor: `Undefined(DivisionByZero)`;
    the selected member in the type''s interval: `Completed` with the comparison''s
    Boolean over that member; otherwise `Refused(DivisionOutOfDomain)` with that member),
    on a grid of dividends and divisors inside the declared domain, at its edges and
    outside it, that includes the two '
- id: FR-031-AC-18
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). A divide or remainder node over a `saturate` integer
    type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with
    no artifact and no raw `/` or `%` in any output, for the `saturate` rows S-1 to
    S-4 of TC-044 step 11, in both consumers; the diagnostic message names the missing
    saturating operation and the refusal has terminal state `unsupported`. On the
    tree before the change each row is refused with `UnsupportedIntegerDivision` instead.
    This criterion calls no kernel and can land before the lock prerequisite, while
    the interim code still exists for a divide
- id: FR-031-AC-27
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before
    the native division of AC-2 exists). The code `UnsupportedIntegerDivision` does
    not exist: the crate''s `GenerationErrorCode` has no such variant, nothing under
    `src/`, `tests/` or `schemas/` spells it, no stable code string of that name is
    emitted, and no diagnostic message that generation can emit names IR-601. On the
    tree before the change the variant exists and a divide or remainder over `reject`
    is refused with it.'
- id: FR-031-AC-28
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-664). The V1 Boolean oracle admits a typed IR integer domain
    or literal only when its value and both inclusive domain endpoints fit i64. A
    minimum of `i64::MIN - 1` or `i128::MIN`, or a maximum of `i64::MAX + 1` or `i128::MAX`,
    produces `UnsupportedExpression` at the first affected expression source span
    and no generated source; the same rule applies to an out-of-range literal. Values
    are never narrowed, clamped or wrapped to create an oracle.
- id: FR-031-AC-19
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose arithmetic
    nodes are add, subtract and multiply renders them as the infix `+`, `-` and `*`
    on `i64` with one binary operator per arithmetic node, contains no `exact::` call
    and no `checked_`, `wrapping_`, `saturating_` or `overflowing_` method (a remainder
    node's `wrapping_rem` is FR-031-AC-23's, not this criterion's), takes no meter,
    and its file states that its arithmetic is checked by Kani and is not a native
    evaluator.
- id: FR-031-AC-20
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle
    with unconstrained operands at which IR''s discharge does not hold fails with
    Kani''s arithmetic-overflow check and a counterexample: the oracle of `x < 5 &&
    x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained, fails
    the "attempt to multiply with overflow" check, and the counterexample''s playback
    value satisfies `x < 5` and overflows `x * 2` (any such value is valid; the assertion
    does not name one); a hand-written probe harness that assumes the declared domain
    on `x`, calls the same bundle oracle an'
- id: FR-031-AC-21
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool
    the Kani bundle oracle returns equals the `bool` inside `Completed` of the native
    oracle of the same typed expression, on every vector of a grid inside the declared
    domain at which the expression's guards pass and IR's discharge holds, including
    the domain's edges, with the bundle oracle compiled and run natively in a generated
    crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This
    is the differential that holds the two implementations of the rule in step.
- id: FR-031-AC-22
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle with an arithmetic node aliases `quire_exact`
    as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`, and contains no
    `quire_contract_runtime::exact` path; a generated crate that compiles and runs
    it, with divide and remainder nodes and with add, subtract and multiply nodes
    in the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one
    such oracle), resolves one `quire-exact`; CG''s own `Cargo.toml` depends on `quire-exact`
    directly from `agent-ix/quire-exact`, and '
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native oracle holds no `quire_contract_runtime::operators`
    path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`;
    a short-circuit connective is an inline `match` over the left operand's `rt::Outcome`
    that returns the first stop and does not evaluate the right operand when the left
    decides, and a total connective evaluates both operands in order and returns the
    first stop, none charging any point and none calling `evaluate_boolean`; in a
    generated crate the outcomes of FR-031-AC
- id: FR-031-AC-23
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject`
    type as the infix `/` and Remainder as `wrapping_rem` on `i64`, one operator or
    call per arithmetic node, contains no `exact::` call and no `modulo`, `div_euclid`,
    `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method, no `wrapping_`
    method other than `wrapping_rem` on a remainder node, takes no meter, returns
    `bool`, and its file states that its arithmetic is checked by Kani. `generate_kani_bundle`
    over a divide or a remainder over a `reject` type generates a bundle, and over
    a `saturate` divide or remainder it
- id: FR-031-AC-24
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle
    of the construction Z-1 (a divide) with unconstrained operands fails the checks
    "attempt to divide by zero" and "attempt to divide with overflow", each with a
    counterexample, and one that calls the bundle oracle of the construction Z-3 (a
    remainder) with unconstrained operands fails the check "attempt to calculate the
    remainder with a divisor of zero" and no other arithmetic check; a probe harness
    that assumes the declared domain and the guards, calls the same oracle and discards
    its value verifies, for the divide `10 / '
- id: FR-031-AC-25
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite (its native side calls
    the kernel). For divide and for remainder, the bool the Kani bundle oracle returns
    equals the `bool` inside `Completed` of the native oracle of the same typed expression,
    on every vector of a grid inside the declared domain at which the expression's
    guards pass and the divisor is non-zero and IR's discharge holds, including the
    domain's edges and R-1 and R-2, and including the remainder `y != 0 && x % y <=
    0` over `i64::MIN..=i64::MAX` at `x = i64::MIN`, `y = -1` (native `Completed(true)`,
    bundle `true`, where a raw `%` pa
- id: FR-035-AC-1
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A generated caller-ingress oracle accepts `&TextPayload`, the declared
    `TextType` and `&mut Meter`, calls `rt::admit_text` once, and returns the runtime
    outcome, retained text, original runtime provenance, charge sequence and consumed
    counters unchanged for all six profiles over representative empty, ASCII, combining,
    composed and supplementary payloads.
- id: FR-035-AC-2
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8`
    before admission and leave the admission meter unchanged; `admit_text` itself
    is never claimed to accept those bytes.
- id: FR-035-AC-3
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each profile, `Text[1,1]` admits or refuses according to the profile
    length of the retained sequence, including `e` plus U+0301 under NFC and NFD;
    a length refusal occurs before `text.result-retain`, and denial of each reached
    charge point yields the direct runtime stop, charges and consumed counters.
- id: FR-035-AC-4
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real
    Kani harness symbolically ranges over all 21 sequences of zero to two scalars
    from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares
    the generated admission oracle with a separate direct runtime call including outcome
    and metering, and ends with one satisfied cover after its assertions. The proof
    identity names this finite class and its type.
- id: FR-035-AC-5
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType`
    altered so the oracle accepts a payload the direct runtime call refuses, real
    Kani falsifies the admission agreement assertion with concrete playback; restoring
    the call verifies over the same symbolic class.
- id: FR-035-AC-6
  path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  excerpt: A request outside the declared finite proof class has a typed unsupported
    or requires-bound disposition and no harness, and no caller-ingress claim is marked
    `ir_confirmed` or assigned a checked-package node id. The six attempted text-operand
    `numeric.convert` nodes remain Contract IR refusals and do not count as passing
    caller-admission coverage.
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors
    use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from
    `check_type` and `check_equality`; downstream replay consumers preserve the typed
    payload or match it explicitly, retain charge/failure behavior, and accept neither
    the former unit constructor nor an invented default.
- id: FR-016-AC-1
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every retained witness decodes into typed values or is refused as malformed.
- id: FR-016-AC-2
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An out-of-domain witness is reported as such and never as a contract failure.
- id: FR-016-AC-3
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An in-domain witness is reported as a failure only when native replay reproduces
    the same typed outcome.
- id: FR-016-AC-4
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A native replay that runs and disagrees with the harness result yields
    a typed mismatch, and never a failure or a replay error.
- id: FR-016-AC-5
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness bound to a different harness identity is reported as malformed
    and never replayed.
- id: FR-016-AC-6
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every witness decode receives required caller-supplied byte and value-count
    bounds at its entry point; replay inputs carry both as non-optional fields with
    no default, and an over-bound witness is reported malformed without reading beyond
    either bound.
- id: FR-016-AC-7
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A native replay that matches the harness value but differs in admitted
    charges, consumed counters or limits yields a typed mismatch.
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness schema binds the obligation's persisted argument bindings, in
    the order the obligation persists them, position for position to the harness's
    symbolic arguments, and each decoded value is named by the binding at its position;
    a binding that is not an argument is refused with a typed schema refusal that
    reports no failure and is none of the replay results.
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate
    the clause to false settles as `reproduced-with-evaluated-witness` with category
    `violation`; the verdict depends on the witness value, so a witness at which the
    function holds does not settle so.
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause
    to true settles `inconclusive` with the proved violation and the replayed success
    both named.
- id: FR-016-AC-11
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The adapter refuses, with a distinct typed error each, a decoded value
    no replay parameter binds, a transcript field holding a delimiter, a transcript
    QSL does not admit, a request QSL refuses with any `ReplayRefusal` other than
    `Fault` (returned with its cause), and a witness-sourced request that settles
    on the input arm.
- id: FR-016-AC-12
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused`
    carrying that fault.
- id: FR-016-AC-13
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `Witness` arm that settles `reproduced-with-evaluated-witness` with a
    category other than `violation` yields a typed mismatch, and never a reproduced
    failure.
- id: FR-016-AC-14
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site`
    together with that dependency's lock source, and `qsl_replay::replay` settles
    the unit's falsification through the imported function.
- id: FR-016-AC-15
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The replay request's `package.dependencies` carries one entry per lock
    dependency selection, in ascending identity order, with the lock's recorded `package_id`.
- id: FR-016-AC-16
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A unit whose import no lock dependency selection supplies is refused at
    the call site.
- id: FR-016-AC-17
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock recording another `package_id` for a dependency the unit imports
    is refused by QSL as a dependency identity mismatch.
- id: FR-016-AC-18
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock selecting a library the unit does not import is refused by QSL as
    unselected.
- id: FR-016-AC-19
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Lock libraries whose sources share a source owner are refused as no dependency
    input, before the call site compiles anything.
- id: FR-016-AC-20
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock library whose source has the unit's own source owner is refused
    by the call site.
- id: FR-016-AC-21
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function path''s request `obligation_identity` holds the function-contract
    obligation digest of quire-spec-language ADR-013 O-09: the digest of the RFC 8785
    encoding, made by `quire-canonical` (ADR-013 §2''s one encoder, AD-003 E-1), of
    the `FunctionSite`''s `function` node id, its `declaration` occurrence key, the
    `ObligationKind` of the harness being replayed and `arguments`, each a parameter
    node id with its declared domain, ordered ascending by identifier as O-09 fixes
    and not in the declared order of `FunctionSite.parameters`. It is never the digest
    of the transcript. The `arguments` are'
- id: FR-016-AC-22
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'For two functions with the same parameters, the identities differ. The
    identity of one function is the same across recompiles that only add comments
    or blank lines, and across a change of the harness''s source span. It differs
    between two different `ObligationKind`s over the same function, and when any argument''s
    domain differs. A function yields one identity per existing `ObligationKind` it
    requests, not one per conjunct of its body: a function whose body has two conjuncts,
    replayed under one kind, has one identity.'
- id: FR-016-AC-23
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function-contract identity function derives nothing from the package:
    with the `FunctionSite`''s `function` and `declaration`, the kind and the arguments
    held fixed, perturbing every other node of the compiled package (an unrelated
    declaration, the order of `FunctionSite.parameters`) leaves the identity unchanged,
    and a perturbed `function` or `declaration` changes it.'
- id: FR-016-AC-24
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock whose only defect is one library identity selected twice is refused
    by QSL's `DependencyInput::new` as `DuplicateIdentity`, code `invalid_package`
    with cause `conflicting-definition`, and reaches the caller as `DependencyLockError::Input`.
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The envelope''s obligation identity is QSL''s `ObligationIdentity` built
    as the ADR-013 O-09 digest by the one scheme of FR-024-AC-20: the subject node
    id (function or state frame; a clause or application subject is not yet built),
    its occurrence key, the obligation kind and the arguments, each a parameter node
    id with its declared domain (empty for a frame, which has no parameters), and
    no `source_span`. Changing `source_span` leaves it unchanged, and changing the
    obligation kind or any argument binding changes it. The scheme''s spelling is
    CG''s own and carries no digest label (AD-003 E-1, open '
- id: FR-024-AC-2
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Every backend-witness transcript the generator passes to `Witness::parse`
    is produced by the one adapter rendering function from decoded values, and that
    function takes no backend-native text as input.
- id: FR-024-AC-3
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A counterexample with any value outside its declared domain is reported
    out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints
    of each declared domain are admitted, and the values immediately outside it are
    not.
- id: FR-024-AC-4
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For each `WitnessPacket` member in turn, a packet missing that member is
    refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it,
    and no replay runs. A packet whose trace position is present with the value none
    is admitted.
- id: FR-024-AC-6
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>`
    through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence
    identities reach QSL unchanged.
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both preserve domain validity and the failure; a candidate that
    changes domain validity or the verdict is discarded.
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the original counterexample's.
- id: FR-024-AC-10
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord`
    or `ObligationIdentity` is defined under `src/`, and no source file under `src/`
    imports any of them from `quire_contract_ir`.
- id: FR-024-AC-11
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateClauseReplay::replay` returns exactly the `StateClauseReplayResult`
    that `qsl_replay::replay_state_clause` returns when a test calls it directly with
    the same request and envelope, for a violating run and for a respecting run, and
    the two results differ from each other. `StateClauseReplay::replay_through` hands
    the executor the request and envelope it built and returns the executor''s value
    as it is: an executor returning a sentinel result changes the returned result
    to that sentinel.'
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node,
    and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site`
    returns for the clause name, and the payload's `clause` is that name and its `observation`
    is the `Invocation` arm. For a unit with two postcondition clauses, each clause's
    envelope carries its own node and occurrence. A clause name the unit does not
    declare returns `StateClauseReplayError::CallSite` holding the call site's refusal
    when the request is built, and the executor is not called.
- id: FR-024-AC-13
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The pre snapshot holds, for every declared field of the state object, the
    value the Kani playback bound to that field, and the post snapshot holds the post-state
    value the caller supplied for it; each is one `complete` population holding one
    object with each field as an integer. The invocation document holds the members
    the Behavior bullets state, with `parameters` empty, `result` null and `created`
    and `deleted` empty, and its `pre` and `post` carry each snapshot's identity and
    `sha256-jcs` digest. Changing one playback value changes the pre snapshot's bytes
    and digest, and changing one post-
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a document vector whose members are in non-sorted source order, whose
    text holds characters JSON escapes (a quote, a backslash, a control character,
    a non-ASCII letter) and whose numeric members include a large integer, a number
    with a fractional part and an exponent form, the bytes and the digest the state-clause
    builder returns equal what `core::canonical`'s bytes and digest functions return
    for the same value. The source of the new state-clause module names none of `quire_canonical`'s
    encoder functions, `sha2`, `ByteDigest::of`, a `sort` call over document members
    or a hand-written stri
- id: FR-024-AC-15
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField`
    naming that field, the executor is not called, and no snapshot holds a default
    for it. A playback or post-state name outside the path's declared field list returns
    `StateClauseReplayError::UndeclaredField` and a field bound twice returns `DuplicateField`,
    each naming the field, in the playback and in the post state alike.
- id: FR-024-AC-16
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a subject mutated to violate the postcondition, the replay settles
    `reproduced-with-evaluated-witness` with category `violation` and an evaluated
    `false`. For the unmutated subject's run over the same pre state it settles `inconclusive`
    with cause `Verdicts` (`violation` proved, `success` replayed). The envelope is
    on the `Witness` arm with a payload `witness` of none in both cases.
- id: FR-024-AC-17
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain`
    naming the field and the executor is not called. The range's two endpoints are
    admitted and the values one below and one above are not.
- id: FR-024-AC-18
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the falsified operation-contract harness of
    a postcondition state clause over a subject mutated to debit is replayed from
    its real Kani playback through `replay_state_clause` and settles `reproduced-with-evaluated-witness`,
    `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl`
    in the module `kani_obligations_state_clause_replay`, so the `kani_obligations`
    filter of `make kani` selects it.
- id: FR-024-AC-19
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'An operation that declares a parameter, and one that declares a result,
    each return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation
    and the declaration (its parameters and result), build no document, and do not
    call the executor; an operation that declares neither is not refused for its shape.
    The shape check reads the admitted package the caller supplies, and QSL''s admission
    reads the documents provided in the byte provision; where the two disagree, the
    disagreement surfaces as a QSL refusal and not as this error. The error is not
    read as a QSL refusal: FR-029-AC-16'
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'One obligation-identity function in `src/replay/obligation.rs` mints the
    identity of the function path and of the frame path: the SHA-256 of the RFC 8785
    encoding, made by `core::canonical`, of one object with the four members `function`,
    `declaration`, `kind` and `arguments` of AD-003 E-1, under their existing spelling.
    No function-path identity changes: the existing golden vectors still pass unedited.
    A frame identity over a fixed site and kind, with empty `arguments`, equals the
    digest of a hand-written text of that preimage, written in the RFC 8785 member
    order, which differs from the prei'
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The identity changes when any one of these changes alone: the kind, the
    `function` node and the `declaration` key (node, role or ordinal); it does not
    change when the harness''s clause node, module symbol, harness symbol, state or
    subject path, solver, unwind bound, options, state fields or their ranges change,
    and no input of the function is a source span. Two postcondition clauses of one
    operation therefore mint one frame identity, as QSL gives them one shared frame
    occurrence. Two units that differ only in the frame''s `modifies` grants have
    different frame identities, and two operations of o'
- id: FR-024-AC-22
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'A frame obligation''s identity takes `function` and `declaration` from
    the `OperationSite`''s `frame` and `frame_occurrence`, so they equal the envelope''s
    `clause_node` and `occurrence_key`; its `kind` is `frame` and is derived from
    the harness''s `property`, never supplied; its `arguments` are the empty list,
    since a frame has no parameters, and no state field or range is a member. A harness
    whose `property` is a postcondition returns `FrameReplayError::NotAFrame`, and
    a frame whose granted and checked fields are not exactly its state fields returns
    `FrameReplayError::FieldSetMismatch`; neither '
- id: FR-024-AC-23
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateFrameIdentity` and its persisted record carry `state_fields`, every
    field the harness draws in draw order, which `domains` (ranged fields only) and
    `property`''s granted and checked lists (unordered) do not give. `state_fields`
    is used for decoding and ordering only and is not minted. A field with no declared
    range is listed in `state_fields`, has no entry in `domains`, is decoded and is
    not range-checked or refused (the AD-003 owner may tighten that later). A regenerated
    harness from equal inputs has a byte-identical record, and a record without `state_fields`,
    or one that lists a state '
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`FrameReplayInputs` has no `obligation_identity` member. The frame request''s
    `obligation_identity` and the envelope''s both equal the identity minted from
    the site and the harness, and a harness with a changed grant gives a different
    value in both. The frame twin of `tests/state_frame_support/native_twin.rs` passes
    no identity of its own, so no stand-in value reaches either member; no source
    scan is part of this criterion. The state-clause path''s caller-supplied `obligation_identity`
    is not covered by this criterion (Q-1).'
- id: FR-024-AC-25
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The frame witness is built from the real playback: the playback text decodes
    through the one decoder of `src/replay/witness.rs` against bindings built from
    the harness''s `state_fields` (each `i64`, in draw order), and the transcript
    passed to `Witness::parse` is rendered by the one adapter rendering function from
    the decoded values, with the harness path and the check text the decode names.
    The fixed per-operation assertion text that `frame.rs` passes to `Witness::parse`
    today is gone. A playback of another harness, a playback with the wrong number
    of values and one with a wrong-width value ea'
- id: FR-024-AC-26
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A decoded state-field value outside the field's declared range returns
    `FrameReplayError::OutOfDomain` naming the field and its value, and calls neither
    `call_site` nor `replay_frame`; the range's two endpoints are admitted and the
    values one below and one above are not. A field with no declared range is not
    range-checked.
- id: FR-024-AC-27
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The decoded pre-state is tied to the invocation: for every state field,
    the integer the pre snapshot (the provided document whose `sha256-jcs` digest
    the invocation''s `pre` names) holds for the object the invocation''s `self` addresses
    equals the decoded value, and a field that differs returns `FrameReplayError::PreState`
    naming the field, the decoded value and the snapshot''s value; an invocation or
    pre snapshot that is not among the provided documents, is not the document shape
    the state-clause path writes (the typed leaf shapes of `src/replay/state_clause.rs`),
    or lacks the object or a field '
- id: FR-024-AC-28
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A harness whose `scope.operation` is not the operation requested returns
    `FrameReplayError::ScopeMismatch` naming `operation` before `call_site` is called;
    one whose `scope.anchor` or `scope.frame` is not the anchor or frame `call_site`
    names returns `ScopeMismatch` naming that member and does not call `replay_frame`.
    The harness's module and harness symbols name the generated artifact, are not
    members of the identity, and are checked only by the decode (FR-024-AC-25).
- id: FR-024-AC-29
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The frame envelope's `declared_domains` is the empty list, no source of
    `src/replay/frame.rs` builds a `DeclaredDomain` or a `DomainKey`, and the module's
    header states, once, that the declaration is empty until QSL-345 settles the declared-domain
    key and refuses an empty declaration. The frame path adopts no key shape before
    then, whatever shape another path builds.
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the real playback of the falsified frame harness
    of a subject that writes a forbidden field is replayed through `FrameReplay::new`
    and `replay`, the caller supplying the harness's `StateFrameIdentity`, the playback
    text and the inputs of FR-024's Inputs list but no obligation identity and no
    transcript, and settles a reproduced violation naming the written field. The harness
    is generated from QSL's emitted package and its scope node ids equal those `qsl_replay::call_site`
    names, without `Twin::aligned` (FR-024-AC-32). A subject that writes only a granted
    field leave
- id: FR-024-AC-31
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. For a QSL-emitted package, `StateClauseReplay` takes names and order
    from its inputs'' `state_fields` and validates each name with `model_object_fields(&object_id)`:
    for the twin''s `balance` and unread `audit`, both are present with the accessor''s
    0 to 1000 inclusive range. A playback or post state omitting one returns `MissingField`;
    a bound name outside the list returns `UndeclaredField`; a listed name absent
    from the accessor returns `ModelFields { object, cause: Absent { field } }`, and
    an accessor er'
- id: FR-024-AC-32
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A frame harness generated from the package QSL emits for the twin is
    replayed through `FrameReplay::new` and `replay`: its generated `scope.anchor`
    and `scope.frame` equal the ids `call_site` names, asserted before replay; `Twin::aligned`
    is gone from test support; a forbidden write settles `reproduced-with-evaluated-witness`
    with category `violation` naming the written field; its `state_fields` retain
    request order and its `domains` equal the accessor''s ranges for listed fields,
    including an unread rang'
- id: FR-024-AC-33
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A postcondition harness generated from the package QSL emits for the
    twin, with no identity alignment, is replayed through `StateClauseReplay::new`
    and `replay`: the debit mutation settles `reproduced-with-evaluated-witness` with
    category `violation` and evaluated `false`; the unmutated subject over the same
    pre state settles `inconclusive` with cause `Verdicts` (FR-024-AC-16); the clause
    id is the one `call_site` names for `BalanceNeverDrops`, and the declared field
    ranges equal those returned by the ac'
- id: FR-024-AC-34
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the installed backend, the real playback of the falsified frame
    harness of a forbidden write (FR-024-AC-30) and the falsified postcondition harness
    of a debit mutation (FR-024-AC-18) replay and settle as those criteria state,
    each harness generated from QSL's emitted package using accessor-derived ranges.
    The modules `kani_obligations_state_frame` and `kani_obligations_state_clause_replay`
    are selected by the `kani_obligations` filter of `make kani`.
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsified frame or postcondition harness draws a present model field
    whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint
    below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value
    outside that model range. QSL refuses the pre state and settlement is `Inconclusive`
    with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange`
    reason; it never reports `Verified` or a violation. Playback inside the model
    range is unaffected, a
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignments, each adapter''s backend identity, the same opaque
    obligation identity, declared domains, source/provenance and replay packet members;
    decode refusal carries only backend identity, obligation identity and typed cause.
    Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each
    successful adapter''s original backend and source identity in a complete packet.'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode
    evidence failure retaining its cause, with no envelope or replay. Before native
    replay, an out-of-domain assignment yields a distinct typed domain evidence failure;
    an incomplete or QSL-refused witness/packet yields a distinct typed QSL admission
    evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract
    failure, and none of the pre-replay failures invokes replay. Kani's exact decode
    and refusal partition remain governed by FR
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure becomes a new CG lineage revision referring to its
    exact admitted QSL envelope, its exact parent revision and the unchanged opaque
    obligation identity. After retention, the original QSL envelope compares equal
    to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal
    to their captured values. No lineage member is added to QSL''s envelope or packet.'
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that changes the native verdict each create no revision. After each
    rejection, every prior QSL envelope compares equal to its pre-proposal clone and
    every prior CG record's revision identity, parent, obligation and typed evidence
    fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript.
- id: FR-032-AC-1
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consumer passes the routed harness's `ScalarObligationIdentity` directly
    to the scalar builder; its typed call constructs neither `KaniObligationIdentity`
    nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request
    returns unsupported-capability only while a named step-5 capability is unavailable;
    when all are available it returns an admitted plan, and authenticated completion
    enables replay/conversion. Conversion of a capability refusal returns no settlement
    rather than a synthe
- id: FR-032-AC-2
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631). The builder decodes actual routed-harness playback in
    persisted symbolic call order through the Kani output adapter; another harness
    name, missing/extra operand, wrong byte width, playback above the builder-owned
    8 MiB per-harness ceiling or malformed assertion block yields the distinct typed
    refusal under the stated precedence before replay.
- id: FR-032-AC-3
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain
    value is refused before replay. The admitted QSL inputs preserve the catalogued
    operator, operands, operand ranges, result range and harness limits. Another node/operation,
    altered operand schema or replay limit differing from the retained original generation
    context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-16.
- id: FR-032-AC-4
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's actual native observation of the proved generated oracle is required
    and tied to this operand vector, limits and canonical proof-content identity;
    the checked completion requires driver authentication; a bare outcome/digest echo,
    missing observation, another artifact's observation or fresh regeneration that
    removes the arithmetic mutation refuses before QSL evaluation. Playback bytes
    and Kani falsification alone cannot supply a generated outcome.
- id: FR-032-AC-5
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The public consumer executes the same proved generated artifact and invokes QSL
    between the builder and converter. QSL compares its actual native observation
    with authoritative exact operator evaluation using the four-operator projection
    owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37,
    without adding refusal-cause or accounting-counter equality; changing the generated
    arithmetic while keeping original node/operator identity changes the measured
    parity result. `Undefined`, `Inco
- id: FR-032-AC-6
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete`
    and other `Refused` outcome, the converter compares the report's full claim to
    the retained sent claim and actual sent wire against the original proving context
    before conversion; a changed package/function/source/dependency or original limit
    refuses even with an equal claim. At the immutable-report/retained-binding boundary,
    an absent report, another result/run, or changing only the obligation, node, occurrence,
    kind, canonica
- id: FR-032-AC-7
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    QSL's divergence between generated and authoritative exact outcomes becomes a
    CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`;
    native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed` while
    exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` preserves
    `invalid_runtime_input`; other typed QSL refusals follow the existing refusal
    map and non-fault refusals retain their code. This scalar route never returns
    `Reproduce
- id: FR-032-AC-8
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises
    routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact
    native observation. A separate bounded-addition fixture with both operands and
    result in `[-1000,1000]` exercises an admitted operand pair whose exact result
    is outside the result range. The expected exact result is derived from the actual
    retained playback, and its correct refusal is never labelled a source violation.
    The fixture ranges and wit
- id: FR-032-AC-9
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention).
    Independent canonical-preimage checks include only the claimed application node,
    authentic expression occurrence, authoritative scalar kind and one argument per
    operand position in actual draw order, each with position ordinal, typed `OperandIdentity`
    and harness operand range. Actual graph references use `GraphChild { node_id }`;
    inline literals use `InlineLiteral { node_id, occurrence_key, position }` from
    the authentic application/occurrence and position, with their value in the singleton
    range. QSL emits no ext
- id: FR-032-AC-10
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family retention and actual
    CG/QSL conformance). When the authoritative scalar kind and retained positional
    metadata are available, the public shared mint produces the identity for the independently
    checked exact O-09 canonical scalar preimage, and the actual QSL operator route
    accepts it. A copied encoder, stand-in verifier or synthetic success cannot pass.
- id: FR-032-AC-11
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    actual CG/QSL route). When the obligation digest differs from the retained positional
    preimage, after an otherwise membership-valid preimage member change with the
    old digest, the consumer observes QSL's typed `ScalarIdentity::Obligation` refusal
    with claimed/recomputed digests and `stale_dependency`/`revision-mismatch` before
    comparison.
- id: FR-032-AC-12
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    actual CG/QSL route). When a correctly minted claim invents an application occurrence
    or operand child, or misstates an inline literal's value/singleton range, the
    consumer observes the corresponding merged typed membership refusal before identity
    comparison; self-consistent hashing cannot admit the invented metadata.
- id: FR-032-AC-13
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    CG builder). When the owning mint cannot encode the preimage, the public `parity_obligation`
    returns `IdentityEncodeError` with no identity, and the CG builder returns typed
    setup refusal without replay. No claimed digest or usable sentinel can replace
    that error.
- id: FR-032-AC-14
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding
    the scalar builder/converter preserves existing function/clause carry-only behavior
    and function/frame identity encoding; none is redirected through scalar parity
    or changed by its positional argument rule.
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 verified Eq graph-child constructor controls and Ne parameter/parameter
    and parameter/literal controls, including operator mismatch refusal; Ne literal/literal
    is AC-14 and inline operands are AC-16 and AC-17, all PLANNED under IR-814; falsified
    and generated-artifact routes planned, IR-806). A public consumer builds the equality-parity
    request directly from the composite node and original proving context. CG submits
    the original source/package and node selector to QSL and accepts only its admitted
    same-package/node result under QSpec FR-322. CG validates admitted node/occurren
- id: FR-033-AC-2
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family,
    including beyond-i64 composite integer leaves as ExactInteger (while top-level
    Integer remains i64), IEEE NaN and signed-zero bits, normalized rational, retained
    decimal, delimiter-containing Unicode text, enum member, exact quantity/unit and
    reference identity, and every listed composite kind. Mixed nested values retain
    declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends
    on a currently supported Boolean/i64 harness alone.
- id: FR-033-AC-3
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration
    or unit/reference identity, wrong enum/union member, missing/extra or wrongly
    present record slot, wrong tuple arity, excess cardinality, out-of-domain leaf,
    duplicate set/ordered-set element under authoritative equality or altered canonical
    element order yields a typed operand/decode refusal before exact evaluation when
    that stage is reached, after common claim checks and same-valid-claim Disagreed
    precedence. An admitted claim whose operand field x is declared Int[0, 9] but
    supplied as x: 12 with Disagreed yields'
- id: FR-033-AC-4
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Public caller input exactly at the configured encoded-byte
    limit is admitted and one byte over refuses before parsing; occurrence/work exhaustion
    refuses with checked accounting. A 100,000-link admitted recursive value decodes,
    clones, compares, formats redacted Debug and drops on a 512 KiB native stack without
    a depth refusal. Its one-byte-too-small budget refuses truthfully; launcher capture
    limits alone cannot satisfy this check.
- id: FR-033-AC-5
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand
    values by the FR-025 original-node/leaf bindings; another harness or persisted
    binding schema refuses claim binding. For the same admitted claim, swapped playback
    leaf order, missing/extra playback leaf or wrong primitive width refuses reconstruction
    only after Disagreed precedence; retained Disagreed remains Failed/CgDefect. Literal
    payloads remain their singleton value and absent/null/present states remain distinct.
    Removing or changing a leaf binding changes reconstruction or refuses, rather
    than silently decoding an
- id: FR-033-AC-6
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. For falsified replay the driver executes the same proved
    generated artifact and supplies its actual native outcome/count; QSL receives
    this observation, the retained shadow result/count and the canonical content tie.
    Another artifact/context, fresh regeneration that removes the mutation or changed
    original limits refuses at claim binding. For the same admitted claim, missing
    observation cannot mask Disagreed/Failed/CgDefect; without Disagreed it refuses
    a replay continuing beyond F-1 before exact evaluation, with no fabricated observation.
    A canned observation or Kani transcript
- id: FR-033-AC-7
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public F-7 converter controls; IR-806 original-artifact
    path planned). For an identity-valid claim reaching row F-7, actual QSL exact
    equality/count divergence from a falsified shadow maps to CG-defect `Failed`;
    an assertion-only mutation with otherwise agreeing equality/count maps to `Inconclusive(ScalarAgrees)`
    retaining the CompositeEquality claim/outcome. Neither maps to `Refuted`. Independently
    mutating equality result and occurrence-pair count is detected, including no early
    exit after an unequal pair.
- id: FR-033-AC-8
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and
    literal singleton domains and consumes QSL-derived bound keys without a CG-authored
    declared-bound list. Omitting a bounded position or using an empty list with declared/unbounded/recursive
    keys leaves it uncovered; unknown/duplicate/kind-mismatched keys refuse. Exact
    declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth
    does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when
    every source-declared variant is included; a Variants bound naming an undeclared
    varia
- id: FR-033-AC-9
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 report binding covered; IR-635 Eq-only verified source/package/node/occurrence
    prechecks and genuine invocation covered; generated-artifact authentication and
    falsified invocation planned, IR-806). CG compares `report.claim()` with `CompositeIdentity::new`
    over the sent obligation, node, occurrence, operator, obligation kind, harness
    bounds, limits, content identity and complete observation: falsified operands,
    shadow verdict/count, native observation/cause and refinement, or verified SUCCESS
    count and refinement. Changing each CG-owned retained sent-identity member independent'
- id: FR-033-AC-10
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed
    route has a real verify and falsify control with retained backend evidence and
    mutation-killed same-artifact replay; all canonical families have the AC-2/3 decode
    checks. Families awaiting shadow generation or upstream admission remain explicitly
    unsupported/gated and are reported as gaps, rather than counted as passing end-to-end
    coverage. Open real Text-profile, nested set/bag performance and source-unbounded
    coverage limitations remain explicit; API presence or rational substitution supplies
    no generated-family acceptanc
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PARTIAL (IR-635 Eq graph-child positional O-09 controls and Ne parameter/parameter
    and parameter/literal controls; Eq self-comparison is covered; Ne literal/literal
    (AC-14), Ne self-comparison (AC-15) and the inline-operand refusals (AC-16, AC-17)
    are PLANNED under IR-814. Native integration follows from AC-6 (IR-806); the harness
    identity record is FR-015-AC-76, and only the Population-keyed metadata part has
    no owning criterion yet; the planner will specify it as a new FR-033 criterion
    under IR-806). CG computes O-09 through the owning typed parity_obligation from
    the exact claimed applicati
- id: FR-033-AC-12
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct QSL F-1 precedence controls; IR-635 Eq-only verified
    constructor and QSL membership/unknown-bound refusals before supplied Disagreed;
    falsified and cross-outcome integration planned, IR-806). Both paths carry the
    same four-state Refinement. Claim/source/node/occurrence/bounds/O-09 refusal precedes
    Disagreed; for a valid claim Disagreed wins over operand refusal (including an
    operand field x declared Int[0, 9] but supplied as x: 12), missing native observation,
    native fault, exact limit, refinement ceiling and agreeing replay. No early operand/native
    setup refusal erases '
- id: FR-033-AC-13
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct public F-2 to F-6 controls; IR-806 must reach the
    same rows from a CG-built request over the original proving context). On a valid
    falsified claim without Disagreed, a native Incomplete or ExecutionFault wins
    over an invalid operand, too-small admission limit, too-small exact limit and
    CeilingReached: CG observes GeneratedFault/Failed retaining its NativeCause. With
    a Completed native, an operand outside its declared domain yields RefusedInput/ReplayRefused
    even with CeilingReached; a request limit reached during admission yields Incomplete/Admission
    with its counter, an'
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records,
    with bounded-field narrowing literals (IR-691) and collection-constructor literals
    gated). When the original application node is a `structural.ne` whose two operands
    are both composite literals and the requested operator is NotEqual, the verified
    request carries NotEqual and an O-09 of two positional `graph_child` arguments,
    each the literal's own node with empty Bounds and no parameter position.
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne`
    comparing one parameter with itself and the requested operator is NotEqual, the
    verified request carries NotEqual and an O-09 of two positional `graph_child`
    arguments with the same node identity, each retaining the drawn Node-keyed bounds
    naming that parameter and neither deduplicated.
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814; verified by Inspection of the single conversion that AC-17
    tests). If the owning operand projection refuses an inline integer literal operand
    (`InlineInteger`), then the builder returns that refusal through `CompositeBuildError::Operands`
    before any QSL invocation, with no report or terminal value and no built argument.
    The owning contract treats this branch as defensive because reader admission already
    refuses an integer operand of `structural.eq` and `structural.ne`, so no admitted
    fixture can reach it. Both inline refusals pass through the same `CompositeBuildError::Operand
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer
    literal operand (`InlineNonInteger`), then the builder returns that refusal through
    `CompositeBuildError::Operands` before any QSL invocation, with no report or terminal
    value and no new CG error variant. No test of this path exists yet.
- id: FR-019-AC-1
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Settlement dispatches one arm per variant of the closed backend kind, and
    every variant reaches an arm that settles rather than falling through.
- id: FR-019-AC-2
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Every requested item receives exactly one of `supported`, `requires-bound`,
    `unsupported` and `invalid-request`, settled from its `candidates` and extent
    classification, and `candidates` is read from the request rather than computed
    here.
- id: FR-019-AC-3
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement
    settles `requires-bound` when a finite bound is available, rather than `unsupported`;
    with no finite bound available it settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`;
    and it never settles `supported`.
- id: FR-019-AC-4
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with more than one candidate and no named backend settles `invalid-request`
    with `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate
    order, identically under every registration order.
- id: FR-019-AC-10
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Only an item settled `supported` routes.
- id: FR-019-AC-45
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A manifest with a second descriptor of one backend identity causes a whole-envelope
    `EnvelopeRefusal::ManifestRepeatsBackendIdentity { identity: String }`, whether
    the descriptors are identical, differ in `ProviderOrigin` alone, or differ in
    any other CG descriptor field, and whether adjacent or non-adjacent. The call
    returns no item settlements. The variant has no string cause, manifest or digest
    payload; `negotiate_backend_provider` keeps its return signature.'
- id: FR-019-AC-46
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With several repeated identities, the refusal names the bytewise-first
    repeated identity independent of manifest order; identities differing only in
    case are distinct. An invalid contract version or capability vocabulary is refused
    as `EnvelopeRefusal::InvalidCapability` before repeat detection or any kind read.
- id: FR-019-AC-47
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: QSL's registry snapshot holds each backend identity at most once (FR-290:258-265;
    FR-331:57), so no driver-built FR-331 manifest repeats an identity.
- id: FR-019-AC-48
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A `Refused(refusal)` outcome from the current Kani oracle-function preflight
    read produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection
    })` for the whole request, with the reader''s `code`, `path`, `cause`, `locus`,
    `contract_version`, `document_pointer`, `expected` and `actual` copied into the
    named CG projection; the private `expected_node_id` is absent. No item settlement
    or artifact is returned.'
- id: FR-019-AC-49
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'An `Incomplete(incomplete)` outcome from the current Kani oracle-function
    preflight read produces `Err(EnvelopeRefusal::ContractPackageReadIncomplete {
    incomplete: ContractPackageIncompleteProjection })` for the whole request, with
    the reader''s `limit_kind`, `limit`, `consumed` and `path` copied into the named
    CG projection. No item settlement or artifact is returned.'
- id: FR-019-AC-50
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application preflight read whose exact generator
    produces a claim for an otherwise supported item retains the existing `supported`
    settlement.
- id: FR-019-AC-51
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Among several request items with non-`Admitted` preflight outcomes, the
    first failing item in manifest/item request order determines the whole-request
    `ContractPackageReadRefused` or `ContractPackageReadIncomplete` variant and projection,
    irrespective of outcome kind. Earlier successful items contribute no returned
    settlement or artifact.
- id: FR-019-AC-52
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An `Admitted` Kani function-application read whose exact generator emits
    no claim, and a structural preflight rejection before any read, each retain item-level
    `KaniFormRefused { OperationNotRendered }` without a reader envelope refusal.
- id: FR-019-AC-53
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Given an admitted package with bound driver items at driver indexes 0 and
    2 around an excluded accounting-only position at index 1 whose accounting is `lowered`
    with a Kani-supported form, an FR-331 envelope with exactly two items settles
    the bound items without an `EnvelopeItemCount` refusal. Both `ContractItemSettlement.request_index`
    and its nested `ItemSettlement.request_index` equal 0 and 2 respectively, not
    envelope indexes 0 and 1. The excluded position produces no settlement or claim,
    and its own classifier-call and Kani-preflight-read counts are both zero. An envelope
    with fewer or mo
- id: FR-019-AC-7
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An envelope whose `capability_vocabulary` is not exactly `quire.capability-kind/v1`,
    or is absent, is refused as `invalid_capability`/`unsupported-version` with none
    of its kinds read.
- id: FR-019-AC-8
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: An item with an absent extent classification settles `invalid-request`
    with `invalid_capability`/`absent-extent`, and an absent or unknown kind settles
    before the candidate table is consulted.
- id: FR-019-AC-9
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'A backend kind added without a negotiation arm does not compile: the dispatch
    is an exhaustive `match` over the closed kind with no catch-all arm.'
- id: FR-019-AC-11
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). `BackendKind` has `Process(id)` beside `Kani`; `ALL`
    lists finite built-in kinds only, while CG enumerates process candidates from
    descriptors. CG's descriptor-to-kind conversion returns `Process(id)` exactly
    when a descriptor has process origin, including identity text `kani`. A named,
    registered process backend reaches that arm and receives one disposition; no process
    item disappears for being absent from `ALL`.
- id: FR-019-AC-12
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; non-empty bounded rows depend on QSL-654's producer and
    admit-side enforcement). A bounded process item with an uncovered `extent.bounds[].kind`
    settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
    naming the offending domain kind and candidate (QSpec FR-290-AC-13). An item that
    passes the domain check, including one with empty `bounds`, settles `supported`
    exactly when its descriptor advertises `bounded` for its kind; otherwise it settles
    `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`.
- id: FR-019-AC-16
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). An unbounded process item settles `supported` when its
    descriptor advertises `unbounded` for its kind, regardless of whether any `extent.domains[].kind`
    belongs to manifest `domains`.
- id: FR-019-AC-17
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item settles `requires-bound` exactly when `finite_bound_available` is true and
    every boundable `extent.domains[].kind` is in manifest `domains` (QSpec FR-290-AC-13).
- id: FR-019-AC-18
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). Given the same process advertisements and item extent,
    changing only backend identity, manifest position, manifest run-limit defaults
    or ambient state leaves the disposition and cause unchanged apart from the backend
    named in the output.
- id: FR-019-AC-19
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654's producer). The process
    arm reads each explicit `extent.bounds[].kind` and never derives a domain kind
    from `DomainKey`; two otherwise equal bounded items with the same `DomainKey`
    and different explicit kinds can settle differently under one manifest `domains`
    set.
- id: FR-019-AC-20
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; driver projection remains paired work). CG's process descriptor
    retains the FR-331 manifest's advertised (kind, mode) pairs, `domains` and optional
    `bounds` beside the IR-633 `id` and `origin` projection.
- id: FR-019-AC-42
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two otherwise equal CG Process descriptors with absent and present-empty
    manifest `bounds` remain distinct typed values. Changing only between those forms
    leaves process settlement's disposition and request-validation precedence unchanged;
    run-limit defaults are not proof bounds.
- id: FR-019-AC-21
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with any unadvertised boundable `extent.domains[].kind` settles `unsupported`,
    warned, with `unsupported_projection`/`unsupported-requested-capability` naming
    the offending domain kind and candidate, regardless of `finite_bound_available`;
    non-boundable kinds are not compared (QSpec FR-290-AC-13).
- id: FR-019-AC-22
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629). On a `bounded`-only process descriptor, an unbounded
    item with every boundable `extent.domains[].kind` advertised and the item's `finite_bound_available=false`
    settles `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`.
    CG reads that flag from the item and does not derive it from the domain kinds
    (QSpec FR-290-AC-13).
- id: FR-019-AC-23
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-629; bounded row depends on QSL-654). Changing only a bounded
    item's proof-bound numeric maximum leaves its process-provider `supported` or
    `unsupported` disposition and cause unchanged.
- id: FR-019-AC-13
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'Settling a process-provider item reaches no plugin: a descriptor whose
    identity names a non-existent executable settles identically to one with an ordinary
    identity, apart from the backend each names, and starts nothing, and a descriptor
    whose identity names an executable that records its own start leaves no record.
    A mutant arm that starts or resolves the identity as a process either changes
    the first disposition or leaves the record, and fails this.'
- id: FR-019-AC-14
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: The process-provider arm's return type is `Disposition`, which has no terminal-value,
    verification-result or artifact member, so settling returns none of them. A change
    that returns one does not compile against that type.
- id: FR-019-AC-15
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin`
    projecting exactly QSL layer R's `Linked` and `Process` meanings (QSL FR-288,
    ADR-029 PV-1); layer R owns that vocabulary. The driver projects each descriptor
    from `Registry::descriptors()` once into CG's descriptor, copying `id` and advertised
    pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and
    `Process` to `Process`, with no wildcard or origin inference from identity, manifest
    or provider bytes or a side map. Two registry descriptors identical except for
    origin produce CG descriptors identical e
- id: FR-019-AC-24
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at
    an unbounded `s: Set<Int[0,9999]>`, an empty candidate registry settles `unsupported`
    with a warning naming `value-validity` and emits no artifact or Kani outcome;
    one linked Kani candidate advertising (`value-validity`, `bounded`) with `finite_bound_available=true`
    settles the unbounded item `requires-bound` without substituting a bound or emitting
    an artifact; a separately requested bounded `ProofBound::Cardinality{maximum:
    8}` item with that same sole candidate settles `supported` under its own request
    index, while the un'
- id: FR-019-AC-25
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: 'The normative Kani form set is the complete V1 proof set defined by IR-269
    and FR-015: scalar families Boolean, integer, rational, decimal, IEEE float, text,
    enum and quantity; record and tuple composite equality; function application;
    precondition, postcondition and invariant `BoundClause` roles; and bounded collection
    and graph forms. V1 `ClauseKind` has no frame variant. State before/after and
    frame obligations are typed V2 `StateFrame` items with the `contract` or `frame`
    role, included where FR-015 defines their typed lowering. Each item is identified
    by its typed IR tag/form, family, ope'
- id: FR-019-AC-26
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For any form in FR-019-AC-25 with one validated Kani candidate, a bounded
    extent settles `supported` exactly when the form's FR-015 lowering succeeds and
    the candidate advertises the bounded mode; negotiation creates no artifact as
    part of settlement.
- id: FR-019-AC-27
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: With one Kani candidate, a typed form outside the normative set in FR-019-AC-25
    settles `unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
    naming its typed tag/form and Kani; no artifact is emitted and no generator is
    invoked. A malformed in-set form settles with the applicable typed FR-015 validation
    refusal.
- id: FR-019-AC-33
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a valid bounded typed form inside FR-019-AC-25 that matches one specific
    Planned subpath in TC-030's status register and does not match an Implemented
    subpath, negotiation returns `unsupported` with that subpath's single exact typed
    refusal cause, emits no artifact, and never settles `supported` before its renderer
    is implemented; this refusal remains distinct from the out-of-V1 refusal in AC-27.
- id: FR-019-AC-28
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: A form refusal under FR-019-AC-27 is distinct from an empty candidate set,
    which retains the missing-capability cause, and from multiple candidates without
    a named backend, which retains `invalid_capability`/`ambiguous-backend` naming
    each candidate in candidate order.
- id: FR-019-AC-29
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: After request and candidate/manifest validation select one Kani candidate,
    Kani checks the typed IR form before its extent and advertised-mode rules; if
    both the form and mode/extent are unsupported, the form refusal of FR-019-AC-27
    settles the item.
- id: FR-019-AC-30
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For the same typed IR tag/form, changing only its display or debug text
    leaves Kani's disposition, cause and named form identity unchanged.
- id: FR-019-AC-31
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a Kani-supported unbounded form against a `bounded`-only advertisement,
    `finite_bound_available=true` settles `requires-bound`, with no substituted bound
    and no artifact.
- id: FR-019-AC-32
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: When a caller supplies a finite bound in a new bounded request for a Kani-supported
    form, negotiation settles `supported` and preserves the exact supplied proof-bound
    value, domain kind and domain identity.
- id: FR-019-AC-34
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1
    through TP-4, TP-5, SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles
    through its explicit-state arm with the recorded form and model-or-protocol subject
    intact; an unadvertised form or HP-1 code claim does not settle `supported` through
    EN-1.
- id: FR-019-AC-35
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's
    kind and form, TP-1 through TP-4 route only to an EN-2 or EN-3 method ADR-018
    assigns to that form and subject; TP-4 does not route to EN-3, and a form absent
    from the candidate's advertised form set settles `unsupported` with no engine
    request.
- id: FR-019-AC-36
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item
    retains its safety and optional liveness halves on the concrete subject through
    EN-1, and the same whole item reaches an advertising SMT arm for RE-3 refutation;
    a separately requested RE-2 `operation-contract` item reaches the SMT arm for
    per-step safety checking, but its disposition cannot substitute for the whole-refinement
    item. An arm lacking the required TP-5 form settles the item `unsupported`.
- id: FR-019-AC-37
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising
    EN-1 arm; only HP-2 reaches an advertising SMT arm under its applicable EN-2 or
    EN-3 method. HP-4 and an unadvertised HP form settle unsupported before an engine,
    HP-5 stays on its possible-property path, and an HP-1 code claim is not settled
    as an EN-1 model claim.
- id: FR-019-AC-38
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with
    its recorded form; only advertised SG-1 reaches an SMT arm. SG-2 or SG-3 against
    an SMT-only candidate settles unsupported and reaches no SMT engine.
- id: FR-019-AC-39
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising
    that exact reduction, with the selection preserved in the routed request. Symmetry
    declarations and POR enter the obligation identity, so their reduced and unreduced
    requests for one claim have distinct obligation identities; a state constraint
    changes request identity but leaves obligation identity unchanged. An unadvertised
    selection or POR on a subject with no footprints settles `unsupported` without
    an artifact, and no arm drops or adds a reduction.
- id: FR-019-AC-40
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising
    the subject's resolved `tso` or `ra` model receives that exact model; a candidate
    lacking it settles `unsupported` with no artifact, and no arm weakens the resolved
    model. An `sc` subject is a separate strong-model control and does not take the
    weak-memory branch.
- id: FR-019-AC-41
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model
    and protocol subject classes are read separately; an arm that discharges only
    one class settles the other `unsupported`, while a supported protocol item retains
    its protocol transition-system subject identity through routing.
- id: FR-019-AC-43
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: Two driver items selecting distinct `expression` occurrences of one checked
    `integer.add` node and different direct `numeric.narrow` occurrences in one admitted
    ContractPackage settle independently under the same Kani descriptor. Each preflight
    result is classified from its own selected result context; changing the selected
    narrow changes that item's typed result identity or refusal without changing the
    other. A node-only classifier or package-wide choice of either bound fails this
    control.
- id: FR-019-AC-44
  path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  excerpt: For a V2 scalar claim, an absent `OwnResult`, an application occurrence
    absent from the admitted source, a conversion occurrence from another source or
    unrelated to the application, a missing conversion context root, or a ContractPackage
    source identity different from the checked source settles `invalid-request` with
    a typed form/context cause and no artifact or harness. Request and candidate/manifest
    defects retain their earlier precedence; for a valid candidate this selected-form
    refusal precedes extent and advertised-mode refusal. Neither a source message
    nor a caller maximum repairs the se
- id: FR-022-AC-1
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Generation dispatches one arm per variant of the closed `BackendKind` through
    an exhaustive `match` with no catch-all, and `GenerationContexts::has` handles
    each variant; adding a variant without either arm does not compile. `Process`
    requires no context.
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals
    what `negotiate_kani_obligations` returns for the same items in ascending request-index
    order with the same context. The one difference is that every index inside a record
    is the driver's request index, and each harness is paired with the record whose
    `harness_symbol` names it.
- id: FR-022-AC-3
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The entry point accepts no backend registry, candidate set, extent or capability
    kind, and constructs no FR-019 `Disposition`.
- id: FR-022-AC-4
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629 internal-invariant test migration). A crate-internal malformed
    routed Kani item whose private backend identity has no CG built-in kind, or converts
    to a kind other than Kani, refuses the whole call as `BackendKindDisagrees`, naming
    the request index, backend, routed kind and converted kind or its absence, with
    no artifact.
- id: FR-022-AC-5
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two routed items with one request index refuse as `DuplicateRequestIndex`
    naming it. A routed Kani item without Kani context refuses as `MissingKindContext`
    naming Kani. Each returns no artifact.
- id: FR-022-AC-6
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group-level refusal (for example an unwind outside `1..=1024`, or
    an unparsable subject path) is returned as `Kani` carrying the unchanged `KaniObligationError`,
    with no artifact.
- id: FR-022-AC-7
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A Kani group containing an `invalid_request` item is listed in `rejected`,
    and every Kani item's record is returned with no harness. A `DuplicateItem` reported
    by FR-015 has `first_index` rewritten to the driver's request index of the first
    occurrence. On the FR-330 package path, an identical selected scalar occurrence
    pair also retains FR-039's earlier typed `DuplicateRequest` claim refusal at both
    positions.
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Every routed item appears exactly once in `items`, in ascending request
    index, under its routed backend and the `KindOutput` variant of its routed kind.
    An item the arm refuses at lowering keeps its typed FR-015 refusal and no harness.
    An empty routed set returns an empty result.
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of
    the routed-item slice yields an identical result. The same node routed at a different
    request index or under another candidate of the same backend yields a byte-identical
    harness when the admitted FR-330 package and selected occurrence are the same.
- id: FR-022-AC-10
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The Kani arm derives each item's operation and domain from the package
    alone; the caller supplies no claim map, and `x + 1` over a parameter `Int[0,
    9]` routes to a supported `quire.op.integer.add` harness with arguments `[0, 9]`
    and `[1, 1]` (the literal at its own value, FR-015-AC-16).
- id: FR-022-AC-11
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'An item with no derivable descriptor keeps its typed refusal and no harness,
    and its siblings'' records and harnesses are unaffected. On the FR-330 package
    path, an absent root retains its exact admitted `InvalidInput` accounting and
    `PackageRefused` output before Kani. On a direct FR-015 request, a node absent
    from the graph is `invalid_request` and rejects the group. A node that does not
    lower or has a refused bound keeps the FR-015 disposition it has when its claim
    is generated by FR-014 (`requires_bound`, `blocked_on_upstream`, `unsatisfiable_bound`).
    A repeated selected pair on the FR-330 '
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.claim_map` is `Some` after a Kani group and contains
    one claim per selected node/occurrence admitted past FR-039 accounting, retaining
    any earlier typed `DuplicateRequest` refusal; an underivable node has one refused
    claim of provenance `underived`, ordered by node id and then selected occurrence
    key.'
- id: FR-022-AC-13
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes an integer node over bounded parameters to a supported
    harness whose arguments are one range per operand and whose result assertion uses
    the result bound: `a + b` over `[0, 9]` and `[10, 20]` with result `[0, 29]` has
    arguments `[0, 9]` and `[10, 20]`; `-e` over `[1, 9]` with result `[-9, -1]` and
    `e * f` over `[1, 9]` and `[100, 200]` with result `[100, 1800]` are supported;
    a node with a plain-typed `reference` operand beside bounded typing (FR-014-AC-25)
    settles `requires_bound` and has no harness.'
- id: FR-022-AC-14
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.oracle_artifacts` is `Some` after a Kani group and `None`
    when nothing is routed. Its `claim-map.json` equals the complete package-derived
    claim map, including typed refused items; a group with no generated claim still
    returns its crate. For `x + 1` over a parameter `Int[0, 9]`, each `Generated`
    claim''s oracle symbol is defined in the returned `src/lib.rs` and appears in
    the supported harness''s Rust source.'
- id: FR-022-AC-15
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'The Kani arm routes the packages QSL emits for `x + 1` over `x: Int[0,
    9]` into `Int[0, 10]`, `x + y` over `Int[0, 9]` and `Int[10, 20]` into `Int[10,
    29]`, and `-z` over `Int[0, 9]` into `Int[-9, 0]` (a literal as a reference to
    its own `value` node, the declared bound on a narrowing `conversion` consuming
    the plain-typed arithmetic node) to supported harnesses with arguments `[0, 9]`
    and `[1, 1]`; `[0, 9]` and `[10, 20]`; and `[0, 9]`, each asserting the result
    against the conversion''s bound (`[0, 10]`, `[10, 29]`, `[-9, 0]`). In that shape
    a plain-Integer parameter beside a bounded one, and'
- id: FR-022-AC-16
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Two Kani harnesses with one `harness_symbol` refuse the call as `DuplicateHarness`
    naming the second harness's `module::harness` path, and no harness is overwritten
    or dropped.
- id: FR-022-AC-17
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A routed `Process(id)` item appears exactly once with
    its own identity and empty `KindOutput::Process`, without a generation context
    or membership in `BackendKind::ALL`.
- id: FR-022-AC-18
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). A crate-internal routed `Process(id)` item whose private
    backend and kind disagree refuses the whole call as `BackendKindDisagrees` with
    `converted=Some(Process(item.backend.identity))` and no generated output.
- id: FR-022-AC-19
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). Permuting a routed-item slice containing Kani and multiple
    process identities leaves its request-index-ordered output unchanged.
- id: FR-022-AC-20
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `RoutedGenerationItem.backend` and `.kind` are private
    while `request_index` and `node_id` remain public; external callers cannot create
    an item with a hand-built `Process(id)` or reassign its backend after `from_descriptor`
    construction.
- id: FR-022-AC-21
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` constructs Kani for a linked `kani`
    descriptor and `Process(id)` for a process descriptor with the same identity text.
    An unknown linked identity never produces Process.
- id: FR-022-AC-22
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: PLANNED (IR-629). `from_descriptor` rejects a backend identity different
    from the descriptor identity as `DescriptorBackendMismatch`, naming both; it rejects
    an unknown linked identity as `UnknownLinkedBackend`, naming the backend. Neither
    refusal produces a routed item.
- id: FR-022-AC-23
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: After FR-019 authenticates two `integer.add` driver items on one admitted
    ContractPackage with distinct selected application/direct-`numeric.narrow` pairs,
    routed generation retains the pair for each item in its claim and harness identity
    and asserts against its own selected result bound. The two generated harnesses
    have distinct identities and expected ranges; a fresh settlement of a changed
    selection changes or refuses only that item, without borrowing the other bound.
- id: FR-022-AC-24
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: A routed scalar item's retained authenticated selection and its driver
    lowering position must name the same application/conversion pair in the same admitted
    ContractPackage. A changed pair, missing conversion context root or occurrence
    absent from that package refuses the whole call with a typed selection error before
    any harness. No package-wide bound search, source-text reparse or copied QSL context
    supplies a replacement.
- id: FR-022-AC-25
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: The public CG API exposes the concrete `ContractScalarError` carried by
    `RoutedGenerationError::ContractScalar`, so an external driver can match `ContractScalarError::Oracle(OracleGenerationError::SourceTooLarge
    { bytes, limit, setting })` and distinguish the scalar source ceiling from request-accounting
    and other oracle failures without parsing text or naming a private module.
- id: FR-022-AC-26
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one
    application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver
    supplies two distinct request indexes and occurrence keys. CG matches each to
    its own requirement record and selected narrow; both supported scalar outputs
    use the shared application operation and their respective result bounds, and their
    obligation and harness identities differ by application occurrence. Reversing
    routed-item order leaves these pairings unchanged.
- id: FR-022-AC-27
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'For `if p then x + 1 else x + 1`, each occurrence''s generated scalar
    claim is conditional on its own QSL path guard: the same guard occurrence with
    `holds=true` for the then item and `holds=false` for the else item. CG preserves
    the full outermost-first guard sequence and uses it when forming the proof obligation;
    a result under `p=false` cannot prove the then claim and one under `p=true` cannot
    prove the else claim. An empty path condition remains unguarded.'
- id: FR-022-AC-28
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement
    records, its key's node disagrees with the authenticated application selection,
    or the keyed record disagrees with the item's result bound, direct conversion
    occurrence or ordered path condition, CG returns a typed refusal identifying the
    request index and occurrence before emitting an oracle, harness or obligation
    identity for that request; it does not use another record with the same node or
    reconstruct missing values from the IR graph or source text.
- id: FR-022-AC-29
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: 'Given an admitted package whose complete lowering sequence has bound driver
    items at indexes 0 and 2 and an excluded accounting-only position at index 1,
    a routed item with `request_index` 1 and the excluded position''s exact `node_id`
    refuses the whole call as `ExcludedDriverIndex { request_index: 1, node_id }`
    before any generation arm, oracle or harness. The same routed backend and descriptor
    at indexes 0 and 2 remain valid controls, and CG never substitutes an envelope
    position or the next bound item''s selection for index 1.'
- id: FR-026-AC-1
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The Kani adapter implements the adapter trait, and a function generic over
    that trait reaches the Kani adapter's generation arm, execution, transcript parser
    and witness renderer through it.
- id: FR-026-AC-4
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The execution evidence type is an associated type of the adapter trait,
    and the Kani adapter's is `KaniExecutionEvidence`.
- id: FR-026-AC-5
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-629). `Process(id)` has no CG adapter trait implementation
    or CG execution path; an inspection of all adapter implementations and execution
    dispatch arms finds no process arm.
- id: FR-002-AC-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Rejected inputs remain distinct from passing cases in generated reports.
- id: FR-002-AC-2
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Boundary strategies exercise immediately inside and outside supported constraints.
- id: FR-002-AC-3
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Supported correlated constraints do not rely on improbable filtering.
- id: FR-002-AC-4
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Shrinking preserves generated constraints or records a residual rejection.
- id: FR-002-AC-5
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: The generated campaign runner owns execution, explicit-discard accounting,
    and conclusion, returns a typed terminal result, and rejects campaigns below either
    configured coverage floor or above the explicit-discard ceiling; exhausted searches
    retain their reason and any missed floor without becoming a completed floor failure.
- id: FR-002-AC-6
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: Integer and generated Boolean harness campaign cases bind exact values
    to executable accepted/rejected expectations; enum cases execute an all-admitted
    membership expectation.
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's
    FR-014 V2 clause oracle.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and postcondition oracle record that the snapshot precedes precondition
    evaluation, which precedes subject invocation, which precedes postcondition evaluation,
    for every case.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator opens
    a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names
    no `BoundPackage`, `ClauseRef`, `quire_contract_model::TypedExpression` or `oracle::boolean_v1`
    item, each of which today's V1 harness names or uses.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same for both selections yields a typed, located diagnostic
    naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsupported diagnostic and no artifact, and the generator
    emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected clause identities and campaign policy is byte-identical. The
    requirement and revision identity that the generated header, `RequirementId` and
    symbols carry is not an input here and waits on OQ-1.
- id: FR-002-AC-13
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is a pass.
- id: FR-002-AC-14
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected precondition and the subject is not invoked.
- id: FR-002-AC-15
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondition oracle completes false is a failed postcondition.
- id: FR-002-AC-16
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition
    oracle returns an outcome that does not complete (stopped or undefined) is reported
    as a non-completed case retaining the oracle's cause, is not counted as a pass.
- id: FR-002-AC-17
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not invoke the subject.
- id: FR-002-AC-18
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not complete does not evaluate the postcondition oracle.
- id: FR-008-AC-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A fresh QSL-produced, IR-admitted ConfigVersion V2 postcondition with direct
    `self.versionNumber = pre(self.versionNumber)` yields two `State` census entries
    in authored order, `Post` then `Pre`, each 0..=1000, with the same project id,
    `quire.op.integer.eq`, and the selected clause and authentic claim retained. The
    let-bound alias form yields a typed operand refusal and no bundle.
- id: FR-008-AC-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: An IR-admitted V2 `amount < 7` invariant with a typed current read of range
    0..=1000 yields operator `Less`, primary `amount`, literal 7 and that inclusive
    census domain. Reversing the two authored operands preserves the reversed positions
    and corresponding operator semantics.
- id: FR-008-AC-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A valid selected comparison generates despite a comparison-oracle-refused
    sibling claim in the same package; requesting the refused claim returns its typed
    selected-claim cause and locus with no bundle. An oracle-construction failure
    for the selected claim returns no bundle. Each of integer.eq/ne/lt/le/gt/ge has
    an executable selected-claim comparison oracle whose result agrees with the ordered
    relation and generated tags. Arithmetic, alias, Boolean and literal-only shapes
    refuse at their own structural loci.
- id: FR-008-AC-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Unknown clause, absent claim, unsupported clause kind and malformed comparison
    report distinct typed causes in the stated first-defect order. Diagnostics carry
    selected clause and claim, structural path and reached child id when available,
    without claiming a unique child source region.
- id: FR-008-AC-5
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: A request with several applicable defects returns the first only. Every
    refusal returns zero artifacts; a successful request returns the complete source,
    census and runner bundle.
- id: FR-008-AC-6
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Either read endpoint or a literal at i64::MIN - 1, i64::MAX + 1, i128::MIN
    or i128::MAX refuses `UnsupportedRelation` at that operand without narrowing.
    Two individually i64-representable reads with unequal ranges also refuse; equal
    ranges admit when other rules hold.
- id: FR-008-AC-7
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repeat, for each of integer.eq, ne, lt, le, gt and ge
    the selected claim's V2 comparison oracle returns, for every valuation of a 1-
    to 4-member domain (read-versus-read and read-versus-literal, the literal at each
    member and in either authored position), the result of an independently evaluated
    ordered relation.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result
    for the selected claim, and no V1 bound oracle or `BoundPackage` is an input to
    it.
- id: FR-008-AC-9
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor
    through its public API only, and the generator's census, ordered relation and
    diagnostics equal the accessor's operator, ordinals, provenance, observation,
    bounds and refusal fields for each closed operator, provenance and observation
    variant.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body`
    field and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-008-CON-1
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL read V2 operand identity, provenance, observation,
    operator and bounds only through IR's public typed accessor. A private decoder
    or copied schema is forbidden.
- id: FR-008-CON-2
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: Admission SHALL finish before any strategy, population, census or runner
    source is published; a refusal returns no artifact or partial bundle.
- id: FR-008-CON-3
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each read endpoint for i64 representability before
    constructing the shared-domain relation.
- id: FR-008-CON-4
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  excerpt: The generator SHALL check each comparison literal for i64 representability
    before conversion.
- id: FR-009-AC-1
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim
    (FR-008-AC-1) over 0..=1000, every `Satisfying` case has post = pre, every `Violating`
    case has post ≠ pre, and every value lies in 0..=1000.
- id: FR-009-AC-2
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: 'For every V2 comparison operator, every domain of 1, 2, 3, and 4 members,
    and both a two-read relation and a read-versus-literal relation with the literal
    at each domain member, the generatable set of each population equals the independently
    enumerated satisfying or violating set: none missing, none extra. An empty side
    is refused with `EmptyPopulation` and never produces an empty strategy.'
- id: FR-009-AC-3
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Generated population source contains no filter, assume, reject, or discard
    call, and a seeded 10,000-case run of each population records zero discards and
    zero proptest global rejects.
- id: FR-009-AC-4
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: For `amount < 0` over 0..=1000, `Satisfying` and `Broad` are refused with
    `EmptyPopulation` naming `Satisfying`, and `Violating` generates; for `amount
    >= 0`, `Violating` and `Broad` are refused naming `Violating`; no refused request
    emits an artifact.
- id: FR-009-AC-5
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`, and `Equal`
    compiles as generated Rust and generates 10,000 seeded cases without overflow
    or panic, every case in domain and on its side.
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: Repeated generation for the same `CheckedPackageV2`, selected clause id,
    authentic claim occurrence, and population is byte-identical.
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the
    literal in the first position produces the same satisfying and violating sets
    as its mirrored read-first form, enumerated independently.
- id: FR-010-AC-1
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: For `amount < 7` over 0..=1000, the in-domain census is exactly [0 Holds,
    1 Holds, 6 Holds, 7 Violated, 8 Violated, 999 Violated, 1000 Violated], and the
    out-of-domain array is exactly [-1, 1001]; for `amount < 1`, where the literal
    edges coincide with the domain edges, the in-domain census is exactly [0 Holds,
    1 Violated, 2 Violated, 999 Violated, 1000 Violated] with no duplicates, and the
    out-of-domain array is exactly [-1, 1001].
- id: FR-010-AC-2
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: 'For the QSL-produced, IR-admitted V2 `VersionUnchanged` selected claim
    (primary post, partner pre; FR-008-AC-1) over 0..=1000, the in-domain census is
    exactly the 10 pairs (post, pre): (0,0 H), (0,1 V), (1,0 V), (1,1 H), (1,2 V),
    (999,998 V), (999,999 H), (999,1000 V), (1000,999 V), (1000,1000 H); the out-of-domain
    array is exactly the 10 pairs (-1,0), (0,-1), (0,1001), (1,-1), (1,1001), (999,-1),
    (999,1001), (1000,-1), (1000,1001), (1001,1000).'
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX`
    (representable under FR-008-AC-6) compiles as generated Rust without overflow
    or panic, its out-of-domain array holds no case for either outer edge, and its
    unrepresentable-edge array lists both.
- id: FR-010-AC-4
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: '`amount <= 1000` over 0..=1000 refuses `Boundary` with `UnsupportedCampaignConstraint`
    and emits no artifact for that request, while its `Satisfying` bundle still carries
    the out-of-domain array [-1, 1001].'
- id: FR-010-AC-5
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: Every in-domain census tag equals an independent evaluation of the relation,
    every out-of-domain case has at least one value outside the domain, and repeated
    generation produces byte-identical arrays.
- id: FR-011-AC-1
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`)
    `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition`
    and passes; a `Precondition` fixture records `Violated` cases as `RejectedPrecondition`
    in `rejected` and passes with zero proptest global rejects; the `amount < 7` `Invariant`
    records `Violated` cases as `FailedPostcondition`.
- id: FR-011-AC-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A V2 comparison oracle deliberately swapped for its negation fails each
    campaign with `ConformanceMismatch` carrying the case values, expected tag, and
    observed verdict kind.
- id: FR-011-AC-3
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: The census runner evaluates each of the 10 `VersionUnchanged` in-domain
    census cases exactly once, in census order, and evaluates none of the out-of-domain
    cases.
- id: FR-011-AC-4
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A campaign over a report seeded with fixed prior counts returns `discard_rate()
    == Some((discarded, attempted))` and `rejection_rate() == Some((rejected, attempted))`
    with those exact values, a fresh zero-attempt summary returns `None` for both,
    and a runtime snapshot whose counters are `at_limit` returns `None` for both through
    the generated snapshot-summary operation.
- id: FR-011-AC-5
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate()
    == Some((0, attempted))` with `attempted > 0`, passes a zero discard ceiling,
    and does not end `Exhausted`.
- id: FR-012-AC-1
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For every V2 comparison operator and every domain of 1 to 4 members from
    FR-009-AC-2, an exhaustive walk of every valid `simplify` and `complicate` path
    of `Satisfying`, `Violating`, and `Broad` value trees visits only in-domain candidates
    that keep their original tag; a valid path calls `complicate` only after at least
    one successful `simplify`.
- id: FR-012-AC-2
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: For 1,000 seeded `VersionUnchanged` value trees of each population over
    0..=1000, every candidate on the greedy shrink path stays in domain and keeps
    its tag.
- id: FR-012-AC-3
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: A `Violating` V2 `VersionUnchanged` campaign whose comparison oracle is
    replaced by `post == pre
- id: FR-012-AC-4
  path: spec/strategy/functional/FR-012-constraint-preserving-shrinking.md
  role: examined
  excerpt: After a shrinking campaign, `attempted` equals the number of oracle evaluations
    the runner performed, including shrink replays.
- id: FR-013-AC-1
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: A consumer fixture crate that includes the generated Rust source and whose
    manifest depends only on `proptest` and `quire-contract-runtime` compiles under
    denied warnings, draws `VersionUnchanged` cases, and reads the selected `versionNumber`
    StateField at `"pre"` and `"post"` through the generated typed-identity, path
    and observation constants.
- id: FR-013-AC-2
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated bundle contains no serialized case, census, or summary file,
    and the change adds no file under `schemas/`.
- id: FR-013-AC-4
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: The generated Rust header carries the selected checked clause id and authentic
    claim occurrence, and generating two distinct selected claims from one package
    produces different headers and artifact paths.
- id: FR-013-AC-5
  path: spec/strategy/functional/FR-013-it010-consumable-output.md
  role: examined
  excerpt: interface-001 `bound_strategy_slice.consumer` records the case, census,
    and runner surface a downstream consumer compiles against.
- id: NFR-004-AC-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Fresh 10,000-case and 256-case campaigns of every population over the V2
    selected claims `VersionUnchanged`, `amount < 7`, and a `Precondition` fixture
    record zero discards, zero proptest global rejects, and no `Exhausted` conclusion.
- id: NFR-004-AC-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: For every admitted fixture in TC-019, the in-domain census plus out-of-domain
    array holds at most 20 cases.
- id: NFR-004-M-1
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and
    fixture
- id: NFR-004-M-2
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Proptest global rejects on the same campaigns
- id: NFR-004-M-3
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config
    campaigns
- id: NFR-004-M-4
  path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  excerpt: In-domain plus out-of-domain census cases per admitted clause
bindings:
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_census_reports_every_construct_including_ones_after_an_early_refusal
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::profile_reader_rejects_legacy_inconclusive_before_census_admission
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::profile_refusals_preserve_original_causes_and_context_in_both_families
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_missing_construct_is_rejected_with_kani_capability_missing
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::empty_or_duplicate_construct_name_refuses_without_a_partial_census
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_still_prepares_exact_collection_and_graph_results
  ac_id: FR-015-AC-86
  trace: correct
- test_id: src/kani/generate/lower/mod.rs::tests::supported_profile_still_prepares_exact_collection_and_graph_results
  ac_id: FR-015-AC-88
  trace: correct
```

## Dispositions

Round1: verified actual current IR878 inventory has exactly these309finding criterion IDs and actual IR615 scoped decision byte-matches the supplied current decision. Fresh matrix on fix candidate retains all880criterion statements/methods/statuses, including exactly291untagged+18ignored-only. All309are explicitly DEFERRED individually to current owners. Full original gap verdict remains FAIL; initial findings unchanged. Prior305finding decision was not reused. No NFR006 gate or aggregate waiver.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | FR-005-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-002 | deferred | FR-039-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-003 | deferred | NFR-001-M-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-004 | deferred | NFR-002-M-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-005 | deferred | NFR-002-M-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-006 | deferred | FR-004-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-007 | deferred | FR-004-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-008 | deferred | FR-004-AC-10 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-009 | deferred | FR-004-AC-11 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-010 | deferred | FR-004-AC-12 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-011 | deferred | FR-004-AC-13 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-012 | deferred | FR-004-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-013 | deferred | FR-004-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-014 | deferred | FR-004-AC-18 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-015 | deferred | FR-036-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-016 | deferred | FR-036-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-017 | deferred | FR-036-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-018 | deferred | FR-036-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-019 | deferred | FR-036-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-020 | deferred | FR-036-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-021 | deferred | FR-036-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-022 | deferred | FR-015-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-023 | deferred | FR-015-AC-21 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-024 | deferred | FR-015-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-025 | deferred | FR-015-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-026 | deferred | FR-015-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-027 | deferred | FR-015-AC-30 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-028 | deferred | FR-015-AC-31 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-029 | deferred | FR-015-AC-32 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-030 | deferred | FR-015-AC-38 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-031 | deferred | FR-015-AC-39 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-032 | deferred | FR-015-AC-40 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-033 | deferred | FR-015-AC-41 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-034 | deferred | FR-015-AC-42 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-035 | deferred | FR-015-AC-43 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-036 | deferred | FR-015-AC-44 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-037 | deferred | FR-015-AC-45 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-038 | deferred | FR-015-AC-46 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-039 | deferred | FR-015-AC-47 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-040 | deferred | FR-015-AC-48 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-041 | deferred | FR-015-AC-49 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-042 | deferred | FR-015-AC-51 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-043 | deferred | FR-015-AC-52 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-044 | deferred | FR-015-AC-56 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-045 | deferred | FR-015-AC-57 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-046 | deferred | FR-015-AC-69 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-047 | deferred | FR-015-AC-70 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-048 | deferred | FR-015-AC-71 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-049 | deferred | FR-015-AC-72 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-050 | deferred | FR-015-AC-73 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-051 | deferred | FR-015-AC-74 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-052 | deferred | FR-015-AC-75 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-053 | deferred | FR-015-AC-76 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-054 | deferred | FR-015-AC-80 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-055 | deferred | FR-015-AC-84 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-056 | deferred | FR-015-AC-85 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-057 | deferred | FR-015-AC-87 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-058 | deferred | FR-015-AC-89 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-059 | deferred | FR-015-AC-90 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-060 | deferred | FR-015-AC-91 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-061 | deferred | FR-015-AC-92 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-062 | deferred | FR-015-AC-93 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-063 | deferred | FR-015-AC-95 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-064 | deferred | FR-015-AC-98 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-065 | deferred | FR-015-AC-105 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-066 | deferred | FR-015-AC-106 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-067 | deferred | FR-015-AC-107 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-068 | deferred | FR-015-AC-108 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-069 | deferred | FR-015-AC-110 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-070 | deferred | FR-015-AC-111 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-071 | deferred | FR-015-AC-117 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-072 | deferred | FR-015-AC-119 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-073 | deferred | FR-017-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-074 | deferred | FR-017-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-075 | deferred | FR-017-AC-26 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-076 | deferred | FR-017-AC-27 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-077 | deferred | FR-017-AC-30 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-078 | deferred | FR-017-AC-47 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-079 | deferred | FR-017-AC-52 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-080 | deferred | FR-017-AC-53 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-081 | deferred | FR-017-CON-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-082 | deferred | FR-025-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-083 | deferred | FR-025-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-084 | deferred | FR-025-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-085 | deferred | FR-025-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-086 | deferred | FR-025-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-087 | deferred | FR-025-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-088 | deferred | FR-025-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-089 | deferred | FR-025-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-090 | deferred | FR-025-AC-9 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-091 | deferred | FR-028-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-092 | deferred | FR-028-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-093 | deferred | FR-028-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-094 | deferred | FR-028-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-095 | deferred | FR-028-AC-9 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-096 | deferred | FR-028-AC-13 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-097 | deferred | FR-028-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-098 | deferred | FR-028-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-099 | deferred | FR-028-AC-16 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-100 | deferred | FR-028-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-101 | deferred | FR-028-AC-18 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-102 | deferred | FR-028-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-103 | deferred | FR-028-AC-20 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-104 | deferred | FR-028-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-105 | deferred | FR-028-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-106 | deferred | FR-028-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-107 | deferred | FR-029-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-108 | deferred | FR-029-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-109 | deferred | FR-029-AC-21 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-110 | deferred | FR-029-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-111 | deferred | FR-029-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-112 | deferred | FR-029-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-113 | deferred | FR-029-AC-25 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-114 | deferred | FR-029-AC-26 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-115 | deferred | FR-029-AC-27 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-116 | deferred | FR-029-AC-30 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-117 | deferred | FR-029-AC-31 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-118 | deferred | FR-030-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-119 | deferred | FR-030-AC-20 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-120 | deferred | FR-030-AC-21 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-121 | deferred | FR-030-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-122 | deferred | FR-034-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-123 | deferred | FR-034-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-124 | deferred | FR-034-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-125 | deferred | FR-034-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-126 | deferred | FR-034-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-127 | deferred | FR-034-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-128 | deferred | FR-034-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-129 | deferred | FR-034-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-130 | deferred | FR-034-AC-9 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-131 | deferred | FR-034-AC-10 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-132 | deferred | FR-034-AC-11 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-133 | deferred | FR-034-AC-12 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-134 | deferred | FR-034-AC-13 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-135 | deferred | FR-034-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-136 | deferred | FR-034-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-137 | deferred | FR-034-AC-16 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-138 | deferred | FR-034-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-139 | deferred | FR-034-AC-18 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-140 | deferred | FR-034-AC-19 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-141 | deferred | FR-034-AC-20 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-142 | deferred | FR-034-AC-21 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-143 | deferred | FR-034-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-144 | deferred | FR-034-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-145 | deferred | FR-034-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-146 | deferred | FR-034-AC-25 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-147 | deferred | FR-034-AC-26 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-148 | deferred | FR-034-AC-27 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-149 | deferred | FR-034-AC-28 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-150 | deferred | FR-034-AC-29 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-151 | deferred | FR-034-AC-31 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-152 | deferred | FR-034-AC-32 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-153 | deferred | FR-034-AC-33 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-154 | deferred | FR-034-AC-34 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-155 | deferred | FR-034-AC-35 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-156 | deferred | FR-034-AC-36 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-157 | deferred | FR-034-AC-37 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-158 | deferred | FR-034-AC-38 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-159 | deferred | FR-034-AC-39 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-160 | deferred | FR-034-AC-40 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-161 | deferred | FR-034-AC-51 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-162 | deferred | FR-034-AC-52 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-163 | deferred | FR-034-AC-53 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-164 | deferred | FR-034-AC-54 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-165 | deferred | FR-034-AC-57 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-166 | deferred | FR-034-AC-58 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-167 | deferred | FR-034-AC-59 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-168 | deferred | FR-034-AC-60 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-169 | deferred | FR-034-AC-61 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-170 | deferred | FR-034-AC-62 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-171 | deferred | FR-034-AC-63 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-172 | deferred | FR-034-AC-65 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-173 | deferred | FR-034-AC-66 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-174 | deferred | FR-034-AC-67 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-175 | deferred | FR-034-AC-69 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-176 | deferred | FR-034-AC-70 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-177 | deferred | FR-034-AC-71 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-178 | deferred | FR-034-AC-73 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-179 | deferred | FR-034-AC-74 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-180 | deferred | FR-034-AC-75 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-181 | deferred | FR-034-AC-76 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-182 | deferred | FR-034-AC-77 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-183 | deferred | FR-034-AC-78 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-184 | deferred | FR-034-AC-79 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-185 | deferred | FR-034-AC-80 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-186 | deferred | FR-034-AC-81 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-187 | deferred | FR-034-AC-82 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-188 | deferred | FR-034-AC-83 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-189 | deferred | FR-034-AC-84 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-190 | deferred | FR-034-AC-85 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-191 | deferred | FR-034-AC-86 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-192 | deferred | FR-034-AC-87 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-193 | deferred | FR-034-AC-88 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-194 | deferred | FR-034-AC-89 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-195 | deferred | FR-034-AC-90 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-196 | deferred | FR-034-AC-91 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-197 | deferred | FR-034-AC-92 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-198 | deferred | FR-034-AC-94 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-199 | deferred | FR-034-AC-95 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-200 | deferred | FR-034-AC-96 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-201 | deferred | FR-034-AC-97 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-202 | deferred | FR-034-AC-98 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-203 | deferred | FR-034-AC-99 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-204 | deferred | FR-034-AC-100 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-205 | deferred | FR-041-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-206 | deferred | NFR-006-M-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-207 | deferred | NFR-006-M-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-208 | deferred | NFR-006-M-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, titan:kani:plan and the assigned Kani proof queue, including parked IR17/scalar work, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-209 | deferred | FR-018-AC-21 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-210 | deferred | FR-018-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-211 | deferred | FR-018-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-212 | deferred | FR-018-AC-33 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-213 | deferred | FR-018-AC-34 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-214 | deferred | FR-018-AC-35 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-215 | deferred | FR-018-AC-36 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-216 | deferred | FR-018-AC-37 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-217 | deferred | FR-018-AC-38 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-218 | deferred | FR-018-AC-39 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-219 | deferred | FR-018-AC-40 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-220 | deferred | FR-018-AC-41 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-221 | deferred | FR-018-AC-42 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-222 | deferred | FR-018-AC-43 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-223 | deferred | FR-021-AC-30 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-224 | deferred | FR-031-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-225 | deferred | FR-031-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-226 | deferred | FR-031-AC-16 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-227 | deferred | FR-031-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-228 | deferred | FR-031-AC-27 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-229 | deferred | FR-031-AC-20 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-230 | deferred | FR-031-AC-22 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-231 | deferred | FR-031-AC-26 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-232 | deferred | FR-031-AC-23 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-233 | deferred | FR-031-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-234 | deferred | FR-031-AC-25 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-235 | deferred | FR-035-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-236 | deferred | FR-035-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-237 | deferred | FR-035-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-238 | deferred | FR-035-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-239 | deferred | FR-035-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-240 | deferred | FR-035-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-241 | deferred | FR-042-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-242 | deferred | FR-016-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-243 | deferred | FR-024-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-244 | deferred | FR-024-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-245 | deferred | FR-024-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-246 | deferred | FR-024-AC-18 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-247 | deferred | FR-024-AC-34 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-248 | deferred | FR-024-AC-35 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-249 | deferred | FR-024-AC-36 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-250 | deferred | FR-024-AC-37 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-251 | deferred | FR-024-AC-38 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-252 | deferred | FR-024-AC-39 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-253 | deferred | FR-024-AC-40 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-254 | deferred | FR-032-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-255 | deferred | FR-032-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-256 | deferred | FR-032-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-257 | deferred | FR-032-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-258 | deferred | FR-032-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-259 | deferred | FR-032-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-260 | deferred | FR-032-AC-9 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-261 | deferred | FR-032-AC-10 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-262 | deferred | FR-032-AC-11 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-263 | deferred | FR-032-AC-12 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-264 | deferred | FR-032-AC-13 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-265 | deferred | FR-032-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-266 | deferred | FR-033-AC-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-267 | deferred | FR-033-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-268 | deferred | FR-033-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-269 | deferred | FR-033-AC-5 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-270 | deferred | FR-033-AC-6 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-271 | deferred | FR-033-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-272 | deferred | FR-033-AC-10 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-273 | deferred | FR-033-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-274 | deferred | FR-033-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-275 | deferred | FR-033-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-276 | deferred | FR-019-AC-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-277 | deferred | FR-019-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-278 | deferred | FR-019-AC-24 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-279 | deferred | FR-019-AC-34 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-280 | deferred | FR-019-AC-35 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-281 | deferred | FR-019-AC-36 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-282 | deferred | FR-019-AC-37 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-283 | deferred | FR-019-AC-38 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-284 | deferred | FR-019-AC-39 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-285 | deferred | FR-019-AC-40 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-286 | deferred | FR-019-AC-41 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-287 | deferred | FR-022-AC-26 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-288 | deferred | FR-022-AC-27 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-289 | deferred | FR-022-AC-28 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-290 | deferred | FR-026-AC-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-291 | deferred | FR-026-AC-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-292 | deferred | FR-002-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-293 | deferred | FR-002-AC-8 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-294 | deferred | FR-002-AC-10 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-295 | deferred | FR-002-AC-11 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-296 | deferred | FR-002-AC-12 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-297 | deferred | FR-002-AC-13 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-298 | deferred | FR-002-AC-14 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-299 | deferred | FR-002-AC-15 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-300 | deferred | FR-002-AC-16 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-301 | deferred | FR-002-AC-17 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-302 | deferred | FR-002-AC-18 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-303 | deferred | FR-008-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-304 | deferred | FR-008-AC-9 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-305 | deferred | FR-009-AC-7 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-306 | deferred | NFR-004-M-1 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-307 | deferred | NFR-004-M-2 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-308 | deferred | NFR-004-M-3 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
| FND-309 | deferred | NFR-004-M-4 is unchanged current-main debt outside IR615; explicitly deferred to existing IR878, luna:IR:plan for non-Kani strict-coverage triage, verified current inventory handoff 3db226b2-9381-476f-a880-cf37f082ac9e and scoped decision 37eaa49e-f02d-4e08-b567-035c4fc672f4. It remains unqualified; no completed proof or repository PASS is claimed. |
