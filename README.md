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

### Config

As of now, some of the configuration file can be managed via the `repos config`. But not all of it.
For example, `home_path` can be set on the `$REPOS_PATH/.config.json`, but is not managed via the config
on the CLI (as of now).

For better details on the avaialble config, check the [config handling file](src/config.rs).

### Example workflows

#### List with filter and cd by index

```bash
> repos list -f dns
repos
└ github.com
  ├ lus
  │ └ (53) libdns-rs
  ├ libdns
  │ ├ (55) cloudflare
  │ └ (56) libdns
  ├ caddy-dns
  │ └ (61) cloudflare
  └ hickory-dns
    └ (68) hickory-dns

> rcd @61
> pwd
/repos/github.com/caddy-dns/cloudflare
```

### Install from source

```bash
cargo install --path .
```
