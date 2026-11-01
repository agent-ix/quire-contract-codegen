---
id: SR-202610310132
title: IR-1091 PR 694 code-review review
type: SpecReview
analysis: code-review
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
| FND-001 | high | Postcondition nonvacuity cover requires a cardinality endpoint after sibling assumptions. A supported OrderedSet<Boolean>[0,2] with forall(item != false) and exists(item != false) siblings admits only length 1. A correct subject passes every assertion but can never reach this cover. Cover any assumption-satisfying draw after assertions; keep endpoint completeness in domain controls. | src/kani/generate/render/collection_operation.rs:297-298 | implementation-bug-despite-evidence |
| FND-002 | medium | MissingSiblingPrecondition and UnsupportedSiblingPrecondition carry no sibling node identity; the outer CollectionClaimNotRendered node_id names the postcondition. With multiple siblings the caller cannot identify the missing or unsupported precondition required by AC-43. Preserve its CheckedNodeId in each typed reason and assert it in refusal controls. | src/kani/generate/collection.rs:552-555 | implementation-bug-despite-evidence |

## Coverage

Examined canonical fields() declaration order and declaring_node_id, exact Value state and unread fields, schema-aware builders/inspectors, all produced-domain checks, typed parameter/result ABI, sibling assumptions/refusals, retained prestate and authored conjunction/equality evaluation, symbolic uniqueness/domain/resource precedence, playback ordering, identity material, and compiled real-exact test driver. No applicable AssuranceProfile exists under spec/. No code or repository edits were made. No cargo gate was run by reviewer while shared baseline lock was held; parent owns final-head checks. git diff --check passed. Installed-backend/native replay proof credit was not claimed. Existing proof-equivalence lane #654 remains separate.

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
| FND-001 | fixed | At 565ce888, operation HarnessSpec cover is Completed(true) and emitted after all assertions. The real generated-source regression restricts OrderedSet<Boolean>[0,2] to length 1 and checks the actual cover Boolean; focused log has 13 passes. |
| FND-002 | fixed | Both refusal variants now carry the exact CheckedNodeId; missing-request and unsupported-body branches retain the sibling ID, with discriminating identity assertions. |

## Final disposition verdict

PASS for the scoped PR findings at 565ce888. Original findings remain unchanged. Focused ordinary controls (13/0), all-target Clippy and fmt logs were inspected. Parent owns the full integration baseline comparison and merge gate. No installed-backend or native-replay proof credit is inferred. The gap analysis remains no repository-wide completeness assertion.

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-003 | low | CollectionOperationConditionIdentity is a public field type of CollectionOperationIdentity but is not re-exported through the crate root; its defining kani and identity modules are private. External callers cannot name or match the typed condition. Export it alongside CollectionOperationIdentity. | src/lib.rs:247; src/kani/identity.rs:297 | implementation-bug-despite-evidence |

## Completion delta audit

Inspected 565ce888..ea1f91a9. The one typed condition representation is shared by lowering, renderer, serialized operation identity and symbol material; nonquantifier literals and sibling bodies therefore change both context and module identity. The authentic QSL regression compares distinct scalar literals in primary/sibling conditions. The old operation fixture now declares only its supported finite items/flags state; whole-state playback decodes both fields, and conjunct/parameter support assertions reflect the new ABI. Existing unsupported controls remain. Installed-backend test staging gains its real exact dependency, but no backend/native-replay execution credit is claimed. Earlier FND-001/002 fixes remain intact. New final-head gates are owned by the parent and were pending during this audit.

## Dispositions (pass 2)

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | At 1383564b the crate root re-exports CollectionOperationConditionIdentity alongside CollectionOperationIdentity; the external integration test now pattern-matches exported And and Scalar variants, so the public field vocabulary is accessible. |

## Completion delta verdict

PASS for the scoped fix audit at 1383564b; no open review findings. No new cargo gate was run by the reviewer. Parent owns pending final-head gate evidence.

## Final execution-fix audit

Inspected d4fe78fc and c928f303 after 1383564b. Quantifier discovery now skips non-reference Boolean operands of conjunctions; condition_operand retains inline scalar operands through the existing checked scalar lowering. Canonical Boolean values are read with as_bool rather than as_str, with no fallback or duplicate parser. The authentic QSL test compiles and executes both true-before-forall and forall-before-true authored bodies. Shared budget exhaustion remains a refusal; complete conditions remain in the single identity/render AST. No new defect found; all earlier findings remain fixed. Observed /tmp/ir1091-all-collection-final.log: 37 passed, 0 failed, 3 explicitly ignored backend-lane tests in 2.81s. This adds ordinary execution evidence only; full-head integration/Clippy/unit gates remain parent-owned.

PASS for the scoped review at c928f303.

## Cover-audit integration completion

Inspected 8dd5756e after the clean rebase onto f04c8a83. The global cover audit now explicitly drives both whole-object precondition and postcondition via the real public negotiate_kani_obligations path. guard_sources requires exactly two emitted clause harnesses and derives family labels from their typed obligation kinds; the audit compares the complete emitted family set to FAMILIES. Existing syn inspection still requires exactly one proof function, exactly one cover in its complete body, and that cover as the final statement. The one new renderer template is counted exactly; spelling exclusions name only the exact shared renderer child file. No broader prefix exemption, skipped assertion, or weakened checker was introduced. Existing missing/duplicated/misplaced-cover mutant controls remain. No new finding; all earlier findings remain fixed. Final focused/full execution evidence is parent-owned.

PASS for the scoped review at 8dd5756e.
