pub const HLT: u32 = 1 << 0;
pub const LAI: u32 = 1 << 1;
pub const HAI: u32 = 1 << 2;
pub const MO: u32 = 1 << 3;
pub const II: u32 = 1 << 4;
pub const MI: u32 = 1 << 5;
pub const SR: u32 = 1 << 6;
pub const LPO: u32 = 1 << 7;

pub const LPI: u32 = 1 << 8;
pub const HPO: u32 = 1 << 9;
pub const PCC: u32 = 1 << 10;
pub const HPI: u32 = 1 << 11;
pub const AO: u32 = 1 << 12;
pub const AI: u32 = 1 << 13;
pub const CO: u32 = 1 << 14;
pub const CI: u32 = 1 << 15;

pub const AL3: u32 = 1 << 16;
pub const AL2: u32 = 1 << 17;
pub const AL1: u32 = 1 << 18;
pub const AL0: u32 = 1 << 19;
pub const ALO: u32 = 1 << 20;
pub const ALE: u32 = 1 << 21;
pub const ALM: u32 = 1 << 22;
pub const ALC: u32 = 1 << 23;

pub const BO: u32 = 1 << 24;
pub const BI: u32 = 1 << 25;
pub const DO: u32 = 1 << 26;
pub const DI: u32 = 1 << 27;
pub const LCM: u32 = 1 << 28;
pub const LCE: u32 = 1 << 29;
pub const SPO: u32 = 1 << 30;
pub const SPI: u32 = 1 << 31;

// from ucode/instructions.py
pub const NOT_A: u32 = ALE;
pub const A_NOR_B: u32 = AL0 | ALE;
pub const A_NAND_B: u32 = AL2 | ALE;
pub const NOT_B: u32 = AL2 | AL0 | ALE;
pub const A_XOR_B: u32 = AL2 | AL1 | ALE;
pub const A_XNOR_B: u32 = AL3 | AL0 | ALE;
pub const A_AND_B: u32 = AL3 | AL1 | AL0 | ALE;
pub const A_OR_B: u32 = AL3 | AL2 | AL1 | ALE;

pub const ADD_A_B: u32 = ALM | AL3 | AL0 | ALE;
pub const ADC_A_B: u32 = ALM | ALC | AL3 | AL0 | ALE;
pub const SUB_A_B: u32 = ALM | ALC | AL2 | AL1 | ALE;
pub const SBC_A_B: u32 = ALM | AL2 | AL1 | ALE;
pub const CMP_A_B: u32 = ALM | ALC | AL2 | AL1 | ALE;
pub const INC_A: u32 = ALM | ALC | ALE;
pub const DEC_A: u32 = ALM | AL3 | AL2 | AL1 | AL0 | ALE;
pub const SHL_A: u32 = ALM | AL3 | AL2 | ALE;
