# Rustversi

This is a Rust implementation of Reversi game (a-ka Othello) https://en.wikipedia.org/wiki/Reversi 
It could be compiled into WASM and used in PWA. Working example https://denis0x4d.github.io/rustversi/


## prerequisites
`cargo install wasm-pack`


## build
* exe: `cargo build --bin rustversi`
* lib: `cargo build --lib`
* nodejs: `wasm-pack build --target nodejs` (see also "node" folder in the project)
* web: `wasm-pack build` (see also "www" folder in the project)


## license
Mozilla Public License 2.0 https://www.mozilla.org/en-US/MPL/2.0/
