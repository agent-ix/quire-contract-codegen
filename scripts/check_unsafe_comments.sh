#!/usr/bin/env bash
# Enforce that every `unsafe {` block in src/ has a `// SAFETY:` comment within
# the 3 lines preceding it. Pre-existing exemptions live in the baseline file
# below; regenerate with `--update-baseline`.
# Implements: NFR-002
set -euo pipefail

baseline_file="scripts/unsafe_comment_baseline.txt"
update_baseline=false
trusted_grep=/usr/bin/grep
trusted_mktemp=/usr/bin/mktemp
trusted_rm=/usr/bin/rm
trusted_sed=/usr/bin/sed
trusted_sort=/usr/bin/sort

if [[ "${1:-}" == "--update-baseline" ]]; then
  update_baseline=true
fi

search_roots=()
for candidate in src tests benches examples; do
  if [[ -d "$candidate" ]]; then
    search_roots+=("$candidate")
  fi
done
if [[ ${#search_roots[@]} -eq 0 ]]; then
  echo "unsafe audit inconclusive: no Rust source roots are available" >&2
  exit 2
fi

grep_output="$("$trusted_mktemp")"
trap '"$trusted_rm" -f -- "$grep_output"' EXIT
set +e
"$trusted_grep" -rEn --include='*.rs' 'unsafe[[:space:]]*\{' "${search_roots[@]}" >"$grep_output"
grep_status=$?
set -e
if (( grep_status > 1 )); then
  echo "unsafe audit inconclusive: source scan failed with status ${grep_status}" >&2
  exit 2
fi
mapfile -t unsafe_lines <"$grep_output"

if [[ ${#unsafe_lines[@]} -eq 0 ]]; then
  echo "unsafe audit passed"
  exit 0
fi

missing_lines=()
missing=0
for entry in "${unsafe_lines[@]}"; do
  file=${entry%%:*}
  rest=${entry#*:}
  line=${rest%%:*}

  start=$(( line > 3 ? line - 3 : 1 ))
  if ! "$trusted_sed" -n "${start},${line}p" "$file" | "$trusted_grep" -q '// SAFETY:'; then
    missing_lines+=("${file}:${line}")
  fi
done

if [[ "$update_baseline" == true ]]; then
  if [[ ${#missing_lines[@]} -eq 0 ]]; then
    : > "$baseline_file"
    echo "wrote empty ${baseline_file}"
  else
    printf '%s\n' "${missing_lines[@]}" | "$trusted_sort" -u > "$baseline_file"
    echo "wrote ${baseline_file} with ${#missing_lines[@]} entries"
  fi
  exit 0
fi

if [[ ${#missing_lines[@]} -eq 0 ]]; then
  echo "unsafe audit passed"
  exit 0
fi

if [[ ! -f "$baseline_file" ]]; then
  echo "missing unsafe comment baseline: ${baseline_file}" >&2
  echo "run: bash scripts/check_unsafe_comments.sh --update-baseline" >&2
  exit 1
fi

for entry in "${missing_lines[@]}"; do
  if ! "$trusted_grep" -Fxq "$entry" "$baseline_file"; then
    echo "missing SAFETY comment near ${entry}" >&2
    missing=1
  fi
done

exit "$missing"
