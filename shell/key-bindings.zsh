function __oh_hai_save__() {
  local command
  command="${BUFFER}"
  oh-hai -i "$command"
  BUFFER=""
  CURSOR=0x7fffffff
  zle reset-prompt
}

function __oh_hai_search__() {
  local query res
  query="${BUFFER}"

  # Use file descriptor to allow stdout passthrough while capturing stderr
  exec 3>&1
  res=$(oh-hai -s "$query" 2>&1 1>&3)
  exec 3>&-

  BUFFER="${res}"
  CURSOR=0x7fffffff
  zle reset-prompt
}

zle -N __oh_hai_save__
zle -N __oh_hai_search__

bindkey '^B' __oh_hai_save__
bindkey '^G' __oh_hai_search__
