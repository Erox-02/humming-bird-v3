#!/usr/bin/env bash
set -euo pipefail

# Only require git so `git checkout -- src Cargo.toml` works as an undo.
# Working tree is allowed to be dirty.
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "warning: not inside a git repo. proceed anyway? [y/N]" >&2
    read -r ans
    [[ "${ans,,}" == "y" ]] || exit 1
fi

# Order matters: longer / more specific names first.
declare -a PAIRS=(
    # --- struct / type names ---
    "SessionAwareGenerator:Sesawplcgen"
    "PlaceholderGenerator:Plcgen"
    "PlaceholderValidator:Plcval"
    "PlaceholderRestorer:Plcres"
    "ExtractorManager:Exman"
    "SessionManager:Sesman"

    # --- SessionManager methods ---
    "create_session:Crtses"
    "get_session:get_ses"
    "remove_session:rm_ses"
    "list_sessions:ls_ses"
    "session_count:ses_cnt"
    "process_with_session:prc_wses"
    "restore_with_session:res_wses"

    # --- ExtractorManager methods ---
    "add_config_extractor:add_conex"
    "add_config_from_json:add_conjson"
    "add_config_from_file:add_conf"
    "enable_extractor:enex"
    "disable_extractor:dis_ex"
    "is_enabled:ena"
    "list_extractors:ls_ex"
    "list_enabled:lsen"
    "extract_all:exall"
    "reset_to_defaults:rst_def"
    "extractor_count:excnt"
)

# Files to touch: src/**/*.{rs,toml,md} + root Cargo.toml + src/pyproject.toml
mapfile -t FILES < <(
    find src -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.md' \) -print
    [[ -f Cargo.toml ]]         && echo Cargo.toml
    [[ -f src/pyproject.toml ]] && echo src/pyproject.toml
)

echo "Renaming in ${#FILES[@]} file(s)..."
echo

for pair in "${PAIRS[@]}"; do
    old="${pair%%:*}"
    new="${pair##*:}"
    printf '  %-28s ->  %s\n' "$old" "$new"

    for f in "${FILES[@]}"; do
        # \b word boundaries so partial matches are impossible
        perl -i -pe "s/\b\Q${old}\E\b/${new}/g" "$f"
    done
done

echo
echo "Done."
echo "  git diff --stat   # review"
echo "  git checkout -- src Cargo.toml   # undo"
echo
echo "Leftover check:"
grep -rnE '\b(SessionManager|ExtractorManager|PlaceholderGenerator|PlaceholderValidator|PlaceholderRestorer|SessionAwareGenerator|create_session|get_session|remove_session|list_sessions|session_count|process_with_session|restore_with_session|add_config_extractor|add_config_from_json|add_config_from_file|enable_extractor|disable_extractor|is_enabled|list_extractors|list_enabled|extract_all|reset_to_defaults|extractor_count)\b' \
    src/ Cargo.toml 2>/dev/null || echo "  (no leftovers)"