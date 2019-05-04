//! This is a platform agnostic Rust driver for the AD9833 and AD9837 low-power
//! programmable waveform generators / direct digital synthesizers (DDS)
//! using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! ## The devices
//!
//! The AD9833 and AD9837 are low power, programmable waveform generators
//! capable of producing sine, triangular, and square wave outputs. Waveform
//! generation is required in various types of sensing, actuation, and time
//! domain reflectometry (TDR) applications. The output frequency and phase
//! are software programmable, allowing easy tuning. No external components
//! are needed. The frequency registers are 28 bits wide: with a 25 MHz clock
//! rate, resolution of 0.1 Hz can be achieved; with a 1 MHz clock rate, the
//! AD9833 can be tuned to 0.004 Hz resolution.
//!
//! The AD9833 and AD9837 are written to via a 3-wire serial interface (SPI).
//! This serial interface operates at clock rates up to 40 MHz and is
//! compatible with DSP and microcontroller standards. The device operates
//! with a power supply from 2.3 V to 5.5 V.
//!
//! Datasheets:
//! - [AD9833](https://www.analog.com/media/en/technical-documentation/data-sheets/ad9833.PDF)
//! - [AD9837](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9837.PDF)
//!
//! Application Note:
//! - [Programming the AD9833/AD9834](https://www.analog.com/media/en/technical-documentation/application-notes/AN-1070.pdf)
//!
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::spi::{Mode, Phase, Polarity};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// SPI communication error
    Spi(E),
}

/// SPI mode (CPOL = 1, CPHA = 0)
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleHigh,
};

/// SPI interface
#[doc(hidden)]
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
}

/// AD983x direct digital synthesizer
#[derive(Debug, Default)]
pub struct Ad983x<DI> {
    iface: DI,
}
