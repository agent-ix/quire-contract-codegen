#!/usr/bin/env python3
"""Run every Python test file recursively without package-directory assumptions."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


def discover_test_files(root: Path = ROOT) -> list[Path]:
    return sorted(
        path
        for path in (root / "tests").rglob("*.py")
        if "__pycache__" not in path.parts
    )


# Implements: MP-001
def main() -> int:
    tests = discover_test_files()
    if not tests:
        print("no Python tests discovered", file=sys.stderr)
        return 1
    for path in tests:
        completed = subprocess.run([sys.executable, str(path)], cwd=ROOT, check=False)
        if completed.returncode != 0:
            return completed.returncode
    print(f"executed {len(tests)} Python test files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
