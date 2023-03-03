# CBT emulator

[CBT](https://gitlab.com/MaksRawski/cbt/) emulator written in rust compiled to webassembly with an interface in react.
Available to play with at: https://maksrawski.gitlab.io/cbt-emulator/.

## Microcode
Refer to `ucode/README.md` for information about generating microcode.

Generated microcode gets compressed and then hardcoded into a binary, then decompressed at runtime.
This way instead of throwing over 256KB into a binary we put just a bit shy (pun intended) of 1KB.
`zstd` is used for compression so the runtime overhead isn't that big.

## Tests
Unit tests for the emulator itself are available to run via `cargo test`.

# TODO
## functionality
- [x] add labels to each bit of CW
- [X] add LCD module in UI (using react setters)
- [-] disable CLK when in HLT
- [-] ability to use custom programs
- [ ] add a way of previewing what's in memory
- [ ] add a way to set up custom interrupts, 
once you create an interrupt button will be generated on the page
to which there needs to be assigned address of the routine run on each interrupt
and optionally to which key on the keboard should it be mapped
- [ ] create a "gaming mode" (of both backend and frontend)
where the speed is determined at the backend (or more likely it's run as quickly as it can)
and LCD is the only module visible

## aesthetics
- [ ] style for mobile; supply "monospace" font
- [ ] add a "real view"; image of real cbt as background
- [ ] add tooltips
- [ ] cw module should be toggleable
- [ ] improve clock module's controls

## extra
- [ ] setup codecov

# Acknowledgments 
- ["LCD Dot Matrix HD44780U" font](https://fontstruct.com/fontstructions/show/476121/lcd_dot_matrix_hd44780u)
was used for the LCD's font.
