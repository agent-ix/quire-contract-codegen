---
id: "SR-1386"
title: "CG PR 258 spec review (integrity): batch rules against FR-017's existing single-run rules and FR-028"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@1add57d77cfcd3a9e39e515f063fde12b81f4d70; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/assurance/AD-004-cg-crate-layout.md:1256-1260 (diff origin/main...HEAD); src/kani/abi.rs:74-100, src/kani/generate/{frame,scalar,negotiate}.rs adapter_options call sites, src/kani/output/{report,playback}.rs, src/kani/run/execute.rs read at base 4de9f20"
---

# SR-1386: CG PR 258 spec review (integrity)

## Summary

Ticket: IR-277. I checked the new batch bullets and criteria against the FR-017 rules they sit
beside, and against FR-028 and the code at the base.

- **Grouping is implementable.** `src/kani/abi.rs:84-89` puts `--harness <path> --exact` inside
  the identity's option vector. Removing that triple gives a shared vector that can be compared.
- **The value passed is the harness path, not the symbol.** Every call site passes
  `module::harness` (`frame.rs:827`, `scalar.rs:368`, `negotiate.rs:852`, `v1_bundle.rs:184`), and
  Kani echoes it back as `harness_id` (measured, SR-1385).
- **Playback comes from the console.** `src/kani/output/playback.rs:21-39` returns the first
  non-cover playback block in the whole console text. It ignores which harness printed it.
- **FR-028 ceilings do not exist in code.** No harness identity in `src/` records a memory or
  wall-clock ceiling, and FR-028 is 0/10 backed.
- **AD-004 agrees.** The AD-004 edit matches FR-028's batch bullet and FR-017-AC-21 to AC-23.

## Verdict

Not mergeable. Two high contradictions inside FR-017 must be fixed in spec text first: the
playback source, and the argument-vector and evidence rule. Two medium gaps make the batch rules
unimplementable as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The batch verdict bullet and FR-017-AC-22 take each member's falsifying playback "from that member's own entry in the batch's report". The report carries no playback: FR-017 line 163 says so, and a real batch report has no playback member (measured). Playbacks are console blocks headed with the harness name. Today's `counterexample_playback` would hand every member the first failing block printed by any member. The spec must say that the playback is taken from the console block for that member's harness, and refuse the batch when that block is missing or cannot be attributed. | spec/kani/functional/FR-017-kani-execution-evidence.md:111-118,163-166,195, src/kani/output/playback.rs:21-39 |
| FND-002 | high | Contradiction. FR-017 lines 141-144 require the evidence's argument vector to be "the `kani` subcommand, the harness identity's option vector unchanged and the report-export flags, in that order and nothing else, so the evidence cannot claim an invocation the harness did not specify", and FR-017-AC-6 says the same. The new bullet says each member's evidence carries the batch's full vector: other members' `--harness` pairs, the selection moved ahead of the shared options. That breaks the unchanged and nothing-else rule for every member of a batch of more than one. Scope the existing rule to single runs, and state what a batch member's evidence must contain instead, for example its own option vector plus the batch vector as a separate field. | spec/kani/functional/FR-017-kani-execution-evidence.md:105-118,138-144,182 |
| FND-003 | medium | The existing refusal of a report "that does not hold exactly one harness result" (FR-017 lines 149-153, and FR-017-AC-18 "holds other than one harness result") is not scoped to single runs. A batch report holds N entries, so as written every batch report is refused before FR-017-AC-22 can apply. Scope AC-18 to a single run. Then state the batch counterpart: exactly the requested set, which FR-017-AC-23 already describes. | spec/kani/functional/FR-017-kani-execution-evidence.md:149-153,191,195-196, src/kani/output/report.rs:319-330 |
| FND-004 | medium | The grouping rule compares "FR-028 ceilings", but no identity records any ceiling yet, because FR-028 is entirely Planned. Where T comes from is not stated: FR-017's per-harness request `timeout`, or FR-028's identity wall-clock ceiling. No batch request shape is stated: who forms batches, and whether members can carry different timeouts. FR-028's batch bullet also holds the process to a "shared memory ceiling", while FR-017 Inputs say "The run has no memory ceiling". IR-277 code either waits for FR-028 or needs a stated interim (group on equal request timeouts). | spec/kani/functional/FR-017-kani-execution-evidence.md:41-43,105-110,194, spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-58 |
| FND-005 | medium | Members are "matched by harness symbol", and FR-017-AC-21 says `--harness <symbol>`. In this codebase the symbol is the bare `HarnessSymbol`. Every frame harness has the same one, `const HARNESS: &str = "check"` (frame.rs:953), so matching by symbol is ambiguous. The key is the fully qualified `module::harness` path that is passed to `--harness` and echoed as `harness_id`. "Harness path" in FR-017 evidence already means the generated file path, so name the key explicitly. | spec/kani/functional/FR-017-kani-execution-evidence.md:107,112,194-195, src/kani/generate/frame.rs:824-828,953 |

## New findings (disposition pass 1)

Reviewed at a0eba86e850e08f0e19edfa99ce804b166dfda53.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | "If a member's report entry names a failed property check and no block is headed for it, the generator shall refuse the batch as an unattributable playback." This conflicts with FR-017-AC-5, where a failure with no playback is inconclusive "for that reason". It also refuses far more than it needs to. A missing block is absent, not misattributed, yet all N members lose their results. Classify that member inconclusive with no counterexample, as a single run would, and refuse only a block that cannot be attributed: one headed for a path that was not requested, or two blocks headed for the same path. | spec/kani/functional/FR-017-kani-execution-evidence.md:133-138,219,234 |
| FND-007 | low | FR-017-AC-22 lists only the batch fields a member's evidence carries: the batch vector, the member list, the batch statement and the exit code. With AC-6 now scoped to single runs, no criterion says a batch member's evidence still carries the kind, harness path, launcher path, unwind bound, solver, outcome and checks. The new batch entry in `kani/run/execute.rs` and the added evidence fields are also missing from the interface-001 execution slice (l.111: `KaniExecutionEvidence \| KaniExecutionRefusal`). | spec/kani/functional/FR-017-kani-execution-evidence.md:43-45,150-155,220,233, spec/core/functional/interface-001-codegen-api.md:111 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a0eba86 | A batch member's playback now comes from the console block Kani heads for that member's `module::harness` path, never from the first failing block. `counterexample_playback` is named as needing that path. Re-measured: the heading reads "Concrete playback unit test for `proofs::bad_harness`:". The new refusal sub-rule is FND-006. |
| FND-002 | fixed a0eba86 | The single-run argument-vector rule and AC-6 are now scoped to single runs. A batch member's evidence holds the batch vector, the member list and a batch statement. AC-6's purpose is kept honestly: the evidence records the invocation that actually happened, and says it was shared, rather than claiming an invocation for one harness. |
| FND-003 | fixed a0eba86 | The single-run report rule (l.188) and FR-017-AC-18 now say "for a single run" and "in a single run". The batch counterpart is FR-017-AC-23. |
| FND-004 | fixed a0eba86 | Batches are grouped by equal option vector and equal request timeout T, and the FR-028 ceilings join the key when they land. A batch entry in `kani/run/execute.rs` forms the groups. The memory ceiling stays FR-017's "no memory ceiling" until FR-028. |
| FND-005 | fixed a0eba86 | Members are matched by the `module::harness` path that Kani echoes as `harness_id`, never by the bare symbol (`check`), and FR-017-AC-22 tests two members that share `check`. |
| FND-006 | fixed ce7e0f0 | A failed property check with no console block for its path is now that member inconclusive with no counterexample, as in FR-017-AC-5. Only a block headed for a path outside the batch, or two blocks for one path, refuses the batch (FR-017 l.140-145, AC-23, TC-043 step 8 and result 8). |
| FND-007 | fixed ce7e0f0 | A batch member's evidence now carries kind, harness path, launcher path, unwind bound, solver, outcome and checks, plus the batch vector, member list, batch statement and exit code (FR-017 l.157-161, AC-22). interface-001 adds `execute_kani_obligations` (planned, name provisional) with matching inputs, output and semantics, and adds it to the Features table. Consistent with FR-017. A sweep of spec/ for 'refuses the batch', 'no report', 'Duration::MAX', 'unattributable', 'one process' and 'single' found no stale statement. |
