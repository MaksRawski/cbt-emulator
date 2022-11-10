#control word

HLT=1<<0 
LAI=1<<1 
HAI=1<<2 
MO=1<<3 
II=1<<4 
MI=1<<5 
SR=1<<6 
LPO=1<<7 

LPI=1<<8 
HPO=1<<9 
PCC=1<<10 
HPI=1<<11 
AO=1<<12 
AI=1<<13 
CO=1<<14 
CI=1<<15 

AL3=1<<16 
AL2=1<<17 
AL1=1<<18 
AL0=1<<19 
ALO=1<<20 
ALE=1<<21 
ALM=1<<22 
ALC=1<<23 

BO=1<<24 
BI=1<<25 
DO=1<<26 
DI=1<<27 
LCM=1<<28 
LCE=1<<29 
SPO=1<<30 
SPI=1<<31 

RI=(
    AI,   #000
    BI,   #001
    CI,   #010
    DI,   #011
    SPI,  #100
    LPI,  #101
    LCE,  #110
    000   #111
)

RO=(
    AO,   #000
    BO,   #001
    CO,   #010
    DO,   #011
    SPO,  #100
    LPO,  #101
    000,  #110
    000   #111
)

# register pair
RP=(
    [
        DO|HAI,
        CO|LAI,
    ],

    [
        CO|HAI,
        BO|LAI,
    ],

    [
        BO|HAI,
        AO|LAI,
    ],

    [
        DO|HAI,
        AO|LAI,
    ]
)

DATA=[SR|PCC]*65536
