# RustyWebTriangles <img alt="npm" src="https://img.shields.io/npm/v/rusty_web_triangles">

This repo is for directly integrating the Three JS data into the NPhysics3D Rust physics library.

Becuase NColliders is a dependency of the full NPhysics runtime it can also be partially used to do runtime collision detection with minimal overhead.

[nphysics website](https://nphysics.org/)

## Building

To build you will need to do a couple of specific things.

1. Register on npmjs.com
2. ` cargo install --git https://github.com/rustwasm/wasm-pack.git `
3. ` cargo build `
4. ` wasm-pack build `
5. ` wasm-pack login `
6. ` wasm-pack publish `

## Future

This project will integrate with the MIRA robotics simulation project.

It will also include a custom open protobuf spec that can be interfaced with instead of the regular JS types.
