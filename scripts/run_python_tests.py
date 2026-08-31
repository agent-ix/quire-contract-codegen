#!/usr/bin/env python3
"""Load and execute every Python unittest recursively without package assumptions."""

from __future__ import annotations

import sys
import types
import unittest
from pathlib import Path
from typing import TextIO

ROOT = Path(__file__).resolve().parent.parent


def discover_test_files(root: Path = ROOT) -> list[Path]:
    return sorted(
        path
        for path in (root / "tests").rglob("*.py")
        if "__pycache__" not in path.parts
    )


def load_test_suite(paths: list[Path]) -> unittest.TestSuite:
    """Load tests from exact paths, including nested non-package directories."""
    suite = unittest.TestSuite()
    loader = unittest.TestLoader()
    for index, path in enumerate(paths):
        name = f"quire_foundation_test_{index}_{path.stem}"
        module = types.ModuleType(name)
        module.__file__ = str(path)
        source = path.read_text(encoding="utf-8")
        exec(compile(source, str(path), "exec"), module.__dict__)
        suite.addTests(loader.loadTestsFromModule(module))
    return suite


def run_tests(root: Path = ROOT, stream: TextIO | None = None) -> int:
    """Execute the recursively discovered suite and return a gate exit status."""
    paths = discover_test_files(root)
    if not paths:
        print("no Python tests discovered", file=stream or sys.stderr)
        return 1
    try:
        suite = load_test_suite(paths)
    except (Exception, SystemExit) as error:
        print(f"cannot load Python tests: {error}", file=stream or sys.stderr)
        return 1
    result = unittest.TextTestRunner(stream=stream or sys.stderr).run(suite)
    if result.testsRun == 0:
        print("no Python tests executed", file=stream or sys.stderr)
        return 1
    if not result.wasSuccessful():
        return 1
    print(
        f"executed {result.testsRun} Python tests from {len(paths)} files",
        file=stream or sys.stdout,
    )
    return 0


# Implements: MP-001
def main() -> int:
    return run_tests()


if __name__ == "__main__":
    raise SystemExit(main())
