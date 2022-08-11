use ad983x::{marker, Ad983x};
use embedded_hal_mock::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct BitFlags;
impl BitFlags {
    pub const D15: u8 = 1 << 7;
    pub const D14: u8 = 1 << 6;
    pub const D13: u8 = 1 << 5;
    pub const B28: u8 = 1 << 5;
    pub const HLB: u8 = 1 << 4;
    pub const FSELECT: u8 = 1 << 3;
    pub const PSELECT: u8 = 1 << 2;
    pub const PIN_SW: u8 = 1 << 1;
    pub const SLEEP_MCLK: u8 = 1 << 7;
    pub const SLEEP_DAC: u8 = 1 << 6;
    pub const OPBITEN: u8 = 1 << 5;
    pub const SIGN_PIB: u8 = 1 << 4;
    pub const DIV2: u8 = 1 << 3;
    pub const MODE: u8 = 1 << 1;
    pub const RESET: u8 = 1;
    pub const FREQ0: u8 = 1 << 6;
    pub const FREQ1: u8 = 1 << 7;
}

pub fn new_ad9833(transactions: &[SpiTrans]) -> Ad983x<SpiMock, marker::Ad9833Ad9837> {
    let wrapped: Vec<SpiTrans> = transactions
        .iter()
        .flat_map(|trans| {
            [
                SpiTrans::transaction_start(),
                trans.clone(),
                SpiTrans::transaction_end(),
            ]
        })
        .collect();
    Ad983x::new_ad9833(SpiMock::new(wrapped.iter()))
}

pub fn new_ad9834(transactions: &[SpiTrans]) -> Ad983x<SpiMock, marker::Ad9834Ad9838> {
    let wrapped: Vec<SpiTrans> = transactions
        .iter()
        .flat_map(|trans| {
            [
                SpiTrans::transaction_start(),
                trans.clone(),
                SpiTrans::transaction_end(),
            ]
        })
        .collect();
    Ad983x::new_ad9834(SpiMock::new(wrapped.iter()))
}

pub fn new_ad9837(transactions: &[SpiTrans]) -> Ad983x<SpiMock, marker::Ad9833Ad9837> {
    let wrapped: Vec<SpiTrans> = transactions
        .iter()
        .flat_map(|trans| {
            [
                SpiTrans::transaction_start(),
                trans.clone(),
                SpiTrans::transaction_end(),
            ]
        })
        .collect();
    Ad983x::new_ad9837(SpiMock::new(wrapped.iter()))
}

pub fn new_ad9838(transactions: &[SpiTrans]) -> Ad983x<SpiMock, marker::Ad9834Ad9838> {
    let wrapped: Vec<SpiTrans> = transactions
        .iter()
        .flat_map(|trans| {
            [
                SpiTrans::transaction_start(),
                trans.clone(),
                SpiTrans::transaction_end(),
            ]
        })
        .collect();
    Ad983x::new_ad9838(SpiMock::new(wrapped.iter()))
}

pub fn destroy<IC>(device: Ad983x<SpiMock, IC>) {
    device.destroy().done();
}
