---
id: SR-001
title: Shared assurance migration code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#13 on issue/13-shared-assurance-migration; the domain work inherited from the three-deep PR #9 -> #10 -> #12 stack; FR-006 and the deletion of the local evidence framework"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: references
---

# SR-001: Shared assurance migration code review

## Summary

This change carries two things at once. It inherits the whole of the Wave 2 codegen domain work,
which exists only on an unmerged three-deep stack — `main` holds a stub `src/lib.rs` and nothing else
— and it migrates the repository's QA machinery onto the released Engineering Assurance, Quire and
Quoin contracts. It supersedes PRs #9, #10 and #12.

The domain half is inherited, not authored here. `git diff <stack head> HEAD -- src/` is empty: not
one byte of the oracle, harness or strategy lowering changed. One line of `tests/` changed, and that
change is a reconciliation between the stack's two divergent heads rather than new work.

The migration half replaces 4,073 lines of local evidence machinery across eleven whole-file
deletions with gates that delegate: component versions to `engineering_assurance.compatibility`,
retained bytes to `map_pgm01_bytes`, and everything dynamic to Quoin's change-assurance surface.

## Verdict

**CONDITIONAL.** Findings below; every one is dispositioned in SR-003 after the independent
adversarial review.

## The stacked-branch divergence

PR #12's head (`wave2-agent-b-harnesses`, `a003f4c`) was never restacked onto PR #10's head
(`wave2-agent-b-oracles`, `d8a9dbe`). GitHub reports PR #12 CONFLICTING for exactly that reason. A
local variant, `wave2-agent-b-harnesses-round3` (`d692a05`), is the correct restack and carries two
commits the base does not.

The divergence was read across every domain path rather than assumed to be evidence machinery.
Outside `evidence/` it is eight files, six of which are machinery this change deletes. The remaining
two point in opposite directions and were dispositioned separately:

| File | Direction | Disposition |
| --- | --- | --- |
| `scripts/check_unsafe_comments.sh` | variant is stronger | **carried forward** |
| `tests/oracle_generation.rs` | variant is weaker | **rejected** |

The unsafe-audit change is a false-green fix, measured rather than argued: with no Rust source roots
present the base version prints `unsafe audit passed` and exits 0; the variant prints
`unsafe audit inconclusive` and exits 2. A gate that reports success when it had nothing to audit is
the exact class this whole programme exists to remove. It also stops `grep` failures being swallowed
by `|| true`.

The `tests/oracle_generation.rs` change replaced

```rust
assert!(!extension.generator_source_dirty);
assert!(!generator_source_is_dirty());
```

with

```rust
assert_eq!(extension.generator_source_dirty, generator_source_is_dirty());
```

`src/oracle.rs:825` assigns that field the value of that call. The replacement compares an expression
to itself, and it holds on a dirty tree. It is a tautology and the base's pin is kept.

Neither branch is deleted. The four evidence collections that exist only on the variant remain
fetchable there.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-101 | high | The hosted workflow pinned `@agent-ix/quoin` at a version the accepted compatibility matrix names incompatible: it predates the `quoin change-assurance` surface entirely, so seal, intake and receipt do not exist in it. Nothing local read the workflow, so nothing noticed. Repinned, and the omission is now a gate rather than a grep | `.github/workflows/ci.yml`, `scripts/check_shared_pins.py` | correct-requirement-no-evidence |
| FND-102 | medium | `ix-flow` was absent from the hosted install list altogether. A pinned component that is never installed cannot be classified against the matrix, so its verdict was unobtainable rather than wrong, which is a quieter failure than a wrong pin | `.github/workflows/ci.yml` | correct-requirement-no-evidence |
| FND-103 | high | `scripts/check_unsafe_comments.sh` printed `unsafe audit passed` and exited 0 when no Rust source root was present. Measured on both branch heads before deciding. Fixed by carrying the divergent branch's version, which exits 2 as inconclusive | `scripts/check_unsafe_comments.sh` | correct-requirement-no-evidence |
| FND-104 | medium | The retained `verify_foundation_evidence.py` re-derives a parameter digest over controlling source files, so any source change fails it. The old path therefore cannot be run green at the migration revision. That is a property of the old path, recorded in the dual run rather than worked around | `scripts/verify_foundation_evidence.py` | wrong-requirement |
| FND-105 | medium | `quire coverage --strict` prints `39 unbacked row(s) and 6 contradicted status(es)` and returns 0, so a Coverage Status column contradicting its own row is advisory upstream. The local checker that used to compensate was a second traceability implementation with a hand-copied matrix and is deleted; `tc_010` now gates on the export's own `status_lies` instead | `spec/test-matrix.md`, `agent-ix/quire-contract-ir#21` | wrong-requirement |
| FND-106 | medium | FR-003 and FR-004 name Kani obligation lowering and vacuity evidence and neither has any implementation. A proof obligation over them would run, report pass, and mean nothing, so none was created | `spec/functional/FR-003-kani-lowering.md`, `spec/functional/FR-004-vacuity-evidence.md` | correct-requirement-no-evidence |
| FND-107 | medium | Make is not a trust root and the guard that pretended otherwise is deleted. Measured: control exits 2 at `fmt-check`; `.IGNORE:` gives exit 0 with seven of eleven prerequisites failing and none failing the build. Recorded in four places rather than closed | `Makefile`, `agent-ix/quire-contract-codegen#14` | wrong-requirement |
| FND-108 | low | The frozen-schema set here is two, not the sibling's four. `schemas/pgm01-derivation-evidence-envelope-v1.schema.json` is included by `src/oracle.rs` as `PGM_SCHEMA` and validated against on every generation: a live output contract that retained records also happen to name. Inheriting the sibling's list would have described a live dependency as dead machinery | `schemas/`, `assurance/pins.json` | wrong-requirement |
| FND-109 | low | `evidence/ANCHORS` loses its only reader. It is left byte-identical rather than deleted, because the constraint is that retained bytes stay unchanged and an unread census inside an immutable tree is a smaller problem than editing that tree | `evidence/ANCHORS` | wrong-requirement |

## Dispositions

Carried to SR-003 after the independent adversarial review.
