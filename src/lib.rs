//! This is a platform agnostic Rust driver for the AD9833 and AD9837 low-power
//! programmable waveform generators / direct digital synthesizers (DDS)
//! using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable/reset the device. See [`enable()`].
//! - Set the frequency registers. See: [`set_frequency()`].
//! - Select the output frequency register. See: [`select_frequency()`].
//! - Set the phase registers. See: [`set_phase()`].
//! - Select the output phase register. See: [`select_phase()`].
//! - Set the frequency registers MSBs/LSBs separately. See: [`set_frequency_msb()`].
//! - Set the output waveform. See: [`set_output_waveform()`].
//! - Power down/up device parts. See: [`set_powered_down()`].
//!
//! [`enable()`]: struct.Ad983x.html#method.enable
//! [`set_frequency()`]: struct.Ad983x.html#method.set_frequency
//! [`select_frequency()`]: struct.Ad983x.html#method.select_frequency
//! [`set_phase()`]: struct.Ad983x.html#method.set_phase
//! [`select_phase()`]: struct.Ad983x.html#method.select_phase
//! [`set_frequency_msb()`]: struct.Ad983x.html#method.set_frequency_msb
//! [`set_output_waveform()`]: struct.Ad983x.html#method.set_output_waveform
//! [`set_powered_down()`]: struct.Ad983x.html#method.set_powered_down
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
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! In the following examples an instance of the device AD9833 will be created
//! as an example. Other devices can be created with similar methods like:
//! `Ad983x::new_ad9837(...)`.
//!
//! Please find additional examples using hardware in this repository: [driver-examples].
//!
//! This includes an example MIDI player that plays Beethoven's ninth symphony.
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Set the frequency register 0 and enable
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, FrequencyRegister};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9833(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! dds.set_frequency(FrequencyRegister::F0, 4724).unwrap();
//! dds.enable().unwrap();
//! // Given a 25 MHz clock, this now outputs a sine wave
//! // with a frequency of 440 Hz, which is a standard
//! // A4 tone.
//!
//! // Get SPI device and CS pin back
//! let (_spi, _chip_select) = dds.destroy();
//! # }
//! ```
//!
//! ### Set frequency registers 0 and 1 and alternate between them
//!
//! With a 25 MHz clock this alternates between A4 and D5 tones.
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, FrequencyRegister};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9833(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! // A4 tone for a 25 MHz clock
//! dds.set_frequency(FrequencyRegister::F0, 4724).unwrap();
//! // D5 tone for a 25 MHz clock
//! dds.set_frequency(FrequencyRegister::F1, 6306).unwrap();
//! dds.enable().unwrap();
//! loop {
//!     // some delay
//!     dds.select_frequency(FrequencyRegister::F1).unwrap();
//!     // some delay
//!     dds.select_frequency(FrequencyRegister::F0).unwrap();
//! }
//! # }
//! ```
//!
//! ### Set the phase register 1 and select it
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, PhaseRegister};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9833(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! dds.set_phase(PhaseRegister::P1, 4724).unwrap();
//! dds.select_phase(PhaseRegister::P1).unwrap();
//! # }
//! ```
//!
//! ### Set output waveform to be triangular
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, OutputWaveform};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9833(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! dds.set_output_waveform(OutputWaveform::Triangle).unwrap();
//! # }
//! ```
//!
//! ### Power down the DAC
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, PoweredDown};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9833(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! dds.set_powered_down(PoweredDown::Dac).unwrap();
//! # }
//! ```
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::spi::{Mode, Phase, Polarity};

struct BitFlags;
impl BitFlags {
    const D15: u16 = 1 << 15;
    const D14: u16 = 1 << 14;
    const D13: u16 = 1 << 13;
    const B28: u16 = 1 << 13;
    const HLB: u16 = 1 << 12;
    const FSELECT: u16 = 1 << 11;
    const PSELECT: u16 = 1 << 10;
    const RESET: u16 = 1 << 8;
    const SLEEP_MCLK: u16 = 1 << 7; // SLEEP1
    const SLEEP_DAC: u16 = 1 << 6; // SLEEP12
    const OPBITEN: u16 = 1 << 5;
    const DIV2: u16 = 1 << 3;
    const MODE: u16 = 1 << 1;
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

/// Phase registers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhaseRegister {
    /// Phase register 0
    P0,
    /// Phase register 1
    P1,
}

/// Output waveform
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputWaveform {
    /// Sinusoidal wave
    Sinusoidal,
    /// Triangle wave
    Triangle,
    /// Square wave with its value matching the MSB of DAC data
    SquareMsbOfDac,
    /// Square wave with its value matching the MSB of DAC data divided by 2
    SquareMsbOfDacDiv2,
}

/// Powered-down device configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoweredDown {
    /// All chip parts are enabled
    Nothing,
    /// Power down only the DAC
    Dac,
    /// Disable only the internal clock
    InternalClock,
    /// Power down the DAC and disable the internal clock
    DacAndInternalClock,
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

    fn write_control_if_different(&mut self, control: Config) -> Result<(), Error<E>> {
        if control != self.control {
            self.write_control(control)
        } else {
            Ok(())
        }
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

    fn check_value_fits<T>(value: T, bit_count: T) -> Result<(), Error<E>>
    where
        T: From<u8> + PartialOrd + core::ops::Shl<Output = T>,
    {
        if value >= (T::from(1) << bit_count) {
            Err(Error::InvalidArgument)
        } else {
            Ok(())
        }
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
        Self::check_value_fits(value, 28)?;
        let control = self.control.with_high(BitFlags::B28);
        self.write_control_if_different(control)?;
        let lsb = value & ((1 << 14) - 1);
        let msb = value >> 14;
        let reg = Self::get_freq_register_bits(register);
        self.write(reg | lsb as u16)?;
        self.write(reg | msb as u16)
    }

    fn get_freq_register_bits(register: FrequencyRegister) -> u16 {
        match register {
            FrequencyRegister::F0 => BitFlags::D14,
            FrequencyRegister::F1 => BitFlags::D15,
        }
    }

    /// Set the frequency 14-bit MSBs
    ///
    /// This will deactivate the 28-bit mode if it is not already the case.
    /// Returns `Error::InvalidArgument` if providing a value that does not fit in 14 bits.
    pub fn set_frequency_msb(
        &mut self,
        register: FrequencyRegister,
        value: u16,
    ) -> Result<(), Error<E>> {
        Self::check_value_fits(value, 14)?;
        let control = self
            .control
            .with_low(BitFlags::B28)
            .with_high(BitFlags::HLB);
        self.write_control_if_different(control)?;
        let reg = Self::get_freq_register_bits(register);
        self.write(reg | value as u16)
    }

    /// Set the frequency 14-bit LSBs
    ///
    /// This will deactivate the 28-bit mode if it is not already the case.
    /// Returns `Error::InvalidArgument` if providing a value that does not fit in 14 bits.
    pub fn set_frequency_lsb(
        &mut self,
        register: FrequencyRegister,
        value: u16,
    ) -> Result<(), Error<E>> {
        Self::check_value_fits(value, 14)?;
        let control = self.control.with_low(BitFlags::B28).with_low(BitFlags::HLB);
        self.write_control_if_different(control)?;
        let reg = Self::get_freq_register_bits(register);
        self.write(reg | value as u16)
    }

    /// Select the frequency register that is used
    pub fn select_frequency(&mut self, register: FrequencyRegister) -> Result<(), Error<E>> {
        let control = match register {
            FrequencyRegister::F0 => self.control.with_low(BitFlags::FSELECT),
            FrequencyRegister::F1 => self.control.with_high(BitFlags::FSELECT),
        };
        self.write_control(control)
    }

    /// Set a phase register (12-bit value)
    ///
    /// Returns `Error::InvalidArgument` if providing a value that does not fit in 12 bits.
    pub fn set_phase(&mut self, register: PhaseRegister, value: u16) -> Result<(), Error<E>> {
        Self::check_value_fits(value, 12)?;
        let value = value | BitFlags::D14 | BitFlags::D15;
        let value = match register {
            PhaseRegister::P0 => value,
            PhaseRegister::P1 => value | BitFlags::D13,
        };
        self.write(value)
    }

    /// Select the phase register that is used
    pub fn select_phase(&mut self, register: PhaseRegister) -> Result<(), Error<E>> {
        let control = match register {
            PhaseRegister::P0 => self.control.with_low(BitFlags::PSELECT),
            PhaseRegister::P1 => self.control.with_high(BitFlags::PSELECT),
        };
        self.write_control(control)
    }

    /// Set the output waveform
    pub fn set_output_waveform(&mut self, waveform: OutputWaveform) -> Result<(), Error<E>> {
        let control = match waveform {
            OutputWaveform::Sinusoidal => self
                .control
                .with_low(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE),
            OutputWaveform::Triangle => self
                .control
                .with_low(BitFlags::OPBITEN)
                .with_high(BitFlags::MODE),
            OutputWaveform::SquareMsbOfDac => self
                .control
                .with_high(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE)
                .with_high(BitFlags::DIV2),
            OutputWaveform::SquareMsbOfDacDiv2 => self
                .control
                .with_high(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE)
                .with_low(BitFlags::DIV2),
        };
        self.write_control(control)
    }

    /// Set device parts powered-down state
    pub fn set_powered_down(&mut self, config: PoweredDown) -> Result<(), Error<E>> {
        let control = match config {
            PoweredDown::Nothing => self
                .control
                .with_low(BitFlags::SLEEP_MCLK)
                .with_low(BitFlags::SLEEP_DAC),
            PoweredDown::Dac => self
                .control
                .with_low(BitFlags::SLEEP_MCLK)
                .with_high(BitFlags::SLEEP_DAC),
            PoweredDown::InternalClock => self
                .control
                .with_high(BitFlags::SLEEP_MCLK)
                .with_low(BitFlags::SLEEP_DAC),
            PoweredDown::DacAndInternalClock => self
                .control
                .with_high(BitFlags::SLEEP_MCLK)
                .with_high(BitFlags::SLEEP_DAC),
        };
        self.write_control(control)
    }
}
