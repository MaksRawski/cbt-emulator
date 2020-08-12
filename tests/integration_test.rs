use cbt_emulator;
use cbt_emulator::DataBus;
use std::num::Wrapping;

mod test_alu {
    use super::*;

    // TODO: somehow get rid of this repetetive declaration of variables
    #[test]
    fn not() {
        let mut alu = cbt_emulator::Alu::new();
        let mut b = cbt_emulator::Register::new();

        b.set(1);
        assert_eq!(alu.not(&b), Wrapping(254));
    }

    #[test]
    fn nor() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(15);
        b.set(240);
        assert_eq!(alu.nor(&a, &b), Wrapping(0));
    }
    #[test]
    fn nand() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(14);
        b.set(28);
        assert_eq!(alu.nand(&a, &b), Wrapping(255 - 12));
    }
    #[test]
    fn xor() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(14);
        b.set(28);
        assert_eq!(alu.xor(&a, &b), Wrapping(18));
    }
    #[test]
    fn xnor() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(14);
        b.set(28);
        assert_eq!(alu.xnor(&a, &b), Wrapping(255 - 18));
    }
    #[test]
    fn and() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(14);
        b.set(28);
        assert_eq!(alu.and(&a, &b), Wrapping(12));
    }
    #[test]
    fn or() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(14);
        b.set(28);
        assert_eq!(alu.or(&a, &b), Wrapping(30));
    }

    #[test]
    fn add() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(127);
        b.set(127);

        assert_eq!(alu.add(&a, &b), Wrapping(254));
        assert_eq!(alu.get_flag('c'), false);
        assert_eq!(alu.get_flag('n'), true);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), false);
    }
    #[test]
    fn adc() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        alu.set_flag('c', true);
        a.set(63);
        b.set(64);

        assert_eq!(alu.adc(&a, &b), Wrapping(128));
        assert_eq!(alu.get_flag('c'), false);
        assert_eq!(alu.get_flag('n'), true);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), false);
    }
    #[test]
    fn sub() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(128);
        b.set(128);
        // -128-(-128) = 0

        assert_eq!(alu.sub(&a, &b), Wrapping(0));
        assert_eq!(alu.get_flag('c'), true);
        assert_eq!(alu.get_flag('n'), false);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), true);
    }
    #[test]
    fn sbc() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        alu.set_flag('c', false);
        a.set(128);
        b.set(128);

        assert_eq!(alu.sbc(&a, &b), Wrapping(255));
        assert_eq!(alu.get_flag('c'), false);
        assert_eq!(alu.get_flag('n'), true);
        assert_eq!(alu.get_flag('o'), false);
        assert_eq!(alu.get_flag('z'), false);
    }
    #[test]
    fn cmp() {
        let mut alu = cbt_emulator::Alu::new();
        let mut a = cbt_emulator::Register::new();
        let mut b = cbt_emulator::Register::new();

        a.set(128);
        b.set(128);

        alu.cmp(&a, &b);
        assert_eq!(alu.get_flag('c'), false);
        assert_eq!(alu.get_flag('n'), false);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), true);
    }
    #[test]
    fn inc() {
        let mut alu = cbt_emulator::Alu::new();
        let mut b = cbt_emulator::Register::new();

        b.set(255);

        assert_eq!(alu.inc(&b), Wrapping(0));
        assert_eq!(alu.get_flag('c'), true);
        assert_eq!(alu.get_flag('n'), false);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), true);
    }
    #[test]
    fn dec() {
        let mut alu = cbt_emulator::Alu::new();
        let mut b = cbt_emulator::Register::new();

        b.set(0);

        assert_eq!(alu.dec(&b), Wrapping(255));
        assert_eq!(alu.get_flag('c'), false);
        assert_eq!(alu.get_flag('n'), true);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), false);
    }
    #[test]
    fn shl() {
        let mut alu = cbt_emulator::Alu::new();
        let mut b = cbt_emulator::Register::new();

        b.set(128);

        assert_eq!(alu.shl(&b), Wrapping(0));
        assert_eq!(alu.get_flag('c'), true);
        assert_eq!(alu.get_flag('n'), false);
        assert_eq!(alu.get_flag('o'), true);
        assert_eq!(alu.get_flag('z'), true);
    }
}
