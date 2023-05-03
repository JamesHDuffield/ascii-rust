## Outer Shell

use `cargo run --features bevy/dynamic_linking` to run locally

### Web Build

- `rustup target install wasm32-unknown-unknown`
- `cargo install wasm-bindgen-cli`
- `cargo build --profile release-web --target wasm32-unknown-unknown`
- `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release-web/ascii.wasm`