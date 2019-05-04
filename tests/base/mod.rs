use ad983x::{Ad983x, SpiInterface};
extern crate embedded_hal_mock as hal;
use self::hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_ad9833(transactions: &[SpiTrans]) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>> {
    Ad983x::new_ad9833(SpiMock::new(&transactions), DummyOutputPin)
}

pub fn destroy(device: Ad983x<SpiInterface<SpiMock, DummyOutputPin>>) {
    device.destroy().0.done();
}
