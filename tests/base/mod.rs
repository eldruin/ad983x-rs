use ad983x::{marker, Ad983x, SpiInterface};
use hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct BitFlags;
impl BitFlags {
    pub const D15: u8 = 1 << 7;
    pub const D14: u8 = 1 << 6;
    pub const D13: u8 = 1 << 5;
    pub const B28: u8 = 1 << 5;
    pub const HLB: u8 = 1 << 4;
    pub const FSELECT: u8 = 1 << 3;
    pub const PSELECT: u8 = 1 << 2;
    pub const SLEEP_MCLK: u8 = 1 << 7;
    pub const SLEEP_DAC: u8 = 1 << 6;
    pub const OPBITEN: u8 = 1 << 5;
    pub const DIV2: u8 = 1 << 3;
    pub const MODE: u8 = 1 << 1;
    pub const RESET: u8 = 1;
    pub const FREQ0: u8 = 1 << 6;
    pub const FREQ1: u8 = 1 << 7;
}

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_ad9833(
    transactions: &[SpiTrans],
) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>, marker::Ad9833Ad9837> {
    Ad983x::new_ad9833(SpiMock::new(transactions), DummyOutputPin)
}

pub fn new_ad9834(
    transactions: &[SpiTrans],
) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>, marker::Ad9834Ad9838> {
    Ad983x::new_ad9834(SpiMock::new(transactions), DummyOutputPin)
}

pub fn new_ad9837(
    transactions: &[SpiTrans],
) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>, marker::Ad9833Ad9837> {
    Ad983x::new_ad9837(SpiMock::new(transactions), DummyOutputPin)
}

pub fn new_ad9838(
    transactions: &[SpiTrans],
) -> Ad983x<SpiInterface<SpiMock, DummyOutputPin>, marker::Ad9834Ad9838> {
    Ad983x::new_ad9838(SpiMock::new(transactions), DummyOutputPin)
}

pub fn destroy<IC>(device: Ad983x<SpiInterface<SpiMock, DummyOutputPin>, IC>) {
    device.destroy().0.done();
}
