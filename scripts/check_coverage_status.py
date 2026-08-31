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
INCONCLUSIVE_REASONS = {"status-column-matches-nothing", "hollow-denominator"}


def ignored_trace_tests() -> list[str]:
    findings = []
    for root_name in ("src", "tests", "benches", "examples"):
        root = ROOT / root_name
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.rs")):
            lines = path.read_text(encoding="utf-8").splitlines()
            for index, line in enumerate(lines):
                if "#[ignore" not in line:
                    continue
                context = "\n".join(lines[max(0, index - 10) : index + 8])
                identifiers = sorted(set(TRACE_ID.findall(context)))
                if identifiers:
                    findings.append(
                        f"{path.relative_to(ROOT)}:{index + 1}: ignored trace-bearing test "
                        + ", ".join(identifiers)
                    )
    return findings


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
    inconclusive = sorted(
        {
            item.get("reason")
            for item in report.get("diagnostics", [])
            if item.get("reason") in INCONCLUSIVE_REASONS
        }
    )
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
