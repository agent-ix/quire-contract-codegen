#!/usr/bin/env python3
"""Prove local-check recipes, tool identities, and policy gates are substantive."""

from __future__ import annotations

import argparse
import json
import os
import pwd
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GUARD_TARGET = "ci-guard"
CI_ORDER = (
    GUARD_TARGET,
    "fmt-check",
    "spec",
    "lint",
    "test",
    "msrv",
    "deny",
    "audit-unsafe",
    "rustdoc",
    "coverage",
    "evidence-tool",
    "verify-evidence",
)
CI_PROBES = set(CI_ORDER) - {GUARD_TARGET}
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):(?:\s+(.*?))?\s*$")
SHELL_CONTROL = re.compile(r"&&|\|\||[;|&]")
MAKEFLAGS_ASSIGNMENT = re.compile(r"^\s*MAKEFLAGS\s*(?::|\+|\?)?=\s*(.*)$")
MINIMUM_PYTHON_TESTS = 60


def trusted_home() -> Path:
    return Path(pwd.getpwuid(os.getuid()).pw_dir)


def expected_tools() -> dict[str, str]:
    home = trusted_home()
    return {
        "bash": "/usr/bin/bash",
        "cargo": str(home / ".cargo" / "bin" / "cargo"),
        "make": "/usr/bin/make",
        "python3": "/usr/bin/python3",
        "quire": str(home / ".npm-global" / "bin" / "quire"),
        "rustup": str(home / ".cargo" / "bin" / "rustup"),
    }


def parse_makefile(text: str) -> tuple[dict[str, list[str]], dict[str, list[str]]]:
    dependencies: dict[str, list[str]] = {}
    recipes: dict[str, list[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        if line.startswith("\t"):
            if current is not None:
                recipes.setdefault(current, []).append(line[1:])
            continue
        current = None
        if not line or line[0].isspace() or line.startswith("#"):
            continue
        match = TARGET.fullmatch(line)
        if match is None or match.group(1).startswith("."):
            continue
        current = match.group(1)
        dependencies[current] = (match.group(2) or "").split()
    return dependencies, recipes


def makeflags_errors(value: str) -> list[str]:
    """Allow only GNU Make parallelism/load flags; reject every execution modifier."""
    try:
        tokens = shlex.split(value)
    except ValueError:
        return ["MAKEFLAGS cannot be parsed safely"]
    errors: list[str] = []
    optional_value = False
    for token in tokens:
        if optional_value and re.fullmatch(r"[0-9]+(?:\.[0-9]+)?", token):
            optional_value = False
            continue
        optional_value = False
        if token in {"-j", "--jobs", "-l", "--load-average"}:
            optional_value = True
        elif re.fullmatch(r"-j[0-9]+", token):
            continue
        elif re.fullmatch(r"-l[0-9]+(?:\.[0-9]+)?", token):
            continue
        elif re.fullmatch(r"--(?:jobs|load-average)=[0-9]+(?:\.[0-9]+)?", token):
            continue
        elif re.fullmatch(r"--jobserver-(?:auth|fds)=.+", token):
            continue
        elif token:
            errors.append(f"unsafe MAKEFLAGS token: {token}")
    return errors


def command_parts(command: str) -> tuple[str, str]:
    stripped = command.lstrip()
    modifiers = ""
    while stripped[:1] in {"@", "+", "-"}:
        modifiers += stripped[0]
        stripped = stripped[1:].lstrip()
    return modifiers, stripped


def inspect_makefile(makefile: Path) -> list[str]:
    text = makefile.read_text(encoding="utf-8")
    dependencies, recipes = parse_makefile(text)
    errors: list[str] = []
    observed_order = dependencies.get("ci", [])
    if tuple(observed_order) != CI_ORDER:
        errors.append(f"ci prerequisite order/census drift: observed={observed_order}")
    for number, line in enumerate(text.splitlines(), start=1):
        if re.match(r"^\s*\.(?:IGNORE|SILENT)\s*(?::|$)", line):
            errors.append(
                f"Makefile:{number} declares a global recipe-control directive"
            )
        assignment = MAKEFLAGS_ASSIGNMENT.match(line)
        if assignment is not None:
            errors.extend(
                f"Makefile:{number} {error}"
                for error in makeflags_errors(assignment.group(1))
            )
    for target in sorted(set(CI_ORDER)):
        commands = recipes.get(target, [])
        if not commands:
            errors.append(f"mandatory target {target} has no recipe")
            continue
        for command in commands:
            modifiers, stripped = command_parts(command)
            if any(modifier in modifiers for modifier in "-+"):
                errors.append(
                    f"mandatory target {target} uses forbidden recipe modifier: {command}"
                )
            if SHELL_CONTROL.search(stripped):
                errors.append(
                    f"mandatory target {target} uses forbidden shell control operators: {command}"
                )
    return errors


def inspect_environment() -> list[str]:
    errors = makeflags_errors(os.environ.get("MAKEFLAGS", ""))
    if os.environ.get("MAKE"):
        errors.append("ambient MAKE override is not permitted")
    if os.environ.get("PYTHONOPTIMIZE") or sys.flags.optimize:
        errors.append("optimized Python disables policy execution")
    return errors


def inspect_toolchain() -> list[str]:
    expected = expected_tools()
    errors = [
        f"{name} must resolve to {path}, got {shutil.which(name)}"
        for name, path in expected.items()
        if shutil.which(name) != path
    ]
    version_commands = {
        "cargo": ([expected["cargo"], "--version"], r"^cargo \d+\.\d+\.\d+"),
        "python3": ([expected["python3"], "--version"], r"^Python 3\.\d+\.\d+"),
        "quire": ([expected["quire"], "--version"], r"^quire \d+\.\d+\.\d+"),
    }
    versions: dict[str, str] = {}
    for name, (command, pattern) in version_commands.items():
        try:
            completed = subprocess.run(
                command, check=False, capture_output=True, text=True
            )
        except OSError as error:
            errors.append(f"cannot execute {name}: {error}")
            continue
        output = (completed.stdout + completed.stderr).strip()
        versions[name] = output
        if completed.returncode != 0 or re.search(pattern, output) is None:
            errors.append(f"unexpected {name} identity: {output!r}")
    try:
        selected = subprocess.run(
            [expected["rustup"], "which", "--toolchain", "stable", "cargo"],
            check=False,
            capture_output=True,
            text=True,
        )
        selected_path = Path(selected.stdout.strip())
        selected_version = subprocess.run(
            [str(selected_path), "--version"],
            check=False,
            capture_output=True,
            text=True,
        )
    except OSError as error:
        errors.append(f"cannot cross-check rustup-selected cargo: {error}")
    else:
        if selected.returncode != 0 or not selected_path.is_file():
            errors.append(
                "rustup could not resolve the committed stable Cargo toolchain"
            )
        elif (
            selected_version.returncode != 0
            or selected_version.stdout.strip() != versions.get("cargo")
        ):
            errors.append(
                "Cargo proxy identity disagrees with rustup-selected stable Cargo"
            )
    return errors


def probe_command_positions(makefile: Path) -> list[str]:
    """Substitute false at every mandatory recipe position and require Make to fail."""
    _, recipes = parse_makefile(makefile.read_text(encoding="utf-8"))
    clean_env = dict(os.environ)
    clean_env.pop("MAKEFLAGS", None)
    clean_env.pop("MAKE", None)
    errors: list[str] = []
    with tempfile.TemporaryDirectory() as directory:
        probe = Path(directory) / "Makefile"
        for target in sorted(CI_PROBES):
            commands = recipes.get(target, [])
            for selected in range(len(commands)):
                lines = [f".PHONY: {target}", f"{target}:"]
                for index, command in enumerate(commands):
                    modifiers, _ = command_parts(command)
                    lines.append(
                        f"\t{modifiers}{'false' if index == selected else 'true'}"
                    )
                probe.write_text("\n".join(lines) + "\n", encoding="utf-8")
                result = subprocess.run(
                    ["/usr/bin/make", "--no-print-directory", "-f", str(probe), target],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                    env=clean_env,
                )
                if result.returncode == 0:
                    errors.append(
                        f"mandatory target {target} swallowed failure at recipe position "
                        f"{selected + 1}"
                    )
    return errors


def inspect_gate_outputs() -> list[str]:
    """Execute independent gate entry points and require substantive positive output."""
    commands = {
        "coverage": ["/usr/bin/python3", "scripts/check_coverage_status.py"],
        "evidence-tool": ["/usr/bin/python3", "scripts/run_python_tests.py"],
        "verify-evidence": [
            "/usr/bin/python3",
            "scripts/verify_foundation_evidence.py",
        ],
    }
    errors: list[str] = []
    for name, command in commands.items():
        completed = subprocess.run(
            command,
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        combined = completed.stdout + "\n" + completed.stderr
        if completed.returncode != 0:
            errors.append(f"{name} entry point failed its integrity probe")
            continue
        if name == "coverage":
            try:
                first_line = completed.stdout.splitlines()[0]
                report = json.loads(first_line)
            except (IndexError, json.JSONDecodeError):
                errors.append(
                    "coverage entry point emitted no parseable positive summary"
                )
            else:
                if report.get("statusLies") != 0 or not isinstance(
                    report.get("totals", {}).get("total"), int
                ):
                    errors.append(
                        "coverage entry point emitted an invalid positive summary"
                    )
        elif name == "evidence-tool":
            match = re.search(r"executed (\d+) Python tests from (\d+) files", combined)
            if (
                match is None
                or int(match.group(1)) < MINIMUM_PYTHON_TESTS
                or "\nOK\n" not in combined
            ):
                errors.append(
                    "Python test runner emitted no substantive success census"
                )
        elif (
            re.search(
                r"verified [1-9][0-9]* authoritative records, [1-9][0-9]* checksums, "
                r"[1-9][0-9]* manifest artifacts",
                combined,
            )
            is None
        ):
            errors.append("evidence verifier emitted no substantive success census")
    return errors


# Implements: MP-001
def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--makefile", type=Path, default=ROOT / "Makefile")
    parser.add_argument("--parse-time", action="store_true")
    parser.add_argument("--static-only", action="store_true")
    parser.add_argument("--inspect-only", action="store_true")
    args = parser.parse_args()
    errors = inspect_makefile(args.makefile)
    errors.extend(inspect_environment())
    if not args.static_only:
        errors.extend(inspect_toolchain())
    if not (args.parse_time or args.static_only or args.inspect_only) and not errors:
        errors.extend(probe_command_positions(args.makefile))
        errors.extend(inspect_gate_outputs())
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    print(f"all {len(CI_PROBES)} mandatory local-check targets propagate failures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
