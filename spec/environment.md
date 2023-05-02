# Environment Specification

This section specifies the format a directory under `environment/<name>/` must follow.

Entries marked with \* are required.

1. \*[`setup.toml`](#setuptoml)
2. [`destroy.toml`](#destroytoml)
3. \*[`home/`](#home)
4. [`root/`](#root)

## setup.toml

This file contains the dependencies, files and other options that describe the environment.

### info

1. `name`
   Display name of this environment.

2. `version`
   A positive whole integer. (e.g. 1, 20, 65)

3. `username`
   Username of the user these changes should be made as, typically used to find the `$HOME` directory.

4. `requires`
   Dependencies needed for this environment to work correctly. Same as running `paru -S <packages>` prior to running the util.

5. `use_shared` An environment could be extended with a "shared" environment, this shared environment must
   reside in the `environments/shared` directory. The files in the extended environment could be overridden by creating a file with a relative match.

   For an example:

   ```
    - [shared] ~/.config/kitty/
        |- ~/.config/kitty/kitty.conf
        |- ~/.config/kitty/current-theme.conf

    - [i3] ~/.config/kitty/
        |-  ~/.config/kitty/kitty.conf

    - [result]
        |- environments/i3/.config/kitty/kitty.conf
        |- environments/shared/.config/kitty/current-theme.conf
   ```

## destroy.toml

Describes the environment when it is being reverted.
Usually, this is not needed as [setup.toml](#setup.toml) already directs to the correct files, dependencies and etc.

## home

Files relative to the given `HOME` directory.

As an example, `environments/i3/home/.config` with `info.username = "Curstantine"` is resolved as `/home/Curstantine/.config`.

## root

For files that access `/boot/` and other directories inaccessible by the user.

Changes done to this directory could be controlled by passing a `--no-root` flag to `confls`.
