---
id: TC-007
title: "Verify cross-backend semantic parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: verifies
---
# TC-007: Verify cross-backend semantic parity

## Description

Verify executable, proptest, Kani, and coverage lowerings agree on the shared bounded corpus and every
permitted differential fixture has an attributed disposition.

CG owns the comparison among the backend artifacts it generates and this test case's Differential
Disposition Register. The current executable slice compares the generated bound-integer oracle
with the bound strategy's independently computed relation census. The generated strategy runner
uses that oracle, so its typed verdict is a consistency observation rather than another
independent semantic result. The live bounded Kani corpus is graph-only and its harness calls the
same oracle; coverage observes oracle probes. Neither is qualified as a separate semantic
backend by this slice. The broader FR-005-AC-4 obligation remains open.

## Test Procedure

1. From one admitted bound-integer relation, generate the bound-strategy census and the executable
   bound oracle. Give each canonical candidate a stable case identifier. Include each inclusive
   model endpoint, adjacent values around each comparison literal where representable, admitted
   cases whose relation holds and fails, and immediately outside model values. Require that the
   generated census enumerates the intended candidate identities and tags rather than merely
   sampling a passing subset.
2. Compile the generated executable oracle. For every in-domain candidate, record the strategy
   census tag computed from its relation, the oracle's Boolean result, and the generated census
   runner's typed verdict. Normalize an admitted census tag and oracle Boolean to the same `holds` or
   `violated` outcome. Retain any generation diagnostic and dependency identity with its case;
   absence is recorded as absence, not inferred agreement. Compare the independently computed
   tag with the compiled oracle result and require the runner to report the corresponding verdict.
3. Preserve an outside-model candidate as `out-of-domain` on the strategy side and retain the
   executable oracle's actual Boolean result as a separate observation. Its normalized pair is
   `NotComparable` because the strategy does not admit that candidate; it is not a semantic
   discrepancy and requires no register row. Do not pass it to the in-domain census runner or a
   Kani proof, compare it as though both backends had evaluated it, or claim parity for it. If an
   immediately outside mathematical value cannot fit the oracle's integer input, retain the
   census's `unrepresentable` edge with its identity instead of silently dropping it or
   fabricating an oracle result.
4. Compare every measured discrepancy with the authoritative register by the exact `(case
   identifier, backend pair)` key. Exercise a mutant that changes one admitted census tag while
   leaving the oracle unchanged; it must produce a named discrepancy. For this control only, pass
   the mutant measurement to the same join with a temporary in-memory register input seeded with
   one valid row naming its real test fixture under `tests/`. From that temporary input,
   independently remove the row, duplicate the key, name a nonexistent test fixture, and leave
   its disposition empty; each change must fail the join. Never add the mutant row to the
   authoritative register: the normal unmutated run would reject it as stale. Do not run a Kani
   proof or revive the retired V1 bundle arithmetic path for this slice.

## Differential Disposition Register

The register is this table. Its current backend-pair key is
`bound_strategy_census / executable_bound_oracle` in that order. Each measured discrepancy has
exactly one row keyed by its case identifier and backend pair. The `Disposition` cell is either
`fixture` or `profile/backend distinction`; the last cell is respectively a path to an existing
regression fixture under `tests/` or a nonempty explanation of the concrete profile/backend
distinction. A missing row, duplicate key, stale row not present in the measured discrepancy
list, nonexistent fixture, disposition without its required value, or row claiming both kinds
fails the join. Only candidates admitted by both backends enter the semantic discrepancy list;
outside-domain `NotComparable` observations are retained separately with no register key. The
comparison produces an inspectable list of case identifiers, pairs, normalized outcomes,
diagnostics and dependencies even when no difference occurs. An empty register is valid only
after an actual complete run has measured zero discrepancies. The mutant's temporary register
input is test data for the join and never changes this table. No such run or register join exists
at this revision, so the empty table below is not completion evidence.

| Case | Backends | Disposition | Fixture or distinction |
|------|----------|-------------|------------------------|

## Expected Results

For each admitted candidate, the independently computed census tag, compiled executable-oracle
result and runner verdict agree after normalization. Every comparable difference appears exactly
once in the Differential Disposition Register. A zero-difference run leaves the register empty
and reports the complete measured candidate list, including separate outside-domain observations.
The mutant produces a named difference; its temporary valid register input joins exactly once,
and each of the four adverse inputs fails with the exact case and pair. The authoritative register
remains unchanged and would reject the mutant row as stale on the unmutated run. No Kani or
coverage semantic-parity result is inferred from this slice.
