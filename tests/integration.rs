use ad983x::{
    ControlSource, FrequencyRegister as FreqReg, OutputWaveform as OW, PhaseRegister as PhaseReg,
    PoweredDown as PD, SignBitOutput as SBO,
};
use embedded_hal_mock::spi::Transaction as SpiTrans;

mod base;
use crate::base::{destroy, new_ad9833, new_ad9834, new_ad9837, new_ad9838, BitFlags as BF};

#[test]
fn can_create_and_destroy_ad9833() {
    let dev = new_ad9833(&[]);
    destroy(dev);
}

#[test]
fn can_create_and_destroy_ad9837() {
    let dev = new_ad9837(&[]);
    destroy(dev);
}

#[test]
fn can_create_and_destroy_ad9834() {
    let dev = new_ad9834(&[]);
    destroy(dev);
}

#[test]
fn can_create_and_destroy_ad9838() {
    let dev = new_ad9838(&[]);
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
    ($name:ident, $create:ident, $ow:ident, $control:expr) => {
        #[test]
        fn $name() {
            let transitions = [SpiTrans::write(vec![BF::RESET, $control])];
            let mut dev = $create(&transitions);
            dev.set_output_waveform(OW::$ow).unwrap();
            destroy(dev);
        }
    };
}

ow_test!(can_set_sinusoidal_out, new_ad9833, Sinusoidal, 0);
ow_test!(can_set_triangle_out, new_ad9833, Triangle, BF::MODE);
ow_test!(
    can_set_sq_msb_out,
    new_ad9833,
    SquareMsbOfDac,
    BF::OPBITEN | BF::DIV2
);
ow_test!(
    can_set_sq_msb_div2_out,
    new_ad9833,
    SquareMsbOfDacDiv2,
    BF::OPBITEN
);
ow_test!(can_set_sinusoidal_out_ad9838, new_ad9838, Sinusoidal, 0);
ow_test!(can_set_triangle_out_ad9838, new_ad9838, Triangle, BF::MODE);

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

#[test]
fn cannot_set_ow_dac_ad9838() {
    let mut dev = new_ad9838(&[]);
    dev.set_output_waveform(OW::SquareMsbOfDac)
        .expect_err("Should return error");
    destroy(dev);
}

#[test]
fn cannot_set_ow_dac_div2_ad9838() {
    let mut dev = new_ad9838(&[]);
    dev.set_output_waveform(OW::SquareMsbOfDacDiv2)
        .expect_err("Should return error");
    destroy(dev);
}

macro_rules! sbo_test {
    ($name:ident, $sbo:ident, $control:expr) => {
        #[test]
        fn $name() {
            let transitions = [SpiTrans::write(vec![BF::RESET, $control])];
            let mut dev = new_ad9838(&transitions);
            dev.set_sign_bit_output(SBO::$sbo).unwrap();
            destroy(dev);
        }
    };
}

sbo_test!(can_set_disabled_sign_out, Disabled, 0);
sbo_test!(
    can_set_comp_sign_out,
    Comparator,
    BF::OPBITEN | BF::SIGN_PIB | BF::DIV2
);
sbo_test!(
    can_set_sq_msb_sign_out,
    SquareMsbOfDac,
    BF::OPBITEN | BF::DIV2
);
sbo_test!(
    can_set_sq_msb_div2_sign_out,
    SquareMsbOfDacDiv2,
    BF::OPBITEN
);

#[test]
fn can_set_control_source_sw() {
    let transitions = [SpiTrans::write(vec![BF::RESET, 0])];
    let mut dev = new_ad9838(&transitions);
    dev.set_control_source(ControlSource::Software).unwrap();
    destroy(dev);
}

#[test]
fn can_set_control_source_hw_pins() {
    let transitions = [SpiTrans::write(vec![BF::RESET | BF::PIN_SW, 0])];
    let mut dev = new_ad9838(&transitions);
    dev.set_control_source(ControlSource::HardwarePins).unwrap();
    destroy(dev);
}
