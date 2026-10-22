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
