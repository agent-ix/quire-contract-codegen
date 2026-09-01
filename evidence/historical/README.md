# Historical foundation evidence

This directory retains superseded, incomplete, or failed-closed foundation collections so they
cannot be mistaken for the current candidate record.

Any `conclusive` result embedded in this directory is explicitly retracted as an authoritative
claim. Historical bytes are immutable diagnostics; only records named directly by `evidence/ANCHORS`
may support the current foundation statement.

- `foundation-a7790d225746-20260831T154248Z` binds clean source
  `a7790d22574666ce092ad2e4cc6f7959121f9849`. Every local lane and the PGM-01 schema validator
  passed, but the PGM-01 custom validator exited `2` because its pinned
  `rfc3339-validator==0.1.4` dependency was unavailable in the invoking Python environment. The
  collector failed closed and made no successful-evidence claim. A later record uses an isolated
  environment containing the exact published validator dependencies.

- `foundation-374a6a3060ad-20260831T163004Z` is an honest failed-closed collection with three
  failed foundation checks and an `inconclusive` result. It predates the complete outcome and
  record-set verifier and is retained only as diagnostic history.
- `foundation-fc6bbcf0d392-20260831T171947Z` is an honest failed-closed collection with four
  failed foundation checks and an `inconclusive` result. It demonstrates that clippy, test, MSRV,
  and rustdoc failures were retained rather than deleted; it is not authoritative evidence.
- `foundation-2753a4f93301-20260831T203535Z` is an honest failed-closed collection whose four Rust
  lanes could not write the sandbox-external Cargo target directory. Its `rejected` result retains
  those failures; a successor collection uses an explicit writable local target directory.
- `retired-pre-parameter-binding/foundation-984594e48a0c-20260831T172037Z` was the prior pending
  authoritative record. It was superseded because its verifier did not yet re-derive the parameter
  digest or enforce the complete manifest/limitation census. Its in-band historical disposition
  retracts it from the authoritative claim set.
- `retired-round4-control-strengthening/foundation-b80f432105ed-20260831T193347Z` was the prior
  pending authoritative record. It was superseded after review found that the recursive Python
  runner could report success without executing `unittest` cases and that the retained control set
  did not yet cover the accepted IR merge, bidirectional matrix census, or Make invocation guards.
  Its in-band historical disposition retracts it from the authoritative claim set.
- `foundation-3cd2918c62e5-20260831T224142Z` is a failed-closed Round 5 collection. Its unsafe
  audit correctly returned nonzero but exposed that the scanner interpreted a Rust source snippet
  inside a Python negative fixture as live Rust; the successor restricts that audit to `*.rs`.
- `retired-round5-boundary-hardening/foundation-35f526287109-20260831T203710Z` was the prior
  pending authority. It predates trusted Python/tool identity, positive transcript corroboration,
  exact matrix tuples, behavioral gate probes, and the review-visible anchor delta controls.
- `retired-round5-boundary-hardening/foundation-dcc215fac22f-20260831T224253Z` was a passing
  intermediate authority superseded when four remaining critical gate call sites received direct
  behavioral fixtures. Its in-band disposition retracts it from the current claim set.
- `retired-round6-evidence-hardening/foundation-287b2ec32ca7-20260831T224619Z` was the prior
  pending authority. It was superseded after review required final-state Make guards, truthful
  collector commands, robust Rust attribute scanning, shared evidence floors, positive formatting
  corroboration, and live re-derivation of every retained tool identity. Its in-band disposition
  retracts it from the current claim set.
- `retired-oracle-rebase-round6/foundation-f71014dbcc02-20260901T001612Z` was the Round 6
  foundation authority. It is superseded by the source-bound deterministic-oracle record after the
  stacked PR was rebased; its in-band disposition retracts it without deleting the audit history.
- `retired-harness-rebase-round6/foundation-ad384644f07f-20260901T005225Z` was the rebased
  deterministic-oracle authority. It is superseded by the source-bound harness/strategy record;
  its in-band disposition preserves the PR #10 boundary beneath the stacked PR #12 work.

## Machine-checked record census

The verifier requires this exact bidirectional census to match every retained historical envelope:

- `foundation-088a1692b78e-20260831T015600Z`
- `foundation-113c1f17730a-20260831T003723Z`
- `foundation-1ed7e8e0d7c0-20260831T005601Z`
- `foundation-2753a4f93301-20260831T203535Z`
- `foundation-287b2ec32ca7-20260831T224619Z`
- `foundation-3536b8bc82a3-20260830T224639Z`
- `foundation-35f526287109-20260831T203710Z`
- `foundation-374a6a3060ad-20260831T163004Z`
- `foundation-38363fe7ed34-20260830T232349Z`
- `foundation-3b3820cf5b6a-20260830T230214Z`
- `foundation-3cd2918c62e5-20260831T224142Z`
- `foundation-51cf32624db0-20260830T223315Z`
- `foundation-5ca054beccdc-20260830T214724Z`
- `foundation-65da338d8149-20260831T001333Z`
- `foundation-747ceab185d8-20260831T154546Z`
- `foundation-795455f27a70-20260830T235823Z`
- `foundation-86a4fe771344-20260831T002340Z`
- `foundation-86eed791a2d4-20260830T220953Z`
- `foundation-87c655011643-20260830T213741Z`
- `foundation-8a4537d32002-20260830T225332Z`
- `foundation-8db0acefa23d-20260830T212921Z`
- `foundation-96cd217bf3ae-20260830T215443Z`
- `foundation-9796a1412e16-20260831T003007Z`
- `foundation-984594e48a0c-20260831T172037Z`
- `foundation-9fc65135d0ce-20260830T215714Z`
- `foundation-a6706357c0b1-20260831T033106Z`
- `foundation-a7790d225746-20260831T154248Z`
- `foundation-a88d192ab03a-20260830T213405Z`
- `foundation-ad384644f07f-20260901T005225Z`
- `foundation-b3bb8615d6f0-20260831T002835Z`
- `foundation-b80f432105ed-20260831T193347Z`
- `foundation-b90d6d5946e5-20260830T220017Z`
- `foundation-cf16bd39aab7-20260831T022900Z`
- `foundation-cf40e894221c-20260831T163108Z`
- `foundation-dcc215fac22f-20260831T224253Z`
- `foundation-e1463f3f7719-20260831T010208Z`
- `foundation-e76fd5eb6ddb-20260831T001807Z`
- `foundation-e8c701cfe024-20260831T002430Z`
- `foundation-f17fdd258b87-20260831T154719Z`
- `foundation-f71014dbcc02-20260901T001612Z`
- `foundation-fc6bbcf0d392-20260831T171947Z`
- `foundation-fcdcc0c1593b-20260830T220427Z`
