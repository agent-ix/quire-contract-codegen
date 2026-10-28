---
id: SR-616001
title: IR-1091 CG PR 616 feasible OrderedSet uniqueness code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR 616 (stacked on PR 613); diff 731029ed..0d685342; src/kani/generate/collection.rs, src/kani/generate/render.rs, src/kani/generate/spec.rs, src/kani/identity.rs, tests/it/kani_obligations_symbolic_domains.rs"
review_set: subset
---

## Summary

Ticket: IR-1091. Reviewed frozen head 0d6853429fe91b4f663edcf8f51b5e32cde622e4 against stacked base 731029ed4fb11f4b9dbacea74a5329e44b6ebb30 (PR 613 head), not origin/main. Applied code-review with the rust-review lane (rust-style baseline plus repository CLAUDE.md/AGENTS.md conventions). Context read: FR-015-AC-96, -97, -99, -100, -101, -102, TC-025, AD-004 shared-renderer rule, and the quire-contract-model `CheckedMemberType::Collection` / `CheckedCollectionKind` accessors at the locked IR revision.

## Verdict

CONDITIONAL. The production path is sound for the claimed slice: the kind comes from the admitted IR field (`CheckedCollectionKind::OrderedSet`) and is cross-checked against the graph's `ordered_set` semantic form; the over-capacity guard runs before any harness is rendered and returns a typed refusal (no vacuous proof, and the backend's cover classification would report a vacuous run as `VacuousProof`, not `Verified`); `unique_domain_can_hold` is overflow-safe across the full i128 span and the i64 harness range is refused, never narrowed, by `collection_i64`; the uniqueness assumption is confined to active slots (`prior < index < len`) and needs no extra unwind beyond `maximum_length + 1`; order is preserved by array index; `kind` enters both the symbol digest material and `CollectionObligationIdentity`. No compatibility layer, no copied or vendored files, no new hash or pin. The one medium finding is test sufficiency of the installed-Kani control, not a production defect.

Focused checks run by the reviewer at this head: `cargo test --locked --lib -- unique_capacity every_identity_component` 2 passed; `cargo test --locked --test it -- kani_obligations_symbolic_domains` 23 passed, 2 Kani-lane ignored. Installed Kani was not rerun (no concrete runtime risk beyond FND-001, which is a test-design gap). No full gate was run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The installed-Kani ordered-set control cannot detect an inert uniqueness assumption: healthy `item >= 0` verifies and mutant `item >= 1` falsifies for a plain Sequence too, and the pairwise-distinct check on one counterexample passes whenever the solver happens to choose distinct values. Add a uniqueness-sensitive control whose verdict flips between Sequence and OrderedSet (for example Boolean ordered set with lower 2 and `exists(flag: flag == true)`, or IntRange 0..=3 with lower 3 and `exists(item >= 2)`). | tests/it/kani_obligations_symbolic_domains.rs:2210 |
| FND-002 | low | `_ => return Err(UnsupportedFieldType)` matches a closed four-variant `CheckedCollectionKind` (not `#[non_exhaustive]`); naming `Set | Bag` keeps a future IR variant a compile-time decision instead of a silent refusal. | src/kani/generate/collection.rs:564 |
| FND-003 | low | `render_collection` spells `crate::kani::identity::CollectionKindIdentity::Sequence/OrderedSet` inline while `spec.rs` imports the type; import it in render.rs to match the module's use list. | src/kani/generate/render.rs:212 |

## Coverage

Examined `resolve_collection_field` (kind mapping, element domain, cardinality, capacity guard), `unique_domain_can_hold`, `collection_element_type`, `render_collection` in collection.rs (unwind, memory ceiling, digest material, identity), `CollectionSpec` and the renderer's uniqueness loop, `CollectionKindIdentity` serde, and the five changed/added tests plus `collection_playback_values` (truncates to active slots, so the distinct check is not polluted by inactive slots).

Over-capacity behaviour: `max > distinct capacity` returns `CollectionDomainRefusal::UnsupportedFieldType` before rendering; the test asserts `Unsupported { CollectionClaimNotRendered { UnsupportedFieldType } }` and no harness, and is deliberately left untagged. The PR does not claim the held PR 615 path. Whether that refusal should carry a distinct typed reason naming the infeasible cardinality is a residual owner decision tied to PR 615, not a finding here.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix head 9f6a3b087e64c39147946ae59f3a7b4ef138e2de adds `tc_025_real_kani_ordered_set_uniqueness_changes_the_verdict`: Boolean flags with cardinality [2, 2] and `exists(flag != false)`. Under OrderedSet only [false,true] and [true,false] are admissible, so the claim verifies; the identical Sequence admits [false,false] and must falsify with exact playback `[len 2, 0, 0]`. An inert or wrong uniqueness assumption fails the OrderedSet `Verified` assertion, and the cover guard rules out a vacuous pass. Reviewer reran it at this head with installed Kani 0.68.0 / CBMC 6.11.0: 1 passed in 15.6s. |
| FND-002 | fixed | `CheckedCollectionKind::Set | CheckedCollectionKind::Bag => return Err(UnsupportedFieldType)` replaces the wildcard; the match is now exhaustive over the four IR variants. |
| FND-003 | fixed | render.rs imports `kani::identity::CollectionKindIdentity` in its `crate::{...}` use list and matches `CollectionKindIdentity::Sequence` / `::OrderedSet`. |

Fix-diff review (0d685342..9f6a3b08) found no new code defect. In `tests/it/skeleton_spine.rs` the two assertions now wrap their expected verdicts in `Some(...)`, following QSL 1fa0f255 ("handle optional parity verdict accessors"), which is in the bumped range; the assertions are not weakened. The Cargo.lock change moves the QSL crates from dc267af to 579d4f40 (68 commits ahead, 0 behind, including QSL-814 #1478 and #1480). That pulls in the quire-eval 5317524d bump, the new transitive `quire-checked` and quire-semantic-value 0f922ad1. Contract IR, exact, runtime and verification-contract entries are unchanged, and `scripts/check_one_copy.awk` passes. Reviewer focused checks at this head: `kani_obligations_symbolic_domains::` and `skeleton_spine::` 54 passed, 3 ignored.
