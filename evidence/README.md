# Retained evidence

Foundation evidence is produced by `scripts/collect_foundation_evidence.sh`. By default it creates a
new revision-and-UTC-timestamp-scoped directory and refuses to overwrite an existing record. It
identifies the exact source state and every provisional upstream dependency. It proves only
specification/baseline gates; it cannot support semantic generation, parity, or release claims.

The collector emits canonical `quire.derivation-evidence/v1` JSON plus separately versioned
foundation-input and manifest schemas. Set `PGM01_VALIDATOR` to the reviewed IR repository's
`scripts/validate_governance.py` to retain an exact cross-repository validation result; absence is
recorded as `skipped-unavailable`, not passed.

CI workflows are manual-only. Any remote run used as evidence must be explicitly triggered and its
source revision and run URL retained alongside the local evidence envelope.
