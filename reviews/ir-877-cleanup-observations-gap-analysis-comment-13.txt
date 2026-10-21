<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 13/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-217 | high | coverage | Inherited FR-034-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-218 | high | coverage | Inherited FR-034-AC-16 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-219 | high | coverage | Inherited FR-034-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-220 | high | coverage | Inherited FR-034-AC-18 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-221 | high | coverage | Inherited FR-034-AC-19 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-222 | high | coverage | Inherited FR-034-AC-20 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-223 | high | coverage | Inherited FR-034-AC-21 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-224 | high | coverage | Inherited FR-034-AC-22 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-225 | high | coverage | Inherited FR-034-AC-23 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-226 | high | coverage | Inherited FR-034-AC-24 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-227 | high | coverage | Inherited FR-034-AC-25 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-228 | high | coverage | Inherited FR-034-AC-26 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-229 | high | coverage | Inherited FR-034-AC-27 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-230 | high | coverage | Inherited FR-034-AC-28 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-231 | high | coverage | Inherited FR-034-AC-29 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-232 | high | coverage | Inherited FR-034-AC-31 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-233 | high | coverage | Inherited FR-034-AC-32 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-234 | high | coverage | Inherited FR-034-AC-33 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-029-AC-30
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive or cover-unsatisfied)
    reaches only the composite parity route on the production path and never `run_terminal_value`, because
    `KaniRunOutcome` cannot tell a shadow from a production proof and a verified shadow outcome there
    would read as `Proved`, which FR-028-AC-17 forbids.
- id: FR-029-AC-31
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). The parity route fabricates no predicate replay and no terminal cause for
    a bounded-shadow outcome: its terminal value is the one QSL''s report carries under AC-17 and AC-19
    to AC-27.'
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `out_of_memory` maps to `Incomplete(ResourceExhausted)`,
    and the same reason with cause `properties_failed`, `exit_code` (any code), `unknown` or no cause
    maps to `Failed`. Over a single-run stand-in that exits 1 and exports a `Failure` report for `proofs::check`
    with no checks and one `error_details` entry with exit status `out_of_memory`, the evidence passed
    to `run_terminal_value` gives `Incomplete(ResourceExhausted)`; the same report with exit status `exit_code_64`,
    `cbmc_exploded`, `properties_failed`, a null status or no `e
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `timeout` maps to `Incomplete(TimedOut)`:
    the single-run stand-in of FR-029-AC-32 with exit status `timeout` gives `Incomplete(TimedOut)`. The
    batch member `c::check` of FR-017-AC-39 (a `Failure` with no checks and exit status `timeout`, classified
    `TimedOut`) keeps `Incomplete(TimedOut)` through the timed-out row, and the member `d::check` (a `Failure`
    listing a failed check with no playback, exit status `timeout`, classified `FailedWithoutCounterexample`
    with cause `timeout`) gives `Incomplete(TimedOut)` through the cause.'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). CG''s own memory stop (the memory-exhausted reason) and a `FailedWithoutCounterexample`
    run with cause `out_of_memory` give the same terminal value, `Incomplete(ResourceExhausted)`, so one
    exhaustion has one value whoever observed it. The cause is read for the failure-without-counterexample
    reason only: every other outcome and reason maps identically whatever cause is passed, including a
    cause passed with `NoVerdict` or `MissingCoverSummary`.'
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). The cause never creates a verdict. Over every combination of outcome, reason
    and cause (absent, `properties_failed`, `timeout`, `out_of_memory`, `exit_code`, `unknown`), and with
    the replay settlement each outcome takes: a `falsified` outcome with a reproduced replay is `Refuted`
    whatever the cause; the cause never turns a non-`Refuted` result into `Refuted`, and never turns any
    result into `Proved` (`Proved` appears only for the rows FR-029-AC-1 and FR-029-AC-2 give). The `NoVerdict`
    and `MissingCoverSummary` reasons map to `Failed` whatever the cause. Classification is unch'
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The rationale of this requirement states that Kani reports any `SIGKILL`
    of CBMC as `out_of_memory`, that an `out_of_memory` cause is accepted as `Incomplete(ResourceExhausted)`
    without corroboration, and that the raw cause stays in the evidence.
- id: FR-030-AC-1
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every pair (`KaniOutcomeKind`, replay settlement) that the map's input can express maps to
    exactly one `TerminalValue`.
- id: FR-030-AC-2
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with `ProofRefusalCause::Refused`,
    `InvalidInput` and `IncompleteInput` respectively, so no refusal cause is lost, and each carries the
    outcome''s `Std001Code` unchanged as `DeclineCode::Std001`, including a code STD-001 does not register.'
- id: FR-030-AC-3
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`TimedOut` and `ResourceExhausted` map to their distinct `Incomplete` causes. `Cancelled {
    source: Requested }` and `Cancelled { source: Deadline }` map to `IncompleteCause::Cancelled` carrying
    the corresponding `quire_exact::CancelCause`, with no source inferred from `code`, `source_id`, `context`
    or elapsed time.'
- id: FR-030-AC-4
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Proved` with three SUCCESS checks maps to `Proved { basis: Checks { success_checks: 3 },
    certification: Certified }`, zero checks maps to `Proved { basis: Checks { success_checks: 0 }, certification:
    Certified }` (category inconclusive/vacuous), and `Counterexample` with reproduced replay maps to
    `Refuted`.'
- id: FR-030-AC-5
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis: Checks { success_checks:
    0 }, certification: Certified }` (category inconclusive/vacuous), and `Inconclusive` with any other
    cause maps to `Failed`.'
- id: FR-030-AC-6
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: No outcome maps to `Tested`.
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard
    arm.
- id: FR-030-AC-8
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Unavailable` with cause `kani_checker_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent`
    or any other cause it maps to `Unsupported(BackendAbsent)`.'
- id: FR-030-AC-9
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`, and with
    a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()`
    of that refusal.'
- id: FR-030-AC-10
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a fault, walked through every wrapper FR-029-AC-10 lists, and with each
    CG-raised failure FR-029-AC-11 lists, maps to `Failed`.'
- id: FR-030-AC-11
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `Counterexample` maps to a value other
    than `Refuted`.
- id: FR-030-AC-12
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each
    bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal''s QSL catalog code,
    never to `Declined`.'
- id: FR-030-AC-13
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity`
    refusal (code `invalid_package`), as a lock whose only defect is a repeated library identity produces
    it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`.'
- id: FR-030-AC-14
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value.
- id: FR-030-AC-15
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same rows from a CG-built
    request over the original proving context). For an identity-valid IR `Counterexample` composite claim,
    F-1 Disagreed yields Failed/CgDefect even with a native fault or invalid operand; without Disagreed,
    native Incomplete and ExecutionFault each yield a binding-checked GeneratedFault/Failed report with
    the actual NativeCause even when an operand would refuse, admission or exact limits would stop, or
    refinement is CeilingReached. CG asserts the typed result and terminal value; QSL FR-358 owns the
    inte
- id: FR-030-AC-16
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 direct public report controls; IR-806 must reach the same rows from a CG-built
    request over the original proving context). With valid identity, no Disagreed and Completed native,
    an out-of-domain operand yields F-3 RefusedInput/ReplayRefused with its index and QSL code even with
    CeilingReached; a request accounting limit reached during admission yields F-4 Incomplete(ResourceExhausted)/Admission
    with the counter; an exact limit reached after admission yields F-5 Incomplete(ResourceExhausted)/ExactEvaluation
    even with CeilingReached; with sufficient limits, CeilingReached yields
- id: FR-030-AC-17
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: PARTIAL (IR-666 F-7 and binding controls; IR-806 original-artifact path planned). With valid
    identity and all earlier rows absent, changing only the retained shadow verdict or pair count yields
    F-7 Diverged/Failed/CgDefect; agreement yields Inconclusive(ScalarAgrees) with the CompositeEquality
    claim and Equality outcome. A binding-valid QSL `prepare` non-fault `Refused` report precedes Disagreed
    and yields Inconclusive(ReplayRefused) with QSL's code. `Refused` from later admission or exact comparison
    follows Disagreed; if QSL returns `Fault` or `Admission(Fault)`, its bound report yields Faile
- id: FR-030-AC-18
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every Kani `Proved` row carries `ProofBasis::Checks` with the transcript count and `Certification::Certified`,
    including the zero-check row; zero checks remain category inconclusive/vacuous. No Kani non-`Proved`
    row carries basis or certification.
- id: FR-030-AC-19
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT proof whose certificate
    FR-314 verifies maps to `Proved` with the same basis/depth and `Certified`; no certificate or an `Unverifiable`
    unchecked rule maps to `Proved` with the same basis/depth and `Uncertified`, retaining the unverifiable
    locus when present. No SMT proof maps to `Trusted`.
- id: FR-030-AC-20
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid checked proof step
    or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected { rule,
    at })` retaining the exact rejection; none produces `Proved`, basis or certification. When cvc5 and
    Z3 also disagree, `CertificateRejected` remains the one terminal cause while the parity failure and
    both verdicts are retained alongside it.
- id: FR-030-AC-21
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts and settles
    inconclusive; agreement does not change certification; missing Z3 retains its warning and cvc5 settlement.
    An SMT counterexample becomes `Refuted` only after FR-197 replay reproduces it; replay disagreement,
    non-fault refusal and fault map to their distinct non-proof results.
- id: FR-030-AC-22
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces one terminal result,
    never `Tested`; no non-`Proved` result carries basis or certification. Negotiation refusals produce
    no terminal from this map, and neither SMT nor Kani map consumes the other backend's outcome.
- id: FR-031-AC-1
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). For each of Add, Subtract and Multiply over a `reject` integer type,
    the generated source of the native oracle calls `exact::evaluate_integer_arithmetic` with the matching
    `IntegerArithmetic` variant and the type's interval as the bound, and the oracle's body holds no Rust
    arithmetic operator between operand expressions and none of `checked_`, `wrapping_`, `saturating_`
    or `overflowing_` method calls.
- id: FR-031-AC-2
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder over a `reject`
    integer type, the generated source of the native oracle calls `exact::divide` with `DivisionProfile::Truncating`,
    with `DivisionMember::Quotient` for a divide and `DivisionMember::Remainder` for a remainder, and
    with `IntegerDomain::Bounded` over the node's interval as the domain, and the oracle's body holds
    no Rust `/` or `%` and none of `checked_div`, `checked_rem`, `wrapping_rem`, `div_euclid`, `rem_euclid`
    or `modulo`, and no division of a quotient and remainder pair.
- id: FR-031-AC-3
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns `Outcome<bool>` and takes
    a trailing `&mut Meter`; an oracle with none returns `bool` and takes no meter.
- id: FR-031-AC-4
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free operands is emitted
    as the one native operator the table names, and over the seven values `i64::MIN`, `i64::MIN + 1`,
    `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, with two interior values added, taken pairwise in both
    orders, agrees with the exact kernel's own evaluation, as the test takes it from Contract Runtime
    today (`order_numbers` for the four orderings, `check_equality` then `CheckedEquality::evaluate` for
    equal and not-equal; after the migration of FR-031-AC-22, `quire_exact`'s `order_numbers` and `planned_equ
- id: FR-031-AC-5
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract or multiply result outside
    the type''s interval evaluates to `Refused(IntegerOutOfDomain)`, and the oracle neither panics nor
    returns a wrapped, saturated or clamped value. The cases are O-1 `i64::MAX + 1`, O-2 `i64::MIN - 1`
    as a subtract, and O-4 `i64::MIN * 2`, each built as TC-044 step 4 gives. On the tree before the change
    each fails: a debug build panics with an overflow message, a release build returns a wrapped value,
    and no case returns the refusal.'
- id: FR-031-AC-6
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type, a zero divisor
    evaluates to `Undefined(DivisionByZero)` for both Divide and Remainder, for the cases Z-1 `x / 0`,
    Z-2 `0 / 0`, Z-3 `x % 0` and Z-4 `0 % 0`, each built as TC-044 step 5 gives (Z-3 and Z-4 reach the
    zero through the add `y + 11`, which completes with 0), and the oracle does not panic and does not
    return `Refused`. On the tree before the change each case is refused at generation with the interim
    code, and a raw operator would panic with a division-by-zero message.
- id: FR-031-AC-7
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer type is refused
    with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact, for the rows `x + 1`
    at `x = i64::MAX` and `x - 1` at `x = i64::MIN` over an integer type of `i64::MIN..=i64::MAX`, and
    `x * 2` at `x = i64::MAX` over `0..=10` (where an in-domain `x = 10` also gives the wrong value `20
    <= 10` against the policy's `10 <= 10`); the refusal span is the first such node in authored preorder
    when a negation also occurs later, and the diagnostic message names the missing runtime saturating
    operation
- id: FR-031-AC-8
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). In a generated crate compiled and executed against `quire-contract-runtime`,
    the outcome of a native add, subtract or multiply oracle equals a plain-integer model of IR FR-015''s
    member-range semantics, computed in the test with `i128` independently of the emitter and of any `exact::`
    call: `Completed` with the exact result when it lies in the type''s interval and `Refused(IntegerOutOfDomain)`
    when it does not. The grid covers operand values inside the declared domain, at its edges and outside
    it, and the outcome kind is compared, so no vector returns `Completed` where the '
- id: FR-031-AC-9
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order is the oracle''s
    outcome: an arithmetic stop in the left operand of a short-circuit connective is returned, a stop
    in a right operand the left decided is never reached, a total connective returns the first stop of
    its two operands, and a stop is never returned as `Completed(true)` or `Completed(false)`.'
- id: FR-031-AC-10
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The tri-state harness generator, the bound strategy generators and the
    Kani obligation clause lowering each refuse a clause holding an arithmetic node with the refusal the
    Consumers section names for it, emit no raw arithmetic operator, and never read an `Outcome<bool>`
    as `bool`.
- id: FR-031-AC-11
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause that runs QSL's IT-011
    and IT-010-SC-05 against this branch is pending the quire-integration run; CG's in-repo run of the
    same bundle shape under real Kani is the closest equivalent and does not stand for it. `generate_kani_bundle`
    over the postcondition `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000`
    under `reject`, generates a bundle whose postcondition oracle returns `bool`, holds exactly one syntactic
    addition (`syn::ExprBinary` with `BinOp::Add`) and no `exact::` call, and, run wit
- id: FR-031-AC-12
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Apart from the generated Rust header lines required by NFR-002, the oracle
    generated for an expression with no arithmetic node is byte-identical to the oracle the generator
    produced for it before this requirement. This criterion passes on the tree before the change and is
    a held agreement, not a defect proof.
- id: FR-031-AC-13
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic node twice,
    and from a permuted request, yields identical bytes. This criterion passes on the tree before the
    change and is a held agreement, not a defect proof.
- id: FR-031-AC-14
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic node contains no
    `unwrap`, `expect` or panic macro. This criterion passes on the tree before the change and is a held
    agreement, not a defect proof.
- id: FR-031-AC-15
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons that has an arithmetic
    operand is emitted as the exact-kernel call the table names (for Equal and NotEqual, until the migration
    of FR-031-AC-22, the checked equality of Contract Runtime that the Comparison section records), with
    no Rust comparison operator between its two operands.
- id: FR-031-AC-16
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type of `-5..=5`,
    the minimum divided by negative one is out of domain and the minimum remainder negative one is not,
    and neither panics: O-3 `i64::MIN / -1` evaluates to `Refused(DivisionOutOfDomain)` whose member is
    `Quotient` and whose domain is `-5..=5`, with the refusal''s code `division_out_of_domain` and cause
    `quotient-outside-domain`; O-5 `i64::MIN % -1` evaluates to `Completed(true)`, the remainder being
    the exact 0 and `0 <= 5` true. Each is built as TC-044 step 4 gives. On the tree before the change
    each cons'
- id: FR-031-AC-17
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate, the outcome of a divide
    or remainder oracle equals a plain-integer model of IR''s member-only semantics under the truncating
    law, computed with `i128` independently of the emitter and of any `exact::` call (zero divisor: `Undefined(DivisionByZero)`;
    the selected member in the type''s interval: `Completed` with the comparison''s Boolean over that
    member; otherwise `Refused(DivisionOutOfDomain)` with that member), on a grid of dividends and divisors
    inside the declared domain, at its edges and outside it, that includes the two '
- id: FR-031-AC-18
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: PLANNED (IR-602). A divide or remainder node over a `saturate` integer type is refused with
    `UnsupportedSaturatingArithmetic` at the node's span, with no artifact and no raw `/` or `%` in any
    output, for the `saturate` rows S-1 to S-4 of TC-044 step 11, in both consumers; the diagnostic message
    names the missing saturating operation and the refusal has terminal state `unsupported`. On the tree
    before the change each row is refused with `UnsupportedIntegerDivision` instead. This criterion calls
    no kernel and can land before the lock prerequisite, while the interim code still exists for a divide
- id: FR-031-AC-27
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: 'PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before the native division
    of AC-2 exists). The code `UnsupportedIntegerDivision` does not exist: the crate''s `GenerationErrorCode`
    has no such variant, nothing under `src/`, `tests/` or `schemas/` spells it, no stable code string
    of that name is emitted, and no diagnostic message that generation can emit names IR-601. On the tree
    before the change the variant exists and a divide or remainder over `reject` is refused with it.'
- id: FR-031-AC-28
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-664). The V1 Boolean oracle admits a typed IR integer domain or literal only
    when its value and both inclusive domain endpoints fit i64. A minimum of `i64::MIN - 1` or `i128::MIN`,
    or a maximum of `i64::MAX + 1` or `i128::MAX`, produces `UnsupportedExpression` at the first affected
    expression source span and no generated source; the same rule applies to an out-of-range literal.
    Values are never narrowed, clamped or wrapped to create an oracle.
- id: FR-031-AC-19
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose arithmetic nodes are add,
    subtract and multiply renders them as the infix `+`, `-` and `*` on `i64` with one binary operator
    per arithmetic node, contains no `exact::` call and no `checked_`, `wrapping_`, `saturating_` or `overflowing_`
    method (a remainder node's `wrapping_rem` is FR-031-AC-23's, not this criterion's), takes no meter,
    and its file states that its arithmetic is checked by Kani and is not a native evaluator.
findings:
- fnd: FND-217
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-15
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1858'
  excerpt: Typed private controls reject malformed and unknown fields and enforce finite encoded-byte,
    pending-message and startup-work bounds. Ancillary descriptor count/type is exact, received descriptors
    are CLOEXEC, and unknown/extra/truncated ancillary data refuses while closing all received descriptors.
    Invalid/overlimit control or EOF cancels or refuses rather than authorizing Dispatch.
  finding: 'Inherited FR-034-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-218
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-16
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1859'
  excerpt: Only the original caller holds the pair's executor endpoint. Guardian receives only its control
    end as fd0 through safe CLOEXEC child-only mapping, borrows it with a safe descriptor API and receives
    actual backend stdin separately as OwnedFd through safe ancillary rights. Backend fd0 restores that
    original stdin or closed state, while the control/lease descriptor is excluded from backend and unrelated
    exec inheritance. No arbitrary raw-fd adoption or unsafe exception occurs.
  finding: 'Inherited FR-034-AC-16 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-219
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-17
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1860'
  excerpt: 'Backend receives unchanged raw argument bytes except the explicitly allocated internal report
    locator: harness options then -Z unstable-options --export-json /proc/self/fd/N with actual mapped
    N >=5. Evidence records that exact argv. Inherited stdin, inherited/overridden environment and cwd
    remain unchanged.'
  finding: 'Inherited FR-034-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-220
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-18
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1861'
  excerpt: Guardian diagnostics and control use separate channels from backend stdout/stderr captures
    and cannot contaminate reports; existing bounded capture and capture-failure behavior remain authoritative.
  finding: 'Inherited FR-034-AC-18 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-221
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-19
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1862'
  excerpt: 'Original identity ceilings and existing outcome classifications remain authoritative after
    confirmed cleanup: memory excess and wall expiry stay distinct, ambiguous live-worker RSS refuses,
    and ordinary completed/refused/falsified reports retain their meanings. Native refinement uses this
    ownership path when its FR-028 AC-24 typed entry is implemented, with unchanged refinement class/evidence
    rules.'
  finding: 'Inherited FR-034-AC-19 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-222
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-20
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1863'
  excerpt: Guardian connection, startup, identity verification and Dispatch use the original monotonic
    identity deadline without resetting it. An already expired deadline causes no Dispatch and settles
    inconclusive with the timed-out reason as FR-028 AC-2 states; actual identity-deadline expiry during
    setup or execution uses the same classification.
  finding: 'Inherited FR-034-AC-20 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-223
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-21
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1864'
  excerpt: Guardian connection and handshake have a finite setup cap within the remaining original deadline.
    Cap expiry while that deadline remains live is a typed setup refusal, distinct from identity-deadline
    timeout.
  finding: 'Inherited FR-034-AC-21 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-224
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-22
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1865'
  excerpt: Control/capture shutdown and cleanup observation waits have finite bounds. Unconfirmed termination
    refuses with a live caller rather than accepting a proof or claiming physical disappearance of an
    uninterruptible task.
  finding: 'Inherited FR-034-AC-22 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-225
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-23
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1866'
  excerpt: Caller-death fixtures use the actual matching normal helper and shared production functions/order
    with bounded prefix selection as input only. Actual caller/group death is followed by independent
    original owned M/I pidfd termination, no backend Dispatch/marker and no leaked owned inner processes
    before emergency cleanup. Unclaimed Bootstrap supplies no invented INIT pin or teardown credit; AC-31
    remains independently owed. Cached prefix or gate-retained metadata proves no live gate. Feature-off
    positive post-Dispatch controls remain required.
  finding: 'Inherited FR-034-AC-23 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-226
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-24
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1867'
  excerpt: Actual lease-EOF cancellation passes independent original I termination and no backend Dispatch/marker
    before escalation, plus death of a positively acknowledged pinned worker after Dispatch. Ignored EOF,
    skipped actual lease close, removed positive Dispatch, non-INIT watcher, broken session isolation
    and startup-order mutants independently fail their genuine predicates; later cleanup cannot rescue
    them. The immediate private LeaseCloseObservation remains control flow, not a stored certificate.
    Stopped-I pending-authorization coordination remains owed without close/publication snapshots or ord
  finding: 'Inherited FR-034-AC-24 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-227
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-25
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1868'
  excerpt: Before backend Dispatch, the helper handshake matches the actual running CG library's build,
    protocol and lifecycle-capability identity against the actual invoked first-party executable. A stale
    helper or changed lifecycle implementation refuses, even if a caller supplies a matching version label
    or digest. Expected identity derives from actual library/helper build artifacts, not caller assertions
    or manually maintained tracking pins.
  finding: 'Inherited FR-034-AC-25 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-228
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-26
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1869'
  excerpt: The guardian session/group is distinct from the original caller before Ready/Dispatch. Real
    feature-on caller-group deaths and feature-off positive post-Dispatch death controls require independent
    owned termination/no backend/no leaked processes. Unclaimed Bootstrap cannot infer I death or held
    gate from monitor death or cached phase; AC-31 remains separately mandatory. Claimed I termination,
    operational lease cleanup and directly killed guardian kernel teardown remain required.
  finding: 'Inherited FR-034-AC-26 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-229
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-27
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1870'
  excerpt: guardian-test-support is off by default, absent from default features, and exposes exactly
    one documented fixture operation only when explicitly enabled. A feature-off consumer cannot use that
    operation. No public lease, process-ownership handle, cancellation entry or cleanup-deferring callback
    is exported. The private initialized death witness transfers exactly one actual owned monitor or validated
    INIT pin, with its typed authority; BeforeMonitor transfers none. Reporter startup stdout is safely
    duplicated into a non-stdio CLOEXEC OwnedFd and itself marked CLOEXEC before any spawn; every chi
  finding: 'Inherited FR-034-AC-27 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-230
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-28
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1871'
  excerpt: Fixture prefix selects unchanged production transitions, never certifies live state. Actual
    original identity/right custody, independent pre-escalation I/worker/marker checks and immediate unchanged
    cleanup on success/refusal/error replace stored stage/close/publication certificates. Intentional
    C/group self-death follows complete bounded operational pin delivery without ACK/pause or fabricated
    result. Missing identity, observation or genuine coordination fails; unavailable exact early-release
    observation remains UNBACKED. No stored record or later cleanup repairs an oracle.
  finding: 'Inherited FR-034-AC-28 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-231
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-29
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1872'
  excerpt: The packaged caller fixture and real helper link the same normal library artifact through consumer-manifest
    package selection with matching target/profile/features/compiler flags. Separate named feature-off/on
    invocations avoid self dev-dependency feature unification. No cfg-test library or identity override
    is accepted. The bounded executor refuses feature mismatch in both directions before Dispatch; feature-off
    consumer compilation verifies absence of the fixture operation.
  finding: 'Inherited FR-034-AC-29 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-232
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-31
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1874'
  excerpt: 'Real production fixtures fail the merged PR #295 recipe at exact post-clone/pre-internal-child_wait
    caller death and natural M/info-reader failure, and pass only with confirmed actual outer INIT and
    unclaimed inner/escaped descendant termination. Exercise C/group, L, O and I death before/after parent-death
    arm with positive barriers; pre-arm has no inner child and exclusive EOF gives bounded refusal. Original-exclusive-lease
    race, actual host/outer identity translation, mapping/private-proc capability refusal, real observer,
    ordinary-caller capability measurement, spawning-thread lifetime and '
  finding: 'Inherited FR-034-AC-31 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-233
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-32
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1875'
  excerpt: The bounded O event loop collects anonymous report pipe into memfd without a backend/collector
    completion wait cycle. A hard 16 MiB plus one detection-byte retention bound and the defined owned_RSS
    + caller_run_buffers + page-rounded F_GETPIPE_SZ + page-rounded pre-reserved memfd maximum comparison
    applies while writing against the original ceiling; unmapped shmem is not zero. Beyond cap yields
    owned cancellation and single-run KaniRunOutcome::Inconclusive with MemoryExhausted; batches keep
    whole-batch memory-exhausted refusal, with no member classified. Evidence names the report cap separatel
  finding: 'Inherited FR-034-AC-32 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-234
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-33
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1876'
  excerpt: Authenticated Completed precedes original-lease close, confirmed inner teardown and M termination/reap
    closing ALL pipe writers, including reopened procfd and descendant copies. Bounded actual-EOF drain
    precedes immutable WRITE/GROW/SHRINK/SEAL seals, consumer F_GET_SEALS verification and stable actual
    OwnedFd reads, all under the original deadline. Separate final control/report delivery remains live
    after lease close. Pre-Completed reader failure cancels O/I. All storage-owner death reclaims backing
    at final close without persistent report residue or surviving-owner dependence. Inheritance, s
  finding: 'Inherited FR-034-AC-33 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 13
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++