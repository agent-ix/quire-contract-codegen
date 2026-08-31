#!/usr/bin/env bash
set -euo pipefail

# Implements: MP-001

run_and_retain() {
  local name="$1"
  shift
  set +e
  "$@" >"$evidence_dir/$name.stdout" 2>"$evidence_dir/$name.stderr"
  local status=$?
  set -e
  echo "$status" >"$evidence_dir/$name.status.txt"
  if (( status != 0 )); then
    collection_failed=1
  fi
  return 0
}

record_status_word() {
  local numeric_status="$1"
  local word_status="$2"
  if [[ "$(<"$numeric_status")" == 0 ]]; then
    echo passed >"$word_status"
  else
    echo failed >"$word_status"
  fi
}

envelope_matches_sha256() {
  local expected="$1"
  [[ "$(sha256sum "$evidence_dir/evidence-envelope.json")" == "$expected" ]]
}

if [[ "${1:-}" == "--self-test" ]]; then
  self_test_dir="$(mktemp -d)"
  trap 'rm -rf -- "$self_test_dir"' EXIT
  evidence_dir="$self_test_dir"
  collection_failed=0

  run_and_retain passing true
  [[ "$(<"$evidence_dir/passing.status.txt")" == 0 ]]
  (( collection_failed == 0 ))

  run_and_retain failing false
  [[ "$(<"$evidence_dir/failing.status.txt")" != 0 ]]
  (( collection_failed == 1 ))

  record_status_word "$evidence_dir/passing.status.txt" "$evidence_dir/passing-word.txt"
  record_status_word "$evidence_dir/failing.status.txt" "$evidence_dir/failing-word.txt"
  [[ "$(<"$evidence_dir/passing-word.txt")" == passed ]]
  [[ "$(<"$evidence_dir/failing-word.txt")" == failed ]]

  echo stable >"$evidence_dir/evidence-envelope.json"
  self_test_sha256="$(sha256sum "$evidence_dir/evidence-envelope.json")"
  envelope_matches_sha256 "$self_test_sha256"
  echo changed >>"$evidence_dir/evidence-envelope.json"
  if envelope_matches_sha256 "$self_test_sha256"; then
    echo "collector fixed-point self-test accepted a changed envelope" >&2
    exit 1
  fi

  echo "foundation collector fail-closed self-test passed"
  exit 0
fi

if [[ $# -gt 0 ]]; then
  evidence_dir="$1"
else
  evidence_revision="$(git rev-parse --short=12 HEAD)"
  evidence_timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
  evidence_dir="evidence/foundation-${evidence_revision}-${evidence_timestamp}"
fi
if [[ -e "$evidence_dir" ]]; then
  echo "refusing to overwrite retained evidence: $evidence_dir" >&2
  exit 2
fi
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "refusing to collect evidence from a modified or untracked source tree" >&2
  exit 2
fi
source_state=clean
: "${PGM01_SCHEMA:?PGM01_SCHEMA must name the pinned PGM-01 envelope schema}"
: "${PGM01_VALIDATOR:?PGM01_VALIDATOR must name the pinned PGM-01 validator}"
pgm01_schema_path="$(realpath "$PGM01_SCHEMA")"
pgm01_validator_path="$(realpath "$PGM01_VALIDATOR")"
if [[ ! -f "$pgm01_schema_path" || ! -f "$pgm01_validator_path" ]]; then
  echo "PGM-01 schema and validator must both be regular files" >&2
  exit 2
fi
pgm01_repo="$(git -C "$(dirname "$pgm01_validator_path")" rev-parse --show-toplevel)"
if [[ -n "$(git -C "$pgm01_repo" status --porcelain --untracked-files=all)" ]]; then
  echo "refusing to use a modified or untracked PGM-01 validator checkout" >&2
  exit 2
fi
case "$pgm01_schema_path" in
  "$pgm01_repo"/*) ;;
  *) echo "PGM-01 schema and validator must come from the same checkout" >&2; exit 2 ;;
esac
if ! python3 -c 'import jsonschema' >/dev/null 2>&1; then
  echo "jsonschema is required for evidence collection" >&2
  exit 2
fi
mkdir -p "$evidence_dir"
collection_failed=0

git rev-parse HEAD >"$evidence_dir/source-revision.txt"
echo "$source_state" >"$evidence_dir/source-state.txt"
rustc --version --verbose >"$evidence_dir/rustc-version.txt"
rustc +1.75.0 --version --verbose >"$evidence_dir/msrv-rustc-version.txt"
cargo --version --verbose >"$evidence_dir/cargo-version.txt"
python3 --version >"$evidence_dir/python-version.txt"
python3 -c 'import jsonschema; print(jsonschema.__version__)' >"$evidence_dir/jsonschema-version.txt"
python3 -m pip freeze --all >"$evidence_dir/python-packages.txt"
echo "$pgm01_schema_path" >"$evidence_dir/pgm01-schema-path.txt"
sha256sum "$pgm01_schema_path" | cut -d' ' -f1 >"$evidence_dir/pgm01-schema-sha256.txt"
echo "$pgm01_validator_path" >"$evidence_dir/pgm01-validator-path.txt"
sha256sum "$pgm01_validator_path" | cut -d' ' -f1 >"$evidence_dir/pgm01-validator-sha256.txt"
git -C "$pgm01_repo" rev-parse HEAD >"$evidence_dir/pgm01-revision.txt"
quire provenance --pretty >"$evidence_dir/quire-provenance.json"
run_and_retain quire-validate \
  quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'
run_and_retain fmt cargo fmt --all -- --check
run_and_retain clippy cargo clippy --all-targets -- -D warnings
run_and_retain test cargo test
run_and_retain msrv cargo +1.75.0 check --lib
run_and_retain deny cargo deny check licenses
run_and_retain unsafe-audit bash scripts/check_unsafe_comments.sh
run_and_retain metadata cargo metadata --format-version 1
run_and_retain rustdoc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps
run_and_retain coverage quire coverage --scope .
run_and_retain evidence-tool make evidence-tool

run_schema_validators() {
  run_and_retain pgm01-pinned-schema \
    python3 scripts/validate_json_schema.py \
    schemas/pgm01-derivation-evidence-envelope-v1.schema.json \
    "$evidence_dir/evidence-envelope.json"
  run_and_retain input-schema \
    python3 scripts/validate_json_schema.py \
    schemas/foundation-evidence-input-v1.schema.json "$evidence_dir/collection-input.json"
  run_and_retain manifest-schema \
    python3 scripts/validate_json_schema.py \
    schemas/foundation-evidence-manifest-v1.schema.json "$evidence_dir/evidence-manifest.json"

  run_and_retain pgm01-schema \
    python3 scripts/validate_json_schema.py \
    "$pgm01_schema_path" "$evidence_dir/evidence-envelope.json"
  record_status_word \
    "$evidence_dir/pgm01-schema.status.txt" \
    "$evidence_dir/pgm01-schema-status.txt"

  run_and_retain pgm01-envelope \
    python3 "$pgm01_validator_path" --fixture "$evidence_dir/evidence-envelope.json"
  record_status_word \
    "$evidence_dir/pgm01-envelope.status.txt" \
    "$evidence_dir/pgm01-envelope-status.txt"
}

python3 scripts/build_foundation_envelope.py "$evidence_dir"
run_schema_validators
python3 scripts/build_foundation_envelope.py "$evidence_dir"
run_schema_validators
validated_envelope_sha256="$(sha256sum "$evidence_dir/evidence-envelope.json")"
python3 scripts/build_foundation_envelope.py "$evidence_dir"
if ! envelope_matches_sha256 "$validated_envelope_sha256"; then
  echo "foundation evidence envelope changed after final validation" >&2
  collection_failed=1
fi

(
  cd "$evidence_dir"
  find . -maxdepth 1 -type f ! -name sha256sums.txt -print0 \
    | sort -z \
    | xargs -0 sha256sum >sha256sums.txt
)

if (( collection_failed != 0 )); then
  echo "one or more retained foundation evidence commands failed" >&2
  exit 1
fi
