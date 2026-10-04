---
id: "SR-1352"
title: "IR-545 gap analysis: FR-021-AC-24 and TC-031 step 12 against code and tests"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@7dd5a391ddc801a6f9fa1c1e5376f4575ccbad48; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md, src/oracle/function/mod.rs, tests/it/exact_function_generation.rs"
relationships:
  - target: "ix://agent-ix/quire-contract-codegen/FR-021"
    type: reviews
---

# SR-1352: IR-545 gap analysis (PR #253)

## Summary

Ticket: IR-545. PR: agent-ix/quire-contract-codegen#253, head 7dd5a39, base 3033993. This review
maps FR-021-AC-24 and TC-031 step 12, cases (i) to (vi), to the six new tests and to the code. It
also checks the FR-021 Behavior bullets on duplicate requests and on claim ordering.

## Method

A throwaway probe at the head asked for each case and printed the claim-map dispositions:

- (i) `zz_unknown` gives `UnknownFunction{zz_unknown}`.
- (ii) Both orders give `UnknownFunction{aa_unknown}`, then `{zz_unknown}`, with no
  `DuplicateRequest`.
- (iii) `zz_unknown` twice gives one `DuplicateRequest`.
- (iv) Both orders give `UnknownFunction{zz_unknown}`, then the `Generated` `add_fn` claim, which
  equals the solo `add_fn` claim.
- (v) Both orders give `Zz_unknown`, then `aa_unknown`.
- (vi) For `m_multi`/`z_pair`, both orders give `DuplicateDeclaringNode{N1}`, then `{N2}`.
  For `z_multi`/`a_pair`, both orders give `{N2}`, then `{N1}`.
- Extra: five items (`zz_unknown`, `add_fn`, `aa_unknown`, `zz_unknown`, `Zz_unknown`) give
  `Zz`, `aa`, `DuplicateRequest` (for zz), then `add_fn`. A non-ASCII name (`é_unknown`, first
  byte 0xC3) sorts after `z_unknown`.

Each case maps to one test: `tc_031_ac24_case_i` … `case_vi`. Each test asserts the exact
dispositions in order. Cases (ii), (iv), (v) and (vi) check both request orders, and case (vi)
also checks every declaration order. `quire coverage --scope . --strict` gives 66 unbacked rows
at both base and head. Backed rows go from 234 to 235: the AC-24 row was planned and is now
backed, and FR-021 is now 23/24, with AC-18 still planned. `make spec` passes.

A second probe tested the general Behavior bullet ("two items on one `call` node name the two
members of a duplicate-node pair … order them by the function name"). It used three node ids,
N1 < N2 < N3 in node-id order, and five declarations: `a_pair` and `z_pair` on N2, `z_pair` and
`q_extra` on N1, and `a_pair` alone on N3. The N3 declaration is valid AC-22 input: a
distinct-node declaration that shares a name with a member of the pair. Items `a_pair` and
`z_pair` on one call node give `DuplicateDeclaringNode{N1}` (for `z_pair`), then `{N2}` (for
`a_pair`), under both request orders. Name order would put `a_pair{N2}` first.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Members of a duplicate-node pair are not always ordered by name, as the FR-021 Behavior bullet requires. `ItemKey` puts `function_node_id` before `name`, and `function_node_id` is the highest-sorted declaration holding the name. So when one member's name is also held by a distinct-node declaration on a larger node id, the order follows node id, not name. The probe shows `{N1}` (z_pair) before `{N2}` (a_pair). The output is still the same under every request order. | src/oracle/function/mod.rs:1420-1430; spec/oracle/functional/FR-021-function-application-oracles.md:261-265 |

## Verdict

FR-021-AC-24 is fully implemented, and every example in it is backed by a hand-written test that
kills the mutants in its mutation row. The status flips in FR-021, TC-031 and the oracle matrix
are accurate for the AC. They agree with `spec/tests.md`, whose oracle row stays partial for
AC-18 and FR-018.

One gap remains. The Behavior bullet states the name order for pair members in general, but the
code gives it only when both members resolve to the same `function_node_id`. That holds for the
AC-24 example, but not when a member's name has a namesake on a larger distinct node. The
TC-031 matrix row's wording ("one entry per duplicate-node pair member … in byte order of the
name") has the same gap. This was the same at base, because `refused_name` was also the last
field. There are two ways to resolve it:

- Narrow the bullet and the matrix wording to "when their preceding key fields are equal".
- Give refused duplicate-node items a uniform `function_node_id` in the key, so that the name
  decides. This would change the order of AC-22 items relative to other items.

Either way, add a test for the five-declaration case.

## Dispositions

Round 1, checked at 0e8dd504eec6295265a22de1d822b74854a75efe. The fix narrows the spec and leaves
the code alone: `src/` is unchanged since 7dd5a39.

- **Behavior bullet.** It now says the function name is only the final tie-break, and that an
  item's declaring node id is the one of the highest-sorted declaration holding its name. Both
  statements match `ItemKey` and `function_node_id_by_name`.
- **New AC-24 example and TC-031 case (vii).** They state the order the code actually produces.
  My own probe at this head, with the five declarations, gives `{N1}` (z_pair), then `{N2}`
  (a_pair), under both request orders.
- **No unconditional name-order claim left.** A grep finds none in FR-021, TC-031, the oracle
  matrix or `spec/tests.md`. AC-24's opening sentence about pair members is qualified by the next
  sentence in the same cell.
- **The new test is a real oracle.** `tc_031_ac24_case_vii_differing_declaring_node_ids_order_before_names`
  fails under the name-first mutant, together with case_iv. It also fails under a mutant that
  keys duplicate-node items with one shared declaring node id, so that the name decides.
- **Gates.** `make spec` passes. `quire coverage --strict` gives 66 unbacked rows, 235/344 backed,
  and FR-021 at 23/24.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0e8dd504eec6295265a22de1d822b74854a75efe |
