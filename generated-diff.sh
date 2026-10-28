#!/usr/bin/env bash
#
# Review the meaningful part of a code-generation diff.
#
# A Keycloak version bump rewrites, in every generated file, the doc link of
# every method:
#
#   -    /// Documentation: <https://www.keycloak.org/docs-api/26.7.3/rest-api/index.html#_...>
#   +    /// Documentation: <https://www.keycloak.org/docs-api/26.8.1/rest-api/index.html#_...>
#
# and the trailing counter of every generated file:
#
#   -// left 267
#   +// left 268
#
# That is ~800 changed lines of noise which hides the actual API changes, so ask
# git to ignore changed lines matching those two patterns (`-I` matches the line
# without its +/- marker).
#
# Usage:
#   ./generated-diff.sh                                # working tree, no context lines
#   ./generated-diff.sh --stat                         # summary per file
#   ./generated-diff.sh --with-spec                    # include api/openapi.json
#   ./generated-diff.sh HEAD                           # diff against a revision
#   ./generated-diff.sh -- src/types.rs src/resource   # limit to paths
#
# `git diff` cannot show files that generation created, so untracked files under
# src/ and api/ are listed at the end.
set -euo pipefail

cd "$(dirname "$0")"

# Recurring churn: per-method doc links and the "not all paths processed" counter.
ignore=(
    -I 'docs-api/[0-9][0-9.]*/rest-api'
    -I '^// left [0-9]+$'
)

mode=(-U0)
with_spec=false

while (($#)); do
    case "$1" in
        --stat) mode=(--stat) ;;
        --with-spec) with_spec=true ;;
        --) break ;;
        *) break ;;
    esac
    shift
done

# The spec dump is huge and better reviewed on its own (`git diff -- api/openapi.json`).
paths=()
$with_spec || paths+=(':(exclude)api/openapi.json')

# Pathspecs (after the user's `--`) have to follow the user's arguments.
if ((${#paths[@]})); then
    has_separator=false
    for arg in "$@"; do
        [[ $arg == -- ]] && has_separator=true
    done
    if $has_separator; then
        git diff "${mode[@]}" "${ignore[@]}" "$@" "${paths[@]}"
    else
        git diff "${mode[@]}" "${ignore[@]}" "$@" -- "${paths[@]}"
    fi
else
    git diff "${mode[@]}" "${ignore[@]}" "$@"
fi

untracked=$(git ls-files --others --exclude-standard -- src api)
if [[ -n $untracked ]]; then
    printf '\nNot shown above (created by generation, not tracked yet):\n%s\n' "$untracked" >&2
fi