# bevy_pipelines_ready

[![crates.io](https://img.shields.io/crates/v/bevy_pipelines_ready.svg)](https://crates.io/crates/bevy_pipelines_ready)
[![docs](https://docs.rs/bevy_pipelines_ready/badge.svg)](https://docs.rs/bevy_pipelines_ready)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A tiny Bevy plugin that counts the number of render pipelines that are ready and makes that data available as a resource in the main world.

This is useful for creating a nice loading experience for your Bevy app, especially on the web where execution is single-threaded and pipeline building is disruptive.

## Usage

See [`examples/states.rs`](examples/states.rs).

## WebGL2 and WebGPU

Install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner).

```bash
# WebGL
cargo run --example states --target=wasm32-unknown-unknown --features=webgl2

# WebGPU
cargo run --example states --target=wasm32-unknown-unknown --features=webgpu
```

## Compatibility

| `bevy_pipelines_ready`   | `bevy` |
| :--                      | :--    |
| `0.3`                    | `0.13` |
| `0.2`                    | `0.12` |
| `0.1`                    | `0.11` |

## Contributing

Please feel free to open a PR.
