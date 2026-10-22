---
id: SR-15350
title: IR-705 bounded multiplication gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; IR-705; spec/kani/functional/FR-015-bounded-kani-obligations.md;
  spec/assurance/AD-006-admitted-native-integer-oracles.md
review_set: subset
---

## Summary

Independent source-only review of PR506 exact a3e249c5181e4d61cd0a8ef7d25a95001050f2ef. Whole-repository fresh computed matrix reconciliation: 896 criteria, 523 tagged, 303 untagged, 18 tagged only by ignored tests, 52 method-without-symbol. All 321 criterion gaps are retained individually. Two inherited actual quantity source-port defects and three actual stale criterion attributes remain. A fresh raw attribute audit excludes prior false positives from a plain FR322 documentation reference and a non-tagged AD004 mention; only actual criterion-binding attributes are classified as stale. The broader coverage unmatched-token census is reported as raw tool data, not 123 proven defects. No production or test source differs from the reviewed base. No new arithmetic selection implementation or proof is delivered by this specification amendment.

## Verdict

FAIL — Assurance/implementation evidence remains incomplete. Specification-first amendment does not deliver or waive these gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-005-cli-conformance.md:49 |
| FND-002 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-039-contract-package-consumption.md:213 |
| FND-003 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:31 |
| FND-004 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:32 |
| FND-005 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:33 |
| FND-006 | high | Inherited criterion has no tagged test. | spec/core/functional/FR-043-owned-lock-coherence-gate.md:35 |
| FND-007 | high | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-001-reproducible-atomic.md:34 |
| FND-008 | high | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-002-provenance-boundary.md:50 |
| FND-009 | high | Inherited criterion has no tagged test. | spec/core/non-functional/NFR-002-provenance-boundary.md:52 |
| FND-010 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:186 |
| FND-011 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:188 |
| FND-012 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:190 |
| FND-013 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:191 |
| FND-014 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:192 |
| FND-015 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:193 |
| FND-016 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:194 |
| FND-017 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:195 |
| FND-018 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-004-vacuity-evidence.md:198 |
| FND-019 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:140 |
| FND-020 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:141 |
| FND-021 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:142 |
| FND-022 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:143 |
| FND-023 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:144 |
| FND-024 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:145 |
| FND-025 | high | Inherited criterion has no tagged test. | spec/evidence/functional/FR-036-source-obligation-census.md:146 |
| FND-026 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:763 |
| FND-027 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:765 |
| FND-028 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:766 |
| FND-029 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:767 |
| FND-030 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:768 |
| FND-031 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:774 |
| FND-032 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:775 |
| FND-033 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:776 |
| FND-034 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:782 |
| FND-035 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:783 |
| FND-036 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:784 |
| FND-037 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:785 |
| FND-038 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:786 |
| FND-039 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:787 |
| FND-040 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:788 |
| FND-041 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:789 |
| FND-042 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:790 |
| FND-043 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:791 |
| FND-044 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:792 |
| FND-045 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:793 |
| FND-046 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:795 |
| FND-047 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:796 |
| FND-048 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:800 |
| FND-049 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:801 |
| FND-050 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:813 |
| FND-051 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:814 |
| FND-052 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:815 |
| FND-053 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:816 |
| FND-054 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:817 |
| FND-055 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:818 |
| FND-056 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:819 |
| FND-057 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:820 |
| FND-058 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:824 |
| FND-059 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:828 |
| FND-060 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:829 |
| FND-061 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:831 |
| FND-062 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:833 |
| FND-063 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:834 |
| FND-064 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:835 |
| FND-065 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:836 |
| FND-066 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:837 |
| FND-067 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:839 |
| FND-068 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:842 |
| FND-069 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:849 |
| FND-070 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:850 |
| FND-071 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:851 |
| FND-072 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:852 |
| FND-073 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:854 |
| FND-074 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:855 |
| FND-075 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-015-bounded-kani-obligations.md:861 |
| FND-076 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:863 |
| FND-077 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:864 |
| FND-078 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:865 |
| FND-079 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 |
| FND-080 | high | Prospective criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:869 |
| FND-081 | high | Prospective criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:870 |
| FND-082 | high | Prospective criterion has no tagged implementation or qualification test. | spec/kani/functional/FR-015-bounded-kani-obligations.md:871 |
| FND-083 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md:859 |
| FND-084 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:868 |
| FND-085 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:875 |
| FND-086 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md:876 |
| FND-087 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md:879 |
| FND-088 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md:896 |
| FND-089 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:901 |
| FND-090 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:902 |
| FND-091 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:903 |
| FND-092 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-017-kani-execution-evidence.md:904 |
| FND-093 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-017-kani-execution-evidence.md:848 |
| FND-094 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:94 |
| FND-095 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:95 |
| FND-096 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:96 |
| FND-097 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:97 |
| FND-098 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:98 |
| FND-099 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:99 |
| FND-100 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:100 |
| FND-101 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:101 |
| FND-102 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-025-generated-subject-abi.md:102 |
| FND-103 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:206 |
| FND-104 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:207 |
| FND-105 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:208 |
| FND-106 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:209 |
| FND-107 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:210 |
| FND-108 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:212 |
| FND-109 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:213 |
| FND-110 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:214 |
| FND-111 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:215 |
| FND-112 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:216 |
| FND-113 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:217 |
| FND-114 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:218 |
| FND-115 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:219 |
| FND-116 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:221 |
| FND-117 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:222 |
| FND-118 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:223 |
| FND-119 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:377 |
| FND-120 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:379 |
| FND-121 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:381 |
| FND-122 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:382 |
| FND-123 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:383 |
| FND-124 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:384 |
| FND-125 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:385 |
| FND-126 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:386 |
| FND-127 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:387 |
| FND-128 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:390 |
| FND-129 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:391 |
| FND-130 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:237 |
| FND-131 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:238 |
| FND-132 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:239 |
| FND-133 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:240 |
| FND-134 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1844 |
| FND-135 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1845 |
| FND-136 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1846 |
| FND-137 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1847 |
| FND-138 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1848 |
| FND-139 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1849 |
| FND-140 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1850 |
| FND-141 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1851 |
| FND-142 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1852 |
| FND-143 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1853 |
| FND-144 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1854 |
| FND-145 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1855 |
| FND-146 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1856 |
| FND-147 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1857 |
| FND-148 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1858 |
| FND-149 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1859 |
| FND-150 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1860 |
| FND-151 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1861 |
| FND-152 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1862 |
| FND-153 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1863 |
| FND-154 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1864 |
| FND-155 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1865 |
| FND-156 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1866 |
| FND-157 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1867 |
| FND-158 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1868 |
| FND-159 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1869 |
| FND-160 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1870 |
| FND-161 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1871 |
| FND-162 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1872 |
| FND-163 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1874 |
| FND-164 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1875 |
| FND-165 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1876 |
| FND-166 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1877 |
| FND-167 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1878 |
| FND-168 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1879 |
| FND-169 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1880 |
| FND-170 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1881 |
| FND-171 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1882 |
| FND-172 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1883 |
| FND-173 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1894 |
| FND-174 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1895 |
| FND-175 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1896 |
| FND-176 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1897 |
| FND-177 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1900 |
| FND-178 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1901 |
| FND-179 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1902 |
| FND-180 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1903 |
| FND-181 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1904 |
| FND-182 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1905 |
| FND-183 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1906 |
| FND-184 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1908 |
| FND-185 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1909 |
| FND-186 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1910 |
| FND-187 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1912 |
| FND-188 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1913 |
| FND-189 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1914 |
| FND-190 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1916 |
| FND-191 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1917 |
| FND-192 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1918 |
| FND-193 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1919 |
| FND-194 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1920 |
| FND-195 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1921 |
| FND-196 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1922 |
| FND-197 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1923 |
| FND-198 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1924 |
| FND-199 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1925 |
| FND-200 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1926 |
| FND-201 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1927 |
| FND-202 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1928 |
| FND-203 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1929 |
| FND-204 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1930 |
| FND-205 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1931 |
| FND-206 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1932 |
| FND-207 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1933 |
| FND-208 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1934 |
| FND-209 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1935 |
| FND-210 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1937 |
| FND-211 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1938 |
| FND-212 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1939 |
| FND-213 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1940 |
| FND-214 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1941 |
| FND-215 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1942 |
| FND-216 | high | Inherited criterion has no tagged test. | spec/kani/functional/FR-034-caller-death-ownership.md:1943 |
| FND-217 | high | Inherited criterion has only ignored-test bindings. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:69 |
| FND-218 | high | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:171 |
| FND-219 | high | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:172 |
| FND-220 | high | Inherited criterion has no tagged test. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:173 |
| FND-221 | high | New quantity ownership criterion has no tagged implementation or agreement test; renderer still uses Runtime-owned quantities and drops arithmetic metadata. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:560 |
| FND-222 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:631 |
| FND-223 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:632 |
| FND-224 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:633 |
| FND-225 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:643 |
| FND-226 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:644 |
| FND-227 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:645 |
| FND-228 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:646 |
| FND-229 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:647 |
| FND-230 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:648 |
| FND-231 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:649 |
| FND-232 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:650 |
| FND-233 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:651 |
| FND-234 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:652 |
| FND-235 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-018-composite-equality-oracles.md:653 |
| FND-236 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-021-function-application-oracles.md:277 |
| FND-237 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:504 |
| FND-238 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:508 |
| FND-239 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:518 |
| FND-240 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:519 |
| FND-241 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:521 |
| FND-242 | high | Inherited criterion has only ignored-test bindings. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:524 |
| FND-243 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:526 |
| FND-244 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:527 |
| FND-245 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:528 |
| FND-246 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:529 |
| FND-247 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:530 |
| FND-248 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:107 |
| FND-249 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:108 |
| FND-250 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:109 |
| FND-251 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:110 |
| FND-252 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:111 |
| FND-253 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-035-caller-text-admission.md:112 |
| FND-254 | high | Inherited criterion has no tagged test. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:31 |
| FND-255 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:347 |
| FND-256 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:352 |
| FND-257 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:353 |
| FND-258 | high | Inherited criterion has only ignored-test bindings. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:362 |
| FND-259 | high | Inherited criterion has only ignored-test bindings. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:378 |
| FND-260 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:379 |
| FND-261 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:380 |
| FND-262 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:381 |
| FND-263 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:382 |
| FND-264 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:383 |
| FND-265 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-024-counterexample-envelope-intake.md:384 |
| FND-266 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:382 |
| FND-267 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:385 |
| FND-268 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:386 |
| FND-269 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:387 |
| FND-270 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:388 |
| FND-271 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:389 |
| FND-272 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:390 |
| FND-273 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:391 |
| FND-274 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:392 |
| FND-275 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:393 |
| FND-276 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:394 |
| FND-277 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:395 |
| FND-278 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:362 |
| FND-279 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:363 |
| FND-280 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:364 |
| FND-281 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:365 |
| FND-282 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:366 |
| FND-283 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:368 |
| FND-284 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:370 |
| FND-285 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:374 |
| FND-286 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:375 |
| FND-287 | high | Inherited criterion has no tagged test. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:377 |
| FND-288 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:519 |
| FND-289 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:547 |
| FND-290 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:548 |
| FND-291 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:558 |
| FND-292 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:559 |
| FND-293 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:560 |
| FND-294 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:561 |
| FND-295 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:562 |
| FND-296 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:563 |
| FND-297 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:564 |
| FND-298 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-019-capability-settlement.md:565 |
| FND-299 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md:361 |
| FND-300 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md:362 |
| FND-301 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-022-routed-generation.md:363 |
| FND-302 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-026-backend-adapter-contract.md:60 |
| FND-303 | high | Inherited criterion has no tagged test. | spec/routed/functional/FR-026-backend-adapter-contract.md:61 |
| FND-304 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:91 |
| FND-305 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:92 |
| FND-306 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:94 |
| FND-307 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:95 |
| FND-308 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:96 |
| FND-309 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:97 |
| FND-310 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:98 |
| FND-311 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:99 |
| FND-312 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:100 |
| FND-313 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:101 |
| FND-314 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-002-tristate-proptest.md:102 |
| FND-315 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:123 |
| FND-316 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:125 |
| FND-317 | high | Inherited criterion has no tagged test. | spec/strategy/functional/FR-009-constructive-correlated-populations.md:77 |
| FND-318 | high | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:43 |
| FND-319 | high | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:44 |
| FND-320 | high | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:45 |
| FND-321 | high | Inherited criterion has no tagged test. | spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md:46 |
| FND-322 | high | Existing conversion renderer and agreement adapter use &rt::QuantityUnit and Runtime quantities, so tagged tests do not establish the amended IdentifiedUnit target and checked-id handoff. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:556 |
| FND-323 | high | Existing scalar renderer still imports the retired Runtime exact facade; tagged family-generation tests do not establish the amended matching direct-owner calls. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:511 |
| FND-324 | medium | Inherited test tag FR-049-AC-1 names no computed criterion. | src/routed/contract_package_tests.rs:1667 |
| FND-325 | medium | Inherited test tag FR-049-AC-5 names no computed criterion. | src/routed/contract_package_tests.rs:1667 |
| FND-326 | medium | Inherited test tag FR-049-AC-1 names no computed criterion. | src/routed/contract_package_tests.rs:1582 |

## Examined units

```yaml
scope:
- path: spec/core/functional/FR-005-cli-conformance.md
  role: examined
  id: FR-005-AC-3
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Different
- path: spec/core/functional/FR-039-contract-package-consumption.md
  role: examined
  id: FR-039-AC-4
  excerpt: 'For a genuinely admitted FR-330 package, CG binds a `protocol`/`protocol_reference`
    carrier''s exact '
- path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  id: FR-043-AC-1
  excerpt: The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock`
    command on this rep
- path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  id: FR-043-AC-2
  excerpt: Through the same normal `make deny` recipe, a scratch lock with duplicate
    first-party Git package na
- path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  id: FR-043-AC-3
  excerpt: A missing checker executable, malformed scratch lock, or checker refusal
    cannot produce successful `
- path: spec/core/functional/FR-043-owned-lock-coherence-gate.md
  role: examined
  id: FR-043-AC-5
  excerpt: Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged
    and adds no runtime d
- path: spec/core/non-functional/NFR-001-reproducible-atomic.md
  role: examined
  id: NFR-001-M-1
  excerpt: Byte differences between repeated generations
- path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  id: NFR-002-M-1
  excerpt: Generated Rust files missing generator or input-carried subject identity
- path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: examined
  id: NFR-002-M-3
  excerpt: Silent unsupported/inconclusive states
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-6
  excerpt: Campaign counts and test outcome remain complete facts independent of coverage
    classification.
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-8
  excerpt: Adverse coverage and non-success native execution cannot discharge the
    coverage obligation merely be
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-10
  excerpt: 'PLANNED. CG binds coverage inside the same live campaign-runner invocation,
    using the original `/2` '
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-11
  excerpt: PLANNED. The consuming report retains vacuity and native outcome as evidence
    for IR FR-045; an oracl
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-12
  excerpt: PLANNED. CG consumes QSL's typed strict-reader clause-run or command-error
    variant for exact `native
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-13
  excerpt: PLANNED. QSL `run_clause` success and a decisive witness remain source-clause
    facts and do not count
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-14
  excerpt: 'PLANNED. Only CG''s campaign runner constructs the private, non-cloneable,
    non-serializable live run '
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-15
  excerpt: PLANNED. A controlled attempt to replace the strict-reader typed result,
    execution record, generated
- path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  id: FR-004-AC-18
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication
    count unequal to th
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-1
  excerpt: A genuine typed fixture with criteria C1/C2, obligations O1/O2/O3, source
    identities S1/S2, domain D
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-2
  excerpt: For the unchanged AC-1 fixture, omitting a result, reporting unsupported
    or pre-run refusal, retryin
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-3
  excerpt: Capability facts preserve unsupported capability and reason without artifact/run/outcome;
    pre-run re
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-4
  excerpt: 'A matching executed production fact retains its actual typed artifact
    and run identities separately '
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-5
  excerpt: Shadow-only evidence, a narrower effective bound and separately executed
    sampled native refinement r
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-6
  excerpt: A downstream consumer above CG and IR borrows the public census/fact types
    over the reviewed neutral
- path: spec/evidence/functional/FR-036-source-obligation-census.md
  role: examined
  id: FR-036-AC-7
  excerpt: Missing/incomplete authoritative inputs, a relation targeting a foreign
    selection and a dangling rel
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-19
  excerpt: A zero-input transition over one bounded-integer state value verifies for
    an identity subject and is
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-21
  excerpt: 'A precondition that reads a post-state value is refused with a typed reason
    naming the node, and no '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-22
  excerpt: 'A declared proof-dependency census with an empty or duplicate identity,
    an inconsistent kind, state '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-23
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`,
    `requires-bound`, `u
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-24
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison
    claim is reported through Ka
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-30
  excerpt: 'With the installed backend, the operation contract verifies for a healthy
    subject and is falsified, '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-31
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden
    effect is falsified n
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-32
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a
    frame violation of that field
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-38
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `oper
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-39
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state fi'
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-40
  excerpt: 'A harness for a V2 clause whose condition, inline or referenced from an
    admitted Boolean expression '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-41
  excerpt: A V2 clause whose body holds integer arithmetic (add, subtract, multiply,
    negate) as an operand of a
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-42
  excerpt: 'A V2 clause body holding an operator the V2 arm does not support (division,
    modulo, absolute value, '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-43
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing
    its anchor node as a `requ
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-44
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency
    identity, an inconsiste
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-45
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's
    identity record, readines
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-46
  excerpt: Every V2 clause harness contains exactly one non-vacuity cover, placed
    after every assumption, after
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-47
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical,
    and changing the unwind bou
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-48
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from
    the clause node id, its oc
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-49
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters
    within their declared doma
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-51
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corp
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-52
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corp
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-56
  excerpt: With the installed backend, a V1 bundle harness whose requires clause some
    bounded argument satisfie
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-57
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed
    backend, a bounded-corpus
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-69
  excerpt: A composite-equality claim over operands that are parameters or literals
    of one record, tuple or opt
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-70
  excerpt: 'The harness draws one nondeterministic value per leaf of each parameter
    operand and no other value: '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-71
  excerpt: Over every drawn pair the harness asserts that the shadow's verdict equals
    the expectation's and tha
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-72
  excerpt: The emitted shadow and harness, parsed, contain no loop (`while`, `loop`,
    `for`), no recursive call,
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-73
  excerpt: 'Each row of the unsupported-shape table above is produced by an item reaching
    that shape and yields '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-74
  excerpt: With the installed backend, the harness of the corpus case (two record-typed
    and two tuple-typed val
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-75
  excerpt: 'With the installed backend each seeded mutant of the shadow is `Falsified`
    with a concrete playback:'
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-76
  excerpt: The identity of a composite-equality harness records family `composite_equality`,
    proof subject `bou
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-80
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-84
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, a profile/di
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-85
  excerpt: 'Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic
    lowerer, an inverted '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-87
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer,
    a zero expansion bound or un
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-89
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection
    lowerer, an input longer th
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-90
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-91
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corp
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-92
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm
    ends at its own slice, FR-015
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-93
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes
    the hand-built bounded-corp
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-95
  excerpt: AD-004 step 4g retires the hand-built bounded-corpus path in the three
    per-family slices FR-015-AC-1
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-98
  excerpt: 'A graph claim (one reachability application of either form above, at its
    `state_clause` node id and '
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-105
  excerpt: For a graph claim the harness draws an object count in `[0, N]` of the
    one covering population and s
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-106
  excerpt: The independent native oracle replays the same authored clause over each
    original-domain concrete gr
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-107
  excerpt: 'Graph claims are located by `state_clause` node id and argument path:
    one reachability written twice'
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-108
  excerpt: A graph claim settles by the disposition sequence in the Description (the
    paragraph "Disposition seq
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-110
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and
    on AD-004 step 4h, because th
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-111
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b,
    on FR-015-AC-109 and FR-015-
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-117
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port,
    the unmutated scalar harnes
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-119
  excerpt: PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source
    authors an operation with one
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-120
  excerpt: PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After
    the selected owner and occu
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-121
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned
    `StepSimulationRecord` as a
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-122
  excerpt: PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88
    through AC-90 with a
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-125
  excerpt: Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs
    generate kernel-owned na
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-126
  excerpt: Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917
    clause implication and IR-
- path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  id: FR-015-AC-127
  excerpt: End-to-end denied/cancelled admitted-native controls retain all real Meter
    counters, charge order, c
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-7
  excerpt: A crate whose library source does not contain the harness source byte for
    byte is refused, and no ba
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-19
  excerpt: The generator appends `-Z unstable-options --export-json /proc/self/fd/N`
    after unchanged harness op
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-26
  excerpt: A generated FR-035 caller Text-admission harness enters the same production
    execution path in single
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-27
  excerpt: 'A genuine routed `x + 1` scalar harness from `generate_routed`, its complete
    emitted oracle package '
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-30
  excerpt: 'The public receipt exposes the measured identity, scalar identity, ordered
    operands, exact supplied '
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-47
  excerpt: 'Two receipts for the same package and harness with different operands,
    different `ScalarLimits` and '
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-52
  excerpt: Production emitted bytes and feature-free exact dependency remain unchanged.
    The isolated private ob
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-53
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/u
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-54
  excerpt: A successful claimed-init confirmation poll with no `IN` yields public
    `MemoryObservationFailed` wit
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-AC-55
  excerpt: Through the real public execution entry and real owned namespace, an unchanged
    real-poll control ret
- path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  id: FR-017-CON-1
  excerpt: The generator SHALL NOT convert a non-verified outcome into a proof claim.
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-1
  excerpt: For every harness kind, the identifiers of the emitted `kani::any()` bindings,
    in source order, equa
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-2
  excerpt: Every argument binding in a persisted identity names its parameter node
    id and declared domain.
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-3
  excerpt: 'A Boolean argument is a `bool` that reaches the oracle unchanged, and
    a bounded-integer argument is '
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-4
  excerpt: An argument whose domain has no declared finite bound yields no harness
    and is never narrowed to a m
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-5
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no
    harness.
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-6
  excerpt: A binding that names no parameter of the selected function, and a parameter
    of that function with no
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-7
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal,
    IEEE, text, enum, com
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-8
  excerpt: A harness over a state-reading obligation passes each state value to the
    subject as `&mut` to a harn
- path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  id: FR-025-AC-9
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its
    original operand/parameter
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-5
  excerpt: Every harness identity, generation refusal and execution evidence names
    its family.
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-6
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-7
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow`
    and its refinem
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-8
  excerpt: 'An argument narrowed inside its declared domain records the declared domain,
    the narrowed bound and '
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-9
  excerpt: A narrowing outside the argument's declared domain is refused with a typed
    reason and no harness.
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-13
  excerpt: The source of a `bounded_shadow` harness, parsed, names no crate but `core`,
    `alloc`, `std` and `kan
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-14
  excerpt: The result of a `bounded_shadow` family carries a refinement obligation,
    as generated Rust source in
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-15
  excerpt: For every case the refinement run executes, the production subject's complete
    outcome under an unlim
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-16
  excerpt: A refinement run over a domain of at most the request's case cap cases
    executes every case and is cl
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-17
  excerpt: A verified `production` harness reports `production_proved`; a verified
    `bounded_shadow` harness rep
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-18
  excerpt: A harness identity of a shadow family lists each abstraction the shadow
    makes by name with the evide
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-19
  excerpt: A refinement domain that does not contain every case the shadow harness's
    assumptions admit (a bound
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-20
  excerpt: 'An item whose shape lies outside its family''s shadow support is refused:
    a missing bound as `require'
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-22
  excerpt: 'A shadow family''s machinery catches each seeded mutant: a shadow harness
    over a mutated shadow is `F'
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-23
  excerpt: Every execution evidence records the Kani version, the Rust compiler version,
    the solver name and ve
- path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  id: FR-028-AC-24
  excerpt: The refinement run is a native process launched through one typed refinement
    execution entry that re
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-17
  excerpt: PLANNED (IR-806). IR-635's Eq-only verified constructor invokes QSL with
    supplied evidence; the clos
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-19
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule,
    verified shadow CeilingRe
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-21
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength
    input and `TerminalPai'
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-22
  excerpt: 'PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17''s
    closed set; adding a '
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-23
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary
    timeout/memory inconclusi
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-24
  excerpt: PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09
    validation, retained refi
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-25
  excerpt: PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow
    inconclusive reason
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-26
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL
    record category/cause: Proved'
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-27
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement
    input refuses a missing sett
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-30
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive
    or cover-unsatisfied
- path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  id: FR-029-AC-31
  excerpt: PLANNED (IR-806). The parity route fabricates no predicate replay and no
    terminal cause for a bounde
- path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  id: FR-030-AC-19
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT
    proof whose certificate FR-31
- path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  id: FR-030-AC-20
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid
    checked proof step or missin
- path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  id: FR-030-AC-21
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts
    and settles inconclus
- path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  id: FR-030-AC-22
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces
    one terminal result, never
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-1
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer,
    observer re
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-2
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death
    protection and orig
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-3
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF
    causes guardian cancellation
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-4
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks
    the actual mapped creator'
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-5
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through
    host pidfds, start/parent/n
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-6
  excerpt: 'The owned tree memory observer is ready before typed Dispatch; failed
    observer preparation releases '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-7
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M
    death/reparenting or malforme
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-8
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid,
    late-born and nested-P
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-9
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated
    host children unaffe
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-10
  excerpt: Normal backend completion, backend exec failure, explicit cancellation,
    timeout, memory excess, capt
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-11
  excerpt: Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady
    or immediately after Dis
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-12
  excerpt: Every startup refusal, completion and cancellation settles actual owned
    roles before conclusions. An
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-13
  excerpt: The bounded executor requires an explicit path to this package's actual
    Cargo executable quire-kani-
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-14
  excerpt: Setup docs require the matched package helper roles, safe caller-context
    user/PID/private-mount perm
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-15
  excerpt: Typed private controls reject malformed and unknown fields and enforce
    finite encoded-byte, pending-
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-16
  excerpt: 'Only the original caller holds the pair''s executor endpoint. Guardian
    receives only its control end '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-17
  excerpt: Backend receives unchanged raw argument bytes except the explicitly allocated
    internal report locato
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-18
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr
    captures and canno
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-19
  excerpt: Original identity ceilings and existing outcome classifications remain
    authoritative after confirmed
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-20
  excerpt: Guardian connection, startup, identity verification and Dispatch use the
    original monotonic identity
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-21
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining
    original deadline. Ca
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-22
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds.
    Unconfirmed termination r
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-23
  excerpt: Caller-death fixtures use the actual matching normal helper and shared
    production functions/order wi
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-24
  excerpt: Actual lease-EOF cancellation passes independent original I termination
    and no backend Dispatch/mark
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-25
  excerpt: Before backend Dispatch, the helper handshake matches the actual running
    CG library's build, protoco
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-26
  excerpt: The guardian session/group is distinct from the original caller before
    Ready/Dispatch. Real feature-
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-27
  excerpt: guardian-test-support is off by default, absent from default features,
    and exposes exactly one docum
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-28
  excerpt: Fixture prefix selects unchanged production transitions, never certifies
    live state. Actual original
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-29
  excerpt: The packaged caller fixture and real helper link the same normal library
    artifact through consumer-m
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-31
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-32
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without
    a backend/collector compl
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-33
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner
    teardown and M termination/re
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-34
  excerpt: Live-C production close_lease_and_observe retains L/O/M/I ownership and
    final report control indepen
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-35
  excerpt: PLANNED/UNRUN. L creates private network/root before O, O validates before
    M, and nested bind / / re
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-36
  excerpt: PLANNED/UNRUN. C inventories actual backend fd0/fd1/fd2. Real OriginalStdin::Open
    socket input and f
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-37
  excerpt: PLANNED/UNRUN. Trusted I retains its exclusive lease through Dispatch;
    O retains separate final deli
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-38
  excerpt: 'PLANNED/UNRUN. Named fixed SETTLE_RESERVE R is measured/rounded; its finite
    effective reserve stays '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-39
  excerpt: 'PLANNED/UNRUN. Native guardian/backend-installer policy support is source/cfg
    Analysis; unreachable '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-40
  excerpt: 'PLANNED/UNRUN. Only declared trusted role/operation/phase causes use the
    cross-role representation. '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-51
  excerpt: PLANNED/UNRUN. Given genuine live-I construction preconditions, the child's
    own acknowledgement esta
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-52
  excerpt: PLANNED/UNRUN. Actual retained-I pidfd termination confirmation strictly
    precedes matching kernel re
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-53
  excerpt: 'PLANNED/UNRUN. Actual retained M consuming Child wait/reap strictly precedes
    matching kernel report '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-54
  excerpt: PLANNED/UNRUN. On the independently grounded supported cooperative cancellation
    path, the actual ori
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-57
  excerpt: "PLANNED/UNRUN (IR-687). When L awaits its first O arm result, the newly\
    \ allocated authenticated O\u2192L "
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-58
  excerpt: PLANNED/UNRUN (IR-687). L accepts that negative only from its actual retained
    O Child PID with kerne
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-59
  excerpt: "PLANNED/UNRUN (IR-687). For L\u2192C AwaitArm negatives, C rejects missing,\
    \ extra, wrong-type or substitu"
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-60
  excerpt: PLANNED/UNRUN (IR-687). The negative O pidfd grants C only exact O termination
    observation and exist
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-61
  excerpt: PLANNED/UNRUN (IR-687). Negative capability custody grants no Armed, namespace/network,
    phase, repor
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-62
  excerpt: PLANNED/UNRUN (IR-687). C adopts the authentic O producer stop before L
    wait and preserves the origi
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-63
  excerpt: PLANNED/UNRUN (IR-687). C exposes a negative original-cause result only
    after actual O Child wait at
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-65
  excerpt: 'PLANNED/UNRUN (IR-687). Before first-byte COMMIT, a fresh complete exhausting
    pre-send tick retires '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-66
  excerpt: PLANNED/UNRUN (IR-687). Only the allocated L-local pre-arm negative route
    forwards the same bytes to
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-67
  excerpt: PLANNED/UNRUN (IR-687). C authenticates the L-forwarded negative only in
    AwaitArm through the actual
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-69
  excerpt: PLANNED/UNRUN (IR-687). Missing or abnormal whole-chain settlement overrides
    the provisional negativ
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-70
  excerpt: PLANNED/UNRUN (IR-687). The first emitted terminal byte irreversibly fixes
    the candidate; partial/fu
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-71
  excerpt: PLANNED/UNRUN (IR-687). O includes every due observation and a genuine
    fresh complete named-input ti
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-73
  excerpt: PLANNED/UNRUN (IR-687). A claimed-startup negative publication remains
    provisional until positive ac
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-74
  excerpt: "PLANNED/UNRUN (IR-687). The negative\u2019s absent peak field supplies\
    \ no zero measurement, no-history cl"
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-75
  excerpt: PLANNED/UNRUN (IR-687). A later genuine observation/collector/transport
    error after COMMIT remains o
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-76
  excerpt: PLANNED/UNRUN (IR-687). A genuine admitted open-file-description Test oracle
    distinguishes L's retai
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-77
  excerpt: PLANNED/UNRUN (IR-687). On a fresh complete Exhausted tick after first-byte
    COMMIT, O immediately in
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-78
  excerpt: PLANNED/UNRUN. C admits actual original O Armed only through genuine retained
    O/build/run authentica
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-79
  excerpt: PLANNED/UNRUN. Actual production control reception rejects malformed, partial
    or forbidden-right tra
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-80
  excerpt: 'PLANNED/UNRUN. Genuine original authenticated I Completed alone establishes
    completion reception; M '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-81
  excerpt: PLANNED/UNRUN. Every surviving actual simultaneous named fixture/control
    storage and pipe/backing re
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-82
  excerpt: PLANNED/UNRUN. Actual supported-source/artifact Analysis and mechanical
    paired consumer/helper check
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-83
  excerpt: PLANNED/UNRUN. The original exclusive caller lease closes independently
    of retained monitor/INIT own
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-84
  excerpt: PLANNED/UNRUN. Every remaining fixture reporter/control/report endpoint
    is excluded from untrusted c
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-85
  excerpt: PLANNED/UNRUN. Actual retained M consuming reap is not established by repeated
    or cached Child Some(
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-86
  excerpt: PLANNED/UNRUN. O establishes same-M parent-only prior-WNOWAIT/FIRST-uncached-Some/post-ECHILD
    proven
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-87
  excerpt: 'PLANNED/UNRUN. Actual I confirmation/M consuming reap precede report seal.
    On AC-54''s independently '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-88
  excerpt: 'PLANNED/UNRUN. O preserves every due ordinary sample and existing control/cutoff
    transition without '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-89
  excerpt: PLANNED/UNRUN. Genuine original I Completed reception and original stop
    adoption precede actual orig
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-90
  excerpt: PLANNED/UNRUN. Any remaining optional fixture write failure does not terminate
    O or alter ordinary s
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-91
  excerpt: 'PLANNED/UNRUN. Ordinary feature-on execution remains available without
    retired observation binding, '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-92
  excerpt: PLANNED/UNRUN. Genuine adopted-orphan acknowledgement and release-dependent
    child birth after an act
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-94
  excerpt: 'PLANNED/UNRUN. C disposes the defined pending claim through actual close
    of its raw I right without '
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-95
  excerpt: PLANNED/UNRUN. Actual O retains the original report backing identity through
    collection, kernel seal
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-96
  excerpt: PLANNED/UNRUN. Missing, malformed, wrong or unvalidated original O/build/run/control
    authority refus
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-97
  excerpt: PLANNED/UNRUN. Retired observation-binding echo creates no replacement
    comparison; actual original r
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-98
  excerpt: PLANNED/UNRUN. Feature-off paired consumers/helpers retain unchanged phase
    bytes/rights with no reti
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-99
  excerpt: PLANNED/UNRUN. Any remaining optional O fixture write requires actual ignored/unblocked/untraced
    sta
- path: spec/kani/functional/FR-034-caller-death-ownership.md
  role: examined
  id: FR-034-AC-100
  excerpt: PLANNED/UNRUN. Unsupported, malformed, unavailable or unproved profiles
    permit no optional fixture w
- path: spec/kani/functional/FR-041-claimed-module-proof-gate.md
  role: examined
  id: FR-041-AC-4
  excerpt: Each claimed module and computed shared helper has a positive unmodified
    real-Kani proof and a sourc
- path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  id: NFR-006-M-1
  excerpt: Merged pull requests touching the Kani-touching set without an actual full
    gate passing on the curre
- path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  id: NFR-006-M-2
  excerpt: Lane runs reported `passed` with `ran` different from `expected` or `ran`
    of 0
- path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: examined
  id: NFR-006-M-3
  excerpt: '`#[ignore = "kani lane: ..."]` tests the lane does not select'
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-2
  excerpt: Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, or
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-6
  excerpt: Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSp
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-48
  excerpt: A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-49
  excerpt: When QSV refuses a `quire.op.quantity.convert` with `IllTyped` before any
    charge, the oracle stops a
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-50
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar:
    its result is always a `Quanti'
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-51
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other qu
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014-AC-52
  excerpt: 'Every generated quantity operand is a QSV `UnitQuantity<''_>` resolved
    through the kernel quantity''s '
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-21
  excerpt: Each generated item's claim-map entry records its declaration closure as
    every declaration's key, fo
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-22
  excerpt: 'For every pair of operand values in the item''s refinement domain (FR-028-AC-16:
    every case where the'
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-23
  excerpt: For every item of the TC-029 corpus whose leaves are Boolean or bounded
    integers, the claim-map clos
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-33
  excerpt: A genuine root-facade compilation of authentic retained source, original
    dependencies and admitted r
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-34
  excerpt: The actual generated ABI executes ordered and unordered enum comparisons
    over all declared cases, in
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-35
  excerpt: Unknown declaration, unknown case and a directly or transitively imported
    declaration produce the or
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-36
  excerpt: Emitted source uses only WireNodeId and case identifiers for lookup and
    contains no NodeKey construc
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-37
  excerpt: Equality's generated manifest and native scratch consumers use the one
    profile authority's direct ex
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-38
  excerpt: With `None`, standalone generation and emitted preparation accept successfully
    reconstructed non-nom
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-39
  excerpt: '`None` at the first reached nominal identity returns typed MissingAdmittedContext
    naming that declar'
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-40
  excerpt: Some(wrong-package context) fails the whole standalone call after existing
    setting/accounting checks
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-41
  excerpt: A reached enum still reconstructs every owner-admitted case, including
    unused cases and an enum para
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-42
  excerpt: Generated oracle invocations under Some or valid context-free None preserve
    private package/item/des
- path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  id: FR-018-AC-43
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_ra'
- path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  id: FR-021-AC-30
  excerpt: A requested generation work limit of `u64::MAX` returns `InvalidGenerationWorkLimit`
    before lowering
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-2
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder
    over a `reject` integer t
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-6
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type, a zero divisor eval
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-16
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer
    type of `-5..=5`, the min
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-17
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate,
    the outcome of a divide or r
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-27
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before
    the native division of AC-2 '
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-20
  excerpt: IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle
    with unconstrained opera
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-22
  excerpt: 'PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native '
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-26
  excerpt: 'PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The
    generated source of a native '
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-23
  excerpt: PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject`
    type as the infix `/` and Re
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-24
  excerpt: PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle
    of the construction Z-1 (a
- path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  id: FR-031-AC-25
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite (its native side calls
    the kernel). For divide and '
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-1
  excerpt: 'A generated caller-ingress oracle accepts `&TextPayload`, the declared
    `TextType` and `&mut Meter`, '
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-2
  excerpt: Invalid UTF-8 bytes make `TextPayload::from_utf8` return `InvalidUtf8`
    before admission and leave th
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-3
  excerpt: For each profile, `Text[1,1]` admits or refuses according to the profile
    length of the retained sequ
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-4
  excerpt: 'For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real
    Kani harness symbolically '
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-5
  excerpt: In a copy of the generated oracle with its admission call or declared `TextType`
    altered so the orac
- path: spec/oracle/functional/FR-035-caller-text-admission.md
  role: examined
  id: FR-035-AC-6
  excerpt: A request outside the declared finite proof class has a typed unsupported
    or requires-bound disposit
- path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  id: FR-042-AC-1
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors
    use the exact nine CG
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-1
  excerpt: The envelope's obligation identity is QSL's `ObligationIdentity` built
    as the ADR-013 O-09 digest by
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-7
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run
    and native replay both pres
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-8
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's
    transcript, never the ori
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-18
  excerpt: With the installed backend, the falsified operation-contract harness of
    a postcondition state clause
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-34
  excerpt: 'PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. With the '
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-35
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A falsifi
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-36
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success
    carries equivalent assignme'
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-37
  excerpt: 'The common intake maps an adapter''s decode-refusal arm to a typed decode
    evidence failure retaining '
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-38
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate
    that preserves the failure '
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-39
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid
    reduction that chang
- path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  id: FR-024-AC-40
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend
    re-run, bound to its o
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-1
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    A public consume
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-4
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The driver's act
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-5
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The public consu
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-6
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    For every `Diver
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-7
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    QSL's divergence
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-8
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance).
    The real QSL-emi
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-9
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention).
    Independent canonica
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-10
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family retention and actual
    CG/QSL conformance). When th
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-11
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    actual CG/QSL route). When
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-12
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    actual CG/QSL route). When
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-13
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and
    CG builder). When the owni
- path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  role: examined
  id: FR-032-AC-14
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding
    the scalar builder/conver
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-2
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family,
    including beyond-i64 composi
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-3
  excerpt: PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration
    or unit/reference id
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-4
  excerpt: 'PLANNED/GATED. Public caller input exactly at the configured encoded-byte
    limit is admitted and one '
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-5
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand
    values by the FR-025 orig
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-6
  excerpt: PLANNED/GATED. For falsified replay the driver executes the same proved
    generated artifact and suppl
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-8
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and
    literal singleton domains an
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-10
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed
    route has a real verify and f
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-14
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records,
    with bounded-field narro
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-15
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne`
    comparing one parameter wi
- path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  id: FR-033-AC-17
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer
    literal operand (`I
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-3
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement
    settles `requires-bound`
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-15
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin`
    projecting exactly QSL layer
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-24
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at
    an unbounded `s: Set<Int[0,999'
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-34
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1
    through TP-4, TP-5, SG-1 thro
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-35
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's
    kind and form, TP-1 through
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-36
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item
    retains its safety and option
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-37
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising
    EN-1 arm; only HP-2 reaches a
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-38
  excerpt: PLANNED (IR-516). An advertised SG-1, SG-2 or SG-3 item reaches EN-1 with
    its recorded form; only ad
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-39
  excerpt: PLANNED (IR-516). A selected reduction reaches only a candidate advertising
    that exact reduction, wi
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-40
  excerpt: PLANNED (IR-516). For an item over a weak `parallel`, a candidate advertising
    the subject's resolved
- path: spec/routed/functional/FR-019-capability-settlement.md
  role: examined
  id: FR-019-AC-41
  excerpt: PLANNED (IR-516). With otherwise equal form and kind advertisement, model
    and protocol subject class
- path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  id: FR-022-AC-26
  excerpt: Given the checked QSL RR-16 package with two `x + 1` occurrences of one
    application node, directly n
- path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  id: FR-022-AC-27
  excerpt: For `if p then x + 1 else x + 1`, each occurrence's generated scalar claim
    is conditional on its own
- path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  id: FR-022-AC-28
  excerpt: If a routed scalar item's occurrence has no entry in the supplied requirement
    records, its key's nod
- path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  id: FR-026-AC-1
  excerpt: The Kani adapter implements the adapter trait, and a function generic over
    that trait reaches the Ka
- path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  id: FR-026-AC-4
  excerpt: The execution evidence type is an associated type of the adapter trait,
    and the Kani adapter's is `K
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-7
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from
    an admitted `CheckedPacka
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-8
  excerpt: PLANNED (IR-815). In the V2 harness, an instrumented snapshot, precondition
    oracle, subject and post
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-10
  excerpt: 'PLANNED (IR-815). A selected clause identity that is unknown, of the wrong
    clause kind, or the same '
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-11
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape
    yields a structured unsuppor
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-12
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted
    package, selected clause iden
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-13
  excerpt: 'PLANNED (IR-815). A seeded V2 case on which both oracles complete true
    runs the subject once and is '
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-14
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    false is a rejected pr
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-15
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle completes
    true and the postcondi
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-16
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition or postcondition
    oracle returns an outc
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-17
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not
- path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  id: FR-002-AC-18
  excerpt: PLANNED (IR-815). A seeded V2 case on which the precondition oracle returns
    an outcome that does not
- path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  id: FR-008-AC-7
  excerpt: PLANNED. Beyond the per-operator agreement stated in FR-008-AC-3, which
    this criterion does not repe
- path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: examined
  id: FR-008-AC-9
  excerpt: PLANNED. A CG integration test calls IR's public clause-context accessor
    through its public API only
- path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  id: FR-009-AC-7
  excerpt: PLANNED. For each noncommutative operator, a comparison written with the
    literal in the first positi
- path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  id: NFR-004-M-1
  excerpt: Discarded / attempted on a fresh 10,000-case campaign per population and
    fixture
- path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  id: NFR-004-M-2
  excerpt: Proptest global rejects on the same campaigns
- path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  id: NFR-004-M-3
  excerpt: Campaigns ending `Exhausted` on the same campaigns, and on 256-case default-config
    campaigns
- path: spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md
  role: examined
  id: NFR-004-M-4
  excerpt: In-domain plus out-of-domain census cases per admitted clause
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014#quantity-operands
  excerpt: "- Generated quantity oracles shall receive each quantity operand as QSV\n\
    \  `UnitQuantity<'_>`, obtain"
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014#quantity-arithmetic
  excerpt: "- Quantity arithmetic shall call QSV `evaluate_quantity_unit` and return\n\
    \  `Result<(Outcome<Quantity"
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  id: FR-014#quantity-conversion
  excerpt: "- If a requested node's `operation.identity` is `quire.op.quantity.convert`,\n\
    \  then the generator sh"
- path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  id: TC-024#quantity-agreement
  excerpt: "For quantity vectors, obtain every generated and direct operand through\n\
    \   actual QSV `UnitTable::re"
- path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  id: TC-024#quantity-conversion
  excerpt: "with `QuantityTarget::Exact`, a resolved QSV `UnitQuantity` operand and\n\
    \   `unit.unit()`, and return"
- path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  id: FR-014#owner-migration
  excerpt: 'The direct-owner migration is PLANNED until implementation and focused

    qualification. Kernel values,'
- path: src/kani/generate/scalar.rs
  role: examined
  id: src/kani/generate/scalar.rs::tests::tc_025_derive_domain_reads_binding_shaped_range_members
  excerpt: '    fn tc_025_derive_domain_reads_binding_shaped_range_members() {'
- path: src/routed/contract_package_tests.rs
  role: examined
  id: src/routed/contract_package_tests.rs::tests::duplicate_expression_uses_each_selected_conversion_context
  excerpt: '    fn duplicate_expression_uses_each_selected_conversion_context() {'
- path: src/routed/contract_package_tests.rs
  role: examined
  id: src/routed/contract_package_tests.rs::tests::scalar_own_result_without_conversion_differs_from_not_applicable
  excerpt: '    fn scalar_own_result_without_conversion_differs_from_not_applicable()
    {'
- path: tests/it/oracle_generation.rs
  role: examined
  id: tests/it/oracle_generation.rs::tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid
  excerpt: fn tc_001_boolean_oracle_bundle_is_deterministic_traceable_and_schema_valid()
    {
- id: FR-015#3
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'CG SHALL use FR-384''s operator-specific capacity policy through the actual
    checked kernel plan: native i128 carriers and interval endpoints, full admitted
    signed-i128 Add/Subtract/Negate and ordering '
- id: FR-015#4
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Generated artifacts and canonical identity inputs SHALL bind the actual
    selected operator-capacity policy and its revision together with the selected
    ABI and interval decisions, under the owning ident
- id: AD-006#5
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: CG reads operand intervals and every reachable arithmetic operation from
    the admitted graph through the existing IR-owned seams. It derives conservative
    interval bounds in expression order, preserving
- id: AD-006#6
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: CG records the selected kernel capability, actual operator-capacity policy
    and its revision, and input/result interval decisions in the generated artifacts
    and identity inputs using the repository's e
- id: AD-006#7
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: A kernel capacity refusal is a typed generation/capability disposition
    carrying the offending source and capacity reason, before artifact success. It
    is distinct from IntegerOutOfDomain for a complete
- id: AD-006#8
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: 'Kernel qualification supplies machine-checked full-capacity refinement
    against an independent mathematical abstraction of actual BigInt semantics; sampled
    differential tests alone are not a universal '
- id: AD-006#9
  path: spec/assurance/AD-006-admitted-native-integer-oracles.md
  role: examined
  excerpt: '| Decision | Choice | Alternatives and trade-off |

    | --- | --- | --- |

    | Value ownership | Kernel Copy i128 carrier, checked per-operation interval plan
    and opaque per-call admission. | General Intege'
- id: TC-027
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: context_only
  excerpt: 'Verify that a bounded Kani obligation runs against the installed backend,
    that

    its outcome is read from the backend rather than defaulted, and that the

    returned evidence describes the invocation that '
```

## Coverage

Plan completion: not assessed

Applicable AssuranceProfile: none found. Review set is the applicable subset selected by the lead. Rust review is inapplicable: no Rust/manifest code changed. Dependency analysis is inapplicable: no new relationship edge. Object review is inapplicable: no domain-object edits. Optional exhaustive semantic gap review, Jev strength review and architecture-evaluation were not requested and were not run. No Cargo, dependency resolver, compiler, Kani, namespace or host-security operation was run. Whole matrix was mechanically reconciled; untouched tagged criteria were not individually semantically reviewed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-002 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-003 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-004 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-005 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-006 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-007 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-008 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-009 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-010 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-011 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-012 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-013 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-014 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-015 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-016 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-017 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-018 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-019 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-020 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-021 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-022 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-023 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-024 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-025 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-026 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-027 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-028 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-029 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-030 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-031 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-032 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-033 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-034 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-035 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-036 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-037 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-038 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-039 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-040 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-041 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-042 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-043 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-044 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-045 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-046 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-047 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-048 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-049 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-050 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-051 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-052 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-053 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-054 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-055 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-056 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-057 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-058 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-059 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-060 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-061 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-062 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-063 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-064 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-065 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-066 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-067 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-068 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-069 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-070 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-071 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-072 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-073 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-074 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-075 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-076 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-077 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-078 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-079 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-080 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-081 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-082 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-083 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-084 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-085 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-086 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-087 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-088 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-089 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-090 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-091 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-092 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-093 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-094 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-095 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-096 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-097 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-098 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-099 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-100 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-101 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-102 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-103 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-104 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-105 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-106 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-107 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-108 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-109 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-110 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-111 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-112 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-113 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-114 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-115 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-116 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-117 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-118 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-119 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-120 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-121 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-122 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-123 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-124 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-125 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-126 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-127 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-128 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-129 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-130 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-131 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-132 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-133 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-134 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-135 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-136 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-137 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-138 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-139 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-140 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-141 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-142 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-143 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-144 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-145 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-146 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-147 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-148 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-149 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-150 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-151 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-152 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-153 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-154 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-155 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-156 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-157 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-158 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-159 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-160 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-161 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-162 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-163 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-164 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-165 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-166 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-167 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-168 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-169 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-170 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-171 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-172 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-173 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-174 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-175 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-176 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-177 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-178 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-179 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-180 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-181 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-182 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-183 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-184 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-185 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-186 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-187 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-188 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-189 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-190 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-191 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-192 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-193 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-194 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-195 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-196 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-197 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-198 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-199 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-200 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-201 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-202 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-203 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-204 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-205 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-206 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-207 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-208 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-209 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-210 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-211 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-212 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-213 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-214 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-215 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-216 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-217 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-218 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-219 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-220 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-221 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-222 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-223 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-224 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-225 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-226 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-227 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-228 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-229 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-230 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-231 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-232 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-233 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-234 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-235 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-236 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-237 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-238 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-239 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-240 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-241 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-242 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-243 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-244 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-245 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-246 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-247 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-248 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-249 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-250 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-251 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-252 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-253 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-254 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-255 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-256 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-257 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-258 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-259 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-260 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-261 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-262 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-263 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-264 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-265 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-266 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-267 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-268 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-269 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-270 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-271 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-272 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-273 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-274 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-275 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-276 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-277 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-278 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-279 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-280 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-281 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-282 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-283 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-284 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-285 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-286 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-287 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-288 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-289 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-290 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-291 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-292 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-293 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-294 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-295 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-296 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-297 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-298 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-299 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-300 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-301 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-302 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-303 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-304 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-305 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-306 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-307 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-308 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-309 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-310 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-311 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-312 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-313 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-314 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-315 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-316 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-317 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-318 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-319 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-320 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-321 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-322 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-323 | deferred | IR-705 owns specification-first native/quantity implementation and genuine proof qualification; this amendment neither delivers nor waives that evidence. |
| FND-324 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-325 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
| FND-326 | deferred | IR-878 owns inherited repository traceability and assurance closure; no implementation or waiver is delivered by this narrow specification amendment. |
