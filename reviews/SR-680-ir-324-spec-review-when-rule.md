---
id: "SR-680"
title: "CG PR 216 spec review: AD-003 R-Q1 when-rule"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a1f3baaab6177dd871aee13658b842ece2d9b7ed; spec/assurance/AD-003-evidence-chain.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
---

# SR-680: CG PR 216 spec review

## Summary

Ticket: IR-324. PR: agent-ix/quire-contract-codegen#216 at a1f3baa, base main. One file
changed, `spec/assurance/AD-003-evidence-chain.md` (+7 -3): link 7's pre-replay row (second
group), E-3 and the R-Q1 routing row. Methods: spec-review (integrity, consistency,
traceability). Gap analysis was not run as a separate pass: the change is spec-only, and the
one gap seen is recorded here as FND-004.

The QSL decision itself was relayed by the IR planner. QSL-352 is not yet verifiable. It is
cited as relayed and was not checked.

Checked and found as the PR states:

- CG FR-030 (`spec/kani/functional/FR-030-ir-outcome-terminal-map.md:62-64`, FR-030-AC-2)
  maps IR `Refused`, `InvalidInput` and `IncompleteInput` to
  `Declined(ProofRefusalCause::Refused | InvalidInput | IncompleteInput)`. FR-030's precedence
  rule says it applies to an IR outcome that no CG run produced, which holds for a refusal made
  before Kani runs. FR-029 has no `Declined` row, as expected, because it covers runs only.
- IR origin/main 457566c: `KaniOutcomeKind` has `Refused`, `InvalidInput` and
  `IncompleteInput` (`src/kani/outcome.rs:14-18`), and `provider_result` maps all three to
  `Declined` (:46).
- QSL origin/main b5ef6475: `ProofRefusalCause { Refused, InvalidInput, IncompleteInput }`
  exists (`qsl-replay/src/proof_result.rs:69`), and `TerminalValue::Declined(ProofRefusalCause)`
  (:120) has category `Refusal`. So the IR-outcome map can produce `Declined(ProofRefusalCause)`
  at the CG base. ADR-013 C-09 (:629) gives the replay-refusal-to-`replay_refused` and
  fault-to-`Failed` rules that (b) relies on.
- AD-003 join 1 to 2 (:72) already names IR's `Refused`, `InvalidInput` and `IncompleteInput`
  as that join's failure outcome, so (a) cites a join that exists.
- AD-002's failure table (:87-97) defers terminal values to AD-003 and does not contradict the
  when-rule. Grepping AD-003 and AD-002 for `Failed`, `Declined`, `ReplayRefused` and `R-Q1`
  found no contradicting text. AD-003:187-189 ("until QSL decides, CG keeps option A's rows")
  is about QSL-351 option B, not about R-Q1's when-rule.
- No pins, SHAs or version records were added. No requirement ids were minted (QSL-352 is a
  ticket). Nothing from quire-research is disclosed.
- `make spec` with TRUSTED_HOME set to the scratchpad home exits 0 with the 3 baseline warnings
  (FR-017:137, FR-014:278 twice). The PR adds none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The example in R-Q1 (a), "a refused caller lock" before Kani giving `Declined`, collides with (b). In CG the caller-lock refusal is `DependencyLockError`, reached through `ReplayPackageError::Dependencies` while building the replay package (interface-001:155; FR-016-AC-16 to AC-19). That happens only after Kani refuted, and link 7 puts `Dependencies` in the (b) group (`ReplayRefused`). CG #215 AD-004 step 5 calls `DependencyLockError` "a refused caller lock". Also, no IR `KaniOutcome` at IR origin/main refuses a lock (`src/kani` has no lock refusal; IR checks the lock at checked-package admission with `stale_dependency`). So nothing that produces FR-030's input backs the example at this base. A reader of AD-003 and AD-004 together would map `DependencyLockError` to `Declined`. Needed: say that a lock refused at replay setup after a refutation is (b), and say which producer refuses a lock before Kani as an IR outcome, or keep QSL's example only as relayed with that clarification | spec/assurance/AD-003-evidence-chain.md:349, :77 |
| FND-002 | low | "This sits before the C-09 replay map" (R-Q1) and "outside this replay map" (E-3) treat C-09 as the counterexample map only. But ADR-013 O-16 (:397) and C-09 (:629) define C-09 over all ten IR `KaniOutcomeKind`s together with the replay result, so FR-030's `Declined` rows are C-09 rows. AD-003:197 itself plans "one C-09 table". Say "outside the replay (counterexample) rows of C-09" | spec/assurance/AD-003-evidence-chain.md:349, :142-144 |
| FND-003 | low | "a non-fault `CallSite` refusal (`Compile`, `UnknownFunction`)" reads as a complete list. QSL `CallSiteRefusal` (`call_site.rs:142`) also has `ModelIntake`, `DependencyInput`, `Import`, `Dependency`, `UnknownOperation` and `UnknownClause`. A total map with no wildcard needs all of them placed. Say "for example", or name every non-fault variant | spec/assurance/AD-003-evidence-chain.md:77, :349 |
| FND-004 | low | R-Q1 (a) is worded generally ("Before Kani runs, when the obligation's own input is refused"), but the rule is stated only for IR-reported refusals. CG's own refusals before Kani, at join 2 to 3 (`UnsupportedObligations`, `NoDerivableClaim`) and join 3 to 5 (`HarnessNotInCrate`), have no stated terminal value, and FR-029 has no `Declined` row. Say whether (a) covers them or whether they stay generation errors with no terminal value | spec/assurance/AD-003-evidence-chain.md:72-74, :349 |

## Verdict

The layering claim holds. FR-030 maps IR's `Refused`, `InvalidInput` and `IncompleteInput` to
`Declined(ProofRefusalCause)`. That type and variant exist at QSL's base. IR's outcome type has
those kinds, and join 1 to 2 is the right join. The (b) group, the interim `Failed` and "faults
stay `Failed`" agree with link 7 and with ADR-013 C-09. The link 7 row, E-3 and R-Q1 agree with
each other.

Not mergeable as is: FND-001 (medium) should be fixed first. It is a one-sentence fix. The
three low findings are small wording fixes.

Consistency with CG #215 (AD-004 at 9d06673, open). These mismatches are #215's to fix, not
this PR's:

1. Step 5 still carries the R-Q1 open question ("not decided here").
2. Step 5 names `DependencyLockError` as a candidate for `Declined`. Under the when-rule it is
   (b), `ReplayRefused` once QSL-352 lands.
3. Step 5's replay-outcome type has a QSL result, a `ReplayRefusal`, a fault and a CG-defect
   variant, and no path for a CG pre-replay data refusal that carries a QSL code.
4. Step 5 does not mention QSL-352, although AD-003 says those codes land with step 5.
