let oh_bindings = [
    {
            name: "oh-hai search"
            modifier: control
            keycode: char_g
            mode: [emacs vi_normal vi_insert]
            event: {
                    send: executehostcommand
                    cmd: "commandline | oh-hai -s $in e>| commandline edit -r $in"
            }
    }
    {
            name: "oh-hai input"
            modifier: control
            keycode: char_b
            mode: [emacs vi_normal vi_insert]
            event: {
                    send: executehostcommand
                    cmd: 'commandline | oh-hai -i $in e>| commandline edit -r $in'
            }
    }
]

$env.config.keybindings = ($env.config.keybindings | append [$oh_bindings])
