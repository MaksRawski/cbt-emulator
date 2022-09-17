from variables import *

#invert all inputs despite MI, PCC, ALU inputs and obviously the one(s) given that should be activated
def CW(x = 0):
    return ((1<<32)-1)^MI^PCC^AL0^AL1^AL2^AL3^int(x)

def addData(data,opcode,ut,flag):
    DATA[(flag<<12)+(ut<<8)+(opcode)]=CW(data)


#these opcode functions just return data
def fetch(utime):
    return{
        0: LPO|LAI,
        1: HPO|HAI,
        2: II|MO|PCC
    }[utime]


def mov(opcode, utime, flags):

    #for easy selection of parts of opcodes
    dst=(opcode&0b00111000)>>3
    src=(opcode&0b00000111)>>0

    data=[
        (RO[src]|RI[dst]) #src out, dst in
    ]

    if src==0 and dst==0: #nop
        data=[
            0
        ]

    elif src==0b110: #mov to lcd as a command
        if dst==0b111: #actual src is 111-immediate operand
            data=[
                HPO|HAI,
                LPO|LAI,
                MO|LCE|LCM|PCC
            ]
        elif dst==0b110: #00 110 110 - straight up bs
            data=[
                 HLT
            ]
        else:
            data=[
                RO[dst]|LCE|LCM # use dst as a command for lcd
            ]

    elif src==0b111: #src is imm, data opcode
        data=[
            HPO|HAI,
            LPO|LAI,
            MO|(RI[dst])|PCC 
        ]
    elif dst==0b111:
        # src = 0 xx 
        # xx==00 dc
        # xx==01 cb
        # xx==10 ba
        # xx==11 da
        if src&0b11==0b00:
            data=[
                HPO|HAI,
                LPO|LAI|PCC,

                MO|DI,

                HPO|HAI,
                LPO|LAI|PCC,

                MO|CI,
            ]

        elif src&0b11==0b01:
            data=[
                HPO|HAI,
                LPO|LAI|PCC,

                MO|CI,

                HPO|HAI,
                LPO|LAI|PCC,

                MO|BI,
            ]
        elif src&0b11==0b10:
            data=[
                HPO|HAI,
                LPO|LAI|PCC,

                MO|BI,

                HPO|HAI,
                LPO|LAI|PCC,

                MO|AI,
            ]
        elif src&0b11==0b11:
            data=[
                HPO|HAI,
                LPO|LAI|PCC,

                MO|DI,

                HPO|HAI,
                LPO|LAI|PCC,

                MO|AI,
            ]
    # CF and HF are active low so they need to be inverted - xor'd with ones
    CF=(flags&0b0001)^1
    HF=(flags&0b0010)^1
    OF=(flags&0b0100)
    ZF=(flags&0b1000)

    # (conditional) jumps
    if dst==0b101:
        if (src==0 and CF) or (src==1 and HF) or (src==2 and OF) or (src==3 and ZF) or (src==0b111):
            data=[
                LPO|LAI|PCC,
                HPO|HAI,
                MO|ALM|ALE, #dst HPC is in memory but we save it in alu

                LPO|LAI|PCC,
                HPO|HAI,
                MO|LPI,

                ALO|HPI,
                SR
            ]

        else:
            data=[
                PCC,
                PCC, # skip over the next two bytes which contain destionation address
                SR, # flags might change (due to saving in alu) therefore jump on flag continues here its execution

                LPO|LAI|PCC,
                HPO|HAI,
                MO|LPI,

                ALO|HPI,
                SR
            ]

    else:
        data.append(SR)

    try:
        return data[utime]
    except IndexError:
        return SR

def load(opcode, utime, flags):

    dst=(opcode&0b00111000)>>3
    src=(opcode&0b00000111)>>0

    data=[
        
    ]

    if src<=0b011:
        # src = 0 xx 
        # xx==00 dc
        # xx==01 cb
        # xx==10 ba
        # xx==11 da
        if src==0b00:
            data=[
                DO|HAI,
                CO|LAI,
                MO|RI[dst]
            ]
        elif src==0b01:
            data=[
                CO|HAI,
                BO|LAI,
                MO|RI[dst]
            ]
        elif src==0b10:
            data=[
                BO|HAI,
                AO|LAI,
                MO|RI[dst]
            ]
        elif src==0b11:
            data=[
                DO|HAI,
                AO|LAI,
                MO|RI[dst]
            ]
        

    elif src==0b111:
        data=[
            LPO|LAI,
            HPO|HAI|PCC, #first byte provided is now in memory (LMAR)

            MO|ALM|ALE, #save HMAR into alu

            LPO|LAI,
            HPO|HAI|PCC, #second byte provided is now in memory (HMAR)

            MO|HAI,  #HMAR is now what it's supposed to be
            ALO|LAI, #LMAR is now what it's supposed to be 

            MO|RI[dst]
        ]

    elif src==0b100:
        #pop into register
        data=[ 
            HAI, #set HMAR to 0xff

            SPO|ALM|ALC|ALE, #increment SP,by default S=0000 M=L Cn=L
            SPI|ALO|LAI, # increment SP and LMAR

            MO|RI[dst]  #pop the value into dst register
            
        ]

        if dst==0b101:
            #ret
            data=[
                HAI, #set HMAR to 0xff

                SPO|ALM|ALC|ALE, #increment SP,by default S=0000 M=L Cn=L
                SPI|ALO|LAI, # increment SP and LMAR

                LPI|MO,  #pop the value into LPC

                SPO|ALM|ALC|ALE, #increment SP,by default S=0000 M=L Cn=L
                SPI|ALO|LAI, # increment SP and LMAR

                HPI|MO #pop the other value off stack into HPC
            ]

    data.append(SR)
    try:
        return data[utime]
    except IndexError:
        return SR

def sto(opcode, utime, flags):

    dst=(opcode&0b00111000)>>3
    src=(opcode&0b00000111)>>0

    data=[

    ]

    if dst<=0b011:
        # normal store

        # store [rp],imm
        if src==0b111:
            data=[
                MO|ALM|ALE,

                RP[dst][0],
                RP[dst][1],

                ALO|MI
            ]
        # can't read from lcd
        elif src==0b110:
            data=[]

        # store [rp],src
        else:
            data=[
                RP[dst][0],
                RP[dst][1],

                MI|RO[src]
            ]

    elif dst==0b100:
        # push related stores

        # push src
        if src<=0b11:
            data=[
                HAI, #set HMAR to 0xff
                SPO|LAI, #set LMAR to value in SP

                RO[src]|MI, #push the register onto stack 

                SPO|ALM|AL0|AL1|AL2|AL3|ALE, #decrement SP, S=1111 M=L Cn=H
                SPI|ALO
            ]

        # push value
        elif src==0b111:
            data=[
                HPO|HAI,
                LPO|LAI,
                MO|ALM|ALE, # save given byte in alu

                HAI, #set HMAR to 0xff
                SPO|LAI, #set LMAR to value in SP

                ALO|MI, #push the value onto stack 

                SPO|ALM|AL0|AL1|AL2|AL3|ALE, #decrement SP, S=1111 M=L Cn=H
                SPI|ALO
            ]
        
        # call
        elif src==0b101:
            data=[
                HAI, #set HMAR to 0xff
                SPO|LAI, #set LMAR to value in SP
                # MAR -> stack
                HPO|MI, # push HPC

                SPO|ALM|AL0|AL1|AL2|AL3|ALE, #decrement SP, S=1111 M=L Cn=H
                SPI|ALO|LAI, # decrement LMAR and SP
                
                LPO|ALM|ALC|ALE,
                ALO|ALM|ALC|ALE,
                ALO|ALM|ALC|ALE,

                ALO|MI,

                SPO|ALM|AL0|AL1|AL2|AL3|ALE, #decrement SP, S=1111 M=L Cn=H
                SPI|ALO # decrement SP

            ]

        # can't read from lcd
        elif src==0b110:
            data=[]

    # can't read from lcd
    elif dst==0b110:
        data=[]

    elif dst==0b111:    
        data=[
            LPO|LAI,
            HPO|HAI|PCC, #first byte provided is now in memory (HMAR)

            MO|ALM|ALE, #save HMAR into alu

            LPO|LAI,
            HPO|HAI|PCC, #second byte provided is now in memory (LMAR)

            MO|LAI,  #LMAR is now what it's supposed to be
            ALO|HAI, #HMAR is now what it's supposed to be 

            MI|RO[src] #store src 

        ]


    data.append(SR)
    try:
        return data[utime]
    except IndexError:
        return SR


def alu(opcode, utime, flags):
    
    op=(opcode&0b00111100)>>2
    src=opcode&0b00000011

    CF=(flags&0b0001)^1

    ops=[
        ALE, # NOT A
        AL0|ALE, # A NOR B
        AL2|ALE, # A NAND B
        AL2|AL0|ALE, # NOT B
        AL2|AL1|ALE, # A XOR B
        AL3|AL0|ALE, # A XNOR B
        AL3|AL1|AL0|ALE, # A AND B
        AL3|AL2|AL1|ALE, # A OR B

        ALM|AL3|AL0|ALE, # ADD A,B
        ALM|ALC*(CF^1)|AL3|AL0|ALE, # ADC A,B ALC=CF`
        ALM|ALC|AL2|AL1|ALE, # SUB A,B
        ALM|ALC*(CF)|AL2|AL1|ALE, # SBC A,B ALC=CF
        ALM|ALC|AL2|AL1|ALE, # CMP A,B
        ALM|ALC|ALE, # INC A
        ALM|AL3|AL2|AL1|AL0|ALE, # DEC A
        ALM|AL3|AL2|ALE # DBL A/SHIFT LEFT A
    ]
    
    if op==0b1100:
        data=[
            RO[src]|ops[op]
        ]
    else:
        data=[
            RO[src]|ops[op],
            ALO|RI[src] #output of ALU operation goes to src register
        ]

    data.append(SR)
    try:
        return data[utime]
    except IndexError:
        return SR
