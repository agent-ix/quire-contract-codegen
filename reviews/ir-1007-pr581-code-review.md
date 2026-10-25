---
id: SR-581001
title: "Code review of IR-1007 CG PR 581 (QSL checked-closure facade delivery status)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@ee51b2590f4e2909fc121fbeda082348a64fd37d; spec/assurance/AD-004-cg-crate-layout.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

## Summary

Ticket: IR-1007. PR: quire-contract-codegen#581, frozen head ee51b2590f4e2909fc121fbeda082348a64fd37d
over base f353f91224e09ee6aabc5929123139dfb3d6e671 (CG main at review time). Reviewer model:
Claude Opus 5.5 (`claude-opus-5-5`). Method: code-review. The diff changes three Markdown spec
files, with 38 insertions and 36 deletions. It has no Rust, Cargo or production code, so the
rust-review lane does not apply. The PR rewrites the "draft QSL PR #1483 / candidate" wording
as "delivered on QSL main through merged PR #1483". It drops IR-1007 from the pending gates and
keeps IR-595 as the remaining gate.

## Examined scope

- QSL #1483 merge claim: examined. The local QSL checkout was fetched from origin main. Commit
  `ec73a6b69 IR-1007: acquire and apply checked closures through qsl-replay (#1483)` is on
  main and adds `qsl-replay/src/application.rs`. The spec #1470 (`e363866e4`) is also on main.
  The `gh` token on this host was invalid (HTTP 401), so merge state comes from git history
  rather than the GitHub API.
- API citations against QSL main `qsl-replay/src/application.rs`: examined.
  `acquire_checked_closure(source, path, bytes, packages, dependencies, limits, replay_limits)
  -> Result<CheckedClosureCompilation, AdmittedRefusal>` (seven arguments, line 202).
  `CheckedClosureCompilation::handle()` (line 47). `CheckedClosureHandle::package_id`,
  `lookup_callable(owner_package_id, wire_node_id)` and `registry()` (lines 74-141).
  `CallableSelection::owner_package_id` and `signature` (lines 174-182).
  `CallableLookupRefusal::{Unknown, Foreign(DigestRecord), WrongKind}` (line 189).
  `ApplicationRegistry::register(&selection, &signature, factory)` and `begin_call` (lines
  271-284). `CheckedApplicationSession::step` and `run` (lines 313-321). Every name the diff
  cites as delivered (AD-004:364-370, FR-021:81-83, AC-32, AC-35, AC-36, AC-37) exists with
  the cited shape.
- FR-262 root-call-site behavior cited as delivered (FR-021:46-47, :464-466): examined. QSL
  main `qsl-eval/src/value/expression/closure.rs:179` implements `root_call_site`. quire-eval
  main (`a464a64f`) `crates/quire-eval/src/registry.rs:138-139` returns
  `root-call-site-absent` when it is missing. This matches FR-262-AC-13.
- CG concrete oracle and corpus remain Planned: examined. CG main and the PR head have no
  `acquire_checked_closure`, `lookup_callable` or `begin_call` in `src/` or `tests/`.
  `src/core/profile.rs:77` still calls `quire_contract_runtime::exact`. Every touched AC
  still reads PLANNED pending IR-595, which is truthful as a code fact. The tracker side of
  this gate is SR-581002 FND-001.
- AD-004 "CG's `oracle` module will use this application facade" (was "uses"): examined. This
  tense correction is accurate, because no CG code uses the facade yet.
- No duplicate evaluator, adapter or compatibility layer is introduced: examined. The text
  does not change.

## Verdict

Clean for code-review. Every delivered-API and merge claim matches QSL main and quire-eval
main. The Planned status of the CG consumer matches CG code. The stale sibling matrix index
and the closed IR-595 gate are recorded under the spec-review methods (SR-581004, SR-581002),
so they are not duplicated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
