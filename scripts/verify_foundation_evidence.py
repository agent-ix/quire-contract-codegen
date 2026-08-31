#!/usr/bin/env python3
"""Verify every authoritative retained codegen foundation evidence record."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator

SCRIPTS = Path(__file__).resolve().parent
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from build_foundation_envelope import (
    COMMAND_TRANSCRIPTS,
    command_outcomes,
    expected_manifest_artifact_names,
    foundation_limitations,
    hash_parameter_files,
    summarize_outcomes,
)
from validate_json_schema import checked_format_checker


ROOT = Path(__file__).resolve().parent.parent
EVIDENCE_ROOT = ROOT / "evidence"
ENVELOPE_SCHEMA = ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "foundation-evidence-manifest-v1.schema.json"
CHECKSUM_LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")
ANCHORS = EVIDENCE_ROOT / "ANCHORS"
REVISION = re.compile(r"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])")


class EvidenceError(ValueError):
    """Raised when retained evidence is incomplete or inconsistent."""


class VerificationUnavailable(EvidenceError):
    """Raised when the committed verification boundary is unavailable."""


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    """Hash a directory census and every regular file beneath it."""
    state = hashlib.sha256()
    for path in sorted(root.rglob("*"), key=lambda item: item.as_posix()):
        if path.is_symlink():
            raise EvidenceError(f"symlink is not allowed in retained evidence: {path}")
        relative = path.relative_to(root).as_posix()
        kind = b"d" if path.is_dir() else b"f"
        if not path.is_dir() and not path.is_file():
            raise EvidenceError(f"unsupported retained-evidence entry: {path}")
        state.update(kind)
        state.update(b"\0")
        state.update(relative.encode("utf-8"))
        state.update(b"\0")
        if path.is_file():
            state.update(bytes.fromhex(sha256_file(path)))
        state.update(b"\0")
    return state.hexdigest()


def safe_root_path(value: str) -> Path:
    relative = Path(value)
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise EvidenceError(f"unsafe evidence anchor path: {value!r}")
    path = ROOT / relative
    if not path.is_relative_to(EVIDENCE_ROOT):
        raise EvidenceError(f"anchor escapes evidence root: {value!r}")
    return path


def safe_record_path(record: Path, value: str) -> Path:
    relative = Path(value)
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise EvidenceError(f"unsafe retained-evidence path: {value!r}")
    resolved = record / relative
    if resolved.parent != record:
        raise EvidenceError(f"nested retained-evidence path is not allowed: {value!r}")
    return resolved


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise EvidenceError(f"cannot load {path}: {error}") from error
    if not isinstance(value, dict):
        raise EvidenceError(f"{path} must contain a JSON object")
    return value


def validate_json(instance: dict[str, Any], schema_path: Path, label: str) -> None:
    schema = load_json(schema_path)
    try:
        checker = checked_format_checker()
    except RuntimeError as error:
        raise VerificationUnavailable(str(error)) from error
    errors = sorted(
        Draft7Validator(schema, format_checker=checker).iter_errors(instance),
        key=lambda error: (list(error.absolute_path), error.message),
    )
    if errors:
        first = errors[0]
        location = "/".join(str(part) for part in first.absolute_path) or "<root>"
        raise EvidenceError(f"{label} schema violation at {location}: {first.message}")


def verify_checksums(record: Path) -> int:
    checksum_path = record / "sha256sums.txt"
    try:
        lines = checksum_path.read_text(encoding="utf-8").splitlines()
    except OSError as error:
        raise EvidenceError(f"cannot read {checksum_path}: {error}") from error
    expected: dict[Path, str] = {}
    for line in lines:
        match = CHECKSUM_LINE.fullmatch(line)
        if match is None:
            raise EvidenceError(f"invalid checksum line in {record.name}: {line!r}")
        path = safe_record_path(record, match.group(2))
        if path in expected:
            raise EvidenceError(f"duplicate checksum entry in {record.name}: {path.name}")
        expected[path] = match.group(1)
    actual = {path for path in record.iterdir() if path != checksum_path}
    if set(expected) != actual:
        unlisted = sorted(path.name for path in actual - set(expected))
        absent = sorted(path.name for path in set(expected) - actual)
        raise EvidenceError(
            f"checksum census mismatch in {record.name}: unlisted={unlisted}, absent={absent}"
        )
    for path, digest in expected.items():
        if path.is_symlink():
            raise EvidenceError(f"symlink is not allowed in retained evidence: {path}")
        observed = sha256_file(path)
        if observed != digest:
            raise EvidenceError(
                f"checksum mismatch in {record.name}/{path.name}: "
                f"expected {digest}, got {observed}"
            )
    return len(expected)


def verify_artifacts(record: Path, manifest: dict[str, Any]) -> int:
    seen: set[Path] = set()
    for artifact in manifest["artifacts"]:
        path = safe_record_path(record, artifact["path"])
        if path in seen:
            raise EvidenceError(f"duplicate manifest artifact in {record.name}: {path.name}")
        seen.add(path)
        if not path.is_file():
            raise EvidenceError(f"missing manifest artifact in {record.name}: {path.name}")
        if path.stat().st_size != artifact["size"]:
            raise EvidenceError(f"manifest size mismatch in {record.name}: {path.name}")
        if sha256_file(path) != artifact["sha256"]:
            raise EvidenceError(f"manifest digest mismatch in {record.name}: {path.name}")
    expected = expected_manifest_artifact_names(record)
    observed = {path.name for path in seen}
    if observed != expected:
        raise EvidenceError(
            f"manifest artifact census mismatch in {record.name}: "
            f"missing={sorted(expected - observed)}, extra={sorted(observed - expected)}"
        )
    return len(seen)


def verify_envelope_links(record: Path, envelope: dict[str, Any]) -> None:
    for artifact in [*envelope["inputs"], *envelope["outputs"]]:
        uri = artifact["uri"]
        if "://" in uri:
            continue
        path = safe_record_path(record, uri)
        if not path.is_file():
            raise EvidenceError(f"missing envelope artifact in {record.name}: {path.name}")
        expected = artifact["contentDigest"]["value"]
        if sha256_file(path) != expected:
            raise EvidenceError(f"envelope digest mismatch in {record.name}: {path.name}")


def verify_anchors() -> list[Path]:
    """Verify the committed record-set boundary and return authoritative records."""
    if not ANCHORS.is_file():
        raise VerificationUnavailable("committed evidence/ANCHORS is missing")
    expected: dict[Path, str] = {}
    for line in ANCHORS.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        match = CHECKSUM_LINE.fullmatch(line)
        if match is None:
            raise EvidenceError(f"invalid evidence anchor line: {line!r}")
        target = safe_root_path(match.group(2))
        if target in expected:
            raise EvidenceError(f"duplicate evidence anchor: {target.relative_to(ROOT)}")
        expected[target] = match.group(1)

    actual: set[Path] = set()
    for path in EVIDENCE_ROOT.iterdir():
        if path == ANCHORS:
            continue
        if (
            path.is_dir()
            and path.name.startswith("foundation-")
            and path.name != "foundation-remote"
            and (path / "evidence-envelope.json").is_file()
        ):
            actual.add(path / "sha256sums.txt")
        else:
            actual.add(path)
    if set(expected) != actual:
        unanchored = sorted(str(path.relative_to(ROOT)) for path in actual - set(expected))
        absent = sorted(str(path.relative_to(ROOT)) for path in set(expected) - actual)
        raise EvidenceError(
            f"evidence anchor census mismatch: unanchored={unanchored}, absent={absent}"
        )
    for path, digest in expected.items():
        if not path.exists():
            raise EvidenceError(f"anchored evidence target is absent: {path.relative_to(ROOT)}")
        observed = tree_digest(path) if path.is_dir() else sha256_file(path)
        if observed != digest:
            raise EvidenceError(
                f"evidence anchor mismatch for {path.relative_to(ROOT)}: "
                f"expected {digest}, got {observed}"
            )
    return sorted(
        path.parent
        for path in expected
        if path.name == "sha256sums.txt" and path.parent.name.startswith("foundation-")
    )


def verify_documented_revisions(records: list[Path]) -> None:
    record_revisions = {
        (record / "source-revision.txt").read_text(encoding="utf-8").strip()
        for record in records
    }
    documents = [
        EVIDENCE_ROOT / "README.md",
        *sorted((EVIDENCE_ROOT / "historical").rglob("README.md")),
    ]
    for document in documents:
        for revision in REVISION.findall(document.read_text(encoding="utf-8")):
            if revision in record_revisions:
                continue
            resolved = subprocess.run(
                ["git", "cat-file", "-e", f"{revision}^{{commit}}"],
                cwd=ROOT,
                check=False,
                capture_output=True,
            )
            if resolved.returncode != 0:
                label = (
                    str(document.relative_to(ROOT))
                    if document.is_relative_to(ROOT)
                    else str(document)
                )
                raise EvidenceError(
                    f"documented source revision does not exist in {label}: {revision}"
                )


def verify_historical_dispositions() -> None:
    expected = {
        "authoritative": False,
        "status": "retracted",
        "reason": "superseded historical foundation record",
    }
    for path in sorted((EVIDENCE_ROOT / "historical").rglob("evidence-envelope.json")):
        envelope = load_json(path)
        observed = envelope.get("extensions", {}).get(
            "dev.agent-ix.codegen", {}
        ).get("historicalDisposition")
        if observed != expected:
            raise EvidenceError(
                f"historical disposition missing or invalid in {path.relative_to(ROOT)}"
            )


# Implements: MP-001
def verify_record(record: Path) -> tuple[int, int]:
    checksums = verify_checksums(record)
    manifest = load_json(record / "evidence-manifest.json")
    envelope = load_json(record / "evidence-envelope.json")
    validate_json(manifest, MANIFEST_SCHEMA, f"{record.name} manifest")
    validate_json(envelope, ENVELOPE_SCHEMA, f"{record.name} envelope")
    recorded_schema_digest = (record / "pgm01-schema-sha256.txt").read_text(
        encoding="utf-8"
    ).strip()
    vendored_schema_digest = sha256_file(ENVELOPE_SCHEMA)
    if recorded_schema_digest != vendored_schema_digest:
        raise EvidenceError(
            f"PGM-01 schema anchor mismatch in {record.name}: "
            f"recorded {recorded_schema_digest}, vendored {vendored_schema_digest}"
        )
    artifacts = verify_artifacts(record, manifest)
    verify_envelope_links(record, envelope)
    revision = (record / "source-revision.txt").read_text(encoding="utf-8").strip()
    identities = {
        revision,
        manifest["sourceRevision"],
        envelope["producer"]["sourceRevision"],
        envelope["provenance"]["sourceRevision"],
    }
    if len(identities) != 1:
        raise EvidenceError(f"source revision mismatch in {record.name}: {sorted(identities)}")
    if envelope["parametersDigest"]["value"] != hash_parameter_files():
        raise EvidenceError(f"parameters digest mismatch in {record.name}")
    retained_outcomes = {
        path.name.removesuffix(".status.txt") for path in record.glob("*.status.txt")
    }
    retained_outcomes.update(
        path.name.removesuffix("-status.txt")
        for path in record.glob("*-status.txt")
        if not path.name.endswith(".status.txt")
    )
    declared_outcomes = {item["name"] for item in manifest["outcomes"]}
    configured_outcomes = {transcript for _, transcript in COMMAND_TRANSCRIPTS}
    if retained_outcomes != declared_outcomes or declared_outcomes != configured_outcomes:
        raise EvidenceError(
            f"outcome census mismatch in {record.name}: "
            f"retained={sorted(retained_outcomes)}, declared={sorted(declared_outcomes)}, "
            f"configured={sorted(configured_outcomes)}"
        )
    derived_outcomes = command_outcomes(record)
    if manifest["outcomes"] != derived_outcomes:
        raise EvidenceError(
            f"outcome value mismatch in {record.name}: "
            f"derived={derived_outcomes}, declared={manifest['outcomes']}"
        )
    result_status, result_summary, _ = summarize_outcomes(derived_outcomes)
    if envelope["result"]["status"] != result_status:
        raise EvidenceError(
            f"result status mismatch in {record.name}: "
            f"derived={result_status}, declared={envelope['result']['status']}"
        )
    if envelope["result"]["summary"] != result_summary:
        raise EvidenceError(f"result summary mismatch in {record.name}")
    if manifest["limitations"] != foundation_limitations(derived_outcomes):
        raise EvidenceError(f"manifest limitations mismatch in {record.name}")
    return checksums, artifacts


def verify_authoritative_records() -> list[tuple[int, int]]:
    records = verify_anchors()
    if not records:
        raise VerificationUnavailable(
            "no authoritative foundation records are named by evidence/ANCHORS"
        )
    verify_historical_dispositions()
    verify_documented_revisions(records)
    return [verify_record(record) for record in records]


def main() -> int:
    try:
        totals = verify_authoritative_records()
    except VerificationUnavailable as error:
        print(f"foundation evidence verification unavailable: {error}", file=sys.stderr)
        return 2
    except (EvidenceError, KeyError, OSError, TypeError) as error:
        print(f"foundation evidence verification failed: {error}", file=sys.stderr)
        return 1
    print(
        f"verified {len(totals)} authoritative records, "
        f"{sum(item[0] for item in totals)} checksums, "
        f"{sum(item[1] for item in totals)} manifest artifacts"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
