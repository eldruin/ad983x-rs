extern crate ad983x;
extern crate embedded_hal_mock as hal;
use self::hal::spi::Transaction as SpiTrans;

mod base;
use base::{destroy, new_ad9833, BitFlags as BF};

// TODO: wait for resolution of https://github.com/dbrgn/embedded-hal-mock/issues/25

#[test]
fn can_create_and_destroy() {
    let dev = new_ad9833(&[]);
    destroy(dev);
}

#[test]
fn can_enable() {
    let transitions = [SpiTrans::write(vec![0])];
    let mut dev = new_ad9833(&transitions);
    dev.enable().unwrap();
    destroy(dev);
}

#[test]
fn can_disable() {
    let transitions = [SpiTrans::write(vec![BF::RESET])];
    let mut dev = new_ad9833(&transitions);
    dev.disable().unwrap();
    destroy(dev);
}

#[test]
fn can_reset() {
    let transitions = [SpiTrans::write(vec![BF::RESET])];
    let mut dev = new_ad9833(&transitions);
    dev.reset().unwrap();
    destroy(dev);
}
