//! This is a platform agnostic Rust driver for the AD9833, AD9834, AD9837
//! and AD9838 low-power programmable waveform generators / direct digital
//! synthesizers (DDS) using the [`embedded-hal`] traits.
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
//! - Select control source on AD9834/AD9838. See: [`set_control_source()`].
//!
//! [`enable()`]: struct.Ad983x.html#method.enable
//! [`set_frequency()`]: struct.Ad983x.html#method.set_frequency
//! [`select_frequency()`]: struct.Ad983x.html#method.select_frequency
//! [`set_phase()`]: struct.Ad983x.html#method.set_phase
//! [`select_phase()`]: struct.Ad983x.html#method.select_phase
//! [`set_frequency_msb()`]: struct.Ad983x.html#method.set_frequency_msb
//! [`set_output_waveform()`]: struct.Ad983x.html#method.set_output_waveform
//! [`set_powered_down()`]: struct.Ad983x.html#method.set_powered_down
//! [`set_control_source()`]: struct.Ad983x.html#method.set_control_source
//!
//! ## The devices
//!
//! The AD9833, AD9834, AD9837 and AD9838 are low power, programmable waveform
//! generators capable of producing sine, triangular, and square wave outputs.
//! Waveform generation is required in various types of sensing, actuation,
//! and time domain reflectometry (TDR) applications. The output frequency and
//! phase are software programmable, allowing easy tuning. No external
//! components are needed. The frequency registers are 28 bits wide: with a
//! 25 MHz clock rate, resolution of 0.1 Hz can be achieved; with a 1 MHz
//! clock rate, the AD9833 can be tuned to 0.004 Hz resolution.
//!
//! The devices are written to via a 3-wire serial interface (SPI).
//! This serial interface operates at clock rates up to 40 MHz and is
//! compatible with DSP and microcontroller standards. The devices operate
//! with a power supply from 2.3 V to 5.5 V.
//!
//! Datasheets:
//! - [AD9833](https://www.analog.com/media/en/technical-documentation/data-sheets/ad9833.PDF)
//! - [AD9834](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9834.PDF)
//! - [AD9837](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9837.PDF)
//! - [AD9838](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9838.PDF)
//!
//! Application Note:
//! - [Programming the AD9833/AD9834](https://www.analog.com/media/en/technical-documentation/application-notes/AN-1070.pdf)
//!
//! Article explaining DDS using an AD9833:
//! - [All about direct digital synthesis](https://www.analog.com/en/analog-dialogue/articles/all-about-direct-digital-synthesis.html)
//!
//! ## Pin / Software control source on AD9834/AD9838
//!
//! AD9834/AD9838 devices offer the possibility to control several functions
//! either through hardware pins or software settings. While hardware pin
//! control is selected, these software operations will be ignored by the hardware.
//! This driver allows this as well as it would be a valid use case to
//! configure the status of these functions while on hardware pin control mode
//! in preparation for a smooth switch to software control.
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
//!
//! ### Use hardware pins as control source
//!
//! ```no_run
//! extern crate ad983x;
//! extern crate linux_embedded_hal;
//!
//! use ad983x::{Ad983x, ControlSource};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//! let mut dds = Ad983x::new_ad9838(spi, chip_select);
//! dds.reset().unwrap(); // reset is necessary before operation
//! dds.set_control_source(ControlSource::HardwarePins).unwrap();
//! // Hardware pins can now be used to control the device.
//! // The corresponding software settings will be ignored.
//! # }
//! ```
#![deny(unsafe_code, missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use core::marker::PhantomData;
use hal::spi::{Mode, Phase, Polarity};

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
    /// Sinusoidal wave (default)
    Sinusoidal,
    /// Triangle wave
    Triangle,
    /// Square wave with its value matching the MSB of DAC data
    /// (not available on AD9834/AD9838, use `SignBitOutput`)
    SquareMsbOfDac,
    /// Square wave with its value matching the MSB of DAC data divided by 2
    /// (not available on AD9834/AD9838, use `SignBitOutput`)
    SquareMsbOfDacDiv2,
}

/// Sign bit output on AD9834/AD9838 devices
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignBitOutput {
    /// Disabled (high impedance) (default)
    Disabled,
    /// Comparator output
    Comparator,
    /// Square wave with its value matching the MSB of DAC data
    SquareMsbOfDac,
    /// Square wave with its value matching the MSB of DAC data divided by 2
    SquareMsbOfDacDiv2,
}

/// Powered-down device configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoweredDown {
    /// All chip parts are enabled (default)
    Nothing,
    /// Power down only the DAC
    Dac,
    /// Disable only the internal clock
    InternalClock,
    /// Power down the DAC and disable the internal clock
    DacAndInternalClock,
}

/// Hardware pin / software control source for the functions:
/// frequency register selection, phase register selection,
/// reset of internal registers, and DAC power-down.
/// (Only available on AD9834 and AD9838 devices)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlSource {
    /// Functions are controlled only through software (default)
    Software,
    /// Functions are controlled only through hardware pins
    HardwarePins,
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

/// Markers
#[doc(hidden)]
pub mod marker {
    /// AD9833/AD9837 device
    pub struct Ad9833Ad9837(());
    /// AD9834/AD9838 device
    pub struct Ad9834Ad9838(());
}

struct BitFlags;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}

/// AD983x direct digital synthesizer
#[derive(Debug, Default)]
pub struct Ad983x<DI, IC> {
    iface: DI,
    control: Config,
    _ic: PhantomData<IC>,
}

mod ad9833_ad9837;
mod ad9834_ad9838;
mod common;

mod private {
    use super::{marker, SpiInterface};
    pub trait Sealed {}

    impl<SPI, CS> Sealed for SpiInterface<SPI, CS> {}

    impl Sealed for marker::Ad9833Ad9837 {}
    impl Sealed for marker::Ad9834Ad9838 {}
}
