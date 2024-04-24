__crabmark_save__() {
  command=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  ./target/debug/crabmark -i "$command"
  READLINE_LINE=""
  READLINE_POINT=0x7fffffff
}

__crabmark_search__() {
  local output
  query=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  output=$(./target/debug/crabmark -s "$query")
  READLINE_LINE=${output}
  READLINE_POINT=0x7fffffff
}

bind -x '"\C-b":__crabmark_save__'
bind -x '"\C-g":__crabmark_search__'