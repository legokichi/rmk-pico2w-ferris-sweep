#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

apply_patch() {
  local repo="$1"
  local patch="$2"
  if git -C "$repo" apply --check "$patch" >/dev/null 2>&1; then
    git -C "$repo" apply "$patch"
    echo "Applied $(basename "$patch") in $repo"
  else
    echo "Skip $(basename "$patch") (already applied or not clean)"
  fi
}

apply_patch "$root" "$root/scripts/cleanup-debug-root.patch"
apply_patch "$root/rmk" "$root/scripts/cleanup-debug-rmk.patch"
