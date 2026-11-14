---
id: SR-1121035
title: "spec-review/base of IR-1535 PR 829"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-049-enum-scalar-harnesses.md, spec/kani/matrix/TC-063-enum-scalar-harnesses.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-1535. PR #829 at bedac3c4, scope `git diff origin/main...HEAD` (FR-049 Behavior, AC-3, AC-6; TC-063 steps 3 and 5).
Checked against the owner spec quire-exact FR-407 and `src/native_enum.rs` on quire-exact main (fec2780):
`NativeEnum::from_member(declaration, &EnumShape, &EnumMember)` refuses `MemberNotInShape` exactly when
`shape.rank(member.variant()) != Some(member.rank())`; `compare_native_enum` compares rank only; `EnumShape`
holds `Vec<VariantId>` and `VariantId` holds a `NodeKey` and a `String`. CG's current oracle renderer
(`src/oracle/scalar/render.rs:561`) still renders `enum_owner::compare_enum` over `&enum_owner::EnumValue`.
Criterion strength judged manually (Jev not used). No pins, hashes or digests in the changed prose; no parked or unowned AC.

## Verdict

**FAIL.** The amendment uses FR-407's terms correctly (`from_member`, `EnumShape`, `MemberNotInShape`,
rank-only eq/ne) and correctly drops the impossible "minted identity fails eq/ne" claim. But the
replacement refusal test exercises quire-exact's `from_member`, not CG, so FR-049's own "SHALL NOT mint"
has no CG-side oracle left; and the new carrier wording contradicts AC-1 and the rest of FR-049, which
still name the owner `compare_enum` oracle. Two high, one medium, two low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-3's minted-identity clause and TC-063 step 3 test only that quire-exact's `from_member` refuses a hand-built foreign member, which FR-407-AC-3/TC-955 already owns. The CG defect FR-049 forbids (generator mints a key) is built into the shape and the members from the same route data, so `shape.rank(v) == Some(rank)` holds, `from_member` accepts, and the rank-only proof is blind. Scenario: generator emits case `Red_` for `Red` in both its `EnumShape` and members; eq/ne Verified, step 3 still passes. Require the `EnumShape` handed to `from_member` to equal the owner route's admitted shape, with a generator-mint mutant caught by that comparison; cite FR-407-AC-3 for the refusal itself. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:82-90,144; spec/kani/matrix/TC-063-enum-scalar-harnesses.md:29-34 |
| FND-002 | high | The new Behavior paragraph makes the operands rank-only `NativeEnum` carriers, but AC-1 (oracle is FR-014's generated enum comparison), the Description (owner `compare_enum`, semantics through FR-014) and Behavior lines 97-98 (`compare_enum` returns `Incomplete`) are unchanged, and FR-049 never names `compare_native_enum`. `compare_enum` takes `&EnumValue` (render.rs:561), not a `NativeEnum`. Scenario: an implementer following AC-1 keeps the heap `compare_enum` oracle FR-407 exists to replace and the `from_member` carrier has no consumer; one following the Behavior makes AC-1 false, with no FR-014 renderer for the carrier. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:39-51,86-88,97-98,142 |
| FND-003 | medium | "The harness SHALL build each operand with `from_member` over the `EnumShape` and the checked member at its index" puts a `Vec<VariantId>` (NodeKey `Arc` + `String`) lookup and its drop glue under the symbolic index inside the Kani proof, the heap shape FR-407's Description names as the timeout cause; FR-407's use case builds symbolic operands with `from_index`. It also contradicts AC-3's "refused at construction ... before any proof". Scenario: harness indexes the shape's members by symbolic `i64` and the proof times out or exhausts memory. State that `from_member` runs at generation time over each admitted member and the harness uses `from_index(tag, ordered, index, count)`. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:82-85,144 |
| FND-004 | low | AC-6 now carries a non-mutant obligation ("a minted identity is not a proof mutant ... which a refusal test asserts") that duplicates AC-3, so AC-5's "each mutant of AC-6" reads ambiguously and TC-063 step 5 traces the minted refusal to AC-6 while step 3 traces it to AC-3. Keep the refusal in AC-3 only. TC-063 line 43 is an unwrapped long line. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:146-147; spec/kani/matrix/TC-063-enum-scalar-harnesses.md:42-44 |
| FND-005 | low | FR-049 now depends on quire-exact's `from_member`, `EnumShape` and `MemberNotInShape`, but its relationships have no edge to quire-exact FR-407 and the Description still says enum semantics belong to the QSV owner through FR-014. Add `references ix://agent-ix/quire-exact/FR-407` and name the carrier owner. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:5-19,49-51 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049 Behavior (operand construction) | examined | The harness SHALL build each operand with the owner's `from_member` over the owner-admitted `EnumShape` and the checked member at its index. `from_member` refuses a member the shape does not admit at that position with the typed refusal `MemberNotInShape`, so a minted identity (same position, different key) never reaches an operand. The proof's carrier holds the member's rank, the member count, the ordered flag and an opaque declaration tag, so the `eq` and `ne` proofs are over rank only. |
| FR-049-AC-1 | examined | ... CG generates one harness whose oracle is FR-014's generated enum comparison and whose expected value is the native operator over indices. |
| FR-049-AC-3 | examined | Operands are built with `from_member` from the owner's checked members of the admitted `EnumShape`, in declared order. A minted member identity (same position, different key) is refused at construction with `MemberNotInShape`, before any proof, and yields no operand; the `eq` and `ne` proofs are over rank only and make no claim about minted identity. ... |
| FR-049-AC-5 | examined | Each mutant of AC-6 produces a failed assertion with solver playback of in-domain indices ... |
| FR-049-AC-6 | examined | ... reversed ordering fails `lt`, `le`, `gt` and `ge`; a minted identity is not a proof mutant: it is refused at construction with `MemberNotInShape`, which a refusal test asserts. |
| TC-063 step 3 | examined | Present a minted identity (same position, different key) and assert `from_member` refuses it with `MemberNotInShape` and no operand is built (refusal test, not a proof failure) ... |
| TC-063 step 5 | examined | A minted identity is not run as a proof mutant; step 3 covers its refusal (AC-6). |
| quire-exact FR-407 | context_only | Behavior 4 (`from_member` refuses `MemberNotInShape` when `shape.rank(member.variant())` is not `Some(member.rank())`), Behavior 8 (rank-only Boolean), Relationship to CG FR-049 (Limitation). |
| FR-049-AC-2, AC-4, AC-7..AC-11 | context_only | Unchanged; read for consistency with the amendment. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | Stale-oracle text survives the FND-002 fix. The Dependencies bullet still says FR-014 "owns enum comparison semantics and the generated oracle; this requirement adds only the bounded proof row over it", which contradicts the Description and AC-1 (the generated side is quire-exact's `compare_native_enum`, FR-407). AC-4 still says "the oracle outcome" and "a path reaching `OracleStop::IllTyped`", the Description says the causes are "surfaced as `OracleStop::IllTyped`", and TC-063 steps 1 and 4 still say "oracle" and force "the `OracleStop::IllTyped` path", while Behavior now says "the comparison's `Err(IllTyped)`". `OracleStop` is the FR-014 oracle wrapper. Scenario: an implementer reads Dependencies and AC-4 as requiring the harness to route through FR-014's oracle (the heap `compare_enum` path), or a test author finds no `OracleStop` path when the harness calls `compare_native_enum` directly, so AC-4's last sentence cannot be exercised as written. The FR-014 `depends_on` edge itself is still right (the checked member list and ordered flag come from the owner route through FR-014); the bullet text and the `OracleStop` naming are what need changing. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:46,153,164-166; spec/kani/matrix/TC-063-enum-scalar-harnesses.md:24,39 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix 5d7793c1: Behavior requires CG to use the owner-admitted `EnumShape` as supplied; AC-3 adds a generator-mint mutant (renamed key in both shape and members) that fails the comparison with the owner-admitted shape; TC-063 step 3 runs that mutant and cites FR-407-AC-3 for the refusal itself. The check is CG-side against the owner route's own shape, so step 3 can fail on a CG mint. |
| FND-002 | fixed | Fix 5d7793c1: Description, Outputs, AC-1 and Behavior now name `compare_native_enum` over `NativeEnum` carriers (FR-407) as the generated side; `compare_enum` no longer appears. Residual stale-oracle text in Dependencies, AC-4 and TC-063 recorded separately as FND-006. |
| FND-003 | fixed | Fix 5d7793c1: `from_member` runs at generation time; inside the proof operands are built with `from_index(tag, ordered, index, count)`, so the proof holds no variant list or heap value. AC-3 matches. |
| FND-004 | fixed | Fix 5d7793c1: AC-6's minted-identity clause removed (the refusal lives in AC-3 only); TC-063 step 5 no longer traces the refusal to AC-6 and its long line is wrapped. |
| FND-005 | fixed | Fix 5d7793c1: frontmatter adds `references ix://agent-ix/quire-exact/FR-407`; the Description names quire-exact as owner of the carrier, its construction and comparison. |
| FND-006 | fixed | Fix 90e7390f: the Dependencies bullet names quire-exact FR-407 as owner of the carrier, its construction and `compare_native_enum`, with FR-014 supplying only the checked member list and ordered flag; AC-4, Description line 46 and TC-063 steps 1 and 4 now say "the comparison" and "the comparison's `Err(IllTyped)`". A grep of FR-049 and TC-063 at 90e7390f finds no `OracleStop`, "generated oracle", "oracle outcome" or `compare_enum`; the FR-015, FR-025, FR-028 and FR-039 mentions of FR-049 name the row or its draw rule and assert nothing about the oracle. |
