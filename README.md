# vizshhist

Edit your `zsh_history` file with a text editor.

Bash's history file, `.bash_history`, is just a list of commands even if a command contains multi-byte strings, thus it is editable with a text editor. But Zsh's history file, `.zsh_history`, uses some magical encoding, called as metafy, for some non-ascii characters. This prevents us from editing the history file with a text editor. If you attempt to edit the history file with a text editor directly, your command history will be broken.

This tool decodes `zsh_history` file to a temporary file then launches the text editor. After closing the text editor, this tools automatically encodes and writes `zsh_history` file back.

## Install

## from pre-built binaries

Download a tar ball from [releases](https://github.com/ytoku/vizshhist/releases) page, and extract and copy the `vizshhist` file to a directory in `$PATH` such as `/usr/local/bin`.

- For x86-64 (amd64) GNU/Linux system: `vizshhist-*-x86_64-unknown-linux-musl.tar.gz`
- For aarch64 (arm64) GNU/Linux system: `vizshhist-*-aarch64-unknown-linux-musl.tar.gz`

## from source code

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

## Configuration

The configuration file is:

- GNU/Linux: `~/.config/vizshhist/config.toml`
- macOS: `~/Library/Application Support/vizshhist/config.toml`

You can specify a command to run your text editor in `editior` option.

```toml
[vizshhist]
editor = "/usr/bin/vim --cmd 'set fileencodings=utf-8'"
```

If `editor` option is not set in the configuration file, vizshhist checks `VISUAL` and `EDITOR` environment variable. Moreover if these environment variables are not set, vizshhist uses the first command found from the following list:  `/usr/bin/editor`, `/usr/bin/vi`, `/usr/bin/nano`.
