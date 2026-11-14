---
id: TC-025
title: "Verify separate bounded Kani obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-025: Verify separate bounded Kani obligations

## Description

**IR-348 retirement disposition (2026-10-27).** The V1 bundle controls in steps 16 and 17
are historical V1-bundle procedures, not active V2 evidence. Step 18's inspection remains required for surviving emitters,
and FR-015-AC-7/53/58 remain open for their surviving harness families. The historical V1
procedure below remains for traceability and does not count as current V2 evidence. IR-1044
backs the V2 generation-time census in steps 4, 7 and 13 through two tagged tests over an admitted
QSL precondition; no installed-Kani verdict follows from that evidence. IR-917 owns the remaining
QSL-authored V2 clause control.

Verify that bounded contracts produce one harness per obligation kind and
refuse non-finite obligations. IR-268 covers the collection half of planned V2
symbolic obligations: the claim is an admitted QSL clause/function body over a
selected model collection field. Graph reachability is required by
FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108 (IR-805); it is not IR-268
evidence. The finite-input collection and graph corpus is not proof evidence
for either symbolic family.

## Test Procedure

For IR-268, author a QSL model with a bounded `Sequence<IntRange>` field
whose cardinality and element domains both have nonzero inclusive endpoints,
and clauses/function bodies that apply `forall` and `exists` to that selected
field. Emit each package through QSL's public replay path and admit the emitted
V2 package through Contract IR. Inspect that the admitted claim is the clause /
function body and that its `quire.op.collection.forall` or
`quire.op.collection.exists` operation identifies the selected field path; do
not map these operations to the finite-input `QueryKind` as if they were the
claim. Also use a bounded Boolean collection field. No copied package wire,
hand-built package builder, concrete `Vec` input or caller semantic ceiling is
used.

Distinguish the two missing-upper-bound cases using otherwise identical
`Sequence<IntRange>` fields. With multiplicity lower bound `0` and no `upper`,
emit and admit the QSL V2 package, confirm the selected field is an unbounded
collection (`bounds: None`), then submit its collection claim to IR and require
`requires_bound` naming the selected field or its unbounded collection type,
with no harness. With multiplicity lower bound `1` and no `upper`, require QSL
assembly to refuse `unknown_required_feature`/`unsupported-feature` for
`UnsupportedModelMember`; no V2 package or IR item is produced, so this case
must not be reported as an IR `requires_bound`. Separately use an admitted,
finite-cardinality field whose element type is unbounded `Integer`; require IR
`requires_bound` naming the element type and no harness.

For each supported function-body and invariant collection claim, inspect the
generated `kani::any()` draws and assumptions. Check both cardinality endpoints and every intervening
length, both inclusive `IntRange` endpoints and intervening values, both
Boolean values, and preservation of declared order and uniqueness. For a function-body control, replay the same authored function with
`qsl_replay::replay` as the independent native oracle and compare its result
with the generated claim's assertion for the same input. For a falsified
invariant, use `qsl_replay::replay_state_clause` with a snapshot observation. Seed a claim or subject defect and require installed Kani to report
`Falsified` with an in-domain playback; the healthy direct-assertion case must
be `Verified` with its final cover satisfied. Refuse an unbounded cardinality, unbounded
element type, unsupported collection form or unsupported `forall`/`exists`
body, malformed or absent selected declaration, and an over-ceiling domain.
Check typed `requires_bound` versus `unsupported`, no emitted harness for each
refusal, and one disposition per item in a mixed request.

For FR-015-AC-96/99 (IR-1091), author precondition and postcondition
collection clauses over the same QSL-emitted, IR-admitted OrderedSet field as a
function-body and invariant control. Submit one item per clause through the
public V2 request. Require the same inclusive cardinality, element, order and
uniqueness assumptions for every drawn input kind, with no duplicate standalone
proof. The precondition harness asserts that every drawn evaluation completes
a Boolean, permitting both true and false, and separately covers a true case;
an impossible but total precondition reports `CoverUnsatisfied`. Seed a
precondition oracle mutant that yields `Undefined`, `Refused` or `Incomplete`
on an in-domain draw and require a falsified totality assertion, never an
assumption. Compare its native result with QSL's public clause-evaluation
facade once available; counterexample replay alone cannot classify true, false and incomplete
evaluation across the admitted domain.
For the collection postcondition, identify the selected model object and
every admitted model field, including fields neither the postcondition nor
any sibling precondition reads. Read each
field's admitted type and domain; require the selected field's exact
`CollectionType`, inclusive cardinality and element domain. Generate one
`CgObjectState` struct with a `quire_exact::Value` field for every admitted model field in checked declaration order, with generated names that remain distinct for two
source spellings that would collide after naive Rust identifier conversion.
Compare the persisted mapping of each generated name to its checked declaring node ID
and field name before and after equal regeneration. Draw and validate all fields
for the whole-object
prestate. Construct the selected collection field through the FR-392 checked
builder with a caller-authenticated schema-aware element authority. Check both
cardinality endpoints, inclusive Boolean/IntRange element endpoints, order and
uniqueness in that field. Verify a nonselected contract-read field and an unread admitted field both
stay present in the generated state, retain their identities, and participate
in their declared domain assumptions.
Copy every field value, then call the customer subject at `subject_path` as
`fn(&mut quire_exact::Value, &mut quire_exact::Value, ..., P0, P1, ...) -> R`: one
mutable exact-value reference per admitted field in checked declaration
order, then FR-025-ordered supported parameters and the admitted supported
result type (`()` if absent). Inspect the generated call and the retained
prestate copy to verify FR-025-AC-8/TC-036 per-value semantics. Verify the
call and result type in generated Rust. A selected-field-only state must fail
this control and leaves FR-015-AC-96/99 partial. Refuse an unsupported
parameter/result type, missing field/type mapping, any required field CG cannot
symbolically construct and validate, unavailable domain or builder, absent
authenticated authority or unavailable typed invocation with
a typed item reason and no harness or proof record. No CG-local `Vec`/array
collection ABI or exact record/tuple masquerading as an object is allowed.

The postcondition harness assumes every supported sibling precondition on the
same complete prestate and operation parameters, and refuses before invocation
if any sibling is unsupported. Then it calls the subject. Check every
subject-produced field against its admitted type and domain; inspect the
selected produced `Value::Collection` with FR-392 using its selected field type
and authenticated schema-aware authority. Assert successful inspection, the
produced post-state cardinality, element domain, order and uniqueness, and the
authored postcondition; then cover non-vacuity. Seed subjects that produce a
duplicate, below-minimum or over-maximum count, out-of-range element, forged
nested element or wrong exact collection type; each must falsify the
poststate-domain assertion with in-domain whole-object prestate playback,
without an assumption hiding it. Independently mutate a nonselected contract-read field and an unread admitted
field beyond each admitted domain; require each field's domain assertion to
fail. Separately author a postcondition requiring the selected
OrderedSet's stored member order to match its prestate order and seed a subject
that reorders distinct members. FR-392 inspection must admit that reordered
set while the authored order postcondition falsifies; a mere permutation is
not a collection-domain failure. The invariant keeps a direct assertion.
Replay a falsified invariant through `qsl_replay::replay_state_clause` with a
snapshot observation, and a falsified postcondition with an invocation plus
pre/post snapshots. A healthy postcondition verifies; an authored
postcondition or subject mutant falsifies with in-domain playback. These
pre/post controls are planned and receive no current proof credit. FR-392
defines the collection value and checked boundary; CG owns the generated call
and state struct, while this procedure claims no QSL generation of either.

For FR-015-AC-128 (IR-1091), author a QSL invariant and a constraint clause
with collection quantifiers over the same bounded field, emit the package through
QSL and admit it through Contract IR. Submit the invariant as one claim and the
constraint as a separate claim. The constraint item must settle `unsupported` with
`CollectionClaimNotRendered` / `UnsupportedClaimShape`, no harness and no proof
credit; it must not turn into a `kani::assume` or change the invariant item.
A constraint never supplies a missing cardinality or element bound or rescues a
unique-capacity refusal. Constrained-method request identity, boundary-state
evaluation, `Undefined`/`Refused`/`Incomplete` handling and Kani outcome mapping
remain planned under IR-1157; no constrained proof is expected from this control.

For FR-015-AC-96, use QSL-emitted, IR-admitted unique Boolean and bounded `IntRange`
fields after the applicable
form, empty-domain, representation, unwind and memory checks pass. A feasible
unique maximum equal to its distinct-value capacity retains both cardinality
endpoints and every distinct-value/order assumption. With the same adequate
ceilings, a maximum one above capacity settles `unsupported` with
`UniqueElementCapacityExceeded` naming the capacity and declared maximum,
with no harness or proof credit even when the minimum is at most capacity;
it is not an invalid-model or invalid-package result. Set the minimum above
capacity and require the same typed refusal, not a shortened maximum or a
vacuous proof. Keep a comparable non-unique field eligible for its full
maximum. Check Boolean capacity `2` and an inclusive bounded `IntRange` capacity.

At the capacity decision boundary, use a property over representable `i64`
endpoints and declared `u64` maxima with an independent widened-integer oracle:
refuse exactly when `maximum > upper - lower + 1` for nonempty `IntRange`, and
when `maximum > 2` for Boolean, after the earlier checks pass. Cover a
single-value interval, negative endpoints, maximum below/equal/above capacity,
and the full `i64::MIN..=i64::MAX` interval whose capacity is `2^64`; the
full-width case must not overflow or falsely refuse a `u64` maximum. Exercise
small admitted fixtures end to end; the full-width capacity arithmetic can be
checked before rendering without allocating an enormous symbolic array.

For precedence, separately pair a unique maximum above the mathematical
capacity with an unsupported collection form, an unrepresentable endpoint,
insufficient unwind, and insufficient memory. Use an empty `IntRange` with a
positive declared maximum as its own case; it has no distinct-value capacity.
Require the existing
`UnsupportedFieldType`, `UnsatisfiableElementDomain`,
`DomainNotRepresentableInI64`, `UnwindBelowCollectionBound`, and
`SymbolicDomainExceedsMemoryCeiling` reasons respectively, each with no
harness; the representability, unwind and memory controls are three distinct
FR-015-AC-101 checks, not one disjunctive control. The current
`ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics`
control establishes only the present unsupported-form behavior; it does not
back the new capacity-refusal or feasible-unique obligations.

For AC-101's selected-field boundary, ask QSL to bind a missing selected model
field and check its typed source-located refusal before it emits a package.
Separately submit a malformed selected projection to Contract IR strict
admission and check its typed located refusal before it admits a claim. Neither
case produces a CG item or harness. Inspect CG's admitted-claim resolution
and test its defensive helper invariant: if the checked selected declaration
or field cannot be resolved, it returns `FieldNotAdmitted`, which negotiation
maps to `unsupported` without a harness; resolution must not select a
same-named field from another declaration. The helper control does not claim
an authentic malformed selected projection can pass strict admission. Add a
public admitted-claim negative control only if one can reach this branch
through strict admission without bypassing its validation.

Change one selected field bound, claim, subject and unwind separately to check
identity changes; repeat equal inputs for byte identity. Compare recorded
identity domains and complete ordered options with generated source and FR-017
run evidence. Assert that the evidence argument vector contains exactly the
`kani` subcommand, the harness identity's options in order and the report-export
flags in order, and that its solver kind and unwind equal the identity's. The
collection-family tests exist and are tagged
(IR-268, CG #396); the Kani-lane run is `Verified` (94 SUCCESS) with the
mutant `Falsified` (93 SUCCESS and 1 FAILURE), per IR-268's close-out evidence.
Graph family: Slice A of IR-805 (relationship-end graph claims) is delivered and every graph criterion is PARTIAL with its own gap, as FR-015's graph paragraph states: slice D implements AC-98 and AC-105 for the field-edge `reaches_field` form and leaves them PARTIAL on QSL-1904's population link; AC-106's slice B is delivered for the relationship-end form (the negated-claim, zero-length-path and rejected-self-loop mutants are Falsified under installed Kani, each replayed through `qsl_replay` over the authored clause; the wrong-role clause is NOT evidenced; the lib tests run outside `make kani-gate`); slice C is delivered for the relationship-end form (the domain oracle, healthy pass, boundary probes and the three pure-bound mutants, each over a unique and a non-unique end, run under installed Kani; status in FR-015-AC-106), and its `{0,3}` installed-Kani runs are tautological smoke runs with no healthy-case credit; AC-107's one-node-under-two-clauses clause is evidenced as its row states; AC-108's evidence is stated branch by branch, with inspection-only branches named. AC-100's and AC-101's graph clauses are implemented for both forms. The collection-family AC-100 ordered-option comparison is covered by the AC-100/TC-025-tagged `collection_run_evidence_preserves_every_ordered_identity_option` test, which compares actual launched arguments and evidence with the identity options, solver kind and unwind; the graph-family clause is covered by `graph_run_evidence_preserves_every_ordered_identity_option` (AC-100/TC-025). This is launcher evidence, not a new installed-Kani proof. The representational-ceiling clause has AC-101-tagged generator tests; selected-field boundary refusals still need source-located QSL and Contract IR tests. FR-015-AC-96 has tagged feasible OrderedSet uniqueness and Boolean/IntRange capacity-refusal controls; CG #616's installed-Kani controls verify an OrderedSet over IntRange lengths `[2,3]`, falsify and natively replay its claim mutant over that domain, and verify a Boolean OrderedSet at length `[2,2]` while falsifying the comparable Sequence on duplicate Boolean values. AC-96 remains PARTIAL: tagged generation tests cover function-body Sequence and OrderedSet symbolic domains, order, uniqueness and capacity, plus invariant-clause Sequence and OrderedSet domains, order and uniqueness; whole-object collection preconditions and postconditions have generation and compiled ordinary-execution controls in `tests/it/kani_collection_operation.rs`, including extra parameters, results, unread fields, sibling assumptions and retained prestate order. These controls do not add installed-backend or independent native-replay proof credit; CG #654 owns native precondition equivalence, and submitted constraint clauses remain typed unsupported under FR-015-AC-128 pending constrained-method semantics (IR-1157). Nonunique and unsupported-form admitted-item controls are tagged. The admitted full `i64` element interval at feasible maximum `3` has an AC-96-tagged generation control through QSL and IR; it adds no installed-Kani proof credit. The `u64::MAX` cardinality comparison remains helper/property-only because earlier representational, unwind or memory checks preempt rendering. An admitted QSL/IR control now backs `DomainNotRepresentableInI64` before unique capacity for an endpoint above `i64::MAX`; empty-IntRange precedence remains helper-only because QSL rejects that malformed model before emission. The installed controls exercise only the stated OrderedSet bounds. FR-015-AC-99 has tagged symbolic-draw and native replay controls, while its final reviewed-head Kani gate and graph family remain pending. The finite-input
corpus's Boolean and generation-time outcome do not satisfy these controls.

For the graph family (IR-805), author a QSL unit whose model has an object
type `Order` with an optional `Reference<Order>` field, a `Reference` field and
a bounded `Sequence<Reference<Order>>` field, and relationships `next`
(`{0,1}`), `kids` (`{0,3}` unique), `litter` (`{0,3}` not unique) and a `{2,3}`
end; write invariant/precondition/postcondition clauses of the form
`reaches(self, target, <name>)` and `not reaches(self, self, <name>)`. Emit
through QSL and admit through Contract IR (no copied wire or hand-built
builder). The IR-851 `relationship_end` and IR-852 `reaches_population`
accessors are merged (IR #438 and #442); relationship-end graph harness
generation is delivered (IR-805 slice A) and the field-edge form is implemented (slice D), needing QSL's population link (QSL-1904) to generate.
Real-package population emission and
the claim's population link remain owned by QSL-820. The following refusal
controls remain Planned CG checks, with no harness: (1) the claim is the
`state_clause` node id and argument path of a reachability application of one
of the forms defined by [QSL FR-365](ix://agent-ix/quire-spec-language/FR-365)
and [QSL FR-105](ix://agent-ix/quire-spec-language/FR-105); (2) a claim with
`Unavailable` population selection settles `unsupported` naming it, without
assuming the merged IR mapping makes a missing package link available;
(3) with an `Unbounded` selection, a claim whose population has no finite
authored bound settles `requires_bound`, and an unbounded collection edge
settles `requires_bound`; (4) each other FR-015-AC-108 `unsupported` input
reachable from an admitted package settles `unsupported` naming it,
including a relationship-end claim whose typed end is unavailable to CG; (5)
claim location: the same reachability twice in a condition, one node under two
clauses, the two forms, a collection claim and a bad path (FR-015-AC-107); no
caller `max_expansions`, population or multiplicity changes any outcome.
Inspection: the three `relationship_end` refusals (`UnknownNode`,
`NotReachesApplication`, `UnresolvedEnd`; Contract IR FR-038-AC-262 to
FR-038-AC-268) cannot arise from a QSL-emitted, IR-admitted package
(`UnresolvedEnd` needs a crate-internal post-admission mutation), so their
`unsupported` mapping in FR-015-AC-108 is checked by inspection of the
generator's error mapping, or by a named in-crate accessor test double, and is
not a real-package runtime step. AC-107's location refusal runs before the accessor, so
a path naming no reachability is never `NotReachesApplication`. Also, QSL refuses `reaches` in a function body, so no function-body graph
claim can reach the generator. Slice D delivers the field-edge healthy and mutant controls (FR-015-AC-106 records them; they run only against a QSL that links `reaches_field` to its population, QSL-1904). Real-package steps that wait on CG integration and QSL-820 emission (unbacked):
inclusive
populations `N = 0` and the maximum, edge counts `lower`, `upper` and between,
`kids` uniqueness, `litter` repeats and `{2,3}`; the field-edge `{0,3}` run and the field-edge `Set`/`Bag`/`OrderedSet` runs, and a wrong-role mutant (slice B delivered the relationship-end claim
mutant and the zero-length and rejected-self-loop harness mutants: the claim
mutant reproduces the violation under `qsl_replay`, each harness mutant is a
counterexample the unchanged authored clause does not reproduce); the separate
domain-bound controls below; and identity
changes for the clause path, form, field or role, population extent, edge
multiplicity, subject and unwind, with byte identity on equal inputs. The
finite-input `GraphRequest` corpus is not evidence.

For FR-015-AC-106's separate domain controls (IR-939, slice C of IR-805: delivered for the relationship-end form in `tests/it/kani_graph_domain_private.rs`; the field-edge form is not delivered by slice D), retain the
original authored QSL source, admitted package, selected clause/path and typed
population/edge projections. Independently check the expected domain against
the authored declarations before seeding a mutation. Use an available real-
package population profile with `N = 4`, lower degree `0` and upper degree `U
= 3`, once with a unique selected end and once with a non-unique selected end;
retain the existing field-edge, `{2,3}`, zero-population, identity and refusal
cases above. The fixed verification carrier has capacity for five objects and
four edges per row. Draw candidate counts, active degrees, targets and
operands across that full carrier, independently of original and generated
domain admission. Carrier padding/encoding constraints must preserve every
active graph; do not assume the predicate being checked. Check the observer of
the actual emitted sampler's ranges, arrays and guards, including inactive
slots and insufficient capacity, against its generated source. The original
production arrays need no fifth row or fourth edge; observation of their
rejection must not index those absent slots.

Run the unmutated domain oracle with installed Kani, asserting both
conformance and completeness as FR-015-AC-106 defines. Require `Verified`, a
satisfied final cover with `D = G = true` at count four, and separate
boundary-probe runs with `D = G = true` for degree zero and three, each with exactly one final cover. Run three separate
mutants against the unchanged original expected domain, candidate carrier and
clause evaluation rules: generated population maximum five, maximum three, and
selected edge maximum four. Change associated production storage when the
selected bound determines it; do not change the traversal or count
interpretation. Require `Falsified` at the named domain assertion and decode a
typed active-graph witness, with original/generated bounds and capacities and
independently evaluated membership results. Require a separate carrier
boundary-probe run for each mutant, with exactly one final cover for its
required mismatch and no membership assumptions; each installed-backend cover
is satisfied. These concrete boundary witnesses must be reachable in the
candidate carrier:

- Population five: five objects, zero edges, operands at object zero;
  `G = true`, `D = false`, violating conformance.
- Population three: four objects, zero edges, operands at object zero;
  `D = true`, `G = false`, violating completeness. A capacity rejection is an
  observed omission, not a memory fault or a clause-result change.
- Edge maximum four: four objects, one row containing four targets and every
  other row empty, operands at object zero; `G = true`, `D = false`, violating
  conformance. For the unique end use targets zero through three; for the
  non-unique end include repeated target zero. Both are valid in their
  respective mutant domains and violate only the original edge maximum.

Retain source, actual installed-backend evidence, failed property identity,
non-vacuity/boundary cover results and native domain-witness checks for each
profile and run. Slice C's runs do not yet retain the generated source, failed property identity or FR-017 evidence as files (the lib tests keep only the verdict and the decoded witness checks, and delete their scratch crate); that retention is an open gap, so the recorded verdicts in FR-015-AC-106 must be repeated to inspect them. This is a separately identified domain oracle, not a new
reachability assertion or a claim that the original QSL clause changes truth.
Do not replay mutant-only graphs as original-domain semantic counterexamples.
The semantic mutants still require same-authored-clause `qsl_replay` on
original-valid graphs (reproduced violation for a claim mutant, a holding clause for a harness mutant). No sampler filtering, reduced authored range, metadata
comparison, generation-time Boolean or substituted traversal mutant qualifies.
If the actual owner inputs or verification carrier are unavailable, record
that control unqualified; do not infer a field population, fabricate a package
or mark IR-805 or downstream graph retirement FR-015-AC-111 complete.

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain, one with unsatisfiable bounds, and one over a
caller-declared oracle operation. Inspect harness identities, bounds and
assumptions, and run the installed Kani backend on the bounded harnesses.

Additionally: count the covers in each generated harness and locate the
contract cover relative to the contract call; request a postcondition without
the precondition sharing its anchor; read the solver and both option vectors (proof and witness) out
of each identity; regenerate from equal inputs and compare bytes; regenerate
with a changed unwind bound and a changed subject and compare identities;
search each harness source for a loop bound; and submit a request with no
items, with more items than the ceiling, with an unparsable subject path, and
with an unwind bound on each side of the admissible range.

For FR-015-CON-1 and FR-015-CON-2, call the clause ABI and subject-binding seams directly with
bounded integer dependencies whose minimum or maximum is just outside i64 and at the i128 extreme.
Assert `UnsupportedDependency` in the clause-lowering detail and `UnsupportedBinding` at the
subject-binding seam; neither produces a binding or harness. This direct check remains required
even when the public Boolean-oracle path refuses the same declaration earlier.

State clauses: generate the operation-contract and frame-effect harnesses of one
`postcondition` clause over one integer field; read each identity's scope, field
ranges, granted and forbidden fields and cover count; and submit a clause that is
a precondition, one comparing two reads of the same side, one comparing two
fields, a frame that creates or deletes an object or grants a relationship or a foreign field, a condition that is a negation, compares to a literal or reads through a second parameter, a clause field with no integer range,
a state without the clause's field, a frame granting every field, and an
unparsable path and an out-of-range unwind bound. Run the installed Kani backend
over a healthy subject, a subject mutated to debit, a subject writing a granted
field, a subject also writing an ungranted field, and the allowed subject against
a frame regenerated from a package whose `modifies` is emptied; then execute the
forbidden counterexample natively and replay it, and the allowed run, through
QSL's `replay_frame`.

## Expected Results

The QSL-authored V2 postcondition declares a Boolean operation result, binds
both `amount` reads to one drawn operation parameter, and binds the exact
Boolean returned by the subject to `result` without a separate draw or
parameter argument. The compiled postcondition, clause oracle and independent
native expectation use that same `amount`; the harness retains its native
arithmetic probes, compares the subject result with the independent expectation,
and separately asserts the clause oracle over that result. It produces a
clause-identified oracle and harness, a compiled runtime-helper call, a healthy
installed-Kani `Verified` result with satisfied cover, and a helper-body-only
installed-Kani `Falsified` result with in-domain playback. The two runs retain
identical clause and generated proof source; the independent implication
expectation rejects a helper and oracle that agree on a wrong result
(FR-015-AC-119, planned).

Four distinct harnesses carry their bounds; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded, the
unsatisfiable and the caller-declared obligations are refused with no harness.
For collection claims, the function-body and invariant direct harnesses
assert their authored bodies over the full symbolic domain. The precondition asserts Boolean completion on every draw and separately
covers a true case; it does not assert universal truth. The postcondition calls
the subject after all supported sibling preconditions (refusing unsupported
siblings) and asserts produced post-state cardinality, element domain, order
and uniqueness as well as its authored clause. These pre/post
results remain planned under IR-1091 and are not credited by the current
function-body evidence.
For the collection claim, lower `0` with no finite upper bound is admitted by
QSL and settles as IR `requires_bound`; lower greater than `0` with no finite
upper bound is rejected by QSL assembly as `unknown_required_feature`/
`unsupported-feature` before IR receives a package or item. A finite
collection containing unbounded `Integer` elements reaches IR and settles as
`requires_bound` for the element type. A constraint clause submitted as its
own collection item settles `unsupported` with `CollectionClaimNotRendered` /
`UnsupportedClaimShape`, emits no harness and leaves the sibling invariant
claim unchanged. No constrained Kani verdict is credited (IR-1157).

Each harness contains exactly one cover, and the contract harness's cover
follows its contract call. Each contract harness carries a `requires` for every
package precondition sharing its anchor and embeds no other obligation's
oracle; the postcondition requested without its sibling precondition is refused
naming that precondition, with no harness. Every identity records solver
`cadical` and both ordered option vectors (the proof vector with no concrete-playback
option, and the witness vector that inserts `-Z concrete-playback` before `--harness` and ends
with `--concrete-playback print`, byte-equal to the pre-IR-1022 vector), and no option in either
enables stubbing (FR-015-AC-9). The expected vectors are literal argument lists written in the
test, never recomputed with the production `adapter_options`, so a mutant planted in that builder
cannot change both sides. For harness `m::h`, unwind 3 and solver `cadical` the literal proof
vector is `-Z function-contracts --harness m::h --exact --unwind 3 --solver cadical
--output-format regular` and the literal witness vector, which is also the literal pre-IR-1022
argv, is `-Z function-contracts -Z concrete-playback --harness m::h --exact --unwind 3 --solver
cadical --output-format regular --concrete-playback print`. Mutants, each caught by a tagged
equality against these literals plus a refusal case that presents the mutated identity to the
seal checks named in FR-015-AC-9 (the constrained-identity recomputation and the V2 clause and
collection `has_valid_provenance`): leave `--concrete-playback print` in the proof vector; omit
the witness vector; move `-Z concrete-playback` after `--harness`; emit a stubbing option in the
witness vector only. `HarnessSpec::new_constrained` (`src/kani/generate/spec.rs`) rejects any
other vector with `InvalidExpression` today for constrained identities only; the other kinds'
checks are as FR-015-AC-9 states, and IR-1022 changes the code.
Regeneration is byte-identical, while the changed unwind bound and the changed
subject each produce a different identity. Every symbolic argument
carries an inclusive assumption equal to its IR domain, the post-state result
is required to lie in the same domain, and no harness source contains a loop
bound. Each of the four malformed requests is refused whole, with no item
accounted.

## Implementation

`tests/it/kani_obligations.rs`, and for state clauses `tests/it/kani_obligations_state_frame.rs`,
whose module name `make kani`'s `kani_obligations` filter selects. The default lane negotiates every item before
any harness is exposed and checks separate harnesses, IR-derived bounds,
assumptions and every refusal. `make kani` runs the ignored lane serially under
a host lock against the installed Kani backend: it verifies the precondition,
postcondition and invariant harnesses, falsifies a seeded postcondition defect
with a concrete counterexample, and reports a contract harness with jointly
unsatisfiable requires as `cover_unsatisfied`.

## State transitions, censuses and dispositions

1. Generate a zero-input transition over one bounded-integer state value, run it for an identity
   subject and for a subject that changes the value. The identity subject verifies, and the
   changing subject is falsified with a concrete counterexample (FR-015-AC-19).
2. Generate a harness for each Boolean connective and each integer comparison claim, and compare
   the embedded oracle with the FR-014 crate's function byte for byte. They are identical
   (FR-015-AC-20).
3. Request a precondition that reads a post-state value. It is refused naming the node, with no
   harness (FR-015-AC-21).
4. Request a supported V2 clause obligation with a declared census that is empty-identity,
   duplicate-identity, kind/state/path-inconsistent or non-`Required`. Each is refused as a typed
   invalid request with no harness (FR-015-AC-22, FR-015-AC-44).
5. Request items that settle `supported`, `requires-bound` and `unsupported` together. Each has
   exactly one `ObligationDisposition`, and each item that is not `supported` keeps its source
   identity and reason with no harness and no assumption (FR-015-AC-23).
6. Run a falsifiable plain bounded-integer comparison harness. The falsified run prints a concrete
   playback (FR-015-AC-24).
7. Request a supported V2 clause obligation with valid censuses whose dependencies are all passed,
   one missing and one failed. Parse each persisted identity record and inspect the full sorted
   census, readiness `ready`, `incomplete` and `incomplete`, and execution `not_run`
   (FR-015-AC-25, FR-015-AC-45).
8. Generate the operation-contract and frame-effect harnesses of one `postcondition` state clause
   over one integer field. Each carries one cover, an identity scoped to the operation, anchor,
   frame and object, and the IR range of each state field (FR-015-AC-26 to FR-015-AC-28). Each clause or frame
   shape outside that encoding is refused by name with no harness (FR-015-AC-29). The installed
   backend verifies the healthy subject, falsifies the debiting subject, verifies the granted write,
   falsifies the ungranted write naming its field, falsifies the allowed subject against a frame
   regenerated with nothing granted, and QSL's `replay_frame` reproduces the forbidden write and
   finds the allowed run inside its frame (FR-015-AC-30 to FR-015-AC-32). The default suite builds that
   replay's request and envelope through `qsl_replay::call_site` and settles a forbidden and a
   granted write without Kani, and refuses an operation with no frame, declared or not (FR-015-AC-33 to FR-015-AC-36).
9. Generate the scalar harness of each integer operation (add, subtract, multiply, negate). Each
   asserts the oracle's outcome against the operation evaluated natively in `i128`, with a `Completed`
   arm, a `Refused` arm and a failing catch-all. The installed backend verifies the healthy `x + 1`
   harness and falsifies it with the oracle's `Add` replaced by `Subtract` (FR-015-AC-37).

## V2 clause claims and the V2 census

The generation-time census portion of step 13 is implemented for supported V2 clauses (IR-1044).
AC-42 has partial generation-time evidence for admitted division, modulo and Text equality clauses
(IR-948).
AC-40 and AC-47 have implemented generation-time controls (IR-948). AC-40 covers admitted
referenced operators and an inline Boolean literal. Other V2 controls retain their row statuses.

10. Request one V2 clause claim each over a `precondition`, a `postcondition` and an
    `invariant` node of an admitted `CheckedPackageV2`. Each yields one harness of its own
    kind with one drawn value per drawn input (the pre-state of every field the body reads, bare or `pre(...)`, and the
    operation parameters, bounded by their declared domains; the postcondition's result
    and post-state come from the subject call and their domain is asserted), and none takes a
    bound from a caller (FR-015-AC-38, FR-015-AC-39). The invariant harness asserts the
    clause with no subject call (FR-015-AC-49).
11. Generate admitted clauses whose Boolean conditions reference expression nodes holding
    each listed connective and bounded-integer comparison, and one clause with an inline
    Boolean literal condition. Compare each embedded oracle with the FR-014 crate's
    clause-identified function byte for byte, retaining the clause node identity in both
    forms (FR-015-AC-40; needs FR-014-AC-38). Operator applications stand at their
    referenced nodes' body roots under QSpec FR-322's flat body grammar, rather than
    nested in the clause's condition argument. Then generate the
    arithmetic postcondition `amount < 1000` implies `amount + 1 <= 1000`, check the
    native `i128` assertion of each arithmetic subterm. Admit QSL's checked
    `numeric.narrow` wrapper around literal `1` through FR-014-AC-54, without
    treating it as an arbitrary conversion or adding an admission charge.

    Run the installed backend over the healthy clause and, in separate runs of
    the same clause, mutate the `+` arithmetic executed by the embedded clause
    oracle (or the authoritative kernel operation it calls) to return
    `left + right + 1` and `left`. Require both unconstrained installed-Kani
    mutants to be `Falsified` on the independent native assertion with typed
    in-domain playback, without requiring the backend to select a particular
    witness. Retain the plus-one mutant's sealed generated source and body,
    identity record, oracle bytes, clause identity and native expectation for
    TC-027 step 5, which owns FR-015-AC-41's deterministic
    `amount_current = 999` playback through the public
    `execute_kani_concrete_playback` entry (FR-017-AC-60). That control must
    execute the actual generated independent arithmetic assertion against the
    same patched authoritative owner, attribute its failure to that assertion,
    and restore the full sealed rust artifact after removing the appended test.
    A mutation confined to a hand-written compiled subject or a harness probe
    does not satisfy this control (FR-015-AC-40, FR-015-AC-41).
    Each unconstrained mutant run is the two-phase run of FR-017-AC-63 (IR-1022): its witness
    phase needs the trace mode. On the meter harness the traced refusal-order mutant peaked at
    19,671,392,256 B on a Peter-authorised 40 GiB probe and exhausted 16 GiB with no verdict (quire-exact TC-931
    Status and IR-844); no healthy-proof fit is claimed. The AC-41, AC-119 and AC-126 harnesses are unmeasured, so their mutant
    controls may not fit 16 GiB. A witness phase stopped by the ceiling is `Inconclusive` and
    leaves the control unmet; no ceiling raise, smaller range or weaker assertion may stand in.
    The package comes from QSL's facade on source, never a copied fixture
    (AD-004 step 4a); if the facade cannot build it, the real-Kani control is the
    quire-integration exemplar of AD-004 L-5, a test in a repository above both.

    Current AC-40 controls: `qsl_v2_precondition_embeds_exact_referenced_clause_oracle`
    admits QSL preconditions for `and`, `or`, `not`, `implies`, Boolean `eq` and `ne`,
    and integer `eq`, `ne`, `lt`, `le`, `gt`, `ge`; it compares the embedded oracle
    bytes with `generate_v2_clause_oracle` for the same clause id.
    `inline_v2_precondition_embeds_exact_clause_oracle` makes the same comparison
    for a checked inline Boolean literal and checks its clause id.
12. Request a clause with an unsupported operator (division, modulo, absolute value), a
    node that is not a modelled clause role and an absent node id beside a supported item.
    The operator is `unsupported`, the other two refused as FR-015-AC-14 states, each
    keeps its reason and gets no harness (FR-015-AC-42). A clause value outside the three
    kinds and a non-Boolean body cannot reach this arm on an admitted package (Contract IR
    FR-040-AC-8 and FR-040-AC-10 refuse them at admission), so that guard is unreachable
    and not tested here. Request a postcondition without its sibling precondition
    (FR-015-AC-43).

    Current AC-42 control: `v2_unsupported_clause_operator_names_exact_identity_without_suppressing_sibling`
    admits separate division and modulo clause bodies with integer literal operands and a Text
    equality clause over Text literals, requests all three beside an admitted QSL bounded-parameter
    precondition, and checks one typed
    `V2ClauseOperatorNotSupported` per unsupported item with its exact clause node and operator
    identity, no harness for any of the three, and the supported sibling's one harness. The Text
    equality case exercises operator classification before clause operand reading. The traced
    `nested_absolute_value_names_its_exact_operator` unit test checks that a nested
    `quire.op.integer.abs` term is selected by the operator audit. The closed authoritative
    checked-operation catalog does not contain an absolute-value entry, so an admitted absolute
    clause cannot be constructed yet. The nonclause and absent-node request cases in this step
    remain separate FR-015-AC-14 evidence to complete.
13. Declare the four invalid censuses of FR-015-AC-22 and the three valid censuses of
    FR-015-AC-25 on a supported V2 clause claim, with the dependency list reordered. The invalid
    ones yield typed refusal and no harness. Parse every valid harness's persisted identity record:
    require the complete normalized census, `ready`, `incomplete` and `incomplete`, and
    `not_run`; reordering leaves its bytes unchanged (FR-015-AC-44, FR-015-AC-45).
14. Count the covers and locate each relative to the assumptions and the subject call; run
    a clause with jointly unsatisfiable assumptions; regenerate from equal inputs; vary the
    unwind bound, the subject and the source span while holding QSL's producer-issued
    structural identity fixed. The harness identity record changes with the unwind bound
    and subject; the carried structural identity remains unchanged. Compare claims with
    a changed structural node, model or producer ordinal: each has a different structural
    identity. Compare a changed source occurrence, kind, parameter declared name or domain,
    and state-field read declaring node, field name or domain (never a generated identifier), against an independently selected
    claim while holding structural identity fixed: each mismatch refuses before harness
    generation. The source span is excluded from the structural identity
    (FR-015-AC-46 to FR-015-AC-48).

    Current AC-47 generation-time controls: `qsl_v2_scalar_clause_identity_tracks_request_facts`
    regenerates authentic QSL precondition, postcondition and invariant harnesses from equal
    requests, then varies unwind and customer subject independently. Each equal request has
    identical generated source and persisted record bytes; each variation changes the harness
    identity and record. The invariant source calls neither customer subject. The authentic
    whole-object collection postcondition in
    `whole_object_identity_regenerates_and_changes_with_unread_domains` has the same repeatability
    and request-variation results. These controls do not claim AC-46 non-vacuity or AC-48 producer
    identity evidence. AC-48 evidence is `tests/it/v2_clause_obligation_identity.rs` and the two
    `whole_object_*`/`a_record_mismatch_*` tests in `tests/it/kani_collection_operation.rs`, whose
    records are written by hand from the fixture sources and never read back from the generator.
    The carried identity equals the supplied one for a changed node, model and ordinal; unwind and
    subject changes leave it unchanged while the harness identity changes; a source-span-only
    change leaves identity and harness record unchanged; a correct claim is accepted although the
    harness generates other identifiers; and a changed occurrence, kind, parameter name or domain,
    or state-field read declaring node, field name or domain refuses with the named record member
    and no harness, on the scalar route and on the whole-object collection route, including ahead
    of a render refusal. The sequence-field element domain and the NodeKey binding are specified
    by steps 52 (implemented, IR-1517) and 53 (implemented, IR-1518) below; the unread drawn field is evidenced by the AC-134 test above.
15. Feed `negotiate` a scalar refusal of each of `LoweringByteLimitExceeded` and
    `LoweringLimitUnrecognised` and read `OracleRefused` carrying it unchanged, field for field
    (FR-015-AC-50, IR-547).

## A cover in every harness kind (IR-464)

16. Generate a harness of each kind: precondition, V1 contract, scalar, state-clause,
    frame-effect and V1 bundle (the corpus kind is TC-023's). Parse each emitted source
    and read the last statement of every function attributed `#[kani::proof]` or
    `#[kani::proof_for_contract]`: it is the only `kani::cover!` of the body, and no
    assertion follows it (FR-015-AC-53).
17. Run the installed backend over a V1 bundle harness whose requires clause some
    bounded argument satisfies and whose `ensures` holds for every such argument, which classifies
    `Verified`; over the same bundle with its cover removed, which classifies
    `Inconclusive` as `MissingCoverSummary`; over the same bundle with a requires clause no
    bounded argument satisfies, which classifies `CoverUnsatisfied` (0 of 1 covers) and is
    neither `Verified` nor `Falsified`; and over the same bundle with a subject that breaks
    its `ensures`, which classifies `Falsified` under the historical V1 bundle procedure.
18. The guard: drive every emitting entry point (precondition, V1 contract, scalar,
    state-clause and frame-effect families, `generate_kani_bundle` and the corpus generator for each family) through the
    inspection of item 16, and scan the non-test string literals of `src/` for spellings of
    a proof attribute: `#[kani::` other than `requires`, `ensures`, `stub` and `unwind`,
    `kani::proof`, and `proof_for_contract`, with `\` line continuations joined, so a
    `format!` template, a split `concat!` and a continued literal are seen. The files that
    spell one are exactly the files the inspection drives, each with exactly the number of
    templates the guard counts, so a new emitter file and a new template in a driven file
    fail it until driven. A spelling built from fragments none of which holds `#[kani::` or
    `kani::proof` is not seen (FR-015-AC-58).

## A disposition for every state clause (IR-461)

19. Negotiate one request of `StateFrame` items over one admitted QSL-emitted package with
    selected model declarations: first a `precondition` clause (refused), then one clause in
    both roles that yields, one whose lowering is a `RequiresBound` record, one whose selected
    model field has no representable `i64` range, one comparing two fields, one whose frame
    grants a relationship and one whose frame grants every field. The `RequiresBound`
    case must come from lowering in that same admitted package; a separate request or
    a body-member target does not satisfy this mixed-disposition check. Read one record
    per item in request order, `kind`
    `postcondition` or `frame`, and each later record equal to the record the same item has in
    a request of that item only; repeat with three refused items and read three records. An
    implementation that stops at the first refusal reads one record and fails
    (FR-015-AC-59).
20. Compare the `supported` records' harnesses, read from `state_frame_harnesses` in request
    order, with `generate_state_frame_role` for the same clause and role (and, for a clause
    whose roles both succeed, `generate_state_frame_obligations`), source byte
    for byte (FR-015-AC-60).
21. Read `requires_bound` and the record's `unbounded_type` for a request whose lowering is
    a requires-bound record. For a selected model declaration's clause read of a native
    `Integer` field, verify lowering's `RequiresBound` wins before accessor resolution.
    For a present listed `Integer` field not read by that clause, verify value-based
    `NonRangeType` only if the item reaches accessor resolution. For a non-model or
    admitted unselected model/object_type body, read `ModelFieldsUnavailable` without a
    fabricated `unbounded_type` or a body range (FR-015-AC-61, FR-015-AC-78).
22. Read `unsupported` `NoFiniteEncoding` for a lowering unsupported-family record and
    `UnknownNodeKind` for a node that is not a `state_clause` (FR-015-AC-62).
23. Read `unsupported` `StateFrameRefused`, never `NoFiniteEncoding`, for a negation, a literal
    comparison, an operator outside the six comparisons, a read through another parameter, two
    fields, two reads of one side, a frame that creates, deletes or grants a relationship or a
    foreign field (naming the effect), an invariant clause, a clause with a malformed shape,
    a frame granting every field and an over-budget lowering. For a selected model
    declaration, check an accessor-absent field, a non-range member type, and an endpoint
    outside `i64`; for a non-model object and an admitted unselected model/object_type
    with a nonempty body, check `NotModelObjectType` and no harness. These replace the old body's
    `NotIntegerRange`, `EndpointOutsideI64`, `MemberAbsent` and `ValueNotReference`
    probes (FR-015-AC-63, FR-015-AC-78).
24. Read `invalid_request` for an unparsable state path, an unparsable item subject path, an
    invalid state field name, a state lacking the clause's field, an absent node, a repeated
    item and an item of another package, and `Rejected` with every record and no harness bytes
    (FR-015-AC-64).
25. Negotiate no items, `MAX_OBLIGATION_ITEMS` plus one, an unparsable request subject path and
    an unwind bound outside the range, each with `StateFrame` items present or absent as the
    case allows, and read the existing `KaniObligationError` variant with no record
    (FR-015-AC-65).
26. Build every retained `StateFrameRefusal` variant and call the mapping; read the table's
    record for each. Include `ModelFieldsUnavailable` for `UnknownNode`,
    `NotModelObjectType` and `AmbiguousField`, `MemberAbsent`, and
    `ModelMemberNotI64Range` with `NoMemberType`, `NonRangeType` and
    `EndpointOutsideI64`, plus each of the six `NotLowered` refusal arms. Do not construct
    retired body-member `BoundNotResolvedCause` cases. Build `UnwindOutOfRange`,
    `InvalidGeneratedSyntax` and `RecordSerialization` directly, and reach
    `ResourceLimitExceeded` with a state-field list that passes the 1 MiB ceiling or build
    it directly. Inspect that the mapping is a `match` with no wildcard arm (FR-015-AC-66).
27. Call the single-clause engine on a clause whose graph field name is not a Rust identifier
    and on a request whose caller-supplied field name is not an identifier: read
    `MalformedClause` and `InvalidField` (FR-015-AC-67).
28. Negotiate a clause whose frame grants every field in both roles, one whose condition is a
    negation in both roles, and one whose lowering is refused in both roles: read `frame`
    `unsupported` with `contract` `supported`, `contract` `unsupported` with `frame`
    `supported`, and both alike, comparing each `supported` harness with
    `generate_state_frame_role` and reading `generate_state_frame_obligations` return the
    refusal of the failing role (FR-015-AC-68).

The first code change backs steps 26 and 27, and the second backs steps 19 and 21 to 25; both
have landed. Crate-internal tests now back steps 20 and 28 by comparing each negotiated item with
`generate_state_frame_role` over an admitted V2 package, including exact supported harness bytes,
both one-role refusals and a lowering refusal that records both roles alike. They also compare the
two-role entry with the same fixture and carry trace tags for FR-015-AC-60 and FR-015-AC-68. The
broader disposition tests remain in `tests/it/kani_obligations_state_frame.rs`; the package they
negotiate holds twenty-four shapes, each over nodes of its own and read under a byte ceiling above
the default, which the corpus package alone nearly fills.

## Composite equality over a bounded shadow (planned, IR-264, IR-241)

29. Request a composite-equality claim over parameters of one record type, one tuple type and one
    option type (Boolean and bounded-integer leaves, an optional field, a nested option), under
    `Equal` and `NotEqual`: read one harness per item with family `composite_equality`, proof subject
    `bounded_shadow` and a refinement obligation in the same result, distinct from every other
    item's, with the operator and operand types from the descriptor and no bound from it
    (FR-015-AC-69).
30. Read the harness's drawn values (a `bool` or `i64` per leaf, the presence draws, the
    `present`/`absent`/`null` draw of an optional field), the assumption on each integer leaf, the
    pinned leaves of a literal operand, the order of the arguments by parameter node id then leaf
    path with each bound to both, and that no composite is an argument (FR-015-AC-70).
31. Read the harness's assertions against the expectation computed from the drawn values (verdict
    and pair count, `absent` against `null`, two `none`, `none` against present, a `some`/`some`
    option, no early exit, the negation under `NotEqual`) and that it ends with one cover after
    them (FR-015-AC-71).
32. Parse the shadow and the harness for loops, recursion and heap types; generate a closure one
    pair node under, at and one over `SHADOW_PAIR_NODE_BUDGET` and read the identity's budget and
    count (FR-015-AC-72).
33. Request one item reaching each row of the unsupported-shape table, beside one supported item,
    and read each disposition and reason, that no harness exists for the refused ones, and that
    the supported item settles as it does alone (FR-015-AC-73).
34. With the installed backend, run the corpus case's harness and its refinement run: read
    `Verified` with its cover satisfied inside the identity's ceilings, the versions and options in
    the evidence, and the refinement class `exhaustive` with its case count (the case's domain fits
    the cap) and agreement; run a second case whose domain exceeds the cap and read `sampled`
    (FR-015-AC-74).
35. With the installed backend, run each of the six shadow mutants, and natively each of the five
    production mutants (opposite operator, `absent` equal to `null`, one pair too many, an optional
    field declared required, a legal pair `Refused`) and the unmutated case: read `Falsified` with
    a playback for each shadow mutant, `refinement_failed` naming the first disagreeing case for
    each production mutant, and `Verified` with an agreeing refinement for the unmutated case. Run
    the closure-reader mutants (a dropped member, a bound narrowed by one, an optional member read
    as required) against FR-018-AC-23's independent check, which they fail, and permute the member
    order and read that no check is required to fail (FR-015-AC-75).
36. Read the identity's closure, domains, operator, budget, count, abstractions and unexercised
    behaviours with each covering criterion's backing state against the matrix rows, change each
    of the closure, a bound, the operator and the budget in turn, and regenerate from equal inputs
    (FR-015-AC-76).

Steps 29 to 36 are planned. Nothing in `src/` renders a composite harness: the corpus families
(arithmetic, graph, collection) draw no input, FR-025's argument table has rows for Boolean and
bounded integer only, and composite equality has an oracle (FR-018) and a native agreement corpus
(TC-029) but no Kani harness. The value of `SHADOW_PAIR_NODE_BUDGET` and the case cap's default are
set by the code change's measurement and recorded here.

## State field ranges from the package QSL emits (planned, IR-624)

37. Call `model_object_fields(&object_id)` for the QSL-emitted twin's model declaration
    object and read both `balance` and unread `audit` as `IntRange { lower: 0, upper: 1000 }`;
    convert their `i128` endpoints to `i64`, and compare the resulting `domains` with the
    state-clause replay's declared ranges. Admit a second unit with the same declaration but
    no read of `audit` and observe its range unchanged; cite IR FR-038-AC-140 for the
    independent post-admission body mutation (FR-015-AC-77; emitted path awaits CG's dependency
    update and implementation).
38. For a model declaration, request an accessor-absent field; exercise each accessor error
    (`UnknownNode`, `NotModelObjectType`, `AmbiguousField`) and check a typed refusal without
    falling back to a body or read. Exercise a present `None`, unread `Integer`,
    `Option`, and an `IntRange` with one endpoint outside `i64` in items whose
    lowering succeeds; read `ModelMemberNotI64Range` with the object,
    field and `NoMemberType`, `NonRangeType` or `EndpointOutsideI64` (exact `i128` endpoints)
    rather than a fabricated bound node. Check that only a present range with two `i64`
    endpoints enters `domains`, an unranged non-clause field is still drawn, and an unranged
    clause field refuses its item as `unsupported`, `StateFrameRefused`. For a non-model
    object and an admitted unselected model/object_type with a nonempty body, verify
    `NotModelObjectType` refuses without any assumption, `domains` entry or harness, even
    when the body contains an apparent range. Use an admitted QSL-emitted graph with
    selected-model-document override and recomputed digests for `None` and out-of-`i64`
    type cases QSL cannot emit directly. Separately submit a selected model/object_type
    whose nonempty body is read by the state clause and verify IR admission refuses
    `StaleNodeKey` before CG receives a package; do not expect a CG `NotModelObjectType`
    result from that rejected input (FR-015-AC-78; IR TC-227).
39. Generate both roles of each twin clause from the emitted package with no `Twin::aligned`;
    assert request order rather than accessor name order for `state_fields`, accessor ranges
    for `domains`, and scope ids equal to `call_site`'s. An absent accessor field, an omitted
    clause field and an omitted granted field each refuse (FR-015-AC-79).
40. With installed Kani, run the healthy, debiting, granted-write and forbidden-write subjects
    against harnesses generated from the emitted package (FR-015-AC-80).
41. Generate a harness with an unread present ranged field, and another with a present
    non-range field; check the first has a `domains` entry and no unranged reason, the second
    has no range assumption and records `TypeNotRange` in identity and persisted record.
    Regenerate for byte identity, and reject a record naming a field twice (FR-015-AC-81).

Steps 37 to 41 are planned. The IR-628 accessor has merged; the QSL-emitted model declaration
path awaits CG's dependency update and implementation. The hand-built non-declaration
body-member range path is retired; an agreeing read does not make that body trusted. The code change
replaces `tc_035_the_generator_reads_no_field_range_from_the_object_shape_qsl_emits`, updates
the typed accessor refusal and AC-66 mapping tests, and edits or supersedes the test of
FR-024-AC-30 that calls `Twin::aligned`; this PR restates AC-30 over the emitted-package
harness, so its previous test evidence remains planned until that migration lands.

## One renderer and the scalar family port (implemented, IR-858)

These steps verify AD-004 step 4b (AD-004 "The HarnessSpec and render contract (step 4b)"); they back FR-015-AC-112 to FR-015-AC-117. Steps 42 to 47
are numbered after step 41 and are independent of the IR-805 prose above.

Implementation evidence: the private constructor, renderer and scalar differential tests pass
(`cargo test --locked --lib -- --test-threads=1 kani::generate::spec:: kani::generate::render::
kani::generate::scalar::`), as does the twelve-entrypoint inspection (`cargo test --locked
--test it cover_last -- --test-threads=1`). The private installed-backend control is explicitly
run with `cargo test --locked --lib
kani_render_private::unsatisfiable_assume_is_cover_unsatisfied_under_real_kani --
--ignored --exact --test-threads=1`: it requires successful reachable checks and exactly
`CoverUnsatisfied { satisfied: 0, total: 1 }`. The existing healthy/mutated scalar control runs
with the required author `make kani-gate`; independent review and final acceptance remain separate.

42. Build `HarnessSpec`s by hand that break each invariant: no cover, two covers, a `true` cover
    condition, no call, two calls, an input after the call, an assertion before the call, a name bound
    twice, a name that is not a Rust identifier, an inverted domain, a domain outside its type, an
    `unwind` of 0 and of `MAX_OBLIGATION_UNWIND` plus one, a message containing each of `"`, `\`, `{`,
    `}` and a line break, and `ExprText`s that do not parse, spell `kani::`, hold a block, `unsafe`,
    closure, loop or `let` expression, or hold a macro other than `matches!` (`assert!`, `panic!`,
    a `kani` macro), a `matches!` with a guard, and a `matches!` whose expression holds a nested
    `panic!` or block. Read a typed `HarnessSpecError` for each and no spec; read that a `match` and a
    `matches!` expression construct (FR-015-AC-112).
43. Render valid hand-built specs whose statements are in several legal orders and read the cover as
    the last statement of the function in each; render the same spec twice, and in a second process,
    and compare the bytes; reorder two inputs and read different bytes; render a spec with non-scalar
    labels, expressions and messages and read that the source is the fixed structure plus exactly what
    the spec supplied (FR-015-AC-113, FR-015-AC-114, FR-015-AC-116).
44. Keep today's `render_scalar` body as a `#[cfg(test)]` reference function. Over the claim set of
    FR-015-AC-115 (four operations, a literal operand, an inferred operand, `i64` extremes, two unwind
    values including `MAX_OBLIGATION_UNWIND`, two ceilings values) compare the ported path with the
    reference: artifact path and bytes, identity, options and record, equal. There is no bless mode.
45. Drive a claim whose source exceeds `MAX_GENERATED_SOURCE_BYTES` and read `ResourceLimitExceeded`
    with the same byte count as the reference, and drive invalid generated syntax and read
    `InvalidGeneratedSyntax`, each raised by `render`, the size check first (FR-015-AC-115).
46. Run the FR-015-AC-58 inspection with `render.rs` among the driven files and `scalar.rs` out of the
    list; read the scalar harness's cover as its last statement (FR-015-AC-58, FR-015-AC-116).
47. With the installed backend, run the existing scalar lane tests (`tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic`
    and its healthy run): `Verified` with its cover satisfied, and the
    `Add` to `Subtract` mutation `Falsified`; then run a hand-built spec with an unsatisfiable `Assume`
    through `render` and read that it is not `Verified` (FR-015-AC-117).

## V2 Boolean implication mutation (IR-914)

48. Using the FR-014-AC-35 V2 Boolean oracle, generate a scalar `implies`
    claim over two symbolic Boolean operands. Inspect that the embedded oracle
    matches FR-014's function byte for byte, and that the generated subject calls
    `quire_contract_runtime::operators::implies_short_circuit` with `left` and a
    closure returning `right`. Inspect separate assertions that the subject
    result equals independent native `!left || right` over those operands and
    the oracle's `Outcome<bool>` is `Completed` with that expectation. Run the
    healthy harness with installed Kani and require `Verified` with its final
    cover satisfied. Parse the generated source to find the subject call, then
    replace only that call expression with `false`; fail the control if the
    call is absent. Run the
    mutated harness with the same claim and require `Falsified` with concrete
    Boolean-domain playback where `left = false`, while the embedded oracle and
    native expectation remain unchanged. Do not count a generation-time Boolean
    as a proof result (FR-015-AC-118).

## QSL-authored V2 clause runtime-helper mutation (IR-917; installed controls in FR-015-AC-151, pipeline subject planned)

49. Author a QSL operation with one finite-bounded integer parameter
    `amount: Int[0, 1000]`, a declared Boolean result, and its postcondition
    `result = (amount < 1000 implies amount + 1 <= 1000)`. Both bare `amount` reads
    name that operation parameter; no read names a state field or
    subject-produced post-state value. Emit its exact `CheckedPackageV2`
    through QSL's public facade,
    admit it through Contract IR, and select the postcondition by its clause
    node identity. Check that the FR-014-AC-38 oracle and FR-015 harness name
    that same clause, that the oracle is embedded byte for byte, and that the
    harness draws `amount` once under its declared domain, passes that exact
    value to the compiled subject, never draws or passes `result` as a parameter,
    and binds the subject's returned `bool` to the clause oracle's `result` input.
    Require the native arithmetic subterm probes, an assertion comparing that
    same subject result with independent native
    `!left || right` with `left = amount < 1000` and
    `right = amount + 1 <= 1000` over that same value, and a separate clause
    assertion that the oracle completes true for the produced result. Confirm
    the compiled postcondition and clause oracle use that same parameter value. Inspect the
    compiled subject to require a call to
    `quire_contract_runtime::operators::implies_short_circuit` and inspect
    the parsed authoritative helper body; fail before running either proof if
    the call or body is absent. Run the unmodified package, harness and subject
    with installed Kani and require `Verified` with its final cover satisfied.
    In an isolated checkout of `quire-contract-runtime`, change only the
    parsed body of that helper to return `false`, rebuild the same compiled
    subject against it, and run the same clause, oracle and harness again.
    Require `Falsified` with a concrete in-domain playback whose independent
    implication expectation is true. Compare the package, clause identity,
    oracle bytes, generated harness and non-helper subject source across the
    two runs; each must be unchanged. Keep the native expectation independent
    of both the oracle and helper, and reject generation-time truth values as
    proof evidence. Use no copied runtime file, binary or package fixture.
    This parameter fixture does not exercise the separate IR-624 selected-model
    state-field range path (FR-015-AC-119; distinct from the scalar
    FR-015-AC-118 and arithmetic mutant of step 11).

## QSL-source four-kind installed-backend run (IR-948, planned)

50. Author the FR-015-AC-129 fixture: the domain package declaring `Bank`, `Account`,
    `balance: Int[0, 100]`, `withdraw(amount: Int[0, 100])` and its frame, and the QSL unit with
    the named `pre`, `post` and `invariant` clauses. Emit and admit its one package. Request the
    four claims; require four harnesses with pairwise distinct identities, every drawn domain
    equal to the package's accessor range or `bounded_domain`, and no FR-021 prepared-function
    descriptor. With the installed backend, one job at a time at the AC-126 ceiling and unwind 8,
    require the unmutated precondition, postcondition and invariant `Verified` with the final
    cover satisfied (the frame's allowed effect by FR-015-AC-31). Run the two invariant mutants
    (`balance >= 1`; the package field widened to `Int[-1, 100]`) and require each invariant
    harness `Falsified` with counterexample `balance = 0` and `balance = -1`, replayed through
    `qsl_replay::replay_state_clause` as a reproduced violation of the same clause; credit no
    other kind's verdict of the widened package. Run the `amount > 100` precondition and require
    `CoverUnsatisfied`. Record the teardown of each job and its peak when the environment provides one (ADR-007);
    `WitnessDisagreed`, `NoVerdict`, `MissingCoverSummary` and timeout are `Inconclusive`, the
    control unmet, and the peak is reported (FR-015-AC-129).

## Exhaustive concrete enumeration (IR-1022 specified, IR-1509 implemented)

51. Build the FR-015-AC-119 clause crate from QSL source as in step 49, with no Kani launch, and run
    the host enumeration of FR-015-AC-132 over `amount_current = 0` through `1000`, evaluating all
    1001 values whatever fails: the sealed generated harness body as a host test (draw replaced by
    the value) and the independent `i128` comparison of the subject result. Healthy: require
    `sequence_exact` true, `checked` 1001, `passed` 1001, `failing_inputs` empty, `symbolic_proof`
    false and the other record fields of FR-017-AC-64. Then apply one mutant per run at its owner's
    site and rerun the same enumeration, requiring the exact `failing_inputs` set: in the compiled
    subject source, the consequent `amount + 2 <= 1000` gives {999}, `amount + 1 < 1000` gives {999}
    and the antecedent `amount <= 1000` gives {1000}; in an isolated quire-contract-runtime
    checkout, the body of `implies_short_circuit` returning `false` gives all 1001 values; in an
    isolated quire-exact checkout, `left.0.checked_add(right.0)` rewritten to
    `left.0.checked_add(right.0)?.checked_add(1)` and, separately, to `Some(left.0)` each give exactly {0, 1, ..., 999} through the generated probe. Run an
    enumeration that omits 999 and one that visits 998 twice and omits 999, and require each to
    fail the sequence check. Check the printed record and the status column of the FR-015-AC-132
    and FR-017-AC-64 rows of `tests.md` for the tokens listed in FR-017-AC-64 (none may appear)
    and for the phrase `enumeration evidence`; a record renderer printing `Proved` must fail. The
    symbolic runs of the same fixture are step 56. Implemented by the default-lane tests of
    `tests/it/kani_obligations_v2_clause_controls.rs` (the healthy enumeration, the subject, helper and kernel
    mutants, the sequence mutants, the record wording check); each mutant test names its exact
    `failing_inputs` set.

## AC-48 obligation-record gaps (IR-1515 spec; code IR-1517 and IR-1518 implemented)

52. Whole-object collection clause, FR-015-AC-133 (AC-134 is evidenced by the existing test
    named in its row). Implemented by IR-1517 in `tests/it/kani_collection_operation.rs`:
    `sequence_field_read_domain_is_cardinality_range_plus_element_domain`,
    `boolean_element_domain_is_the_pair_zero_and_one` and
    `nested_sequences_stay_refused_ahead_of_any_record_comparison`. The fixture (`element_document`)
    is the `Node` object whose `z_items` is a one-level ordered set of 0 to 3 elements each
    `Int[1, 9]` (cardinality range and element domain distinct, element minimum nonzero), read by
    the clause, beside the unread scalars `a_unread` (Boolean) and `m_count` (`Int[0, 3]`). Both
    sequence and scalars lower (`field_domain` admits a scalar element). Write every claim record
    by hand from the fixture source (never copied from a `V2ClauseClaimMismatch` the generator
    returned), and assert before the cases that the record lists the three reads, `z_items` with
    cardinality 0..=3 and element `Some(1..=9)` and each scalar with element `None`. Cases, each
    against the same independently selected clause: (a) control, the record at the declared
    domains is admitted and a harness is generated; (b) `z_items` element maximum 9 changed to
    8, cardinality unchanged: refuses `V2ClauseClaimMismatch` naming `StateReads`, no harness,
    and the refusal's `selected` diagnostic carries element `Some(1..=9)`, cardinality 0..=3 and
    `None` for the scalars; (c) `z_items` element changed to the Boolean pair `0..=1`,
    cardinality unchanged: same refusal; (d) `z_items` cardinality maximum 3 changed to 2,
    element domain unchanged: same refusal; (f) `z_items` element minimum 1 changed to 2,
    maximum unchanged: same refusal; (g) the cardinality range 0..=3 recorded as the element
    domain: same refusal; (h) `z_items` recorded with no element domain, and a scalar given
    `Some(1..=9)`: same refusal. The Boolean-pair case (`boolean_element_domain_is_the_pair_zero_and_one`)
    makes `z_items` a Boolean ordered set of 0 to 2: element `Some(0..=1)` is admitted and
    `Some(0..=2)` is refused. Nested sequences (`nested_sequences_stay_refused_ahead_of_any_record_comparison`):
    (e) a fixture adding a drawn field `grid` whose element is itself a sequence: refused as
    `CollectionClaimNotRendered` with `UnsupportedObjectState` ahead of any record comparison,
    with a record that omits `grid`; its one-level control (`grid` of `Small` elements) is the
    record mismatch instead; (e2) the same fixture with a quantifier over `grid`: refused with
    `UnsupportedFieldType`, likewise. Mutants, each applied alone and each reverted: M1,
    `operation_selected_reads` in `src/kani/generate/collection.rs` discarding the element
    (`None` in the `SymbolicBounds::Sequence` arm), fails 52(a), the control, and the Boolean
    case; M5, `element_domain` in the same file hard-coding the Integer element minimum to 0,
    fails 52(a) (selected minimum 0, record 1); M6, the same arm copying the cardinality range
    into the element domain, fails 52(a); M1b, `V2ClauseObligationClaim::mismatch`
    (`src/kani/identity.rs`) ignoring the element on both sides, fails 52(b), 52(c), 52(f) and
    52(g) and the Boolean case's wider-domain refusal while 52(a) stays green; M2, the same
    comparison comparing the element maximum only, fails 52(f) and leaves 52(b) green; M4, the
    `selected` diagnostic in `classify_v2_clause` (`src/kani/generate/negotiate.rs`) dropping
    the element, fails 52(b)'s diagnostic assertion (FR-015-AC-133). The mutant that selects
    only the fields the clause reads belongs to FR-015-AC-134 and is evidenced by that
    criterion's test.
53. NodeKey binding, FR-015-AC-135 and FR-015-AC-136. Author a package with two clauses A and B
    on different operations, request both with B listed first. Take each clause's expected key
    from a hand-written literal built from the package, node path and ordinal in the test (the
    literal, not the production resolver, so a mutated resolver cannot change both sides), and
    write each claim's record by hand. Cases: (a) control, A's claim carrying A's key with A's
    record is admitted and the carried identity equals the supplied one; (b) A's record with
    B's key: refuses `UnsupportedObligation::V2ClauseNodeKeyMismatch` naming A, no harness, and
    the refusal holds neither key nor a selection; (c) A's record with A's key but a different
    node ordinal in the `NodeKey`: refuses `V2ClauseNodeKeyMismatch`; (d) A's claim with B's key
    and also a changed state-read domain (the two-clause fixture has no parameters), and, on the
    single-clause parameter fixtures, a claim with another clause's key and a changed parameter
    domain: refuses `V2ClauseNodeKeyMismatch`, not `V2ClauseClaimMismatch`, while the same
    changed record under the resolved key is the record refusal; (d2) a clause whose lowering
    refuses (an unbounded second parameter) with another clause's key refuses
    `V2ClauseNodeKeyMismatch`, the resolved key reaching the lowering refusal; (e) the resolver-returns-none branch has no hand-built fixture that
    reaches classification (a clause node without a structural key fails admission earlier), so
    it is evidenced by the same comparison form `resolved.as_ref() != Some(&claim_key)` read in
    review, not by a runtime case. Mutants at the new comparison in `classify_v2_clause`
    (`src/kani/generate/negotiate.rs`): M4, skipping the comparison, fails 53(b), 53(c) and
    53(d); M5, resolving the key of the first requested clause instead of the selected clause
    (here B), fails 53(a) and 53(b); M6, resolving the key from the claim itself, fails 53(b);
    M7, comparing path only and not the node ordinal, fails 53(c); M8, running the record
    comparison first, fails 53(d) (FR-015-AC-135, FR-015-AC-136).
54. (Implemented, IR-1520.) State-frame identity member, FR-015-AC-137 to FR-015-AC-142. Take two
    hand-written identities I_c and I_f that differ in node, model and ordinal, and a third I_c2
    that equals I_c except for its ordinal. Cases: (a) `generate_state_frame_role` for each role
    with its identity: the harness's `obligation` equals it member for member; and
    `generate_state_frame_obligations` with the pair carries I_c on the `contract` harness and
    I_f on the `frame` harness (FR-015-AC-137). (b) Round trip: `StateFrameIdentity::from_record`
    of each harness's record returns an `obligation` equal to the generated one member for member
    (FR-015-AC-138), including a fourth identity I_n whose `node` path holds a `Node` segment that
    itself nests a key with a `Node` segment, which must round-trip equal and encode the segment as
    `{"node": ...}`. (c) A record with `obligation` removed, one with `node` removed and one with
    `ordinal` removed each return `StateFrameRecordError` (FR-015-AC-139). (d) Generate with I_c
    and then with I_c2: sources byte-identical, records differ only in `obligation`
    (FR-015-AC-140). (e) A negotiated request holding a `contract` item with I_c and a `frame`
    item with I_f for one clause (and, as a separate case, both items with the same identity I_c:
    both are carried unchanged and neither item is refused) returns each in `state_frame_harnesses` carrying its own
    identity; classifying the same item with I_c and with I_f gives equal results and the routed
    preflight builds no identity (FR-015-AC-141). (f) `StateFrameRoleIdentities::new(I_c, I_c)`
    returns `SharedIdentity`; `new(I_c, I_f)` and `new(I_c, I_c2)` build (FR-015-AC-142).
    Mutants and the assertion each fails: M1, `generate_state_frame_role` storing a default
    identity, fails (a); M2, `generate_state_frame_obligations` storing I_c on both harnesses (site:
    the pair splitting in that function), fails the `frame` equality of (a); M3, swapping the
    roles, fails both of (a); M4, the record writer dropping the `ordinal` (site:
    the record serialiser), keeps (a) and fails the round trip (b) and nothing weaker; M5, the
    record writer dropping the whole member, fails (b); M6, `from_record` defaulting a missing
    member (site: the parser), fails (c); M7, folding the identity into the harness source or
    symbol (site: the renderer), fails (d); M8, negotiate passing a placeholder identity to the
    item's harness (site: the `ObligationItem::StateFrame` arm), fails (e); M9, classification
    reading the identity so that results differ, fails (e); M11, a serialiser that flattens a `Node` segment to text or drops its nested key (site: the
    record writer's segment encoder), fails the I_n round trip of (b); M12, negotiate refusing or
    deduplicating equal identities across items (site: the `ObligationItem::StateFrame` arm), fails
    the same-identity case of (e); M10, the pair constructor comparing
    only `node`, refuses I_c2 and fails (f), and one accepting equal identities fails (f). Evidence: the tagged tests of `tests/it/state_frame_obligation_identity.rs`
    (`tc_025_each_role_harness_carries_its_own_identity` for (a),
    `tc_025_the_record_wire_form_round_trips_including_nested_node_segments` for (b),
    `tc_025_a_record_lacking_the_obligation_or_its_members_is_refused` for (c),
    `tc_025_the_identity_is_not_an_input_to_the_harness_source_or_symbols` for (d),
    `tc_025_negotiation_carries_each_items_identity_and_compares_none` for (e),
    `tc_025_the_pair_constructor_refuses_only_an_equal_pair` for (f), and
    `tc_025_a_key_nested_past_the_readable_depth_is_refused_when_written` for the writer's depth
    bound). Mutants M1 to M8, M10 and M11 to M12 were each applied at the named site and each fails
    the named test; M9 is not expressible, since `classify_state_frame_role` (the routed preflight's call) takes no
    identity parameter, so it cannot read one; negotiate's disposition is rendering, which can
    depend on the identity only through the 32-level node depth bound (FR-015-AC-137), pinned
    by `tc_025_a_key_nested_past_the_readable_depth_is_refused_when_written` (exactly 32 levels
    accepted and round trips, 33 refused as `RecordSerialization`) and
    `tc_025_a_record_nested_past_the_readable_depth_is_not_read` (the reader refuses 33). The
    bound mutants 32 to 40 and 32 to 31 each fail one of them.

## Tractable clause harness (IR-1022)

55. Native clause generation, FR-015-AC-143 to FR-015-AC-150. Generate the FR-015-AC-119 clause
    harness through the native route. Require the oracle's `ClauseStop` to be a `Copy` enumeration
    with no payload and `oracle_clause` to return `Result<bool, ClauseStop>`, the harness to
    assert `matches!(&outcome, Ok(true))`, and `core::mem::forget` of the meter after the assertions and
    the cover-condition read, with the single cover still the last statement (FR-015-AC-7), and of each
    arithmetic probe's outcome and meter. Control: the general oracle keeps
    `Runtime(rt::Outcome<bool>)`, and its harness also forgets the outcome (FR-015-AC-149).
    Mutants: restoring the `Runtime` variant in the thin stop type (site: `THIN_STOP_TYPE` in the
    clause generator), dropping the forget statements (site: `forget_fat_values` and the
    arithmetic probe body; the general-path outcome forget is the `!native` branch of
    `forget_fat_values`) and emitting a forget after the cover or the cover not last (site: the statement renderer)
    each fail the tests of this step. FR-015-AC-146 is evidenced by
    `generated_native_boolean_equality_matches_the_typed_route`, which compiles the helpers the
    generator rendered for `==` and `!=`; the mutant at the `related == {equal}` site of
    `src/oracle/scalar/render.rs` that never negates `!=` fails it. FR-015-AC-150 is evidenced by the
    same test (the out-of-domain stop) and by `native_conditions_stop_the_method_without_becoming_a_boundary`.
    Setup of the installed-Kani measurements (FR-015-AC-147 and FR-015-AC-148, recorded in
    `docs/qualification/ir1022-kani-clause-measurements.md`, a demonstration and not a matrix
    test): the generated crate for the harness above with a hand-written subject, raw
    `cargo kani -Z function-contracts --harness <module>::prove_clause --exact --unwind 3
    --solver cadical` inside a 24 GiB cgroup with swap 0; `Verified` for the healthy subject, then,
    with `-Z concrete-playback --concrete-playback print`, `Falsified` for the subject
    `amount + 2 <= 1000` (playback 999), the oracle `<=` to `<` and the runtime helper returning
    `false`; the cover witness printed with a failure is not a counterexample. The installed
    controls through CG's runner are step 56 (FR-015-AC-151).

## Installed clause controls through CG's runner (IR-1509)

56. FR-015-AC-151. Compile the FR-015-AC-119 clause package from QSL source, generate the sealed harness
    and, one run at a time in a 24 GiB cgroup with swap 0 and a 30 minute cap, run CG's
    `execute_kani_obligation` over a crate holding the sealed artifact and the `crate::subject` module.
    Healthy: require `Verified` from one proof-phase launch. Then one run per mutant, each requiring
    `Falsified` from the witness phase with the proof phase nested, `concrete-playback` only in the witness
    vector, equal ceilings across phases, one harness path and an in-domain playback: in an isolated
    copy of the locked quire-exact checkout `left.0.checked_add(right.0)` rewritten to
    `left.0.checked_add(right.0)?.checked_add(1)` and separately to `Some(left.0)` (failing check: the
    generated arithmetic assertion, playback below 1000); in the compiled subject the consequent
    `amount + 2 <= 1000` (the produced-result comparison, playback exactly 999); in an isolated copy of
    the locked quire-contract-runtime checkout the body of `implies_short_circuit` returning `false` (the
    produced-result comparison). Append the 999 playback test inside the generated module before its
    closing brace, require that removing the appended bytes restores the sealed artifact exactly and that
    regeneration reproduces the artifact and record, run it on the host against the plus-one kernel mutant
    and require it to pass as an expected panic on the generated arithmetic assertion's message; run the
    same test against the unmutated owner and the `amount + 2` subject and require each to fail. Executed
    verdicts are in `docs/qualification/ir1509-kani-clause-controls.md`. Require the `amount + 2` playback
    control to fail at the produced-result comparison, not at a preamble. The Kani-backed playback through
    `execute_kani_concrete_playback` is TC-027 step 5 (FR-017-AC-60, IR-917; `docs/qualification/ir917-kani-playback.md`).
    PLANNED: the subject compiled by the pipeline (blocked on QSL operation bodies).

## Control construct lowering (IR-1447)

57. FR-015-AC-152 to FR-015-AC-173 (PLANNED, IR-1447; FR-025-AC-19 is TC-036 step 19). Control construct lowering.
    Generate a native V2 clause harness for each fixture of FR-015-AC-152 to FR-015-AC-155,
    FR-015-AC-164, FR-015-AC-170 and FR-015-AC-173 (`let`, `if`, `case` over a three-member arity-0
    union drawn as an `i64` member index, an `if` in an invariant collection `forall`, the nested
    `let`/`if`, and the shadowing and nested-rebinding `let`, the last two as constructed packages
    because QSL refuses shadowing) and require one harness each, the native `let`, `if` and `match`
    forms (no wildcard arm for the `case`), the union parameter bound as one `i64` (FR-025-AC-19) and
    the independent `i128` expectation, with each fixture's arithmetic-node bounds as its criterion states them
    (the kernel checks an integer result only against the arithmetic node's own bound, never an `if`, `let` or
    `case` node). Then, one Kani job at a time, run each fixture through CG's
    `execute_kani_obligation` (FR-017-AC-63) and require the healthy subject `Verified` with the
    cover satisfied, and each mutant `Falsified` from the witness phase at the step `the produced V2 Boolean result agrees with the independent checked condition` with the
    named input: `let` subject `amount + 2 <= 1000`, `amount = 999`; `if` subject test
    `amount <= 500`, `amount = 500`; `case` subject arm `B` as `amount * 3`, member index 1 and
    `amount = 400`; nested subject test `amount <= 500`, `amount = 500`; shadowing subject
    `amount <= 1000`, `amount = 999`; nested-rebinding subject `amount + 1 <= 1000`, `amount = 999`.
    The collection fixture has no subject: its mutant is the generated oracle's element `if` with
    else branch `501`, falsified at the direct harness's assertion that the admitted body holds,
    with a collection holding an element of 500. Then run the laziness and refusal fixtures:
    FR-015-AC-168 (`amount: Int[0, 999]`, the never-selected else branch `amount + 1000`: verifies,
    and the eager oracle is falsified at `the V2 postcondition holds for the produced result` with amount 0), FR-015-AC-169
    (`amount: Int[0, 1000]`, the selected else branch `amount + 1` at amount 1000: falsified at `the V2 postcondition holds for the produced result`
    with the recorded stop `ClauseStop::IntegerOutOfDomain`, and the `false`-mapping mutant told
    apart only by the recorded stop) and FR-015-AC-172 (the unread refusing `let` initializer at
    amount 1000, with the skip-unread mutant verifying). Then the refusal fixtures of
    FR-015-AC-160 to FR-015-AC-163 and FR-015-AC-171 (`unsupported` with
    `V2ClauseControlNotSupported`, the exact control reason `LetValueNotNativeScalar`,
    `IfBranchNotNativeScalar`, `CaseUnionPayloadNotSupported`, `CaseArmNotNativeScalar` or
    `CaseScrutineeNotParameter` (the last from a union state field scrutinee), the node and
    operation identity, no harness, and an unaffected supported sibling) and the precedence
    fixtures of FR-015-AC-166 (an unsupported operator before a failing control, a failing control
    before an unsupported operator, a `let` binding an option-typed parameter expecting
    `LetValueNotNativeScalar`, a `let` binding `quire.op.option.value` of an Integer result decided
    by FR-054, two failing controls) with the last-refusal-met mutant, and the collapse mutant of
    FR-015-AC-167. The probe that fixed the tractability expectation is
    `docs/qualification/ir1447-kani-control-probe.md`.

## Blocked

- Frame harnesses in the clause negotiation: FR-025 accounts every frame obligation
  `unsupported` there until QSpec decides how a frame node lowers into a Kani form (ADR-004). The
  frame of a `postcondition` state clause over one integer field is instead generated with that
  clause by `generate_state_frame_obligations` (FR-015-AC-26 to FR-015-AC-32).
- V2 scalar harnesses outside `IntegerArithmetic`: an IR-confirmed claim over
  one of the four `quire.op.integer.{add,sub,mul,negate}` identities reaches a
  real harness unless a ground independent of the operation (an
  i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
  confirmed family is refused as `OperationNotRendered`, an unbuilt renderer in
  this generator, not an upstream block, and so is a claim of a rendered family
  from a claim map this generator did not produce (NFR-005-AC-2). A claim this generator lowered but
  whose operation it did not confirm against the node's own catalogued
  identity, mode or law definition is refused as `CallerDeclaredOperation`.
- Model and graph bounds: refused as blocked on `agent-ix/quire-spec-language#120`.
