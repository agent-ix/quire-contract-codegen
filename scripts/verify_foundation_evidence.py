#!/usr/bin/env python3
"""Verify every authoritative retained codegen foundation evidence record."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator, FormatChecker


ROOT = Path(__file__).resolve().parent.parent
EVIDENCE_ROOT = ROOT / "evidence"
ENVELOPE_SCHEMA = ROOT / "schemas" / "pgm01-derivation-evidence-envelope-v1.schema.json"
MANIFEST_SCHEMA = ROOT / "schemas" / "foundation-evidence-manifest-v1.schema.json"
CHECKSUM_LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")


class EvidenceError(ValueError):
    """Raised when retained evidence is incomplete or inconsistent."""


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


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
    errors = sorted(
        Draft7Validator(schema, format_checker=FormatChecker()).iter_errors(instance),
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
    actual = {path for path in record.iterdir() if path.is_file() and path != checksum_path}
    if set(expected) != actual:
        missing = sorted(path.name for path in actual - set(expected))
        extra = sorted(path.name for path in set(expected) - actual)
        raise EvidenceError(
            f"checksum census mismatch in {record.name}: missing={missing}, extra={extra}"
        )
    for path, digest in expected.items():
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


# Implements: MP-001
def verify_record(record: Path) -> tuple[int, int]:
    checksums = verify_checksums(record)
    manifest = load_json(record / "evidence-manifest.json")
    envelope = load_json(record / "evidence-envelope.json")
    validate_json(manifest, MANIFEST_SCHEMA, f"{record.name} manifest")
    validate_json(envelope, ENVELOPE_SCHEMA, f"{record.name} envelope")
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
    retained_outcomes = {
        path.name.removesuffix(".status.txt") for path in record.glob("*.status.txt")
    }
    retained_outcomes.update(
        path.name.removesuffix("-status.txt")
        for path in record.glob("*-status.txt")
        if not path.name.endswith(".status.txt")
    )
    declared_outcomes = {item["name"] for item in manifest["outcomes"]}
    if retained_outcomes != declared_outcomes:
        raise EvidenceError(
            f"outcome census mismatch in {record.name}: "
            f"retained={sorted(retained_outcomes)}, declared={sorted(declared_outcomes)}"
        )
    return checksums, artifacts


def authoritative_records() -> list[Path]:
    return sorted(
        path
        for path in EVIDENCE_ROOT.glob("foundation-*")
        if path.is_dir() and (path / "evidence-envelope.json").is_file()
    )


def main() -> int:
    records = authoritative_records()
    if not records:
        print("no authoritative foundation evidence records found", file=sys.stderr)
        return 1
    try:
        totals = [verify_record(record) for record in records]
    except (EvidenceError, KeyError, OSError, TypeError) as error:
        print(f"foundation evidence verification failed: {error}", file=sys.stderr)
        return 1
    print(
        f"verified {len(records)} authoritative records, "
        f"{sum(item[0] for item in totals)} checksums, "
        f"{sum(item[1] for item in totals)} manifest artifacts"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
