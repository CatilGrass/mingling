#!/usr/bin/env bash
_<<<bin_name>>>_bash_completion() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local prev=""
    [ $COMP_CWORD -gt 0 ] && prev="${COMP_WORDS[COMP_CWORD-1]}"

    local word_index=$((COMP_CWORD + 1))

    local args=()
    args+=(-f="${COMP_LINE//-/^}")
    args+=(-C="$COMP_POINT")
    args+=(-w="${cur//-/^}")
    args+=(-p="${prev//-/^}")
    args+=(-c="${COMP_WORDS[0]//-/^}")
    args+=(-i="$word_index")
    args+=(-F="bash")

    for word in "${COMP_WORDS[@]}"; do
        args+=(-a="${word//-/^}")
    done

    local suggestions
    if suggestions=$(<<<bin_name>>> __comp "${args[@]}" 2>/dev/null); then
        if [ $? -eq 0 ]; then
            if [ "$suggestions" = "_file_" ]; then
                compopt -o default
                COMPREPLY=()
                return
            fi

            if [ -n "$suggestions" ]; then
                local -a all_suggestions filtered
                mapfile -t all_suggestions < <(printf '%s\n' "$suggestions")

                for suggestion in "${all_suggestions[@]}"; do
                    [ -z "$cur" ] || [[ "$suggestion" == "$cur"* ]] && filtered+=("$suggestion")
                done

                [ ${#filtered[@]} -gt 0 ] && COMPREPLY=("${filtered[@]}")
                return
            fi
        fi
    fi

    COMPREPLY=()
}

complete -F _<<<bin_name>>>_bash_completion <<<bin_name>>>
