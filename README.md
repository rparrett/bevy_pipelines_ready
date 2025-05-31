# bevy_pipelines_ready

[![crates.io](https://img.shields.io/crates/v/bevy_pipelines_ready.svg)](https://crates.io/crates/bevy_pipelines_ready)
[![docs](https://docs.rs/bevy_pipelines_ready/badge.svg)](https://docs.rs/bevy_pipelines_ready)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A tiny Bevy plugin that counts the number of render pipelines that are ready and makes that data available as a resource in the main world.

Bevy's shaders are compiled on-demand. On native platforms, they are compiled asynchronously, which can create pop-in effects. On the web, they are compiled synchronously in the single-threaded environment, which can cause hitches and can be pretty disastrous if music is playing.

This plugin can help you get your shaders compiled during a loading state where it won't cause as much trouble for your players.

## Usage

See [`examples/states.rs`](examples/states.rs).

## WebGL2 and WebGPU

### Build

```bash
# WebGL
cargo build --target wasm32-unknown-unknown --example "states" --features="webgl2"

# WebGPU
cargo build --target wasm32-unknown-unknown --example "states" --features="webgpu"
```

### Bindgen

```bash
mkdir -p examples/wasm/target
wget https://raw.githubusercontent.com/bevyengine/bevy/refs/tags/v0.15.0/examples/wasm/index.html -O examples/wasm/index.html
wasm-bindgen --out-dir examples/wasm/target --out-name wasm_example --target web target/wasm32-unknown-unknown/debug/examples/states.wasm
```

### Serve

```bash
basic-http-server examples/wasm/
```

## Compatibility

| `bevy_pipelines_ready`   | `bevy` |
| :--                      | :--    |
| `0.6`                    | `0.16` |
| `0.5`                    | `0.15` |
| `0.4`                    | `0.14` |
| `0.3`                    | `0.13` |
| `0.2`                    | `0.12` |
| `0.1`                    | `0.11` |

## Contributing

Please feel free to open a PR.
