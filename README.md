# vizshhist

Edit your zsh_history file with a text editor.

Bash's history file, .bash_history, is just a list of commands even if a command contains multi-byte string, thus it is editable with a text editor. But Zsh's history file, .zsh_history, uses some magical encoding, called as metafy, for some non-ascii characters. This prevents us from editing the history file with a text editor. If you attempt to edit the history file with a text editor directly, your command history will be broken.

This tool decodes zsh_history file to a temporary file then launches the text editor. After closing the text editor, this tools automatically encodes and writes zsh_history file back.

## Install

```
cargo install --git https://github.com/ytoku/vizshhist.git
```

## Usage

To edit `~/.zsh_history`:

```
vizshhist
```

To edit a specific file:

```
vizshhist filename
```