#!/usr/bin/env bash
# Run and verify the complete real-Kani lane on a stable, clean candidate.
set -euo pipefail

if [[ ${1-} == --held ]]; then
    shift
    held=true
else
    held=false
fi

if (($# < 4)) || [[ ${2-} != -- ]]; then
    printf 'usage: %s LOCK -- CARGO +TOOLCHAIN test ...\n' "$0" >&2
    exit 2
fi
lock=$1
shift 2
test_command=("$@")

if [[ $held == false ]]; then
    exec flock -x "$lock" /usr/bin/bash "$0" --held "$lock" -- "${test_command[@]}"
fi

result() {
    local status=$1 ran=$2 expected=$3 version=$4 tree=$5
    printf 'kani-gate: result=%s ran=%s expected=%s elapsed=%ss kani=%s tree=%s\n' \
        "$status" "$ran" "$expected" "$SECONDS" "$version" "$tree"
}

if ! command -v cargo-kani >/dev/null 2>&1; then
    printf 'kani-gate: not run: launcher absent\n'
    exit 1
fi
if ! version=$(cargo-kani --version 2>/dev/null); then
    result failed 0 0 unavailable clean
    exit 1
fi
version=${version%%$'\n'*}
if [[ -z ${version//[[:space:]]/} ]]; then
    result failed 0 0 unavailable clean
    exit 1
fi

before_head=$(git rev-parse HEAD)
before_tree=$(git status --porcelain --untracked-files=all)
if [[ -n $before_tree ]]; then
    result failed 0 0 "$version" dirty
    exit 1
fi
if ! freshness=$(/usr/bin/bash scripts/kani_scope.sh --main-freshness); then
    printf 'kani-gate: source stale against origin/main (%s)\n' "${freshness//$'\n'/, }" >&2
    result failed 0 0 "$version" clean
    exit 1
fi

# Replace libtest's execution flags with its list flag; keep the exact six
# filters and the same cargo target. The list invocation executes no tests.
delimiter=-1
for index in "${!test_command[@]}"; do
    if [[ ${test_command[index]} == -- ]]; then
        delimiter=$index
        break
    fi
done
if ((delimiter < 0)) || [[ ${test_command[delimiter+1]-} != --ignored ]] ||
    [[ ${test_command[delimiter+2]-} != --test-threads=1 ]]; then
    result failed 0 0 "$version" clean
    exit 1
fi
list_command=("${test_command[@]:0:delimiter+1}" --list --ignored "${test_command[@]:delimiter+3}")

list_log=$(mktemp)
run_log=$(mktemp)
trap 'rm -f "$list_log" "$run_log"' EXIT
if ! "${list_command[@]}" >"$list_log" 2>&1; then
    cat "$list_log" >&2
    result failed 0 0 "$version" clean
    exit 1
fi
expected=$(awk '/: test$/ { n++ } END { print n+0 }' "$list_log")
if ((expected == 0)); then
    result failed 0 0 "$version" clean
    exit 1
fi

run_ok=false
if "${test_command[@]}" 2>&1 | tee "$run_log"; then
    run_ok=true
fi
ran=$(awk '/^test result: / { for (i=2; i<=NF; i++) if ($i == "passed;") print $(i-1) }' "$run_log")
after_head=$(git rev-parse HEAD)
after_tree=$(git status --porcelain --untracked-files=all)
tree=clean
if [[ -n $after_tree ]]; then
    tree=dirty
fi
if [[ $run_ok == true && $ran =~ ^[0-9]+$ && $ran -eq $expected &&
      $before_head == "$after_head" && $tree == clean ]]; then
    result passed "$ran" "$expected" "$version" "$tree"
    exit 0
fi
result failed "${ran:-0}" "$expected" "$version" "$tree"
exit 1
