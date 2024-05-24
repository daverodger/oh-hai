__oh_hai_save__() {
  command=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  oh-hai -i "$command"
  READLINE_LINE=""
  READLINE_POINT=0x7fffffff
}

__oh_hai_search__() {
  local output
  query=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  oh-hai -s "$query"

  # Get absolute path of this script
  SCRIPT_SOURCE="${BASH_SOURCE[0]}"
  SCRIPT_DIR="$( dirname "$( readlink -f "$SCRIPT_SOURCE" )" )"
  SCRIPT_DIR="${SCRIPT_DIR%/shell}"

  output=$(cat "$SCRIPT_DIR/data/.command.txt");
  READLINE_LINE=${output}
  READLINE_POINT=0x7fffffff
}

bind -x '"\C-b":__oh_hai_save__'
bind -x '"\C-g":__oh_hai_search__'