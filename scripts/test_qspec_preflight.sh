#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture_root=$(mktemp -d)
trap 'rm -rf "$fixture_root"' EXIT

expect_reject() {
  local label=$1
  shift
  local output status
  set +e
  output=$(env -u MAKEFLAGS -u QSPEC_DIR make --no-print-directory -s -C "$repo_root" "$@" 2>&1)
  status=$?
  set -e
  if (( status == 0 )) || [[ $output != *QSPEC_DIR* ]]; then
    printf 'qspec preflight selftest: %s did not reject with a QSPEC_DIR diagnostic (exit %s):\n%s\n' \
      "$label" "$status" "$output" >&2
    exit 1
  fi
  if [[ $output == *cargo* ]]; then
    printf 'qspec preflight selftest: %s reached Cargo before rejecting:\n%s\n' \
      "$label" "$output" >&2
    exit 1
  fi
}

expect_reject 'unset root' qspec-preflight
expect_reject 'empty root' qspec-preflight QSPEC_DIR=
expect_reject 'nonexistent root' qspec-preflight "QSPEC_DIR=$fixture_root/absent"
expect_reject 'test before Cargo' test QSPEC_DIR=
expect_reject 'ci before Cargo' ci QSPEC_DIR=
expect_reject 'ci -k before Cargo' -k ci QSPEC_DIR=

fixture_dir=$fixture_root/proposals/checked-package-v2/fixtures
mkdir -p "$fixture_dir"
fixtures=(positive-union-nodes positive-clause-operations positive-all-families)
for fixture in "${fixtures[@]}"; do
  : > "$fixture_dir/$fixture.json"
done

env -u MAKEFLAGS -u QSPEC_DIR make --no-print-directory -s -C "$repo_root" \
  qspec-preflight "QSPEC_DIR=$fixture_root"

for fixture in "${fixtures[@]}"; do
  rm "$fixture_dir/$fixture.json"
  expect_reject "missing $fixture" qspec-preflight "QSPEC_DIR=$fixture_root"
  : > "$fixture_dir/$fixture.json"
done

echo 'qspec preflight selftest passed'
