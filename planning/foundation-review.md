---
id: REV-002
title: "Codegen foundation composite review"
type: Review
---

# Foundation composite review

Date: 2026-08-30

Scope: codegen issue #1, epic children #2 through #6, the specification under `spec/`, PGM-01, IR
corpus issue #10, and runtime issue #3 / draft PR #5.

| Review dimension | Result | Evidence or disposition |
|---|---|---|
| Dependency | draft-clear | PGM-01 PR #12 and runtime PR #5 are pinned and provisionally reconciled; the IR corpus has no candidate. Every provisional identity must be reconciled after merge. |
| Risk | clear | AP-001 and AD-001 enumerate semantic drift, silent loss, nondeterminism, proof gaps, vacuity, atomicity, and provenance risks. |
| Evidence | clear | MP-001 fixes populations, repetitions, identities, failure retention, and the distinction between foundation and semantic evidence. |
| Integrity | clear | Shared lowering plan, no success fallback, proof dependency closure, source-region observation, and atomic publication are required. |
| Scope | clear | IR parsing/canonicalization, external engines, customer code, Quoin/Quire integration, and release authority are excluded. |
| Failure domains | clear | Invalid, unsupported, unavailable, rejected, discarded, failed, inconclusive, I/O, vacuous, and differential states remain distinct. |
| Licensing/provenance | clear | Crate and generated Rust are `MIT OR Apache-2.0`; every artifact carries the PGM identity envelope; publication remains disabled. |

No unresolved foundation-specification finding was identified. Upstream dependency closure, human
review, implementation evidence, and the source-release decision remain explicit gates rather than
being represented as success.
