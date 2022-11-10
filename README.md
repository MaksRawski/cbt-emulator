# CBT emulator

[CBT](https://gitlab.com/MaksRawski/cbt/) emulator written in rust compiled to webassembly with an interface in react.
Available to play with at: https://maksrawski.gitlab.io/cbt-emulator/.

## Microcode
Refer to `ucode/README.md` for information about generating microcode.

Generated microcode gets compressed and then hardcoded into a binary, then decompressed at runtime.
This way instead of throwing over 256KB into a binary we put just a bit shy (pun intended) of 1KB.
`zstd` is used for compression so the runtime overhead isn't that big.

## Tests
Unit tests for the emulator itself are available to run via `cargo test`, 
however its integration tests have to be run with `wasm-pack test --node`


# TODO
## functionality
- [x] add labels to each bit of CW
- [X] fix hello world
- [X] add LCD module in UI (using react setters)
- [ ] disable CLK when in HLT
- [ ] add a way of previewing what's in ram
- [ ] add a way to set up custom interrupts, which key on the keyboard would trigger which routine in _interrupt space_
- [ ] ability to set custom programs after clicking on ROM module

## aesthetics
- [ ] style for mobile, add a "real view"
- [ ] add tooltips
- [ ] improve clock module's controls
- [ ] cw module should be toggleable

## extra
- [ ] setup codecov
- [ ] make web interface to be a git submodule instead

## changing state
try to use [this](https://stackoverflow.com/a/31869669) somehow

# Acknowledgments 
- ["LCD Dot Matrix HD44780U" font](https://fontstruct.com/fontstructions/show/476121/lcd_dot_matrix_hd44780u)
was used for the LCD's font.
