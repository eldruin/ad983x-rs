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

struct BitFlags;
impl BitFlags {
    const B28: u16 = 1 << 13;
    const RESET: u16 = 1 << 8;
}
/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// SPI communication error
    Spi(E),
    /// Invalid argument provided
    InvalidArgument,
}

/// Frequency registers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrequencyRegister {
    /// Frequency register 0
    F0,
    /// Frequency register 1
    F1,
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

/// AD983x direct digital synthesizer
#[derive(Debug, Default)]
pub struct Ad983x<DI> {
    iface: DI,
    control: Config,
}

impl<SPI, CS> Ad983x<SpiInterface<SPI, CS>> {
    /// Create a new instance of an AD9833 device.
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9833(spi: SPI, chip_select: CS) -> Self {
        Ad983x {
            iface: SpiInterface {
                spi,
                cs: chip_select,
            },
            control: Config {
                bits: BitFlags::RESET,
            },
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

impl<SPI, CS, E> Ad983x<SpiInterface<SPI, CS>>
where
    SPI: hal::blocking::spi::Write<u8, Error = E>,
    CS: hal::digital::OutputPin,
{
    /// Resets the internal registers and leaves the device disabled.
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.disable()
    }

    /// Disable the device (enable reset)
    ///
    /// This resets the internal registers
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let control = self.control.with_high(BitFlags::RESET);
        self.write_control(control)
    }

    /// Enable the device (disable reset)
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let control = self.control.with_low(BitFlags::RESET);
        self.write_control(control)
    }

    fn write_control(&mut self, control: Config) -> Result<(), Error<E>> {
        let payload = control.bits & 0b0011_1111_1111_1111;
        self.write(payload)?;
        self.control = control;
        Ok(())
    }

    fn write(&mut self, payload: u16) -> Result<(), Error<E>> {
        self.iface.cs.set_low();
        let result = self
            .iface
            .spi
            .write(&[(payload >> 8) as u8, payload as u8])
            .map_err(Error::Spi);
        self.iface.cs.set_high();
        result
    }

    /// Set the frequency as a 28-bit word
    ///
    /// This will change the mode to 28-bit if it is not used.
    /// Returns `Error::InvalidArgument` if providing a value that does not fit in 28 bits.
    pub fn set_frequency(
        &mut self,
        register: FrequencyRegister,
        value: u32,
    ) -> Result<(), Error<E>> {
        if value >= (1 << 28) {
            return Err(Error::InvalidArgument);
        }
        let control = self.control.with_high(BitFlags::B28);
        if control != self.control {
            self.write_control(control)?;
        }
        let lsb = value & ((1 << 14) - 1);
        let msb = value >> 14;
        let reg = match register {
            FrequencyRegister::F0 => 1 << 14,
            FrequencyRegister::F1 => 1 << 15,
        };
        self.write(reg | lsb as u16)?;
        self.write(reg | msb as u16)
    }
}
