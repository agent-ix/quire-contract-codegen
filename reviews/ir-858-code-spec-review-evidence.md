---
id: SR-12913
title: IR858 spec-review/evidence
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; IR858 PR440; AD004 step4b, private generate
  spec/render/scalar and owning status/evidence delta
review_set: subset
---

## Summary

Ticket: IR-858. Frozen PR440 review; Reviewed the complete assigned code/spec delta and its owning obligations.

## Verdict

PASS for this evidence-documentation delta. Focused/differential checks and installed-backend smoke are correctly distinguished; the explicit private ignored-lib invocation is documented separately from make kani-gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

quoin advise --json and quoin catalog methods --json were run, not recalled. AC112..117 are authored Test; all six advisor records report mismatch=false and inconclusive=false. AC112 matches parser/invariant/temporal/memory-safety rules offering design-by-contract, property/fuzz, monitoring and analysis options; these are catalog suggestions, not proof that a sanitizer or model-checking lane ran. AC113..117 match example rules recommending unit-testing and bdd-spec-by-example. Judgement: existing unit differential/boundary behavior tests plus the specified real installed-backend controls fit these obligations. No requirement/method column changed in this PR. The matrix index's Analysis category is unchanged metadata, not substituted for the ACs' Test method. Source, oracles and actual author execution logs were read to verify the new status prose, with the constructor defects remaining subject to SR12910. Independent acceptance remains pending in the changed index; no backend result is used as proof of byte equivalence. No future evidence is represented as already measured.
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
bindings: []
```
