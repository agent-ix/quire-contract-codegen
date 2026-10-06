---
id: SR-2210
title: "IR-665 namespace orphan adoption race code review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@98337b0ab10d876d5995c329031701a47813391b; src/kani/run/namespace.rs (tests module: child_with_nspid, completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample); base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60; context: src/kani/run/namespace.rs NamespaceOwner::cleanup and Drop, src/kani/run/memory.rs MemoryObserver::observe, FR-028-AC-21, FR-017-AC-24"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
---

## Summary

Ticket: IR-665. PR: quire-contract-codegen#311, frozen head 98337b0ab10d876d5995c329031701a47813391b over base 5d3eaa2bbedcfbd59d8bd3d8df681b70e74cad60. The diff touches one file, and only its `#[cfg(test)]` module. This one artifact holds both the code-review and the rust-review lanes. I read the repo's CLAUDE.md, AGENTS.md and CONTRIBUTING.md and loaded the Rust review skill before analysis.

The change holds the orphan's intermediate parent until the caller writes `adopt-now`. The orphan now publishes its ready file with a temp-file write and an atomic rename, and it does so before adoption. The caller asserts that INIT has not yet adopted the orphan, releases the parent, and then polls `children(init)` for up to 5 s for the orphan's NSpid. The failure message names the target NSpid, INIT and the observed children.

## Verdict

**CONDITIONAL.** The held parent → no adoption → release → adoption sequence is built for real, and the pre-release `is_none()` check is deterministic. While the intermediate spins on `adopt-now`, the orphan's parent is that intermediate, so it cannot appear in INIT's children list. After release, the poll is bounded and checks the deadline after each fresh observation. The late-fork, `fork-now`/`forked` barrier, init pinning, completion wait, owned-cleanup and fallback-teardown assertions below the lookup are unchanged byte for byte. No assertion or oracle was weakened.

Failure paths still clean up. A panic in either new assertion drops `NamespaceOwner`, whose `Drop` runs `cleanup()`, so INIT receives SIGKILL and the spinning fixture processes go with the namespace. The observer still walks the whole owned subtree, so taking the last sample before adoption leaves the "later fork is not sampled" property intact.

I checked the sibling single-shot lookups in this module. `children(orphan)[0]` runs after `forked`, and fork links the child into its parent's list before the child runs. `children(info.child).is_empty()` sits behind the pre-claim gate. Neither has the same race.

Three findings remain, one medium and two low. The PR also changes the ready-file publish to a temp-file write plus atomic rename. That change removes the old failure mechanism most directly reachable from the base fixture, which is reading an empty `orphan-ready`. No assertion guards that change, and the comments do not mention it.

This is a source-only review. I ran no build, test, Kani or replay, and I do not claim that the old flaky interleaving was replayed or proven. The lead owns gate execution.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The ready-file PID is never checked to be a non-empty numeric NSpid. Empty or garbled content makes the held-parent `is_none()` assertion vacuous and turns a ready-file publish defect into a misleading "not adopted" deadline failure. The atomic temp-and-rename publish that prevents this has no guard. | src/kani/run/namespace.rs:622-628 |
| FND-002 | low | The new comment says the held parent reproduces "the previously raced direct-child lookup". In the base fixture the orphan waited for `getppid()==1` before it created the ready file, so a pre-adoption lookup was not directly reachable there. Python's non-atomic `write_text` (create/truncate, then write) let `wait_for` + `read_to_string` read `""`, which matches no NSpid and would also produce the observed `Option::unwrap` failure. | src/kani/run/namespace.rs:624 |
| FND-003 | low | The deadline failure lists the children's host PIDs but not their NSpid lines or status-read results, and `child_with_nspid` silently treats an unreadable status as a non-match. A failure therefore cannot tell "orphan absent" from "orphan present but NSpid unreadable or different". | src/kani/run/namespace.rs:636-639 |

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@be267fe33944f8977f04263bf0851f07d019a14f (fix commit over prior 98337b0ab10d876d5995c329031701a47813391b). Source-only re-check: no build, test, Kani or replay run, and no acceptance credit for code or gates. The fix round introduced no new finding. All original assertions below the lookup (late fork, barrier, init pin, completion wait, owned cleanup, fallback teardown) are unchanged.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | be267fe33944f8977f04263bf0851f07d019a14f |
| FND-002 | fixed | be267fe33944f8977f04263bf0851f07d019a14f |
| FND-003 | fixed | be267fe33944f8977f04263bf0851f07d019a14f |
