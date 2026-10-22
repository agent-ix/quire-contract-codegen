---
id: SR-15371
title: IR-17 main landing PR513 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; IR-17; spec/, src/, tests/
review_set: subset
---

## Summary

Ticket IR-17; PR513, frozen 5e60b358eef84d28fa1a1897d1a703633458e8a5, base origin/main 0d7813df955576edc867cfa90e2071bd56d844ff. Scope is the complete 88-file main landing, not PR511's increment. Reviewer-only: no product edits, commits, pushes, Cargo, compiler, test, native execution, Kani/backend, namespace creation, bwrap, unshare, probes or new gate attempt. No applicable AssuranceProfile exists under spec/. Historical review and six-PASS qualification cohorts apply only to their recorded old heads. They are context, not fresh qualification of this candidate. Current method computations and source evidence are retained beside this artifact; tracker text is data.

## Verdict

FAIL for repository completeness; no new untagged criterion was introduced by this landing. Product readiness separately FAILS under SR-15370; static trace improvements do not discharge its source findings or unrun execution gates.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-005-AC-3 remains untagged on both main and candidate; the existing SR-15352/FND-001 baseline inventory retains its owning remediation. | spec/core/functional/FR-005-cli-conformance.md:49 |
| FND-002 | high | FR-016-AC-25 is untagged on both current main and candidate; this newer baseline obligation is outside SR-15352 and retains its existing IR-604/PR498 replay-owner scope. | spec/replay/functional/FR-016-witness-native-replay.md:156 |

## Coverage

Current full repository matrix: 898 criteria; 541 tagged, 289 untagged, 16 tagged-by-ignored-test, 52 method-without-symbol. Exact main baseline: 897 criteria; 524 tagged, 303 untagged, 18 ignored-only, 52 method-without-symbol. Seventeen static status changes are recorded in matrix-comparison.json: FR017 AC27/30 ignored-only -> tagged, AC52/53 untagged -> tagged; FR018 AC33-43 untagged -> tagged; new AC44 tagged; FR031 AC26 untagged -> tagged. The last improvement is presence only: SR15370 FND004 records its currently contradictory assertion/spec intent. No tag is treated as passing execution. The user has now authorized the existing namespace tests for later final qualification after fixes and review; none ran in this review phase.

All 289 candidate untagged IDs are inherited from main. 288 current untagged IDs are present in the SR15352 reviews/ir-954-gap-analysis.md inventory and its explicit outcomes. The remaining FR016AC25 is a newer existing-main obligation from IR604/PR498, outside that older inventory; FND002 explicitly retains that existing replay owner. SR14141 retained original owner backlog and SR15361 inherited-scope disposition remain context; no new issue or duplicate unowned backlog is created. FND001 records the concrete FR005AC3 example for this artifact's explicit disposition. Existing ignored-only dedicated Kani controls are intentional named-lane classification and are not new defects merely because the default lane skips them. All quoin evidence rows report no run evidence; no proof completion is inferred.

Coverage comparison: five untracked tags and ten diagnostics on both baseline/candidate, with the same path/symbol/trace IDs and diagnostic categories. Existing uncatalogued verification methods and empty inspection target constrain what the model measures. Handwritten status/matrix rows are not evidence. Full-tree reverse/stub inventory covers 96 Rust source files, 90 Rust test files and 500 public/suspicious surface declarations retained in surface.txt, grouped through the master subsystem registry and interim-source ownership table (core/publication, strategy/bounds, oracle, Kani generation/run/output, routed/capability, replay and evidence). No production TODO/unimplemented/todo!/FIXME placeholder exists. New behavior is owned by FR014 scalar owner consumption, FR017 paired custody, FR018 admitted capability/Union and FR031 Boolean native metering (the metering spec contradiction is already SR15370 FND004). No newly unowned public behavior or hollow implementation found; native helper compilation is a real SR15370 defect, not skipped as a source-only test. No claim of full individual-symbol semantic correctness is made.

Reconciliation: quoin 0.28.4 / quire 0.36.2 engine 0.50.2; baseline computed from an exact origin/main git archive of spec/src/tests. Whole-repository mechanical criterion binding, source ownership and stub checks ran; semantic gap review skipped (not opted in), while mandatory code-review oracle checks ran by source inspection. Plan completion: not assessed. No Cargo/tests/builds/native/backend/namespaces or gates ran; no acceptance credit from old qualification cohorts.

## Examined scope

```yaml
clean: false
scope:
  - {id: FR-002-AC-1, path: &p0 "spec/strategy/functional/FR-002-tristate-proptest.md", role: examined, excerpt: "Rejected inputs remain distinct "}
  - {id: FR-002-AC-2, path: *p0, role: examined, excerpt: "Boundary strategies exercise imm"}
  - {id: FR-002-AC-3, path: *p0, role: examined, excerpt: "Supported correlated constraints"}
  - {id: FR-002-AC-4, path: *p0, role: examined, excerpt: "Shrinking preserves generated co"}
  - {id: FR-002-AC-5, path: *p0, role: examined, excerpt: "The generated campaign runner ow"}
  - {id: FR-002-AC-6, path: *p0, role: examined, excerpt: "Integer and generated Boolean ha"}
  - {id: FR-002-AC-7, path: *p0, role: examined, excerpt: "PLANNED (IR-815). For one precon"}
  - {id: FR-002-AC-8, path: *p0, role: examined, excerpt: "PLANNED (IR-815). In the V2 harn"}
  - {id: FR-002-AC-9, path: *p0, role: examined, excerpt: "PLANNED (IR-815). Source inspect"}
  - {id: FR-002-AC-10, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A selected cla"}
  - {id: FR-002-AC-11, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A selected cla"}
  - {id: FR-002-AC-12, path: *p0, role: examined, excerpt: "PLANNED (IR-815). Repeated V2 ha"}
  - {id: FR-002-AC-13, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-002-AC-14, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-002-AC-15, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-002-AC-16, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-002-AC-17, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-002-AC-18, path: *p0, role: examined, excerpt: "PLANNED (IR-815). A seeded V2 ca"}
  - {id: FR-004-AC-1, path: &p1 "spec/evidence/functional/FR-004-vacuity-evidence.md", role: examined, excerpt: "An evaluated implication whose c"}
  - {id: FR-004-AC-2, path: *p1, role: examined, excerpt: "A clause whose oracle-evaluation"}
  - {id: FR-004-AC-3, path: *p1, role: examined, excerpt: "The four measured clause classif"}
  - {id: FR-004-AC-4, path: *p1, role: examined, excerpt: "Successful analysis verifies the"}
  - {id: FR-004-AC-5, path: *p1, role: examined, excerpt: "Malformed, summary-only, identit"}
  - {id: FR-004-AC-6, path: *p1, role: examined, excerpt: "Campaign counts and test outcome"}
  - {id: FR-004-AC-7, path: *p1, role: examined, excerpt: "Removing an expected clause, con"}
  - {id: FR-004-AC-8, path: *p1, role: examined, excerpt: "Adverse coverage and non-success"}
  - {id: FR-004-AC-9, path: *p1, role: examined, excerpt: "Complete bound observations with"}
  - {id: FR-004-AC-10, path: *p1, role: examined, excerpt: "PLANNED. CG binds coverage insid"}
  - {id: FR-004-AC-11, path: *p1, role: examined, excerpt: "PLANNED. The consuming report re"}
  - {id: FR-004-AC-12, path: *p1, role: examined, excerpt: "PLANNED. CG consumes QSL's typed"}
  - {id: FR-004-AC-13, path: *p1, role: examined, excerpt: "PLANNED. QSL `run_clause` succes"}
  - {id: FR-004-AC-14, path: *p1, role: examined, excerpt: "PLANNED. Only CG's campaign runn"}
  - {id: FR-004-AC-15, path: *p1, role: examined, excerpt: "PLANNED. A controlled attempt to"}
  - {id: FR-004-AC-16, path: *p1, role: examined, excerpt: "PLANNED (IR-815). The expected c"}
  - {id: FR-004-AC-17, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Source inspect"}
  - {id: FR-004-AC-18, path: *p1, role: examined, excerpt: "PLANNED (IR-815). A V2 clause wh"}
  - {id: FR-004-AC-19, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-004-AC-20, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-004-AC-21, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-004-AC-22, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-004-AC-23, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-004-AC-24, path: *p1, role: examined, excerpt: "PLANNED (IR-815). Over a V2 clau"}
  - {id: FR-005-AC-1, path: &p2 "spec/core/functional/FR-005-cli-conformance.md", role: examined, excerpt: "Failed generation leaves no part"}
  - {id: FR-005-AC-2, path: *p2, role: examined, excerpt: "Supported platforms reproduce by"}
  - {id: FR-005-AC-3, path: *p2, role: examined, excerpt: "For every shared-corpus case on "}
  - {id: FR-005-AC-4, path: *p2, role: examined, excerpt: "Executable, proptest, Kani, and "}
  - {id: FR-005-AC-5, path: *p2, role: examined, excerpt: "A bundle naming more than 4096 a"}
  - {id: FR-005-AC-6, path: *p2, role: examined, excerpt: "When an artifact path is absolut"}
  - {id: FR-005-AC-7, path: *p2, role: examined, excerpt: "When publication fails before th"}
  - {id: FR-005-AC-8, path: *p2, role: examined, excerpt: "When removal of the replaced bun"}
  - {id: FR-005-AC-9, path: *p2, role: examined, excerpt: "When replacement fails and resto"}
  - {id: FR-005-AC-10, path: *p2, role: examined, excerpt: "When two artifacts of one bundle"}
  - {id: FR-008-AC-1, path: &p3 "spec/strategy/functional/FR-008-bound-domain-strategy-admission.md", role: examined, excerpt: "A fresh QSL-produced, IR-admitte"}
  - {id: FR-008-AC-2, path: *p3, role: examined, excerpt: "An IR-admitted V2 `amount < 7` i"}
  - {id: FR-008-AC-3, path: *p3, role: examined, excerpt: "A valid selected comparison gene"}
  - {id: FR-008-AC-4, path: *p3, role: examined, excerpt: "Unknown clause, absent claim, un"}
  - {id: FR-008-AC-5, path: *p3, role: examined, excerpt: "A request with several applicabl"}
  - {id: FR-008-AC-6, path: *p3, role: examined, excerpt: "Either read endpoint or a litera"}
  - {id: FR-008-AC-7, path: *p3, role: examined, excerpt: "PLANNED. Beyond the per-operator"}
  - {id: FR-008-AC-8, path: *p3, role: examined, excerpt: "PLANNED. The V2 comparison oracl"}
  - {id: FR-008-AC-9, path: *p3, role: examined, excerpt: "PLANNED. A CG integration test c"}
  - {id: FR-008-AC-10, path: *p3, role: examined, excerpt: "PLANNED. Source inspection shows"}
  - {id: FR-008-CON-1, path: *p3, role: examined, excerpt: "The generator SHALL read V2 oper"}
  - {id: FR-008-CON-2, path: *p3, role: examined, excerpt: "Admission SHALL finish before an"}
  - {id: FR-008-CON-3, path: *p3, role: examined, excerpt: "The generator SHALL check each r"}
  - {id: FR-008-CON-4, path: *p3, role: examined, excerpt: "The generator SHALL check each c"}
  - {id: FR-009-AC-1, path: &p4 "spec/strategy/functional/FR-009-constructive-correlated-populations.md", role: examined, excerpt: "For the QSL-produced, IR-admitte"}
  - {id: FR-009-AC-2, path: *p4, role: examined, excerpt: "For every V2 comparison operator"}
  - {id: FR-009-AC-3, path: *p4, role: examined, excerpt: "Generated population source cont"}
  - {id: FR-009-AC-4, path: *p4, role: examined, excerpt: "For `amount < 0` over 0..=1000, "}
  - {id: FR-009-AC-5, path: *p4, role: examined, excerpt: "A domain of `i64::MIN..=i64::MAX"}
  - {id: FR-009-AC-6, path: *p4, role: examined, excerpt: "Repeated generation for the same"}
  - {id: FR-009-AC-7, path: *p4, role: examined, excerpt: "PLANNED. For each noncommutative"}
  - {id: FR-010-AC-1, path: &p5 "spec/strategy/functional/FR-010-domain-boundary-campaigns.md", role: examined, excerpt: "For `amount < 7` over 0..=1000, "}
  - {id: FR-010-AC-2, path: *p5, role: examined, excerpt: "For the QSL-produced, IR-admitte"}
  - {id: FR-010-AC-3, path: *p5, role: examined, excerpt: "A selected claim whose accessor "}
  - {id: FR-010-AC-4, path: *p5, role: examined, excerpt: "`amount <= 1000` over 0..=1000 r"}
  - {id: FR-010-AC-5, path: *p5, role: examined, excerpt: "Every in-domain census tag equal"}
  - {id: FR-011-AC-1, path: &p6 "spec/strategy/functional/FR-011-numeric-harness-campaigns.md", role: examined, excerpt: "Over V2 selected claims, a QSL-p"}
  - {id: FR-011-AC-2, path: *p6, role: examined, excerpt: "A V2 comparison oracle deliberat"}
  - {id: FR-011-AC-3, path: *p6, role: examined, excerpt: "The census runner evaluates each"}
  - {id: FR-011-AC-4, path: *p6, role: examined, excerpt: "A campaign over a report seeded "}
  - {id: FR-011-AC-5, path: *p6, role: examined, excerpt: "A fresh 10,000-case `Broad` camp"}
  - {id: FR-012-AC-1, path: &p7 "spec/strategy/functional/FR-012-constraint-preserving-shrinking.md", role: examined, excerpt: "For every V2 comparison operator"}
  - {id: FR-012-AC-2, path: *p7, role: examined, excerpt: "For 1,000 seeded `VersionUnchang"}
  - {id: FR-012-AC-3, path: *p7, role: examined, excerpt: "A `Violating` V2 `VersionUnchang"}
  - {id: FR-012-AC-4, path: *p7, role: examined, excerpt: "After a shrinking campaign, `att"}
  - {id: FR-013-AC-1, path: &p8 "spec/strategy/functional/FR-013-it010-consumable-output.md", role: examined, excerpt: "A consumer fixture crate that in"}
  - {id: FR-013-AC-2, path: *p8, role: examined, excerpt: "The generated bundle contains no"}
  - {id: FR-013-AC-4, path: *p8, role: examined, excerpt: "The generated Rust header carrie"}
  - {id: FR-013-AC-5, path: *p8, role: examined, excerpt: "interface-001 `bound_strategy_sl"}
  - {id: FR-014-AC-1, path: &p9 "spec/oracle/functional/FR-014-exact-scalar-oracles.md", role: examined, excerpt: "Every requested item receives ex"}
  - {id: FR-014-AC-2, path: *p9, role: examined, excerpt: "Integer (add, subtract, multiply"}
  - {id: FR-014-AC-3, path: *p9, role: examined, excerpt: "A descriptor whose arity, result"}
  - {id: FR-014-AC-4, path: *p9, role: examined, excerpt: "Generated bytes are identical ac"}
  - {id: FR-014-AC-5, path: *p9, role: examined, excerpt: "Every claim-map entry carries th"}
  - {id: FR-014-AC-6, path: *p9, role: examined, excerpt: "Executing generated oracles on t"}
  - {id: FR-014-AC-7, path: *p9, role: examined, excerpt: "Composite, collection, function,"}
  - {id: FR-014-AC-8, path: *p9, role: examined, excerpt: "The generated crate declares `pu"}
  - {id: FR-014-AC-9, path: *p9, role: examined, excerpt: "Generated oracle functions do no"}
  - {id: FR-014-AC-10, path: *p9, role: examined, excerpt: "A descriptor parameter whose adm"}
  - {id: FR-014-AC-11, path: *p9, role: examined, excerpt: "A claim whose descriptor passes "}
  - {id: FR-014-AC-12, path: *p9, role: examined, excerpt: "A descriptor naming a different "}
  - {id: FR-014-AC-13, path: *p9, role: examined, excerpt: "A descriptor whose implied ident"}
  - {id: FR-014-AC-14, path: *p9, role: examined, excerpt: "A claim whose node this generato"}
  - {id: FR-014-AC-15, path: *p9, role: examined, excerpt: "A requested item whose node clos"}
  - {id: FR-014-AC-16, path: *p9, role: examined, excerpt: "A bound is read from `binding` m"}
  - {id: FR-014-AC-17, path: *p9, role: examined, excerpt: "A `reference` operand whose targ"}
  - {id: FR-014-AC-18, path: *p9, role: examined, excerpt: "`derive_exact_scalar_items` retu"}
  - {id: FR-014-AC-19, path: *p9, role: examined, excerpt: "A derived item generates an orac"}
  - {id: FR-014-AC-20, path: *p9, role: examined, excerpt: "An integer node over two distinc"}
  - {id: FR-014-AC-21, path: *p9, role: examined, excerpt: "A node whose result is typed by "}
  - {id: FR-014-AC-22, path: *p9, role: examined, excerpt: "A node whose result is typed by "}
  - {id: FR-014-AC-23, path: *p9, role: examined, excerpt: "An operand, or the result, typed"}
  - {id: FR-014-AC-24, path: *p9, role: examined, excerpt: "An operand's own bound need not "}
  - {id: FR-014-AC-25, path: *p9, role: examined, excerpt: "A `reference` operand to a plain"}
  - {id: FR-014-AC-26, path: *p9, role: examined, excerpt: "A `reference` operand whose targ"}
  - {id: FR-014-AC-27, path: *p9, role: examined, excerpt: "A node whose result is typed by "}
  - {id: FR-014-AC-28, path: *p9, role: examined, excerpt: "The referenced node's body, not "}
  - {id: FR-014-AC-29, path: *p9, role: examined, excerpt: "`x + p` narrowed to `Int[0, 29]`"}
  - {id: FR-014-AC-30, path: *p9, role: examined, excerpt: "`q + q` narrowed to `Int[0, 29]`"}
  - {id: FR-014-AC-31, path: *p9, role: examined, excerpt: "A literal operand of an integer "}
  - {id: FR-014-AC-32, path: *p9, role: examined, excerpt: "A node that narrowing conversion"}
  - {id: FR-014-AC-33, path: *p9, role: examined, excerpt: "A node narrowed to a `text_bound"}
  - {id: FR-014-AC-34, path: *p9, role: examined, excerpt: "A node that a narrowing conversi"}
  - {id: FR-014-AC-35, path: *p9, role: examined, excerpt: "Boolean `and`, `or`, `not`, `imp"}
  - {id: FR-014-AC-36, path: *p9, role: examined, excerpt: "Every oracle's source map declar"}
  - {id: FR-014-AC-37, path: *p9, role: examined, excerpt: "A differential corpus covering e"}
  - {id: FR-014-AC-38, path: *p9, role: examined, excerpt: "The Boolean condition in the thi"}
  - {id: FR-014-AC-39, path: *p9, role: examined, excerpt: "The generated `src/lib.rs` of ev"}
  - {id: FR-014-AC-40, path: *p9, role: examined, excerpt: "A package whose lowering returns"}
  - {id: FR-014-AC-41, path: *p9, role: examined, excerpt: "A `failed` record whose `limit_k"}
  - {id: FR-014-AC-42, path: *p9, role: examined, excerpt: "No `limit_kind` of a `failed` re"}
  - {id: FR-014-AC-44, path: *p9, role: examined, excerpt: "For each IR-admitted direct requ"}
  - {id: FR-014-AC-45, path: *p9, role: examined, excerpt: "For a QSL-shaped outer integer o"}
  - {id: FR-014-AC-46, path: *p9, role: examined, excerpt: "An inferred interval uses only t"}
  - {id: FR-014-AC-47, path: *p9, role: examined, excerpt: "No test name, trace tag, test ou"}
  - {id: FR-014-AC-48, path: *p9, role: examined, excerpt: "A `quire.op.quantity.convert` no"}
  - {id: FR-014-AC-49, path: *p9, role: examined, excerpt: "When QSV refuses a `quire.op.qua"}
  - {id: FR-014-AC-50, path: *p9, role: examined, excerpt: "No oracle generated for `quire.o"}
  - {id: FR-014-AC-51, path: *p9, role: examined, excerpt: "A selected `quire.op.quantity.co"}
  - {id: FR-014-AC-52, path: *p9, role: examined, excerpt: "Every generated quantity operand"}
  - {id: FR-015-AC-1, path: &p10 "spec/kani/functional/FR-015-bounded-kani-obligations.md", role: examined, excerpt: "Pre, post, invariant and frame o"}
  - {id: FR-015-AC-2, path: *p10, role: examined, excerpt: "Every harness identity records i"}
  - {id: FR-015-AC-3, path: *p10, role: examined, excerpt: "An unbounded or non-finite oblig"}
  - {id: FR-015-AC-4, path: *p10, role: examined, excerpt: "No harness assumption excludes a"}
  - {id: FR-015-AC-5, path: *p10, role: examined, excerpt: "An obligation whose bounds are u"}
  - {id: FR-015-AC-6, path: *p10, role: examined, excerpt: "An obligation over an oracle who"}
  - {id: FR-015-AC-7, path: *p10, role: examined, excerpt: "Every generated harness contains"}
  - {id: FR-015-AC-8, path: *p10, role: examined, excerpt: "A postcondition or invariant har"}
  - {id: FR-015-AC-9, path: *p10, role: examined, excerpt: "Every harness identity records s"}
  - {id: FR-015-AC-10, path: *p10, role: examined, excerpt: "Regeneration from equal inputs i"}
  - {id: FR-015-AC-11, path: *p10, role: examined, excerpt: "Every symbolic argument carries "}
  - {id: FR-015-AC-12, path: *p10, role: examined, excerpt: "A request naming no items, more "}
  - {id: FR-015-AC-13, path: *p10, role: examined, excerpt: "An otherwise-supported obligatio"}
  - {id: FR-015-AC-14, path: *p10, role: examined, excerpt: "A scalar-claim item whose graph "}
  - {id: FR-015-AC-15, path: *p10, role: examined, excerpt: "A scalar-claim item whose claim "}
  - {id: FR-015-AC-16, path: *p10, role: examined, excerpt: "A scalar harness constrains a li"}
  - {id: FR-015-AC-17, path: *p10, role: examined, excerpt: "A scalar claim whose literal ope"}
  - {id: FR-015-AC-18, path: *p10, role: examined, excerpt: "A scalar claim whose operation o"}
  - {id: FR-015-AC-19, path: *p10, role: examined, excerpt: "A zero-input transition over one"}
  - {id: FR-015-AC-20, path: *p10, role: examined, excerpt: "A harness for a Boolean-connecti"}
  - {id: FR-015-AC-21, path: *p10, role: examined, excerpt: "A precondition that reads a post"}
  - {id: FR-015-AC-22, path: *p10, role: examined, excerpt: "A declared proof-dependency cens"}
  - {id: FR-015-AC-23, path: *p10, role: examined, excerpt: "Every requested item receives ex"}
  - {id: FR-015-AC-24, path: *p10, role: examined, excerpt: "A falsifying in-domain assignmen"}
  - {id: FR-015-AC-25, path: *p10, role: examined, excerpt: "A valid declared census is folde"}
  - {id: FR-015-AC-26, path: *p10, role: examined, excerpt: "One `postcondition` `state_claus"}
  - {id: FR-015-AC-27, path: *p10, role: examined, excerpt: "The operation-contract harness a"}
  - {id: FR-015-AC-28, path: *p10, role: examined, excerpt: "The frame-effect harness takes t"}
  - {id: FR-015-AC-29, path: *p10, role: examined, excerpt: "A state clause that is not a pos"}
  - {id: FR-015-AC-30, path: *p10, role: examined, excerpt: "With the installed backend, the "}
  - {id: FR-015-AC-31, path: *p10, role: examined, excerpt: "With the installed backend, a fr"}
  - {id: FR-015-AC-32, path: *p10, role: examined, excerpt: "The forbidden frame counterexamp"}
  - {id: FR-015-AC-33, path: *p10, role: examined, excerpt: "The frame-replay payload's ancho"}
  - {id: FR-015-AC-34, path: *p10, role: examined, excerpt: "The frame-replay envelope's `cla"}
  - {id: FR-015-AC-35, path: *p10, role: examined, excerpt: "An operation the unit names no f"}
  - {id: FR-015-AC-36, path: *p10, role: examined, excerpt: "`FrameReplay::replay` returns QS"}
  - {id: FR-015-AC-37, path: *p10, role: examined, excerpt: "A scalar function-application ha"}
  - {id: FR-015-AC-38, path: *p10, role: examined, excerpt: "A V2 clause claim naming a `stat"}
  - {id: FR-015-AC-39, path: *p10, role: examined, excerpt: "A V2 clause harness draws one no"}
  - {id: FR-015-AC-40, path: *p10, role: examined, excerpt: "A harness for a V2 clause whose "}
  - {id: FR-015-AC-41, path: *p10, role: examined, excerpt: "A V2 clause whose body holds int"}
  - {id: FR-015-AC-42, path: *p10, role: examined, excerpt: "A V2 clause body holding an oper"}
  - {id: FR-015-AC-43, path: *p10, role: examined, excerpt: "A V2 postcondition harness emits"}
  - {id: FR-015-AC-44, path: *p10, role: examined, excerpt: "A census declared on a V2 clause"}
  - {id: FR-015-AC-45, path: *p10, role: examined, excerpt: "A valid census declared on a V2 "}
  - {id: FR-015-AC-46, path: *p10, role: examined, excerpt: "Every V2 clause harness contains"}
  - {id: FR-015-AC-47, path: *p10, role: examined, excerpt: "Regeneration of a V2 clause harn"}
  - {id: FR-015-AC-48, path: *p10, role: examined, excerpt: "The obligation identity of a V2 "}
  - {id: FR-015-AC-49, path: *p10, role: examined, excerpt: "A V2 invariant claim yields a ha"}
  - {id: FR-015-AC-50, path: *p10, role: examined, excerpt: "`negotiate` reports an obligatio"}
  - {id: FR-015-AC-51, path: *p10, role: examined, excerpt: "Until the final retirement slice"}
  - {id: FR-015-AC-52, path: *p10, role: examined, excerpt: "Until the final retirement slice"}
  - {id: FR-015-AC-53, path: *p10, role: examined, excerpt: "Each of the seven harness kinds "}
  - {id: FR-015-AC-54, path: *p10, role: examined, excerpt: "A V1 bundle harness (`generate_k"}
  - {id: FR-015-AC-55, path: *p10, role: examined, excerpt: "While a family's corpus arm rema"}
  - {id: FR-015-AC-56, path: *p10, role: examined, excerpt: "With the installed backend, a V1"}
  - {id: FR-015-AC-57, path: *p10, role: examined, excerpt: "While its arm remains (FR-015-AC"}
  - {id: FR-015-AC-58, path: *p10, role: examined, excerpt: "A test generates a harness throu"}
  - {id: FR-015-AC-59, path: *p10, role: examined, excerpt: "A request of `StateFrame` items "}
  - {id: FR-015-AC-60, path: *p10, role: examined, excerpt: "A `StateFrame` item that yields "}
  - {id: FR-015-AC-61, path: *p10, role: examined, excerpt: "A `StateFrame` item whose loweri"}
  - {id: FR-015-AC-62, path: *p10, role: examined, excerpt: "A `StateFrame` item whose loweri"}
  - {id: FR-015-AC-63, path: *p10, role: examined, excerpt: "A `StateFrame` item whose condit"}
  - {id: FR-015-AC-64, path: *p10, role: examined, excerpt: "A `StateFrame` item with an unpa"}
  - {id: FR-015-AC-65, path: *p10, role: examined, excerpt: "A request holding `StateFrame` i"}
  - {id: FR-015-AC-66, path: *p10, role: examined, excerpt: "A test builds every retained `St"}
  - {id: FR-015-AC-67, path: *p10, role: examined, excerpt: "`generate_state_frame_obligation"}
  - {id: FR-015-AC-68, path: *p10, role: examined, excerpt: "A `StateFrame` request whose fra"}
  - {id: FR-015-AC-69, path: *p10, role: examined, excerpt: "A composite-equality claim over "}
  - {id: FR-015-AC-70, path: *p10, role: examined, excerpt: "The harness draws one nondetermi"}
  - {id: FR-015-AC-71, path: *p10, role: examined, excerpt: "Over every drawn pair the harnes"}
  - {id: FR-015-AC-72, path: *p10, role: examined, excerpt: "The emitted shadow and harness, "}
  - {id: FR-015-AC-73, path: *p10, role: examined, excerpt: "Each row of the unsupported-shap"}
  - {id: FR-015-AC-74, path: *p10, role: examined, excerpt: "With the installed backend, the "}
  - {id: FR-015-AC-75, path: *p10, role: examined, excerpt: "With the installed backend each "}
  - {id: FR-015-AC-76, path: *p10, role: examined, excerpt: "The identity of a composite-equa"}
  - {id: FR-015-AC-77, path: *p10, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-015-AC-78, path: *p10, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-015-AC-79, path: *p10, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-015-AC-80, path: *p10, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-015-AC-81, path: *p10, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-015-AC-82, path: *p10, role: examined, excerpt: "Until FR-015-AC-111 deletes the "}
  - {id: FR-015-AC-83, path: *p10, role: examined, excerpt: "Until FR-015-AC-111 deletes the "}
  - {id: FR-015-AC-84, path: *p10, role: examined, excerpt: "Until the arithmetic slice (FR-0"}
  - {id: FR-015-AC-85, path: *p10, role: examined, excerpt: "Until the arithmetic slice (FR-0"}
  - {id: FR-015-AC-86, path: *p10, role: examined, excerpt: "Until the graph slice (FR-015-AC"}
  - {id: FR-015-AC-87, path: *p10, role: examined, excerpt: "Until the graph slice (FR-015-AC"}
  - {id: FR-015-AC-88, path: *p10, role: examined, excerpt: "Until the collection slice (FR-0"}
  - {id: FR-015-AC-89, path: *p10, role: examined, excerpt: "Until the collection slice (FR-0"}
  - {id: FR-015-AC-90, path: *p10, role: examined, excerpt: "Until FR-015-AC-111 deletes the "}
  - {id: FR-015-AC-91, path: *p10, role: examined, excerpt: "Until the final retirement slice"}
  - {id: FR-015-AC-92, path: *p10, role: examined, excerpt: "Until FR-015-AC-111 deletes the "}
  - {id: FR-015-AC-93, path: *p10, role: examined, excerpt: "Until the final retirement slice"}
  - {id: FR-015-AC-94, path: *p10, role: examined, excerpt: "Until FR-015-AC-111 deletes the "}
  - {id: FR-015-AC-95, path: *p10, role: examined, excerpt: "AD-004 step 4g retires the hand-"}
  - {id: FR-015-AC-96, path: *p10, role: examined, excerpt: "An admitted QSL V2 clause/functi"}
  - {id: FR-015-AC-97, path: *p10, role: examined, excerpt: "For an admitted collection claim"}
  - {id: FR-015-AC-98, path: *p10, role: examined, excerpt: "A graph claim (one reachability "}
  - {id: FR-015-AC-99, path: *p10, role: examined, excerpt: "The collection harness asserts t"}
  - {id: FR-015-AC-100, path: *p10, role: examined, excerpt: "For the collection family, chang"}
  - {id: FR-015-AC-101, path: *p10, role: examined, excerpt: "A request whose symbolic collect"}
  - {id: FR-015-AC-102, path: *p10, role: examined, excerpt: "An admitted collection body clai"}
  - {id: FR-015-AC-103, path: *p10, role: examined, excerpt: "Through `generate_routed`, `(x +"}
  - {id: FR-015-AC-104, path: *p10, role: examined, excerpt: "A plain-Integer parameter as any"}
  - {id: FR-015-AC-105, path: *p10, role: examined, excerpt: "For a graph claim the harness dr"}
  - {id: FR-015-AC-106, path: *p10, role: examined, excerpt: "The independent native oracle re"}
  - {id: FR-015-AC-107, path: *p10, role: examined, excerpt: "Graph claims are located by `sta"}
  - {id: FR-015-AC-108, path: *p10, role: examined, excerpt: "A graph claim settles by the dis"}
  - {id: FR-015-AC-109, path: *p10, role: examined, excerpt: "Implemented (IR-598 arithmetic s"}
  - {id: FR-015-AC-110, path: *p10, role: examined, excerpt: "PLANNED (IR-598 collection slice"}
  - {id: FR-015-AC-111, path: *p10, role: examined, excerpt: "PLANNED (IR-598 graph and final "}
  - {id: FR-015-AC-112, path: *p10, role: examined, excerpt: "Implemented (IR-858, AD-004 step"}
  - {id: FR-015-AC-113, path: *p10, role: examined, excerpt: "Implemented (IR-858). For every "}
  - {id: FR-015-AC-114, path: *p10, role: examined, excerpt: "Implemented (IR-858). `render` o"}
  - {id: FR-015-AC-115, path: *p10, role: examined, excerpt: "Implemented (IR-858). After the "}
  - {id: FR-015-AC-116, path: *p10, role: examined, excerpt: "Implemented (IR-858). `render` h"}
  - {id: FR-015-AC-117, path: *p10, role: examined, excerpt: "Implemented (IR-858). With the i"}
  - {id: FR-015-AC-118, path: *p10, role: examined, excerpt: "For an admitted V2 scalar `impli"}
  - {id: FR-015-AC-119, path: *p10, role: examined, excerpt: "PLANNED (IR-917; depends on FR-0"}
  - {id: FR-015-AC-120, path: *p10, role: examined, excerpt: "PLANNED (IR-942; depends on QSL "}
  - {id: FR-015-AC-121, path: *p10, role: examined, excerpt: "PLANNED (IR-942). A per-step `Ka"}
  - {id: FR-015-AC-122, path: *p10, role: examined, excerpt: "PLANNED (IR-942). A selected pro"}
  - {id: FR-015-AC-123, path: *p10, role: examined, excerpt: "External public-consumer calls t"}
  - {id: FR-015-AC-124, path: *p10, role: examined, excerpt: "After infallible-constructor ado"}
  - {id: FR-015-AC-125, path: *p10, role: examined, excerpt: "Authentic QSL-emitted, IR-admitt"}
  - {id: FR-015-AC-126, path: *p10, role: examined, excerpt: "Genuine TC-027 healthy and bound"}
  - {id: FR-015-AC-127, path: *p10, role: examined, excerpt: "End-to-end denied/cancelled admi"}
  - {id: FR-015-CON-1, path: *p10, role: examined, excerpt: "Clause ABI construction SHALL ch"}
  - {id: FR-015-CON-2, path: *p10, role: examined, excerpt: "Subject binding SHALL check both"}
  - {id: FR-016-AC-1, path: &p11 "spec/replay/functional/FR-016-witness-native-replay.md", role: examined, excerpt: "Every retained witness decodes i"}
  - {id: FR-016-AC-2, path: *p11, role: examined, excerpt: "An out-of-domain witness is repo"}
  - {id: FR-016-AC-3, path: *p11, role: examined, excerpt: "An in-domain witness is reported"}
  - {id: FR-016-AC-4, path: *p11, role: examined, excerpt: "A native replay that runs and di"}
  - {id: FR-016-AC-5, path: *p11, role: examined, excerpt: "A witness bound to a different h"}
  - {id: FR-016-AC-6, path: *p11, role: examined, excerpt: "Every witness decode receives re"}
  - {id: FR-016-AC-8, path: *p11, role: examined, excerpt: "A witness schema binds the oblig"}
  - {id: FR-016-AC-9, path: *p11, role: examined, excerpt: "A decoded falsification whose wi"}
  - {id: FR-016-AC-10, path: *p11, role: examined, excerpt: "A decoded falsification at which"}
  - {id: FR-016-AC-11, path: *p11, role: examined, excerpt: "The adapter refuses, with a dist"}
  - {id: FR-016-AC-12, path: *p11, role: examined, excerpt: "A `replay` call that returns `Re"}
  - {id: FR-016-AC-13, path: *p11, role: examined, excerpt: "A `Witness` arm that settles `re"}
  - {id: FR-016-AC-14, path: *p11, role: examined, excerpt: "A proved unit that imports a loc"}
  - {id: FR-016-AC-15, path: *p11, role: examined, excerpt: "The replay request's `package.de"}
  - {id: FR-016-AC-16, path: *p11, role: examined, excerpt: "A unit whose import no lock depe"}
  - {id: FR-016-AC-17, path: *p11, role: examined, excerpt: "A lock recording another `packag"}
  - {id: FR-016-AC-18, path: *p11, role: examined, excerpt: "A lock selecting a library the u"}
  - {id: FR-016-AC-19, path: *p11, role: examined, excerpt: "Lock libraries whose sources sha"}
  - {id: FR-016-AC-20, path: *p11, role: examined, excerpt: "A lock library whose source has "}
  - {id: FR-016-AC-21, path: *p11, role: examined, excerpt: "The function path's request `obl"}
  - {id: FR-016-AC-22, path: *p11, role: examined, excerpt: "For two functions with the same "}
  - {id: FR-016-AC-23, path: *p11, role: examined, excerpt: "The function-contract identity f"}
  - {id: FR-016-AC-24, path: *p11, role: examined, excerpt: "A lock whose only defect is one "}
  - {id: FR-016-AC-25, path: *p11, role: examined, excerpt: "In an installed-Kani run that cl"}
  - {id: FR-017-AC-2, path: &p12 "spec/kani/functional/FR-017-kani-execution-evidence.md", role: examined, excerpt: "An absent launcher is refused wi"}
  - {id: FR-017-AC-4, path: *p12, role: examined, excerpt: "A run is verified only when the "}
  - {id: FR-017-AC-5, path: *p12, role: examined, excerpt: "A failed non-unwinding check wit"}
  - {id: FR-017-AC-6, path: *p12, role: examined, excerpt: "The execution evidence of a sing"}
  - {id: FR-017-AC-7, path: *p12, role: examined, excerpt: "A crate whose library source doe"}
  - {id: FR-017-AC-11, path: *p12, role: examined, excerpt: "A routed FR-022/FR-014 exact-sca"}
  - {id: FR-017-AC-12, path: *p12, role: examined, excerpt: "Real Kani captures of a verified"}
  - {id: FR-017-AC-13, path: *p12, role: examined, excerpt: "A run whose process exited succe"}
  - {id: FR-017-AC-14, path: *p12, role: examined, excerpt: "A launcher whose stdout or stder"}
  - {id: FR-017-AC-15, path: *p12, role: examined, excerpt: "A timeout too large to add to th"}
  - {id: FR-017-AC-16, path: *p12, role: examined, excerpt: "The launcher's capture threads a"}
  - {id: FR-017-AC-17, path: *p12, role: examined, excerpt: "A run that times out has the who"}
  - {id: FR-017-AC-18, path: *p12, role: examined, excerpt: "A report that is malformed, of a"}
  - {id: FR-017-AC-19, path: *p12, role: examined, excerpt: "The generator appends `-Z unstab"}
  - {id: FR-017-AC-20, path: *p12, role: examined, excerpt: "The evidence and the classified "}
  - {id: FR-017-AC-21, path: *p12, role: examined, excerpt: "N harnesses with equal option ve"}
  - {id: FR-017-AC-22, path: *p12, role: examined, excerpt: "Over a real or captured batch re"}
  - {id: FR-017-AC-23, path: *p12, role: examined, excerpt: "A group's report that lacks a re"}
  - {id: FR-017-AC-24, path: *p12, role: examined, excerpt: "A launcher that exits on its own"}
  - {id: FR-017-AC-25, path: *p12, role: examined, excerpt: "A capture thread that panics, or"}
  - {id: FR-017-AC-26, path: *p12, role: examined, excerpt: "A generated FR-035 caller Text-a"}
  - {id: FR-017-AC-27, path: *p12, role: examined, excerpt: "A genuine routed `x + 1` scalar "}
  - {id: FR-017-AC-28, path: *p12, role: examined, excerpt: "Before either child starts, a pa"}
  - {id: FR-017-AC-29, path: *p12, role: examined, excerpt: "The runner measures one canonica"}
  - {id: FR-017-AC-30, path: *p12, role: examined, excerpt: "The public receipt exposes the m"}
  - {id: FR-017-AC-31, path: *p12, role: examined, excerpt: "The paired runner executes an or"}
  - {id: FR-017-AC-32, path: *p12, role: examined, excerpt: "In the `make kani` lane, a crate"}
  - {id: FR-017-AC-33, path: *p12, role: examined, excerpt: "Over single-run launcher stand-i"}
  - {id: FR-017-AC-34, path: *p12, role: examined, excerpt: "A redacted stream of exactly 819"}
  - {id: FR-017-AC-35, path: *p12, role: examined, excerpt: "With crate directory `/h/u/ws/cr"}
  - {id: FR-017-AC-36, path: *p12, role: examined, excerpt: "The diagnostic record never feed"}
  - {id: FR-017-AC-37, path: *p12, role: examined, excerpt: "Over single-run launcher stand-i"}
  - {id: FR-017-AC-38, path: *p12, role: examined, excerpt: "A single-run stand-in that exits"}
  - {id: FR-017-AC-39, path: *p12, role: examined, excerpt: "A batch stand-in of members `a::"}
  - {id: FR-017-AC-40, path: *p12, role: examined, excerpt: "The cause never feeds classifica"}
  - {id: FR-017-AC-41, path: *p12, role: examined, excerpt: "Every `KaniScalarNativeError` va"}
  - {id: FR-017-AC-42, path: *p12, role: examined, excerpt: "`class()` is `CompletionRefusal`"}
  - {id: FR-017-AC-43, path: *p12, role: examined, excerpt: "A canonical-identity encoding fa"}
  - {id: FR-017-AC-44, path: *p12, role: examined, excerpt: "The code table and the mapping t"}
  - {id: FR-017-AC-45, path: *p12, role: examined, excerpt: "The receipt exposes its proof-co"}
  - {id: FR-017-AC-46, path: *p12, role: examined, excerpt: "For a genuine routed `x + 1` pac"}
  - {id: FR-017-AC-47, path: *p12, role: examined, excerpt: "Two receipts for the same packag"}
  - {id: FR-017-AC-48, path: *p12, role: examined, excerpt: "The receipt exposes read-only th"}
  - {id: FR-017-AC-49, path: *p12, role: examined, excerpt: "A change to the preimage schema "}
  - {id: FR-017-AC-50, path: *p12, role: examined, excerpt: "The `NativeMeterObservation` of "}
  - {id: FR-017-AC-51, path: *p12, role: examined, excerpt: "A consumer compile test names an"}
  - {id: FR-017-AC-52, path: *p12, role: examined, excerpt: "Production emitted bytes and fea"}
  - {id: FR-017-AC-53, path: *p12, role: examined, excerpt: "Both package locks are independe"}
  - {id: FR-017-AC-54, path: *p12, role: examined, excerpt: "A successful claimed-init confir"}
  - {id: FR-017-AC-55, path: *p12, role: examined, excerpt: "Through the real public executio"}
  - {id: FR-017-AC-56, path: *p12, role: examined, excerpt: "Inspection of the cleanup change"}
  - {id: FR-017-CON-1, path: *p12, role: examined, excerpt: "The generator SHALL NOT convert "}
  - {id: FR-017-CON-2, path: *p12, role: examined, excerpt: "The generator SHALL NOT report a"}
  - {id: FR-018-AC-1, path: &p13 "spec/oracle/functional/FR-018-composite-equality-oracles.md", role: examined, excerpt: "Every requested item receives ex"}
  - {id: FR-018-AC-2, path: *p13, role: examined, excerpt: "On every vector of the composite"}
  - {id: FR-018-AC-3, path: *p13, role: examined, excerpt: "Each generated item's claim-map "}
  - {id: FR-018-AC-4, path: *p13, role: examined, excerpt: "Each generated item's claim-map "}
  - {id: FR-018-AC-5, path: *p13, role: examined, excerpt: "A descriptor with a `convert<T>`"}
  - {id: FR-018-AC-6, path: *p13, role: examined, excerpt: "An operand type bearing an IEEE "}
  - {id: FR-018-AC-7, path: *p13, role: examined, excerpt: "On the FR-330 package path, mode"}
  - {id: FR-018-AC-8, path: *p13, role: examined, excerpt: "A declaration closure `TypeEnvir"}
  - {id: FR-018-AC-9, path: *p13, role: examined, excerpt: "Every generated oracle function "}
  - {id: FR-018-AC-10, path: *p13, role: examined, excerpt: "Generated bytes are identical ac"}
  - {id: FR-018-AC-11, path: *p13, role: examined, excerpt: "Every claim marks its operation "}
  - {id: FR-018-AC-12, path: *p13, role: examined, excerpt: "The generated crate declares `pu"}
  - {id: FR-018-AC-13, path: *p13, role: examined, excerpt: "Every claim-map entry carries th"}
  - {id: FR-018-AC-14, path: *p13, role: examined, excerpt: "Admitted `bounded_domain` nodes "}
  - {id: FR-018-AC-15, path: *p13, role: examined, excerpt: "A `binary` node's operand is rea"}
  - {id: FR-018-AC-16, path: *p13, role: examined, excerpt: "Genuine public tuple members nam"}
  - {id: FR-018-AC-17, path: *p13, role: examined, excerpt: "The `src/lib.rs` that `generate_"}
  - {id: FR-018-AC-18, path: *p13, role: examined, excerpt: "`render_value_type` called with "}
  - {id: FR-018-AC-19, path: *p13, role: examined, excerpt: "The non-test code of `src/oracle"}
  - {id: FR-018-AC-20, path: *p13, role: examined, excerpt: "When an admitted FR-330 package "}
  - {id: FR-018-AC-21, path: *p13, role: examined, excerpt: "Each generated item's claim-map "}
  - {id: FR-018-AC-22, path: *p13, role: examined, excerpt: "For every pair of operand values"}
  - {id: FR-018-AC-23, path: *p13, role: examined, excerpt: "For every item of the TC-029 cor"}
  - {id: FR-018-AC-24, path: *p13, role: examined, excerpt: "The exact QSpec `positive-recurs"}
  - {id: FR-018-AC-25, path: *p13, role: examined, excerpt: "The per-item type-resolution cou"}
  - {id: FR-018-AC-26, path: *p13, role: examined, excerpt: "Reading the QSpec List `next` fi"}
  - {id: FR-018-AC-27, path: *p13, role: examined, excerpt: "Reading the QSpec Tree `kids` fi"}
  - {id: FR-018-AC-28, path: *p13, role: examined, excerpt: "With a caller-selected type-reso"}
  - {id: FR-018-AC-29, path: *p13, role: examined, excerpt: "A cycle detected after a charged"}
  - {id: FR-018-AC-30, path: *p13, role: examined, excerpt: "The 100,000-entry acyclic closur"}
  - {id: FR-018-AC-31, path: *p13, role: examined, excerpt: "A caller-selected type-resolutio"}
  - {id: FR-018-AC-32, path: *p13, role: examined, excerpt: "Each IR-admitted direct `tempora"}
  - {id: FR-018-AC-33, path: *p13, role: examined, excerpt: "A genuine root-facade compilatio"}
  - {id: FR-018-AC-34, path: *p13, role: examined, excerpt: "The actual generated ABI execute"}
  - {id: FR-018-AC-35, path: *p13, role: examined, excerpt: "Unknown declaration, unknown cas"}
  - {id: FR-018-AC-36, path: *p13, role: examined, excerpt: "Emitted source uses only WireNod"}
  - {id: FR-018-AC-37, path: *p13, role: examined, excerpt: "Equality's generated manifest an"}
  - {id: FR-018-AC-38, path: *p13, role: examined, excerpt: "With `None`, standalone generati"}
  - {id: FR-018-AC-39, path: *p13, role: examined, excerpt: "`None` at the first reached nomi"}
  - {id: FR-018-AC-40, path: *p13, role: examined, excerpt: "Some(wrong-package context) fail"}
  - {id: FR-018-AC-41, path: *p13, role: examined, excerpt: "A reached enum still reconstruct"}
  - {id: FR-018-AC-42, path: *p13, role: examined, excerpt: "Generated oracle invocations und"}
  - {id: FR-018-AC-43, path: *p13, role: examined, excerpt: "For genuine source with a: Int[0"}
  - {id: FR-018-AC-44, path: *p13, role: examined, excerpt: "Genuine Union equality generatio"}
  - {id: FR-019-AC-1, path: &p14 "spec/routed/functional/FR-019-capability-settlement.md", role: examined, excerpt: "Settlement dispatches one arm pe"}
  - {id: FR-019-AC-2, path: *p14, role: examined, excerpt: "Every requested item receives ex"}
  - {id: FR-019-AC-3, path: *p14, role: examined, excerpt: "On the Kani arm, an unbounded ex"}
  - {id: FR-019-AC-4, path: *p14, role: examined, excerpt: "An item with more than one candi"}
  - {id: FR-019-AC-10, path: *p14, role: examined, excerpt: "Only an item settled `supported`"}
  - {id: FR-019-AC-45, path: *p14, role: examined, excerpt: "A manifest with a second descrip"}
  - {id: FR-019-AC-46, path: *p14, role: examined, excerpt: "With several repeated identities"}
  - {id: FR-019-AC-47, path: *p14, role: examined, excerpt: "QSL's registry snapshot holds ea"}
  - {id: FR-019-AC-48, path: *p14, role: examined, excerpt: "A `Refused(refusal)` outcome fro"}
  - {id: FR-019-AC-49, path: *p14, role: examined, excerpt: "An `Incomplete(incomplete)` outc"}
  - {id: FR-019-AC-50, path: *p14, role: examined, excerpt: "An `Admitted` Kani function-appl"}
  - {id: FR-019-AC-51, path: *p14, role: examined, excerpt: "Among several request items with"}
  - {id: FR-019-AC-52, path: *p14, role: examined, excerpt: "An `Admitted` Kani function-appl"}
  - {id: FR-019-AC-53, path: *p14, role: examined, excerpt: "Given an admitted package with b"}
  - {id: FR-019-AC-7, path: *p14, role: examined, excerpt: "An envelope whose `capability_vo"}
  - {id: FR-019-AC-8, path: *p14, role: examined, excerpt: "An item with an absent extent cl"}
  - {id: FR-019-AC-9, path: *p14, role: examined, excerpt: "A backend kind added without a n"}
  - {id: FR-019-AC-11, path: *p14, role: examined, excerpt: "PLANNED (IR-629). `BackendKind` "}
  - {id: FR-019-AC-12, path: *p14, role: examined, excerpt: "PLANNED (IR-629; non-empty bound"}
  - {id: FR-019-AC-16, path: *p14, role: examined, excerpt: "PLANNED (IR-629). An unbounded p"}
  - {id: FR-019-AC-17, path: *p14, role: examined, excerpt: "PLANNED (IR-629). On a `bounded`"}
  - {id: FR-019-AC-18, path: *p14, role: examined, excerpt: "PLANNED (IR-629). Given the same"}
  - {id: FR-019-AC-19, path: *p14, role: examined, excerpt: "PLANNED (IR-629; bounded row dep"}
  - {id: FR-019-AC-20, path: *p14, role: examined, excerpt: "PLANNED (IR-629; driver projecti"}
  - {id: FR-019-AC-42, path: *p14, role: examined, excerpt: "Two otherwise equal CG Process d"}
  - {id: FR-019-AC-21, path: *p14, role: examined, excerpt: "PLANNED (IR-629). On a `bounded`"}
  - {id: FR-019-AC-22, path: *p14, role: examined, excerpt: "PLANNED (IR-629). On a `bounded`"}
  - {id: FR-019-AC-23, path: *p14, role: examined, excerpt: "PLANNED (IR-629; bounded row dep"}
  - {id: FR-019-AC-13, path: *p14, role: examined, excerpt: "Settling a process-provider item"}
  - {id: FR-019-AC-14, path: *p14, role: examined, excerpt: "The process-provider arm's retur"}
  - {id: FR-019-AC-15, path: *p14, role: examined, excerpt: "PLANNED (IR-633). CG's `BackendD"}
  - {id: FR-019-AC-24, path: *p14, role: examined, excerpt: "PLANNED (IR-338). Given QSL FR-3"}
  - {id: FR-019-AC-25, path: *p14, role: examined, excerpt: "The normative Kani form set is t"}
  - {id: FR-019-AC-26, path: *p14, role: examined, excerpt: "For any form in FR-019-AC-25 wit"}
  - {id: FR-019-AC-27, path: *p14, role: examined, excerpt: "With one Kani candidate, a typed"}
  - {id: FR-019-AC-33, path: *p14, role: examined, excerpt: "For a valid bounded typed form i"}
  - {id: FR-019-AC-28, path: *p14, role: examined, excerpt: "A form refusal under FR-019-AC-2"}
  - {id: FR-019-AC-29, path: *p14, role: examined, excerpt: "After request and candidate/mani"}
  - {id: FR-019-AC-30, path: *p14, role: examined, excerpt: "For the same typed IR tag/form, "}
  - {id: FR-019-AC-31, path: *p14, role: examined, excerpt: "For a Kani-supported unbounded f"}
  - {id: FR-019-AC-32, path: *p14, role: examined, excerpt: "When a caller supplies a finite "}
  - {id: FR-019-AC-34, path: *p14, role: examined, excerpt: "PLANNED (IR-516). With one EN-1 "}
  - {id: FR-019-AC-35, path: *p14, role: examined, excerpt: "PLANNED (IR-516). With one SMT t"}
  - {id: FR-019-AC-36, path: *p14, role: examined, excerpt: "PLANNED (IR-516). A TP-5 whole-r"}
  - {id: FR-019-AC-37, path: *p14, role: examined, excerpt: "PLANNED (IR-516). HP-1 model, HP"}
  - {id: FR-019-AC-38, path: *p14, role: examined, excerpt: "PLANNED (IR-516). An advertised "}
  - {id: FR-019-AC-39, path: *p14, role: examined, excerpt: "PLANNED (IR-516). A selected red"}
  - {id: FR-019-AC-40, path: *p14, role: examined, excerpt: "PLANNED (IR-516). For an item ov"}
  - {id: FR-019-AC-41, path: *p14, role: examined, excerpt: "PLANNED (IR-516). With otherwise"}
  - {id: FR-019-AC-43, path: *p14, role: examined, excerpt: "Two driver items selecting disti"}
  - {id: FR-019-AC-44, path: *p14, role: examined, excerpt: "For a V2 scalar claim, an absent"}
  - {id: FR-021-AC-1, path: &p15 "spec/oracle/functional/FR-021-function-application-oracles.md", role: examined, excerpt: "Every requested `call` item rece"}
  - {id: FR-021-AC-2, path: *p15, role: examined, excerpt: "For each function-application co"}
  - {id: FR-021-AC-3, path: *p15, role: examined, excerpt: "Each scheduled item records the "}
  - {id: FR-021-AC-4, path: *p15, role: examined, excerpt: "The oracle returns the arity, wr"}
  - {id: FR-021-AC-5, path: *p15, role: examined, excerpt: "The schedule orders arity valida"}
  - {id: FR-021-AC-6, path: *p15, role: examined, excerpt: "A function whose declared operat"}
  - {id: FR-021-AC-7, path: *p15, role: examined, excerpt: "A tree-shaped 100,000-node V2 `e"}
  - {id: FR-021-AC-8, path: *p15, role: examined, excerpt: "The schedule marks only linked-p"}
  - {id: FR-021-AC-9, path: *p15, role: examined, excerpt: "The schedule preserves a `functi"}
  - {id: FR-021-AC-10, path: *p15, role: examined, excerpt: "A declared parameter or result t"}
  - {id: FR-021-AC-11, path: *p15, role: examined, excerpt: "Model and relation lowering reco"}
  - {id: FR-021-AC-12, path: *p15, role: examined, excerpt: "A declared function whose own bo"}
  - {id: FR-021-AC-13, path: *p15, role: examined, excerpt: "Schedule source and claim-map by"}
  - {id: FR-021-AC-14, path: *p15, role: examined, excerpt: "The schedule crate declares `pub"}
  - {id: FR-021-AC-15, path: *p15, role: examined, excerpt: "Each scheduled call point has on"}
  - {id: FR-021-AC-16, path: *p15, role: examined, excerpt: "The schedule and its claim/locat"}
  - {id: FR-021-AC-17, path: *p15, role: examined, excerpt: "Each location record uses the re"}
  - {id: FR-021-AC-18, path: *p15, role: examined, excerpt: "No test name, trace tag, test ou"}
  - {id: FR-021-AC-19, path: *p15, role: examined, excerpt: "The emitted schedule crate for s"}
  - {id: FR-021-AC-20, path: *p15, role: examined, excerpt: "For each scalar and equality ins"}
  - {id: FR-021-AC-21, path: *p15, role: examined, excerpt: "A scalar `Negate` body is refuse"}
  - {id: FR-021-AC-22, path: *p15, role: examined, excerpt: "When two or more declarations sh"}
  - {id: FR-021-AC-23, path: *p15, role: examined, excerpt: "For an admitted FR-330 package c"}
  - {id: FR-021-AC-24, path: *p15, role: examined, excerpt: "Items on one `call` node, with e"}
  - {id: FR-021-AC-25, path: *p15, role: examined, excerpt: "Across one body, the counter cha"}
  - {id: FR-021-AC-26, path: *p15, role: examined, excerpt: "The target-neutral schedule's br"}
  - {id: FR-021-AC-27, path: *p15, role: examined, excerpt: "An admitted nested body consists"}
  - {id: FR-021-AC-28, path: *p15, role: examined, excerpt: "Under the default 1,048,576-byte"}
  - {id: FR-021-AC-29, path: *p15, role: examined, excerpt: "A 100,000-call linear body produ"}
  - {id: FR-021-AC-30, path: *p15, role: examined, excerpt: "A requested generation work limi"}
  - {id: FR-021-AC-31, path: *p15, role: examined, excerpt: "No test name, trace tag, test ou"}
  - {id: FR-021-AC-32, path: *p15, role: examined, excerpt: "Neither the schedule crate nor a"}
  - {id: FR-021-AC-33, path: *p15, role: examined, excerpt: "With an IR-admitted `temporal`/`"}
  - {id: FR-021-AC-34, path: *p15, role: examined, excerpt: "Given one authentic strict-reade"}
  - {id: FR-022-AC-1, path: &p16 "spec/routed/functional/FR-022-routed-generation.md", role: examined, excerpt: "Generation dispatches one arm pe"}
  - {id: FR-022-AC-2, path: *p16, role: examined, excerpt: "For a set of routed Kani items, "}
  - {id: FR-022-AC-3, path: *p16, role: examined, excerpt: "The entry point accepts no backe"}
  - {id: FR-022-AC-4, path: *p16, role: examined, excerpt: "PLANNED (IR-629 internal-invaria"}
  - {id: FR-022-AC-5, path: *p16, role: examined, excerpt: "Two routed items with one reques"}
  - {id: FR-022-AC-6, path: *p16, role: examined, excerpt: "A Kani group-level refusal (for "}
  - {id: FR-022-AC-7, path: *p16, role: examined, excerpt: "A Kani group containing an `inva"}
  - {id: FR-022-AC-8, path: *p16, role: examined, excerpt: "Every routed item appears exactl"}
  - {id: FR-022-AC-9, path: *p16, role: examined, excerpt: "Regeneration from equal inputs i"}
  - {id: FR-022-AC-10, path: *p16, role: examined, excerpt: "The Kani arm derives each item's"}
  - {id: FR-022-AC-11, path: *p16, role: examined, excerpt: "An item with no derivable descri"}
  - {id: FR-022-AC-12, path: *p16, role: examined, excerpt: "`RoutedGeneration.claim_map` is "}
  - {id: FR-022-AC-13, path: *p16, role: examined, excerpt: "The Kani arm routes an integer n"}
  - {id: FR-022-AC-14, path: *p16, role: examined, excerpt: "`RoutedGeneration.oracle_artifac"}
  - {id: FR-022-AC-15, path: *p16, role: examined, excerpt: "The Kani arm routes the packages"}
  - {id: FR-022-AC-16, path: *p16, role: examined, excerpt: "Two Kani harnesses with one `har"}
  - {id: FR-022-AC-17, path: *p16, role: examined, excerpt: "PLANNED (IR-629). A routed `Proc"}
  - {id: FR-022-AC-18, path: *p16, role: examined, excerpt: "PLANNED (IR-629). A crate-intern"}
  - {id: FR-022-AC-19, path: *p16, role: examined, excerpt: "PLANNED (IR-629). Permuting a ro"}
  - {id: FR-022-AC-20, path: *p16, role: examined, excerpt: "PLANNED (IR-629). `RoutedGenerat"}
  - {id: FR-022-AC-21, path: *p16, role: examined, excerpt: "PLANNED (IR-629). `from_descript"}
  - {id: FR-022-AC-22, path: *p16, role: examined, excerpt: "PLANNED (IR-629). `from_descript"}
  - {id: FR-022-AC-23, path: *p16, role: examined, excerpt: "After FR-019 authenticates two `"}
  - {id: FR-022-AC-24, path: *p16, role: examined, excerpt: "A routed scalar item's retained "}
  - {id: FR-022-AC-25, path: *p16, role: examined, excerpt: "The public CG API exposes the co"}
  - {id: FR-022-AC-26, path: *p16, role: examined, excerpt: "Given the checked QSL RR-16 pack"}
  - {id: FR-022-AC-27, path: *p16, role: examined, excerpt: "For `if p then x + 1 else x + 1`"}
  - {id: FR-022-AC-28, path: *p16, role: examined, excerpt: "If a routed scalar item's occurr"}
  - {id: FR-022-AC-29, path: *p16, role: examined, excerpt: "Given an admitted package whose "}
  - {id: FR-024-AC-1, path: &p17 "spec/replay/functional/FR-024-counterexample-envelope-intake.md", role: examined, excerpt: "The envelope's obligation identi"}
  - {id: FR-024-AC-2, path: *p17, role: examined, excerpt: "Every backend-witness transcript"}
  - {id: FR-024-AC-3, path: *p17, role: examined, excerpt: "A counterexample with any value "}
  - {id: FR-024-AC-4, path: *p17, role: examined, excerpt: "For each `WitnessPacket` member "}
  - {id: FR-024-AC-6, path: *p17, role: examined, excerpt: "A frame counterexample is submit"}
  - {id: FR-024-AC-7, path: *p17, role: examined, excerpt: "A reduced `Witness`-arm candidat"}
  - {id: FR-024-AC-8, path: *p17, role: examined, excerpt: "A retained reduced `Witness`-arm"}
  - {id: FR-024-AC-10, path: *p17, role: examined, excerpt: "No type named `Witness`, `Replay"}
  - {id: FR-024-AC-11, path: *p17, role: examined, excerpt: "`StateClauseReplay::replay` retu"}
  - {id: FR-024-AC-12, path: *p17, role: examined, excerpt: "The state-clause envelope's `cla"}
  - {id: FR-024-AC-13, path: *p17, role: examined, excerpt: "The pre snapshot holds, for ever"}
  - {id: FR-024-AC-14, path: *p17, role: examined, excerpt: "For a document vector whose memb"}
  - {id: FR-024-AC-15, path: *p17, role: examined, excerpt: "A playback that binds no value f"}
  - {id: FR-024-AC-16, path: *p17, role: examined, excerpt: "For a subject mutated to violate"}
  - {id: FR-024-AC-17, path: *p17, role: examined, excerpt: "A state field value outside its "}
  - {id: FR-024-AC-18, path: *p17, role: examined, excerpt: "With the installed backend, the "}
  - {id: FR-024-AC-19, path: *p17, role: examined, excerpt: "An operation that declares a par"}
  - {id: FR-024-AC-20, path: *p17, role: examined, excerpt: "One obligation-identity function"}
  - {id: FR-024-AC-21, path: *p17, role: examined, excerpt: "The identity changes when any on"}
  - {id: FR-024-AC-22, path: *p17, role: examined, excerpt: "A frame obligation's identity ta"}
  - {id: FR-024-AC-23, path: *p17, role: examined, excerpt: "`StateFrameIdentity` and its per"}
  - {id: FR-024-AC-24, path: *p17, role: examined, excerpt: "`FrameReplayInputs` has no `obli"}
  - {id: FR-024-AC-25, path: *p17, role: examined, excerpt: "The frame witness is built from "}
  - {id: FR-024-AC-26, path: *p17, role: examined, excerpt: "A decoded state-field value outs"}
  - {id: FR-024-AC-27, path: *p17, role: examined, excerpt: "The decoded pre-state is tied to"}
  - {id: FR-024-AC-28, path: *p17, role: examined, excerpt: "A harness whose `scope.operation"}
  - {id: FR-024-AC-29, path: *p17, role: examined, excerpt: "The frame envelope's `declared_d"}
  - {id: FR-024-AC-30, path: *p17, role: examined, excerpt: "With the installed backend, the "}
  - {id: FR-024-AC-31, path: *p17, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-024-AC-32, path: *p17, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-024-AC-33, path: *p17, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-024-AC-34, path: *p17, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-024-AC-35, path: *p17, role: examined, excerpt: "PLANNED (IR-624), IR-628 accesso"}
  - {id: FR-024-AC-36, path: *p17, role: examined, excerpt: "Two distinct test backend adapte"}
  - {id: FR-024-AC-37, path: *p17, role: examined, excerpt: "The common intake maps an adapte"}
  - {id: FR-024-AC-38, path: *p17, role: examined, excerpt: "Starting from one admitted origi"}
  - {id: FR-024-AC-39, path: *p17, role: examined, excerpt: "From a retained failure revision"}
  - {id: FR-024-AC-40, path: *p17, role: examined, excerpt: "A retained reduced `Witness` env"}
  - {id: FR-025-AC-1, path: &p18 "spec/kani/functional/FR-025-generated-subject-abi.md", role: examined, excerpt: "For every harness kind, the iden"}
  - {id: FR-025-AC-2, path: *p18, role: examined, excerpt: "Every argument binding in a pers"}
  - {id: FR-025-AC-3, path: *p18, role: examined, excerpt: "A Boolean argument is a `bool` t"}
  - {id: FR-025-AC-4, path: *p18, role: examined, excerpt: "An argument whose domain has no "}
  - {id: FR-025-AC-5, path: *p18, role: examined, excerpt: "A frame obligation is accounted "}
  - {id: FR-025-AC-6, path: *p18, role: examined, excerpt: "A binding that names no paramete"}
  - {id: FR-025-AC-7, path: *p18, role: examined, excerpt: "An argument of a family with no "}
  - {id: FR-025-AC-8, path: *p18, role: examined, excerpt: "A harness over a state-reading o"}
  - {id: FR-025-AC-9, path: *p18, role: examined, excerpt: "PLANNED (IR-806). Every emitted "}
  - {id: FR-026-AC-1, path: &p19 "spec/routed/functional/FR-026-backend-adapter-contract.md", role: examined, excerpt: "The Kani adapter implements the "}
  - {id: FR-026-AC-4, path: *p19, role: examined, excerpt: "The execution evidence type is a"}
  - {id: FR-026-AC-5, path: *p19, role: examined, excerpt: "PLANNED (IR-629). `Process(id)` "}
  - {id: FR-028-AC-1, path: &p20 "spec/kani/functional/FR-028-bounded-proof-ceilings.md", role: examined, excerpt: "Every harness identity records t"}
  - {id: FR-028-AC-2, path: *p20, role: examined, excerpt: "A backend Kani run that exceeds "}
  - {id: FR-028-AC-3, path: *p20, role: examined, excerpt: "A backend Kani run that exceeds "}
  - {id: FR-028-AC-4, path: *p20, role: examined, excerpt: "Every execution evidence records"}
  - {id: FR-028-AC-5, path: *p20, role: examined, excerpt: "Every harness identity, generati"}
  - {id: FR-028-AC-6, path: *p20, role: examined, excerpt: "A harness of a family with no su"}
  - {id: FR-028-AC-7, path: *p20, role: examined, excerpt: "A family with a supplied shadow "}
  - {id: FR-028-AC-8, path: *p20, role: examined, excerpt: "An argument narrowed inside its "}
  - {id: FR-028-AC-9, path: *p20, role: examined, excerpt: "A narrowing outside the argument"}
  - {id: FR-028-AC-12, path: *p20, role: examined, excerpt: "In a group of N harnesses (one l"}
  - {id: FR-028-AC-13, path: *p20, role: examined, excerpt: "The source of a `bounded_shadow`"}
  - {id: FR-028-AC-14, path: *p20, role: examined, excerpt: "The result of a `bounded_shadow`"}
  - {id: FR-028-AC-15, path: *p20, role: examined, excerpt: "For every case the refinement ru"}
  - {id: FR-028-AC-16, path: *p20, role: examined, excerpt: "A refinement run over a domain o"}
  - {id: FR-028-AC-17, path: *p20, role: examined, excerpt: "A verified `production` harness "}
  - {id: FR-028-AC-18, path: *p20, role: examined, excerpt: "A harness identity of a shadow f"}
  - {id: FR-028-AC-19, path: *p20, role: examined, excerpt: "A refinement domain that does no"}
  - {id: FR-028-AC-20, path: *p20, role: examined, excerpt: "An item whose shape lies outside"}
  - {id: FR-028-AC-21, path: *p20, role: examined, excerpt: "The backend process tree of ever"}
  - {id: FR-028-AC-22, path: *p20, role: examined, excerpt: "A shadow family's machinery catc"}
  - {id: FR-028-AC-23, path: *p20, role: examined, excerpt: "Every execution evidence records"}
  - {id: FR-028-AC-24, path: *p20, role: examined, excerpt: "The refinement run is a native p"}
  - {id: FR-029-AC-1, path: &p21 "spec/kani/functional/FR-029-run-outcome-terminal-record.md", role: examined, excerpt: "`verified` with three SUCCESS ch"}
  - {id: FR-029-AC-2, path: *p21, role: examined, excerpt: "`inconclusive` with the vacuous-"}
  - {id: FR-029-AC-3, path: *p21, role: examined, excerpt: "The timed-out reason maps to `In"}
  - {id: FR-029-AC-4, path: *p21, role: examined, excerpt: "The no-verdict reason maps to `F"}
  - {id: FR-029-AC-5, path: *p21, role: examined, excerpt: "The missing-cover-summary reason"}
  - {id: FR-029-AC-6, path: *p21, role: examined, excerpt: "The ordinary production/source-p"}
  - {id: FR-029-AC-8, path: *p21, role: examined, excerpt: "`falsified` with a replay disagr"}
  - {id: FR-029-AC-9, path: *p21, role: examined, excerpt: "`falsified` with a non-fault `Re"}
  - {id: FR-029-AC-10, path: *p21, role: examined, excerpt: "`falsified` with a fault maps to"}
  - {id: FR-029-AC-11, path: *p21, role: examined, excerpt: "`falsified` with each CG-raised "}
  - {id: FR-029-AC-12, path: *p21, role: examined, excerpt: "Across every replay settlement o"}
  - {id: FR-029-AC-13, path: *p21, role: examined, excerpt: "`falsified` with a non-fault `Ca"}
  - {id: FR-029-AC-14, path: *p21, role: examined, excerpt: "`falsified` with a `DependencyLo"}
  - {id: FR-029-AC-15, path: *p21, role: examined, excerpt: "For the ordinary source-predicat"}
  - {id: FR-029-AC-16, path: *p21, role: examined, excerpt: "The state-clause replay path (FR"}
  - {id: FR-029-AC-17, path: *p21, role: examined, excerpt: "PLANNED (IR-806). IR-635's Eq-on"}
  - {id: FR-029-AC-18, path: *p21, role: examined, excerpt: "For the `falsified` state-clause"}
  - {id: FR-029-AC-19, path: *p21, role: examined, excerpt: "PLANNED/GATED (IR-806/QSL-640). "}
  - {id: FR-029-AC-20, path: *p21, role: examined, excerpt: "PARTIAL (IR-666 direct F-1 to F-"}
  - {id: FR-029-AC-21, path: *p21, role: examined, excerpt: "PLANNED (IR-806): `run_terminal_"}
  - {id: FR-029-AC-22, path: *p21, role: examined, excerpt: "PLANNED (IR-806). The strength s"}
  - {id: FR-029-AC-23, path: *p21, role: examined, excerpt: "PLANNED/GATED (IR-806/QSL-640). "}
  - {id: FR-029-AC-24, path: *p21, role: examined, excerpt: "PLANNED/GATED (IR-806/QSL-640). "}
  - {id: FR-029-AC-25, path: *p21, role: examined, excerpt: "PLANNED (IR-806). With no retain"}
  - {id: FR-029-AC-26, path: *p21, role: examined, excerpt: "PLANNED/GATED (IR-806/QSL-640). "}
  - {id: FR-029-AC-27, path: *p21, role: examined, excerpt: "PLANNED/GATED (IR-806/QSL-640). "}
  - {id: FR-029-AC-28, path: *p21, role: examined, excerpt: "IR-666 public converter delivere"}
  - {id: FR-029-AC-29, path: *p21, role: examined, excerpt: "A direct `KaniRunOutcome::Cancel"}
  - {id: FR-029-AC-30, path: *p21, role: examined, excerpt: "PLANNED (IR-806). A `bounded_sha"}
  - {id: FR-029-AC-31, path: *p21, role: examined, excerpt: "PLANNED (IR-806). The parity rou"}
  - {id: FR-029-AC-32, path: *p21, role: examined, excerpt: "PLANNED (IR-848). A `FailedWitho"}
  - {id: FR-029-AC-33, path: *p21, role: examined, excerpt: "PLANNED (IR-848). A `FailedWitho"}
  - {id: FR-029-AC-34, path: *p21, role: examined, excerpt: "PLANNED (IR-848). CG's own memor"}
  - {id: FR-029-AC-35, path: *p21, role: examined, excerpt: "PLANNED (IR-848). The cause neve"}
  - {id: FR-029-AC-36, path: *p21, role: examined, excerpt: "PLANNED (IR-848). The rationale "}
  - {id: FR-030-AC-1, path: &p22 "spec/kani/functional/FR-030-ir-outcome-terminal-map.md", role: examined, excerpt: "Every pair (`KaniOutcomeKind`, r"}
  - {id: FR-030-AC-2, path: *p22, role: examined, excerpt: "`Refused`, `InvalidInput` and `I"}
  - {id: FR-030-AC-3, path: *p22, role: examined, excerpt: "`TimedOut` and `ResourceExhauste"}
  - {id: FR-030-AC-4, path: *p22, role: examined, excerpt: "`Proved` with three SUCCESS chec"}
  - {id: FR-030-AC-5, path: *p22, role: examined, excerpt: "`Inconclusive` with cause `kani_"}
  - {id: FR-030-AC-6, path: *p22, role: examined, excerpt: "No outcome maps to `Tested`."}
  - {id: FR-030-AC-7, path: *p22, role: examined, excerpt: "The map is one `match` over the "}
  - {id: FR-030-AC-8, path: *p22, role: examined, excerpt: "`Unavailable` with cause `kani_c"}
  - {id: FR-030-AC-9, path: *p22, role: examined, excerpt: "`Counterexample` with a replay d"}
  - {id: FR-030-AC-10, path: *p22, role: examined, excerpt: "`Counterexample` with a fault, w"}
  - {id: FR-030-AC-11, path: *p22, role: examined, excerpt: "Across every replay settlement o"}
  - {id: FR-030-AC-12, path: *p22, role: examined, excerpt: "`Counterexample` with a non-faul"}
  - {id: FR-030-AC-13, path: *p22, role: examined, excerpt: "`Counterexample` with a `Depende"}
  - {id: FR-030-AC-14, path: *p22, role: examined, excerpt: "A `Counterexample` given no repl"}
  - {id: FR-030-AC-15, path: *p22, role: examined, excerpt: "PARTIAL (IR-666 direct public re"}
  - {id: FR-030-AC-16, path: *p22, role: examined, excerpt: "PARTIAL (IR-666 direct public re"}
  - {id: FR-030-AC-17, path: *p22, role: examined, excerpt: "PARTIAL (IR-666 F-7 and binding "}
  - {id: FR-030-AC-18, path: *p22, role: examined, excerpt: "Every Kani `Proved` row carries "}
  - {id: FR-030-AC-19, path: *p22, role: examined, excerpt: "For both `BoundedComplete { dept"}
  - {id: FR-030-AC-20, path: *p22, role: examined, excerpt: "For either SMT basis, a foreign "}
  - {id: FR-030-AC-21, path: *p22, role: examined, excerpt: "SMT/cross-check disagreement wit"}
  - {id: FR-030-AC-22, path: *p22, role: examined, excerpt: "Every expressible accepted SMT b"}
  - {id: FR-031-AC-1, path: &p23 "spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md", role: examined, excerpt: "IMPLEMENTED (IR-596). For each o"}
  - {id: FR-031-AC-2, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-031-AC-3, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). A native o"}
  - {id: FR-031-AC-4, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). Each of th"}
  - {id: FR-031-AC-5, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). Under a `r"}
  - {id: FR-031-AC-6, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-031-AC-7, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). An add, su"}
  - {id: FR-031-AC-8, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). In a gener"}
  - {id: FR-031-AC-9, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). The first "}
  - {id: FR-031-AC-10, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). The tri-st"}
  - {id: FR-031-AC-11, path: *p23, role: examined, excerpt: "IMPLEMENTED IN CG (IR-596); THE "}
  - {id: FR-031-AC-12, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). Apart from"}
  - {id: FR-031-AC-13, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). Generating"}
  - {id: FR-031-AC-14, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). The genera"}
  - {id: FR-031-AC-15, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). In the nat"}
  - {id: FR-031-AC-16, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-031-AC-17, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-031-AC-18, path: *p23, role: examined, excerpt: "PLANNED (IR-602). A divide or re"}
  - {id: FR-031-AC-27, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-031-AC-28, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-664). The V1 Boo"}
  - {id: FR-031-AC-19, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). The Kani b"}
  - {id: FR-031-AC-20, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). Under real"}
  - {id: FR-031-AC-21, path: *p23, role: examined, excerpt: "IMPLEMENTED (IR-596). For each o"}
  - {id: FR-031-AC-22, path: *p23, role: examined, excerpt: "PLANNED (IR-602), CONTINGENT on "}
  - {id: FR-031-AC-26, path: *p23, role: examined, excerpt: "PLANNED (IR-602), CONTINGENT on "}
  - {id: FR-031-AC-23, path: *p23, role: examined, excerpt: "PLANNED (IR-602). The Kani bundl"}
  - {id: FR-031-AC-24, path: *p23, role: examined, excerpt: "PLANNED (IR-602). Under real Kan"}
  - {id: FR-031-AC-25, path: *p23, role: examined, excerpt: "PLANNED (IR-602), GATED on the l"}
  - {id: FR-032-AC-1, path: &p24 "spec/replay/functional/FR-032-routed-scalar-replay-binding.md", role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-2, path: *p24, role: examined, excerpt: "PLANNED (IR-631). The builder de"}
  - {id: FR-032-AC-3, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-4, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-5, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-6, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-7, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-8, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (C"}
  - {id: FR-032-AC-9, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (a"}
  - {id: FR-032-AC-10, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (t"}
  - {id: FR-032-AC-11, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (t"}
  - {id: FR-032-AC-12, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (t"}
  - {id: FR-032-AC-13, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (t"}
  - {id: FR-032-AC-14, path: *p24, role: examined, excerpt: "PLANNED (IR-631) / CODE-GATED (a"}
  - {id: FR-033-AC-1, path: &p25 "spec/replay/functional/FR-033-composite-parity-replay-binding.md", role: examined, excerpt: "PARTIAL (IR-635 verified Eq grap"}
  - {id: FR-033-AC-2, path: *p25, role: examined, excerpt: "PLANNED/GATED. Canonical round t"}
  - {id: FR-033-AC-3, path: *p25, role: examined, excerpt: "PLANNED/GATED. Noncanonical JCS/"}
  - {id: FR-033-AC-4, path: *p25, role: examined, excerpt: "PLANNED/GATED. Public caller inp"}
  - {id: FR-033-AC-5, path: *p25, role: examined, excerpt: "PLANNED/GATED. The retained actu"}
  - {id: FR-033-AC-6, path: *p25, role: examined, excerpt: "PLANNED/GATED. For falsified rep"}
  - {id: FR-033-AC-7, path: *p25, role: examined, excerpt: "PARTIAL (IR-666 direct public F-"}
  - {id: FR-033-AC-8, path: *p25, role: examined, excerpt: "PLANNED/GATED. CG submits the or"}
  - {id: FR-033-AC-9, path: *p25, role: examined, excerpt: "PARTIAL (IR-666 report binding c"}
  - {id: FR-033-AC-10, path: *p25, role: examined, excerpt: "PLANNED/GATED. Each lawful gener"}
  - {id: FR-033-AC-11, path: *p25, role: examined, excerpt: "PARTIAL (IR-635 Eq graph-child p"}
  - {id: FR-033-AC-12, path: *p25, role: examined, excerpt: "PARTIAL (IR-666 direct QSL F-1 p"}
  - {id: FR-033-AC-13, path: *p25, role: examined, excerpt: "PARTIAL (IR-666 direct public F-"}
  - {id: FR-033-AC-14, path: *p25, role: examined, excerpt: "PLANNED (IR-814; literal operand"}
  - {id: FR-033-AC-15, path: *p25, role: examined, excerpt: "PLANNED (IR-814). When the origi"}
  - {id: FR-033-AC-16, path: *p25, role: examined, excerpt: "PLANNED (IR-814; verified by Ins"}
  - {id: FR-033-AC-17, path: *p25, role: examined, excerpt: "PLANNED (IR-814). If the owning "}
  - {id: FR-034-AC-1, path: &p26 "spec/kani/functional/FR-034-caller-death-ownership.md", role: examined, excerpt: "Before the exclusive caller leas"}
  - {id: FR-034-AC-2, path: *p26, role: examined, excerpt: "Before any M spawn, O is positiv"}
  - {id: FR-034-AC-3, path: *p26, role: examined, excerpt: "In ClaimedBootstrap, InitReady o"}
  - {id: FR-034-AC-4, path: *p26, role: examined, excerpt: "Mutual authority is the specific"}
  - {id: FR-034-AC-5, path: *p26, role: examined, excerpt: "C retains actual unreaped Child "}
  - {id: FR-034-AC-6, path: *p26, role: examined, excerpt: "The owned tree memory observer i"}
  - {id: FR-034-AC-7, path: *p26, role: examined, excerpt: "Bounded inner startup/identity r"}
  - {id: FR-034-AC-8, path: *p26, role: examined, excerpt: "After Dispatch, original-caller "}
  - {id: FR-034-AC-9, path: *p26, role: examined, excerpt: "Cancelling one owned namespace l"}
  - {id: FR-034-AC-10, path: *p26, role: examined, excerpt: "Normal backend completion, backe"}
  - {id: FR-034-AC-11, path: *p26, role: examined, excerpt: "Guardian SIGKILL, abort or OOM-e"}
  - {id: FR-034-AC-12, path: *p26, role: examined, excerpt: "Every startup refusal, completio"}
  - {id: FR-034-AC-13, path: *p26, role: examined, excerpt: "The bounded executor requires an"}
  - {id: FR-034-AC-14, path: *p26, role: examined, excerpt: "Setup docs require the matched p"}
  - {id: FR-034-AC-15, path: *p26, role: examined, excerpt: "Typed private controls reject ma"}
  - {id: FR-034-AC-16, path: *p26, role: examined, excerpt: "Only the original caller holds t"}
  - {id: FR-034-AC-17, path: *p26, role: examined, excerpt: "Backend receives unchanged raw a"}
  - {id: FR-034-AC-18, path: *p26, role: examined, excerpt: "Guardian diagnostics and control"}
  - {id: FR-034-AC-19, path: *p26, role: examined, excerpt: "Original identity ceilings and e"}
  - {id: FR-034-AC-20, path: *p26, role: examined, excerpt: "Guardian connection, startup, id"}
  - {id: FR-034-AC-21, path: *p26, role: examined, excerpt: "Guardian connection and handshak"}
  - {id: FR-034-AC-22, path: *p26, role: examined, excerpt: "Control/capture shutdown and cle"}
  - {id: FR-034-AC-23, path: *p26, role: examined, excerpt: "Caller-death fixtures use the ac"}
  - {id: FR-034-AC-24, path: *p26, role: examined, excerpt: "Actual lease-EOF cancellation pa"}
  - {id: FR-034-AC-25, path: *p26, role: examined, excerpt: "Before backend Dispatch, the hel"}
  - {id: FR-034-AC-26, path: *p26, role: examined, excerpt: "The guardian session/group is di"}
  - {id: FR-034-AC-27, path: *p26, role: examined, excerpt: "guardian-test-support is off by "}
  - {id: FR-034-AC-28, path: *p26, role: examined, excerpt: "Fixture prefix selects unchanged"}
  - {id: FR-034-AC-29, path: *p26, role: examined, excerpt: "The packaged caller fixture and "}
  - {id: FR-034-AC-30, path: *p26, role: examined, excerpt: "CG publishes the test-only featu"}
  - {id: FR-034-AC-31, path: *p26, role: examined, excerpt: "Real production fixtures fail th"}
  - {id: FR-034-AC-32, path: *p26, role: examined, excerpt: "The bounded O event loop collect"}
  - {id: FR-034-AC-33, path: *p26, role: examined, excerpt: "Authenticated Completed precedes"}
  - {id: FR-034-AC-34, path: *p26, role: examined, excerpt: "Live-C production close_lease_an"}
  - {id: FR-034-AC-35, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. L creates private"}
  - {id: FR-034-AC-36, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. C inventories act"}
  - {id: FR-034-AC-37, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Trusted I retains"}
  - {id: FR-034-AC-38, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Named fixed SETTL"}
  - {id: FR-034-AC-39, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Native guardian/b"}
  - {id: FR-034-AC-40, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Only declared tru"}
  - {id: FR-034-AC-41, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). For a na"}
  - {id: FR-034-AC-42, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). The proo"}
  - {id: FR-034-AC-43, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). Every na"}
  - {id: FR-034-AC-44, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). A reacha"}
  - {id: FR-034-AC-45, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). The nati"}
  - {id: FR-034-AC-46, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). The anal"}
  - {id: FR-034-AC-47, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). Each rel"}
  - {id: FR-034-AC-48, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). The name"}
  - {id: FR-034-AC-49, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). A change"}
  - {id: FR-034-AC-50, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-682). Missing "}
  - {id: FR-034-AC-51, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Given genuine liv"}
  - {id: FR-034-AC-52, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual retained-I"}
  - {id: FR-034-AC-53, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual retained M"}
  - {id: FR-034-AC-54, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. On the independen"}
  - {id: FR-034-AC-55, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Independently che"}
  - {id: FR-034-AC-56, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Published-source "}
  - {id: FR-034-AC-57, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). When L a"}
  - {id: FR-034-AC-58, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). L accept"}
  - {id: FR-034-AC-59, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). For L\u2192C "}
  - {id: FR-034-AC-60, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). The nega"}
  - {id: FR-034-AC-61, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). Negative"}
  - {id: FR-034-AC-62, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). C adopts"}
  - {id: FR-034-AC-63, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). C expose"}
  - {id: FR-034-AC-64, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). The new "}
  - {id: FR-034-AC-65, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). Before f"}
  - {id: FR-034-AC-66, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). Only the"}
  - {id: FR-034-AC-67, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). C authen"}
  - {id: FR-034-AC-68, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). C does n"}
  - {id: FR-034-AC-69, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). Missing "}
  - {id: FR-034-AC-70, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). The firs"}
  - {id: FR-034-AC-71, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). O includ"}
  - {id: FR-034-AC-72, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). After CO"}
  - {id: FR-034-AC-73, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). A claime"}
  - {id: FR-034-AC-74, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). The nega"}
  - {id: FR-034-AC-75, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). A later "}
  - {id: FR-034-AC-76, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). A genuin"}
  - {id: FR-034-AC-77, path: *p26, role: examined, excerpt: "PLANNED/UNRUN (IR-687). On a fre"}
  - {id: FR-034-AC-78, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. C admits actual o"}
  - {id: FR-034-AC-79, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual production"}
  - {id: FR-034-AC-80, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Genuine original "}
  - {id: FR-034-AC-81, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Every surviving a"}
  - {id: FR-034-AC-82, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual supported-"}
  - {id: FR-034-AC-83, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. The original excl"}
  - {id: FR-034-AC-84, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Every remaining f"}
  - {id: FR-034-AC-85, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual retained M"}
  - {id: FR-034-AC-86, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. O establishes sam"}
  - {id: FR-034-AC-87, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual I confirma"}
  - {id: FR-034-AC-88, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. O preserves every"}
  - {id: FR-034-AC-89, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Genuine original "}
  - {id: FR-034-AC-90, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Any remaining opt"}
  - {id: FR-034-AC-91, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Ordinary feature-"}
  - {id: FR-034-AC-92, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Genuine adopted-o"}
  - {id: FR-034-AC-93, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Every surviving f"}
  - {id: FR-034-AC-94, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. C disposes the de"}
  - {id: FR-034-AC-95, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Actual O retains "}
  - {id: FR-034-AC-96, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Missing, malforme"}
  - {id: FR-034-AC-97, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Retired observati"}
  - {id: FR-034-AC-98, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Feature-off paire"}
  - {id: FR-034-AC-99, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Any remaining opt"}
  - {id: FR-034-AC-100, path: *p26, role: examined, excerpt: "PLANNED/UNRUN. Unsupported, malf"}
  - {id: FR-035-AC-1, path: &p27 "spec/oracle/functional/FR-035-caller-text-admission.md", role: examined, excerpt: "A generated caller-ingress oracl"}
  - {id: FR-035-AC-2, path: *p27, role: examined, excerpt: "Invalid UTF-8 bytes make `TextPa"}
  - {id: FR-035-AC-3, path: *p27, role: examined, excerpt: "For each profile, `Text[1,1]` ad"}
  - {id: FR-035-AC-4, path: *p27, role: examined, excerpt: "For each of the six profiles and"}
  - {id: FR-035-AC-5, path: *p27, role: examined, excerpt: "In a copy of the generated oracl"}
  - {id: FR-035-AC-6, path: *p27, role: examined, excerpt: "A request outside the declared f"}
  - {id: FR-036-AC-1, path: &p28 "spec/evidence/functional/FR-036-source-obligation-census.md", role: examined, excerpt: "A genuine typed fixture with cri"}
  - {id: FR-036-AC-2, path: *p28, role: examined, excerpt: "For the unchanged AC-1 fixture, "}
  - {id: FR-036-AC-3, path: *p28, role: examined, excerpt: "Capability facts preserve unsupp"}
  - {id: FR-036-AC-4, path: *p28, role: examined, excerpt: "A matching executed production f"}
  - {id: FR-036-AC-5, path: *p28, role: examined, excerpt: "Shadow-only evidence, a narrower"}
  - {id: FR-036-AC-6, path: *p28, role: examined, excerpt: "A downstream consumer above CG a"}
  - {id: FR-036-AC-7, path: *p28, role: examined, excerpt: "Missing/incomplete authoritative"}
  - {id: FR-037-AC-1, path: &p29 "spec/core/functional/FR-037-fixture-identity.md", role: examined, excerpt: "Both `tests/exact_scalar_support"}
  - {id: FR-037-AC-2, path: *p29, role: examined, excerpt: "A fixture containing an applicat"}
  - {id: FR-037-AC-3, path: *p29, role: examined, excerpt: "A canonical enum-member fixture "}
  - {id: FR-037-AC-4, path: *p29, role: examined, excerpt: "Positive exact-scalar and compos"}
  - {id: FR-037-AC-5, path: *p29, role: examined, excerpt: "The consumer change leaves deter"}
  - {id: FR-038-AC-1, path: &p30 "spec/core/functional/FR-038-typed-operator-catalog.md", role: examined, excerpt: "A source inventory compares ever"}
  - {id: FR-038-AC-2, path: *p30, role: examined, excerpt: "The typed source distinguishes i"}
  - {id: FR-038-AC-3, path: *p30, role: examined, excerpt: "Focused V2 oracle, scalar Kani a"}
  - {id: FR-038-AC-4, path: *p30, role: examined, excerpt: "The four scalar Kani harnesses s"}
  - {id: FR-038-AC-5, path: *p30, role: examined, excerpt: "The V1 `strategy/bound` relation"}
  - {id: FR-038-AC-6, path: *p30, role: examined, excerpt: "While the V1 `strategy/bound` re"}
  - {id: FR-039-AC-1, path: &p31 "spec/core/functional/FR-039-contract-package-consumption.md", role: examined, excerpt: "Each of two genuine QSL-admitted"}
  - {id: FR-039-AC-2, path: *p31, role: examined, excerpt: "A mixed request with a represent"}
  - {id: FR-039-AC-3, path: *p31, role: examined, excerpt: "A selected dependency function r"}
  - {id: FR-039-AC-4, path: *p31, role: examined, excerpt: "For a genuinely admitted FR-330 "}
  - {id: FR-039-AC-5, path: *p31, role: examined, excerpt: "A malformed or noncanonical FR-3"}
  - {id: FR-039-AC-6, path: *p31, role: examined, excerpt: "An item whose admitted package a"}
  - {id: FR-039-AC-7, path: *p31, role: examined, excerpt: "Generated scalar, composite equa"}
  - {id: FR-039-AC-8, path: *p31, role: examined, excerpt: "For each admitted `lowered` boun"}
  - {id: FR-039-AC-9, path: *p31, role: examined, excerpt: "In an admitted package with a bo"}
  - {id: FR-039-AC-10, path: *p31, role: examined, excerpt: "Given a bound driver item follow"}
  - {id: FR-040-AC-1, path: &p32 "spec/kani/functional/FR-040-cancellation-producer.md", role: examined, excerpt: "A caller retains a live `Cancel`"}
  - {id: FR-040-AC-2, path: *p32, role: examined, excerpt: "In two live single runs, issue `"}
  - {id: FR-040-AC-3, path: *p32, role: examined, excerpt: "A run that reaches its execution"}
  - {id: FR-040-AC-4, path: *p32, role: examined, excerpt: "Completion before observed cance"}
  - {id: FR-040-AC-5, path: *p32, role: examined, excerpt: "A multi-member group sharing one"}
  - {id: FR-040-AC-6, path: *p32, role: examined, excerpt: "The single-run producer construc"}
  - {id: FR-040-AC-7, path: *p32, role: examined, excerpt: "The direct `quire-exact::CancelC"}
  - {id: FR-041-AC-1, path: &p33 "spec/kani/functional/FR-041-claimed-module-proof-gate.md", role: examined, excerpt: "A nonempty reviewed declaration "}
  - {id: FR-041-AC-2, path: *p33, role: examined, excerpt: "With two claimed modules sharing"}
  - {id: FR-041-AC-3, path: *p33, role: examined, excerpt: "The current build's transitive e"}
  - {id: FR-041-AC-4, path: *p33, role: examined, excerpt: "Each claimed module and computed"}
  - {id: FR-041-AC-5, path: *p33, role: examined, excerpt: "A stubbed executor, predetermine"}
  - {id: FR-042-AC-1, path: &p34 "spec/oracle/functional/FR-042-generated-checked-invariant-causes.md", role: examined, excerpt: "CG's native Boolean, equality, f"}
  - {id: FR-043-AC-1, path: &p35 "spec/core/functional/FR-043-owned-lock-coherence-gate.md", role: examined, excerpt: "The normal `make deny` recipe in"}
  - {id: FR-043-AC-2, path: *p35, role: examined, excerpt: "Through the same normal `make de"}
  - {id: FR-043-AC-3, path: *p35, role: examined, excerpt: "A missing checker executable, ma"}
  - {id: FR-043-AC-4, path: *p35, role: examined, excerpt: "The committed `scripts/check_one"}
  - {id: FR-043-AC-5, path: *p35, role: examined, excerpt: "Adoption leaves the committed Ca"}
  - {id: NFR-001-AC-1, path: &p36 "spec/core/non-functional/NFR-001-reproducible-atomic.md", role: examined, excerpt: "Identical inputs and configurati"}
  - {id: NFR-001-AC-2, path: *p36, role: examined, excerpt: "An injected generation failure l"}
  - {id: NFR-001-AC-3, path: *p36, role: examined, excerpt: "Generation does not modify devel"}
  - {id: NFR-001-M-1, path: *p36, role: examined, excerpt: "Byte differences between repeate"}
  - {id: NFR-001-M-2, path: *p36, role: examined, excerpt: "Partial published bundles after "}
  - {id: NFR-001-M-3, path: *p36, role: examined, excerpt: "Developer-owned files modified"}
  - {id: NFR-002-AC-1, path: &p37 "spec/core/non-functional/NFR-002-provenance-boundary.md", role: examined, excerpt: "Every generated Rust file in a f"}
  - {id: NFR-002-AC-2, path: *p37, role: examined, excerpt: "Every generated Rust file carrie"}
  - {id: NFR-002-AC-3, path: *p37, role: examined, excerpt: "Unsupported and inconclusive sta"}
  - {id: NFR-002-AC-4, path: *p37, role: examined, excerpt: "Generated output makes no projec"}
  - {id: NFR-002-M-1, path: *p37, role: examined, excerpt: "Generated Rust files missing gen"}
  - {id: NFR-002-M-2, path: *p37, role: examined, excerpt: "Generated files missing dual-lic"}
  - {id: NFR-002-M-3, path: *p37, role: examined, excerpt: "Silent unsupported/inconclusive "}
  - {id: NFR-004-AC-1, path: &p38 "spec/strategy/non-functional/NFR-004-constructive-generation-efficiency.md", role: examined, excerpt: "Fresh 10,000-case and 256-case c"}
  - {id: NFR-004-AC-2, path: *p38, role: examined, excerpt: "For every admitted fixture in TC"}
  - {id: NFR-004-M-1, path: *p38, role: examined, excerpt: "Discarded / attempted on a fresh"}
  - {id: NFR-004-M-2, path: *p38, role: examined, excerpt: "Proptest global rejects on the s"}
  - {id: NFR-004-M-3, path: *p38, role: examined, excerpt: "Campaigns ending `Exhausted` on "}
  - {id: NFR-004-M-4, path: *p38, role: examined, excerpt: "In-domain plus out-of-domain cen"}
  - {id: NFR-005-AC-1, path: &p39 "spec/core/non-functional/NFR-005-no-generation-panics.md", role: examined, excerpt: "Every `.rs` file under `src/`, e"}
  - {id: NFR-005-AC-2, path: *p39, role: examined, excerpt: "`lower_scalar_claim` returns `Er"}
  - {id: NFR-005-AC-3, path: *p39, role: examined, excerpt: "`ItemSettlement::warning` return"}
  - {id: NFR-005-AC-4, path: *p39, role: examined, excerpt: "With a coverage export supplied,"}
  - {id: NFR-005-AC-5, path: *p39, role: examined, excerpt: "The step of `generate_kani` that"}
  - {id: NFR-005-AC-6, path: *p39, role: examined, excerpt: "`rewrite_duplicate_position`, a "}
  - {id: NFR-005-AC-7, path: *p39, role: examined, excerpt: "`observe_clause`, given a map of"}
  - {id: NFR-005-AC-8, path: *p39, role: examined, excerpt: "The scan locates the body of `ge"}
  - {id: NFR-005-M-1, path: *p39, role: examined, excerpt: "Panic tokens in the non-test, li"}
  - {id: NFR-005-M-2, path: *p39, role: examined, excerpt: "Measured sites a test can reach "}
  - {id: NFR-005-M-3, path: *p39, role: examined, excerpt: "Index, slice and unchecked subtr"}
  - {id: NFR-006-AC-1, path: &p40 "spec/kani/non-functional/NFR-006-real-kani-lane-gating.md", role: examined, excerpt: "`make kani-scope` prints `requir"}
  - {id: NFR-006-AC-2, path: *p40, role: examined, excerpt: "`make kani-scope` prints `not re"}
  - {id: NFR-006-AC-3, path: *p40, role: examined, excerpt: "A rename of `src/kani/old.rs` to"}
  - {id: NFR-006-AC-4, path: *p40, role: examined, excerpt: "`make -n kani` and `make -n kani"}
  - {id: NFR-006-AC-5, path: *p40, role: examined, excerpt: "`make kani-gate` starts the lane"}
  - {id: NFR-006-AC-6, path: *p40, role: examined, excerpt: "`make kani-gate` exits non-zero "}
  - {id: NFR-006-AC-7, path: *p40, role: examined, excerpt: "The passing run-result line has "}
  - {id: NFR-006-AC-8, path: *p40, role: examined, excerpt: "When the tree state or the head "}
  - {id: NFR-006-AC-9, path: *p40, role: examined, excerpt: "With a dirty working tree `make "}
  - {id: NFR-006-AC-10, path: *p40, role: examined, excerpt: "With no `cargo-kani` launcher `m"}
  - {id: NFR-006-AC-11, path: *p40, role: examined, excerpt: "The tests marked `#[ignore = \"ka"}
  - {id: NFR-006-AC-12, path: *p40, role: examined, excerpt: "`make -n ci` expands to no `kani"}
  - {id: NFR-006-AC-13, path: *p40, role: examined, excerpt: "For a Kani-touching pull request"}
  - {id: NFR-006-AC-14, path: *p40, role: examined, excerpt: "Given Kani-reaching paths change"}
  - {id: NFR-006-AC-15, path: *p40, role: examined, excerpt: "A Kani-touching pull request who"}
  - {id: NFR-006-AC-16, path: *p40, role: examined, excerpt: "A pull request whose scope is `n"}
  - {id: NFR-006-AC-17, path: *p40, role: examined, excerpt: "Before a version tag is pushed, "}
  - {id: NFR-006-AC-18, path: *p40, role: examined, excerpt: "Every `include!`, `include_str!`"}
  - {id: NFR-006-AC-19, path: *p40, role: examined, excerpt: "On a clean current candidate, `m"}
  - {id: NFR-006-M-1, path: *p40, role: examined, excerpt: "Merged pull requests touching th"}
  - {id: NFR-006-M-2, path: *p40, role: examined, excerpt: "Lane runs reported `passed` with"}
  - {id: NFR-006-M-3, path: *p40, role: examined, excerpt: "`#[ignore = \"kani lane: ...\"]` t"}
findings:
- fnd: FND-001
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-005-AC-3
  related_ids: []
  path: spec/core/functional/FR-005-cli-conformance.md
  lines: '49'
  excerpt: For every shared-corpus case on which two backends' normalized outcomes
    differ, the TC-007 Differential Disposition Register holds exactly one row naming
    the case, the backends, and either a regression fixture that exists in the test
    tree or a documented profile/backend distinction; a differing case with no row,
    a row naming a fixture that does not exist, or a row with neither disposition
    fails the check.
  finding: FR-005-AC-3 remains untagged on both main and candidate; the existing SR-15352/FND-001
    baseline inventory retains its owning remediation.
- fnd: FND-002
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-016-AC-25
  related_ids: []
  path: spec/replay/functional/FR-016-witness-native-replay.md
  lines: '156'
  excerpt: In an installed-Kani run that classifies a generated harness as `Falsified`,
    the retained counterexample is byte-identical to the first eligible assertion
    playback for that harness in the same launcher's complete captured stdout order,
    excluding cover blocks, and native replay uses values decoded from that retained
    counterexample; a `Verified` run supplies no counterexample. A test oracle observing
    stdout independently compares the complete fence interior, trimmed only of surrounding
    whitespace, with the retained counterexample before decoding or replay. Substituting
    another valid, self-consis
  finding: FR-016-AC-25 is untagged on both current main and candidate; this newer
    baseline obligation is outside SR-15352 and retains its existing IR-604/PR498
    replay-owner scope.
bindings: []
```

## Dispositions

Round 1, fix head 64fabc7e4c3545f9a16261d72521931656a42d9a. Original Findings and all original review text above are preserved.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Baseline-main CLI owning-spec/plan obligation outside the authorized IR-17 owner migration and host controls. Existing reviews/ir-954-gap-analysis.md SR-15352/FND-001 retains the criterion, owning path and explicit deferred outcome routed through existing IR-954/IR-17 planner records. No new issue, fix, proof or completion claimed; repository FAIL remains. |
| FND-002 | deferred | Baseline-main FR-016-AC-25 replay obligation belongs to existing IR-604/PR498 and spec/replay/functional/FR-016-witness-native-replay.md, outside this IR-17 migration. Retain that existing owner for its first-eligible-playback/native replay oracle; no duplicate ticket, fix, proof or completion claimed. Repository FAIL remains. |

## Round 1 scope and qualification

Round 1 source inspection at 64fabc7e only. All eight original source/custody defects are fixed by inspected changes; this is CONDITIONAL source acceptance, with final generated-native execution and broader qualification still pending. No new source defect was found in the fix diff. The user has authorized existing namespace tests, but no Cargo, namespace/backend execution or gate ran in this reviewer pass. The kernel owns the next heavy slot; this review acquired no slot and changed no host/security/account settings.

Supplied author evidence: fix-round-1/compile.log finishes all-targets no-run compilation, and scalar-generation.log has exactly one passing scalar-family host test, both before the final trace constraint change. scalar-native-agreement.log fails fresh dependency resolution at ix-trace-rs =0.1.0 (candidate 0.1.2), before generated test compilation or native assertions. Current root and agreement manifests consistently allow 0.1, which addresses that selection mismatch by source inspection, but there is no successful rerun yet. fmt.log is a formatting action, not an independent fmt-check result. None of these logs proves all current generated-native vectors, paired backend receipts, aggregate gates, or future integrated head.

Quantity source inspection checked owner UnitQuantity resolution, conversion target.id() handoff, complete paired IdentifiedUnit including stops, unflattened IllTyped/Identity errors, subsequent UnitScope::form consumption, original numeric/quantity vectors and denied-charge prefixes. Public QSV owns identities; no QSL947/unit-table facade dependency or copied private identity recipe was added. Equality Quantity Unsupported AC18/38 is untouched. Per-item standalone scalar oracle source keeps quantity-only error variants out of integer proof/observer bodies, preserving the scalar native ABI. The restored Boolean spec agrees with existing retain/stop semantics. Historical SR11690 custody is exact reviewer-owned text.

This is not the final mergeable freeze: AC45 unit metadata and the codex2 port are explicitly scheduled to join later, requiring integrated review and safe final qualification. No aggregate PASS, positive quantity-equality acceptance, native Union acceptance or backend proof is claimed.

Fresh read-only matrix: 898 criteria, 542 tagged, 288 untagged, 16 ignored-only and 52 method-without-symbol. The only static change from the reviewed candidate is FR014 AC52 untagged -> tagged. All evidence rows still say no run evidence; the new binding is not execution discharge. Both inherited findings remain untagged and are explicitly deferred to their existing owners. Repository FAIL remains; no new incremental gap was found. Plan completion: not assessed.

```yaml
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: Baseline-main CLI owning-spec/plan obligation outside the authorized IR-17
    owner migration and host controls. Existing reviews/ir-954-gap-analysis.md SR-15352/FND-001
    retains the criterion, owning path and explicit deferred outcome routed through
    existing IR-954/IR-17 planner records. No new issue, fix, proof or completion
    claimed; repository FAIL remains.
- fnd: FND-002
  outcome: deferred
  reason: Baseline-main FR-016-AC-25 replay obligation belongs to existing IR-604/PR498
    and spec/replay/functional/FR-016-witness-native-replay.md, outside this IR-17
    migration. Retain that existing owner for its first-eligible-playback/native replay
    oracle; no duplicate ticket, fix, proof or completion claimed. Repository FAIL
    remains.
```

## Round 2 source and evidence review

Round 2 source review at cd202f1b8fda0c23428c19aa47c40ce3769ff76c covers the entire origin/main...HEAD landing by retaining the original whole-diff review and inspecting all post-64fabc7e changes. Read-only git ls-remote confirms remote main and local origin/main both 0d7813df955576edc867cfa90e2071bd56d844ff. The working tree is clean; no refs were changed. The eight original source/custody findings remain fixed; their original and round-1 disposition rows remain unchanged. Two new test-intent findings and one independently confirmed product finding require another fix round. Source mergeability: FAIL until FND009/FND010/FND011 are resolved, then conditional on final qualification and the independent non-authored review. No product source edits or Cargo/native/namespace/backend/gate execution ran in this reviewer pass.

Production inspection: admit_environment attaches the owner's complete unit_table clone before Union seals, and the emitted recipe does the equivalent after context/package checks. None preserves construction. Direct/nested Quantity Unsupported remains unchanged. Genuine source helper checks owner node lookup versus owner-issued IDs, exact metadata, quantity resolution, unused km and None preservation. Those helper assertions are sound in isolation but do not exercise the production attachment boundary (FND009). The public integration currently checks typed direct/nested refusals, healthy Generated statuses, missing nominal context and wrong-package whole-request precedence, but omits the planned absent-symbol and isolated-source checks (FND010). No positive Quantity equality acceptance is claimed.

QSL lock is 48b47ab99175a80f799b786d6feda1b1ad37cffb; read-only merge-base confirms descendant of required 0b839dd. Named scalar unit fixtures compile authentic public QSL source for the two original owners, preserve unit dimensions/scales/offsets/labels, retain owner-issued node/ID metadata and complete tables, and independently compile enum members. The reserved keyword in becomes source inch while vector label and 127/5000 scale remain unchanged. Arbitrary low-level labels are confined to explicit malformed-topology controls, not nominal fixtures. No private schema/hash recipe or compatible value implementation is introduced. Paired scalar results retain complete IdentifiedUnit across stops and chained operations, typed IllTyped/Identity and charge/counter/denial oracles. HRTB quantity comparison now supports independent operand lifetimes. The existing record/tuple fixture explicitly excludes Union. No native Union acceptance follows from host source tests.

Evidence inspected from fix-round-2: focused-final-summary.json plus each referenced raw log and command JSON confirms the exact named outer native scalar test passed one test, whose runner requires every #[test] in the current source (20 cases), and the exact record/tuple native test passed one test (direct/transitive inside). Each six original host controls matched exactly one test; source iterates all original fifteen fixtures plus mixed. Union closure and public integration each passed two. Metadata helper one and integration two passed, with the coverage limits above. Quantity refusal, source panic scan, scalar-family generation and Boolean retention each passed one. fmt-check, clippy all-targets -D warnings, and spec validation exit zero; spec warnings remain visible. Earlier failed attempts remain in separate logs and are not counted as final PASS. These are author-run focused observations, not reviewer execution and not aggregate proof.

resolved-all-cohorts.json and the three actual root/generated locks retain one-copy outcomes. QSL/QSV/Exact owner revisions align; root Runtime a60c508 and independently fresh generated Runtime 3d2d820 differ because fresh generated crates resolve their public branch constraints. The compatibility trace range permits root 0.1.0 and fresh 0.1.2; neither equality of those Runtime revisions nor a universal sealed cohort is claimed. No lock was seeded into generated scratch crates. Protected source-preservation assertions and unchanged original case declarations were inspected. Main's Quantity -> NonRangeType and equality AC18/38 refusal are unchanged.

FND011 provenance: externally reported by codex2 through the user and parent, independently confirmed here from mark_arithmetic, outcome_bool, plain_node and outcome_short_connective. In a globally metered root, a false holds_arithmetic mark routes an entire Boolean-only subtree through unmetered plain_node, so nested Not/connectives miss the required completed result retain. The outer short-circuit connective then retains once and skips the arithmetic right branch, yielding the wrong stop boundary for result_units=1. Source proof only; no regression execution performed by this reviewer. Preserve original FR031 spec-fix disposition FND004; this is a distinct implementation defect under corrected AC26. The same coder owns the native regression and production fix in this PR.

The AC45 specification lineage is independently reviewed in the nine retained SR15380..15388 artifacts; this pass checks consuming code and test intent, without rewriting their original verdicts. The initial Union amendment reviews SR15372..15377 and PR510 remain intact. Full aggregate make ci, Kani/CBMC/backend acceptance and final retained receipt remain pending. Source inspection/emission fragments never count as execution of the emitted metadata constructor. No new issue or host/security/account change is proposed.

Fresh computed quoin matrix has 899 criteria: 543 tagged, 288 untagged, 16 tagged-by-ignored-test and 52 method-without-symbol. AC45 is the sole post-round1 new criterion, tagged to the helper and two integration tests; static tagging is not evidence discharge. All matrix evidence_status entries remain no run evidence. New FND009/FND010 in SR15370 record the AC45 assertion gaps, rather than duplicating them as new unowned remediation here. The complete source/requirement/test chain and coverage diagnostics were compared to the retained base inventory and the preceding computed comparison. Quire coverage still reports ten diagnostics and the same seven resemblance suspicions as the initial candidate; these are not newly invented or execution claims. Raw coverage traverses additional reviewed artifacts and must not be confused with the 899 acceptance-criterion matrix denominator.

Both repository-level findings remain untagged: FR005AC3 is deferred to the existing SR15352/FND001 baseline CLI inventory, already routed via IR954/IR17 planner records; FR016AC25 is deferred to existing IR604/PR498 replay ownership. The explicit round1 reasons and rows are preserved. The original inventory of 320 findings and its explicit outcomes remain authoritative for inherited gaps. Of current 288 untagged criteria, 287 occur in that inventory and the remaining FR016AC25 has its existing owner. No new issue is allowed or needed. Repository completeness: FAIL; incremental static traceability: PASS; introduced test-intent coverage remains FAIL in SR15370 pending fixes. Dedicated ignored Kani controls are not called defects merely for their lane. Plan completion: not assessed. Optional semantic gap review was not opted in; mandatory code-review oracle checks ran and produced the two new findings. No source execution or aggregate acceptance is asserted.

```yaml
dispositions: []
review_head: cd202f1b8fda0c23428c19aa47c40ce3769ff76c
scope:
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table.
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: Deleting either attachment must fail its corresponding check.
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point.
prior_outcomes_confirmed:
  FND-001: deferred
  FND-002: deferred
```

## Round 3 source and evidence review

Round 3 frozen source head 82051284537d9468f03b8a158a3e4f3605309a3f. Independent code/Rust disposition covers every cd202..820512 product/test/documentation/custody delta and retains the complete original whole-landing review scope above. Read-only git ls-remote confirms remote main equals unchanged local origin/main 0d7813df955576edc867cfa90e2071bd56d844ff. Tree is clean; no refs changed. No product edits, Cargo/tests/native execution, namespace/backend work or gates ran in this reviewer pass.

The eight original findings remain fixed with their earlier immutable dispositions. FND009/FND010/FND011 are now fixed by the inspected source and discriminating author-run evidence. FND012 records the independently confirmed original low ownership-prose defect from external SR15389/FND002 at cd202 and its actual correction here; original external findings remain untouched. No additional current defect of any severity was found. All twelve code findings have explicit fixed outcomes; both inherited gap findings retain existing-owner deferred outcomes. Source readiness: PASS. Mergeability: CONDITIONAL on final make ci and canonical Kani/backend qualification, retained evidence/custody, and the external review's own final dispositions. Aggregate/Kani remain UNRUN, so no full product or proof PASS is asserted.

FND009: the new test traverses actual admit_environment instead of directly calling only the attachment helper. It verifies complete original units including unused km, authentic public node lookup, owner-issued IDs, exact quantity metadata/resolution and production context-free None behavior. The retained helper check additionally preserves a populated table under None. The mutation script removes exactly the production attachment, compiles, fails the complete-table assertion with exit101, and restores source in finally; the restored exact one-test command passes. The red was assertion failure, not setup/compilation failure.

FND010: mixed direct and nested Quantity requests retain all original source declarations and typed Unsupported/context precedence. Both call the new isolation helper. All code artifacts excluding claim-map.json equal their isolated surviving siblings byte-for-byte, so private helper/code leakage is also detected. Generated claim comparisons retain all fields except contract_package_identity: public lowering includes selected root sets in that identity, while source-owner identity, node/operation, descriptors, source map, dependencies, schedule and symbols remain equal. This is a justified request identity normalization, not relaxed nominal identity. Syn parses genuine emitted Rust; the exact public entrypoint census matches only surviving environment/oracle names. Both integration tests pass. Quantity remains Unsupported; metadata does not add positive Quantity equality/conversion or native Union acceptance.

FND011: in a globally metered native root, the pure-child connective branch now takes a typed preorder index and recursively renders its child connectives, retaining each completed result. Pure leaves use plain_node with its own index consumption. Compare analysis accepts bounded integers only, so a pure comparison cannot conceal unvisited Boolean connective children. Globally arithmetic-free rendering and ABI stay on the existing path. New genuine generated-native checks exercise Not, short And/Or, total And/Or and implication plus a comparison-leaf control. Every budget zero through the independent expected retain count checks Completed(false) only at full budget and Incomplete(BooleanResultRetain) otherwise; counters and admitted charges equal the budget, and every charge is BooleanResultRetain, proving right arithmetic remains skipped. The old-code command compiled and produced six failing inner tests at budget1 plus one passing leaf control. The fixed exact outer test passes, and its runner invokes the entire freshly generated native crate with no test filter. Source generates seven distinct #[test] items. Existing native retention and first-stop controls pass, as do arithmetic-free byte identity and comparison shape controls. These are current focused observations; final backend proof is still pending.

FND012: the module and generated comment now name the actual owning crates. No operator body changed in scalar/mod.rs. Existing scalar native twenty-case, record/tuple direct/transitive, six original host controls over fifteen authentic fixtures plus mixed, four Union host checks and scalar-family/Quantity refusal evidence in fix-round-2 remain valid focused observations for unchanged behavior; this round does not relabel them as final-head aggregate/Kani proof. Round3 focused-summary.json and individual command JSON/raw logs confirm exact matched tests and successful statuses: metadata1, Quantity integration2, retention1, first-stop1, unmetered bytes1, unmetered comparison1; pure-connective exact outer1 recorded separately. make fmt-check, locked Clippy all-targets -D warnings and make spec exit zero; existing spec warnings remain recorded. The final-head.json identifies this exact source checkpoint. No broad execution was repeated by this reviewer.

Review custody: both committed official round2 artifacts are byte-identical to their validated reviewer-owned disposition2 files. All four committed external nonauthor reviews SR15389..15392 are byte-identical to their supplied originals. Earlier source/authentication/protected fixture evidence and the nine independent AC45 spec reviews SR15380..15388 remain intact. Branch changes since cd202 are confined to the five product/test paths plus six custody documents listed by git diff. Original domains, fifteen fixture sources, twenty protected qualification directories, native execution ownership and Quantity->NonRangeType are untouched. QSL 48b47ab9 cohort and current public UnitTable/QSV ownership remain unchanged. Final gates belong to the parent, with no extra permission required.

Fresh read-only computed matrix remains 899 criteria: 543 tagged, 288 untagged, 16 ignored-only, 52 method-without-symbol. AC45 is still tagged to three test functions; AC26 now binds both the existing retention test and the added pure-connective-subtree regression. Computed evidence_status remains no run evidence because external author logs are not bound evidence records; this does not negate the independently inspected focused logs. This pass claims those separately. Full end-to-end ownership, tagged tests, actual assertions and production chain were inspected across the reviewed landing and final delta. No new static gap or unowned production addition was found.

FR005AC3 remains untagged and deferred to the existing SR15352/FND001 CLI baseline inventory and IR954/IR17 planner disposition. FR016AC25 remains untagged and deferred to existing IR604/PR498 replay ownership. Their original findings and explicit round1 reasons stay immutable. The 320-row inherited inventory and existing dispositions are retained, with no duplicate issue or invented remediation. Repository completeness remains FAIL; incremental static gap verdict PASS; the introduced AC45 and Boolean assertion gaps identified by code review are now FIXED rather than excused. External SR15390/FND001 identifies the same Boolean gap as our FND011; new native pure-subtree regression closes its missing case by actual source and red/green evidence. External reviewer owns that original artifact's disposition. Ignored dedicated Kani controls are not defects merely for their lane. Plan completion: not assessed. Optional semantic gap review not opted in; required code-review oracle checks and focused evidence review performed. Full aggregate and canonical Kani acceptance remain pending.

```yaml
dispositions: []
review_head: 82051284537d9468f03b8a158a3e4f3605309a3f
scope:
- id: FR-031-AC-26
  path: spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md
  role: examined
  excerpt: Each completed connective, including unary BooleanNot, calls `quire_exact::retain_boolean`
    exactly once and charges `boolean.result-retain` once, including a result decided
    by the left; a child stop charges no connective result-retain point.
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: Deleting either attachment must fail its corresponding check.
- id: SOURCE_HEADER
  path: src/oracle/scalar/mod.rs
  role: examined
  excerpt: '// Each function calls its `quire-exact` operator or a runtime-owned residue

    // operator with the caller''s `Meter`; no charge amount appears in this source.'
prior_outcomes_confirmed:
  FND-001: deferred
  FND-002: deferred
```

## Round 4 partial source disposition

PARTIAL round4 source review only, frozen head 730d9aba7a502e9f51f55a1c5c473818e9a83453. This is not the completed round4 or a final mergeable verdict. All original twelve code findings remain fixed, and both inherited gap findings retain their existing-owner deferred outcomes. No old finding is reopened or duplicate unowned finding invented for the already owned gate failures. The actual aggregate at 0c4a36b67bb932ade5cfc711146deb2ba23f1bcd failed: 398 library tests passed, four failed, five ignored, make exit2 before integration. That failed gate is retained at final-gates/ci-20261022T024344Z.log and is not replaced by these focused passes. No aggregate retry, Kani/CBMC or reviewer execution occurred here. Current PR acceptance remains pending.

Only src/oracle/equality/mod.rs changed between failed head and this checkpoint. Independent byte comparison confirms everything before its single cfg(test) tests module is identical (SHA256 24b9ce063dd55aab90b5893a91bfaed389d2cef5c4a3fcd6e3e1c717553762c7). No production UnitTable, public API, manifest, lock, source/domain, work limit, stack size or test assertion was changed. No product edits, builds, namespaces, backend operations or git ref mutations were performed by the reviewer.

Cycle fixture fix: the child diagnostic confirms an ordinary failed assertion, MissingAdmittedContext instead of expected TypeResolutionCycle, rather than evidence of a crash. The resolver helper already received the genuine recursive owner, but the item helper passed None. check_nominal_component_item now uses check_nominal_items_with_limits and the same genuine owner. Both Option and Sequence deliberate private graph mutations retain their exact repeated-node assertions, original sibling resolver checks and healthy Integer item assertions. The subprocess wrapper prints exit status, stdout and stderr. Removed unused context-free test helpers are replaced by this explicit nominal component helper, without suppressing warnings. This remains a private inconsistent-input defense, not a claim that malformed identities were publicly admitted. cycles-final.json/log records the exact outer test, one matched pass; source runs both children and requires each to succeed. No stack/shape/domain relaxation was introduced.

Render fixture fix: emit_two now establishes SourceBuilder.expected_package using the genuine admitted source package identity, matching production's required preparation state. It compiles original public QSL source declaring Length/m and a Quantity<m> signature, retains the owner's actual UnitId and complete table, and prepares checked equality through QSV TypeEnvironment. The Quantity variant passes its actual CheckedItem to render_item; the UnsupportedValueType refusal is no longer merely injected. The owner-admitted plan succeeding does not enable public CG Quantity equality: the actual consumer still returns its unchanged Unsupported arm, and production source is byte-identical. No unit identity, compatibility schema or private API is copied. The whole-call generation error remains an explicitly deliberate boundary injection, not claimed public intake. Every original two-item symbol naming, no-code, per-item refusal, healthy sibling and whole-call abort assertion stays intact. render-isolation-owner-final.json/log records the exact one matched test passing. Synthetic transport labels retained in this explicitly private render seam do not acquire compiler provenance from the genuine metadata; no public request acceptance is claimed.

This checkpoint adds no positive Quantity equality/conversion, native Union, or emitted constructor execution claim. The in-process QSV plan and actual renderer are the intended private seam under AC18. UnitTable is used exclusively to supply genuine owner metadata to that check; there is no UnitId derivation, table reconstruction, cross-owner substitution or production refusal change. The unrelated prior metadata and scalar functionality remains as reviewed in rounds1-3.

Recorded author evidence in fix-round-4/partial-summary.json, exact command JSONs and raw logs confirms cycles1 and actual-render-isolation1 pass, make fmt-check zero and locked Clippy all-targets -D warnings zero. Parent-supplied Quire validation is exit0 with existing warnings; this reviewer independently validates the scratch artifacts. git diff --check also passes. No supplied focused result is treated as passing aggregate or as the two remaining large-source controls.

PENDING unchanged controls: the 50,000-record source is 2,027,895 bytes and retains its 512KiB CG stack. The work-bound source is 2,599,335 bytes and retains its original entry, default limit, denied-count and healthy-sibling assertions. Both still fail at source intake under the current QSL public facade's default S1InputBytes1MiB; no successful rerun or acceptance is claimed. Existing QSL960 owns the public StageLimits configuration constructor/re-export. Wait for the real public API, adopt a final merged cohort, configure only finite fixture intake budgets with genuine unchanged sources, and then inspect the two exact passing regressions before a completed review/gate retry. No direct foundation dependency, fabricated wire, guessed API, inference/string escape hatch, or reduced fixture/profile/domain/stack/work target is approved by this partial review. No new issue was filed.

Verdict for the two reviewed independent fixes: PARTIAL source PASS, no additional defect found. Overall source/qualification completion: PENDING on the two upstream-dependent controls and final integrated review. Mergeability is not granted by this partial checkpoint. The prior round3 conditional statement is historical and does not override the known failed aggregate or these explicit pending obligations.

Fresh computed matrix remains 899 criteria, 543 tagged, 288 untagged, 16 ignored-only, 52 method-without-symbol. Trace attributes and tested obligations are unchanged; helper selection now reaches the intended private nominal/renderer seam. Repository gap verdict remains FAIL with existing SR15352/FND001 and IR604/PR498 owners; no new incremental static gap was found. This partial review does not waive failed or pending gates, invent duplicate remediation, or call ignored dedicated Kani controls defects. Plan completion: not assessed. Optional semantic gap review not opted in. All prior baseline inventory outcomes remain intact.

```yaml
dispositions: []
phase: partial
review_head: 730d9aba7a502e9f51f55a1c5c473818e9a83453
scope:
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: context_only
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
prior_outcomes_confirmed:
  FND-001: deferred
  FND-002: deferred
qualification: failed aggregate retained; two focused fixes PASS; two large-source
  controls PENDING existing QSL960; no mergeability granted
```

## Round 5 complete source disposition

Complete Round5 source disposition at published clean head 3de23f5793b2cf4e301de1713cbc70a106bfa434, following the partial round4 record above. Code review including Rust and the full repository gap method completed. No new finding at any severity. All twelve original code findings remain FIXED. No source edits, Cargo/test/native execution, namespace/backend operations, aggregate/Kani gate, commit/push or ref mutation was performed by this reviewer. All earlier findings, still-open rows, fixes and partial verdicts above remain immutable history.

Source verdict: PASS. Mergeability: CONDITIONAL on the parent-owned final make ci and canonical Kani qualification and retaining final official/external dispositions. The actual earlier aggregate failure at 0c4a36b6 remains retained (398 library PASS,4 FAIL,5 ignored; make exit2 before integration); this disposition verifies the source fixes and all four exact regressions on the merged owner cohort, not a passing aggregate rerun. No Kani/CBMC proof or receipt is claimed. The prior partial upstream-API condition is now satisfied; do not continue to call these four regressions unrun. Broader final gate acceptance is still pending.

Baseline reconciliation: fresh remote main and local origin/main both 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd; it is an ancestor of the candidate. The clean merge retains IR604/PR512's stdout-capture tests and Kani lane registration. Makefile/scripts/workflows have zero incremental delta against this current baseline; adding the inherited stdout test to its dedicated Kani lane is not a weakened or new IR17 gate. Current main already supplies FR016AC25 trace bindings, discussed below. Baseline code was not silently fixed or imported into the PR's defect count. Fresh-main ownership/context was inspected, and all migration source/spec/history plus current own authored UnitTable paths remain in the reviewed whole-landing scope.

Merged owner cohort: QSL's eight crates select actual merged 485ccaa3bd289ae94baa74e96a976adce99fd0d8 (PR1197), exposing StageLimits::with_named and ReplayLimits::with_input_bytes through the qsl_replay root. The public implementation validates catalog names, excludes replay/format names from StageLimits, returns typed name refusal and changes no defaults; FR399AC4 explicitly covers real declaration sources requiring all three syntax allowances. No direct foundation dependency, copied wire/schema/private setting enum, guessed public API or string-to-private-type trick is used. Root manifest remains branch main with qsl-replay the sole direct QSL dependency.

The resolved lock advances kernel to b692cc83d84eac33477a7d3d2770bc11f0e0350a and aligns both quire-contract-ir/model to d0737774acfbe64b1019b265952dc4c9720c2f23. QSV remains 1a38c06f39ece40e044a7bef5b868bd478b41c24 and Runtime a60c508f3a2af230aeb51af97bdf549cd5fffa01. Associated Quire/FCD dependency updates are ordinary transitive resolution of that merged owner cohort; no package override or PR pin is committed. Current root lock contains one of each first-party package; supplied one-copy check passes. Independent JSON comparison verifies metadata-before and metadata-after contain identical complete packages arrays AND identical resolve graphs (268 packages), not only selected names. Thus final ir-facade lock alignment changed no actual selected compiled source or feature graph, and the exact expensive checks validly apply across that metadata-only correction.

Large fixture fixes: all changes since the partial checkpoint are private equality test support or dependency/baseline/custody. The production equality prefix before cfg(test) is byte-identical; original record-source generation bytes, 50,000-record/100,001-entry deep target, 65,536 work accounting, both 512KiB stack allocations, integer/domain and source-publication assertions are preserved. Source intake now uses finite actual source length for S1 input and replay bytes, source_bytes+1 for tokens (fixed ASCII grammar plus EOF), and 8*records+32 syntax nodes with checked arithmetic. These allowances configure only authentic fixture compilation; they do not widen CG work/source/depth/stack/proof profiles or production defaults. The independently compiled retained deep source uses the same public finite allowances outside its unchanged 512KiB CG/runtime thread. No synthetic replacement source, ID or result is introduced.

The work fixture already selected an unlimited output allowance to isolate work accounting; its final inspection now calls finish(usize::MAX) instead of reapplying an unrelated default. Default/exact/one-byte-less source-limit checks remain in the separate deep control. The deep provenance assertion now locates the module actually reconstructing Chain0 via its genuine root node lookup, preserving authoritative key sorting instead of assuming that Chain0 sorts into the first module. Root module provenance and every module's item subject are still asserted.

Seeded lock boundary was independently assessed against FR018AC28/30 and FR017AC53/TC027: copying the executing lock is confined to this private flat-compilation/reconstruction fixture, which must reconstruct the same owner-issued package identities. It does not claim independent fresh dependency resolution or the canonical production/observer paired receipt. The latter still requires two independently freshly resolved locks without copying a parent/cached resolution, and its production/observer implementation, literal-pin assertions and regression are unchanged. The local premerge captured fixture lock and root common first-party identities matched; this final merged deep run uses the same seeding recipe and passes real native compilation/reconstruction. No separate fresh-resolution claim is inferred for this seeded fixture. Parent final gates still own current production/observer resolution and receipt acceptance.

The cycle and actual render-isolation fixes reviewed in partial4 remain unchanged. Cycle paths use the genuine recursive owner before the private repeated-node fault, retaining exact Option/Sequence cycle and healthy-sibling assertions; diagnostics retain both streams/status. Render isolation obtains genuine source package/unit metadata, owner-checks Integer/Quantity plans and actually calls render_item to obtain UnsupportedValueType. Whole-call generation failure remains a labelled private injection. This is defensive seam coverage under AC18, not public positive Quantity equality. No production behavior or nominal authorization is inferred from a privately constructed CheckedItem.

Independently reviewed UnitTable scope includes units.rs, actual admit_environment wiring, emitted environment preparation and admitted_quantity_units.rs. All are unchanged from the reviewed fixed round3 source under the now merged owner cohort. Complete original unit_table cloning occurs after guards and before Union sealing; None preserves construction; complete unused metadata/owner identities are asserted through the production preparation path. Mixed direct and nested Quantity controls compare isolated sibling artifacts/dispositions and assert only surviving entrypoint symbols. Only legitimately root-dependent Contract IR request identity is normalized; original owner/descriptor/schedule/source fields are retained. The earlier compiled production-call removal fails the intended table assertion and restores to PASS. Quantity Unsupported AC18/38, frame Quantity->NonRangeType, typed lookup/declaration errors, no-equality-meter-charge failure ordering and owner-key-vs-node Union sealing remain unchanged. No positive Quantity equality/conversion, native Union or emitted metadata-constructor execution is claimed. The complete authored UnitTable slice was reviewed independently here, rather than excluded with the external review's authored slice.

Actual merged-cohort evidence from fix-round-5-merged: work-bound-merged.log, deep-small-stack-merged.log, cycle-merged.log and render-merged.log each name the exact requested test, match one test and finish with one PASS. The deep test performs full generated native compilation plus independent source admission and runtime 512KiB work; the work control retains both default denied-entry and healthy boundary assertions. Earlier token/node allowance failures remain preserved separately. Final JSON durations include surrounding Cargo time; raw work test is 58.27s and deep test202.02s, consistent with observed commands. These are author executions, not reviewer executions. Current merged make fmt-check, locked all-target Clippy -D warnings, make spec and diff/one-copy checks pass; existing catalog/EARS warnings are retained. Older scalar20/native/host/metadata evidence remains provenance-limited to its earlier cohorts. It is neither discarded nor relabelled as execution of all current kernel/cohort paths; the parent aggregate provides that final check.

Historical custody: committed official code/gap files exactly equal validated disposition4-partial copies. Original historical SR11690 FND004 source-literal pin disposition remains intact; all external SR15389..15392 and nine AC45 spec review artifacts remain preserved. No CG spec/plan text changed in this final fixture/dependency round, so previously completed spec lenses/lineage remain applicable without inventing new clean review artifacts. Optional semantic gap analysis remains not opted in; mandatory code/Rust test-oracle review was performed. All targets and evidence are retained; no new issue, host/security/account change or additional permission flow is introduced.

External nonauthor final source review at the same 3de23f57 head now reports PASS with no new finding; its latest four exact reviewer-owned files remain in qualification-logs/review-cg513/reviews for final custody. Private IR17 code comment 2cd8663a-a83e-48cd-b80a-c8012cee52d5 and gap comment 0935e3fc-5b0e-4d76-9d40-746ecb9f1abe record that review. Official UnitTable review above independently covers the authored slice excluded there. Parent retains those validated latest artifacts before final gates.


Fresh complete matrix on candidate: 899 criteria, 544 tagged, 287 untagged, 16 tagged-by-ignored-test and 52 method-without-symbol. Fresh baseline computed with quire coverage over an exact git archive of main43d5a6fd: 897 criteria,525 tagged,302 untagged,18 ignored-only,52 method-without-symbol. quoin matrix on an archive correctly refused absent HEAD; no fake HEAD/ref was manufactured. The equivalent static coverage matrix was used for the baseline, and quoin matrix on the real frozen candidate supplies its current evidence statuses. Nineteen status/addition changes are saved in baseline-comparison.json; no new criterion becomes untagged. Candidate-only AC44/45 are tagged; source-chain and assertion intent were independently inspected rather than inferred from binding counts.

All 287 untagged current criteria occur in the existing SR15352 320-finding inventory, whose original outcomes remain retained. FR005AC3 is still untagged and remains explicitly deferred to SR15352/FND001 and the existing IR954/IR17 planner CLI owner; no new ticket or migration scope expansion. Original gap FND002 concerned untagged FR016AC25. Current merged main43d5a6fd now supplies four real trace binders through IR604/PR512: the ordinary launcher observation/selector tests and the dedicated installed-Kani controls. Thus that static trace gap is now removed by its existing owner in the BASELINE, not a new IR17 fix. Its prior deferred row remains immutable historical scope/disposition; current status is explicitly tagged/remediated-by-existing-owner. This does not claim that ignored installed-Kani assertions executed during this reviewer pass or this fixture qualification. Their dedicated lane is registered in inherited Makefile and remains for final parent qualification.

The full repository requirement/test/production chain was reconciled using current computed matrix and coverage plus the inherited inventory. Both baseline and candidate coverage show ten existing diagnostics and seven resemblance suspicions; no new stub, hollow implementation, unowned production path or inflated acceptance from private fixtures was found in this final delta. Repository completeness remains FAIL because inherited gaps persist; incremental gap verdict PASS. All original code defects remain fixed. Historical deferred ownership is preserved without falsely claiming all baseline requirements complete, inventing duplicate remediation, or treating dedicated ignored Kani lanes as defects. Plan completion: not assessed. Aggregate at0c4 remains failed history; all four specific regressions now PASS on merged owner code; final aggregate/Kani acceptance remains pending.

```yaml
dispositions: []
phase: complete-source-review
review_head: 3de23f5793b2cf4e301de1713cbc70a106bfa434
baseline_head: 43d5a6fdc3743835f90b1d6b720cb9195a7db1dd
scope:
- id: FR-018-AC-18
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '`render_value_type` called with `ValueType::Quantity` and with `ValueType::Reference`
    returns `Err(RenderError::UnsupportedValueType { family })` with `family` equal
    to `"quantity"` and `"reference"` respectively and does not panic, and the item-boundary
    mapping turns `RenderError::UnsupportedValueType` into `CompositeEqualityRefusal::Unsupported`
    with `node_tag` equal to `family` and `unsupported_node_id` equal to the item''s
    expression node id, emitting no code for the item and leaving its siblings unchanged,
    while `RenderError::Generation(OracleGenerationError::UnknownRuntimeVariant {
    .. })`'
- id: FR-018-AC-24
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The exact QSpec `positive-recursive-records.json` retains strict-reader
    and List/Tree shape assertions; independently authored genuine admitted source
    with those List (`Option<List>`) and Tree (`Sequence<Tree>` bounded `[0,3]`) shapes
    closes each back-edge at its own in-progress record key, generates equality items
    over both record types, and terminates; a synthetic unclosed `Option` self-cycle
    and a `Sequence` self-cycle each refuse as `TypeResolutionCycle` naming the repeated
    type node, without a panic, stack overflow or generated symbol. PLANNED (IR-643).
- id: FR-018-AC-25
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'The per-item type-resolution counter charges one unit for each root type-node
    entry or followed type-node reference or bounded-domain `semantic_type` edge,
    including a repeated or memoized reference, across both operands and conversion
    targets, independently of Contract IR lowering; with the default limit, an acyclic
    chain reaches its 65,537th entry and refuses only its item as `TypeResolutionWorkExhausted
    { limit: 65_536, consumed: 65_537 }` before further descent and without call-stack
    exhaustion or generated symbols, while a sibling needing at most 65,536 entries
    still generates. PLANNED (I'
- id: FR-018-AC-28
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: With a caller-selected type-resolution work limit of at least 100,000 and
    `cg.equality.source_bytes` raised to the measured flat source size, an admitted
    acyclic type chain of 100,000 entries reconstructs and generates an oracle with
    the same result as an equivalent shallow type on a 512 KiB stack; with the same
    raised source-byte limit, the default type-resolution work limit refuses it at
    entry 65,537, and a selected work limit one below its entry count refuses it at
    the first denied entry. No depth setting or fixed depth refusal participates in
    these runs. PLANNED (IR-511).
- id: FR-018-AC-30
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: The 100,000-entry acyclic closure renders as flat source without a type-depth-sized
    constructor expression; under the 1,048,576-byte default it returns `SourceTooLarge`
    naming that limit when the source exceeds it, and under a caller limit equal to
    its measured source bytes it emits a complete, compilable oracle within the publication
    artifact and bundle byte limits. One byte less refuses before that byte is appended
    and publishes no partial artifact. PLANNED (IR-511).
- id: FR-018-AC-44
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'Genuine Union equality generation preserves the facade declaration key,
    settled node, authoritative member bindings and declared member/payload order
    across a nested Union closure, using the specified lookup and verified sealing
    in both generator and emitted preparation. In-process owner-API controls admit
    values of every member, including record and nested-Union payloads; distinct same-shaped
    unions retain incompatible identities. Missing/wrong capability, Unknown/Foreign/WrongKind
    lookup, name/form disagreement and seal faults retain their typed causes, no successful
    prepared wrapper and no '
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table. Valid context-free
    None behavior remains unchanged. Wrong-package refusal retains its whole-request
    precedence; direct and nested Quantity items retain their existing typed Unsupported
    behavior and refusal p
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/update workflow, without copying a parent/other lock
    or substituting a cached resolution. Each resolves both owning runtime and exact
    identities to the build's authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently
    of the renderer's list. Both manifests, locks and all other AC-46 entries are
    sealed and compared as a complete path/byte set before proof, after proof and
    after native execution; mutation/replacemen
- id: FR-399-AC-4
  path: /home/peter/.cargo/git/checkouts/quire-spec-language-c986aa40394a5ad7/485ccaa/spec/functional/FR-399-set-a-named-stage-limit-through-the-replay-root.md
  role: context_only
  excerpt: A source of real declarations larger than 1 MiB, with `s1.input_bytes`
    and the reader limit raised, is refused naming the `s1.tokens` ceiling; with `s1.tokens`
    also raised, naming the `s1.nodes` ceiling; and it compiles only with `s1.input_bytes`,
    `s1.tokens`, `s1.nodes` and the reader limit all raised.
prior_outcomes_confirmed:
  FND-001: deferred to existing SR15352 CLI owner
  FND-002: historical deferral to IR604 retained; current baseline trace gap remediated
new_findings: []
qualification: source PASS; four actual merged-cohort regression PASS; parent aggregate
  and canonical Kani pending
current_baseline_owner_status:
  FND-002: FR-016-AC-25 tagged on both baseline and candidate through merged IR604/PR512;
    no native execution claim
```

## Round 6 combined source disposition

Complete official Round6 source review of the ONE combined PR513 candidate 9b2c69f22f8840930fde7a7a29abe1ff1859eab5 against current main724d6bfba77154dde44623655e57d3d7c92d8301. Fresh read-only remote main matches this baseline and it is retained by ancestry; tree is clean. Whole-PR source acceptance retains the complete prior review and inspects all later integration repairs, accepted IR850 source and selective Runtime adoption. Code/Rust PASS; no new current finding of any severity. All twelve earlier source findings remain FIXED. Historical failures, still-open rows, partial scopes and all explicit fixed/deferred outcomes are preserved above; no blanket baseline fixes or new issue is invented.

Mergeability is CONDITIONAL on the parent final aggregate make ci, canonical Kani gate and the separately requested nonauthor lunar lock-hunk review. This official review does not substitute for that independent lock review. Runtimea60 is reviewed only together with its consumer migration in this combined PR; no standalone main Runtime bump is accepted. No reviewer Cargo/compiler/test/native/LLVM/backend/Kani/namespace execution, product/source/lock edits, branch integration, push or ref mutation occurred. Parent owns final custody and one final-head gate sequence. No permission request is introduced.

The earlier def1f3f0 full batch remains failed historical evidence: library402 PASS, integration697 PASS/6 FAIL/39 ignored, before these repairs. It is retained at final-gates/ci-20261022T052217Z.log. The six failing controls shared five root fixture/expectation causes; the intermediate export E0425 and wrong-manifest preflight defect are also retained in their exact evidence/intake records. They are now corrected by inspected source and the actual combined-candidate focused logs, rather than waived or reported as defects still present. Current aggregate/Kani have not yet run, so a full product/qualification PASS is not claimed.

Combined dependency review: versus the accepted source9b01, Cargo.lock changes ONLY the Runtime package entry: its branch-main source advances 3cc88c480d4efe5179fe91acf4634def5d2d82fd to a60c508f3a2af230aeb51af97bdf549cd5fffa01, plus that package's real quire-semantic-value edge. All other lock bytes remain unchanged. Independently parsed package comparison confirms Runtime is the sole changed named package; root lock measures the exact authorized43dd9ef84f43ea430aa0e3408e5a77063a732adda9a3d294b903642b7a0f9582. Against published main724 the additionally visible codegen->QSV root lock edge is already required by this migration's direct Cargo.toml dependency, not an owner revision adjustment or workaround. No broader resolver change, version pin, source patch, foundation dependency, RT exception or copied type bridge is introduced.

The supported ROOT cohort has all eight QSL crates485ccaa3, shared kernel72b25421, QSV1a38c06f, Runtimea60c508f, quire-rsfd1cdd9d, and all three FCD packagesac62202a. Independent parsing verifies all FCD root sources match and there is one authoritative kernel/QSV identity. Actual locked and offline-locked metadata exit0; their full packages arrays and resolve graphs compare identical, independently rechecked. The supplied authoritative IR736 quire check-lock Cargo.lock exits0 with Git lock coherence checked; one-copy and allowlisted selective-lock checks pass. This is a root-cohort fact, not an assertion about any separately resolved generated workspace. The published owner route is unchanged except for the user's explicitly authorized combined Runtime selection.

Known IR962 observation is retained honestly and NOT substituted for a root-cohort verdict. The independent shared agreement runner still resolves fresh public branch-main scratch closures. Captured bounded/function native locks select Runtime3d2d8201, QSL52620956, kernel72b25421 and split FCD extraction/semanticIRac62202a versus schema6e2c98fc. The actual bounded-native owner check-lock exits2 for that split; native test PASS is not evidence of lock coherence. The user's explicit lunar ruling excludes that generated-scratch issue from PR513 acceptance and CG supported ROOT-cohort validation and assigns existing IR962. This scope disposition is recorded rather than ignored, repaired, re-resolved, seeded or relabelled PASS. The runner and captured scratch locks were not changed. No claim is made that these generated native legs executed root Runtimea60/QSL485, or that IR962 is fixed. This is distinct from the canonical production/observer fresh-lock and receipt requirements, whose implementation and source constraints remain unchanged and whose final gate acceptance is still parent-owned.

Six integration repairs independently inspected:

1. Genuine bounded/asymmetric source stays byte-for-byte Int[0,9] versus unbounded Integer with convert<Integer>(a). The stale absence-of-conversion premise is replaced by positive checks of exactly one authentic conversion, target Integer, original bounded parameter node and equality's genuine converted argument. No source substitution, copied IDs or generated-oracle-derived expected bounds. Claim descriptors, source/target order, independent native values including right operands outside[0,9], domain/no-narrowing and charge assertions remain. This repairs both original bounded-native and claim-descriptor failures. Existing spec AC43's erased-conversion limitation is conditional and does not prohibit the genuinely retained path now observed; independent admitted-IR conversion-reference controls are preserved.
2. Function native WrongValueKind Reference control now uses the real public kernel ObjectReference/UniverseId/EffectiveId/ObjectId, preserving opaque test values[6;32], [7;32] and authored object string. It still expects WrongValueKind(parameter0) and no charges, and expressly does not claim DanglingReference coverage. No nominal compiler provenance is invented for these original negative-control bytes.
3. The ACTUAL generated-v2-clause-corpus manifest now names QSV using the shared public SEMANTIC_VALUE_DEPENDENCY_SOURCE. The earlier misplaced dependency in the unrelated order_numbers-only first control was removed, and preflight's sole missing-manifest finding was corrected at33440669. All15 clause expressions, native expected values, source-map/probe checks and bounded inputs remain. No renderer change or suppression replaces the real missing dependency.
4. Routed scalar Quantity conversion expects the actual quantity_owner::convert_quantity call, retaining Generated dispositions, absent harness and OperationNotRendered reason. It is a structural/disposition control and supplies no positive Quantity native execution evidence. Separate scalar Quantity payload/converted-value work is not silently accepted by this check.
5. The LLVM fixture now declares its actual Exact dependency and uses quire_exact as rt in its native test. Whole-file comparison at accepted9b01 showed that isolated import change preserves every input, limit, expected Completed(true), genuine LLVM coverage invocation and evaluation1/consequent0/classification=vacuous assertion. The fixed code does not bypass LLVM or weaken vacuity observations.
6. IR850's existing profile constant is publicly re-exported, with a FR014AC8/TC024 regression asserting the emitted QSV dependency line through that public root binding. Compiler redE0425 before export and one-test green afterward are retained. The real external clause corpus also uses the public constant. No manifest source spelling/owner implementation is duplicated. Function output aliases rt=Runtime exact, exact=kernel and sv=QSV declaration remain; scalar output owner aliases are unchanged. This exact accepted source lineage is independently recorded in validated SR15393/15394 at ir850-export-review/final-readback, now retained by ancestry in this combined candidate.

Actual combined-candidate evidence: final-summary.json and each exact JSON/raw log match bounded-parameter, claim-descriptor, function-native, clause-native, quantity-route and llvm-native; each runs one outer test with zero failures/ignored and exits0. Both native agreement runners require every declared inner assertion, the clause corpus retains its15 expressions, and LLVM executes instrumented native code. Bounded JSON wall158.29s includes build work; raw test80.48s, function native78.51s, clause5.60s, route0.94s and LLVM5.42s agree with the recorded test results. These are fresh combined-candidate author runs, not merely earlier local IR850 lock validation. Root fmt-check, locked all-target Clippy -D warnings, Quire/spec with existing warnings, diff, owner check-lock and metadata checks exit0. The earlier six-control failure remains visible alongside them. No aggregate, Kani/CBMC, proof receipt or all-native Quantity execution is inferred from these focused passes.

The entire authored UnitTable slice remains independently covered by this reviewer: units.rs, real admit_environment wiring, emitted preparation, genuine unused unit/owner-ID assertions, direct/nested Quantity refusals, isolated sibling/source/symbol checks. These files and production equality logic are unchanged since the accepted prior round. Metadata attachment is still post-guard, clone-complete and before Union sealing; None behavior, typed refusal order, no equality charge on preparation failure, key-vs-node roles and owner member seals are unchanged. No positive equality Quantity/conversion, native Union or emitted metadata-constructor execution is claimed. Quantity Unsupported AC18/38 and frame Quantity->NonRangeType remain. Original fifteen genuine sources/domains, six host bodies, protected qualification files, work counts, deep source generator and512KiB stack controls are unchanged by this round. No compatibility layer, fake facade identity or copied owner implementation was added.

Fresh main724 publishes the supported QSL/kernel/FCD cohort and its owner review; these are baseline context, not new consumer defect fixes. Makefile/scripts/workflows have zero incremental change against main. No normative CG spec/plan delta exists beyond the previously reviewed owning amendments, so existing spec/AC45/Union review lineage remains applicable. No applicable AssuranceProfile or new assurance exception is introduced. Optional semantic gap analysis was not opted in; mandatory code/Rust oracle checks and full mechanical repository requirement/test/source reconciliation were performed. Latest committed official code/gap documents match disposition5 byte-for-byte; append-only Round6 records preserve their entire history. Current final artifacts plus IR850 final-readback files must be retained before parent final gates.

Fresh complete matrix: candidate899 criteria,544 tagged,287 untagged,16 ignored-only,52 method-without-symbol. Exact archived current-main724 baseline via quire coverage:897 criteria,525 tagged,302 untagged,18 ignored-only,52 method-without-symbol. The same19 incremental status/addition improvements remain, with no newly untagged criterion. New public-export regression is actually bound to FR014AC8; LLVM/other controls retain their proper trace tags. Static binding does not discharge compiled/native evidence, and the raw runs are documented separately above.

All287 untagged criteria occur in the existing SR15352 inventory with preserved outcomes. Original gapFND001 FR005AC3 remains with that existing CLI/planner owner (IR954/IR17), outside this migration. Original gapFND002's static FR016AC25 gap remains remedied by existing baseline IR604/PR512, retaining its historical deferred disposition and not inventing an IR17 fix or claiming dedicated Kani controls executed here. Both baseline/candidate still report10 inherited coverage diagnostics and7 resemblance suspicions. Whole requirement/source/test ownership chain and current changed assertions were audited; no new static gap, stub, hollow API or unowned behavior was found. Repository completeness FAIL remains inherited; incremental gap PASS. IR962's real fresh-scratch coherence failure has its explicit existing owner and user scope ruling above; no duplicate unowned remediation or false completion is introduced. Plan completion:not assessed. No baseline requirement or gate is silently waived.

```yaml
dispositions: []
new_findings: []
review_head: 9b2c69f22f8840930fde7a7a29abe1ff1859eab5
baseline_head: 724d6bfba77154dde44623655e57d3d7c92d8301
scope:
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts
    of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2`
    clause bodies.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation
    was observed, that has at least one implication and for which no expected consequent
    probe was observed is `vacuous`.
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, each malformed input class
    of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-017-AC-53
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Both package locks are independently freshly resolved with the existing
    bounded offline generation/update workflow, without copying a parent/other lock
    or substituting a cached resolution. Each resolves both owning runtime and exact
    identities to the build's authoritative lock identities; the independent observer
    regression asserts both literal package names and repository identities independently
    of the renderer's list. Both manifests, locks and all other AC-46 entries are
    sealed and compared as a complete path/byte set before proof, after proof and
    after native execution; mutation/replacemen
- id: FR-014-AC-8
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The generated crate declares `publish = false`, names the direct exact/QSV
    and Runtime residue dependencies from the one profile authority without a replay-context
    dependency for Boolean/integer scalar proofs, contains no charge amount (every
    charge comes from runtime metering), and compiles.
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: The Boolean condition in the third argument of a `state`/`state_clause`
    node's `quire.op.state.clause` body, addressed by the clause node id, derives
    a descriptor and generates an `ir_confirmed` oracle over its connectives, integer
    comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`.
    When the condition is a reference to an admitted Boolean expression node, the
    oracle uses that node's expression, not the reference as a Boolean variable; an
    inline condition uses its own expression. In both cases the claim and generated
    oracle retain the clause node identity, not
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269), because the Kani scalar renderer builds no quantity harness; the conversion
    is credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-018-AC-3
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "Each generated item's claim-map entry records the descriptor the request\
    \ supplied \u2014 operator, both operand source types and both conversion targets\
    \ \u2014 and it is equal to the request's, so the independent native run of AC-2\
    \ is driven from the request and never read back out of the generated crate. An\
    \ authentic asymmetric source control preserves the actual bounded-left/unbounded-Integer-right\
    \ source IDs and independent caller target; a source conversion erased from the\
    \ emitted graph is not evidence of a retained conversion-reference branch."
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested conversions to the first node that is not a conversion, and a reference
    to any other node, including a non-`convert` application, as that node''s own
    `semantic_type`. A descriptor whose `source_type` is the read type generates,
    and any other source is refused as an operand-type disagreement at that position,
    reporting the read type as found. A genuine source fixture with no emitted convert
    node exercises its actual parameter source types'
- id: FR-018-AC-43
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'For genuine source with a: Int[0,9] and b: Integer, the admitted left
    parameter names its integer_range and the right parameter names canonical base
    Integer. Direct source/target reconstruction preserves Int[0,9] versus unbounded
    Integer despite the sibling range: a caller descriptor converting the left to
    Integer compares with the unbounded right under the original equality rule, including
    right values outside [0,9]; an unbounded Integer source cannot acquire a narrowing
    conversion by borrowing that sibling interval. Claim/native descriptor checks
    remain independent of generated output. If so'
- id: FR-018-AC-45
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: After the existing setting/accounting and original-package context guards
    succeed, generation-time construction and emitted preparation supplied Some(context)
    attach a clone of that context's complete authoritative unit_table() to the constructed
    TypeEnvironment, retaining declared but unused units without rebuilding a table,
    deriving UnitId/NodeKey or substituting another package's table. Valid context-free
    None behavior remains unchanged. Wrong-package refusal retains its whole-request
    precedence; direct and nested Quantity items retain their existing typed Unsupported
    behavior and refusal p
- id: FR-021-AC-4
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'The oracle returns the arity, wrong-value-kind, dangling-reference and
    unknown-function input refusals unchanged to the caller before any execution charge;
    the schedule records ordered arguments without inventing a refusal result. That
    the refusals equal QSL-owned ones is outside CG''s boundary (IR-842, FR-014-AC-47).
    PLANNED, blocked on IR-853: once Contract Runtime FR-275 deletes the `exact` port,
    the refusal comparison''s target is decided by IR-853.'
prior_outcomes_confirmed:
  FND-001: deferred existing SR15352/IR954 CLI owner
  FND-002: historical IR604 deferral retained; current baseline trace gap remediated
gate_issue_outcomes:
- control: bounded parameter + claim descriptor
  state: fixed and exact combined controls PASS
  source: tests/it/admitted_enum_equality.rs genuine conversion provenance; original
    source/domains retained
- control: function native
  state: fixed and exact combined control PASS
  source: tests/exact_function_support/agreement_cases.rs public kernel reference
    carriers; WrongValueKind/no charges retained
- control: clause corpus
  state: fixed and exact combined control PASS
  source: tests/it/kani_obligations_state_frame.rs actual corpus QSV manifest; public
    export available
- control: Quantity route
  state: fixed and exact combined control PASS
  source: tests/it/routed_generation.rs owner-call assertion; no native Quantity execution
    claim
- control: LLVM vacuity
  state: fixed and exact combined control PASS
  source: tests/it/v2_llvm_coverage.rs Exact dependency and native import; vacuity
    oracle unchanged
included_review_lineage:
- SR-15393
- SR-15394
known_out_of_scope:
  issue: IR-962
  actual_check_lock_exit: 2
  scope: fresh native generated scratch FCD split; user/luna excludes from PR513/root
    supported cohort acceptance
  root_owner_check_lock_exit: 0
qualification: source PASS; six exact combined controls PASS; conditional on final
  parent ci/Kani and separate lunar lock review
```
