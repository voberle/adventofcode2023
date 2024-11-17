# Advent of Code

My *Advent Of Code* code.

It contains the solutions to all years from 2015 to 2023 so far.

## Template

`template` folder has a template to be used with [Cargo Generate](https://cargo-generate.github.io/cargo-generate):

    cd YEAR
    cargo generate aoc --name dayXX

or better using the [aoc_new tool](tools/aoc_new/README.md), in YEAR directory:

    ../aoc_new X

with following in `$CARGO_HOME/cargo-generate.toml`:

    [favorites.aoc]
    description = "Advent of Code template"
    path = "FULL_PATH_TO_TEMPLATE"
    vcs = "Git"

## Clippy

All exercises are free of any Clippy warnings. Clippy is set by default in pedantic mode in the workspace cargo file.

## Compatibility

Tested on Rust 1.82.0.
