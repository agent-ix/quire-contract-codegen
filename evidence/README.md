# Retained evidence

Foundation evidence is produced by `scripts/collect_foundation_evidence.sh`. By default it creates a
new revision-and-UTC-timestamp-scoped directory and refuses to overwrite an existing record. It
identifies the exact source state and every provisional upstream dependency. It proves only
specification/baseline gates; it cannot support semantic generation, parity, or release claims.

The collector emits canonical `quire.derivation-evidence/v1` JSON plus separately versioned
foundation-input and manifest schemas. The collector gates those local schemas with the installed
`jsonschema` Draft 7 implementation and records both its version and the Python version. Set `PGM01_SCHEMA` to the reviewed IR repository's envelope
schema and `PGM01_VALIDATOR` to its `scripts/validate_governance.py` to retain independent schema and
custom-validator results. An absent optional PGM-01 gate is recorded as `skipped-unavailable`, not
passed.

CI workflows are manual-only. Any remote run used as evidence must be explicitly triggered and its
source revision and run URL retained alongside the local evidence envelope.
