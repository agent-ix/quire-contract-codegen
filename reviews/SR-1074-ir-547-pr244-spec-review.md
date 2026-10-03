---
id: "SR-1074"
title: "CG PR 244 spec review: byte-ceiling lowering failure as its own refusal"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@86ac6775de82161ea61b4670f94fe59aa413ee69; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md; diff origin/main...HEAD, base aba2403"
---

# SR-1074: CG PR 244 spec review: byte-ceiling lowering failure as its own refusal

## Summary

Ticket: IR-547. PR: agent-ix/quire-contract-codegen#244 at 86ac677. This is a spec-only diff over 7 files: FR-014, FR-018 and FR-021, TC-024, TC-029 and TC-031, and tests.md. The PR body and author report were treated as claims and measured.

Form and hygiene measured:

- The new ACs are direct assertions marked `PLANNED (IR-547)`, the same marker FR-014-AC-38 uses. The Behavior bullets use `shall`, as the rest of the Behavior sections do.
- Numbering is contiguous. On main the last ACs were FR-014-AC-39, FR-018-AC-19 and FR-021-AC-22, and no FR-014-AC-40+, FR-018-AC-20+ or FR-021-AC-23+ existed anywhere in the repo.
- The tests.md rows, the TC-024/029/031 inventory `Traces To` lists and the TC steps agree. TC-031 keeps `✅ Covered` overall, now listing AC-23 among its planned ACs.
- FR-014 has no mutation table; FR-018 and FR-021 get mutation rows for AC-20 and AC-23.
- `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0, warnings only (quire from PATH, in a detached worktree at the head).
- `quire coverage --strict --scope .` reports 66 unbacked and 0 contradicted at the head and at base aba2403, so no count moved. The planned rows are not counted as unbacked.

IR consistency was measured at quire-contract-ir cbcd790:

- `CheckedPackageLimit` has 7 variants: Bytes, Depth, Nodes, Edges, Occurrences, Diagnostics and Work. It is not `#[non_exhaustive]`, and it derives Clone, Copy, Debug, Eq and PartialEq, with no Serialize and no wire-name method.
- `CompleteLoweringRecordV2::Failed` is built only with `Work` (lower.rs:490) or `Bytes` (lower.rs:338 for the whole package, lower.rs:668 for one node).
- So routing the five other kinds to a typed `LoweringLimitUnrecognised`, never a panic or work exhaustion, is sound and is the right handling for a closed IR enum that IR could still widen.
- The CG statements about `limit`, `consumed` and whole-package versus per-node failure do not contradict FR-038-AC-95.

## Verdict

Sound shape. One classifier, a distinct byte refusal, and the five other kinds kept typed all match IR. Every finding here is a precision gap that makes an AC ambiguous or hard to check. Read this with SR-1075, whose FR-021 contradiction is high.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'one byte below the package's canonical length' does not say which package; the checked package cannot be read under that ceiling. Must name the lowered contract package of the call and read the input under that ceiling. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:384 |
| FND-002 | medium | 'naming that kind' does not fix the spelling (IR enum has no Serialize or wire name); ACs cannot assert a value | spec/oracle/functional/FR-014-exact-scalar-oracles.md:385 |
| FND-003 | low | 'any other CheckedPackageLimit value' literally includes bytes; say 'neither work nor bytes' | spec/oracle/functional/FR-014-exact-scalar-oracles.md:385 |
| FND-004 | low | negotiate mapping placed in FR-014/TC-024; negotiate.rs implements FR-015 (TC-025) | spec/oracle/functional/FR-014-exact-scalar-oracles.md:387 |
| FND-005 | low | Behavior bullet restates IR AC-95 semantics beyond the citation (and drops 'saturating') | spec/oracle/functional/FR-014-exact-scalar-oracles.md:290-300 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | FR-021-AC-23 whole-call ceiling is 'one byte below the lowered contract package of that call', but FR-021 lowers twice; both fail only if the ceiling is below both lengths | spec/oracle/functional/FR-021-function-application-oracles.md:282 |

## Dispositions

Round 1 was reviewed at 10b5ed2602cb17306ea57e9a5a04a2235959aa7d (delta 86ac677..10b5ed2).

- **FND-001.** The ceiling is now defined as "read under a byte ceiling that admits the checked package and is one byte below the canonical length of the lowered contract package of that call". That is coherent with how CG reaches the ceiling.
  - The CG generators take `&CheckedPackageV2`.
  - IR's `lower(&self, requested, profile)` has no byte parameter: `CompleteLoweringProfileV2` holds only `supported_tags`, `require_bounds` and `work_limit`.
  - The encode ceiling is `self.bytes`, which `read` sets to `limits.bytes` (IR v2/mod.rs:685).
  - So the reader limit is the one ceiling, and a document that fits under it is admitted.
  - For scalar and equality there is one lowering per call, so "the lowered contract package of that call" is unique. FR-021 lowers twice; see the new FND-006.
- **FND-002.** `limit_kind` is now a `&'static str`, the snake_case of the variant name, serialised as that string. AC-41, AC-20 and AC-23 assert that value.
- **FND-003.** AC-41 now reads "neither `work` nor `bytes`".
- **FND-004.** The negotiate AC moved to FR-015-AC-50. FR-015's last AC at 86ac677 was AC-49, so the numbering is contiguous. TC-025 step 15 follows step 14. The spec/kani/matrix/tests.md row and the TC-025 inventory carry AC-50. FR-014-AC-43 is removed, FR-014 ends at AC-42, and no reference to FR-014-AC-43 remains in the repo. The kani tests.md row sits between the AC-19..25 and AC-26..36 rows rather than last. That is cosmetic and not a finding.
- **FND-005.** The bullet now cites AC-95 for when the record occurs and what `limit` and `consumed` mean, and it states the `u64::MAX` saturation.
- **Gates.** `quire validate` exits 0 and `quire coverage --strict` reports 66 unbacked and 0 contradicted, unchanged. Both ran in a detached worktree at 10b5ed2, since removed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 10b5ed2 |
| FND-002 | fixed | 10b5ed2 |
| FND-003 | fixed | 10b5ed2 |
| FND-004 | fixed | 10b5ed2 |
| FND-005 | fixed | 10b5ed2 |

Round 2 was reviewed at 200acd8565478dfbea41645ae22eab1c5c9339cc (delta 10b5ed2..200acd8).

- **FND-006.** The FR-021-AC-23 whole-call ceiling is now one byte below the shorter of the two lowered contract packages' canonical lengths (bodies, and requested `call` nodes). The ceiling must still admit the checked package, and the fixture must make both packages longer than the checked document.
  - A ceiling below both lengths fails every body record and every call-node record, so the AC's assertions follow.
  - Requiring different `consumed`, confirmed by lowering each request set alone under that ceiling with the public `lower`, is the right way to make the call-node-first assertion discriminating. Two packages failing at the same ceiling can otherwise give equal `required`.
  - TC-031 step 11 matches the AC.
- **Earlier findings.** Every finding fixed in rounds 0 and 1 is still fixed at 200acd8. The delta touches only the FR-014-AC-40, FR-018-AC-20 and FR-021-AC-23 rows, the FR-021:162 bullet, and TC-024 step 2, TC-029 step 12 and TC-031 step 11. No reference to FR-014-AC-43 and no "beside a lowered record" wording remains anywhere in spec/.
- **Numbering.** It is contiguous: FR-014 AC-1..42, FR-015 AC-1..50, FR-018 AC-1..20 in both the criteria and mutation tables, and FR-021 AC-1..23 in both. The oracle and kani tests.md rows and the TC-024, TC-025, TC-029 and TC-031 inventories agree with the ACs.
- **Gates.** `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with warnings only. `quire coverage --strict` reports 66 unbacked and 0 contradicted, unchanged. Both ran in a detached worktree at 200acd8, since removed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 200acd8 |
