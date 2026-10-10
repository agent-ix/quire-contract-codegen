#!/usr/bin/env bash
# Classify changed paths against NFR-006's real-Kani reachability set.
set -euo pipefail

touches_kani() {
    local path=$1
    case "$path" in
        src/kani/*|src/oracle/*|src/routed/*|src/replay/*|src/core/*|src/publication/*|\
        tests/it/kani_*.rs|tests/it/skeleton_spine.rs|tests/it/bounded_kani_corpus.rs|\
        tests/it/scratch_crate.rs|tests/exact_scalar_support/package.rs|\
        tests/checked_package_support/base.rs|tests/checked_package_support/rekey.rs|\
        tests/state_frame_support/*|\
        schemas/kani-*.schema.json|schemas/generated-rust-kani-*.schema.json|\
        Cargo.toml|Cargo.lock)
            return 0
            ;;
    esac
    return 1
}

base=$(git merge-base HEAD origin/main)
if [[ ${1-} == --main-freshness ]]; then
    # The candidate may omit a newer main commit. Only main changes reachable
    # by the Kani lane force a refresh; unrelated main movement does not.
    main_changed=$(git diff --no-renames --name-only "$base" origin/main --)
    main_reaching=()
    while IFS= read -r path; do
        if touches_kani "$path"; then
            main_reaching+=("$path")
        fi
    done <<< "$main_changed"
    if ((${#main_reaching[@]})) && ! git merge-base --is-ancestor origin/main HEAD; then
        printf 'stale\n'
        printf '%s\n' "${main_reaching[@]}"
        exit 1
    fi
    printf 'fresh\n'
    exit 0
fi
if (($#)); then
    printf 'usage: %s [--main-freshness]\n' "$0" >&2
    exit 2
fi

changed=$(git diff --no-renames --name-only "$base" --)
required=()
while IFS= read -r path; do
    if touches_kani "$path"; then
        required+=("$path")
    fi
done <<< "$changed"

if ((${#required[@]})); then
    printf 'required\n'
    printf '%s\n' "${required[@]}"
else
    printf 'not required\n'
fi
