---
id: SR-202610310133
title: IR-1091 PR 694 gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; IR-1091 PR #694; whole-object collection
  operation context; frozen head 223c4a85'
review_set: subset
---

## Summary

Ticket: IR-1091. Independent frozen-head review of CG PR #694.

## Verdict

FAIL — findings require disposition before merge.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The whole-object operation tests exercise AC-38/39/43 behavior but tag only AC-96/99 (and some AC-101/102); computed matrix reports AC-38, AC-39 and AC-43 untagged. Bind discriminating declared-order/state/parameter/result and sibling controls to these implemented criteria. Do not hand-edit the matrix. | tests/it/kani_collection_operation.rs:136-137,377-378 | correct-requirement-no-evidence |

## Coverage

Examined canonical fields() declaration order and declaring_node_id, exact Value state and unread fields, schema-aware builders/inspectors, all produced-domain checks, typed parameter/result ABI, sibling assumptions/refusals, retained prestate and authored conjunction/equality evaluation, symbolic uniqueness/domain/resource precedence, playback ordering, identity material, and compiled real-exact test driver. No applicable AssuranceProfile exists under spec/. No code or repository edits were made. No cargo gate was run by reviewer while shared baseline lock was held; parent owns final-head checks. git diff --check passed. Installed-backend/native replay proof credit was not claimed. Existing proof-equivalence lane #654 remains separate.

Plan completion: not assessed. quoin matrix completed: 556 tagged, 361 untagged, 93 method-without-symbol, 12 tagged-by-ignored-test across 46 requirements. Global pre-existing gaps prevent a repository-wide PASS; scoped operation AC-96/99/101 are tagged and AC-38/39/43 are not. Whole-tree public surface inventory and mechanical stub scan were sampled, not exhaustively proven; this is no blanket repository completeness claim. Optional semantic gap-analysis fanout was not run; code-review directly examined normative behavior. New production behavior maps to FR-015 and no new unexplained public entry point or hollow implementation was found in changed code.

## Examined scope

```yaml
scope:
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2`
    whose `operation.member.clause` is `precondition`, `postcondition` or `invariant`
    yields one harness of that obligation kind, separate from the harness of every
    other clause of the package; the kind is read from the node and no caller value
    changes it. For a collection-valued operation clause, the one operation-clause
    harness carries a generated `CgObjectState` with every admitted model field as
    an exact `Value`, the selected one being `Value::Collection`; the customer subject
    receives one mutable exact-va
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input:
    the pre-state of each state field of `self` the body reads, whether bare or under
    `pre(...)` (the subject''s input state; how it reaches the subject is FR-025''s),
    and each operation parameter (for a precondition or invariant, each bare state
    field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected
    model declaration state field (FR-015-AC-77), and the IR `bounded_domain` of a
    parameter''s type. A non-model or admitted unsel'
- id: FR-015-AC-43
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing
    its anchor node as a `requires` and records each in its identity's embedded oracles,
    embeds no other obligation's oracle, and a postcondition whose sibling precondition
    is not a supported item of the same request is refused naming that precondition
    with no harness; a V2 invariant is anchored at an object type, binds `self` alone,
    and so has no operation precondition to assume. For a collection postcondition,
    each supported sibling precondition evaluates on the same complete typed prestate
    and operation parameters that re
- id: FR-015-AC-96
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An admitted QSL V2 invariant, precondition or postcondition clause, or
    function body, that contains a `quire.op.collection.forall` or `quire.op.collection.exists`
    operation over a selected model collection field is a collection claim. A constraint
    clause follows FR-015-AC-128; it is not a standalone proof obligation. An invariant
    or function-body collection claim uses the direct collection harness. A precondition
    or postcondition collection claim uses its FR-015-AC-38/39/43 operation-clause
    harness, with the same selected symbolic field domain and no second standalone
    collection proof. For a f
- id: FR-015-AC-99
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For a function-body or invariant collection claim, the direct harness
    asserts the admitted body over symbolic inputs drawn from the selected model field''s
    typed domain and ends with the FR-015-AC-7 cover after its assertions. A precondition
    collection claim is marked as a collection quantifier in its typed V2 clause obligation
    identity and uses the same symbolic field draw in its FR-015-AC-38/39 clause harness:
    an assertion checks that evaluation completes a Boolean on every draw (both true
    and false are allowed), and a separate cover checks that some draw completes true.
    It neither assumes th'
- id: FR-015-AC-101
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request whose symbolic collection domain exceeds the generator's representational
    or resource ceiling settles `unsupported` with a typed reason naming the failing
    domain or ceiling and no harness; it never tightens the model domain to make a
    proof tractable. A missing selected model field is refused at QSL binding with
    its typed source-located reason, and a malformed selected projection is refused
    by Contract IR strict admission with its typed located cause; neither produces
    an admitted collection claim, CG item or harness. For an admitted claim whose
    checked selected declaration or field ca
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fresh quoin matrix reports AC-38/39/43 tagged. Declaration-order/context, produced-domain, and same-anchor assumption/refusal tests bind the implemented obligations; tags are backed by real execution/assertions. |

## Final disposition verdict

PASS for the scoped PR findings at 565ce888. Original findings remain unchanged. Focused ordinary controls (13/0), all-target Clippy and fmt logs were inspected. Parent owns the full integration baseline comparison and merge gate. No installed-backend or native-replay proof credit is inferred. The gap analysis remains no repository-wide completeness assertion.

## Completion delta audit

At 1383564b, the added identity control authenticates the typed exported conjunction/scalar API and distinguishes nonquantifier and sibling-literal meanings. Existing operation-fixture/source/playback expectations now use the whole-state ABI. AC-38/39/43 trace bindings remain in place. No additional scoped gap finding; parent owns final-head execution gates.
