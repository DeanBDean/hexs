# Project

A project to demonstrate a hex grid in Bevy & Rust

## Notes

### Debugging

The application uses the bevy-inspector-egui crate as an optional crate intended to only be used during debugging. By using `#[cfg(feature = "debug")]` and `#[cfg_attr(feature = "debug"), derive(SomeOptionalThing)]`, we can create debug code that is only accessed when running under the debug feature. Running the application for testing normally simply requires `cargo run`, while running with debug features requires `cargo run --features debug`. As a sidenote, both of these include debug symbols and are not fully optimized, if you want a fully optimized binary then you must run `cargo build --release`

### Visual Studio Code

If you use Visual Studio Code, be sure to use the rust-analyzer plugin

### rust-toolchain

rust-toolchain is a file used to specify which version of Rust the project is using. This is useful for ensuring you're always on the intended version

### rustfmt.toml

This file is used to globally override `cargo fmt` with individual preferences

### lint denys and warnings

Rust has a large variety of built in lints, but many need to be turned on. I currently use Embark studio's recommendations <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>, but they have to pasted at the top of the entry point, in this case at the top of `main.rs`. While this is not ideal, formal definitions in a file are not a part of Rust yet
