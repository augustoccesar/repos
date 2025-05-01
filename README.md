## Shortcut functions

- `rclone $1` - Repos clone
  This is a shortcut for running `repos expand $1 --clone`.
- `rcd $1` - Repos cd
  This is a shortcut for running `repos expand $1 --clone` and `cd` into the output.
- `rll $1` - Repos list
  This is a shortcut for running `repos list -f $1`, if `$1` is not empty or `repos list` if `$1` is empty.
- `red $1` - Repos edit
  This is a shortcut for running `repos expand $1 --clone` and opening the expanded repository on the configured editor.

## Installation

Add the binary path to `config.fish`.

```bash
fish_add_path $HOME/.repos/bin
```

To use the shortcut functions, also add the following:

```bash
repos activate fish | source
```

### From downloaded binary (not available yet)

```bash
mkdir -p $HOME/.repos/bin
mv <downloadpath>/repos $HOME/.repos/bin
```

### From source

```bash
# Gradle install compile native and move the result binary to $HOME/.repos/bin
gradle install
```



