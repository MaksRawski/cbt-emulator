pub const CW_LABELS: [&str; 32] = [
    "HLT", "LAI", "HAI", "MO", "II", "MI", "SR", "LPO", "LPI", "HPO", "PCC", "HPI", "AO", "AI",
    "CO", "CI", "AL3", "AL2", "AL1", "AL0", "ALO", "ALE", "ALM", "ALC", "BO", "BI", "DO", "DI",
    "LCM", "LCE", "SPO", "SPI",
];

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

/// Converts control word value into a collection of
/// labels which correspond to each control bit set.
pub fn cw_to_labels(cw: u32) -> Vec<&'static str> {
    let mut res = Vec::new();
    for i in 0..32 {
        if (cw & 1 << i) > 0 {
            res.push(CW_LABELS[i])
        }
    }
    res
}

#[test]
fn test_cw_to_labels() {
    assert_eq!(cw_to_labels(LPO | LAI | PCC), vec!["LAI", "LPO", "PCC"]);
}

#[macro_export]
/// works similarly to normal match except that:
/// - first argument needs to be cw to which each pattern will be compared against
/// - second argument must be the default value for the match
macro_rules! cw_match {
    ($cw:expr, $default:expr, $cb1:expr => $f1:expr, $($cb2:expr => $f2:expr),*) => {
        if $cb1 & $cw > 0 {
            $f1
        } $(else if $cb2 & $cw > 0{
            $f2
        })+ else {
            $default
        }
    }
}

#[test]
fn test_cw_match() {
    let bus = cw_match!(ALO, 0xff,
                        HLT|ALO => 12,
                        ALO => 34);
    assert_eq!(bus, 12);
}

#[macro_export]
/// works similarly to c-like `switch` except that:
/// - first argument needs to be cw to which each pattern will be compared against
/// - there is no `break`; there can be multiple matches
macro_rules! cw_switch {
    ($cw:expr, $($cb:expr => $f:expr),+) => {
        $(if $cb & $cw > 0 {
            $f
        })+
    };
}
