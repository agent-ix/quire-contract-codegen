---
id: SR-15272
title: IR-17 tuple stage integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; IR-17; spec/oracle/functional/FR-018-composite-equality-oracles.md;
  spec/oracle/matrix/TC-029-composite-equality-oracles.md
review_set: subset
---

## Summary

Independent reviewer-only PR499, exact head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Source-stage contract review; no product qualification.

## Verdict

PASS: FR018 satisfies StR001 and depends on FR014; TC029 verifies FR018. Exact public bounded-positive, private two-form/negative and external-QSpec shape obligations have separate prerequisites. No contradiction with QSL bounded emission or IR node preservation remains. No change to Integer+sibling semantics, AC8/26, cycle or no-charge behavior.

## Examined scope

scope:
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range`
    and `text_bounds` nodes reconstruct their exact own bounds, text profile, position
    order and decimal rounding; in particular the numeric controls retain Int[-100,100],
    Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the text control
    retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming
    either reconstructs that node's own `min` and `max`, never the sibling's. The
    legacy private resolver control retains exact generated-source equivalence between
    numeric bo
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation
    time carrying that `DeclarationCause`, including both recursion passes. The authentic
    duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration
    and cause() equal to None for the Assembly refusal, produces no successful AdmittedContext
    and is not passed to public CG generation. Private CG declaration-admission defenses
    retain the exact DuplicateMember name/cause and unchanged healthy sibling separately,
    without '
- id: FR-018-AC-14
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Admitted `bounded_domain` nodes an operand type reaches retain the exact
    named bound values FR-014 specifies. For `integer_range` and `collection_bounds`,
    Contract IR FR-038 admission requires exactly `min` then `max` bindings with their
    canonical typed integer literals; a reordered, bare, missing, duplicate or unlisted
    binding control is an IR-intake refusal before CG, with no admitted package or
    public CG `UnreadableBound` result. CG private parser tests retain their precise
    malformed-bound defenses separately and do not count as public-generator acceptance
    for these reader-ineligi
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional,
    reference Option<List>)]))` yields exactly one `FieldDeclaration` named `next`,
    with `Presence::Optional` and payload type `ValueType::Composite(List NodeKey)`
    on the genuine public admitted List path. At the separately labelled private parser/type-resolution
    seam, removing or renaming `optional`, adding a second nested member, or targeting
    a non-option node retains exact MalformedComposite, no symbol, no required-field
    reinterpretation and an unchanged healthy Tree sibling. These inconsistent internal
- id: FR-018-AC-27
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec Tree `kids` field's direct reference to `collection_bounds`
    follows that node's `semantic_type` to `Sequence<Tree>`, reads its sole element
    reference as the Tree `NodeKey`, and reconstructs cardinality `[0,3]` from that
    bound node's named `min` and `max`; changing the semantic type to a non-sequence,
    dropping or duplicating either bound, or making either value non-integer refuses
    only that item with a typed cause and emits no code, never silently yielding an
    unbounded collection. PLANNED (IR-643).
- id: TC-029-step-8
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "8. Through unchanged genuine source, checked package and admitted context,\n\
    \   generate an equality over a tuple whose bounded member types name their\n\
    \   `bounded_domain` nodes (FR-018-AC-16). Preserve position order and exact\n\
    \   Int[-100,100], Text[0,16;nfc], Decimal[-100,100;0,0;nearest-even] and\n  \
    \ Rational[-10,10;1,1] reconstruction. With a second `text_bounds` over the\n\
    \   same text scalar, independently select either genuine text member and\n  \
    \ assert only its own bounds are reconstructed, never the sibling's.\n\n   At\
    \ the explicitly private resolver seam, retain both numeric member"
- id: TC-029-step-16
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "16. Recursive records and authoritative shape reads (FR-018-AC-24, AC-26\
    \ and AC-27, planned,\n    IR-643). Read the authoritative QSpec\n    `proposals/checked-package-v2/fixtures/positive-recursive-records.json`\
    \ through Contract IR's\n    strict reader, without copying it into this repository.\
    \ Preserve exact\n    admission and inspect the original List `next` binding:\
    \ its value is an\n    aggregate with one `optional` binding to the Option node,\
    \ whose sole\n    payload reference returns to List. Inspect Tree `kids`: its\
    \ direct\n    reference targets `collection_bounds`, whose `semantic_typ"

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence and coverage

Selected QSL owner source was independently examined, rather than trusting the author report: FR-092:302-328 requires bounded-domain references; qsl-semantics/src/check/type_form.rs:340-363 represents the bounded numeric families; assemble.rs:2958-2968 resolves an alias to the same ValueType; lowering.rs:1957-2040 emits the numeric bounded nodes and :2201-2230 descends into actual tuple positions. Contract IR crates/quire-contract-model/src/checked_package/v2/lower.rs:524-590 follows exact successors and :664-670 clones each actual node and semantic_type. A profile/root selection does not rewrite member encoding. QSL qsl-replay/src/admitted.rs:198-202 binds context to package identity and :407-445 builds it only from recompiled admitted source. CG's draft src/oracle/equality/mod.rs:735-746 binds reconstruction to the source package identity. These facts support this bounded three-family stage split; they do not establish that every conceivable public alternate source is impossible.

The candidate keeps exact public Int[-100,100], Text[0,16;nfc], Decimal[-100,100;0,0;nearest-even], Rational[-10,10;1,1], tuple order, each selected text bound and no sibling substitution. The old two-form generated-source equivalence remains mandatory at a labelled private seam; the currently drafted test merely checking reconstruction substrings is not qualified as full equivalence. Exact MissingBound(integer_range)/bound identity, unsupported bounded_domain/bound identity, no code and independent siblings remain mandatory. Float/operator-ineligible reader controls keep their actual cause/path/locus. No public malformed nominal context, key minting or unrelated sibling ownership is licensed. The unbounded Integer/sibling-range ambiguity is explicitly unchanged.

The exact external QSpec recursive fixture retains its own strict-reader admission and original List optional wrapper/Tree [0,3] shape evidence. The independent source positive must compile_admitted, lower its unchanged package, generate both genuine declarations and preserve its own keys/context; it is explicitly not the external fixture extended with equalities. Private fixture-shaped reconstruction retains field presence, keys, cardinality and exact defense assertions. An unavailable QSPEC_REPO check grants no evidence. Synthetic Option/Sequence cycles retain repeated-node TypeResolutionCycle, subprocess protection, no symbol and healthy sibling. AC8/26 from PR496 are unchanged.

Quire validation and git diff --check pass. No applicable enforced AssuranceProfile is installed in the scoped spec. No Cargo, formatter, metadata, native test, backend or proof was run; Rust gates are inapplicable to this two-Markdown-file/no-embedded-consumer diff and root holds the sole heavy workload reservation. EARS was not selected: the FR Description requirement statement is unchanged. Dependency analysis was not selected: no relationship edge changes; the new body citation resolves to the actual QSL FR092 owner. Object/risk analyses were not selected: no object schema, algorithm or resource budget changes. Calibrated criterion-strength/Jev analysis was not selected or claimed. No optional repository-wide semantic review was requested or performed.

## Disposition round 1 verification

Same frozen head ebb26afd36236bda990224c48a2b69fe4ce80a00; base 6b1ba8b4e4906c2f2e5e2b398fd59e7f2fb6c7f8. Actual root scope decision 68f36578-9a30-4687-a4f7-4780518c184f independently read back: partial specification delivery only. No source or test change, no new introduced spec defect, no product qualification, no assertion weakening or changed member-association semantics. All six non-gap methods retain PASS. Full gap retains FAIL; all316 findings remain open under their exact existing owners. AC56 inspection remains pending IR-877 and IR-643 recursion ownership is unchanged. No heavy job ran.
