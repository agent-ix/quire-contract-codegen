---
id: SR-17019
title: "IR-348 CG564 EARS analysis of the changed FR-014 requirement statement"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-348; PR564 fix round; exact da6d12a72e92f080eaae4887dfd52b122c4777a5 (fix diff 1aa7807f..da6d12a7); spec/oracle/functional/FR-014-exact-scalar-oracles.md:525-531"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
---

## Summary

Ticket: IR-348. This EARS analysis covers the single requirement statement the fix round changed: the FR-014 panic-token statement at line 525. It is a ubiquitous EARS requirement, "The <system> shall <response>", with system "The scalar generator, `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`, and every crate it emits" and response "contain no panic token in non-test code". It has one `shall`, no trigger or state clause, and a closed, enumerated token set, so it is singular and verifiable (TC-024 panic-scan step 1).

The other changed lines, FR-014-AC-39 and FR-014-AC-42, are acceptance-criteria table rows, not requirement statements, so EARS does not apply to them. Their wording is covered in SR-17018. `quire validate` reports no EARS diagnostic in the changed lines. The only EARS warning in the file, at line 269, is pre-existing and was not changed.

## Verdict

PASS, clean. The changed statement conforms to the ubiquitous EARS pattern.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The examined statement is FR-014 line 525-531. The line-269 warning, `[ears:non-canonical-trigger] non-EARS trigger before`, is present at `1aa7807f` and `da6d12a7` alike and lies outside this scope.

Plan completion: not assessed

## Examined scope and bindings

```yaml
clean: true
scope:
  - {id: FR-014-statement-525, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "The scalar generator, `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`, and every crate it emits shall contain no panic token in non-test code"}
findings: []
bindings: []
```
