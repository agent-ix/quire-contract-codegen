# Draft dependency pins

Status: provisional; mandatory reconciliation before semantic implementation leaves draft.

| Dependency | Current identity | State | Reconciliation trigger |
|---|---|---|---|
| Program governance | PR #12 head `0b8669b80f98b6c11954f922b32d9edae8a11983`; envelope schema SHA-256 `0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256` | in review; provisionally reconciled | merged policy revision and schema digest |
| IR schema/corpus | `agent-ix/quire-contract-ir#10` | open; no PR/candidate branch | reviewed schema, corpus revision, digest, diagnostics, dependency-set contract |
| Runtime helpers | `agent-ix/quire-contract-runtime#5` head `87f75757e9b6687cf0502c0c55969a13ec10f924` | rebased manual-CI draft; local gates pass, current remote run pending | reviewed/merged source tag, public API and feature reconciliation |

Draft work must not invent missing IR types or publish a stable dependency claim. A later rebase or
rebranch may replace every provisional name and identity without compatibility promises.
