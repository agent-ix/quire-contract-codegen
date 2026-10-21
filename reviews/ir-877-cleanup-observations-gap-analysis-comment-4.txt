<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 4/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-055 | high | coverage | Inherited FR-015-AC-75 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-056 | high | coverage | Inherited FR-015-AC-76 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-057 | high | coverage | Inherited FR-015-AC-80 is tagged-by-ignored-test at the frozen candidate: every bound test is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-058 | high | coverage | Inherited FR-015-AC-84 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-059 | high | coverage | Inherited FR-015-AC-85 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-060 | high | coverage | Inherited FR-015-AC-87 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-061 | high | coverage | Inherited FR-015-AC-89 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-062 | high | coverage | Inherited FR-015-AC-90 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-063 | high | coverage | Inherited FR-015-AC-91 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-064 | high | coverage | Inherited FR-015-AC-92 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-065 | high | coverage | Inherited FR-015-AC-93 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-066 | high | coverage | Inherited FR-015-AC-95 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-067 | high | coverage | Inherited FR-015-AC-98 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-068 | high | coverage | Inherited FR-015-AC-105 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-069 | high | coverage | Inherited FR-015-AC-106 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-070 | high | coverage | Inherited FR-015-AC-107 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-071 | high | coverage | Inherited FR-015-AC-108 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-072 | high | coverage | Inherited FR-015-AC-110 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-015-AC-10
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and changing the unwind bound or the customer
    subject changes the harness identity.
- id: FR-015-AC-11
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every symbolic argument carries an inclusive assumption equal to its IR `bounded_domain` or
    a literal operand's own value (FR-015-AC-16); for a V2 scalar claim, an operand with the FR-014 inferred
    application interval uses that interval (FR-015-AC-103). A bounded-integer post-state result is required
    to lie in the same domain; no generated source carries a `#[kani::unwind]`.
- id: FR-015-AC-12
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request naming no items, more than 256 items, an unparsable subject path, or an unwind bound
    outside `1..=1024` is refused whole, with no item accounted and no harness exposed.
- id: FR-015-AC-13
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An otherwise-supported obligation whose generated harness source exceeds the bounded-resource
    ceiling is refused with a distinct resource-limit reason naming the generated size, and one that fits
    the ceiling but fails to parse as Rust is refused with a distinct syntax reason naming the parse error;
    neither is reported as the internal-invariant render-assembly fallback.
- id: FR-015-AC-14
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose graph node carries a tag/form pair this generator does not model
    as a contract role is refused with a typed reason naming that tag and form, and no harness is emitted;
    a claim naming a node id absent from the graph entirely is refused rather than accounted as supported
    with no contract role recorded.
- id: FR-015-AC-15
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields `unsupported` with the
    `no_derivable_claim` reason naming the node and the derivation refusal, emits no harness, and does
    not reject the group.
- id: FR-015-AC-16
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar harness constrains a literal operand, inline or a reference to a `value` node whose
    body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments
    `[0, 9]` and `[1, 1]`.'
- id: FR-015-AC-17
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is `unsupported` as `domain_not_representable_in_i64`
    naming the literal as both endpoints, with no harness.
- id: FR-015-AC-18
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar claim whose operation over its operand ranges yields no result inside the result
    range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable `[100, 109]`) is `unsupported` as
    `result_bound_unreachable` naming both ranges, with no harness.'
- id: FR-015-AC-19
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A zero-input transition over one bounded-integer state value verifies for an identity subject
    and is falsified with a concrete counterexample for a subject that changes the value.
- id: FR-015-AC-20
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a Boolean-connective or bounded-integer comparison claim (`and`, `or`, `not`,
    `implies`, `eq`, `ne` and the six integer comparisons) embeds its FR-014 oracle byte-identical to
    the oracle crate's function.
- id: FR-015-AC-21
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A precondition that reads a post-state value is refused with a typed reason naming the node,
    and no harness is emitted.
- id: FR-015-AC-22
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A declared proof-dependency census with an empty or duplicate identity, an inconsistent kind,
    state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness.
- id: FR-015-AC-23
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Every requested item receives exactly one `ObligationDisposition` (`supported`, `requires-bound`,
    `unsupported` or `invalid-request`), and an item that is not `supported` keeps its source identity
    and typed reason with no harness and no assumption emitted for it.
- id: FR-015-AC-24
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A falsifying in-domain assignment of a plain bounded-integer comparison claim is reported through
    Kani's concrete playback.
- id: FR-015-AC-25
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid declared census is folded into the harness identity, readiness is `ready` only when
    every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded
    `not_run`.
- id: FR-015-AC-26
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: One `postcondition` `state_clause` of an admitted package, whose condition is one integer comparison
    of the pre and post reads of one field through its first parameter `self`, yields a separate operation-contract
    harness and frame-effect harness for the operation its anchor names. Each has exactly one non-vacuity
    cover, and its identity is scoped to the operation, anchor, frame and framed object; the two harnesses
    of two clauses of one operation are written to different paths.
- id: FR-015-AC-27
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The operation-contract harness assumes each ranged state field within the inclusive `i64` endpoints
    returned by `model_object_fields` for the QSL-emitted selected model declaration (FR-015-AC-77). It
    draws a present non-clause field with no usable range without an assumption (FR-015-AC-78), asserts
    the clause's comparison between pre-state and post-state, and refuses a clause whose own field has
    no usable range. A non-model object or admitted unselected model/object_type body is refused rather
    than used as a range source (FR-015-AC-78). PLANNED (IR-624).
- id: FR-015-AC-28
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-effect harness takes the granted fields from the frame node's `modifies` entries
    and asserts every other state field the caller names unchanged.
- id: FR-015-AC-29
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A state clause that is not a postcondition, whose condition is not one comparison of the pre
    and post reads of one field through `self` (a literal operand, a negation, two fields, two reads of
    one side, or a read through another parameter), whose frame creates, deletes or grants a relationship
    or a foreign field, whose frame grants every state field, whose field the caller's state lacks, or
    whose clause field declares no integer range, is refused with a typed reason and no harness; a malformed
    request is refused first.
- id: FR-015-AC-30
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, the operation contract verifies for a healthy subject and is falsified,
    naming the postcondition, for a subject mutated to debit.
- id: FR-015-AC-31
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a frame-allowed effect verifies, a frame-forbidden effect is falsified
    naming the forbidden field, and regenerating the frame from a package whose `modifies` is mutated
    to grant nothing falsifies the allowed subject.
- id: FR-015-AC-32
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The forbidden frame counterexample, executed natively, reproduces as a frame violation of that
    field through QSL's `replay_frame`, and the allowed run replays as a respected frame.
- id: FR-015-AC-33
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay payload's anchor, frame and frame occurrence are the values `qsl_replay::call_site`
    names for the operation's name.
- id: FR-015-AC-34
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The frame-replay envelope's `clause_node` is the payload's frame node and its `occurrence_key`
    is the payload's frame occurrence.
- id: FR-015-AC-35
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: An operation the unit names no frame for, whether the domain package declares it or not, is
    refused by the call site when the frame-replay request is built.
- id: FR-015-AC-36
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`FrameReplay::replay` returns QSL''s `replay_frame` result: a forbidden write settles a reproduced
    violation naming the written field, and a write the frame grants settles `inconclusive` with no frame
    witness.'
- id: FR-015-AC-37
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A scalar function-application harness asserts the oracle''s outcome against the clause''s
    operation evaluated natively in `i128` over the same symbolic operands, independently of the oracle:
    `Completed` with exactly that value when it lies in the result bound, `Refused` when it does not,
    and any other outcome fails. With the installed backend, the healthy oracle verifies and an oracle
    whose arithmetic is mutated is falsified on that assertion.'
- id: FR-015-AC-38
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2` whose
    `operation.member.clause` is `precondition`, `postcondition` or `invariant` yields one harness of
    that obligation kind, separate from the harness of every other clause of the package; the kind is
    read from the node and no caller value changes it. PLANNED (IR-489).
- id: FR-015-AC-39
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause harness draws one nondeterministic value per drawn input: the pre-state of each
    state field of `self` the body reads, whether bare or under `pre(...)` (the subject''s input state;
    how it reaches the subject is FR-025''s), and each operation parameter (for a precondition or invariant,
    each bare state field read, since the state is the one judged). Each carries an inclusive assumption
    equal to its declared domain: the accessor''s `IntRange` values for a selected model declaration state
    field (FR-015-AC-77), and the IR `bounded_domain` of a parameter''s type. A non-model or admitted
    unsel'
- id: FR-015-AC-40
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A harness for a V2 clause whose condition, inline or referenced from an admitted Boolean expression
    node, contains Boolean connectives and bounded-integer comparisons (`and`, `or`, `not`, `implies`,
    `eq`, `ne` and the six integer comparisons) embeds an FR-014 oracle of that condition byte-identical
    to the oracle crate's clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified
    oracle needs FR-014-AC-38. PLANNED (IR-489).
- id: FR-015-AC-41
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply, negate) as an operand
    of a comparison lowers, and its harness keeps the independent native assertion of FR-015-AC-37 for
    each arithmetic subterm (the oracle''s outcome against the operation evaluated natively in `i128`
    over the same operands, `Refused` assumed away nowhere): a postcondition `amount < 1000 implies amount
    + 1 <= 1000` over `amount: Int[0, 1000]` yields one contract harness. With the installed backend the
    unmutated clause verifies with its cover satisfied, and the same clause with `+` mutated to return
    `left'
- id: FR-015-AC-42
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 clause body holding an operator the V2 arm does not support (division, modulo, absolute
    value, and every operator outside connectives, the six integer comparisons and add, subtract, multiply,
    negate; those operators are planned scope for a later criterion) is `unsupported`, with a typed reason
    naming the node and the operator and no harness; a node that is not a modelled clause role, or a node
    id absent from the package, is refused as FR-015-AC-14 states; no item is skipped silently, and a
    supported item of the same request is unaffected. PLANNED (IR-489).
- id: FR-015-AC-43
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 postcondition harness emits every `precondition` clause node sharing its anchor node as
    a `requires` and records each in its identity's embedded oracles, embeds no other obligation's oracle,
    and a postcondition whose sibling precondition is not a supported item of the same request is refused
    naming that precondition with no harness; a V2 invariant is anchored at an object type, binds `self`
    alone, and so has no operation precondition to assume. PLANNED (IR-489).
- id: FR-015-AC-44
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A census declared on a V2 clause claim with an empty or duplicate dependency identity, an inconsistent
    kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with
    no harness, by the same rules as FR-015-AC-22; the census request and readiness types it uses are
    not `ProofDependencyGraph`. PLANNED (IR-489).
- id: FR-015-AC-45
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A valid census declared on a V2 clause claim is folded into that harness's identity record,
    readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed,
    and proof execution is recorded `not_run`, by the same rules as FR-015-AC-25; in addition, the folded
    census is independent of the order the caller lists its dependencies. PLANNED (IR-489).
- id: FR-015-AC-46
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Every V2 clause harness contains exactly one non-vacuity cover, placed after every assumption,
    after the subject call and after every assertion the harness carries (the last statement, as FR-015-AC-7
    states), that states the family''s reachability, as FR-015-AC-7 does: a precondition harness covers
    that the precondition holds within the bounds; a postcondition harness covers, after the subject call
    returns, that the requires and the bounds are jointly satisfiable; an invariant harness covers that
    the drawn state satisfies its domain assumptions. The subject call is the call of the customer subj'
- id: FR-015-AC-47
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind
    bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). PLANNED
    (IR-489).
- id: FR-015-AC-48
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: The obligation identity of a V2 clause claim is formed by AD-003 E-1 from the clause node id,
    its occurrence key, the obligation kind, and the arguments each as parameter node id and declared
    domain for an operation parameter, as E-1 states (the domain FR-015-AC-39 defines). A state field
    read is named by its declaring node id and field name, never by a node of its own; that E-1 lists
    parameters only is an open question for the AD-003 owner and E-1 is not widened here. The source span
    is excluded; changing any included member changes it, and changing the span, the unwind bound or the
    subject d
- id: FR-015-AC-49
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V2 invariant claim yields a harness that draws its state and parameters within their declared
    domains (FR-015-AC-39) and asserts the invariant clause; its subject is the clause itself, with no
    subject call. Preservation of the invariant under an operation is not specified by this criterion.
    PLANNED (IR-489).
- id: FR-015-AC-50
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: '`negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded`
    or `LoweringLimitUnrecognised` as `Outcome::Unsupported(UnsupportedObligation::OracleRefused { refusal
    })` with `refusal` equal to the scalar refusal, field for field, and neither is mapped to another
    `UnsupportedObligation` variant nor to `LoweringWorkExhausted`.'
- id: FR-015-AC-51
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, a finite input validated under a profile selection other than the offered profile's is refused
    as a typed `InvalidInput` `kani_profile_input_mismatch` result naming the request's source id, with
    no artifact and no case identity recorded.
- id: FR-015-AC-52
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, the context of every outcome and refusal that path returns (generated-case `Proved` or `Counterexample`,
    `kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`)
    is the profile selection's revision. The generated-case values are not Kani run verdicts.
- id: FR-015-AC-53
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Each of the seven harness kinds the generator emits (precondition, V1 contract, scalar, state-clause,
    frame-effect, V1 bundle and corpus) is subject to the cover-last rule of FR-015-AC-7: exactly one
    `kani::cover!`, the last statement of the body. What the cover witnesses, by kind: a precondition
    harness, that the precondition holds within the IR bounds (it asserts nothing); a V1 contract harness
    (postcondition or invariant), that the requires clause and the IR bounds are jointly satisfiable,
    after the contract call whose `ensures` is the harness''s check; a scalar harness, that the oracle''s
    `C'
- id: FR-015-AC-54
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract` harness it emits for
    a requires clause, an ensures clause and the bundle's bounded arguments) ends with exactly one `kani::cover!`
    after the call of its contract, which witnesses that the requires clause and the argument and result
    bounds are jointly satisfiable, so a run whose requires clause no bounded argument satisfies never
    reaches it; the bundle's `ensures` is checked at the contract call, so the cover follows every check
    the harness carries.
- id: FR-015-AC-55
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic, FR-015-AC-110
    for collection, FR-015-AC-111 for graph), a bounded-corpus harness of that family ends with exactly
    one `kani::cover!` after its `assert!` of the case's oracle, which witnesses that the harness runs
    to its end past that assertion.
- id: FR-015-AC-56
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: With the installed backend, a V1 bundle harness whose requires clause some bounded argument
    satisfies and whose `ensures` holds for every such argument classifies `Verified` (FR-017-AC-4); the
    same bundle with a requires clause no bounded argument satisfies classifies `CoverUnsatisfied` with
    its satisfied and total cover counts, never `Verified` and never `Falsified`.
- id: FR-015-AC-57
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed backend, a bounded-corpus
    harness of each of the arithmetic, graph and collection families whose oracle is true classifies `Verified`,
    and a graph or collection harness whose oracle is false (an arithmetic case's oracle is always true)
    classifies `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes the corpus
    replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`),
    never `Inconclusive` for lack of a counterexample.
- id: FR-015-AC-58
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A test generates a harness through every emitting entry point (the precondition, V1 contract,
    scalar, state-clause and frame-effect families, `generate_kani_bundle`, and the corpus generator for
    each of its families whose arm remains, per FR-015-AC-109 to FR-015-AC-111), parses each emitted source,
    and fails for any function attributed `#[kani::proof]` or `#[kani::proof_for_contract]` whose body
    does not contain exactly one `kani::cover!`, which must be the last statement, with no assertion after
    it and no other cover; and a scan of the non-test string literals of `src/` fails when a file othe
- id: FR-015-AC-59
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: A request of `StateFrame` items in which the first item is refused and later items are supported,
    requires-bound, unsupported or invalid yields exactly one `ObligationRecord` per item in request order,
    `kind` `postcondition` for a `contract` item and `frame` for a `frame` item; and each later item's
    record (disposition and reason) equals the record the same item has in a request holding that item
    only; a request of three refused items yields three records. The requires-bound item comes from a
    lowering `RequiresBound` record in that same admitted package, not an object-body member target; remov
findings:
- fnd: FND-055
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-75
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '872'
  excerpt: Mutate only the production oracle or only the shadow; or accept an equivalent mutant as the
    test of a check (a permutation of member order, or swapping two descriptors of one type).
  finding: 'Inherited FR-015-AC-75 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-056
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-76
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '873'
  excerpt: Record `backed` for a covering criterion the matrix lists planned, omit `declaration_reconstruction`,
    or leave the identity unchanged when a bound changes.
  finding: 'Inherited FR-015-AC-76 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-057
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-80
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '882'
  excerpt: Generate the harness from the hand-built fixture rather than the emitted package.
  finding: 'Inherited FR-015-AC-80 is tagged-by-ignored-test at the frozen candidate: every bound test
    is ignored. Existing IR-878/program ownership remains; this diagnostic slice does not implement or
    qualify that obligation.'
- fnd: FND-058
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-84
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '820'
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, a profile/dispatch-admitted
    add, subtract, multiply, divide or remainder returns the exact checked `i128` result only inside the
    request's inclusive result range.
  finding: 'Inherited FR-015-AC-84 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-059
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-85
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '821'
  excerpt: Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, an
    inverted result range is `InvalidInput`; zero division, checked overflow and a result outside the
    range are typed refusals, with no arithmetic result or generated artifact.
  finding: 'Inherited FR-015-AC-85 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-060
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-87
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '823'
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a zero expansion bound
    or unknown endpoint is `InvalidInput`, and exhausting the bound is `ResourceExhausted`, with no graph
    result or generated artifact.
  finding: 'Inherited FR-015-AC-87 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-061
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-89
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '825'
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, an input
    longer than `max_items` is `ResourceExhausted`, with no query result or generated artifact.
  finding: 'Inherited FR-015-AC-89 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-062
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-90
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '826'
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic, graph or collection case emits all four
    artifacts (oracle, finite strategy, Kani harness and proof-dependency graph) from the same selected
    profile, validated finite input and family lowering.
  finding: 'Inherited FR-015-AC-90 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-063
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-91
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '827'
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, that path returns a generation-time case classification of `Proved` when the family lowering's
    Boolean oracle is true and `Counterexample` when false; neither value is an observed Kani run or native
    replay verdict. This interim classification ends when FR-015-AC-111 deletes the path.
  finding: 'Inherited FR-015-AC-91 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-064
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-92
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '828'
  excerpt: Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice,
    FR-015-AC-109 to FR-015-AC-111), a declared dependency census with an empty or repeated identity,
    inconsistent kind/state/path or a kind other than `Required` refuses as typed `InvalidInput` with
    no artifact or case-identity claim; a valid census is normalized into the emitted proof graph.
  finding: 'Inherited FR-015-AC-92 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-065
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-93
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '829'
  excerpt: Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus
    path, a profile/input mismatch, family-lowering refusal, proof-graph serialization refusal or duplicate
    case identity returns its typed refusal with the selected profile revision, emits no artifact and
    claims no new case identity.
  finding: 'Inherited FR-015-AC-93 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-066
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-95
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '831'
  excerpt: 'AD-004 step 4g retires the hand-built bounded-corpus path in the three per-family slices FR-015-AC-109
    (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111 (graph, which deletes the whole public
    corpus API and the generation-time `KaniOutcome`); this criterion is the end state they reach together
    and adds no deletion of its own. The end state is: QSL-emitted packages admitted through Contract
    IR feed the one V2 generator for arithmetic, collection and graph cases; the collection and graph
    cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived symbolic bounds, and each'
  finding: 'Inherited FR-015-AC-95 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-067
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-98
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '834'
  excerpt: A graph claim (one reachability application of either form above, at its `state_clause` node
    id and argument path) generates a harness when the admitted package gives its covering object type
    `E` a finite population `[0, N]` through `select_population` and the selected edge has finite multiplicity
    (for `quire.op.model.reaches_field` the field's `Reference`, `Option` or bounded `Collection` member;
    for `quire.op.model.reaches` the destination end's finite multiplicity). Every symbolic object and
    edge is bounded inclusively by those declarations, including zero and maximum where admitted. Reacha
  finding: 'Inherited FR-015-AC-98 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-068
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-105
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '841'
  excerpt: 'For a graph claim the harness draws an object count in `[0, N]` of the one covering population
    and symbolic objects within it; for every object, the selected edge as its shape states: a `Reference`
    field has exactly one target, an `Option` field an absent or one target, a `Collection` field or finite
    multi-valued relationship end a count in `[lower, upper]` (a unique end admits no repeated target,
    a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at least `lower` targets) and every
    target is an object of the population. Start and target are drawn from that population and may b'
  finding: 'Inherited FR-015-AC-105 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-069
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-106
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '842'
  excerpt: 'The independent native oracle replays the same authored clause over each original-domain concrete
    graph through QSL''s replay (`qsl_replay`), and does not substitute a separately authored reachability.
    With the installed backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover
    satisfied for one field-edge claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}`
    non-unique end. Each semantic mutant yields `Falsified` at the clause assertion with an original-domain
    playback and native replay of that same authored clause: reachability that allows a zero-len'
  finding: 'Inherited FR-015-AC-106 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-070
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-107
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '843'
  excerpt: 'Graph claims are located by `state_clause` node id and argument path: one reachability written
    twice in one condition yields two items at positions 0 and 1; one node under two clauses yields two
    items with different identities; a field-edge and a relationship-end reachability yield separate items;
    a collection claim, a clause with no reachability and a path that does not name a reachability application
    each settle `unsupported` naming the missing claim with no harness. A mixed request settles one disposition
    per item and a supported item''s harness is unchanged by a refused sibling. PLANNED (IR'
  finding: 'Inherited FR-015-AC-107 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-071
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-108
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '844'
  excerpt: A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition
    sequence for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute.
    While the population selection for the claim is `Unavailable` (no available population selection for
    this admitted claim), the claim settles `unsupported` naming it. Given a selection, `Unbounded` or
    `UnknownClosureOpenExtent`, that is a claim whose population has no finite authored bound (once a
    selection is available), settles `requires_bound` naming the population; an unbounded `Collection`
    edg
  finding: 'Inherited FR-015-AC-108 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-072
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-015-AC-110
  related_ids: []
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  lines: '846'
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and on AD-004 step 4h, because
    the collection family renders through `HarnessSpec` and `render.rs` only after 4h). The gate opens
    when the AC-99 installed-backend controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice
    PR's head that is green
  finding: 'Inherited FR-015-AC-110 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 4
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++