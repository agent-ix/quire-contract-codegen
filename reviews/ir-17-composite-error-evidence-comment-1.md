<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15284 method=spec-review/evidence lang=spec pr=quire-contract-codegen#500 model=gpt-6.1-sol run=7dd2e3ef-32c3-41f7-a542-27bc1be8d278 -->

Exact head 508cab924b870851e85970b60e851649cb9a00a7; base cead553e98aaec3e0bf2bdab80ab69c3c3ca775f. Part 1/1. Verdict PASS.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |

+++ [reviewer data]

```yaml
clean: true
scope:
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The `src/lib.rs` that `generate_composite_equality_oracles` returns for the TC-029 step 1 package (every
    generated item) contains zero occurrences of the panic tokens FR-018-AC-19 lists, no `.ok()`, `.unwrap_or(`
    or `.unwrap_or_default(` anywhere, and no `[` index or slice expression directly after an identifier character,
    `)` or `]`; every reconstruction helper in it returns `Result<_, ReconstructionError>`, and each of `rebuild_integer`,
    `rebuild_interval`, `rebuild_rational`, `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` turns a
    failed constructor or parse into exact
- id: FR-018-Outputs
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- Generated `EnvironmentError` SHALL distinguish\n  `MissingAdmittedContext { declaration: qsl_replay::WireNodeId\
    \ }`,\n  `ContextPackageMismatch`,\n  `Lookup(qsl_replay::AdmittedLookupRefusal)`, `Declaration` retaining the\n\
    \  QSV `EnvironmentFailure`, and `Reconstruction(ReconstructionError)`.\n  QSV admission's semantic refusal\
    \ and resource failure remain distinguishable\n  through their original typed causes. `ReconstructionError`\
    \ keeps the\n  payload-free numeric/text/cardinality variants `Integer`, `Interval`,\n  `Rational`, `Decimal`,\
    \ `Text`, `Cardinality`, `Enum` and `Composite`.\n  `En"
- id: TC-029-step-9
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "9. Emitted-source scan (FR-018-AC-17). Read the `src/lib.rs` that\n   `generate_composite_equality_oracles`\
    \ returns for the step 1 package and assert\n   zero occurrences of the panic tokens of step 11, no `.ok()`,\
    \ `.unwrap_or(` or\n   `.unwrap_or_default(`, and no `[` index or slice expression directly after an\n   identifier\
    \ character, `)` or `]`. Assert every reconstruction helper returns\n   `Result<_, ReconstructionError>`, every\
    \ call of one is followed by `?` or a\n   `match` whose `Err` arm yields `EnvironmentError::Reconstruction`\
    \ or\n   `Outcome::Refused(Refusal::CheckedInvariant)`"
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without '
- id: FR-018-AC-16
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Genuine public tuple members naming `integer_range`, `decimal_range`, `rational_range` and `text_bounds`
    nodes reconstruct their exact own bounds, text profile, position order and decimal rounding; in particular the
    numeric controls retain Int[-100,100], Decimal[-100,100;0,0;nearest-even] and Rational[-10,10;1,1], and the
    text control retains Text[0,16;nfc]. With two `text_bounds` nodes over one text scalar, naming either reconstructs
    that node's own `min` and `max`, never the sibling's. The legacy private resolver control retains exact generated-source
    equivalence between numeric bo
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader and List/Tree shape assertions;
    independently authored genuine admitted source with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded
    `[0,3]`) shapes closes each back-edge at its own in-progress record key, generates equality items over both
    record types, and terminates; a synthetic unclosed `Option` self-cycle and a `Sequence` self-cycle each refuse
    as `TypeResolutionCycle` naming the repeated type node, without a panic, stack overflow or generated symbol.
    PLANNED (IR-643).
- id: FR-018-AC-26
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: Reading the QSpec List `next` field as `binding(next, aggregate([binding(optional, reference Option<List>)]))`
    yields exactly one `FieldDeclaration` named `next`, with `Presence::Optional` and payload type `ValueType::Composite(List
    NodeKey)` on the genuine public admitted List path. At the separately labelled private parser/type-resolution
    seam, removing or renaming `optional`, adding a second nested member, or targeting a non-option node retains
    exact MalformedComposite, no symbol, no required-field reinterpretation and an unchanged healthy Tree sibling.
    These inconsistent internal
- id: FR-018-AC-19
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The non-test code of `src/oracle/equality/mod.rs` (comments and every `#[cfg(test)]` item removed wherever
    the item sits, string literals the generator emits counted) contains zero panic tokens: the identifiers `unwrap`,
    `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind`
    however written (called, with whitespace before the paren, named on a path such as `Option::unwrap`, or imported);
    the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`,
    `debug_assert_eq` and `debug'
findings: []
bindings: []
```

+++
