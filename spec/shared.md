# Shared Environment Specification

This section specifies the format `environments/shared` must follow.

Entries marked with \* are required.

1. \*[`setup.toml`](#setuptoml)
2. [`destroy.toml`](#destroytoml)
3. \*[`home/`](#home)
4. [`root/`](#root)

## setup.toml

Unlike a usual [`environment/setup.toml`](./environment#setuptoml), this file only contains dependencies and
other options that could later be merged with the environment that extends this shared environment.

1. `version`
   A positive whole integer. (e.g. 1, 20, 65)

2. `requires`
   Dependencies needed for this environment to work correctly. Same as running `paru -S <packages>` prior to running the util. The dependencies are merged with the environment that extends this shared environment.

## destroy.toml

Describes the environment when it is being reverted.
Usually, this is not needed as [setup.toml](#setup.toml) already directs to the correct files, dependencies and etc.

## home/

Files relative to the given `HOME` directory.

As an example, `environments/i3/home/.config` with `info.username = "Curstantine"` is resolved as `/home/Curstantine/.config`.

## root/

For files that access `/boot/` and other directories inaccessible by the user.

Changes done to this directory could be controlled by passing a `--no-root` flag to `confls`.
