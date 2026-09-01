#!/usr/bin/env python3
"""Gate coverage status truth without requiring implementation coverage yet."""

from __future__ import annotations

import argparse
import json
import os
import pwd
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TRACE_ID = re.compile(r"\b(?:TC-\d{3}|(?:N?FR|StR)-\d{3}(?:-(?:AC|VC)-\d+)?)\b")
IGNORE_ATTRIBUTE = re.compile(r"#!?\[[^\]]*\bignore\b[^\]]*\]", re.DOTALL)
CFG_ATTRIBUTE = re.compile(r"#!?\[[^\]]*\bcfg(?:_attr)?\b[^\]]*\]", re.DOTALL)
ITEM = re.compile(
    r"(?m)^(?P<indent>[ \t]*)(?:pub(?:\([^)]*\))?\s+)?"
    r"(?:(?:async|const|unsafe)\s+|extern(?:\s+\"[^\"]+\")?\s+)*"
    r"(?P<kind>fn|mod)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
EXPECTED_MATRIX_ROWS = (
    ("FR-001", "FR-001-AC-1, FR-001-AC-3", "TC-001", "🚧 Planned"),
    ("FR-001", "FR-001-AC-2", "TC-002", "🚧 Planned"),
    ("FR-001", "FR-001-AC-4", "TC-003", "🚧 Planned"),
    ("FR-002", "FR-002-AC-1 through FR-002-AC-4", "TC-004", "🚧 Planned"),
    ("FR-003", "FR-003-AC-1", "TC-005", "🚧 Planned"),
    ("FR-003", "FR-003-AC-2", "TC-007", "🚧 Planned"),
    ("FR-003", "FR-003-AC-3", "TC-003", "🚧 Planned"),
    ("FR-003", "FR-003-AC-4", "Inspection", "🚧 Planned"),
    ("FR-004", "FR-004-AC-1 through FR-004-AC-3", "TC-006", "🚧 Planned"),
    ("FR-004", "FR-004-AC-4", "Inspection", "🚧 Planned"),
    ("FR-005", "FR-005-AC-1", "TC-002", "🚧 Planned"),
    ("FR-005", "FR-005-AC-2", "TC-001", "🚧 Planned"),
    ("FR-005", "FR-005-AC-3", "Inspection", "🚧 Planned"),
    ("FR-005", "FR-005-AC-4", "TC-007", "🚧 Planned"),
    ("NFR-001", "NFR-001-AC-1", "TC-001", "🚧 Planned"),
    ("NFR-001", "NFR-001-AC-2, NFR-001-AC-3", "TC-002", "🚧 Planned"),
    ("NFR-002", "NFR-002-AC-1, NFR-002-AC-2", "TC-001", "🚧 Planned"),
    ("NFR-002", "NFR-002-AC-3", "TC-003", "🚧 Planned"),
    ("NFR-002", "NFR-002-AC-4", "Inspection", "🚧 Planned"),
    ("StR-001", "StR-001-VC-1, FR-001", "TC-001", "🚧 Planned"),
    ("StR-001", "StR-001-VC-2, FR-003, FR-004", "TC-007", "🚧 Planned"),
    (
        "TC-001",
        "Reproduce artifacts and manifests",
        "Integration",
        "P0",
        "FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2",
        "🚧 Planned",
    ),
    (
        "TC-002",
        "Compile and publish atomically",
        "Integration",
        "P0",
        "FR-001-AC-2, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3",
        "🚧 Planned",
    ),
    (
        "TC-003",
        "Reject unsupported inputs explicitly",
        "Integration",
        "P0",
        "FR-001-AC-4, FR-003-AC-3, NFR-002-AC-3",
        "🚧 Planned",
    ),
    (
        "TC-004",
        "Preserve shaped proptest strategies",
        "Property",
        "P0",
        "FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4",
        "🚧 Planned",
    ),
    (
        "TC-005",
        "Enforce Kani proof dependencies",
        "Analysis",
        "P0",
        "FR-003-AC-1",
        "🚧 Planned",
    ),
    (
        "TC-006",
        "Distinguish vacuity and unexecuted flow",
        "Integration",
        "P0",
        "FR-004-AC-1, FR-004-AC-2, FR-004-AC-3",
        "🚧 Planned",
    ),
    (
        "TC-007",
        "Verify cross-backend semantic parity",
        "Integration",
        "P0",
        "FR-003-AC-2, FR-005-AC-4",
        "🚧 Planned",
    ),
)


def rust_sources(repository_root: Path) -> list[Path]:
    return [
        path
        for path in sorted(repository_root.rglob("*.rs"))
        if path.relative_to(repository_root).parts[0]
        not in {".git", "evidence", "target"}
    ]


def trusted_quire() -> Path:
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    return home / ".npm-global" / "bin" / "quire"


def item_context(contents: str, attribute: re.Match[str], source: Path) -> str:
    """Return the complete Rust item controlled by an outer attribute."""
    if contents[attribute.start() :].startswith("#!["):
        return contents
    item = ITEM.search(contents, attribute.end())
    prior_closes = list(re.finditer(r"(?m)^[ \t]*}\s*$", contents[: attribute.start()]))
    context_start = prior_closes[-1].end() if prior_closes else 0
    if item is None:
        return contents[context_start:]
    declaration_end = contents.find("\n", item.end())
    declaration_end = len(contents) if declaration_end < 0 else declaration_end
    declaration = contents[item.start() : declaration_end]
    if item.group("kind") == "mod" and ";" in declaration:
        candidates = (
            source.with_name(f"{item.group('name')}.rs"),
            source.with_name(item.group("name")) / "mod.rs",
        )
        child = next((path for path in candidates if path.is_file()), None)
        child_text = child.read_text(encoding="utf-8") if child is not None else ""
        return contents[context_start:declaration_end] + "\n" + child_text
    opening = contents.find("{", item.end())
    if opening < 0:
        return contents[context_start:declaration_end]
    depth = 0
    for index in range(opening, len(contents)):
        if contents[index] == "{":
            depth += 1
        elif contents[index] == "}":
            depth -= 1
            if depth == 0:
                return contents[context_start : index + 1]
    return contents[context_start:]


def literal_cfg_test(attribute: str) -> bool:
    normalized = re.sub(r"\s+", "", attribute)
    return re.fullmatch(r"#\[cfg\(test\)\]", normalized) is not None


def trace_attribute_findings(
    pattern: re.Pattern[str], label: str, repository_root: Path = ROOT
) -> list[str]:
    findings = []
    for path in rust_sources(repository_root):
        contents = path.read_text(encoding="utf-8")
        for attribute in pattern.finditer(contents):
            attribute_text = attribute.group(0)
            if pattern is CFG_ATTRIBUTE and literal_cfg_test(attribute_text):
                continue
            context = item_context(contents, attribute, path)
            identifiers = sorted(set(TRACE_ID.findall(context)))
            if identifiers:
                line = contents.count("\n", 0, attribute.start()) + 1
                findings.append(
                    f"{path.relative_to(repository_root)}:{line}: {label} trace-bearing test "
                    + ", ".join(identifiers)
                )
    return findings


def ignored_trace_tests(repository_root: Path = ROOT) -> list[str]:
    return trace_attribute_findings(IGNORE_ATTRIBUTE, "ignored", repository_root)


def configured_trace_tests(repository_root: Path = ROOT) -> list[str]:
    return trace_attribute_findings(CFG_ATTRIBUTE, "cfg-controlled", repository_root)


def unowned_production_rust(repository_root: Path = ROOT) -> list[str]:
    candidates = [repository_root / "build.rs"]
    candidates.extend(sorted((repository_root / "src").rglob("*.rs")))
    return [
        str(path.relative_to(repository_root))
        for path in candidates
        if path.is_file()
        and not re.search(
            r"(?m)^// Implements: (?:N?FR|StR)-\d{3}$",
            path.read_text(encoding="utf-8"),
        )
    ]


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
        if (
            identifier.startswith("TC-") or "-AC-" in identifier or "-VC-" in identifier
        )
        and identifier not in minted
    )


def missing_matrix_ids(report: dict[str, object], matrix_text: str) -> list[str]:
    minted = {
        item.get("id")
        for item in report.get("minted_targets", [])
        if isinstance(item, dict)
        and isinstance(item.get("id"), str)
        and (
            item["id"].startswith("TC-") or "-AC-" in item["id"] or "-VC-" in item["id"]
        )
    }
    return sorted(minted - set(TRACE_ID.findall(matrix_text)))


def matrix_rows(matrix_text: str) -> tuple[tuple[str, ...], ...]:
    return tuple(
        tuple(cell.strip() for cell in line.strip("|").split("|"))
        for line in matrix_text.splitlines()
        if line.startswith(("| FR-", "| NFR-", "| StR-", "| TC-"))
    )


def matrix_row_errors(matrix_text: str) -> list[str]:
    observed = matrix_rows(matrix_text)
    if observed == EXPECTED_MATRIX_ROWS:
        return []
    expected_set = set(EXPECTED_MATRIX_ROWS)
    observed_set = set(observed)
    errors = []
    if any(not cell for row in observed for cell in row):
        errors.append("matrix contains an empty verification cell")
    if len(observed) != len(EXPECTED_MATRIX_ROWS):
        errors.append(
            f"matrix row census {len(observed)} differs from reviewed census "
            f"{len(EXPECTED_MATRIX_ROWS)}"
        )
    if observed_set != expected_set:
        errors.append(
            "matrix reviewed tuple set changed: "
            f"missing={len(expected_set - observed_set)}, extra={len(observed_set - expected_set)}"
        )
    elif observed != EXPECTED_MATRIX_ROWS:
        errors.append("matrix reviewed tuple order changed")
    return errors


def diagnostic_reasons(report: dict[str, object]) -> list[str]:
    return sorted(
        {
            item.get("reason")
            for item in report.get("diagnostics", [])
            if isinstance(item, dict) and isinstance(item.get("reason"), str)
        }
    )


# Implements: MP-001
def coverage_gate(
    repository_root: Path,
    runner=None,
) -> int:
    if runner is None:
        runner = subprocess.run
    unowned = unowned_production_rust(repository_root)
    if unowned:
        print(
            "COVERAGE_STATUS_CONTRADICTION: production Rust lacks an Implements marker: "
            + ", ".join(unowned),
            file=sys.stderr,
        )
        return 1
    inactive = ignored_trace_tests(repository_root) + configured_trace_tests(
        repository_root
    )
    if inactive:
        for finding in inactive:
            print(f"COVERAGE_STATUS_CONTRADICTION: {finding}", file=sys.stderr)
        return 1

    completed = runner(
        [str(trusted_quire()), "coverage", "--scope", ".", "--json"],
        cwd=repository_root,
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
        print(
            f"COVERAGE_STATUS_CONTRADICTION: invalid JSON report: {error}",
            file=sys.stderr,
        )
        return 1
    status_lies = report.get("status_lies", [])
    if status_lies:
        print(
            f"COVERAGE_STATUS_CONTRADICTION: {len(status_lies)} contradicted status rows",
            file=sys.stderr,
        )
        return 1
    matrix_text = (repository_root / "spec/test-matrix.md").read_text(encoding="utf-8")
    dangling = undeclared_matrix_ids(report, matrix_text)
    if dangling:
        print(
            "COVERAGE_STATUS_CONTRADICTION: matrix references undeclared ids: "
            + ", ".join(dangling),
            file=sys.stderr,
        )
        return 1
    missing = missing_matrix_ids(report, matrix_text)
    if missing:
        print(
            "COVERAGE_STATUS_CONTRADICTION: minted ids absent from matrix: "
            + ", ".join(missing),
            file=sys.stderr,
        )
        return 1
    matrix_errors = matrix_row_errors(matrix_text)
    if matrix_errors:
        for error in matrix_errors:
            print(f"COVERAGE_STATUS_CONTRADICTION: {error}", file=sys.stderr)
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


def behavioral_self_test() -> int:
    """Mutation-test every interior gate path without invoking external tooling."""
    matrix = "\n".join("| " + " | ".join(row) + " |" for row in EXPECTED_MATRIX_ROWS)
    minted = sorted(set(TRACE_ID.findall(matrix)))

    def rejected(
        label: str,
        *,
        matrix_text: str = matrix,
        report: dict[str, object] | None = None,
        source_path: str | None = None,
        source_text: str = "",
    ) -> bool:
        selected_report = report or {
            "minted_targets": [{"id": identifier} for identifier in minted],
            "diagnostics": [],
            "status_lies": [],
            "totals": {"total": len(minted)},
        }
        completed = subprocess.CompletedProcess(
            [str(trusted_quire())], 0, json.dumps(selected_report), ""
        )

        def fake_runner(*_args, **_kwargs):
            return completed

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "spec").mkdir()
            (root / "spec" / "test-matrix.md").write_text(
                matrix_text, encoding="utf-8"
            )
            if source_path is not None:
                source = root / source_path
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text(source_text, encoding="utf-8")
            if coverage_gate(root, fake_runner) != 0:
                return True
        print(f"coverage behavioral self-test accepted {label} mutation", file=sys.stderr)
        return False

    base_report: dict[str, object] = {
        "minted_targets": [{"id": identifier} for identifier in minted],
        "diagnostics": [],
        "status_lies": [],
        "totals": {"total": len(minted)},
    }
    status_lie_report = dict(base_report)
    status_lie_report["status_lies"] = [{"id": "TC-001"}]
    undeclared_report = dict(base_report)
    undeclared_report["minted_targets"] = [
        {"id": identifier} for identifier in minted if identifier != "TC-001"
    ]
    missing_report = dict(base_report)
    missing_report["minted_targets"] = [
        *base_report["minted_targets"],
        {"id": "TC-999"},
    ]
    checks = (
        rejected(
            "unowned production",
            source_path="src/lib.rs",
            source_text="pub fn unowned() {}\n",
        ),
        rejected(
            "ignored trace",
            source_path="tests/ignored.rs",
            source_text="// Verifies: TC-001\n#[ignore]\n#[test]\nfn hidden() {}\n",
        ),
        rejected(
            "cfg-controlled trace",
            source_path="tests/configured.rs",
            source_text="// Verifies: TC-001\n#[cfg(any())]\n#[test]\nfn hidden() {}\n",
        ),
        rejected("status lie", report=status_lie_report),
        rejected("matrix id absent from report", report=undeclared_report),
        rejected("report id absent from matrix", report=missing_report),
        rejected(
            "completed matrix status",
            matrix_text=matrix.replace("🚧 Planned", "✅ Complete", 1),
        ),
    )
    if not all(checks):
        return 1
    print("coverage behavioral self-test passed")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    return behavioral_self_test() if args.self_test else coverage_gate(ROOT)


if __name__ == "__main__":
    raise SystemExit(main())
