# repos

Make it easier to manage local repositories.

By default the base path of the project will be `~/repos`.
So all the repositories will be cloned and managed from that. So, for example, the repository
`github.com/rust-lang/rust` will be located at `~/repos/github.com/rust-lang/rust`.

The `expand` command will return the path to the repository locally. If the repository
does not exist locally, it will show you that and prompt to do the expand with the `--clone`
argument, which will clone the repository and after that, it will be able to expand it.

```console
$ repos expand rust-lang/rust
Repo not found locally in /Users/username/repos/github.com/rust-lang/rust.
Run with --clone if want to clone it.

$ repos expand rust-lang/rust --clone
Repo not found locally.
- Local path:	/Users/username/repos/github.com/rust-lang/rust
- Git repo:	git@github.com:rust-lang/rust.git
Do you want to clone it? (y/n - only 'y' continue)
y
Cloning repo...
/Users/username/repos/github.com/rust-lang/rust%

$ repos expand rust-lang/rust
/Users/username/repos/github.com/rust-lang/rust%
```

The argument to the `expand` command (and to the `add-alias`) can be one of the following
formats:
- `git@{host}:{username}/{repo}.git`
    - This format will be used as is.
- `{host}/{username}/{repo}`
    - This format will be used as is.
- `{username}/{repo}`
    - This format will have the {host} be resolved on the following order:
        1. What is on the `default_host` of the config.
        2. Default to "github.com".
- `{repo}`
    - This format will have the `host` be resolved on the following order:
        1. What is on the `default_host` of the config.
        2. Default to "github.com".
    - And the `username` resolved in the following order:
        1. What is on the `default_username` of the config.
        2. Default to `whoami::username()`.

You can add an alias to an expand so that it is easier to access

```console
$ repos config add-alias rust rust-lang/rust
Alias added:
  rust => /Users/username/repos/github.com/rust-lang/rust
```

So now when you run `expand` it will point to the alias

```console
$ repos expand rust
/Users/username/repos/github.com/rust-lang/rust%
```

For more details about things that are configurable, check `repos config --help`.

To conveniently be able to `cd` into the expanded repository, `repos` have an option
to `setup` a shell script so that you can automatically `cd` into the expanded repository.

After running the setup, you should be able to run something like `rcd rust-lang/rust`, which
is equivalent to `cd $(repos expand rust-lang/rust)`.

The changes of this action can be reversed with `repos cleanup`.

### Config
As of now, some of the configuration file can be managed via the `repos config`. But not all of it.
For example, `home_path` can be set on the `$REPOS_PATH/.config.json`, but is not managed via the config
on the CLI (as of now).

For better details on the avaialble config, check the [config handling file](src/config.rs).

### Install from source
```bash
cargo install --path .
```
