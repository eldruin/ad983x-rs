use embedded_hal::spi::blocking::{SpiBus, SpiDevice};

use crate::{marker, Ad983x, BitFlags, Error, OutputWaveform};

impl<DEV, E> Ad983x<DEV, marker::Ad9833Ad9837>
where
    DEV: SpiDevice<Error = E>,
    DEV::Bus: SpiBus,
{
    /// Create a new instance of an AD9833 device.
    ///
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9833(spi: DEV) -> Self {
        Self::create(spi)
    }
    /// Create a new instance of an AD9837 device.
    ///
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9837(spi: DEV) -> Self {
        // Behaves the same as AD9833
        Self::create(spi)
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
}
