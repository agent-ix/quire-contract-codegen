---
id: "SR-780"
title: "CG PR 228 code review: AD-004 step 2e, evidence/ and strategy/ module move"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@1338e38cd96066bf7fe89d88ed5fcb842dd20c66; src/ (the 12 src files the PR touches: lib.rs, strategy/{mod,harness,campaign}.rs, strategy/bound/{mod,census,generation,population,relation}.rs, evidence/{mod,vacuity,bound_coverage}.rs), tests/it/{bound_census,bound_populations}.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-780: CG PR 228 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#228 at 1338e38, one commit on base
origin/main 879de44. A pure-motion PR. The review checks that only paths changed, that output
is byte-identical, that the public API changes only as AD-004 step 2e allows, and that the new
module layout is idiomatic. The Rust lane is folded in, as `rust-review` says.

Measured by the reviewer:

- Pure motion. `git diff -M`: `vacuity.rs` and `bound_strategy/relation.rs` 100%;
  `bound_coverage.rs` 98%; `census`, `generation`, `population`, `campaign` (was `strategy.rs`),
  `harness` 99%; `bound_strategy/mod.rs -> strategy/bound/mod.rs` 71% (the four `pub mod`/`mod`
  lines became `pub(crate) mod` and the `pub use` line was removed). 14 files, +80/-53. Every
  changed line in a moved file is a `use` line, a `mod` visibility, the `include_str!` path or one
  intra-doc link. Nothing was reordered and no logic changed.
- `include_str!` (`evidence/bound_coverage.rs:22`): `../` became `../../` because the file moved
  one directory deeper. Both resolve to `schemas/bound-coverage-observations-v1.schema.json`. The
  file is unchanged, there is no `src/schemas/`, and the byte-identity run below covers it.
- Root-path imports in the moved files. Every `use crate::` in the eight moved files is now a
  `core::`, `oracle::`, `strategy::` or `evidence::` module path. The root imports `classify_clause`,
  `parse_llvm_coverage`, `BoundOracleGeneration`, `GeneratedArtifactBundle`, `OracleRequest`,
  `StrategyDiagnostic`, `StrategyErrorCode` and `generate_bound_oracles` were rewritten. No
  `crate::<Item>` path is left in code or doc links. Test modules use `super::*`.
- Visibility. `strategy/mod.rs` and `evidence/mod.rs` hold only `pub(crate) mod` lines. At base
  these modules were private `mod`s of the root, which the whole crate could already see, so
  `pub(crate)` widens nothing. `strategy/bound/generation` went from private to `pub(crate)`. That
  is needed because `lib.rs` now names `strategy::bound::generation::{...}` directly, after the
  `pub use` in `bound/mod.rs` was removed. The alternative, a `pub(crate) use` in `bound/mod.rs`,
  would add a re-export layer that AD-004 does not ask for. The result matches `oracle/mod.rs`
  (step 2d), and `bound` itself is `pub(crate)`, so nothing becomes visible outside the crate. No
  item's visibility changed.
- Public API. The crate-root `pub use` name set went from 244 to 272 names: exactly +28 and -0,
  plus the removal of `pub mod bound_strategy`. The 28 are every top-level `pub` item of
  `census` (12), `population` (11) and `relation` (5). The only other `pub` items in those files
  appear inside generated-source string templates. There are no duplicate names (an explicit
  `pub use` list would not compile with one), no glob, and no shim at the old path. rustdoc
  `all.html` was built at base and at head with `RUSTDOCFLAGS=-Dwarnings cargo doc --locked
  --no-deps`. Both builds exit 0 with no warnings and list 274 entries. The (page, name) sets are
  equal once the module prefix is stripped, and the only difference is 31 `bound_strategy/...`
  hrefs that became root hrefs. The coder's note about one rustdoc warning (IR-500) does not
  reproduce under `-Dwarnings` public docs.
- Byte identity, reproduced independently on the final head. An uncommitted hook in
  `Artifact::new` and `ArtifactBundle::new` wrote one JSON line per artifact
  (`["new"|"bundle", artifact]`, per-process files under a mutex) in two scratch worktrees outside
  the repo (base 879de44, head 1338e38). Then `cargo test --locked` ran in each. Both runs passed
  115 unit, 252 integration (9 ignored, the Kani lane) and 1 doc test, so the test count is
  unchanged. Sorted dumps: 7,395 records each (7,125 `new` and 270 `bundle`), 166,528,628 bytes,
  sha256 `6ae57ef2...58a4`. `cmp` finds them identical. The encoding differs from the coder's, so
  the figures are not comparable, but both runs found identity. The scratch worktrees are deleted.
- Direction. `strategy/*` imports only `core`, `oracle` and `strategy`. `evidence/*` imports only
  `core`, `oracle` and `evidence`. Neither imports the other, as AD-004 requires
  (`strategy --> oracle, core`; `evidence --> oracle, core`; they are peers). Outside `lib.rs`, no
  crate module imports `strategy::` or `evidence::`. `BoundPackage`/`BoundClause` are named only in
  `strategy/bound/{mod,generation}.rs`, `evidence/bound_coverage.rs`, `oracle/bound_v1.rs` and
  `kani_obligations.rs` (the V1 arm that step 4f deletes), so AD-004:307-309 holds.
- Gates. `make ci` exit 0 and `make kani` 9 passed at 1338e38 are the coder's. Both logs were read
  (`cg-2e-ci.log` and `cg-2e-kani.log`, each ending `head=1338e38... exit=0`). Kani was not re-run.
  The PR is a `git mv` plus path edits with output byte identity reproduced on the final head, so
  it changes no harness text.

## Verdict

PASS with one low finding. The move is pure. Generated output is byte-identical at base and the
final head. The public API changes exactly as AD-004 step 2e directs: the `bound_strategy` path
leaves, and its 28 items are re-exported by name with none dropped. The module layout is
idiomatic. Re-exporting all 28 rather than only the 24 the tests use is right: the other four
(`CensusCase`, `Interval`, `PartnerRule`, `EXPECTATION_FIELD`) appear in the public types and
field shapes of the other 24, so making them private would leave public items that cannot be
named. FND-001 is a cheap in-PR fix of the same kind SR-775 FND-002 fixed in step 2d. The PR is
mergeable as is, or after FND-001 if the lead wants consistency with 2d.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Migration history written into permanent module comments. `lib.rs` says "The evidence subsystem (AD-004 step 2e)." and "The strategy subsystem (AD-004 step 2e).", and `strategy/mod.rs` says "Enum and `i64` strategy campaigns (was `strategy`)". In step 2d the same pattern was a finding (SR-775 FND-002) and was fixed in 903a351: `lib.rs:21` now reads "The oracle subsystem." Drop the history clauses so the comments say what the module owns. | src/lib.rs:42, src/lib.rs:44, src/strategy/mod.rs:9 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 45cdc42 (round 1, reviewed at 45cdc42: `git diff 1338e38 45cdc42` touches three comment lines only, with no code token changed: `src/lib.rs:42,44` now read "The evidence subsystem." and "The strategy subsystem.", and `src/strategy/mod.rs:9` reads "Enum and `i64` strategy campaigns; no V1 input." No history clause is left in the lines this PR adds. The pre-existing step 2b/2c clauses at `src/lib.rs:9,16,18` predate this PR and are out of its scope.) |
