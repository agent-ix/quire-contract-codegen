# Default-branch protection observation

Captured at: `2026-08-30T21:13:49Z`

Repository and branch: `agent-ix/quire-contract-codegen:main`

Source endpoints:

- `GET /repos/agent-ix/quire-contract-codegen/branches/main/protection`
- `GET /repos/agent-ix/quire-contract-codegen/collaborators`
- tracked `.github/CODEOWNERS` at `origin/main`

Observed controls:

| Control | Observed value |
|---|---|
| Strict required checks | enabled |
| Required check contexts | `Rust Checks`, `License Check` |
| Required approving reviews | 1 |
| CODEOWNER review | required |
| Dismiss stale reviews | enabled |
| Enforce for administrators | enabled |
| Conversation resolution | required |
| Force pushes | disabled |
| Branch deletion | disabled |
| CODEOWNER | `@kreneskyp` |
| Direct collaborators returned | `kreneskyp` (admin/maintain/push) |

CI trigger coordination is tracked in PR #8 at commit
`2f95623c8f8e633a3df2226f11328bea960c584f`. It removes automatic push and pull-request
triggers while preserving `workflow_dispatch`. Required checks therefore need a deliberate dispatch
for the candidate revision before merge.

This observation does not record a passing remote run, an approval, or a release decision.
