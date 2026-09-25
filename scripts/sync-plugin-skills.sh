#!/usr/bin/env bash
# Mirror the top-level skills/ into plugins/rokha/skills/.
# An Agent Plugin's files must resolve inside the plugin root (symlinks that
# escape it are rejected), so the plugin carries a copy. Run after any skill
# edit; commit both sides together.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
rsync -a --delete --exclude README.md "$ROOT/skills/" "$ROOT/plugins/rokha/skills/"
echo "synced skills/ -> plugins/rokha/skills/"
