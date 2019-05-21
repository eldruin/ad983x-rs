extern crate ad983x;
use ad983x::{
    FrequencyRegister as FreqReg, OutputWaveform as OW, PhaseRegister as PhaseReg,
    PoweredDown as PD,
};
extern crate embedded_hal_mock as hal;
use self::hal::spi::Transaction as SpiTrans;

mod base;
use base::{destroy, new_ad9833, BitFlags as BF};

#[test]
fn can_create_and_destroy() {
    let dev = new_ad9833(&[]);
    destroy(dev);
}

#[test]
fn can_enable() {
    let transitions = [SpiTrans::write(vec![0, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.enable().unwrap();
    destroy(dev);
}

#[test]
fn can_disable() {
    let transitions = [SpiTrans::write(vec![BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.disable().unwrap();
    destroy(dev);
}

#[test]
fn can_reset() {
    let transitions = [SpiTrans::write(vec![BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.reset().unwrap();
    destroy(dev);
}

#[test]
fn cannot_set_too_fast_frequency() {
    let mut dev = new_ad9833(&[]);
    dev.set_frequency(FreqReg::F0, 1 << 28)
        .expect_err("Should return error");
    destroy(dev);
}

#[test]
fn can_set_freq0() {
    let transitions = [
        SpiTrans::write(vec![BF::B28 | BF::RESET, 0]),
        SpiTrans::write(vec![BF::FREQ0 | 0xD, 0xEF]),
        SpiTrans::write(vec![BF::FREQ0 | 0x26, 0xAF]),
    ];
    let mut dev = new_ad9833(&transitions);
    dev.set_frequency(FreqReg::F0, 0x9AB_CDEF).unwrap();
    destroy(dev);
}

#[test]
fn can_set_freq1() {
    let transitions = [
        SpiTrans::write(vec![BF::B28 | BF::RESET, 0]),
        SpiTrans::write(vec![BF::FREQ1 | 0xD, 0xEF]),
        SpiTrans::write(vec![BF::FREQ1 | 0x26, 0xAF]),
    ];
    let mut dev = new_ad9833(&transitions);
    dev.set_frequency(FreqReg::F1, 0x9AB_CDEF).unwrap();
    destroy(dev);
}

#[test]
fn can_select_freq0() {
    let transitions = [SpiTrans::write(vec![BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.select_frequency(FreqReg::F0).unwrap();
    destroy(dev);
}

#[test]
fn can_select_freq1() {
    let transitions = [SpiTrans::write(vec![BF::FSELECT | BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.select_frequency(FreqReg::F1).unwrap();
    destroy(dev);
}

#[test]
fn cannot_set_wrong_phase() {
    let mut dev = new_ad9833(&[]);
    dev.set_phase(PhaseReg::P0, 1 << 12)
        .expect_err("Should return error");
    destroy(dev);
}

#[test]
fn can_set_phase0() {
    let transitions = [SpiTrans::write(vec![BF::D15 | BF::D14 | 0xA, 0xBC])];
    let mut dev = new_ad9833(&transitions);
    dev.set_phase(PhaseReg::P0, 0xABC).unwrap();
    destroy(dev);
}

#[test]
fn can_set_phase1() {
    let transitions = [SpiTrans::write(vec![
        BF::D15 | BF::D14 | BF::D13 | 0xA,
        0xBC,
    ])];
    let mut dev = new_ad9833(&transitions);
    dev.set_phase(PhaseReg::P1, 0xABC).unwrap();
    destroy(dev);
}

#[test]
fn can_select_phase0() {
    let transitions = [SpiTrans::write(vec![BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.select_phase(PhaseReg::P0).unwrap();
    destroy(dev);
}

#[test]
fn can_select_phase1() {
    let transitions = [SpiTrans::write(vec![BF::PSELECT | BF::RESET, 0])];
    let mut dev = new_ad9833(&transitions);
    dev.select_phase(PhaseReg::P1).unwrap();
    destroy(dev);
}

macro_rules! ow_test {
    ($name:ident, $ow:ident, $control:expr) => {
        #[test]
        fn $name() {
            let transitions = [SpiTrans::write(vec![BF::RESET, $control])];
            let mut dev = new_ad9833(&transitions);
            dev.set_output_waveform(OW::$ow).unwrap();
            destroy(dev);
        }
    };
}

ow_test!(can_set_sinusoidal_out, Sinusoidal, 0);
ow_test!(can_set_triangle_out, Triangle, BF::MODE);
ow_test!(can_set_sq_msb_out, SquareMsbOfDac, BF::OPBITEN | BF::DIV2);
ow_test!(can_set_sq_msb_div2_out, SquareMsbOfDacDiv2, BF::OPBITEN);

macro_rules! pd_test {
    ($name:ident, $pd:ident, $control:expr) => {
        #[test]
        fn $name() {
            let transitions = [SpiTrans::write(vec![BF::RESET, $control])];
            let mut dev = new_ad9833(&transitions);
            dev.set_powered_down(PD::$pd).unwrap();
            destroy(dev);
        }
    };
}

pd_test!(can_set_pd_nothing, Nothing, 0);
pd_test!(can_set_pd_dac, Dac, BF::SLEEP_DAC);
pd_test!(can_set_pd_mclk, InternalClock, BF::SLEEP_MCLK);
pd_test!(
    can_set_pd_dac_mclk,
    DacAndInternalClock,
    BF::SLEEP_MCLK | BF::SLEEP_DAC
);

#[test]
fn cannot_set_wrong_freq_msb() {
    let mut dev = new_ad9833(&[]);
    dev.set_frequency_msb(FreqReg::F0, 1 << 14)
        .expect_err("Should return error");
    destroy(dev);
}

#[test]
fn cannot_set_wrong_freq_lsb() {
    let mut dev = new_ad9833(&[]);
    dev.set_frequency_lsb(FreqReg::F0, 1 << 14)
        .expect_err("Should return error");
    destroy(dev);
}

#[test]
fn can_set_freq_msb() {
    let transitions = [
        SpiTrans::write(vec![BF::HLB | BF::RESET, 0]),
        SpiTrans::write(vec![BF::FREQ0 | 0xD, 0xEF]),
    ];
    let mut dev = new_ad9833(&transitions);
    dev.set_frequency_msb(FreqReg::F0, 0xDEF).unwrap();
    destroy(dev);
}

#[test]
fn can_set_freq_lsb() {
    let transitions = [SpiTrans::write(vec![BF::FREQ1 | 0xD, 0xEF])];
    let mut dev = new_ad9833(&transitions);
    dev.set_frequency_lsb(FreqReg::F1, 0xDEF).unwrap();
    destroy(dev);
}
