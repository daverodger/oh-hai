__oh-hai_save__() {
	local command
	command=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
	oh-hai -i "$command"
	READLINE_LINE=""
	READLINE_POINT=0x7fffffff
}

__oh-hai_search__() {
	local query temp_stderr res
	query=$(echo "${READLINE_LINE:0}" | grep -oE '(.)+$')
	temp_stderr=$(mktemp /tmp/oh-hai_search.XXXXXX) || { echo "Failed to create temp file"; return 1; }
	oh-hai -s "$query" 2> "$temp_stderr"
	res=$(<"$temp_stderr")
	READLINE_LINE=${res}
	READLINE_POINT=0x7fffffff
	rm "$temp_stderr"
}

bind -x '"\C-b":__oh-hai_save__'
bind -x '"\C-g":__oh-hai_search__'
