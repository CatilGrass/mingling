#!/usr/bin/env fish

# Save original directory
set -l _load_original_dir $PWD

# Switch to script directory
set -l _load_dir (dirname (status filename))
cd $_load_dir

# Load mling.fish from path
source .comp/mling_comp.fish

# Add all namespace bin directories to PATH
for _dir in */bin/
    if test -d $_dir
        set -gx PATH $PWD/$_dir $PATH
    end
end

function _load_comp_script
    if string match -q '*.fish' -- $argv[1]
        source $argv[1] 2>/dev/null
    end
end

# Iterate through all namespaces
for _namespace in */
    set _namespace (string trim -r -c / $_namespace)

    # Skip if UNTRUSTED marked or no comp directory
    test -f $_namespace/UNTRUSTED && continue
    test -d $_namespace/comp || continue

    # Find all loadable scripts in comp
    set _scripts (find $_namespace/comp -maxdepth 1 -type f \( -name '*.sh' -o -name '*.zsh' -o -name '*.fish' \) 2>/dev/null)
    test -z "$_scripts" && continue

    # Count scripts
    set _count (count $_scripts)

    # If TRUSTED marked, load directly
    if test -f $_namespace/TRUSTED
        for _script in $_scripts
            _load_comp_script $_script
        end
        continue
    end

    # Ask user
    read -l -p 'printf "%s has %d completion script(s) to load, do you trust it? [Y/n] " $_namespace $_count' _answer
    switch $_answer
        case '' Y y
            for _script in $_scripts
                chmod +x $_script
            end
            touch $_namespace/TRUSTED

            # Ask whether to load immediately
            read -l -p 'printf "Load it immediately? [Y/n] "' _load_answer
            switch $_load_answer
                case '' Y y
                    for _script in $_scripts
                        _load_comp_script $_script
                    end
            end
        case '*'
            touch $_namespace/UNTRUSTED
    end
end

# Restore original directory
cd $_load_original_dir

# Clean up
functions -e _load_comp_script
