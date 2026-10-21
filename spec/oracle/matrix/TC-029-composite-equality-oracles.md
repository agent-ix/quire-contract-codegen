---
id: TC-029
title: "Verify composite equality oracle generation and agreement with direct runtime execution"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/TC-026
    type: references
---
# TC-029: Verify composite equality oracle generation and agreement with direct runtime execution

## Description

Verify that composite equality oracles generated from an admitted
CheckedPackage V2 cover every admitted composite and collection shape, refuse
every non-generated item with its own typed reason, are byte-deterministic, and
agree — outcome, admitted charges and consumed counters — with direct runtime
execution. Agreement with the QSL value authority is outside CG's boundary and
not claimed by CG (FR-014-AC-47); it is owned by quire-integration IR-842.

The discriminating evidence is the agreement of step 4. No generated output is
committed: step 4 generates the corpus crate at test time and executes it
against independently constructed native runs whose descriptor comes from the
request, so a generator change to an emitted operator, operand order or
descriptor fails step 4. Determinism is checked by regeneration in step 2.

The FR-018 admitted-context and direct-owner procedure amendments are
PLANNED/UNRUN until CG implements them and qualifies the generated ABI.
The existing full record/tuple/recursive/collection corpus remains required;
the enum-only owner facade does not excuse dropping it.

## Test Procedure

1. Build an admitted V2 package holding `binary` expression nodes over: record,
   tuple and option types; `sequence`, `set`, `bag` and `ordered_set` collection
   types with their `collection_bounds` domains; nested composites inside
   collections and collections inside records; scalar leaves of every equality
   schedule (text, enum, quantity, and the `Plan` types); admitted `convert<T>`
   operands from each `admits_equality_conversion` row exercised; plus operands
   with distinct text profiles, distinct enum declarations, incompatible
   dimensions, distinct units, no common type, a `convert<T>` outside the table,
   a record with a `float64` field nested inside a `sequence`, a `reference`
   composite form, model, relation, function, `call`, state, temporal and
   protocol nodes, both recursion-rule cycles, an
   unlowered node, a non-`binary` form, a descriptor disagreeing with its
   operand types, one node id repeated under one descriptor, and a request that
   exceeds the lowering work limit.
   Exercise a duplicate record field in authentic source as a separate QSL
   admission control through root `compile_admitted`, with ordinary source and
   dependency provision. Retain `AdmittedRefusal::Package` and its real
   `CompileRefusal` payload and available locus. Through the root facade's
   public accessors assert `code() == Code::AmbiguousDeclaration` and
   `cause() == None` for this Assembly refusal. Do not infer or synthesize an
   `ambiguous-name` public cause from the internal assembly diagnosis, add a
   dependency on QSL assembly internals, or require a new QSL API. The precise
   duplicate name/cause remains asserted in the separate private
   declaration-defense control below.
   No successful `AdmittedContext` or CG artifact is produced; do not attempt
   public CG generation with a fabricated or mismatched context for that source.
   Retain private CG declaration-admission checks of the precise
   `DuplicateMember` cause/name and healthy-sibling isolation separately,
   without claiming public malformed-declaration acceptance. This specific
   allocation does not classify the unrelated recursion controls.

2. Generate twice and with a permuted request that includes both descriptors of
   step 7 over one node id; compare bytes with each other, and inspect claim-map ordering by the descriptor key — expression node
   id, operator rank, then each operand's source-type and conversion-target node
   ids, every node id compared in node-id order; the expression node
   id alone ties those two entries — reconstructed declaration keys against
   the genuine owner-issued keys corresponding to exact V2 node identities,
   without any backend key construction, and `caller_declared` provenance.
   Assert each entry's recorded descriptor is equal to the one the request
   supplied — operator, both operand source types, both conversion targets — and
   its recorded `EqualitySchedule` equal to `CheckedEquality::schedule()` of the
   `CheckedEquality` that descriptor admits, both compared against values the
   test holds, never against values parsed out of the generated source.
3. Inspect each refusal: its typed cause, that the item's symbols are absent
   from the generated source, that its siblings are unchanged, and that the
   three upstream blockers (quire-spec-language#120, quire-contract-runtime#34,
   quire-spec-language#121) are distinct values rather than one reason. Assert
   the generated source contains no `unwrap`, `expect`, panicking index or
   charge or pair-count literal, and that the crate manifest declares
   `publish = false` and the runtime dependency (git source, `branch = "main"`, no pinned `rev`)
   with the `exact` feature only for Runtime residue; direct exact/QSV and
   equality's root replay dependency come from the same profile authority.
4. Generate the corpus crate at test time, compile it with the agreement cases
   as its integration test, and execute it on the corpus vectors. For each vector compare the `Outcome<bool>`, the admitted
   charge sequence and the consumed counters against a direct call to
   `TypeEnvironment::check_equality` and `CheckedEquality::evaluate` on an
   environment, operands **and descriptor** constructed in the test from the
   request, not by or from the generator. No comparison with the QSL value
   authority runs here (FR-014-AC-47). The descriptor
   is the thing an emitter mutation changes, so reading it back out of the
   generated crate would make the comparison follow the mutation and be
   vacuous; it is held by the test.
5. Re-execute every vector with a denial injected at each of
   `equality.plan-form`, `equality.plan`, `equality.pair`,
   `equality.result-retain` and each conversion charge point in turn; confirm
   `Outcome::Incomplete` naming that point and that the denied charge was not
   applied — every counter equal to those of the same run stopped immediately
   before that point, not to the counters at entry.
6. Call each generated oracle whose operand types reach a declaration key with
   a genuine prepared wrapper whose environment is privately replaced by the
   test fixture (no public raw wrapper/index constructor): an
   empty environment, and one omitting a key its operand types reach, and one
   declaring that closure under a different key; confirm `Outcome::Refused(Refusal::CheckedInvariant)`, no panic, and no charge
   admitted on the `Meter`. Assert that this is the emitted `check_type` calls
   deciding it, by confirming the same environments pass `check_equality`: that
   call uses the same genuine enum index and consults the environment through `contains_ieee`, which returns
   `false` for a composite key the environment does not hold, and
   `CheckedEquality::evaluate` takes no environment at all — so without
   `check_type` these vectors complete with a Boolean.
7. Request one node twice in one request under descriptors differing only in
   `EqualityOperator`; confirm both generate under distinct readable symbols,
   each built from its own operator's stem, that both are
   marked `caller_declared`, and
   that their outcomes are complementary on a vector whose operands differ. This
   is not the duplicate case of step 1: a duplicate is one node id under one
   descriptor.
8. Generate an equality over a tuple whose member types are `bounded_domain`
   nodes (FR-018-AC-16): the text position names a `text_bounds` node, and the
   generated declaration is `Text(0, 16, Nfc)`; integer, decimal and rational
   `bounded_domain` members generate source identical to members naming their
   base scalars; with a second `text_bounds` node over the same text scalar,
   naming either reads that node's own bounds. Then name a `text_bounds` node
   over an integer scalar (refused as missing `integer_range`), and
   `bounded_domain` nodes over a boolean scalar, over the base package's enum
   declaration and over a record (each refused as unsupported `bounded_domain`).
   Name a `float_rounding` node over a `float64` scalar as a member: it reads as the
   float and the equality is refused as `OperatorIneligible`.

   For FR-018-AC-14, separate bound reconstruction from intake and defensive
   controls. First read valid `integer_range` and `collection_bounds` nodes
   whose bodies contain exactly `min` then `max`, each a canonical integer
   literal typed at the owning Integer node. Through normal lowering and
   public generation, verify the exact original bound values and domains;
   do not widen, reorder or substitute them. Keep the existing missing-bound
   request/root-selection controls unchanged.

   At Contract IR intake, independently reorder these bindings, replace one
   with a bare literal, remove one, repeat one, and introduce an unlisted name.
   Use an ungrouped bound node in otherwise valid wire. Refresh the
   package-level canonical identity and matching identity-projection mirrors
   while retaining the offending node key, so earlier identity checks do not
   mask the derived closed-shape refusal. Do not ask a candidate-key helper
   to manufacture a key for an invalid closed shape.
   Require its `invalid_package` / `stale-node-key` refusal at the offending
   node's `node_id`, with no admitted package. These rejected inputs never
   call public CG generation and have no CG `UnreadableBound` disposition.
   Do not attach an unrelated genuine context to claim a later refusal.

   Retain private CG bound-parser/type-resolution tests with the same precise
   malformed-bound assertions as defense against inconsistent internal input;
   label them separately from the public valid-bound and IR-intake controls.
   A private defense does not supply public-generator acceptance credit for
   an input the reader rejects. This stage split applies only to
   `integer_range` and `collection_bounds`. For `rational_range`,
   `decimal_range` and `text_bounds`, preserve named lookup in any admitted
   order and each genuinely reader-admitted unreadable-bound negative; do
   not infer their admission or reader refusal from the two closed forms.

9. Emitted-source scan (FR-018-AC-17). Read the `src/lib.rs` that
   `generate_composite_equality_oracles` returns for the step 1 package and assert
   zero occurrences of the panic tokens of step 11, no `.ok()`, `.unwrap_or(` or
   `.unwrap_or_default(`, and no `[` index or slice expression directly after an
   identifier character, `)` or `]`. Assert every reconstruction helper returns
   `Result<_, ReconstructionError>`, every call of one is followed by `?` or a
   `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or
   `Outcome::Refused(Refusal::CheckedInvariant)`, and `EnvironmentError` has the
   variants `Declaration(InvalidDeclaration)` and `Reconstruction(ReconstructionError)`,
   `ReconstructionError` has the unit variants `Integer`, `Interval`, `Rational`,
   `Decimal`, `Text` and `Cardinality`, and no call of `IntegerInterval::new`,
   `RationalDomain::new`, `TextType::new`, `CardinalityBound::new`,
   `DecimalType::new` or an integer `.parse()` appears outside a reconstruction
   helper. Assert each of `rebuild_integer`, `rebuild_interval`, `rebuild_rational`,
   `rebuild_decimal`, `rebuild_text` and `rebuild_cardinality` fails with exactly
   its own `ReconstructionError` variant through one `map_err`, with no other
   variant and no `or_else`, `or`, `ok`, `unwrap_or`, `map_or` or `match`, and show
   by hand-written snippets (a wrong variant, a silent widen, a swallowed failure)
   that the check names each departure. No valid request makes a reconstruction fail, so the failure path is verified
   structurally here and not by execution.
10. Render unit test (FR-018-AC-18). In the `#[cfg(test)]` tests of
    `src/oracle/equality/mod.rs`, call `render_value_type` with
    `ValueType::Quantity` and with `ValueType::Reference` and assert each returns
    `Err(RenderError::UnsupportedValueType { family })` with `family`
    `"quantity"` and `"reference"` respectively. Then feed a `RenderError` through
    the item-boundary mapping and assert it yields
    `CompositeEqualityRefusal::Unsupported` with `node_tag` equal to `family` and
    `unsupported_node_id` equal to the item's expression node id (for an operand
    type and for a composite declaration alike), with the
    item's siblings unchanged. Feed a `RenderError::Generation` carrying
    `UnknownRuntimeVariant` through the same mapping and assert the whole call
    fails with it. A public-API request cannot reach either arm: a
    quantity operand is refused earlier as `CompositeEqualityRefusal::Unsupported`
    and a reference operand by Contract IR, which AC-7 already covers.
11. Generator-source scan (FR-018-AC-19). Read the non-test text of
    `src/oracle/equality/mod.rs` (comments and every `#[cfg(test)]` item removed
    wherever the item sits, string literals kept), assert the scan removed the test
    module and nothing before it, and assert zero panic tokens as FR-018-AC-19
    defines them: `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`,
    `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however
    written (including `Option::unwrap` and `.unwrap ()`), the macros `panic`,
    `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`,
    `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form,
    and `abort` anywhere but as a method call. The scan is shown to name each spelling by
    the TC-024 panic-scan snippet test, which shares it.
12. Lowering byte-ceiling failure (FR-018-AC-20, IR-547). Read a package whose lowered contract
    package for the call is longer than its checked package under a byte ceiling that admits the
    checked package and is one byte below the canonical length of that lowered contract package,
    and request every node: every item is refused `LoweringByteLimitExceeded` with `limit` equal to
    that ceiling and one shared `consumed`, none `LoweringWorkExhausted`, and none generates. In
    the equality module's `#[cfg(test)]` seam, hand its `Failed` arm hand-built records: `bytes`
    gives `LoweringByteLimitExceeded` with the record's `limit` and `consumed` (the per-node case of Contract IR FR-038-AC-95, not
    reachable through the public API); `work` gives `LoweringWorkExhausted`; each of
    `nodes`, `edges`, `occurrences` and `diagnostics` gives `LoweringLimitUnrecognised` with that
    snake_case `limit_kind`, with no panic and never `LoweringWorkExhausted`.
13. Declaration closure (FR-018-AC-21, planned, IR-264). For every item of the step 1 package whose
    leaves are Boolean or bounded integers, read the claim-map entry's closure (each declaration's
    key, form, members in declaration order with name, presence and type, each leaf's family and
    declared bound) and compare it member by member with the declarations of the
    `TypeEnvironment` the item's generated environment constructor returns, held by the test and
    not parsed from the source.
14. Totality and schedule over the refinement domain (FR-018-AC-22, planned, IR-264). For the
    record, tuple and option items of step 13, whose declared integer ranges are chosen so the
    domain fits the case cap (both Booleans, an integer range of three values, every presence
    state of an option and of an optional field), enumerate every operand pair of that domain, the
    whole of it, and call each generated oracle with its own environment and a `Meter` whose
    limits are all `u64::MAX`: read `Completed` with the QSpec FR-149 verdict computed by the test
    from the values, the admitted charges `equality.plan-form`, `equality.plan`, one
    `equality.pair` per node of the occurrence-pair tree counted by the test, then
    `equality.result-retain`, and no `Refused` or `Incomplete`.
15. Independent closure (FR-018-AC-23, planned, IR-264). For each item of step 13, read the
    package's declared types through Contract IR's reader, not through the generator, and compare
    member names, presence and each integer leaf's inclusive bounds with the claim-map closure;
    then hand the check a closure that dropped a member, one whose bound was narrowed by one, and
    one that read an optional member as required, and read each fail.
16. Recursive records and authoritative shape reads (FR-018-AC-24, AC-26 and AC-27, planned,
    IR-643). Read the authoritative QSpec
    `proposals/checked-package-v2/fixtures/positive-recursive-records.json` through Contract IR's
    strict reader, without copying it into this repository. Build equality expression items over
    the fixture's List (`Option<List>`) and Tree (`Sequence<Tree>[0,3]`) record declarations in a
    valid test package; the fixture itself supplies type nodes, not equality expressions. Assert
    both items generate and the call terminates. Independently inspect the exact fixture's
    List `next` binding: its value is an aggregate with one `optional` binding to the Option node,
    whose sole payload reference returns to List. Assert the generated declaration has one
    `next` field with `Presence::Optional` and the List `NodeKey`. Inspect Tree `kids`: its
    direct reference targets `collection_bounds`, whose `semantic_type` names the Sequence node;
    that sequence body has only the Tree element reference. Assert the generated `kids` type
    uses the Tree `NodeKey` and `[0,3]` read from the bound node's named bindings. At the
    equality module's type-resolution seam, make separate synthetic shape mutants without
    altering or copying the fixture: remove or rename the nested `optional` binding, add a second
    nested member, redirect it to a non-option node, make the bound's `semantic_type`
    non-sequence, remove or
    duplicate `min` or `max`, and make either bound non-integer; each affected item receives its
    typed malformed or bound refusal with no symbol while an unaffected sibling generates.
    The four malformed optional-wrapper mutations above are private defense
    controls: preserve exact `MalformedComposite`, no generated symbol, no
    silent reinterpretation as required, and the healthy Tree sibling. They
    are not a claim that the checked source emitter can produce those
    inconsistent wrappers or that they satisfy public genuine-context
    preconditions. The owning emitter derives the one valid wrapper from
    admitted `Presence::Optional`; keep the genuine public List/Tree positive
    controls, original key/source/context binding and field assertions intact.
    For any additional malformed source/wire intake control, assert its real
    owning refusal under that control's prerequisites, before CG; do not invent
    a common intake diagnostic or a source spelling for every internal mutation.
    These intake controls do not discharge the private parser assertions.
    The `collection_bounds` malformed-name/order/shape controls at this private
    seam are defensive parser evidence, not public-generator acceptance;
    step 8 owns their separate IR-intake refusal and valid-bound controls.
    The optional-wrapper defenses have their separate allocation above;
    this change does not reclassify the cycle controls.
    At the equality module's type-resolution seam, supply separate synthetic graphs with `Option`
    referring to itself and `Sequence` referring to itself, so earlier package admission cannot
    mask the resolver behavior. Assert each returns `TypeResolutionCycle` with the repeated node
    id, emits no symbol, and leaves a healthy sibling generated; run these cases in a subprocess
    so a stack overflow or abort is observed as a failed test rather than killing the test runner.
17. Type-resolution work (FR-018-AC-25, planned, IR-643). At that same seam, use an acyclic
    walk whose roots and references across both operands and every conversion target total
    exactly 65,537 entries, with a valid terminating leaf and a healthy sibling. Assert
    the first item refuses `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }`,
    emits no symbol and leaves the sibling generated; a separate walk totaling exactly 65,536
    per-item entries across every operand and target does not receive the work refusal. Count
    root entries and followed member, option and collection references and bounded-domain
    `semantic_type` edges, including both operands and conversion targets and a repeated
    reference to a previously resolved node, in a short mixed-shape case against a counter
    observed at the seam. Confirm the over-limit case uses the type-resolution refusal, never
    `LoweringWorkExhausted`. Run the long chain in a subprocess to detect stack exhaustion.
    The seam supplies a synthetic lowered graph,
    so Contract IR's separate lowering ceiling does not preempt this check.
18. Deep type walk (FR-018-AC-28 and AC-29, planned, IR-511). At the same
    synthetic lowered-graph seam, construct an admitted 100,000-entry acyclic
    chain and an equivalent shallow declaration closure. Run resolution in a
    subprocess on a 512 KiB stack with `cg.equality.source_bytes` raised to
    the measured flat source size throughout: first with the default
    type-resolution work limit, then with a caller-selected work limit exactly
    equal to its entries, then one below.
    Assert the first denied counts, a completed oracle with the shallow case's
    result, and absence of any depth refusal. In separate cyclic fixtures,
    place the repeated entry immediately below and at the work boundary;
    assert `TypeResolutionCycle` after a successful entry charge and
    `TypeResolutionWorkExhausted` at the denied entry, respectively. In every
    refusal case, assert a healthy sibling generates unchanged and no refused
    item emits a symbol.
19. Source bytes (FR-018-AC-30, planned, IR-511). Measure the flat Rust
    source required for the 100,000-entry closure. Run once under the
    1,048,576-byte default and assert `SourceTooLarge` names that effective
    limit when exceeded. Raise the caller byte limit to the measured count,
    generate and compile the complete crate, then lower it by one byte and
    assert refusal before publication with no partial artifact. Inspect that
    the generated type declarations have constant nesting per source item,
    each module fits the publication artifact limit, and the bundle fits its
    own byte limit.
20. Work-limit representation (FR-018-AC-31, planned, IR-511). Supply
    `u64::MAX` as the type-resolution work limit and assert
    `InvalidTypeResolutionWorkLimit` before lowering or any item charge.
    Supply `u64::MAX - 1` to a counter seam at its last admissible unit and
    assert the first denied `consumed` is `u64::MAX`, without wraparound.
21. Direct unsupported roots (FR-018-AC-32). Admit formula, fairness and case
    roots through Contract IR's public V2 reader, using QSpec FR-370/FR-440
    for formula/case shapes and Contract IR FR-038-AC-107 for fairness. Request
    each with a supported equality sibling. Compare the requested claim id
    with the IR equality-profile lowering record: if it is `Unsupported`,
    compare the refusal's first unsupported reachable id and tag with that
    record; if a case root lowers, assert the root id and `expression` tag.
    Assert one `CompositeEqualityRefusal::Unsupported` claim, no generated
    symbol, no panic and no omitted claim. Compare the sibling's claim and
    source bytes with a run without the refused request. As a separate
    control, request a temporal clause whose closure reaches a formula or
    fairness node and assert its existing QSL-121 blocker.

### Package-bound enum migration controls

Use authentic retained QSL source, original dependency provision and original
request stage/replay limits. Call root `compile_admitted`, check the emitted
and context package identities against the original checked-package identity,
strictly re-read emitted bytes and compare the complete package with the
original. Preserve original source-byte/owner and library-lock authentication;
only a successfully authenticated request exposes `admitted_context()`.
Exercise malformed source and dependency input (Package), QSL stage and
above-reader resource refusal (Resource), a subsequent IR read stop
(PackageReadLimit), and the typed internal Fault path;
none exposes a partial successful context. Do not widen the current imported
context guard without retaining its original admitted dependency packages.

Run the real generated `environment_symbol(Some(context))` and
`oracle_symbol(prepared, left, right, meter)` ABI, with ordered and unordered
enums including unused cases and a parameter without member literal nodes.
Check canonical positions and original member identities against genuine
facade/QSV values. Use same-spelled cases under distinct declarations to
expose conflated identity. Reject a different package context before lookup;
reject a different item/descriptor prepared wrapper before charging. Exercise
Unknown declaration, Unknown case and Foreign direct/transitive dependency
identities, preserving typed owning fields. A raw caller-built member index
cannot satisfy the public ABI. Test-only inconsistent reconstruction must
produce the specified typed error, never a default or empty index.

Retain the empty/missing/wrong-key environment controls of step 6 and every
charge-stop vector of step 5 with the same genuine index. Mutate generated
operator, descriptor and case lookup independently so direct-vs-generated
agreement fails. Check deterministic bytes and claim order after context
threading, with no NodeKey constructor/decoder, generated nominal preimage,
hash or ordinal-as-identity in emitted source. All helpers/modules receive
the same actual optional capability, with genuine Some on nominal paths; retaining one only on the planning side is a failing
control. Source compilation occurs before oracle invocation, and the
emitted oracle remains the implementation under test.

The enum-only lookup cannot issue record/tuple keys. Record the missing
owner lookup/preparation capability separately until it exists; preserve all
original closure/cycle/bound/collection corpus requirements rather than
accepting an Unsupported regression or synthetic key. Manifest/scratch checks
cover equality's root replay dependency and scalar Boolean/integer proof
bodies with a legitimate empty index and no added replay compile leg.
Native enum results are not TC025/TC027 healthy/mutant proof receipts, and
those original proof domains and controls remain unchanged.

### Optional standalone capability controls

Keep the native authenticated preparation above mandatory and unchanged:
its successful request always supplies Some. Separately exercise standalone
IR generation with Option context. With None, use reader-admitted
non-nominal primitive and nested Option/Collection closures and verify their
exact original successful behavior, descriptors, values/domains and charges.
Retain the exact conversion FormMismatch("conversion") and quantity
Unsupported("quantity") IR negatives where they need no nominal key; do
not substitute a numeric source fixture for a retained conversion node or
claim the unavailable source quantity declaration compiled. None is neither
source authentication nor an empty nominal capability.

Cover None at nominal identity reconstruction from each left/right operand
source, each primitive-source nominal conversion target and nested
Option/Collection enum types. After the separate genuine record/tuple lookup
capability exists, cover its fields and recursive closures too. Assert
MissingAdmittedContext names the first actually reached nominal declaration,
with no fabricated key, fallback index, Unsupported substitute, missing
member or partial publication. Preserve the existing charged traversal,
first-error/cycle/work precedence, deterministic order and sibling isolation;
capability demand must not become an outer-tag or complete-context pre-scan.

Pass Some of a genuine wrong-package context to primitive-only, nominal and
empty standalone requests. After existing setting/accounting checks, each
must fail the global context guard before classification. Both primitive
and nominal emitted constructors reject that Some, while a context-free
recipe can prepare with None. Nominal emitted preparation with None gives
the typed missing-capability error before lookup or partial publication.
Equality's typed Option ABI may still name root replay in its manifest;
ordinary scalar proof manifests and bodies do not acquire that dependency.

After complete zero-enum closure resolution only, require a private empty
index. Reached enums must still include every genuine case, used or not;
failed lookup, missing context or a stopped walk never proves zero enums.
For wrappers prepared with None and Some, retain expected package/item/
descriptor binding and the complete index privately. Exercise wrong binding,
present-context mismatch, missing context in a nominal recipe, and private
environment-only faults; assert invariant refusal before any meter charge.
Then execute both check_type guards, same-index check_equality and evaluate
on the valid path. No raw caller index or public unchecked wrapper is added.
These controls change neither the merged AC-14 bound-intake split nor the
original TC025/TC027 domains, harnesses or proof obligations.

### Direct Integer root and asymmetric descriptor control

Use the genuine source fixture with `a: Int[0,9]` and `b: Integer` and its
reader-admitted graph, preserving the original package/lock/source bindings
for any native route. Assert a's parameter semantic_type names its own range
and b's names canonical base Integer, which is also that range's base. The
standalone descriptor uses the actual left/right source IDs and an independent
caller-selected Integer conversion target for the left. Reconstruct direct
base Integer sources/targets as unbounded and the explicit range as [0,9],
without adopting the sibling's bound. Compare generated/native behavior over
the original domains, including right values outside [0,9], and preserve the
refusal of a narrowing conversion from unbounded Integer. Keep charges and
descriptor order unchanged and derive the native expectation from the request,
not generated Rust.

Inspect whether the genuine graph actually retains a convert node. If source
`convert<Integer>(a)` is erased, this is the parameter/asymmetric caller-target
control of AC-3/AC-43, not proof of AC-15's conversion-reference branch.
Retain the exact admitted-IR conversion-reference assertion independently,
using the correctly scoped standalone capability without claiming unavailable
source emission. The existing AC-16 numeric member/base equivalence test and
record/tuple member semantics stay unchanged. Their unresolved genuine member
bound association must not be filled by guessing lexical ownership or adding
a public descriptor field; this control is direct-root-only.

## Expected Results

The source scans find zero panic sites, and the render function returns a typed
error for a quantity or reference type rather than panicking.

Direct formula, fairness and case requests have one typed unsupported claim
apiece, with request identity distinct from IR's causative node when they
differ. No refused item contributes source, and the healthy sibling is
unchanged; the temporal-clause control remains blocked on QSL-121.

Every admitted shape and schedule is generated; every refused item is absent
from the source and carries its own typed reason; bytes are identical across
runs and orderings; each entry's recorded descriptor and schedule equal the
request's and `CheckedEquality::schedule()`; all three executions agree on every
vector's outcome, charges and counters; every injected denial yields
`Incomplete` at its point without applying that charge; a foreign environment
refuses as `CheckedInvariant` without panicking and without charging, decided by
the emitted `check_type` calls; the two operators generate under distinct
symbols with complementary results and one `caller_declared`
mark; and the generated crate
compiles with `publish = false` and no charge or pair-count literal.

The recursive List and Tree items generate; unclosed option and sequence cycles
return the typed cycle refusal, and an over-budget acyclic type walk returns the
typed work refusal with the failed-entry count. Neither case aborts or starves a
healthy sibling. Steps 16 and 17 remain planned until their executable tests land.

Operand construction is the corpus's work, not the generator's: the composite
and collection values compared are built by the runtime's own FR-008
constructors, so a set's or bag's retention order is never a choice the
generated code makes. For the same reason the schedule the runtime selects and
the pair count `plan_equality` forms are not separately asserted: the generated
function is a delegation to `CheckedEquality::evaluate`, so both are fixed inside
the runtime and step 4's charge-sequence comparison already covers them. What
this test asserts about them is what the emitter chooses — the descriptor and
schedule the claim map records, in step 2.

No vector produces `Refusal::ForeignReference`: reference operands are excluded,
so `CheckedInvariant` from step 6 is the only run-time `Refused` a generated
oracle here can yield.

Steps 13 to 15 (FR-018-AC-21 to FR-018-AC-23, IR-264) are planned and narrow the paragraph above
for the shapes they cover: for a record, tuple or option over Boolean and bounded-integer leaves
the pair count is asserted, against a count the test makes from the values by QSpec FR-149's
occurrence-pair rule and not from the runtime, because the shadow of FR-015-AC-69 models exactly
that count and the refinement obligation of FR-028-AC-15 compares the oracle with it.
