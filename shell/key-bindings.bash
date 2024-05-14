__crabmark_save__() {
  command=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  oh-hai -i "$command"
  READLINE_LINE=""
  READLINE_POINT=0x7fffffff
}

__crabmark_search__() {
  local output
  query=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  oh-hai -s "$query"
  output=$(cat ".command.txt");
  READLINE_LINE=${output}
  READLINE_POINT=0x7fffffff
}

bind -x '"\C-b":__crabmark_save__'
bind -x '"\C-g":__crabmark_search__'