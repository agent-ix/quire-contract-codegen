# Historical foundation evidence

This directory retains incomplete or failed-closed foundation collections so they cannot be
mistaken for the current candidate record.

- `foundation-a7790d225746-20260831T154248Z` binds clean source
  `a7790d22574666ce092ad2e4cc6f7959121f9849`. Every local lane and the PGM-01 schema validator
  passed, but the PGM-01 custom validator exited `2` because its pinned
  `rfc3339-validator==0.1.4` dependency was unavailable in the invoking Python environment. The
  collector failed closed and made no successful-evidence claim. A later record uses an isolated
  environment containing the exact published validator dependencies.
