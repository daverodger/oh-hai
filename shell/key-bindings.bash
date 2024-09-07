__oh_hai_save__() {
  command=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  oh-hai -i "$command"
  READLINE_LINE=""
  READLINE_POINT=0x7fffffff
}

__oh_hai_search__() {
  local output
  query=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
  res=$(oh-hai -s "$query" >2)
  READLINE_LINE=${res}
  READLINE_POINT=0x7fffffff
}

bind -x '"\C-b":__oh_hai_save__'
bind -x '"\C-g":__oh_hai_search__'
