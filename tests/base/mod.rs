use ad983x::{Ad983x, SpiInterface};
use hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct BitFlags;
impl BitFlags {
    pub const B28: u8 = 1 << 5;
    pub const FSELECT: u8 = 1 << 3;
    pub const RESET: u8 = 1;
    pub const FREQ0: u8 = 1 << 6;
    pub const FREQ1: u8 = 1 << 7;
}

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_ad9833(transactions: &[SpiTrans]) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>> {
    Ad983x::new_ad9833(SpiMock::new(transactions), DummyOutputPin)
}

pub fn destroy(device: Ad983x<SpiInterface<SpiMock, DummyOutputPin>>) {
    device.destroy().0.done();
}
