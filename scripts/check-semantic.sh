#!/usr/bin/env bash
# Checks that each line on stdin is a Conventional Commits title: `type(scope): subject`, with the
# scope optional and a `!` before the colon for a breaking change: `feat(model)!: drop round 2`.
set -euo pipefail

pattern='^(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)(\([a-z0-9][a-z0-9._/-]*\))?!?: [^ ].*$'
status=0

while IFS= read -r title; do
  if [[ "$title" =~ $pattern ]]; then
    echo "ok:   $title"
  else
    echo "fail: $title"
    status=1
  fi
done

if [ "$status" -ne 0 ]; then
  echo
  echo "Each title must be 'type(scope): subject', for example 'fix(model): clamp the poll weights'."
  echo "Types: build, chore, ci, docs, feat, fix, perf, refactor, revert, style, test. The scope is optional."
fi
exit "$status"
