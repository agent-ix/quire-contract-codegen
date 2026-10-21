---
id: SR-15280
title: IR-17 composite reconstruction error code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-17; spec/oracle/functional/FR-018-composite-equality-oracles.md;
  spec/oracle/matrix/TC-029-composite-equality-oracles.md
review_set: subset
---

## Summary

Independent reviewer-only PR500, exact head 508cab924b870851e85970b60e851649cb9a00a7; base cead553e98aaec3e0bf2bdab80ab69c3c3ca775f. Source-only contract review. No product qualification.

## Verdict

PASS: the spec-only diff repairs an omitted generated refusal and stale TC description without copied code, test bypass, digest ledger, API widening or key construction. No Rust or manifest change is in the candidate.

## Examined scope

scope:
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for
    the TC-029 step 1 package (every generated item) contains zero occurrences of
    the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(` or `.unwrap_or_default(`
    anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`,
    and each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`,
    `rebuild_text` and `rebuild_cardinality` turns a failed constructor or parse into
    exact
- id: FR-018-Outputs
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- Generated `EnvironmentError` SHALL distinguish\n  `MissingAdmittedContext\
    \ { declaration: qsl_replay::WireNodeId }`,\n  `ContextPackageMismatch`,\n  `Lookup(qsl_replay::AdmittedLookupRefusal)`,\
    \ `Declaration` retaining the\n  QSV `EnvironmentFailure`, and `Reconstruction(ReconstructionError)`.\n\
    \  QSV admission's semantic refusal and resource failure remain distinguishable\n\
    \  through their original typed causes. `ReconstructionError` keeps the\n  payload-free\
    \ numeric/text/cardinality variants `Integer`, `Interval`,\n  `Rational`, `Decimal`,\
    \ `Text`, `Cardinality`, `Enum` and `Composite`.\n  `En"
- id: TC-029-step-9
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "9. Emitted-source scan (FR-018-AC-17). Read the `src/lib.rs` that\n  \
    \ `generate_composite_equality_oracles` returns for the step 1 package and assert\n\
    \   zero occurrences of the panic tokens of step 11, no `.ok()`, `.unwrap_or(`\
    \ or\n   `.unwrap_or_default(`, and no `[` index or slice expression directly\
    \ after an\n   identifier character, `)` or `]`. Assert every reconstruction helper\
    \ returns\n   `Result<_, ReconstructionError>`, every call of one is followed\
    \ by `?` or a\n   `match` whose `Err` arm yields `EnvironmentError::Reconstruction`\
    \ or\n   `Outcome::Refused(Refusal::CheckedInvariant)`"
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
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
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
  role: context_only
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
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
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every
    `#[cfg(test)]` item removed wherever the item sits, string literals the generator
    emits counted) contains zero panic tokens: the identifiers `unwrap`, `expect`,
    `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any`
    and `resume_unwind` however written (called, with whitespace before the paren,
    named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`,
    `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq`
    and `debug'

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence and coverage

The exact two-document diff was read against the published base and the separate IR17 product draft. Independently examined source: ir17-qsl937/src/oracle/equality/mod.rs:1470 defines the generated eighth payload-free Composite variant; :1484 lookup_composite_key first parses the retained WireNodeId with Composite failure, requires context with MissingAdmittedContext, calls the real owner composite_declaration and maps the original refusal through EnvironmentError::Lookup, checks form and the complete qualified-name iterator before returning admitted.key(). Only actual owner lookup supplies the key. No NodeKey construction, fallback, key substitution, type widening, or context authentication from a printed identity occurs. The enclosing generated prepare path retains package mismatch checks and typed QSV EnvironmentFailure. The standalone admitted.rs:160-194 resolver continues its separate CompositeContextMismatch; this amendment does not rename or collapse that product refusal.

The selected QSL10f owner source, read directly through git show, defines AdmittedComposite.node separately from its kernel key and exposes form/name. qsl-replay/src/admitted.rs:257-289 composite_declaration returns an owning local composite first, then exact Foreign with its package, then WrongKind for another local node, then Unknown with case=None. Unknown/Foreign/WrongKind are not malformed-recipe reconstruction failures. The generated Composite failure follows successful genuine lookup for form/name mismatch only, or failure to parse its retained literal before lookup. Enum reconstruction remains distinct. Source inspection confirms the proposed contract is implementable; it is not native execution, public mutation admission, or proof qualification.

The TC step corrects its stale six-variant/InvalidDeclaration-only description to reference the current owning FR Outputs, including EnvironmentFailure and the Enum/Composite variants. Existing six numeric/text/cardinality helpers retain Result signatures, one-map_err exact causes, prohibition on recovery and propagation/refusal controls. AC17's new Test method is the prior authored class with its TC id removed from the method cell. Independently executed quoin advise reports AC17 authored Test, mismatch=false, inconclusive=false; property invariant yields property-based/design-by-contract/compile recommendations alongside workflow/reliability/security recommendations. Judgment: structural adverse controls can discriminate the emitted lookup/refusal mapping while real owner-context execution remains separately required by the unchanged admitted-context contract. The catalog does not itself establish successful execution.

No applicable AssuranceProfile is installed in the reviewed spec. No Cargo, formatter, metadata, native test, backend or proof command ran. Rust execution lane is inapplicable to this Markdown-only diff without embedded consumers. EARS was not selected because the FR Description statement is unchanged. Dependency analysis was not selected because no relationship edge changes. Object analysis has no dedicated installed SpecReview analysis value and was not claimed as a separate completed method; the existing local error envelope is checked here for consistent typed variants, rather than creating a second cross-context domain object. Risk/architecture analyses were not selected: no algorithm, public bounded API, budget or ownership boundary changes. Jev calibrated criterion-strength and optional whole-repository semantic review were not requested or performed. Quoin authoring pack and installed SpecReview schema were read; the initially omitted required positional repository argument to quoin write was corrected with quoin write . --types SpecReview --json, retained in template.json.

## Disposition round 1 verification

Same frozen head508cab924b870851e85970b60e851649cb9a00a7; basecead553e98aaec3e0bf2bdab80ab69c3c3ca775f. Actual root decisiondbdb9928-c07f-4d9f-86b4-5bb88cb1804b independently read back: partial specification delivery only. No source/test change, introduced spec defect, execution, product qualification or assertion weakening. Six non-gap methods retain PASS. Full gap remains FAIL; all317 findings stay open under explicit existing owners (311 program,2IR877,4IR17). AC56 remains pendingIR877 inspection and IR643 recursion ownership is unchanged. No heavy job ran.
