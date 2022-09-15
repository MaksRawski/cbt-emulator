# CBT emulator

[CBT](https://gitlab.com/MaksRawski/cbt/) emulator written in rust compiled to webassembly with an interface in react.

## Microcode

Real microcode is used although it had to be created with a slightly modified generator
to produce single binary instead of 4 distinct ones for 4 EEPROMs.
Diff for the generator is provided in `ucode/generator.diff` and the generated microcode in `ucode/ucode.bin`.

Control word is 32-bits long so each word in microcode is encode as 4 bytes in big endian.
