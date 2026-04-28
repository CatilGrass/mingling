#!/usr/bin/env bash

# Save original directory, restore after execution
_load_original_dir="$PWD"

cd "$(dirname "$0")" 2>/dev/null || {
    echo "load.sh: failed to cd to script directory" >&2
    return 1
}

# If in zsh, source mling.zsh, otherwise source mling.sh
if [ -n "$ZSH_VERSION" ]; then
    [ -f "./.comp/mling_comp.zsh" ] && source "./.comp/mling_comp.zsh"
else
    [ -f "./.comp/mling_comp.sh" ] && source "./.comp/mling_comp.sh"
fi

# Add bin directories from all namespaces to PATH
for _dir in */bin/; do
    [ -d "$_dir" ] && export PATH="$PWD/${_dir%/}:$PATH"
done

# Helper function: execute script with appropriate shell
_load_script() {
    local script="$1"
    if [ -n "$ZSH_VERSION" ]; then
        case "$script" in
            *.zsh|*.sh)
                source "$script" 2>/dev/null
                ;;
        esac
    else
        case "$script" in
            *.sh)
                bash "$script" 2>/dev/null
                ;;
        esac
    fi
}

# Iterate over all namespaces
for _namespace in */; do
    _namespace="${_namespace%/}"
    [ "$_namespace" = "*" ] && continue

    # Skip if UNTRUSTED marker exists
    [ -f "$_namespace/UNTRUSTED" ] && continue

    _comp_dir="$_namespace/comp"
    [ ! -d "$_comp_dir" ] && continue

    # Find all loadable scripts under comp
    _scripts=$(find "$_comp_dir" -maxdepth 1 -type f \( -name '*.sh' -o -name '*.zsh' -o -name '*.fish' \) 2>/dev/null)
    [ -z "$_scripts" ] && continue

    # Count scripts
    _count=$(echo "$_scripts" | wc -l)

    # If TRUSTED marker exists, load directly
    if [ -f "$_namespace/TRUSTED" ]; then
        echo "$_scripts" | while IFS= read -r _script; do
            _load_script "$_script"
        done
        continue
    fi

    # No marker, ask user
    printf "'%s' has %d completion script(s) to load, do you trust it? [Y/n] " "$_namespace" "$_count"
    read _answer
    case "$_answer" in
        [Yy]*|"")
            # Mark as TRUSTED and set executable permissions
            echo "$_scripts" | while IFS= read -r _script; do
                chmod +x "$_script"
            done
            touch "$_namespace/TRUSTED"

            # Ask whether to load immediately
            printf "Load it immediately? [Y/n] "
            read _load_answer
            case "$_load_answer" in
                [Yy]*|"")
                    echo "$_scripts" | while IFS= read -r _script; do
                        _load_script "$_script"
                    done
                    ;;
            esac
            ;;
        *)
            touch "$_namespace/UNTRUSTED"
            ;;
    esac
done

# Restore original working directory
cd "$_load_original_dir" 2>/dev/null || true

# Cleanup
unset -f _load_script
unset _load_original_dir _dir _namespace _comp_dir _scripts _count _answer _load_answer _script
