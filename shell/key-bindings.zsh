function __oh_hai_save__() {
  local command
  command="${BUFFER}"
  oh-hai -i "$command"
  BUFFER=""
  CURSOR=0x7fffffff
  zle reset-promptf
}

function __oh_hai_search__() {
  local query res
  query="${BUFFER}"

  # Create a file descriptor to point to stdout
  exec 3>&1

  # Capture stderr in the variable 'res' while allowing stdout to pass through
  res=$(oh-hai -s "$query" 2>&1 1>&3)

  # Close the file descriptor
  exec 3>&-

  BUFFER="${res}"
  CURSOR=0x7fffffff
  zle reset-promptf
}

zle -N __oh_hai_save__
zle -N __oh_hai_search__

bindkey '^B' __oh_hai_save__
bindkey '^G' __oh_hai_search__
