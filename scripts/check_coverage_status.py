#!/usr/bin/env python3
"""Gate coverage status truth without requiring implementation coverage yet."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
TRACE_ID = re.compile(r"\b(?:TC-\d{3}|(?:N?FR|StR)-\d{3}(?:-(?:AC|VC)-\d+)?)\b")


def ignored_trace_tests(repository_root: Path = ROOT) -> list[str]:
    findings = []
    for root_name in ("src", "tests", "benches", "examples"):
        root = repository_root / root_name
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.rs")):
            lines = path.read_text(encoding="utf-8").splitlines()
            for index, line in enumerate(lines):
                if "#[ignore" not in line:
                    continue
                context = "\n".join(lines)
                identifiers = sorted(set(TRACE_ID.findall(context)))
                if identifiers:
                    findings.append(
                        f"{path.relative_to(repository_root)}:{index + 1}: ignored trace-bearing test "
                        + ", ".join(identifiers)
                    )
    return findings


def undeclared_matrix_ids(report: dict[str, object], matrix_text: str) -> list[str]:
    minted = {
        item.get("id")
        for item in report.get("minted_targets", [])
        if isinstance(item, dict)
    }
    matrix_ids = set(TRACE_ID.findall(matrix_text))
    return sorted(
        identifier
        for identifier in matrix_ids
        if (identifier.startswith("TC-") or "-AC-" in identifier or "-VC-" in identifier)
        and identifier not in minted
    )


def diagnostic_reasons(report: dict[str, object]) -> list[str]:
    return sorted(
        {
            item.get("reason")
            for item in report.get("diagnostics", [])
            if isinstance(item, dict) and isinstance(item.get("reason"), str)
        }
    )


# Implements: MP-001
def main() -> int:
    ignored = ignored_trace_tests()
    if ignored:
        for finding in ignored:
            print(f"COVERAGE_STATUS_CONTRADICTION: {finding}", file=sys.stderr)
        return 1

    completed = subprocess.run(
        ["quire", "coverage", "--scope", ".", "--json"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    sys.stderr.write(completed.stderr)
    if completed.returncode != 0:
        sys.stdout.write(completed.stdout)
        return completed.returncode
    try:
        report = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        print(f"COVERAGE_STATUS_CONTRADICTION: invalid JSON report: {error}", file=sys.stderr)
        return 1
    status_lies = report.get("status_lies", [])
    if status_lies:
        print(
            f"COVERAGE_STATUS_CONTRADICTION: {len(status_lies)} contradicted status rows",
            file=sys.stderr,
        )
        return 1
    dangling = undeclared_matrix_ids(
        report,
        (ROOT / "spec/test-matrix.md").read_text(encoding="utf-8"),
    )
    if dangling:
        print(
            "COVERAGE_STATUS_CONTRADICTION: matrix references undeclared ids: "
            + ", ".join(dangling),
            file=sys.stderr,
        )
        return 1
    inconclusive = diagnostic_reasons(report)
    summary = report.get("totals", {})
    print(json.dumps({"statusLies": 0, "totals": summary}, sort_keys=True))
    if inconclusive:
        print(
            "COVERAGE_STATUS_INCONCLUSIVE: " + ", ".join(inconclusive),
            file=sys.stderr,
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
