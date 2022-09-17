# Modified [CBT microcode generator](https://gitlab.com/MaksRawski/cbt/-/tree/master/Microcode)

Instead of creating 4 seperate files which would be uploaded into EEPROMs, it creates a single binary
made up of 32 bit words (big endian).

## Generating
```
python generate.py
zstd -c -T0 --ultra -20 ucode.bin -o ucode.zstd
```
