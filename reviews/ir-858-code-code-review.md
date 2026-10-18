---
id: SR-12910
title: IR858 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR858 PR440; AD004 step4b, private generate
  spec/render/scalar and owning status/evidence delta
review_set: subset
---

## Summary

Ticket: IR-858. Frozen PR440 review; Reviewed the complete assigned code/spec delta and its owning obligations.

## Verdict

CONDITIONAL — three medium constructor edge defects require a same-PR fix round. The existing scalar port is otherwise byte-preserving under the reviewed differential cases.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Bare kani and trailing rt::kani are rejected even though neither contains a kani namespace prefix. The path visitor rejects any segment named kani; ExprText::new("kani") and ExprText::new("rt::kani") both return ForbiddenExpression. H-2 permits these ordinary expression names. Restrict namespace detection to an actual following separator while retaining spaced/commented Kani namespace rejection. | src/kani/generate/spec.rs:111-115 |
| FND-002 | medium | Raw Kani identifiers bypass expression validation. ExprText::new("r#kani :: any::<i64>()"), "r#kani /*space*/ :: assume(false)" and "matches!(r#kani :: any::<bool>(), true)" all construct successfully. Rust resolves r#kani as kani, but syn Ident equality against "kani" does not normalize the raw prefix; the text substring check also misses the spaced separator. Normalize identifier spelling when rejecting Kani namespace paths, including nested matches! arguments. | src/kani/generate/spec.rs:111-115 |
| FND-003 | medium | Duplicate bindings are checked by source spelling rather than Rust identifier identity. An Input named x followed by a separate Input or Let named r#x passes identifier parsing and BTreeSet insertion, although Rust treats both as the same binding name. This accepts shadowing outside the input-own-then exception and violates H-3. Canonicalize raw identifiers consistently for duplicate detection and the own-input shadow comparison; retain original spelling for rendering. | src/kani/generate/spec.rs:223-228,298-303 |

## Coverage

The exact frozen candidate is 83372499f946d72174574cbacf4162951de2c8f5, reviewed against c7d0601. No source, dependency, staging, CI, public API or branch edits were made. No applicable AssuranceProfile exists under spec/. No Cargo, Kani or rustfmt was launched by this reviewer. Root granted one Cargo-free rustc probe using the existing syn dependency, a 16GiB/zero-swap systemd scope, QSPEC_DIR and the private review scratch; exact-name process guards were empty before and after. It copied the production ExprText validator verbatim into a scratch probe and reproduced FND-001/FND-002. The duplicate-name probe verified both spellings parse and insert distinctly, and actual Rust shadow resolution; the constructor call flow was traced statically for FND-003. Initial compile selected a syn build lacking visit; retry selected the already-built visit-enabled library. Probe commands/results are retained in probe.rs, probe-build.log and probe-output.log. No wider runtime equivalence is inferred from this probe.

Author logs were inspected as author evidence: focused lib 16/16 plus second-process child, cover_last 5/5 driving twelve entrypoints, lint and formatting pass; the private control explicitly ran --lib --ignored --exact and passed with source assertions success_checks > 0 and exact CoverUnsatisfied { satisfied: 0, total: 1 }. The numeric successful-check count is not printed. The clean frozen author make kani-gate passed 36/36, 861 seconds, Kani 0.68.0, including healthy Verified and Add-to-Subtract Falsified with the soundness-assertion counterexample. make kani-gate does not cover the private ignored lib test. Final independent aggregate and full Kani gates remain with the lead after fixes, custody and rebase.

The pre-port renderer reference is exactly the assigned-base function body apart from its name. The 104-case differential compares full artifact path/bytes, complete identity/options and record. It covers four operations, ordinary/literal/inferred operand metadata, each operand at both i64 extremes, unwind 2/1024 and wall-clock ceilings 30/600 seconds. Normal identities must successfully digest; extremes assert the existing private canonical debug refusal and compare old/new Result diagnostics exactly. Artifact/record digests must succeed and agree. Source both oversized and invalid preserves byte count and size-before-syntax precedence. No encoder change was made. Lowering, operation/domain/stem helpers are unchanged. The copied test-only reference is expressly required by P-3; no external vocabulary was vendored.

Renderer imports and protected fields conform to the private architecture. ExprText checks descend through match bodies, matches! expression/pattern arguments, nested macros and Pat::Macro; guarded matches!, blocks/if bodies, unsafe/async/closures/loops/let/const/try blocks and attributes are refused. Constructor covers typed error/count/order/domain/unwind/message boundaries. Renderer exact output tests use non-scalar labels, expressions and messages; repeated/process output, reordered inputs and final parsed cover are checked. Options are computed once and read from the spec. Only scalar calls render; other families, held steps, native receipts, dependency resolution and execution remain unchanged. IR818 quantity claim fixture initializers are absent from this base and must be retained on the lead's later rebase; this PR adds no quantity Kani renderer.

Method selection: code-review with Rust lane, genuine planless gap-analysis, spec-review/base and spec-review/evidence ran. Integrity, EARS, dependency, domain-object, scope-boundary, failure-domain, risk/complexity, architecture and security lenses were not applied to unchanged requirement grammar, structure, edges or boundaries: the only spec edits update implementation status and evidence. Criterion-strength was not rerun because no criterion behavior changed. The expanded optional repository semantic analysis was not commissioned; the dispatch-authorized focused intent/oracle review is in code-review. No subagents were spawned.

## Examined units

All unit text below is verbatim; full units are retained even when the tracker uses a shortened prefix.

```yaml
scope:
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor and refuses with a
    typed `HarnessSpecError`, never a panic and never a harness: a spec with no cover or more than one cover; a
    cover whose condition is the literal `true`; no `Call` or more than one; an `Input` or `Assume` after the `Call`;
    an `Assert` before it; a binding name bound twice (other than an input''s own rebinding); a name, `result` or
    `callee` that is not a valid Rust identifier; an input domain with `minimum` above `maximum` or outside its
    type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`; a message or label containing `"`, `\`, `{`, `}` or a
    line break; and an `ExprText` that does not parse as a Rust expression, spells `kani::` or `#[`, contains a
    block, `unsafe`, `async`, closure, loop or `let` expression, or contains a macro invocation other than `matches!`,
    a `matches!` with a guard, or a forbidden form nested inside a `matches!` argument (AD-004 "The HarnessSpec
    and render contract (step 4b)", H-2 and H-3).'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). For every spec that constructs, `render` emits the header, the oracle source,
    the module wrapper and the proof attribute, then the spec''s statements in spec order, then its one cover as
    the last statement of the function: after every assumption, the subject call and every assertion, whatever order
    a family listed them in. A hand-built spec in each of several statement orders renders its cover last; a cover
    is not a field of `body` and no statement order moves it (AD-004 "The HarnessSpec and render contract (step
    4b)", R-1; FR-015-AC-7).'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). `render` of one spec returns equal bytes on repeated calls and in a second process,
    and a spec whose `Input` list is reordered renders the reordered source and not the same bytes (AD-004 "The
    HarnessSpec and render contract (step 4b)", R-1).
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of `quire.op.integer.add`,
    `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred operand; a claim with each
    operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`; claims at two ceilings
    values) the ported path renders the same artifact (path and source bytes), `ScalarObligationIdentity`, `options`
    and `ObligationRecord`, and the same digests derived from them, as a test-only reference copy of the pre-port
    `render_scalar` computes in the same test. No stored expectation file and no regenerate mode is involved. The
    port has no explicitly specified difference. The refusals `ResourceLimitExceeded` and `InvalidGeneratedSyntax`
    keep their values and their order, now raised by `render` (AD-004 "The HarnessSpec and render contract (step
    4b)", P-2 and P-3).
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). `render` holds no family-specific template text: for hand-built specs whose label,
    expressions and messages are not the scalar family''s, the rendered source consists of the fixed header, the
    spec''s `oracle_source`, the module wrapper, the proof attribute, the spec''s statements rendered from their
    typed fields, and the one cover, and contains no other text; and the ported scalar harness is emitted by `render`,
    so that the FR-015-AC-58 inspection drives `render.rs` for it (AD-004 "The HarnessSpec and render contract (step
    4b)", R-1 item 3).'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port, the unmutated scalar harness
    classifies `Verified` with its cover satisfied and the existing scalar mutation control (the oracle's `Add`
    swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable
    `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run is a smoke over the ported source
    and not the evidence of equivalence, which FR-015-AC-115 is (AD-004 "The HarnessSpec and render contract (step
    4b)", P-3).
- id: AD-004-H-1
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "**Decision H-1. The shape of `HarnessSpec` (`kani/generate/spec.rs`).** A `HarnessSpec` is a typed\n\
    description of one harness, with no Kani spelling in it. Its fields:\n\n| Field | Type and meaning |\n| ---\
    \ | --- |\n| `label` | The text of the `// Obligation: <label>` header line. The scalar lowerer supplies ``exact-scalar\
    \ `<operation identity>` ``. `render` writes the fixed header: the line `// SPDX-License-Identifier: MIT OR\
    \ Apache-2.0`, the Obligation line, a blank line, then `oracle_source`, then one newline, then the module wrapper\
    \ (the bytes `render_scalar` writes today, `scalar.rs` lines 463 to 470). |\n| `path` | The validated `HarnessPath`\
    \ (module and harness symbols), defined in `core/identity.rs` and built by `harness_path` in `record.rs`; neither\
    \ moves. It is the only source of the `mod` and `fn` names and of the artifact path `src/generated/<module>.rs`.\
    \ |\n| `oracle_source` | The generated FR-014 oracle source, placed verbatim after the header and before the\
    \ harness module. It is opaque to `render.rs`. |\n| `proof` | `ProofKind`. Step 4b defines one variant, `Proof`\
    \ (`#[kani::proof]`). Step 4c adds `ProofForContract` with its own spec change. |\n| `body` | An ordered list\
    \ of `Statement`: `Input`, `Assume`, `Let`, `Call` and `Assert`. See below. |\n| `cover` | Exactly one `Cover`:\
    \ a condition (`ExprText`) and a message. See H-3. |\n| `unwind`, `solver` | The loop bound (`1..=MAX_OBLIGATION_UNWIND`)\
    \ and the `KaniSolver`. They are not emitted into the source; they are the run options. |\n| `options` | The\
    \ adapter option vector (`adapter_options(path, unwind, solver, false)`), computed once by the constructor from\
    \ `path`, `unwind` and `solver`. The family's identity record reads `unwind`, `solver` and `options` from the\
    \ spec, never from the request a second time, so the run options and the identity cannot disagree. |\n\n`Statement`\
    \ variants (all fields typed, none is a line of Rust):\n\n- `Input { name, ty, domain, then }`: a symbolic input.\
    \ `ty` is a `KaniPrimitiveType` from\n  `kani/abi.rs` (4b uses `I64`). `domain` is an inclusive `[minimum, maximum]`\
    \ pair of the same\n  type. Render emits `let <name>: <ty> = kani::any();` and `kani::assume(<name> >= <minimum>\
    \ &&\n  <name> <= <maximum>);`. `then` is the ordered list of `Let`s that must follow this input\n  immediately\
    \ (the scalar harness copies the input to `<name>_native` and shadows it with its\n  `rt::Integer`); it keeps\
    \ today's per-operand grouping, which byte identity needs.\n- `Assume { condition }`: an extra `kani::assume(<condition>)`\
    \ after the inputs. The scalar family\n  uses none. It is in 4b's shape for one reason: the L-4 control (FR-015-AC-117)\
    \ is a hand-built\n  spec with an unsatisfiable `Assume`, since no scalar claim can carry an unsatisfiable input\n\
    \  domain (the constructor and the lowerer both refuse one). At 4b only that test constructs it, so the variant\
    \ carries\n  `#[cfg_attr(not(test), allow(dead_code))]`, scoped to it, so that `make lint` (clippy\n  `--all-targets\
    \ -D warnings`) stays green; the attribute is removed when 4c's lowerer builds an\n  `Assume`. 4c may extend\
    \ its use; it adds no field here.\n- `Let { name, ty, mutable, init }`: a binding. `ty` is an optional annotation\
    \ of a spec-local `LetType`\n  whose only variant at 4b is `I128` (nothing else is constructed, so no unused\
    \ variant exists\n  for `make lint` to reject), because the scalar harness writes `let exact: i128 = ...`. It\
    \ is not\n  `KaniPrimitiveType`, which is public API with `Boolean` and `I64` only and which 4b does not\n \
    \ widen.\n  `init` is an `ExprText` or `Init::UnlimitedMeter` (the all-`u64::MAX` `rt::Meter` of the\n  generated\
    \ oracle runtime, whose multi-line spelling `render.rs` owns because every family that\n  calls a generated\
    \ oracle shares it).\n- `Call { result, callee, args }`: the one subject call. `args` are `Borrow(name)` or\n\
    \  `BorrowMut(name)`.\n- `Assert { condition, message }`: `assert!(<condition>, \"<message>\");`."
- id: AD-004-H-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "**Decision H-2. `ExprText` is the one text a lowerer supplies, and it is an expression only.**\n`ExprText`\
    \ is one Rust expression as text, for the native `i128` expression, the outcome `match`\nand the `matches!`\
    \ of the cover: the family's own meaning, not harness structure. It is a\nvalidated newtype whose constructor\
    \ refuses text that:\n\n- does not parse as a `syn::Expr`;\n- spells `kani::` or `#[`;\n- contains a block,\
    \ `unsafe`, `async`, closure, loop (`loop`, `while`, `for`) or `let` expression\n  anywhere in its tree (a `match`\
    \ is allowed: the scalar outcome check is one); or\n- contains a macro invocation other than `matches!`, or\
    \ a `matches!` that has a guard\n  (`if ...`). The arguments of `matches!` are parsed as an expression and a\
    \ pattern, and the\n  checks above apply to the expression, so a macro, block or closure nested in it is refused\
    \ too.\n\nSo an `ExprText` cannot hold a statement, an assertion, a panic or a cover, and R-1 item 2's\npromise\
    \ that nothing a lowerer supplies changes where the cover goes is enforced rather than\nhoped for. Adding a\
    \ macro to the allowlist is a spec change here. An `ExprText` may span lines;\n`render.rs` preserves its text\
    \ and indentation verbatim, including lines after the first. A typed\nexpression tree was considered and rejected\
    \ for 4b: it would be a second Rust AST inside the crate\nfor four arithmetic expressions, and `syn` is already\
    \ the validity check on the rendered whole.\nThe cost is that an `ExprText` is checked for parse, form and the\
    \ absence of Kani spellings, not\nfor meaning; the real-Kani lane remains the meaning check.\n\nA `message`\
    \ (of an `Assert` or the `Cover`) is a string with no `\"`, `\\`, `{`, `}` or line\nbreak; the constructor refuses\
    \ one that has any, as a typed error, rather than escaping it, because\nthe scalar assertion message interpolates\
    \ the operation identity and an escape rule would change\nbytes. `label` follows the same rule except that it\
    \ may hold backticks and may not hold a line\nbreak."
- id: AD-004-H-3
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "**Decision H-3. Invariants, enforced by `HarnessSpec::new`, the only constructor.** A refusal is a\n\
    typed `HarnessSpecError`; none is a panic.\n\n1. Exactly one cover. The AD's earlier wording was \"at least\
    \ one\"; FR-015-AC-7 says every\n   generated harness has exactly one non-vacuity cover, and L-4 follows it.\
    \ A spec with no cover\n   or two covers does not construct.\n2. Exactly one `Call`, and every `Input` and `Assume`\
    \ precedes it. Every `Assert` follows it.\n3. A binding name is bound once, except that a `then` `Let` may rebind\
    \ its own input's name (the\n   `rt::Integer` shadow). Every name, `result` and `callee` is a valid Rust identifier.\n\
    4. Every `Input` domain has `minimum <= maximum` and lies within the type of `ty`. An inverted or\n   out-of-type\
    \ domain is a typed refusal, not a harness whose assumption is unsatisfiable.\n5. `unwind` is in `1..=MAX_OBLIGATION_UNWIND`.\n\
    6. Every `ExprText` satisfies H-2 and every message satisfies the message rule of H-2. The cover\n   condition\
    \ is an `ExprText` that is not the literal `true`. This last rule is new in 4b: no\n   FR-015 text forbids a\
    \ `true` cover today, but a `true` cover is reachable whenever the harness is\n   and so is not the property's\
    \ own reachability that IR-464 requires. A frame harness's\n   `kani::cover!(true, ...)` (FR-015-AC-7) is 4d's\
    \ to allow in its own spec change.\n\nPlacement is not an invariant of the spec because the spec does not choose\
    \ it: the cover is a field\noutside `body` precisely so no family can put it elsewhere."
- id: AD-004-H-4
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision H-4. Ownership and visibility.** Each family lowerer constructs its `HarnessSpec`

    (`scalar.rs` at 4b); nothing else constructs one, and `negotiate.rs` does not. `HarnessSpec`,

    `Statement`, `ExprText` and `HarnessSpecError` are `pub(super)` to `kani/generate/` and are not

    re-exported: they are not public API, and the public surface of the crate, its

    `KaniScalarObligationHarness`, `ScalarObligationIdentity`, `ObligationRecord` and the

    `negotiate_kani_obligations` signatures, does not change. A `HarnessSpecError` surfaced from a

    lowerer is an internal invariant failing and maps to `UnsupportedObligation::InvalidGeneratedSyntax`

    with the error text, as `harness_path`''s refusal does today (`record.rs`). The spec holds no

    identity record: the family builds its own identity record (`ScalarObligationIdentity`) beside the

    spec, from the same lowered claim, and `record.rs` joins identity and artifact as today.'
- id: AD-004-R-1
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "**Decision R-1. `render.rs` contract (`kani/generate/render.rs`).** One function,\n`render(&HarnessSpec)\
    \ -> Result<Artifact, UnsupportedObligation>`:\n\n1. It is the spelling site for every harness it renders: the\
    \ proof attribute, `kani::any`,\n   `kani::assume`, `kani::cover!`, `#[cfg(kani)]`, the `mod` wrapper, `use\
    \ super::*;` and the\n   generated-runtime ABI text the spec cannot say in a typed field (`Init::UnlimitedMeter`).\n\
    \   At 4b that covers the ported scalar family only; the other families, `v1_bundle.rs`, the\n   corpus and\
    \ collection still spell their own templates, so L-3 (render.rs is the only site for\n   all seven spellings)\
    \ is not yet true and lands after 4f, 4g and 4h, as L-3 says. After the\n   port `scalar.rs` spells none of\
    \ them.\n2. It places the cover. The body is the header, the oracle source, the module wrapper, the\n   attribute,\
    \ then the `body` statements in spec order, then the cover as the last statement of\n   the function, after\
    \ every assumption, the call and every assertion (IR-464, FR-015-AC-7).\n   Nothing a lowerer provides can change\
    \ where the cover goes (H-2 keeps a cover or an assertion\n   out of an `ExprText`).\n3. It holds no family-specific\
    \ template text. It has one emitter per `Statement` variant, per\n   `ProofKind` and per `Init`; a family's\
    \ meaning reaches it only as data (`label`, `ExprText`,\n   `message`). This is evidenced by behaviour (FR-015-AC-113):\
    \ specs whose labels, expressions and\n   messages are not the scalar family's render to source that holds exactly\
    \ what the spec\n   supplied and the fixed structure, and nothing else.\n4. Design rationale: it is a pure function\
    \ of the spec, with no map or set iteration, no clock,\n   no environment and no address-dependent formatting,\
    \ so equal specs render equal bytes.\n   The scalar harness wrapper and body preserve the pre-port template's\
    \ actual whitespace: eight\n   spaces before each `Input` declaration and no renderer-added indentation on other\
    \ lines. Rust\n   string continuations in that template strip its source indentation.\n5. It validates what\
    \ it renders as today's `render_scalar` does, in the same order and with the\n   same refusals: first `ResourceLimitExceeded`\
    \ when the source exceeds `MAX_GENERATED_SOURCE_BYTES`\n   (core/artifact), then `InvalidGeneratedSyntax` when\
    \ `syn::parse_file` rejects the source. Those\n   two checks leave `scalar.rs`; a family not yet ported keeps\
    \ its own check (collection and\n   frame carry the same two checks today).\n6. It returns the `Artifact` at\
    \ `src/generated/<module>.rs`; it builds no identity and no record.\n   It imports `spec`, `outcome` (for `UnsupportedObligation`),\
    \ `core::artifact` and `kani::abi`,\n   and not `record`."
- id: AD-004-P-1
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision P-1. The scalar port.** `scalar.rs` keeps `lower_scalar_claim`, `ScalarOperation`,

    `derive_domain`, `unsatisfiable` and `scalar_stem` unchanged. `render_scalar` becomes

    `scalar_harness` (same arguments and result), and it does four things in order: build the

    `HarnessSpec` from the lowered claim (`path`, one `Input` group per operand, the unlimited-meter

    `Let`, the oracle `Call`, the `exact`, `admitted` and `sound` `Let`s, the one `Assert` on `sound`,

    and the `completed` binding with its cover condition), call `render`, build the

    `ScalarObligationIdentity` from the same lowered claim and the spec''s `unwind`, `solver` and

    `options`, and call `record`. The helpers `native_expression` and `operand_names` stay as the

    source of the `ExprText`s. The rename''s whole surface is: the import and call in `negotiate.rs`

    (`:85`, `:135`) and its module doc (`:23`), the doc mentions in `src/oracle/scalar/mod.rs`

    (`:637`, `:3158`), `tests/exact_scalar_support/agreement_cases.rs` (`:98`, `:102`) and

    `scalar.rs`''s own intra-doc links. The code PR may keep the old name instead; nothing in this

    contract needs the rename.'
- id: AD-004-P-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '**Decision P-2. The port is byte-identical.** For every claim `render_scalar` accepts today, the

    rendered `Artifact` (path and source bytes), the `ScalarObligationIdentity`, the `options`, the

    `ObligationRecord` and every digest derived from them equal what the pre-port renderer computes.

    The port introduces no explicitly-specified difference. Rationale: byte identity makes the

    evidence a comparison, needs no new real-Kani run to establish that the old verdicts still hold,

    and keeps the identity digest of AD-003 stable for callers. A difference of any kind, including

    whitespace, is a spec change to this subsection before it is a code change, and if one is

    accepted the real-Kani lane of P-3 item 2 becomes mandatory for it.'
- id: AD-004-P-3
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "**Decision P-3. Evidence for the port (TC-025 steps 42 to 47).**\n\n1. A differential test. The code\
    \ PR keeps the body of today's `render_scalar` as a `#[cfg(test)]`\n   reference function, test-only, and compares\
    \ the ported path against it, byte for byte, over a\n   fixed claim set: each of the four operations (`add`,\
    \ `subtract`, `multiply`, `negate`), a claim\n   with a literal operand, a claim with an inferred operand, a\
    \ claim with each operand at an `i64`\n   extreme, and claims at two unwind values (including `MAX_OBLIGATION_UNWIND`)\
    \ and two ceilings\n   values. The compared values are the artifact path and bytes, the identity, `options`\
    \ and the\n   record. There is no bless or regenerate mode: the reference is code, not a stored file. The\n\
    \   reference copy is deleted when step 4g retires the scalar render path or sooner if the reviewer\n   agrees\
    \ that the L-4 and AC-58 evidence has replaced it.\n2. The `kani` lane scalar tests of `tests/it/kani_obligations.rs`\
    \ run once under the installed\n   backend after the port, as the smoke that the rendered source still verifies:\
    \ the existing\n   scalar mutation control (`tests/it/kani_obligations.rs:2950`, FR-015-AC-37: the oracle's\
    \ `Add`\n   swapped for `Subtract`) classifies `Falsified`, and the unmutated harness classifies `Verified`\n\
    \   with its cover satisfied. This lane is a smoke, not the proof of equivalence: item 1 is.\n   QSL's `left\
    \ + right + 1` control (QSL-342 QI #10, L-5) is a quire-integration test and not\n   part of this PR's evidence.\n\
    3. The L-4 control: a new hand-built `HarnessSpec` with an unsatisfiable `Assume`, rendered by\n   `render`\
    \ into a generated crate and run under the installed backend, does not classify\n   `Verified` (it classifies\
    \ `CoverUnsatisfied`). No scalar claim can carry this, so it is\n   built from the spec, which is why `Assume`\
    \ is in the 4b shape.\n4. The FR-015-AC-58 inspection still drives every emitting entry point; the ported scalar\n\
    \   harness is emitted by `render`, so that inspection now reaches `render.rs` for scalar. The\n   guard's list\
    \ of driven files loses `scalar.rs` and gains `render.rs`.\n5. Constructor and renderer tests over hand-built\
    \ `HarnessSpec`s that no family produces (see\n   TC-025 steps 42 and 43)."
- id: TC-025-step-42
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "42. Build `HarnessSpec`s by hand that break each invariant: no cover, two covers, a `true` cover\n \
    \   condition, no call, two calls, an input after the call, an assertion before the call, a name bound\n   \
    \ twice, a name that is not a Rust identifier, an inverted domain, a domain outside its type, an\n    `unwind`\
    \ of 0 and of `MAX_OBLIGATION_UNWIND` plus one, a message containing each of `\"`, `\\`, `{`,\n    `}` and a\
    \ line break, and `ExprText`s that do not parse, spell `kani::`, hold a block, `unsafe`,\n    closure, loop\
    \ or `let` expression, or hold a macro other than `matches!` (`assert!`, `panic!`,\n    a `kani` macro), a `matches!`\
    \ with a guard, and a `matches!` whose expression holds a nested\n    `panic!` or block. Read a typed `HarnessSpecError`\
    \ for each and no spec; read that a `match` and a\n    `matches!` expression construct (FR-015-AC-112)."
- id: TC-025-step-43
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "43. Render valid hand-built specs whose statements are in several legal orders and read the cover as\n\
    \    the last statement of the function in each; render the same spec twice, and in a second process,\n    and\
    \ compare the bytes; reorder two inputs and read different bytes; render a spec with non-scalar\n    labels,\
    \ expressions and messages and read that the source is the fixed structure plus exactly what\n    the spec supplied\
    \ (FR-015-AC-113, FR-015-AC-114, FR-015-AC-116)."
- id: TC-025-step-44
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "44. Keep today's `render_scalar` body as a `#[cfg(test)]` reference function. Over the claim set of\n\
    \    FR-015-AC-115 (four operations, a literal operand, an inferred operand, `i64` extremes, two unwind\n  \
    \  values including `MAX_OBLIGATION_UNWIND`, two ceilings values) compare the ported path with the\n    reference:\
    \ artifact path and bytes, identity, options and record, equal. There is no bless mode."
- id: TC-025-step-45
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "45. Drive a claim whose source exceeds `MAX_GENERATED_SOURCE_BYTES` and read `ResourceLimitExceeded`\n\
    \    with the same byte count as the reference, and drive invalid generated syntax and read\n    `InvalidGeneratedSyntax`,\
    \ each raised by `render`, the size check first (FR-015-AC-115)."
- id: TC-025-step-46
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "46. Run the FR-015-AC-58 inspection with `render.rs` among the driven files and `scalar.rs` out of the\n\
    \    list; read the scalar harness's cover as its last statement (FR-015-AC-58, FR-015-AC-116)."
- id: TC-025-step-47
  path: spec/kani/matrix/TC-025-bounded-kani-obligations.md
  role: examined
  excerpt: "47. With the installed backend, run the existing scalar lane tests (`tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic`\n\
    \    and its healthy run): `Verified` with its cover satisfied, and the\n    `Add` to `Subtract` mutation `Falsified`;\
    \ then run a hand-built spec with an unsatisfiable `Assume`\n    through `render` and read that it is not `Verified`\
    \ (FR-015-AC-117).\n\n## Blocked\n\n- Frame harnesses in the clause negotiation: FR-025 accounts every frame\
    \ obligation\n  `unsupported` there until QSpec decides how a frame node lowers into a Kani form (ADR-004).\
    \ The\n  frame of a `postcondition` state clause over one integer field is instead generated with that\n  clause\
    \ by `generate_state_frame_obligations` (FR-015-AC-26 to FR-015-AC-32).\n- V2 scalar harnesses outside `IntegerArithmetic`:\
    \ an IR-confirmed claim over\n  one of the four `quire.op.integer.{add,sub,mul,negate}` identities reaches a\n\
    \  real harness unless a ground independent of the operation (an\n  i64-unrepresentable endpoint, the source\
    \ ceiling) displaces it. Every other\n  confirmed family is refused as `OperationNotRendered`, an unbuilt renderer\
    \ in\n  this generator, not an upstream block, and so is a claim of a rendered family\n  from a claim map this\
    \ generator did not produce (NFR-005-AC-2). A claim this generator lowered but\n  whose operation it did not\
    \ confirm against the node's own catalogued\n  identity, mode or law definition is refused as `CallerDeclaredOperation`.\n\
    - Model and graph bounds: refused as blocked on `agent-ix/quire-spec-language#120`."
- id: src/kani/generate/spec.rs
  path: src/kani/generate/spec.rs
  role: examined
  excerpt: "//! Validated, private harness descriptions (FR-015).\n\nuse super::outcome::{UnsupportedObligation,\
    \ MAX_OBLIGATION_UNWIND};\nuse crate::{\n    core::identity::HarnessPath,\n    kani::abi::{adapter_options,\
    \ KaniPrimitiveType, KaniSolver},\n};\nuse std::collections::BTreeSet;\nuse syn::{\n    parse::{Parse, ParseStream},\n\
    \    visit::{self, Visit},\n    Expr, Pat, Token,\n};\n\n#[derive(Clone, Debug, Eq, PartialEq)]\npub(super)\
    \ enum HarnessSpecError {\n    CoverCount(usize),\n    LiteralTrueCover,\n    CallCount(usize),\n    InputAfterCall,\n\
    \    AssumeAfterCall,\n    AssertBeforeCall,\n    DuplicateBinding(String),\n "
- id: src/kani/generate/render.rs
  path: src/kani/generate/render.rs
  role: examined
  excerpt: "//! Pure ordered emission of validated harness descriptions (FR-015).\n\nuse super::{\n    outcome::UnsupportedObligation,\n\
    \    spec::{Argument, HarnessSpec, Init, LetBinding, LetType, ProofKind, Statement},\n};\nuse crate::{\n   \
    \ core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},\n    kani::abi::{i64_literal, KaniPrimitiveType},\n\
    };\n\nfn line(source: &mut String, text: &str) {\n    for part in text.split('\\n') {\n        source.push_str(part);\n\
    \        source.push('\\n');\n    }\n}\nfn binding(source: &mut String, binding: &LetBinding) {\n    let mutable\
    \ = if binding.mutable { \"mut \" } else { \"\" };\n    le"
- id: src/kani/generate/scalar.rs
  path: src/kani/generate/scalar.rs
  role: examined
  excerpt: "//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani\n//! harness\
    \ (FR-015).\n\nuse crate::oracle::operator_catalog::{\n    Operator as CatalogOperator, ScalarKaniOperator as\
    \ ScalarOperation,\n};\nuse quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};\n\n#[cfg(test)]\n\
    use crate::{\n    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},\n    kani::abi::adapter_options,\n\
    };\n\nuse crate::{\n    kani::abi::{i64_literal, readable_component, KaniSolver},\n    kani::generate::outcome::{DerivedDomain,\
    \ KaniObligationRequest, UnsupportedObligation},\n    kani::gene"
- id: src/kani/generate/mod.rs
  path: src/kani/generate/mod.rs
  role: examined
  excerpt: '//! The one Kani generator: the request and result vocabulary, the negotiation entry and the

    //! family lowerers.


    // Census validation; interim, see the file header.

    pub(crate) mod census_validation;

    // The V1 clause lowering the precondition and contract families share.

    pub(crate) mod clause;

    // Bounded collection claims over admitted V2 model fields.

    pub(crate) mod collection;

    // The contract family.

    pub(crate) mod contract;

    // The bounded-Kani corpus family.

    pub(crate) mod corpus;

    // The state-frame family.

    // Implements: FR-015 (IR-412: state-clause operation contract and frame effects).

    '
- id: tests/it/cover_last.rs
  path: tests/it/cover_last.rs
  role: examined
  excerpt: '//! FR-015-AC-7, AC-53 and AC-58: every harness the generator emits ends with exactly one

    //! `kani::cover!`, as the last statement of its body.

    //!

    //! The guard has two halves. The inspection drives every emitting entry point, parses each

    //! emitted source with `syn` and checks every function attributed `#[kani::proof]` or

    //! `#[kani::proof_for_contract]`. The scan reads the non-test string literals of `src/` and fails

    //! when a file outside the driven set spells either proof attribute, so a new emitter cannot be

    //! added without being driven here. The scalar family uses the private Harn'
bindings:
- test_id: src/kani/generate/spec.rs::tests::constructor_refuses_cover_call_and_statement_order_violations
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::constructor_checks_names_domains_messages_and_unwind_boundaries
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/spec.rs::tests::expression_validation_descends_into_matches_arguments_and_patterns
  ac_id: FR-015-AC-112
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-113
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-114
  trace: correct
- test_id: src/kani/generate/render.rs::tests::rendering_is_identical_in_another_process
  ac_id: FR-015-AC-114
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_refuses_size_before_invalid_syntax_with_exact_byte_count
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_port_is_differentially_identical_for_all_operations_and_boundaries
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/scalar.rs::tests::scalar_port_preserves_resource_and_syntax_refusals
  ac_id: FR-015-AC-115
  trace: correct
- test_id: src/kani/generate/render.rs::tests::renderer_emits_only_fixed_structure_and_supplied_data_in_order
  ac_id: FR-015-AC-116
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-116
  trace: correct
- test_id: src/kani/generate/render.rs::tests::unsatisfiable_assume_is_cover_unsatisfied_under_real_kani
  ac_id: FR-015-AC-117
  trace: correct
- test_id: tests/it/cover_last.rs::tc_025_every_emitted_harness_ends_with_exactly_one_cover
  ac_id: FR-015-AC-58
  trace: correct
```

## Dispositions

Round 1 independently checked frozen PR440 head `1240e7ac8457e2ec9d75e5a0f9853481003b89e7`, rebased on merged IR818 main `03f27b9b8715c517e88d769ed727bde88080ead8`. The original review finding text remains unchanged. No new defect was found in the fix delta.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Namespace detection now requires a following path separator or an actual turbofish colon2 token; bare/trailing kani and raw equivalents construct. Removing the coarse substring check also permits not_kani::any without weakening actual Kani namespace rejection. |
| FND-002 | fixed | identifier_identity uses syn IdentExt::unraw before namespace comparisons; spaced/commented/raw paths and turbofish forms are refused, recursively inside matches! expressions and patterns. Raw matches! spelling is treated as the same allowed macro and its arguments remain inspected. |
| FND-003 | fixed | bind stores normalized identifier identity; the own-input then-shadow comparison uses the same identity, once only. Input/Let/Call-result collisions across x/r#x refuse; original source and diagnostic spelling remains intact. |

Latest disposition verdict: all three findings fixed; no open review finding remains. Root's final aggregate and required full Kani gates remain pending on the final custodied candidate. The original review-pass verdict and findings above are historical and intentionally unedited.

### Round 1 evidence and scope

Static source review checked ExprText construction, namespace path punctuation and colon2 handling, raw identifier normalization, allowed-macro identity, nested expression/pattern traversal, all bind callers and the one permitted input-own shadow. The three new traced regression tests are expression_namespace_checks_use_identifier_identity_and_path_position, raw_identifiers_cannot_evade_duplicate_binding_checks and own_input_shadow_accepts_raw_alias_once_and_preserves_source_spelling (all in src/kani/generate/spec.rs). They assert both acceptance and exact ForbiddenExpression/DuplicateBinding refusals, both raw/plain directions, call-result collisions, then collisions, one permitted shadow and preservation of rendered spelling. The same AC112/AC116 bindings remain semantically correct. No source, branch or tracker finding text was changed by the reviewer. No Cargo, rustc, rustfmt or Kani ran during this disposition; no additional probe was necessary.

Actual coder logs are tied to this exact frozen fix head: fix1-lib-green.log passes 19 tests plus the second-process child (one private backend test intentionally ignored); fix1-cover-last.log passes 5/5 and still drives all twelve entrypoints; fix1-lint.log passes locked all-target clippy -D warnings; fix1-fmt-check.log and fix1-spec-validation.log pass. fix1-private-unsat.log explicitly runs the ignored private lib control and passes 1/1; its unchanged source asserts success_checks > 0 and exact CoverUnsatisfied { satisfied: 0, total: 1 }. The earlier red run caught not_kani::any's residual substring false positive; the final source removes that substring check and the green run exercises the acceptance regression. Prior author healthy/mutant/full-gate evidence remains tied to original 8337249; no repeated author full gate or aggregate is claimed for this fix head.

Delta reconciliation against 8337249 is recorded in disposition-1-delta-check.json. Every changed upstream-only file is byte-identical to merged 03f27b9. render.rs, generate/mod.rs, cover_last.rs and all three owning spec files are byte-identical to the original reviewed candidate. scalar.rs differs only by the two IR818 quantity_conversion_target: None fixture initializers. The only IR858 fix delta is spec.rs normalization/path validation and its regression cases. Quantity public claim fields and tests from IR818 are retained; no quantity Kani renderer was added. The other three clean review artifacts remain byte-unchanged, and the baseline gap audit is not repeated.

### Fixed text

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "        if path.segments.pairs().any(|pair| {\n            let segment = pair.value();\n     \
    \       identifier_identity(&segment.ident) == \"kani\"\n                && (pair.punct().is_some()\n      \
    \              || matches!(&segment.arguments, syn::PathArguments::AngleBracketed(arguments)\n             \
    \           if arguments.colon2_token.is_some()))\n        }) {\n            self.forbidden = true;\n      \
    \  }"
- fnd: FND-002
  outcome: fixed
  after_excerpt: "        if path.segments.pairs().any(|pair| {\n            let segment = pair.value();\n     \
    \       identifier_identity(&segment.ident) == \"kani\"\n                && (pair.punct().is_some()\n      \
    \              || matches!(&segment.arguments, syn::PathArguments::AngleBracketed(arguments)\n             \
    \           if arguments.colon2_token.is_some()))\n        }) {\n            self.forbidden = true;\n      \
    \  }"
- fnd: FND-003
  outcome: fixed
  after_excerpt: "fn bind(names: &mut BTreeSet<String>, name: &str) -> Result<String, HarnessSpecError> {\n    let\
    \ identity = identifier(name)?;\n    if !names.insert(identity.clone()) {\n        return Err(HarnessSpecError::DuplicateBinding(name.to_owned()));\n\
    \    }\n    Ok(identity)\n}"
```
