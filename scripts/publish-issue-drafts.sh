#!/usr/bin/env bash
set -euo pipefail

repo="petertzy/deepLocal"
drafts="$(dirname "$0")/issue-drafts.json"

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI is not authenticated. Run: gh auth login -h github.com" >&2
  exit 1
fi

if [[ "$(gh repo view "$repo" --json nameWithOwner --jq .nameWithOwner)" != "$repo" ]]; then
  echo "Unable to access $repo with the current GitHub account." >&2
  exit 1
fi

while IFS= read -r issue; do
  title="$(jq -r .title <<<"$issue")"
  body="$(jq -r .body <<<"$issue")"

  existing="$(gh issue list --repo "$repo" --state all --search "$title in:title" --json title,url --jq ".[] | select(.title == $(jq -Rn --arg v "$title" '$v')) | .url" | sed -n '1p')"
  if [[ -n "$existing" ]]; then
    printf 'Already exists, skipped: %s (%s)\n' "$title" "$existing"
    continue
  fi

  url="$(gh issue create --repo "$repo" --title "$title" --body "$body" --label "help wanted" --label "up-for-grabs")"
  printf 'Created: %s — %s\n' "$title" "$url"
done < <(jq -c '.[]' "$drafts")
