# Default-branch protection observation

Captured at: `2026-08-31T16:22:09Z`

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
| Enforce for administrators | disabled |
| Conversation resolution | required |
| Force pushes | disabled |
| Branch deletion | disabled |
| CODEOWNER | `@kreneskyp` |
| Direct collaborators returned | `kreneskyp` (admin/maintain/push) |

CI trigger coordination is tracked in PR #8 at commit
`2f95623c8f8e633a3df2226f11328bea960c584f`. It removes automatic push and pull-request
triggers while preserving `workflow_dispatch`. Required checks can be deliberately dispatched for a
candidate revision, but administrator enforcement is disabled and an authorized administrator can
merge without those checks.

This observation does not record a passing remote run, an approval, or a release decision.
