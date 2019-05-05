use ad983x::{Ad983x, SpiInterface};
use hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct BitFlags;
impl BitFlags {
    pub const RESET: u16 = 1 << 8;
}

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_ad9833(
    transactions: &[SpiTrans<u16>],
) -> Ad983x<SpiInterface<SpiMock<u16>, DummyOutputPin>> {
    Ad983x::new_ad9833(SpiMock::new(transactions), DummyOutputPin)
}

pub fn destroy(device: Ad983x<SpiInterface<SpiMock<u16>, DummyOutputPin>>) {
    device.destroy().0.done();
}
