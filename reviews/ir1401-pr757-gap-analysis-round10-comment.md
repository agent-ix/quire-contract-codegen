<!-- reviewer-dispositions repo=agent-ix/quire-contract-codegen visibility=public quoin=0.29.1 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-17940486709564847047571 round=10 model=gpt-6.1-sol run=1d144a51-4f12-4c91-a588-fc92ed068766 -->

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | fixed | Governing AC, adverse case and TC-029 now preserve the actual native public Some("ambiguous-name") cause; unchanged authentic source test passes and still produces no context/CG artifact. |

Targeted frozen review of 9ce68c8dceedc7750e6169bc3d2a623be6a41bfa...9b4ce31758ddfee5c345abd5c42848867166ce5c, all nine files examined. Committed code/gap artifacts compare byte-for-byte with the reviewer's round9 files. All original findings and disposition history are preserved as exact prefixes in these updates. No Cargo, broad review/build, source edit, branch change, push or merge by reviewer. No applicable AssuranceProfile exists; no object/schema/dependency edge change calls for another lens.

FND-013: the four independent algorithms are removed. One tests/checked_package_support/draft_rows.rs::author_row owns explicit detached-row authoring. Iterative LIFO traversal pushes object values/array items in reverse so processing retains canonical field/element order; local document indices deduplicate in encounter order and become fresh operand slots. A nonempty input row or existing local slot fails explicitly. Foreign ID objects are skipped intact, preserving their document frame. This is one shared source compiled into test modules, not copied source or a production protocol mirror. Search confirms only the shared implementation remains.

Call boundaries retain the qualification distinction: the case control authors only its newly appended equality row; the arithmetic control authors only collected intentional modified rows after public holder-aware inspection. The carried producer authors its own deliberate fixture nodes. The opaque control first admits original unchanged published bytes, then authors a detached inspection draft. The helper accepts one selected node and fresh row; it neither reads/replaces whole packages nor normalizes reader inputs. Existing source maps and imported graph nodes are untouched.

Gap FND-002: FR-018-AC-8, its adverse case and TC-029 now require the actual public native Some("ambiguous-name") cause and prohibit erasing or synthesizing it. The AmbiguousDeclaration code, owning Package refusal, absent successful AdmittedContext, no public CG generation and private duplicate-member separation remain explicit. Inspected current locked QSL public-owner evidence in round9 and unchanged exact duplicate-field test. No source/test error projection drops the native cause.

Inspected /tmp/CG757-round10-report.json and every listed log. All seven exact tests actually run one named case and pass: carried/equality/bounded draft producers, incoherent IllTyped, malformed aggregate reader negative, opaque isolation and authentic native duplicate cause. Scoped Clippy reports success; format success and its empty log match the runner receipt. Independent quire validation of the two changed specs passes (/tmp/cg757-round10-review-spec.log). No full forty-seven-case or aggregate rerun is claimed. Docs remain unrun in the lead's continuation.

+++ [reviewer data]

```yaml
scope:
- id: spec/oracle/functional/FR-018-composite-equality-oracles.md
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '---'
- id: src/oracle/scalar/lower/opaque_metadata_tests.rs
  path: src/oracle/scalar/lower/opaque_metadata_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/checked_package_support/draft_rows.rs
  path: tests/checked_package_support/draft_rows.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: tests/exact_scalar_support/package.rs
  path: tests/exact_scalar_support/package.rs
  role: examined
  excerpt: //! Admitted CheckedPackage V2 fixtures for exact scalar generation.
- id: tests/it/composite_equality_generation.rs
  path: tests/it/composite_equality_generation.rs
  role: examined
  excerpt: '//! FR-018: composite/structural equality oracle generation over admitted'
- id: tests/it/v2_clause_integer_literals.rs
  path: tests/it/v2_clause_integer_literals.rs
  role: examined
  excerpt: //! Public generation controls for the general V2 clause integer-leaf policy.
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Test |
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: '`cause() == Some("ambiguous-name")` for this Assembly refusal, retaining
    the'
new_findings: []
dispositions:
- fnd: FND-002
  outcome: fixed
  after_excerpt: Test |
```

+++
