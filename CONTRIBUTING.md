# Contributing

Thanks for taking the time to contribute! Below you'll find a set of guidelines for this project:

## What goes where

- `pakket/`
    - `main.rs`: strictly command line logic that doesn't need to be exported, such as argument parsing and subcommands
    - `lib.rs`: command line logic that may be exported, such as platform specific paths and constants that might be of interest to other crates
- `pakket-core/`
    - `lib.rs`: the inner workings of pakket, such as manipulating lockfiles, mirrors and package metadata. Do not use the filesystem or platform specific calls, use generic I/O instead. Frontends such as `pakket` may implement filesystem logic.

## TODO

More to come later.
