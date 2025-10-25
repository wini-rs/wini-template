# Scripts
This folder contains useful scripts and their related files, for managing the project.

- These scripts are made to work with [`just`](https://github.com/casey/just), that you can find in `./justfile`.
- They **MUST** be run from the root of the project (where there is the `./wini.toml` or `./justfile`). This is done by default when running `just`.
- All the scripts are made to work with `bash`. They haven't been tested to work with something else.
- You should use [`nix develop`](https://nixos.org/learn) to have all the dependencies installed. If not, see the required packages in `./flake.nix`.

If all these scripts are included here, it's because they are meant to be customizable, so feel free to modify them in your project!
Below is a quick overview of what each script is doing.

## Overview
