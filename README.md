# Vemo

Vemo stands for **Ve**rsioning of **mo**norepo. It is a simple cli tool written in Rust 🦀 for bumping services/applications inside your monorepo.

## 🧩 Usage

> **Warning**
> 
> Add documentation here!

## 📋 Configuration

`vemo` can be configured using `.vemo.toml` file in the main directory of your repo and by env variables.

### `.vemo.toml`

If the `.vemo.toml` file does not exist in the repo, default values will be used. To see what are the values, have a look at the comments in the file below.

```toml
# Based on that format the git tag will be created. It is also used to fetch all the applications from the monorepo.
# Default: "{app_name}/v{version}"
format = "{app_name}/v{version}"

# If true, the debug messages will be printed.
# Default: false
debug = false

# Example application config
# This section is optional, used for application specific configuration
[http-gateway]
# Used to generate changelog based on commits in a specific directory
# This setting is optional
# TODO: Not implemented yet
path = "src/commands"
```

### Env variables

By setting an env variable you can override the config from `.vemo.toml` file.

These env variables are supported:

 - `VEMO_FORMAT`
 - `VEMO_DEBUG`

## 🚧 TODO:

 - Generate autocomplete file based on `clap` config - https://docs.rs/clap_complete/latest/clap_complete/
 - Add `install` script
 - Generate release message based on git history for an application directory. The directory can be configured with `path` in `.vemo.toml` file. 
 - Create a release in a git hosting provider. It should support major providers like [`GitHub`](https://github.com/), [`GitLab`](https://gitlab.com/), [`Bitbucket`](https://bitbucket.org/)
