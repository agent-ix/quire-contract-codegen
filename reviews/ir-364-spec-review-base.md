---
id: SR-4526
title: "IR-364 base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@f22eb3b85feb831db50d8c0aa6b34ff0df1ff646; spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md, spec/strategy/functional/FR-013-it010-consumable-output.md, spec/strategy/matrix/TC-017-bound-domain-admission.md, spec/strategy/matrix/TC-022-it010-consumable-output.md, spec/strategy/matrix/tests.md"
review_set: subset
---

## Summary

IR-364: reviewed the seven Markdown files changed by PR #334 at f22eb3b85feb831db50d8c0aa6b34ff0df1ff646. The consumer fixture contract has one inconsistent dependency statement.

## Verdict

**CONDITIONAL** — one medium finding requires a wording fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-013-AC-1 treats generated Rust as a Cargo manifest dependency, but TC-022 specifies a manifest with only proptest and quire-contract-runtime. The fixture dependency contract has two incompatible readings; say that the Rust source is included in the crate, and reserve manifest dependencies for crates. | spec/strategy/functional/FR-013-it010-consumable-output.md:56; spec/strategy/matrix/TC-022-it010-consumable-output.md:19 |

## Scope checked

Exact head: f22eb3b85feb831db50d8c0aa6b34ff0df1ff646. The review checked spec/assurance/AD-001-codegen-architecture.md, spec/assurance/AD-004-cg-crate-layout.md, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md, spec/strategy/functional/FR-013-it010-consumable-output.md, spec/strategy/matrix/TC-017-bound-domain-admission.md, spec/strategy/matrix/TC-022-it010-consumable-output.md, spec/strategy/matrix/tests.md.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 82a0ce9ae4eddc7c72f22e8fe5eecf2c6a6e2e23 |
