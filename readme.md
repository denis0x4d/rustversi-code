# Rustversi

This is a Rust implementation of Reversi game (a-ka Othello) https://en.wikipedia.org/wiki/Reversi 
It compiles into EXE,JS,WASM. PWA (wasm+js) working example you can find at https://denis0x4d.github.io/rustversi/

```
Score Player vs Computer -- 2:2
Options: 4, selected: 2
Computer has moved to (5,6), +2 score

   | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
---|---|---|---|---|---|---|---|---|---
 8 |   |   |   |   |   |   |   |   | 8
---|---|---|---|---|---|---|---|---|---
 7 |   |   |   |   |   |   |   |   | 7
---|---|---|---|---|---|---|---|---|---
 6 |   |   |   |   | O |   |   |   | 6
---|---|---|---|---|---|---|---|---|---
 5 |   |   |   | O | O |   |   |   | 5
---|---|---|---|---|---|---|---|---|---
 4 |   |   |   | # | O |   |   |   | 4
---|---|---|---|---|---|---|---|---|---
 3 |   |   |   |   |   |   |   |   | 3
---|---|---|---|---|---|---|---|---|---
 2 |   |   |   |   |   |   |   |   | 2
---|---|---|---|---|---|---|---|---|---
 1 |   |   |   |   |   |   |   |   | 1
---|---|---|---|---|---|---|---|---|---
   | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |


Score Player vs Computer -- 1:4
 x y ?
```

## prerequisites
`cargo install wasm-pack`


## build
* exe: `cargo build --bin rustversi`
* lib: `cargo build --lib`
* nodejs: `wasm-pack build --target nodejs` (see also "node" folder in the project)
* web: `wasm-pack build` (see also "www" folder in the project)


## license
Mozilla Public License 2.0 https://www.mozilla.org/en-US/MPL/2.0/
