from instructions import *
import struct

# address is composed as so
# FLAGS  MICROTIME INSTRUCTION REGISTER
#  xxxx     xxxx     xxxx xxxx

#addData actually writes data
#opcode,ut and flag are just for getting address
for opcode in range(2**8):
    for ut in range(2**4):
        for flag in range(2**4):
            if ut in [0,1,2]: #first 3 steps of every instruction is to actually fetch it
                addData(fetch(ut),opcode,ut,flag)
            else:
                addData(
                    {
                        0: mov,
                        1: load,
                        2: sto,
                        3: alu,
                    }[opcode>>6]
                    (opcode,ut-3,flag), # function's location parameters
                    opcode,ut,flag # addData's location parameters
                )

    if (opcode+1)%64==0:
        print("Writing "+{
        0: "mov's",
        1: "load's",
        2: "store's",
        3: "alu's"
    }[opcode>>6])

# DATA has 2**16 32bit words
print("Writing to file")

# write 32bit words as 4 big endian bytes
s = struct.Struct(">I")
with open("ucode.bin","wb") as f:
    for d in DATA:
        f.write(s.pack(d))
