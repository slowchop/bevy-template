# Slowchop Studios Bevy Template

## Todo

* [ ] bevy-generate
* [ ] Splash screen
* [ ] Generic menu system

## Overview

Basic boilerplate for a for making quick prototypes or game jams, with no specific gameplay or graphics in mind.

* Bevy `0.10.0`
* Faster compile times:
    * Rust nightly toolchain
    * Dynamic linking
    * LLD linker
* Some test assets
* Crates:
    * `bevy_egui`
    * `bevy_prototype_debug_lines`

# Assets

* [assets/README.md](assets/README.md) for attribution.

## Ubuntu

```
sudo apt-get install lld
```

## Windows

```
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```
