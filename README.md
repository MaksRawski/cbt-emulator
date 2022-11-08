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


# TODO
- [x] add labels to each bit of CW
- [-] fix hello world
- [ ] setup codecov
- [ ] make web interface to be a git submodule instead
- [ ] add LCD module in UI (using react setters)
- [ ] style for mobile, add a "real view"
- [ ] add tooltips
- [ ] improve clock module's controls
- [ ] add a way of previewing what's in ram
- [ ] disable CLK when in HLT
- [ ] ability to set custom programs after clicking on ROM module
- [ ] add a way to set up custom interrupts, which key on the keyboard would trigger which routine in _interrupt space_


## changing state
try to use [this](https://stackoverflow.com/a/31869669) somehow
