# Retained evidence

Foundation evidence is produced by `scripts/collect_foundation_evidence.sh`. It identifies the exact
source state and every provisional upstream dependency. It proves only specification/baseline gates;
it cannot support semantic generation, parity, or release claims.

CI workflows are manual-only. Any remote run used as evidence must be explicitly triggered and its
source revision and run URL retained alongside the local evidence envelope.
