function __oh_hai_save__() {
  local command
  command="${BUFFER}"
  oh-hai -i "$command"
  BUFFER=""
  CURSOR=0x7fffffff
}

function __oh_hai_search__() {
  local query res
  query="${BUFFER}"
  res=$(oh-hai -s "$query" 2>&1)
  BUFFER="${res}"
  CURSOR=0x7fffffff
}

zle -N __oh_hai_save__
zle -N __oh_hai_search__

bindkey '^B' __oh_hai_save__
bindkey '^G' __oh_hai_search__
