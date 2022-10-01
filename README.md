# CBT emulator

[CBT](https://gitlab.com/MaksRawski/cbt/) emulator written in rust compiled to webassembly with an interface in react.

## Microcode
Refer to `ucode/README.md` for information about generating microcode.

Generated microcode gets compressed and then hardcoded into a binary, then decompressed at runtime.
This way instead of throwing over 256KB into a binary we put just a bit shy (pun intended) of 1KB.
`zstd` is used for compression so the runtime overhead isn't that big.

## Tests
Unit tests for the emulator itself are available to run via `cargo test`, 
however its integration tests have to be run with `wasm-pack test --node`

