---
id: SR-12811
title: IR-833 spec-review-ears supplemental review
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-contract-codegen; IR-833; PR #416 own delta against #415; spec/oracle/functional/FR-014-exact-scalar-oracles.md,
  spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md'
review_set: subset
---

## Summary

Ticket: IR-833. Supplemental independent review of the frozen candidate, scoped to this PR’s own delta. PASS for edited requirement statements and boundary clarifications. The changed FR-021 traversal paragraph preserves the named generator and shall obligations; only the source of charge order changes to FR-021-AC-5. The edited boundary prose is a concrete restriction on claims and evidence, not an event/state trigger requiring conversion. No edited shall statement gains multiple responses, a missing subject, vague response or wrong trigger.

## Reviewed scope

```yaml
scope:
- id: FR-014-AC-37
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A differential corpus covering every node FR-014-AC-35 names compiles
    against the runtime alone, and each oracle''s outcome, admitted charges and consumed
    counters equal direct runtime execution. This criterion claims no agreement with
    the QSL value authority: that boundary is owned by quire-integration IR-842 (generated-oracle
    against QSL agreement), and FR-014-AC-47 keeps CG''s artifacts from claiming it.'
- id: FR-014-AC-47
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of every oracle-agreement
    corpus of FR-014, FR-018 and FR-021 (the TC-185, TC-186, TC-187, TC-192 and TC-193
    vectors, the FR-014-AC-37 corpus, the composite-equality corpus and the function-application
    corpus), no TC-024, TC-029 or TC-031 step and no oracle-matrix row for those cases
    or corpora claims, or presents as evidence, agreement with the QSL value authority;
    text that states this boundary is permitted.
- id: FR-018-AC-2
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'On every vector of the composite-equality corpus, the generated oracle''s
    `Outcome<bool>`, its admitted charge sequence and its consumed counters are equal
    to those of `quire_contract_runtime::exact::TypeEnvironment::check_equality` plus
    `CheckedEquality::evaluate` invoked directly on an independently constructed environment,
    operands and fresh `Meter`. This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47).'
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'For each function-application corpus vector, the oracle''s outcome, admitted
    charge sequence and consumed counters equal a direct application independently
    assembled from the request with a fresh meter; today that application is the Contract
    Runtime `exact` port (historical), and its owner-path replacement is settled by
    QSL-358/IR-349 (IR-845). This criterion claims no agreement with the QSL value
    authority: that boundary is owned by quire-integration IR-842 (FR-014-AC-47, FR-021-AC-18).'
- id: FR-021-AC-3
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Each scheduled item records the requested function name and declaring V2
    function node id, plus its request-order function index; any independent application
    a test builds for comparison is constructed from the request, never read back
    from generated code. The owner API origin mapping is verified when that API is
    available. PLANNED (QSL-358/IR-349 for origin mapping).
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: The oracle returns the arity, wrong-value-kind, dangling-reference and
    unknown-function input refusals unchanged to the caller before any execution charge;
    the schedule records ordered arguments without inventing a refusal result. That
    the refusals equal QSL-owned ones is outside CG's boundary (IR-842, FR-014-AC-47).
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: No test name, trace tag, test output line or doc comment of the function-application
    corpus, no TC-031 step and no oracle-matrix row claims, or presents as evidence,
    agreement in outcome, refusal, charge order or consumed counters with the QSL-owned
    application authority; text that states this boundary is permitted. That agreement
    is not claimed by CG and is owned by quire-integration IR-842 (FR-014-AC-47).
- id: FR-021-AC-2
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Compare only with the former runtime port or build the direct application
    from emitted source, so two matching errors pass.
- id: FR-021-AC-18
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: Claim or present as evidence agreement with the QSL-owned application authority
    in a test name, trace tag, output line, doc comment, TC-031 step or matrix row.
- id: spec/oracle/functional/FR-021-function-application-oracles.md#statement-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'This is the function-application slice of complete-V1 oracle generation.

    [FR-014](./FR-014-exact-scalar-oracles.md) supplies scalar forms and

    [FR-018](./FR-018-composite-equality-oracles.md) supplies equality forms.

    FR-018''s entry point still refuses `call` nodes; this entry point alone

    classifies function application. Runtime outcome and charge amounts are not

    decided here, and CG claims no agreement with the QSL value authority on them

    (FR-014-AC-47; owned by quire-integration IR-842), while this requirement

    decides the schedule''s operand and call order.'
- id: spec/oracle/functional/FR-021-function-application-oracles.md#statement-12
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "- When body analysis begins, the generator shall apply the existing duplicate\n\
    \  declaring-node, ambiguous-name, duplicate-request, unknown-function,\n  unsupported-capability\
    \ and admitted package-accounting precedence of AC-1,\n  AC-6, AC-12, AC-22, AC-23\
    \ and AC-24. An admitted failed call-node record\n  wins over that item's function-body\
    \ record; if Contract IR emits no package\n  because of its byte ceiling, FR-039\
    \ refuses the call before item generation.\n  A refused item contributes no schedule\n\
    \  root or concrete oracle symbol; a healthy sibling keeps its disposition.\n\
    - For a declared function "
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for edited requirement statements and boundary clarifications. The changed FR-021 traversal paragraph preserves the named generator and shall obligations; only the source of charge order changes to FR-021-AC-5. The edited boundary prose is a concrete restriction on claims and evidence, not an event/state trigger requiring conversion. No edited shall statement gains multiple responses, a missing subject, vague response or wrong trigger.

## Coverage

Deterministic engine validation of the three edited FRs completed successfully: 2/3 grammar-clean, one ears:non-canonical-trigger warning at FR-014 line 254. That Before production generation begins statement is byte-identical to the parent, outside this PR's changed statements, and is not a newly introduced finding. Acceptance-criterion excerpts are listed for scope/context; this grammar method does not replace criterion-strength or evidence review. Existing base, integrity, evidence and scope-boundary methods were reused rather than rerun.
