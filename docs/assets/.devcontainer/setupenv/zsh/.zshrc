export XDG_CONFIG_HOME="$HOME/.config"

# Set MANPATH correctly
MANPATH="/usr/share/man:/usr/local/share/man"
export MANPATH
export PYTHONPATH="$HOME/.rye/shims/python"

# Set other environment variables and paths
export BUN_INSTALL="$HOME/.bun"
PATH="$BUN_INSTALL/bin:$PATH"
PATH="$HOME/.local/bin:$PATH"
PATH="$HOME/.cargo/bin:$PATH"
PATH="$HOME/workspace/bin:$PATH"
PATH="$HOME/workspace/init.d:$PATH"
PATH="$HOME/.rye/shims:$PATH"
export PATH

# Aliases
alias node='"$BUN_INSTALL/bin/bun"'
alias z0="aichat --role bash1 --execute --code"

function aialias() {
    local options="h:r:ecag:f"
    local role=""
    local OPTIND
    while getopts $options opt; do
        case $opt in
            h) echo "Usage: aialias [-r ROLE] [ARGS...]"
               echo "Run aichat with a specified role"
               return 0
               ;;
            r) role="$OPTARG" ;;
            *) ;;
        esac
    done
    shift $((OPTIND-1))
    if [[ -z "$role" ]]; then
        role="bash1"
    fi
    aichat --role "$role" "$@"
}

function ab() {
    aialias -r bash1 "$@"
}

function z0() {
    aichat --role bash1 --execute --code "$@"
}

# History configuration
HISTFILE="$HOME/.zsh_history"
HISTSIZE=1000000
SAVEHIST=1000000
HISTORY_IGNORE="(ls|ls *|cd|cd *|pwd|exit|date|* --help|clear)"

# Set session-wise fixes
ulimit -n 4096
export OBJC_DISABLE_INITIALIZE_FORK_SAFETY=YES

# Initialize antidote
if [[ -f ${ZDOTDIR:-~}/.antidote/antidote.zsh ]]; then
    source "${ZDOTDIR:-$HOME}/.antidote/antidote.zsh"
fi

# Load plugins
antidote load ${ZDOTDIR:-$HOME}/.zsh_plugins.txt

# Initialize starship prompt
eval "$(starship init zsh)"
