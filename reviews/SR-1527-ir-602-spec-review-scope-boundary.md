---
id: "SR-1527"
title: "CG PR 281 scope boundary: whole-oracle quire_exact migration, lock and cross-repo prerequisites"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-codegen@607a975f0ab00aab0730539b75ac506a1380dbaa; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (Scope, The kernel the native oracle calls, Outputs, Open decisions OD-1 to OD-5, AC-22); Cargo.lock and Cargo.toml at origin/main 6a6e721; QSL main c5fcc438; quire-exact 2ec5e1e; Contract Runtime ccc722b and main 9597b43"
---

# SR-1527: CG PR 281 scope boundary

## Summary

Ticket: IR-602. IR-602 asks for the held divide and remainder criteria. The owner's comment on
IR-602 asks CG to depend on agent-ix/quire-exact directly and to call `quire_exact::divide`,
not Contract Runtime, for division. That comment is ticket data, and I quote it here as
untrusted.

Decision (2) moves the whole native oracle onto `quire_exact`. On the type-identity argument I
agree it is forced once division calls `quire_exact::divide`. `divide` needs a
`&mut quire_exact::Meter`. A call-site conversion would have to build a second meter and copy its
charges back, and it would have to map `DivisionOutOfDomain` onto a runtime refusal vocabulary
that has no such variant (RT has only `DivisionPairOutOfDomain`). That is a bridge between two
meters and two refusal vocabularies, which makes it a compatibility layer, not ordinary
adaptation. Converting `i64` operands into the kernel's `Integer` is ordinary adaptation, but it
does not remove the meter problem.

The end state also agrees with Contract Runtime FR-275: "generated oracles and the code generator
name the owning crates' own paths". So the only smaller correct alternative is the one OD-2
records: hold native division until IR-349 or RT#95 (open) makes Contract Runtime's types
`quire_exact`'s.

What remains is a scope and sequencing question, plus two prerequisites the spec misses.

Settled or recorded, by decision:

- OD-1 (law) may stay recorded, to be confirmed with IR and QSL. Truncating matches IR's own
  i128 preimage and Rust's operators.
- OD-2 must be settled by the owner (and the IR-349 owner) before this merges (FND-002).
- OD-3 can be closed from evidence (SR-1523 FND-003).
- OD-4 is QSL's decision and is recorded correctly.
- OD-5 is false (SR-1523 FND-001).

The lock and deny story is right as far as it goes. quire-exact is first-party and public, it is
spelled `branch = "main"`, and it is already in deny.toml's `allow-git` and in the
`check_one_copy.awk` scope. Byte identity (AC-12) is unaffected, because an arithmetic-free
oracle emits no `rt` header.

## Verdict

Changes requested. FND-001 and FND-002 block. FND-003 is a spec gap that the code would hit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-22 cannot be met yet, and the spec does not say why. CG's lock has one `quire-exact` entry, at a270df3. It is shared with `qsl-replay` and `qsl-semantics` (QSL c8f0c28, the lock; QSL main c5fcc438 is the same on this point). `qsl-semantics/src/value/definition.rs:34,543` and `qsl-foundation/src/diagnostic.rs:1039` call the pair `divide`, `QuotientRemainder` and `DivisionPairOutOfDomain`, all of which quire-exact 2ec5e1e deletes. Bumping the one entry to pick up the single-member `divide` therefore breaks the build of CG's QSL dependency, and the one-copy rule forbids a second entry. The precondition is a QSL migration to the single-member `divide` plus a CG bump of QSL. Name it as a dependency or blocker of the code change. | FR-031 "The kernel the native oracle calls", AC-22 (FR-031:477); Cargo.lock:1315-1350,1440-1442 |
| FND-002 | medium | Decision (2) is written as normative "shall" text: the whole native oracle on `quire_exact`, AC-8 and AC-21 retargeted, and AC-22. Yet OD-2 says the same decision is "To confirm with the IR-349 owner and the owner". It moves the add, subtract, multiply and comparison calls that IR-596 implemented ahead of IR-349's sequencing (RT#95 is open). That is a planner or owner question, not something to state and then defer. Settle OD-2 with the owner before merge, or make the migration criteria contingent on it. | FR-031:252-266,504-508 |
| FND-003 | medium | The migration is larger than "the same calls in `quire_exact`, with the same meaning". The native oracle renders short-circuit connectives as `quire_contract_runtime::operators::{and,or,implies}_short_circuit<R: From<bool>>` with an `rt::Outcome<bool>` closure (CG boolean_v1.rs:1006,1191,1194). That compiles only because Contract Runtime has `impl<T> From<T> for Outcome<T>`. `quire_exact::Outcome` has no `From` impl (src/outcome.rs:303 is the only one, for `Stop`), and the orphan rule bars the generated crate from adding one. Z-3 (`y <= 0 && x % (y + 11) <= 10`) takes exactly this path. The spec should state the connective rendering change, for example an inline match or `quire_exact::evaluate_boolean`. | FR-031:252-266; FR-031 Outcomes and their propagation |

## Dispositions

Round 1, reviewed at f27977083e292d1334aa7a92c63f570af305b532 (fix commit f279770 plus a merge of main 735e704).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f279770: a new subsection, "Prerequisite: the one `quire-exact` lock entry", sets the order: QSL moves first, then CG bumps QSL and quire-exact together. Gating notes appear in Status, Scope, Dependencies, AC-2, 6, 16, 17, 22, 25 and 26, the matrix rows, the TC-044 Description and step 14, and spec/tests.md. Re-measured: the CG lock is still at a270df3; QSL origin/main (now bcca4335, newer than the spec's c5fcc438) still uses `QuotientRemainder` (qsl-semantics/src/value/definition.rs:34,543) and `DivisionPairOutOfDomain` (qsl-foundation/src/diagnostic.rs:1044). The gap in gating AC-18 is a new finding, SR-1524 FND-004. |
| FND-002 | fixed | f279770: "The kernel the native oracle calls" is marked contingent on the prerequisite and OD-2. AC-22 and AC-26 are CONTINGENT on OD-2. OD-2 is "for the owner and the IR-349 owner to decide", and the migration is no longer stated as a settled rule. OD-2 still has to be decided before the IR-602 code lands, but the spec no longer pre-empts it. |
| FND-003 | fixed | f279770: the spec records the short-circuit rendering change. Re-measured: `quire_exact::Outcome` has no `From<bool>`. quire-exact exports `evaluate_boolean` and `retain_boolean`, both charging `boolean.result-retain`, and no short-circuit helper. RT `operators::*_short_circuit` and `*_total` are pure and charge nothing. AC-26 can fail: a source check for any `operators::` path, a no-charge check through the meter, and outcome equality on the AC-9 cases and Z-3. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | AC-26 requires connectives that charge nothing. That keeps IR-596's RT behaviour, but it departs from the kernel's own documented convention: `quire_exact::retain_boolean`'s rustdoc says "a caller's own `and`/`or` short-circuit evaluation ... retains its already-decided `bool` through this function", which charges `boolean.result-retain`. So whether native-oracle metering should match the kernel and QSL's evaluator is an open decision, not a fact. Record it as one, for QSL, or state why the oracle's meter need not match. | FR-031 "The kernel the native oracle calls" (connective bullet), FR-031-AC-26; quire-exact 2ec5e1e src/numeric.rs:402-428 |

Round 2, reviewed at 41526f7be8c13e6b835dbef253b508297de6f6a5 (fix commit 41526f7).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 41526f7: OD-6 (connective metering, for QSL to decide) is added. It cites `quire_exact::retain_boolean`'s rustdoc, which I verified at quire-exact 2ec5e1e src/numeric.rs:412-428: a caller's own `and`/`or` short-circuit "retains its already-decided `bool` through this function", charging `boolean.result-retain`. OD-6 says AC-26 and its unchanged-outcome clause change if QSL requires metering parity. AC-26 is already CONTINGENT (OD-2 and the prerequisite), so it no longer pre-empts the decision. |
