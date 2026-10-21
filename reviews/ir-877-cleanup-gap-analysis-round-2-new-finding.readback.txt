<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15235 method=gap-analysis lang=spec pr=quire-contract-codegen#497 model=gpt-6.1-sol run=53c49625-af99-46bb-a313-114eaa6d8f5e -->

Same-reviewer narrow rebase disposition for PR497. Exact current head 3f1e7668ff49abaa439998bcb98572b0d4ae1b98; base 76dea88f2d22f6fe15208552a24bfdb44fe2038c. Independent binary git diff and every one of 16 owned files are byte-identical across the ordinary rebase (two commits range-diff equal). PR494 changed family/corpus outcome lowering and tests; PR496 changed composite admission stage allocation. Neither alters namespace owner/launch/poll/cancellation source, FR017 diagnostic semantics or TC043 planned controls. The diagnostic remains observational only; no timing, retries, KILL ordering, pidfd identity, errno priority or Drop behavior changes are introduced. No Cargo/fmt/metadata/native/test/proof command ran in this review.

Current base884:523 tagged,293 untagged,50 method-without-symbol,18 ignored-only. Candidate887:523 tagged,295 untagged,51 method-without-symbol,18 ignored-only. All884 base statements/methods/statuses/binder identities are preserved by this PR apart from line offsets. Computed open gaps313 =311 existing IR878/program +2 planned IR877 tests. Additionally, tagged FR018AC8 still lacks its newly required authentic duplicate-source admission control: the unchanged synthetic duplicate test admits corpus_package and calls public generate; it does not call compile_admitted or assert typed Package/CompileRefusal::code(AmbiguousDeclaration)/cause(None). Therefore semantic coverage must retain that separate IR17 gap. Current open findings314 =311 program +1 IR17 +2 IR877. Two historical IR621 findings are fixed by landed PR494, not by this rebase. AC56 Inspection is still explicitly pending IR877 implementation/source inspection. Full gap remains FAIL.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-316 | high | coverage | The authentic compile_admitted duplicate-source admission control required by landed FR-018-AC-8 remains unimplemented; existing tagged synthetic public fixture does not establish its required owning refusal/stage. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without public-acc
findings:
- fnd: FND-316
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-8
  related_ids:
  - FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '591'
  excerpt: A declaration closure `TypeEnvironment::new` refuses is refused at generation time carrying that `DeclarationCause`,
    including both recursion passes. The authentic duplicate-field source control instead retains typed AdmittedRefusal::Package
    with owning public CompileRefusal::code() equal to Code::AmbiguousDeclaration and cause() equal to None for
    the Assembly refusal, produces no successful AdmittedContext and is not passed to public CG generation. Private
    CG declaration-admission defenses retain the exact DuplicateMember name/cause and unchanged healthy sibling
    separately, without public-acc
  finding: The authentic compile_admitted duplicate-source admission control required by landed FR-018-AC-8 remains
    unimplemented; existing tagged synthetic public fixture does not establish its required owning refusal/stage.
bindings: []
```

+++