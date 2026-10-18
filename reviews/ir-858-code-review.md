---
id: SR-12860
title: IR-858 step 4b code-review
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; IR-858 PR #421 own specification delta in
  AD-004, FR-015, TC-025 and matrix index'
review_set: subset
---

## Summary

Ticket IR-858; independently examined the frozen #421 own delta against its prepared #417 parent. No .rs/Cargo files change: no Rust lane or execution gate. No production change: no whole-repository gap audit. Base, integrity, evidence, scope-boundary and criterion-strength retain original SR-12180..12184; no new relationship edge/domain object/security surface justifies additional methods. No AssuranceProfile exists under spec/. Source and tests were inspected as context only. No runtime completion or IR-348 closure is inferred. The existing nested Pat::Macro code-phase observation remains implementation guidance, not a new spec defect.

## Verdict

PASS — no additional defects in this spec-only delta. The typed constructor/renderer split matches current scalar source and refusal order. Planned AC112–117 describe future implementation, with differential comparison distinct from real-backend smoke. Existing semantic review findings remain terminal; no claim is made that the planned code exists.

## Scope

Every changed paragraph/table row examined by this method is recorded below with its exact text excerpt (up to 600 characters). Context-only entries are not assertions of implementation completion.

```yaml
scope:
- id: spec/assurance/AD-004-cg-crate-layout.md:L206
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "      spec.rs                 HarnessSpec and its one constructor (step 4b; shape decided\
    \ in \"The\n                              HarnessSpec and render contract\"; not created by 2f)"
- id: spec/assurance/AD-004-cg-crate-layout.md:L360
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '- Inside `kani/generate/` the order is `outcome`, `record`, `spec`, `render`, `census_validation`,
    then the families'
- id: spec/assurance/AD-004-cg-crate-layout.md:L368
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '  five prefixes for `collection.rs`); `scalar` and `collection` share a rank, so neither imports
    the other. `spec` imports `outcome` (for `MAX_OBLIGATION_UNWIND`), `core::identity` (for `HarnessPath`)
    and `kani::abi`; `render` imports `spec`, `outcome`, `core::artifact` and `kani::abi`, and not `record`;
    a family that has been ported (scalar at 4b) imports `spec` and `render`. A family file may import
    `outcome` and `record` and never `negotiate`: the passes that read the whole request'
- id: spec/assurance/AD-004-cg-crate-layout.md:L403
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- `kani/generate/spec.rs` defines `HarnessSpec`: the proof attribute, the validated `HarnessPath`\n\
    \  (defined in `core/identity.rs`), the symbolic inputs with their domains, the assumptions, the subject\n\
    \  call, the assertions and the cover. Its fields, invariants and constructor are decided in \"The\n\
    \  HarnessSpec and render contract (step 4b)\" below."
- id: spec/assurance/AD-004-cg-crate-layout.md:L412
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "  reached; for the precondition family, that the precondition holds). The constructor refuses\
    \ a\n  spec without exactly one cover (decision H-3), and the order is fixed by the renderer, not\
    \ by the family. A frame harness's"
- id: spec/assurance/AD-004-cg-crate-layout.md:L421
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '  lowered IR claims and return `HarnessSpec`s. They format no harness structure (statements,
    attributes, macros, layout, cover placement); the one text a lowerer supplies is an expression fragment
    (`ExprText`, decision H-2). Until its step'
- id: spec/assurance/AD-004-cg-crate-layout.md:L431
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '### The HarnessSpec and render contract (step 4b)'
- id: spec/assurance/AD-004-cg-crate-layout.md:L433
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'This subsection decides the shape SR-785 left open (its FND-005 held the `HarnessSpec` design
    for

    this step) and states what step 4b changes. It decides only what the scalar family needs. Every

    member another family needs (a `proof_for_contract` target, contract attributes, a frame''s

    `requires`/`ensures`, a collection''s typed domain) is added by the step that ports that family, as

    an edit to this subsection; 4b adds no variant, field or helper for a family it does not port.

    Steps 4c to 4h are not specified here.'
- id: spec/assurance/AD-004-cg-crate-layout.md:L440
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision H-1. The shape of `HarnessSpec` (`kani/generate/spec.rs`).** A `HarnessSpec` is
    a typed

    description of one harness, with no Kani spelling in it. Its fields:'
- id: spec/assurance/AD-004-cg-crate-layout.md:L443
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| Field | Type and meaning |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L444
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| --- | --- |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L445
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `label` | The text of the `// Obligation: <label>` header line. The scalar lowerer supplies
    ``exact-scalar `<operation identity>` ``. `render` writes the fixed header: the line `// SPDX-License-Identifier:
    MIT OR Apache-2.0`, the Obligation line, a blank line, then `oracle_source`, then one newline, then
    the module wrapper (the bytes `render_scalar` writes today, `scalar.rs` lines 463 to 470). |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L446
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `path` | The validated `HarnessPath` (module and harness symbols), defined in `core/identity.rs`
    and built by `harness_path` in `record.rs`; neither moves. It is the only source of the `mod` and
    `fn` names and of the artifact path `src/generated/<module>.rs`. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L447
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `oracle_source` | The generated FR-014 oracle source, placed verbatim after the header and
    before the harness module. It is opaque to `render.rs`. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L448
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `proof` | `ProofKind`. Step 4b defines one variant, `Proof` (`#[kani::proof]`). Step 4c
    adds `ProofForContract` with its own spec change. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L449
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `body` | An ordered list of `Statement`: `Input`, `Assume`, `Let`, `Call` and `Assert`.
    See below. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L450
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `cover` | Exactly one `Cover`: a condition (`ExprText`) and a message. See H-3. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L451
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `unwind`, `solver` | The loop bound (`1..=MAX_OBLIGATION_UNWIND`) and the `KaniSolver`.
    They are not emitted into the source; they are the run options. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L452
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '| `options` | The adapter option vector (`adapter_options(path, unwind, solver, false)`),
    computed once by the constructor from `path`, `unwind` and `solver`. The family''s identity record
    reads `unwind`, `solver` and `options` from the spec, never from the request a second time, so the
    run options and the identity cannot disagree. |'
- id: spec/assurance/AD-004-cg-crate-layout.md:L454
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '`Statement` variants (all fields typed, none is a line of Rust):'
- id: spec/assurance/AD-004-cg-crate-layout.md:L456
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- `Input { name, ty, domain, then }`: a symbolic input. `ty` is a `KaniPrimitiveType` from\n\
    \  `kani/abi.rs` (4b uses `I64`). `domain` is an inclusive `[minimum, maximum]` pair of the same\n\
    \  type. Render emits `let <name>: <ty> = kani::any();` and `kani::assume(<name> >= <minimum> &&\n\
    \  <name> <= <maximum>);`. `then` is the ordered list of `Let`s that must follow this input\n  immediately\
    \ (the scalar harness copies the input to `<name>_native` and shadows it with its\n  `rt::Integer`);\
    \ it keeps today's per-operand grouping, which byte identity needs.\n- `Assume { condition }`: an\
    \ extra `kani::assume"
- id: spec/assurance/AD-004-cg-crate-layout.md:L481
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision H-2. `ExprText` is the one text a lowerer supplies, and it is an expression only.**

    `ExprText` is one Rust expression as text, for the native `i128` expression, the outcome `match`

    and the `matches!` of the cover: the family''s own meaning, not harness structure. It is a

    validated newtype whose constructor refuses text that:'
- id: spec/assurance/AD-004-cg-crate-layout.md:L486
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- does not parse as a `syn::Expr`;\n- spells `kani::` or `#[`;\n- contains a block, `unsafe`,\
    \ `async`, closure, loop (`loop`, `while`, `for`) or `let` expression\n  anywhere in its tree (a `match`\
    \ is allowed: the scalar outcome check is one); or\n- contains a macro invocation other than `matches!`,\
    \ or a `matches!` that has a guard\n  (`if ...`). The arguments of `matches!` are parsed as an expression\
    \ and a pattern, and the\n  checks above apply to the expression, so a macro, block or closure nested\
    \ in it is refused too."
- id: spec/assurance/AD-004-cg-crate-layout.md:L494
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'So an `ExprText` cannot hold a statement, an assertion, a panic or a cover, and R-1 item 2''s

    promise that nothing a lowerer supplies changes where the cover goes is enforced rather than

    hoped for. Adding a macro to the allowlist is a spec change here. An `ExprText` may span lines;

    its lines after the first carry their indentation relative to the statement, and `render.rs` adds

    the statement''s own indent to each, which is what lets the port be byte-identical. A typed

    expression tree was considered and rejected for 4b: it would be a second Rust AST inside the crate

    for four arithmetic expression'
- id: spec/assurance/AD-004-cg-crate-layout.md:L504
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'A `message` (of an `Assert` or the `Cover`) is a string with no `"`, `\`, `{`, `}` or line

    break; the constructor refuses one that has any, as a typed error, rather than escaping it, because

    the scalar assertion message interpolates the operation identity and an escape rule would change

    bytes. `label` follows the same rule except that it may hold backticks and may not hold a line

    break.'
- id: spec/assurance/AD-004-cg-crate-layout.md:L510
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision H-3. Invariants, enforced by `HarnessSpec::new`, the only constructor.** A refusal
    is a

    typed `HarnessSpecError`; none is a panic.'
- id: spec/assurance/AD-004-cg-crate-layout.md:L513
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "1. Exactly one cover. The AD's earlier wording was \"at least one\"; FR-015-AC-7 says every\n\
    \   generated harness has exactly one non-vacuity cover, and L-4 follows it. A spec with no cover\n\
    \   or two covers does not construct.\n2. Exactly one `Call`, and every `Input` and `Assume` precedes\
    \ it. Every `Assert` follows it.\n3. A binding name is bound once, except that a `then` `Let` may\
    \ rebind its own input's name (the\n   `rt::Integer` shadow). Every name, `result` and `callee` is\
    \ a valid Rust identifier.\n4. Every `Input` domain has `minimum <= maximum` and lies within the type\
    \ of `ty`. An inverted or"
- id: spec/assurance/AD-004-cg-crate-layout.md:L528
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'Placement is not an invariant of the spec because the spec does not choose it: the cover is
    a field

    outside `body` precisely so no family can put it elsewhere.'
- id: spec/assurance/AD-004-cg-crate-layout.md:L531
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision H-4. Ownership and visibility.** Each family lowerer constructs its `HarnessSpec`

    (`scalar.rs` at 4b); nothing else constructs one, and `negotiate.rs` does not. `HarnessSpec`,

    `Statement`, `ExprText` and `HarnessSpecError` are `pub(super)` to `kani/generate/` and are not

    re-exported: they are not public API, and the public surface of the crate, its

    `KaniScalarObligationHarness`, `ScalarObligationIdentity`, `ObligationRecord` and the

    `negotiate_kani_obligations` signatures, does not change. A `HarnessSpecError` surfaced from a

    lowerer is an internal invariant failing and maps to `Uns'
- id: spec/assurance/AD-004-cg-crate-layout.md:L542
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision R-1. `render.rs` contract (`kani/generate/render.rs`).** One function,

    `render(&HarnessSpec) -> Result<Artifact, UnsupportedObligation>`:'
- id: spec/assurance/AD-004-cg-crate-layout.md:L545
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "1. It is the spelling site for every harness it renders: the proof attribute, `kani::any`,\n\
    \   `kani::assume`, `kani::cover!`, `#[cfg(kani)]`, the `mod` wrapper, `use super::*;` and the\n \
    \  generated-runtime ABI text the spec cannot say in a typed field (`Init::UnlimitedMeter`).\n   At\
    \ 4b that covers the ported scalar family only; the other families, `v1_bundle.rs`, the\n   corpus\
    \ and collection still spell their own templates, so L-3 (render.rs is the only site for\n   all seven\
    \ spellings) is not yet true and lands after 4f, 4g and 4h, as L-3 says. After the\n   port `scalar.rs`\
    \ spells none of them"
- id: spec/assurance/AD-004-cg-crate-layout.md:L574
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision P-1. The scalar port.** `scalar.rs` keeps `lower_scalar_claim`, `ScalarOperation`,

    `derive_domain`, `unsatisfiable` and `scalar_stem` unchanged. `render_scalar` becomes

    `scalar_harness` (same arguments and result), and it does four things in order: build the

    `HarnessSpec` from the lowered claim (`path`, one `Input` group per operand, the unlimited-meter

    `Let`, the oracle `Call`, the `exact`, `admitted` and `sound` `Let`s, the one `Assert` on `sound`,

    and the `completed` binding with its cover condition), call `render`, build the

    `ScalarObligationIdentity` from the same lowered claim'
- id: spec/assurance/AD-004-cg-crate-layout.md:L588
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision P-2. The port is byte-identical.** For every claim `render_scalar` accepts today,
    the

    rendered `Artifact` (path and source bytes), the `ScalarObligationIdentity`, the `options`, the

    `ObligationRecord` and every digest derived from them equal what the pre-port renderer computes.

    The port introduces no explicitly-specified difference. Rationale: byte identity makes the

    evidence a comparison, needs no new real-Kani run to establish that the old verdicts still hold,

    and keeps the identity digest of AD-003 stable for callers. A difference of any kind, including

    whitespace, is a spec chan'
- id: spec/assurance/AD-004-cg-crate-layout.md:L597
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision P-3. Evidence for the port (TC-025 steps 42 to 47).**'
- id: spec/assurance/AD-004-cg-crate-layout.md:L599
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "1. A differential test. The code PR keeps the body of today's `render_scalar` as a `#[cfg(test)]`\n\
    \   reference function, test-only, and compares the ported path against it, byte for byte, over a\n\
    \   fixed claim set: each of the four operations (`add`, `subtract`, `multiply`, `negate`), a claim\n\
    \   with a literal operand, a claim with an inferred operand, a claim with each operand at an `i64`\n\
    \   extreme, and claims at two unwind values (including `MAX_OBLIGATION_UNWIND`) and two ceilings\n\
    \   values. The compared values are the artifact path and bytes, the identity, `options` and the\n\
    \   record. The"
- id: spec/assurance/AD-004-cg-crate-layout.md:L625
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Not decided here, left to the steps that need it.** The `ProofForContract` variant and contract

    attributes (4c); the frame harness''s `true` cover and its roles (4d); the collection family''s typed

    domain (4h); the corpus template (4g); anything about `negotiate_kani_obligations` beyond the

    renamed call. Whether `record.rs` is absorbed into the spec is not decided: `record.rs` is

    unchanged by 4b.'
- id: spec/assurance/AD-004-cg-crate-layout.md:L851
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '- L-4. A `HarnessSpec` has exactly one cover (FR-015-AC-7; decision H-3), and `render.rs`
    places every cover after all'
- id: spec/assurance/AD-004-cg-crate-layout.md:L986
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "   - 4b: `HarnessSpec`, `render` with the cover rule, and the scalar family ported. Its design\
    \ is\n     decided in \"The HarnessSpec and render contract (step 4b)\" (H-1 to H-3, R-1, P-1 to P-3),\
    \ and\n     its criteria are FR-015-AC-112 to FR-015-AC-117 with TC-025 steps 42 to 47. Scope: `spec.rs`,\n\
    \     `render.rs`, the scalar port, the differential test that proves it byte-identical (P-2, P-3)\
    \ and the\n     rename of `render_scalar` (surface listed in P-1). It does not touch the precondition,\
    \ contract, frame,\n     collection, corpus or V1 templates, and it leaves L-3 unlanded (L-3 lands\
    \ after 4f, "
- id: spec/assurance/AD-004-cg-crate-layout.md:L1584
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "- The step 4b contract (decisions H-1 to P-3) was written against `render_scalar` at `origin/main`\n\
    \  0e0f92e by reading it; no build or test was run. That the port is byte-identical is a claim the\n\
    \  differential test of P-3 checks; the shape of `ExprText` and the statement list is the code PR's\
    \ to\n  confirm by compiling, and a disagreement is fixed in this subsection, not worked around."
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-112 | PLANNED (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor
    and refuses with a typed `HarnessSpecError`, never a panic and never a harness: a spec with no cover
    or more than one cover; a cover whose condition is the literal `true`; no `Call` or more than one;
    an `Input` or `Assume` after the `Call`; an `Assert` before it; a binding name bound twice (other
    than an input''s own rebinding); a name, `result` or `callee` that is not a valid Rust identifier;
    an input domain with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_U'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-113 | PLANNED (IR-858). For every spec that constructs, `render` emits the header,
    the oracle source, the module wrapper and the proof attribute, then the spec''s statements in spec
    order, then its one cover as the last statement of the function: after every assumption, the subject
    call and every assertion, whatever order a family listed them in. A hand-built spec in each of several
    statement orders renders its cover last; a cover is not a field of `body` and no statement order moves
    it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7). | Test (TC-025) |'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-114 | PLANNED (IR-858). `render` of one spec returns equal bytes on repeated calls
    and in a second process, and a spec whose `Input` list is reordered renders the reordered source and
    not the same bytes (AD-004 "The HarnessSpec and render contract (step 4b)", R-1). | Test (TC-025)
    |'
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-115 | PLANNED (IR-858). After the scalar port, for each claim of a fixed set (each
    of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with
    an inferred operand; a claim with each operand at an `i64` extreme; claims at two unwind values including
    `MAX_OBLIGATION_UNWIND`; claims at two ceilings values) the ported path renders the same artifact
    (path and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`, and the same
    digests derived from them, as a test-only reference copy of the pre-port `render_scalar` computes
    in t'
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-116 | PLANNED (IR-858). `render` holds no family-specific template text: for hand-built
    specs whose label, expressions and messages are not the scalar family''s, the rendered source consists
    of the fixed header, the spec''s `oracle_source`, the module wrapper, the proof attribute, the spec''s
    statements rendered from their typed fields, and the one cover, and contains no other text; and the
    ported scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives `render.rs`
    for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1 item 3). | Test (TC-025) |'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '| FR-015-AC-117 | PLANNED (IR-858). With the installed backend, after the scalar port, the
    unmutated scalar harness classifies `Verified` with its cover satisfied and the existing scalar mutation
    control (the oracle''s `Add` swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built
    `HarnessSpec` with an unsatisfiable `Assume`, rendered by `render`, does not classify `Verified` (L-4).
    This run is a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115
    is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3). | Test (TC-025) |'
- id: spec/kani/matrix/TC-025-bounded-kani-obligations.md:L448
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: '## One renderer and the scalar family port (planned, IR-858)'
- id: spec/kani/matrix/TC-025-bounded-kani-obligations.md:L450
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: 'These steps verify AD-004 step 4b (AD-004 "The HarnessSpec and render contract (step 4b)");
    they back FR-015-AC-112 to FR-015-AC-117. Steps 42 to 47

    are numbered after step 41 and are independent of the IR-805 prose above.'
- id: spec/kani/matrix/TC-025-bounded-kani-obligations.md:L453
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "42. Build `HarnessSpec`s by hand that break each invariant: no cover, two covers, a `true`\
    \ cover\n    condition, no call, two calls, an input after the call, an assertion before the call,\
    \ a name bound\n    twice, a name that is not a Rust identifier, an inverted domain, a domain outside\
    \ its type, an\n    `unwind` of 0 and of `MAX_OBLIGATION_UNWIND` plus one, a message containing each\
    \ of `\"`, `\\`, `{`,\n    `}` and a line break, and `ExprText`s that do not parse, spell `kani::`,\
    \ hold a block, `unsafe`,\n    closure, loop or `let` expression, or hold a macro other than `matches!`\
    \ (`assert!`, `panic!`"
- id: spec/kani/matrix/tests.md:L40
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "| FR-015 | FR-015-AC-112 through FR-015-AC-117 | TC-025 | \U0001F6A7 Planned (IR-858, AD-004\
    \ step 4b); no `spec.rs` or `render.rs` exists, and the scalar family still renders in `scalar.rs`\
    \ |"
- id: spec/kani/matrix/tests.md:L124
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: "| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-112, FR-015-AC-113,\
    \ FR-015-AC-114, FR-015-AC-115, FR-015-AC-116, FR-015-AC-117 | \U0001F6A7 Planned (IR-858, AD-004\
    \ step 4b): HarnessSpec constructor, the one renderer, the byte-identical scalar port and its differential\
    \ test; no code yet |"
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: context_only
  excerpt: "//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani\n\
    //! harness (FR-015).\n\nuse crate::oracle::operator_catalog::{\n    Operator as CatalogOperator,\
    \ ScalarKaniOperator as ScalarOperation,\n};\nuse quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};\n\
    \nuse crate::{\n    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},\n    kani::abi::{adapter_options,\
    \ i64_literal, readable_component, KaniSolver},\n    kani::generate::outcome::{DerivedDomain, KaniObligationRequest,\
    \ UnsupportedObligation},\n    kani::generate::record::{harness_path, record},\n    kan"
- id: src/kani/generate/record.rs
  path: src/kani/generate/record.rs
  role: context_only
  excerpt: "//! The validated harness path and the persisted per-harness record that every family's render\n\
    //! step builds last.\n\nuse serde::Serialize;\n\nuse crate::{\n    core::artifact::Artifact,\n  \
    \  core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError},\n    kani::generate::outcome::UnsupportedObligation,\n\
    };\n\n/// The validated `module::harness` identity of a harness, built once where the harness is\n\
    /// generated. The names are `kob_`-prefixed readable components, so a refusal is an internal\n///\
    \ invariant failing and is reported as invalid generated Rust.\npub(super) fn harness_path(\n    m"
- id: src/core/identity.rs
  path: src/core/identity.rs
  role: context_only
  excerpt: '//! Identity newtypes of a generated Kani harness (AD-004 `core/identity`, L-10).

    //!

    //! A harness is named by a module symbol and a harness symbol, and the two are different kinds of

    //! name: swapping them at a call site must not compile. Each is a validated Rust identifier,

    //! built once where a harness is generated, and [`HarnessPath`] is the pair. They serialize as

    //! the bare string, so a persisted identity record is unchanged, and they deserialize from it

    //! through the same validation.


    use std::fmt;


    use serde::{Deserialize, Serialize};


    /// A string offered as a harness or module'
- id: src/kani/abi.rs
  path: src/kani/abi.rs
  role: context_only
  excerpt: "//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver\n\
    //! choice and the adapter option vector.\n\nuse quire_contract_model::{IntegerDomain, IntegerType,\
    \ OverflowPolicy};\nuse serde::{Deserialize, Serialize};\n\n/// Position of one primitive dependency\
    \ in the generated subject ABI.\n#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]\n\
    #[serde(rename_all = \"snake_case\")]\npub enum KaniBindingRole {\n    /// A copied current/pre value\
    \ passed to the customer subject.\n    Argument,\n    /// A copied post-state value returned by the\
    \ customer subject."
- id: tests/it/cover_last.rs
  path: tests/it/cover_last.rs
  role: context_only
  excerpt: '//! FR-015-AC-7, AC-53 and AC-58: every harness the generator emits ends with exactly one

    //! `kani::cover!`, as the last statement of its body.

    //!

    //! The guard has two halves. The inspection drives every emitting entry point, parses each

    //! emitted source with `syn` and checks every function attributed `#[kani::proof]` or

    //! `#[kani::proof_for_contract]`. The scan reads the non-test string literals of `src/` and fails

    //! when a file outside the driven set spells either proof attribute, so a new emitter cannot be

    //! added without being driven here. The emitters have no single seam today '
- id: SR-12180
  path: reviews/ir-858-spec-review-base.md
  role: context_only
  excerpt: '---

    id: "SR-12180"

    title: "CG PR 421 spec review (base checklist): AD-004 step 4b HarnessSpec and render"

    type: SpecReview

    analysis: base

    review_set: subset

    scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858);
    spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step
    4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112
    to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md;
    checked against src/ and '
- id: SR-12184
  path: reviews/ir-858-spec-review-criterion-strength.md
  role: context_only
  excerpt: '---

    id: "SR-12184"

    title: "CG PR 421 spec review (criterion strength): FR-015-AC-112 to FR-015-AC-117"

    type: SpecReview

    analysis: criterion-strength

    review_set: subset

    scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858);
    spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step
    4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112
    to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md;
    checked agains'
- id: SR-12182
  path: reviews/ir-858-spec-review-evidence.md
  role: context_only
  excerpt: '---

    id: "SR-12182"

    title: "CG PR 421 spec review (evidence): AD-004 step 4b HarnessSpec and render"

    type: SpecReview

    analysis: evidence

    review_set: subset

    scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858);
    spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step
    4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112
    to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md;
    checked against src/ and te'
- id: SR-12181
  path: reviews/ir-858-spec-review-integrity.md
  role: context_only
  excerpt: '---

    id: "SR-12181"

    title: "CG PR 421 spec review (integrity): AD-004 step 4b HarnessSpec and render"

    type: SpecReview

    analysis: integrity

    review_set: subset

    scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858);
    spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step
    4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112
    to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md;
    checked against src/ and '
- id: SR-12183
  path: reviews/ir-858-spec-review-scope-boundary.md
  role: context_only
  excerpt: '---

    id: "SR-12183"

    title: "CG PR 421 spec review (scope boundary): AD-004 step 4b HarnessSpec and render"

    type: SpecReview

    analysis: scope-boundary

    review_set: subset

    scope: "agent-ix/quire-contract-codegen@77f5178102658bdac3d77b4024d262f029e5da46 (PR #421, IR-858);
    spec/assurance/AD-004-cg-crate-layout.md (The HarnessSpec and render contract (step 4b), L-4, step
    4b, import rule, Not verified), spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-112
    to FR-015-AC-117), spec/kani/matrix/TC-025-bounded-kani-obligations.md (steps 42 to 47), spec/kani/matrix/tests.md;
    checked against'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
