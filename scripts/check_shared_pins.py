#!/usr/bin/env python3
"""Observe the local toolchain and let Engineering Assurance classify it (FR-006-AC-1).

Four things this file deliberately is not.

It is not a copy of the compatibility matrix. It never says which version of
anything is correct. It observes what is installed and hands every verdict to
`engineering_assurance.compatibility`, because a second copy of the rule is a
second authority, and two authorities drift.

It is not an acceptance gate. The pinned release records
`accepted.state = pending_human_acceptance` and ships no
`human_acceptance_recorded` predicate (agent-ix/engineering-assurance#20). This
script reports the acceptance state the installed distribution carries and gates
only on things that are local and checkable. An absent field is not read as an
approval, and it is not read as a rejection either.

It is not a network probe. It does not ask a registry whether a release landed.
`npm.ix` in particular is a mirror that lags the public registry and is not an
oracle for anything; the only thing this script does about it is refuse to find
it written down anywhere in this repository.

It is not an envelope. It prints a report and exits. It retains nothing.

Both file scans are deliberately literal: they match a substring on any line,
comments included. A scan that learns to skip comments is a scan with a way to
hide an install line from it, and the comment syntax differs in every file it
reads. The cost is that the rule cannot be written down inside a file the scan
reads — `assurance/pins.json` is inspected structurally for exactly that reason,
and the workflow's own comment points at it rather than restating it.

Exit status: 0 when every component is compatible and no local check fails,
1 when something is not compatible, 2 when Engineering Assurance itself cannot
be loaded — which is a different fact from a failing check and gets its own code.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
PINS_PATH = ROOT / "assurance" / "pins.json"

FORBIDDEN_REGISTRY = "npm.ix"

# Files a mirror reference could realistically hide in. Read line by line rather
# than grepped as a blob, so that pins.json's own prose about the mirror does not
# match itself and report a violation that is actually the rule being written down.
MIRROR_SCAN_FILES = (
    "requirements-assurance.txt",
    ".npmrc",
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "package-lock.json",
    ".github/workflows/ci.yml",
    "Makefile",
)

# A version the accepted matrix names incompatible, written down somewhere this
# repository would actually install from. This is not a second matrix: the
# versions come out of the matrix itself at run time. What is local is only the
# list of places an install line can live, which the matrix has no view of.
INSTALL_SCAN_FILES = (
    ".github/workflows/ci.yml",
    "Makefile",
    "package.json",
    "requirements-assurance.txt",
)


class PinError(RuntimeError):
    """The pinned assurance distribution could not be used."""


def observe(argv: list[str]) -> str | None:
    """Run a version probe. An absent tool is None, which upstream calls unknown."""
    try:
        result = subprocess.run(argv, capture_output=True, text=True, check=False)
    except (OSError, ValueError):
        return None
    if result.returncode != 0:
        return None
    value = result.stdout.strip()
    return value or None


def observe_quire() -> str | None:
    """Read the CLI version from quire's own provenance record, not its banner."""
    raw = observe(["quire", "provenance"])
    if raw is None:
        return None
    try:
        return str(json.loads(raw)["cli"]["version"])
    except (json.JSONDecodeError, KeyError, TypeError):
        return None


def observe_engineering_assurance() -> str | None:
    from importlib.metadata import PackageNotFoundError, version

    try:
        return version("engineering-assurance")
    except PackageNotFoundError:
        return None


def artifact_digest_mismatches(pins: dict[str, Any]) -> list[str]:
    """Re-hash every artifact this repository reads out of the pinned release."""
    import hashlib

    import engineering_assurance

    package_root = Path(engineering_assurance.__file__).resolve().parent
    mismatches: list[str] = []
    for artifact in pins["consumed_artifacts"]:
        expected = artifact.get("sha256")
        if expected is None:
            continue
        path = package_root / artifact["path"]
        if not path.is_file():
            mismatches.append(f"{artifact['path']}: absent from the installed release")
            continue
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != expected:
            mismatches.append(f"{artifact['path']}: {actual}, pins record {expected}")
    return mismatches


def mirror_references(pins: dict[str, Any]) -> list[str]:
    """Find any place this repository would resolve a component from the mirror."""
    offenders: list[str] = []
    for name in MIRROR_SCAN_FILES:
        path = ROOT / name
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            if FORBIDDEN_REGISTRY in line:
                offenders.append(f"{name}:{number}")
    # pins.json is inspected structurally: its prose says the mirror's name on
    # purpose, and matching that would be the check reporting its own statement.
    requirement = pins["engineering_assurance"]["requirement"]
    if FORBIDDEN_REGISTRY in requirement:
        offenders.append("assurance/pins.json:engineering_assurance.requirement")
    for artifact in pins["consumed_artifacts"]:
        if FORBIDDEN_REGISTRY in artifact["path"]:
            offenders.append(f"assurance/pins.json:consumed_artifacts:{artifact['path']}")
    return offenders


def incompatible_install_references(matrix: dict[str, Any]) -> list[str]:
    """Find an install line pinning a version the matrix itself names incompatible.

    This exists because it happened. The hosted workflow in this repository
    pinned `@agent-ix/quoin@0.22.5`, which the accepted matrix names incompatible
    for a specific reason — it predates the `quoin change-assurance` surface
    entirely — and nothing noticed, because the workflow is manual-only and the
    local gates never read it. Three of the first five migrated repositories
    carried the same pin.

    The forbidden versions are read out of the matrix at run time. Nothing here
    decides which version is wrong.
    """
    offenders: list[str] = []
    for component in matrix["components"]:
        for version in component.get("incompatible", []):
            needles = (f"{component['name']}@{version}", f"@agent-ix/{component['name']}@{version}")
            for name in INSTALL_SCAN_FILES:
                path = ROOT / name
                if not path.is_file():
                    continue
                try:
                    text = path.read_text(encoding="utf-8")
                except (OSError, UnicodeDecodeError):
                    continue
                for number, line in enumerate(text.splitlines(), start=1):
                    if any(needle in line for needle in needles):
                        reason = component.get("incompatible_reasons", {}).get(version, "")
                        offenders.append(
                            f"{name}:{number}: {component['name']}@{version} is named "
                            f"incompatible by the matrix ({reason})"
                        )
    return offenders


def build_report() -> dict[str, Any]:
    try:
        from engineering_assurance.compatibility import accepted, classify_all, load_matrix
    except ImportError as error:  # pragma: no cover - exercised by the mutation probe
        raise PinError(f"the pinned assurance distribution is unusable: {error}") from error

    pins = json.loads(PINS_PATH.read_text(encoding="utf-8"))
    matrix = load_matrix()
    observed = {
        "quire-cli": observe_quire(),
        "quoin": observe(["quoin", "--version"]),
        "ix-flow": observe(["ix-flow", "--version"]),
        "engineering-assurance": observe_engineering_assurance(),
    }
    classifications = classify_all(matrix, observed)
    mismatches = artifact_digest_mismatches(pins)
    offenders = mirror_references(pins)
    installs = incompatible_install_references(matrix)
    versions_ok = accepted(classifications)
    acceptance = matrix["accepted"]
    return {
        "schemaVersion": "quire-contract-codegen.shared-pin-report/v1",
        "matrix_version": matrix["matrix_version"],
        "acceptance_state": acceptance["state"],
        "acceptance_recorded_here": False,
        "acceptance_authority": (
            "engineering_assurance/compatibility-matrix.json in the installed release. "
            "This repository reports it and is not a second acceptance authority."
        ),
        "versions_compatible": versions_ok,
        "artifact_mismatches": mismatches,
        "mirror_references": offenders,
        "incompatible_install_references": installs,
        "accepted": versions_ok and not mismatches and not offenders and not installs,
        "components": [
            {
                "component": item.component,
                "observed": item.observed,
                "expected": item.expected,
                "verdict": item.verdict,
                "reason": item.reason,
            }
            for item in classifications
        ],
    }


def main(argv: list[str]) -> int:
    as_json = argv[1:] == ["--json"]
    if argv[1:] and not as_json:
        print("usage: check_shared_pins.py [--json]", file=sys.stderr)
        return 2
    try:
        report = build_report()
    except PinError as error:
        print(str(error), file=sys.stderr)
        return 2
    if as_json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        for item in report["components"]:
            observed = item["observed"] if item["observed"] is not None else "not observed"
            print(f"{item['component']}: {observed} -> {item['verdict']} ({item['reason']})")
        for mismatch in report["artifact_mismatches"]:
            print(f"consumed artifact digest mismatch: {mismatch}", file=sys.stderr)
        for offender in report["mirror_references"]:
            print(f"mirror registry reference: {offender}", file=sys.stderr)
        for offender in report["incompatible_install_references"]:
            print(f"incompatible install pin: {offender}", file=sys.stderr)
        print(
            f"acceptance state recorded by the pinned release: {report['acceptance_state']} "
            "(reported, not gated on; see agent-ix/engineering-assurance#20)"
        )
        print(
            "shared pins accepted" if report["accepted"] else "shared pins NOT accepted",
            file=sys.stderr if not report["accepted"] else sys.stdout,
        )
    return 0 if report["accepted"] else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
