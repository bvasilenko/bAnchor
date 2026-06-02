#!/usr/bin/env bash
set -euo pipefail

files=()
while IFS= read -r file; do
  case "$file" in
    .git/*|target/*|node_modules/*|transcripts/*|.github/docs/TODO.md|contribot.config.*.json|contribot.state.*.json|Cargo.lock)
      continue
      ;;
  esac
  if [[ -f "$file" ]]; then
    files+=("$file")
  fi
done < <(git ls-files --others --cached --exclude-standard)

if (( ${#files[@]} == 0 )); then
  exit 0
fi

expected_description="CLI mission-rail anchor for agentic loops. Reads task class; emits next-step directive."

require_fixed_text() {
  local file="$1"
  local text="$2"

  if ! grep -F -- "$text" "$file" >/dev/null; then
    printf '%s: missing required public text: %s\n' "$file" "$text" >&2
    exit 1
  fi
}

blocked_terms=(
  "pi""ll"
  "pi""lls"
  "Q5L"" R-"
  "projects/""b-suite/"
  "hold""ing/"
  "frame""works/"
  "B"":"
  "GOV""-"
  "DEC""ISION "
  "implementation""-open gate"
  "0.1.0""-skeleton"
  "PENDING""-OPENEVOLVE-RUN"
  "PENDING""-FIRST-CONTRIBOT-CYCLE"
  "SCOPE""-BNPM-"
  "DOMAIN""-"
  "IP""-TRANS-"
  "Co""-Authored-By:"
  $'\u2014'
)

for term in "${blocked_terms[@]}"; do
  if grep -R -n -F -- "$term" "${files[@]}" >/tmp/banchor_public_voice_guard_hits 2>/dev/null; then
    cat /tmp/banchor_public_voice_guard_hits >&2
    exit 1
  fi
done

require_fixed_text "README.md" "$expected_description"
require_fixed_text "crates/banchor/Cargo.toml" "description = \"$expected_description\""
