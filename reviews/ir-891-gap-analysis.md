---
id: SR-13293
title: "IR-891 CG PR 458 gap-analysis: NFR-002-AC-1/AC-2 and FR-016-AC-6 against code and matrix"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@242c506c617a078d26d748c3fac375f2056f5931 vs origin/main e45b9d2f; spec/core/**, spec/replay/**, src/strategy/**, src/oracle/**, src/kani/generate/**, src/replay/witness.rs, src/replay/function.rs"
review_set: subset
---

## Summary

Ticket: IR-891. Plan completion: not assessed (planless). `quoin matrix --json` at the PR head:
NFR-002-AC-1, NFR-002-AC-2 and FR-016-AC-6 are untagged with no run evidence, matching their
Planned rows; no test in `src/` or `tests/` names them. The code PR follows, so the absence of
implementation is expected and not a finding. Today no generated family meets restated AC-1 in
full: only the three oracle packages name `quire-contract-codegen`, and they carry no SPDX line.
The decoder (`src/replay/witness.rs:119-173`) has no byte or value-count bound.

## Examined criteria

- NFR-002-AC-1, NFR-002:47 (examined)
- NFR-002-AC-2, NFR-002:48 (examined)
- FR-016-AC-6, FR-016:128 (examined)
- TC-001 procedure and expected results (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-001 checks headers per family, but several families emit more than one `.rs` file and some have no header at all: function schedule part modules `src/schedule_*.rs` (`src/oracle/function/mod.rs:745-760`), equality extra modules (`src/oracle/equality/mod.rs:1813-1820`), and package `src/lib.rs` files that hold many subjects. A per-family sample would pass with these unheaded. Fix: TC-001 Expected Results should require every generated `.rs` artifact of every family, including module part files, to carry the header and SPDX line. | spec/core/matrix/TC-001-reproducibility.md:24-34 |

## Verdict

**CHANGES REQUESTED (minor).** Matrix state agrees with the code; tighten TC-001 so the planned
test covers every emitted Rust file.
