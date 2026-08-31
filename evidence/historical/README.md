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
- `retired-pre-parameter-binding/foundation-984594e48a0c-20260831T172037Z` was the prior pending
  authoritative record. It was superseded because its verifier did not yet re-derive the parameter
  digest or enforce the complete manifest/limitation census. Its in-band historical disposition
  retracts it from the authoritative claim set.
- `retired-round4-control-strengthening/foundation-b80f432105ed-20260831T193347Z` was the prior
  pending authoritative record. It was superseded after review found that the recursive Python
  runner could report success without executing `unittest` cases and that the retained control set
  did not yet cover the accepted IR merge, bidirectional matrix census, or Make invocation guards.
  Its in-band historical disposition retracts it from the authoritative claim set.
