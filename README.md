# repos

An opinionated way to manage local repositories.

By default, the base path of the project will be `$HOME/repos`.
So all the repositories will be cloned and managed from that. So, for example, the repository
`github.com/rust-lang/rust` will be located at `$HOME/repos/github.com/rust-lang/rust`.

The `expand` command will return the path to the repository locally. If the repository
does not exist locally, it will show you that and prompt to do the expand with the `--clone`
argument, which will clone the repository and after that, it will be able to expand it.

```console
$ repos expand rust-lang/rust
Repository not found locally.

$ repos expand --clone rust-lang/rust
Repository not found locally.
Local path: /Users/username/repos/github.com/rust-lang/rust
Git repository: git@github.com:rust-lang/rust
Do you want to clone it? (y, N)
y
Cloning into '/Users/username/repos/github.com/rust-lang/rust'...
remote: Enumerating objects: 2964616, done.
remote: Counting objects: 100% (856/856), done.
remote: Compressing objects: 100% (490/490), done.
remote: Total 2964616 (delta 593), reused 366 (delta 366), pack-reused 2963760 (from 2)
Receiving objects: 100% (2964616/2964616), 643.65 MiB | 15.51 MiB/s, done.
Resolving deltas: 100% (2302669/2302669), done.
Updating files: 100% (53047/53047), done.
/Users/username/repos/github.com/rust-lang/rust

$ repos expand rust-lang/rust
/Users/username/repos/github.com/rust-lang/rust
```

## Shortcut functions

- `rclone $1` - This is a shortcut for running `repos expand $1 --clone`.
- `rcd $1` - This is a shortcut for running `repos expand $1 --clone` and `cd` into the output.
- `rll $1` - This is a shortcut for running `repos list -f $1`, if `$1` is not empty or `repos list` if `$1` is empty.
- `red $1` - This is a shortcut for running `repos expand $1 --clone` and opening the expanded repository on the configured editor.

So, with those, some example workflows look like:

```bash
$ rll dns
└─ github.com
   ├─ mentimeter
   │  └─ (7) caddy-dns-linkup
   └─ hickory-dns
      └─ (43) hickory-dns

$ rcd @7
$ pwd
/Users/username/repos/github.com/mentimeter/caddy-dns-linkup 
```

```bash
$ rcd remkop/picocli
Repository not found locally.
Local path: /Users/username/repos/github.com/remkop/picocli
Git repository: git@github.com:remkop/picocli
Do you want to clone it? (y, N)
y
Cloning into '/Users/username/repos/github.com/remkop/picocli'...
remote: Enumerating objects: 52173, done.
remote: Counting objects: 100% (1023/1023), done.
remote: Compressing objects: 100% (431/431), done.
remote: Total 52173 (delta 756), reused 597 (delta 588), pack-reused 51150 (from 3)
Receiving objects: 100% (52173/52173), 83.95 MiB | 14.78 MiB/s, done.
Resolving deltas: 100% (32018/32018), done.
/Users/username/repos/github.com/remkop/picocli

$ pwd
/Users/username/repos/github.com/remkop/picocli
```

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



