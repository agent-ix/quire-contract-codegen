---
id: "SR-625"
title: "IR-92 slice 1 code review (incl. rust-review lane): replay package, dependencies, one evidence-failure verdict"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@9a299b1f4e7c9f22c2ac571e56fa5d68cced70f0; Cargo.toml, src/spine_replay.rs, src/kani_witness_join.rs, src/lib.rs, tests/it/skeleton_spine.rs, spec/interface/interface-001-codegen-api.md, spec/functional/complete-v1/FR-016-witness-native-replay.md"
relationships: []
---

# SR-625: IR-92 slice 1 code review

## Summary

Ticket: IR-92 (slice 1), also IR-290. PR: agent-ix/quire-contract-codegen#201, head 9a299b1,
diffed against origin/main.

The PR moves the replay-request builder out of `tests/it/skeleton_spine.rs` into
`src/spine_replay.rs` as `ReplayPackage` / `ReplayInputs` / `LockedSource` / `DependencyLock`. It
fills `package.dependencies` from the lock (IR-290), with the byte provision deduplicated by digest.
It adds `replay_counterexample`, which returns `ReplayVerdict::{Reproduced, EvidenceFailure(Decode |
Domain | Verdict)}`, and `first_out_of_domain` for bool and i64 values. The diff does not touch
quire-contract-ir, adds no dependency and copies no files.

## Method

I read the full diff and the QSL `qsl-replay` source at the locked commit 966e7d2 (`call_site.rs`,
and `execute.rs` dependency rules 1 to 7). The rust-review lane covered idioms, panics, integer
conversions, visibility, docs and test oracles. I added temporary probe tests (since reverted) and
ran them against the real QSL executor. I ran nine source mutations against the `skeleton_spine`
tests and reverted each one. Then I ran `make ci` in my own worktree.

Probe results:
- If a request carries the PR's own `dependency_lock()` and goes through `replay`, QSL returns
  `Err(Refused(DependencySelections(Unselected { identity: "test/units" })))`.
- Two locks supplied as `[test/units, test/aaa]` go onto the wire in that order, not sorted.
- Domain boundaries against bounds 0..=1000: 1000 and 0 are in domain; 1001, -1, i64::MIN and
  i64::MAX are all `Domain`. The check is correct.

Mutations. A mutant is "killed" when a test goes red:
- M1, domain check disabled: killed.
- M2, upper bound made exclusive: survived.
- M3, any category counts as reproduced: survived.
- M4, dependencies dropped: killed.
- M5, dedup removed: survived.
- M6, dependency bytes omitted: killed.
- M7, decode failure mapped to Reproduced: killed.
- M8, verdict mismatch mapped to Reproduced: killed.
- M9, version replaced by identity: killed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Filled `package.dependencies` has no path QSL admits. `ReplayPackage::new` compiles through `call_site`, which refuses any unit that imports a library. A standalone unit selects no library, so every request with a non-empty `dependencies` list is refused (`DependencySelections::Unselected`), as the probe confirmed. The only dependency test checks the wire shape of a request QSL refuses. Replaying an imported dependency is untested, and this design cannot test it. | src/spine_replay.rs:241-259, src/spine_replay.rs:285-316, tests/it/skeleton_spine.rs:290-334 |
| FND-002 | medium | Dependency entries go onto the wire in the caller's order. QSL ADR-015 D-4 rule 1 requires identities in strictly ascending UTF-8 order with no repeats. Nothing sorts, dedupes or refuses, so an unsorted lock produces a request QSL refuses as `Unordered`. | src/spine_replay.rs:287-298 |
| FND-003 | medium | A `Reproduced` settlement with a category other than `violation` is correctly mapped to `EvidenceFailure`, but no test pins it: mutant M3 (`(ReproducedWithEvaluatedWitness, _) => Reproduced`) keeps every test green. | src/spine_replay.rs:410-418 |
| FND-004 | low | Mutant M2 survives: no test sits at a domain boundary (min, max, max+1, min-1) or at i64::MIN/MAX. The probe shows the behaviour is correct today. | src/kani_witness_join.rs:160-179, tests/it/skeleton_spine.rs:376-392 |
| FND-005 | low | Mutant M5 survives: no test has a source shared between the proved unit and a dependency, or between two dependencies, so byte-provision dedup is unpinned. The dedup is also a quadratic `Vec` scan where a `BTreeMap` keyed by digest would state the invariant. | src/spine_replay.rs:303-316 |
| FND-006 | low | `DependencyLock::package_id` and `ReplayInputs::backend_manifest` are `DigestRecord`s, which carry a domain. The wire hard-codes `PackageSemanticV2` and `ToolManifestJcsV1` and ignores `record.domain()`, so a record minted under another domain is silently relabelled. | src/spine_replay.rs:293-296, src/spine_replay.rs:330-335 |
| FND-007 | low | The new public `EvidenceFailureCause::Decode(KaniOutcome)` binds a Contract IR type into the new verdict surface, which adds another IR type the later migration off IR must retire. | src/spine_replay.rs:345-351 |
| FND-008 | low | The only test with a real Kani counterexample (the `make kani` lane) still calls `replay_falsification`, not `replay_counterexample`. The new verdict function sees only synthetic playback transcripts. | tests/it/skeleton_spine.rs:451-531 |
| FND-009 | low | `ReplayRefusal::Fault` comes back from `replay_counterexample` as `Err(SpineReplayError::Refused)`, the same variant as a request refusal. FR-016 names it a separate "replay unavailable" result (AC-12). This is outside the slice's scope, but the new verdict API does not separate it. | src/spine_replay.rs:376-409 |

## Verdict

Not mergeable yet. Three medium findings need fixing in this PR or an explicit disposition:
- FND-002 is a real defect in a new public API. Sort the entries by identity, or refuse unsorted
  or duplicate ones.
- FND-003 needs a test. A pure `(settlement, category) -> ReplayVerdict` function with a unit test
  would do it.
- FND-001: either add a test that asserts QSL's refusal and documents the `call_site` limit, or
  state plainly that IR-290 is not complete until QSL's `call_site` accepts a dependency input.

What went right:
- No new `unwrap`, `expect` or panic in src, and no bare `as` casts. `i64::from(bool)` is the only
  conversion.
- The decode, domain and verdict mapping is correct, and mutants M1, M7 and M8 are killed.
- The domain check is inclusive and correct at every boundary probed.
- Filling the dependencies copies identity, version, package_id and sources field for field
  (M4, M6 and M9 killed).
- The diff does not touch IR and adds no QSL-named item there. It adds no dependency, vendors
  nothing and adds no compatibility layer.

Gates: `make ci TRUSTED_HOME=<scratchpad dir symlinking ~/.cargo and the nvm quire>` exited 0.
It ran fmt-check, spec, clippy -D warnings, msrv test (218 passed, 5 ignored), deny, the one-copy
check, audit-unsafe, rustdoc and test (80 + 218 passed, 5 ignored).
