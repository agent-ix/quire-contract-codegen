#!/usr/bin/env python3
"""Read this repository's retained evidence through the pinned shared mapping (FR-006-AC-4).

Four things this file deliberately is not.

It is not a mapping. Every byte it interprets is interpreted by
`engineering_assurance.verification_semantics.map_pgm01_bytes` from the pinned
release. This file chooses which bytes to hand over and reports what came back.
If the answer is unwelcome, the answer is still the answer.

It is not a verifier. It replaces `verify_foundation_evidence.py` and
`update_evidence_anchors.py` and it does not inherit their job. It does not
decide whether a retained record qualifies anything, does not maintain an anchor
file, and has no notion of an authoritative record.

It is not a writer. It opens every file under `evidence/` for reading and proves
it: it digests the whole tree before and after the run and fails if one byte
moved. Read-only is a claim, so it is measured.

It is not a translator of last resort. This repository's 44 retained envelopes
are `quire.derivation-evidence/v1`. The pinned mapping covers
`quire.pgm01-evidence` v1 and v2 and answers `incompatible` for the rest. That
refusal is reported as the compatibility result. Writing a local mapper to turn
it into something friendlier is the exact thing the migration removed.

Exit status: 0 when every declared case matched and no evidence byte moved,
1 when a case did not match or bytes moved, 2 when the pinned release cannot be
loaded.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any, Callable

ROOT = Path(__file__).resolve().parent.parent
EVIDENCE = ROOT / "evidence"
FIXTURES = ROOT / "tests" / "fixtures" / "legacy-compat"
EXPECTATIONS = FIXTURES / "expectations.json"
PINS_PATH = ROOT / "assurance" / "pins.json"

# The mapping's own state vocabulary, as PGM01_STATE_MAP defines it. The census
# requires every one of these to be demonstrated by some case, because a state
# nothing exercises is a state nothing would notice the loss of.
REQUIRED_STATES = ("passed", "failed", "error", "skipped", "inconclusive", "unavailable")

# The outcomes map_pgm01_bytes can return. Same argument.
REQUIRED_OUTCOMES = ("lossy", "incompatible", "unreadable")

# What a declared `kind` obliges the case to have actually observed.
#
# `kind` is the label the twelve-state census counts, and it used to be free text
# that nothing cross-checked: `run_case` asserted the outcome, the record id, the
# schema version and the reasons, then echoed `kind` back untouched. An
# adversarial review relabelled two healthy PGM-01 v1 records — one whose check
# says `fail`, one whose check says `error` — as `malformed` and `unsupported`,
# and the whole suite stayed green while two of the twelve states were
# "demonstrated" by records exhibiting neither.
#
# A case whose label is not supported by its own observation does not match, so
# the label cannot be borrowed. A kind this table does not name is refused, because
# a kind nothing constrains is a kind anything can claim.
KIND_OBLIGATIONS: dict[str, dict[str, Any]] = {
    # A positive control is a case where the mechanism read the record as itself
    # rather than refusing it. That is the claim, and it is what is checked: the
    # mapping must not have answered with a substitute identity. Two different
    # positive controls live here — a released record the mapping accepts
    # outright, and this repository's own retained bytes presented under their own
    # true digest, which the integrity binding accepts and the schema check then
    # refuses for an unrelated reason — and both are controls in the same sense.
    "positive_control": {"record_id_not": ("tampered-source", "unreadable-source")},
    "failed": {"outcome": "lossy", "state": "failed"},
    "unavailable": {"outcome": "lossy", "state": "unavailable"},
    "inconclusive": {"outcome": "lossy", "state": "inconclusive"},
    "error": {"outcome": "lossy", "state": "error"},
    "stale": {"outcome": "lossy", "state": "stale"},
    "not_computed": {"outcome": "unreadable"},
    "malformed": {"outcome": "unreadable"},
    "unreadable": {"outcome": "unreadable", "record_id": "unreadable-source"},
    "unsupported": {"outcome": "incompatible", "reason_contains": "unknown PGM-01 schema version"},
    "tampered": {"outcome": "incompatible", "record_id": "tampered-source"},
}


class ViewError(RuntimeError):
    """The pinned mapping or its inputs could not be used."""


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def load_mapper() -> Callable[..., dict[str, Any]]:
    try:
        from engineering_assurance.verification_semantics import map_pgm01_bytes
    except ImportError as error:
        raise ViewError(
            f"the pinned assurance distribution is unusable: {error}. "
            "Run `make assurance-env`. This is an error and not a skip."
        ) from error
    return map_pgm01_bytes


def release_root() -> Path:
    try:
        import engineering_assurance
    except ImportError as error:
        raise ViewError(f"the pinned assurance distribution is unusable: {error}") from error
    return Path(engineering_assurance.__file__).resolve().parent


def pinned_source_bytes(relative: str) -> bytes:
    """Read a pinned artifact out of the installed release, checked against pins.json.

    The derived fixtures in this repository are one named edit to these bytes. If
    the release's bytes are not the bytes the pin names, the derivation claim is
    void and there is nothing to compare a fixture against.
    """
    pins = json.loads(PINS_PATH.read_text(encoding="utf-8"))
    expected = None
    for artifact in pins["consumed_artifacts"]:
        if artifact["path"] == relative:
            expected = artifact.get("sha256")
            break
    if expected is None:
        raise ViewError(f"{relative} is not a digest-pinned consumed artifact")
    path = release_root() / relative
    if not path.is_file():
        raise ViewError(f"{relative} is absent from the installed release")
    raw = path.read_bytes()
    actual = sha256(raw)
    if actual != expected:
        raise ViewError(f"{relative}: {actual}, pins record {expected}")
    return raw


def derive(raw: bytes, derivation: dict[str, Any]) -> bytes:
    """Apply exactly one named edit to real bytes.

    Every branch refuses an edit that changes nothing. A derivation that leaves
    the bytes as it found them produces a fixture that demonstrates the state of
    its source rather than the state it claims, and a probe that mutates nothing
    detects nothing while reporting that it did.
    """
    operation = derivation["operation"]
    if operation == "truncate":
        count = int(derivation["bytes"])
        if count >= len(raw):
            raise ViewError("a truncation that removes nothing is not a derivation")
        return raw[:count]
    if operation == "replace":
        find = derivation["find"].encode("utf-8")
        replace = derivation["replace"].encode("utf-8")
        if find == replace:
            raise ViewError(
                f"the derivation replaces {derivation['find']!r} with itself; an edit "
                "that changes no byte is not an edit"
            )
        occurrences = int(derivation.get("occurrences", 1))
        seen = raw.count(find)
        if seen != occurrences:
            raise ViewError(
                f"derivation expected {occurrences} occurrence(s) of {derivation['find']!r}, "
                f"found {seen}; the edit is not the single named change it claims to be"
            )
        return raw.replace(find, replace)
    raise ViewError(f"unknown derivation operation: {operation}")


def evidence_census() -> dict[str, str]:
    """Digest every retained byte, so read-only can be measured rather than asserted."""
    return {
        path.relative_to(ROOT).as_posix(): sha256(path.read_bytes())
        for path in sorted(EVIDENCE.rglob("*"))
        if path.is_file()
    }


def uncommitted_evidence_changes() -> list[str]:
    """Ask Git whether any retained byte differs from what was committed.

    The before/after census in this file proves only that *this process* did not
    write, which is a narrower claim than it sounds. Git history and pull-request
    review are the integrity boundary for retained bytes, so the boundary is
    consulted here rather than a second local manifest being invented to replace
    the anchor file this migration removed.

    A tree where Git is unavailable reports that fact instead of an empty list,
    because "nothing changed" and "nobody looked" must not be the same answer.
    """
    try:
        result = subprocess.run(
            ["git", "status", "--porcelain", "--", "evidence"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as error:
        raise ViewError(f"git could not be run to check retained bytes: {error}") from error
    if result.returncode != 0:
        raise ViewError(f"git refused to report on retained bytes: {result.stderr.strip()}")
    return [line for line in result.stdout.splitlines() if line.strip()]


def retained_envelopes() -> list[Path]:
    """Every retained envelope, current and historical. Nothing is skipped."""
    paths = sorted(EVIDENCE.rglob("evidence-envelope.json"))
    if not paths:
        raise ViewError("no retained evidence envelope was found; there is nothing to read")
    return paths


def states_in(view: dict[str, Any]) -> list[str]:
    return [
        mapping["value"]
        for mapping in view["mappings"]
        if mapping["target_field"] == "state" and isinstance(mapping["value"], str)
    ]


def case_bytes(case: dict[str, Any]) -> bytes:
    """Produce the bytes a declared case is about, re-deriving what it claims to derive."""
    if case.get("root") == "release":
        raw = pinned_source_bytes(case["file"])
    elif case.get("root") == "repository":
        raw = (ROOT / case["file"]).read_bytes()
    else:
        source = pinned_source_bytes(case["source"])
        raw = derive(source, case["derivation"])
        committed = FIXTURES / case["file"]
        if not committed.is_file():
            raise ViewError(f"{case['id']}: declared fixture {case['file']} is not committed")
        if committed.read_bytes() != raw:
            raise ViewError(
                f"{case['id']}: the committed fixture is not the declared derivation "
                "re-applied to the pinned source; a fixture that cannot be re-derived "
                "is a hand-written blob claiming to be one named change"
            )
    return raw


def run_case(mapper: Callable[..., dict[str, Any]], case: dict[str, Any]) -> dict[str, Any]:
    raw = case_bytes(case)
    expected_digest = None
    if "bind_digest" in case:
        expected_digest = case["bind_digest"]
        if expected_digest == "self":
            expected_digest = sha256(raw)
    view = mapper(raw, expected_digest=expected_digest)
    observed_states = states_in(view)
    reasons = [item["reason"] for item in view["unmapped_fields"]]
    # The reported identity is compared, not merely printed. A digest a view
    # states and nothing checks is decorative, and a view that can misreport
    # which bytes it read can attribute any answer to any record.
    identity_ok = view["source_digest"] == sha256(raw)
    matched = identity_ok and view["outcome"] == case["expected_outcome"]
    # Where the mapping distinguishes cases by a structured field, that field is
    # what is asserted. `map_pgm01_bytes` sets `source_record_id` to
    # `tampered-source` for a digest mismatch and `unreadable-source` for
    # undecodable bytes, so tampered and unreadable are told apart structurally
    # rather than by the wording of a reason.
    if matched and "expected_record_id" in case:
        matched = view["source_record_id"] == case["expected_record_id"]
    if matched and "expected_schema_version" in case:
        matched = view["source_schema_version"] == case["expected_schema_version"]
    # Two cases remain separable only by the upstream reason text: a malformed
    # field type and an unknown status both return `unreadable` under the real
    # record id. That is the mapping's only discriminator for them, and the
    # limitation is stated rather than papered over.
    if matched and "expected_reason_contains" in case:
        matched = any(case["expected_reason_contains"] in reason for reason in reasons)
    if matched:
        matched = set(case.get("requires_states", [])) <= set(observed_states)
    obligation = KIND_OBLIGATIONS.get(case["kind"])
    if obligation is None:
        label_supported = False
    else:
        label_supported = (
            ("outcome" not in obligation or view["outcome"] == obligation["outcome"])
            and ("state" not in obligation or obligation["state"] in observed_states)
            and (
                "record_id" not in obligation
                or view["source_record_id"] == obligation["record_id"]
            )
            and (
                "record_id_not" not in obligation
                or view["source_record_id"] not in obligation["record_id_not"]
            )
            and (
                "reason_contains" not in obligation
                or any(obligation["reason_contains"] in reason for reason in reasons)
            )
        )
    matched = matched and label_supported
    return {
        "id": case["id"],
        "kind": case["kind"],
        "expected_outcome": case["expected_outcome"],
        "outcome": view["outcome"],
        "source_digest": view["source_digest"],
        "source_identity_verified": identity_ok,
        "source_schema_version": view["source_schema_version"],
        "mapped_states": sorted(set(observed_states)),
        "unmapped_reasons": reasons,
        "matched": matched,
        "label_supported_by_observation": label_supported,
        "why": case["why"],
    }


def declared_schema_version(raw: bytes) -> str:
    """What a retained record says it is, or `unknown` when it does not say.

    The mapping's own answer is what decides the outcome. This is only the family
    label the census reports, and an absent or unreadable one is a fact about the
    record rather than a reason to stop.
    """
    try:
        declared = json.loads(raw).get("schemaVersion")
    except (json.JSONDecodeError, AttributeError):
        return "unknown"
    return declared if isinstance(declared, str) else "unknown"


def retained_report(mapper: Callable[..., dict[str, Any]]) -> dict[str, Any]:
    """Read every retained envelope in this repository through the pinned mapping."""
    entries = []
    for path in retained_envelopes():
        raw = path.read_bytes()
        view = mapper(raw)
        entries.append(
            {
                "path": path.relative_to(ROOT).as_posix(),
                "source_digest": view["source_digest"],
                "source_identity_verified": view["source_digest"] == sha256(raw),
                "declared_schema_version": declared_schema_version(raw),
                "outcome": view["outcome"],
                "reasons": [item["reason"] for item in view["unmapped_fields"]],
            }
        )
    outcomes = sorted({entry["outcome"] for entry in entries})
    families = sorted({entry["declared_schema_version"] for entry in entries})
    return {
        "count": len(entries),
        "declared_schema_versions": families,
        "outcomes": outcomes,
        "statement": (
            "This repository retained no quire.pgm01-evidence record. The pinned mapping "
            "covers quire.pgm01-evidence v1 and v2 and refuses every other schema version, "
            "so each retained envelope reads as 'incompatible'. That is the mapping declining "
            "to interpret a shape it has never seen. It is reported, not converted into a "
            "pass and not converted into a defect of these records. Filed upstream as "
            "agent-ix/engineering-assurance#21."
        ),
        "entries": entries,
    }


def census(mapper: Callable[..., dict[str, Any]]) -> dict[str, Any]:
    uncommitted = uncommitted_evidence_changes()
    before = evidence_census()
    declaration = json.loads(EXPECTATIONS.read_text(encoding="utf-8"))
    floor = declaration["retained_floor"]
    cases = [run_case(mapper, case) for case in declaration["cases"]]
    retained = retained_report(mapper)
    after = evidence_census()
    moved = sorted(path for path in set(before) | set(after) if before.get(path) != after.get(path))
    demonstrated_states = {state for case in cases for state in case["mapped_states"]}
    demonstrated_outcomes = {case["outcome"] for case in cases} | set(retained["outcomes"])
    missing_states = sorted(set(REQUIRED_STATES) - demonstrated_states)
    missing_outcomes = sorted(set(REQUIRED_OUTCOMES) - demonstrated_outcomes)
    unmatched = [case["id"] for case in cases if not case["matched"]]
    misattributed = [
        entry["path"] for entry in retained["entries"] if not entry["source_identity_verified"]
    ]
    # Retained evidence is immutable and may only grow, so a floor is the right
    # shape of check and an equality would be wrong. Without it this census
    # compared the tree to itself: `evidence_files_read` and the test's own walk
    # both counted whatever was on disk, so they agreed at any size, and the only
    # structural floor was `retained_envelopes()` raising on an empty glob. An
    # adversarial review showed 2,203 of 2,205 files and 42 of 44 envelopes could
    # be dropped in a commit with every gate still green. `git status` catches an
    # uncommitted deletion; it does not catch a committed one.
    below_floor = []
    if len(before) < floor["files"]:
        below_floor.append(f"{len(before)} evidence files, floor {floor['files']}")
    if len(retained["entries"]) < floor["envelopes"]:
        below_floor.append(
            f"{len(retained['entries'])} retained envelopes, floor {floor['envelopes']}"
        )
    return {
        "schemaVersion": "quire-contract-codegen.legacy-compatibility-census/v1",
        "mapping_authority": (
            "engineering_assurance.verification_semantics.map_pgm01_bytes from the pinned "
            "release. This repository implements no mapping of its own."
        ),
        "evidence_files_read": len(before),
        # Named for exactly what it measures. This process reads; the array is
        # what changed between its own before and after census, so an empty array
        # means this run wrote nothing. It is not, and does not claim to be, a
        # statement that the retained bytes match what was committed — that is
        # `uncommitted_evidence_changes` below, which asks Git.
        "evidence_bytes_moved_during_this_run": moved,
        "uncommitted_evidence_changes": uncommitted,
        "cases": cases,
        "retained": retained,
        "required_states": list(REQUIRED_STATES),
        "demonstrated_states": sorted(demonstrated_states),
        "undemonstrated_states": missing_states,
        "required_outcomes": list(REQUIRED_OUTCOMES),
        "demonstrated_outcomes": sorted(demonstrated_outcomes),
        "undemonstrated_outcomes": missing_outcomes,
        "unmatched_cases": unmatched,
        "misattributed_records": misattributed,
        "retained_floor": floor,
        "below_retained_floor": below_floor,
        "matched": (
            not unmatched
            and not moved
            and not uncommitted
            and not missing_states
            and not missing_outcomes
            and not misattributed
            and not below_floor
        ),
    }


def run_mutation_probes(mapper: Callable[..., dict[str, Any]]) -> dict[str, Any]:
    """Remove one load-bearing check at a time and require the census to go red.

    A gate that has never been observed to fail is indistinguishable from a gate
    that does not run. Each probe below degrades exactly one thing the census
    relies on; if the census still passes, the check it relies on is decorative.
    """

    def collapsing(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report every non-success state as a pass."""
        view = mapper(raw, **kwargs)
        for mapping in view["mappings"]:
            if mapping["target_field"] == "state" and mapping["value"] != "passed":
                mapping["value"] = "passed"
        return view

    def repairing(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report an unreadable record as readable."""
        view = mapper(raw, **kwargs)
        if view["outcome"] == "unreadable":
            view["outcome"] = "lossy"
        return view

    def accepting(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report a refused schema as an accepted one."""
        view = mapper(raw, **kwargs)
        if view["outcome"] == "incompatible":
            view["outcome"] = "lossy"
        return view

    def unbinding(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Ignore the caller's expected identity, so a tampered record reads normally."""
        kwargs.pop("expected_digest", None)
        return mapper(raw, **kwargs)

    def forgetting_identity(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report a source digest that is not the digest of the source."""
        view = mapper(raw, **kwargs)
        view["source_digest"] = "0" * 64
        return view

    def blurring_record_id(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report every record id as the real one, so tampered stops being structural."""
        view = mapper(raw, **kwargs)
        if view["source_record_id"] in ("tampered-source", "unreadable-source"):
            view["source_record_id"] = "some-real-record"
        return view

    def losing_states(raw: bytes, **kwargs: Any) -> dict[str, Any]:
        """Report no mapped states at all, so a kind that needs one cannot earn it."""
        view = mapper(raw, **kwargs)
        view["mappings"] = [
            mapping for mapping in view["mappings"] if mapping["target_field"] != "state"
        ]
        return view

    probes = {
        "collapse-non-success-states": collapsing,
        "repair-unreadable-outcome": repairing,
        "accept-refused-schema": accepting,
        "unbind-tamper-digest": unbinding,
        "drop-source-identity": forgetting_identity,
        "blur-structural-record-id": blurring_record_id,
        "erase-mapped-states": losing_states,
    }
    results = []
    for name, degraded in probes.items():
        # No exception handling. A probe that crashes has not demonstrated that
        # the census noticed anything — it has demonstrated that the probe is
        # broken — and counting a traceback as a detection is how "6/6 detected"
        # stops meaning anything.
        detected = not census(degraded)["matched"]
        results.append({"probe": name, "detected": detected})
    detected_count = sum(1 for item in results if item["detected"])
    return {
        "probes": results,
        "detected": detected_count,
        "total": len(results),
        "matched": detected_count == len(results),
    }


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="emit the census as JSON")
    parser.add_argument(
        "--mutation-probes",
        action="store_true",
        help="degrade one load-bearing check at a time and require the census to notice",
    )
    arguments = parser.parse_args(argv[1:])
    try:
        mapper = load_mapper()
        report = run_mutation_probes(mapper) if arguments.mutation_probes else census(mapper)
    except ViewError as error:
        print(str(error), file=sys.stderr)
        return 2
    if arguments.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    elif arguments.mutation_probes:
        for item in report["probes"]:
            print(f"{item['probe']}: {'detected' if item['detected'] else 'NOT DETECTED'}")
        print(f"mutation probes detected {report['detected']}/{report['total']}")
    else:
        for case in report["cases"]:
            flag = "ok" if case["matched"] else "MISMATCH"
            print(f"{case['id']}: {case['outcome']} ({flag})")
        retained = report["retained"]
        print(
            f"retained envelopes: {retained['count']} "
            f"{retained['declared_schema_versions']} -> {retained['outcomes']}"
        )
        print(
            f"retained floor: {report['retained_floor']['files']} files / "
            f"{report['retained_floor']['envelopes']} envelopes; below floor: "
            f"{report['below_retained_floor'] or 'no'}"
        )
        print(
            f"evidence files read: {report['evidence_files_read']}, "
            f"bytes moved this run: {len(report['evidence_bytes_moved_during_this_run'])}, "
            f"uncommitted: {len(report['uncommitted_evidence_changes'])}"
        )
        print(f"states demonstrated: {report['demonstrated_states']}")
    if not report["matched"]:
        message = (
            "a load-bearing check was removed and the census did not notice"
            if arguments.mutation_probes
            else "legacy compatibility census did not match"
        )
        print(message, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
