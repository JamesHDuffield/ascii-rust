# ASCII Space Shooter

![Rust Version](https://img.shields.io/static/v1?logo=Rust&label=&message=1.70&color=grey)
![Language](https://img.shields.io/github/languages/top/jameshduffield/ascii-rust)
![Latest Commit](https://img.shields.io/github/last-commit/jameshduffield/ascii-rust)

A retro ASCII art space shooter, inspired by Vampire Survivors, written in the [Bevy](https://bevyengine.org/) game engine.

## How to play

A web (WASM) version of the game is available at [https://jameshduffield.github.io/ascii-rust](https://jameshduffield.github.io/ascii-rust)

No automated build of the windows standalone are available yet, but you can compile the app yourself.

## Development

*Note: This project has been an educational exercise to further learning of the Rust programming language.*


- Install rust: https://www.rust-lang.org/tools/install
- `cargo run --features bevy/dynamic_linking` to run locally

## Web Build

To generate the web build a new target must be installed and bundled.

- `rustup target install wasm32-unknown-unknown`
- `cargo install wasm-bindgen-cli --version 0.2.92`
- `cargo build --profile release-web --target wasm32-unknown-unknown`
- `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release-web/ascii.wasm`

The `/out` directory should then be hosted locally with any web server.