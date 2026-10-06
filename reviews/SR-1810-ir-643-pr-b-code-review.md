---
id: SR-1810
title: IR-643 PR B code-review review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen@5aee240919ff981cde808ec2d9135247b7f3182a; Cargo.lock,
  src/oracle/equality/mod.rs, src/oracle/equality/resolution.rs, tests/composite_equality_support/codes.rs,
  tests/composite_equality_support/package.rs, tests/it/composite_equality_generation.rs
review_set: subset
---

## Summary

Ticket: IR-643. Independently reviewed PR #308 at the pinned head. The QSL lock has one revision and the iterative resolver closes record back-edges and refuses direct Option/Sequence cycles.

## Verdict

**FAIL** — the admitted deep Option path can still exhaust the call stack.

## Coverage

Plan completion: not assessed. Review is scoped to the PR diff; existing repository-wide matrix gaps are outside this verdict.

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018-AC-24 | examined | Recurse from `Option` into itself without tracking an unclosed type, or reject every repeated node including the List and Tree record back-edges; the former overflows the stack and the latter refuses valid recursive records. |
| FR-018-AC-25 | examined | Reuse the Contract IR lowering counter for reconstruction, charge only record declarations while skipping option and sequence edges, or recurse through 65,537 acyclic entries before checking the budget; a large item then completes or overflows the stack instead of yielding the named per-item refusal. |
| FR-018-AC-26 | examined | Expect `next`'s value to be a direct reference and reject QSpec's nested optional binding, or ignore the wrapper and treat the field as required; the List item refuses or generates with the wrong presence. |
| FR-018-AC-27 | examined | Expect an inline second bounds reference in the Sequence body, or reject every bounded domain whose semantic type is not scalar; the QSpec Tree item refuses. Ignoring a missing bound instead generates an unbounded collection. |
| TC-029-step-16 | examined | Read the authoritative QSpec `proposals/checked-package-v2/fixtures/positive-recursive-records.json` through Contract IR's strict reader, without copying it into this repository. Build equality expression items over the fixture's List (`Option<List>`) and Tree (`Sequence<Tree>[0,3]`) record declarations in a valid test package; the fixture itself supplies type nodes, not equality expressions. Assert both items generate and the call terminates. |
| TC-029-step-17 | examined | At that same seam, use an acyclic walk whose roots and references across both operands and every conversion target total exactly 65,537 entries, with a valid terminating leaf and a healthy sibling. Assert the first item refuses `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }`, emits no symbol and leaves the sibling generated; a separate walk totaling exactly 65,536 per-item entries across every operand and target does not receive the work refusal. |
| src/oracle/equality/resolution.rs | examined | The frame loop charges Enter, resolves nested Option/Collection values, and charges an inline collection bound in FinishCollection. |
| src/oracle/equality/mod.rs | examined | check_item resolves four descriptor roots, clones ValueType operands, and render_value_type recursively renders Option/Collection. |
| tests/it/composite_equality_generation.rs | examined | The AC-24 public generation test uses corpus_package and recursive_tree_package, both hand-built. |
| tests/composite_equality_support/package.rs | examined | The new recursive_tree_package builder hand-constructs Tree, Sequence, collection bounds, and equality expression. |
| tests/composite_equality_support/codes.rs | examined | Adds Tree-specific fixture codes. |
| Cargo.lock | examined | Eight QSL crates move together to merged revision 840c36d9a94ecff00700959d23773e8e78c47888. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A valid acyclic chain of up to 65,536 Option nodes is admitted by the iterative resolver and builds a Box-linked ValueType, but check_item clones that type and render_value_type recursively descends it (mod.rs:849-858, 1662-1668). Either path can exhaust the call stack before the item is generated, contradicting AC-25’s no-stack-exhaustion and at-limit generation contract. Bound the complete downstream representation/processing without rejecting allowed work, and exercise a deep admitted Option chain in a subprocess. | src/oracle/equality/resolution.rs:290-317 |
| FND-002 | medium | An inline collection bound is looked up and validated in collection_parts, then charged only in FinishCollection after descending its element. With a 65,536th collection entry and a malformed bound, the item returns UnreadableBound rather than charging the followed bounds reference as entry 65,537 and returning TypeResolutionWorkExhausted before descent. Charge the bound reference before dereference/validation and before scheduling the element. | src/oracle/equality/resolution.rs:73-78 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5cddd07119e33633d8edf3670f93ac23db289f3c — `render_value_type` now walks nested types with an explicit task stack; the RT revision adds iterative `ValueType` clone, equality and drop. The 2,048-deep Option case resolves, clones, compares, renders and drops on a 64 KiB worker stack. |
| FND-002 | fixed | a3f99c4 — the inline bound is charged before its reference is read or looked up, and the focused malformed-bound boundary test asserts the refusal precedence. |

## Post-disposition recheck

Reviewed PR head `5763404d70f544830f76f34c9f95ed1c1472b8c4` after the native-coverage test toolchain change. The two tests now select Cargo.toml's `rust-version` for both `rustc --print sysroot` and `cargo llvm-cov`; `make tools` installs `llvm-tools-preview` for that same `$(MSRV)`. `make -n tools` selected 1.98.1, and both affected focused tests passed under `cargo +1.98.1 test --locked --test it`. No new code or Rust findings.
