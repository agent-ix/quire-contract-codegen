<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15275 method=spec-review/scope-boundary lang=spec pr=quire-contract-codegen#499 model=gpt-6.1-sol run=48b15e0e-90a9-4152-af98-92055cee80a1 -->

Exact head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Part 1/1. Verdict PASS.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |

+++ [reviewer data]

```yaml
clean: true
scope:
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range` and `text_bounds`
    nodes reconstruct their exact own bounds, text profile, position order and decimal rounding; in particular the
    numeric controls retain Int[-100,100], Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the
    text control retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming either reconstructs
    that node's own `min` and `max`, never the sibling's. The legacy private resolver control retains exact generated-source
    equivalence between numeric bo
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader and List/Tree shape assertions;
    independently authored genuine admitted source with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded
    `[0,3]`) shapes closes each back-edge at its own in-progress record key, generates equality items over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse
    as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol.
    PLANNED (IR-643).
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without '
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact named bound values FR-014 specifies.
    For `integer_range` and `collection_bounds`, Contract IR FR-038 admission requires exactly `min` then `max`
    bindings with their canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted binding
    control is an IR-intake refusal before CG, with no admitted package or public CG `UnreadableBound` result. CG
    private parser tests retain their precise malformed-bound defenses separately and do not count as public-generator
    acceptance for these reader-ineligi
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional, reference Option<List>)]))`
    yields exactly one `FieldDeclaration` named `next`, with `Presence::Optional` and payload type `ValueType::Composite(List
    NodeKey)` on the genuine public admitted List path. At the separately labelled private parser/type-resolution
    seam, removing or renaming `optional`, adding a second nested member, or targeting a non-option node retains
    exact MalformedComposite, no symbol, no required-field reinterpretation and an unchanged healthy Tree sibling.
    These inconsistent internal
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds` follows that node's `semantic_type`
    to `Sequence<Tree>`, reads its sole element reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]`
    from that bound node's named `min` and `max`; changing the semantic type to a non-sequence, dropping or duplicating
    either bound, or making either value non-integer refuses only that item with a typed cause and emits no code,
    never silently yielding an unbounded collection. PLANNED (IR-643).
- id: TC-029-step-8
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "8. Through unchanged genuine source, checked package and admitted context,\n   generate an equality\
    \ over a tuple whose bounded member types name their\n   `bounded_domain` nodes (FR-018-AC-16). Preserve position\
    \ order and exact\n   Int[-100,100], Text[0,16;nfc], Decimal[-100,100;0,0;nearest-even] and\n   Rational[-10,10;1,1]\
    \ reconstruction. With a second `text_bounds` over the\n   same text scalar, independently select either genuine\
    \ text member and\n   assert only its own bounds are reconstructed, never the sibling's.\n\n   At the explicitly\
    \ private resolver seam, retain both numeric member"
- id: TC-029-step-16
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "16. Recursive records and authoritative shape reads (FR-018-AC-24, AC-26 and AC-27, planned,\n    IR-643).\
    \ Read the authoritative QSpec\n    `proposals/checked-package-v2/fixtures/positive-recursive-records.json`\
    \ through Contract IR's\n    strict reader, without copying it into this repository. Preserve exact\n    admission\
    \ and inspect the original List `next` binding: its value is an\n    aggregate with one `optional` binding to\
    \ the Option node, whose sole\n    payload reference returns to List. Inspect Tree `kids`: its direct\n    reference\
    \ targets `collection_bounds`, whose `semantic_typ"
findings: []
bindings: []
```

+++