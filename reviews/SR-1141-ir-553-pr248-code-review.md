---
id: SR-1141
title: "code review of quire-contract-codegen PR 248 (IR-553 function-path obligation identity)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@360d58dae87a4486b4bcad945b8b1f4461e6e5f6; src/replay/obligation.rs, src/replay/function.rs, src/core/canonical.rs, src/core/mod.rs, src/replay/mod.rs, src/kani/identity.rs, src/lib.rs, tests/it/skeleton_spine.rs, Cargo.toml, Cargo.lock"
review_set: subset
---

## Summary

Ticket: IR-553. Code review with the Rust lane (rust-review) of `git diff origin/main...HEAD`
(base 7f0d6d0). The PR adds `ReplayPackage::obligation_identity`, which digests a preimage
`{function, declaration:{node,role,ordinal}, kind, arguments:[{parameter, domain}]}` through
`quire-canonical` (RFC 8785, SHA-256, no domain label) and puts it in the function path's
request slot instead of the transcript's `ByteDigest`.

Examined and clean: `core::canonical` is the only `quire-canonical` call site; typed errors
(`DigestError`, `ObligationIdentityError`, `SpineReplayError::Identity`), no new panic site
outside tests; the identity reads only `FunctionSite.function`, `.declaration` and
`.parameters` (for the identifier join) and derives no node id or occurrence key; arguments
are sorted ascending by binding identifier, matching O-09 and AD-016 arrow 5 and the
`BTreeMap` order the generator already uses; `ArgumentDomain` bounds are decimal strings, so
i64 extremes encode; `FixedShape` derives are the serde path the crate documents; the
`quire-canonical` dependency (`=0.3.0`, `branch = "main"`) follows the first-party git
convention, the lock gained only the edge (same b4bb97a5 entry QSL already pulled), the
one-copy check passes, and no SHA is pinned. No production caller passes a fixed identity or a
transcript digest: `replay_counterexample` is the only prod caller of `request`; the frame
path's caller-supplied identity is pre-existing and documented. The heredoc-appended test
module in `obligation.rs` is well-formed and formatted.

Gates run at the reviewed sha: `make fmt-check` pass, `make lint` pass, `make test` pass
(125 + 277 passed, 15 ignored, 1 passed), `make deny` pass (advisories, bans, licenses,
sources ok; one-copy ok).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `contract_arguments` maps the harness bindings only and never checks that they cover every `FunctionSite` parameter, so a harness that binds a strict subset of the function's parameters gets a "function-contract identity" over that subset; O-09 says the function contract's `arguments` are the function's parameters. Scenario: function `f(a, b)` with a harness binding only `a` yields an identity with one argument and no error | src/replay/obligation.rs:104-124 |
| FND-002 | low | `ArgumentDomain::of` maps an `I64` binding with no `integer_bounds` to the full i64 range, inventing a declared domain; AD-016 arrow 5 says an unbounded domain with no declared bound is `requires-bound` and never narrowed implicitly. The generator never builds that binding today (clause.rs and v1_bundle.rs always set bounds for `I64`), so the arm should be a typed refusal, not a value; `(Some(bounds), Boolean)` is likewise silently read as a range | src/replay/obligation.rs:78-82 |
| FND-003 | low | `ReplayPackage::request` is `pub` and takes any `ObligationIdentity`, so the function path's slot is correct only if each caller first calls `obligation_identity`; the integration tests already fill it with a fixed `slot()`. Taking the harness `KaniObligationIdentity` (or making `request` crate-private behind `replay_counterexample`) would make a wrong slot unrepresentable on the function path | src/replay/function.rs:443-455 |
| FND-004 | low | `content_digest`'s doc says O-09 puts the identity's digest domain outside FR-201 as if settled; O-09 records that as current state with QC-4 / TK-07 (QSpec) open to add an FR-201 domain, and ADR-013 section 2 "normalized" equality is "under the same digest domain". When TK-07 lands every identity changes; the comment should say the unlabelled digest is interim | src/core/canonical.rs:25-26 |

## Verdict

The implementation follows O-09's members, AD-016's ordering and ADR-013 section 2's one
encoder, and the gates pass. FND-001 is a real gap on a mismatch edge and should be fixed in
this PR (a typed refusal when the bindings are not exactly the site's parameters). FND-002
to FND-004 are small hardening and doc fixes. Mergeable after FND-001 is resolved.

## New findings (disposition pass 1)

Reviewed at 5aee2cf161b3b1068c3d70d4de895de0cba8a377. Gates: fmt-check, lint, test (126 + 280
passed, 15 ignored, 1 passed), deny all pass.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | `replay_counterexample_through` is a new `pub` seam re-exported from `lib.rs` whose only caller is one integration test; `src/replay/function.rs`'s unit-test module already has a `package()` fixture, so a `pub(crate)` seam tested there would cover the sent slot without adding permanent public API (it is also absent from interface-001, see SR-1143 FND-003) | src/replay/function.rs:558-570 |

## Dispositions

Round 1, reviewed at 5aee2cf161b3b1068c3d70d4de895de0cba8a377.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5aee2cf |
| FND-002 | fixed | 5aee2cf |
| FND-003 | fixed | 5aee2cf |
| FND-004 | fixed | 5aee2cf |

Round 2, reviewed at a485e43b7622df28214062afbbad450ce40c5588 (delta from 5aee2cf is spec text only).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | accepted-no-change | The seam stays pub: the AC-21 slot test needs the `KaniObligationIdentity` and playback fixtures that live in `tests/it`, and a `pub(crate)` seam would mean duplicating them in the unit tests. The seam is now documented in interface-001 with that reason and is a thin wrapper that ends in `qsl_replay::replay`; the cost is one extra public function in a prerelease crate. |
