#!/usr/bin/env bash
# Enforce that every `unsafe {` block in src/ has a `// SAFETY:` comment within
# the 3 lines preceding it. An occurrence written `"unsafe {` — every one on the
# line immediately preceded by a double quote — is a mention rather than a
# block, and is not audited. Pre-existing exemptions live in the baseline file
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
  text=${rest#*:}

  # An occurrence written as `"unsafe {` — the token immediately preceded by a
  # double quote — is a mention, not a block. This repository generates Rust and
  # asserts properties of the generated text, so `"unsafe {"` appears as data in
  # the very tests that forbid unsafe code in generated output, and a
  # `// SAFETY:` comment on a string literal would be a lie.
  #
  # The test is "is every occurrence on this line preceded by a quote", not "is
  # this text inside a string literal". Deciding the latter needs a Rust lexer:
  # a previous attempt deleted `"..."` spans and re-tested, which pairs quote
  # characters positionally and so silently swallowed real blocks after a char
  # literal `'"'` or a raw string `r#"x "y"#`. Quote parity does not rescue that
  # approach either: `let k = '"'; unsafe { f("z\"") };` has an even count and is
  # still swallowed. This condition is sound in the direction that matters,
  # because no valid Rust *expression* puts `"` immediately before an `unsafe`
  # block: an occurrence that is not quote-prefixed always reaches the audit, and
  # a line carrying both a mention and a real block is still audited. Token-tree
  # syntax does admit `m!("x"unsafe { .. })`, which this skips; no such macro
  # exists here, and a lexer is the only thing that would settle it.
  #
  # Only the exact spelling `"unsafe {` is treated as a mention. A mention with
  # a leading space, one mid-literal, or one in a multi-line expected-output
  # fixture is still audited and needs a baseline entry.
  probe=$text
  while [[ "$probe" =~ \"unsafe[[:space:]]*\{ ]]; do
    # Bind before substituting: any command inserted between the test and the
    # substitution would clobber BASH_REMATCH, the removal would be a no-op, and
    # the loop would hang rather than fail.
    mention=${BASH_REMATCH[0]}
    probe=${probe/"$mention"/}
  done
  if [[ ! "$probe" =~ unsafe[[:space:]]*\{ ]]; then
    continue
  fi

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
