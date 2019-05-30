use {marker, Ad983x, BitFlags, ControlSource, Error, OutputWaveform, SignBitOutput, SpiInterface};

impl<SPI, CS> Ad983x<SpiInterface<SPI, CS>, marker::Ad9834Ad9838> {
    /// Create a new instance of an AD9834 device.
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9834(spi: SPI, chip_select: CS) -> Self {
        Self::create(spi, chip_select)
    }

    /// Create a new instance of an AD9838 device.
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9838(spi: SPI, chip_select: CS) -> Self {
        Self::create(spi, chip_select)
    }
}

impl<SPI, CS, E> Ad983x<SpiInterface<SPI, CS>, marker::Ad9834Ad9838>
where
    SPI: hal::blocking::spi::Write<u8, Error = E>,
    CS: hal::digital::OutputPin,
{
    /// Set the output waveform
    ///
    /// Will return `Error::InvalidArgument` for `SquareMsbOfDac` and `SquareMsbOfDacDiv2`
    /// as this is not available on AD9834/AD9838 devices. To set the digital output,
    /// please use
    pub fn set_output_waveform(&mut self, waveform: OutputWaveform) -> Result<(), Error<E>> {
        let control;
        match waveform {
            OutputWaveform::Sinusoidal => {
                control = self
                    .control
                    .with_low(BitFlags::OPBITEN)
                    .with_low(BitFlags::MODE)
            }
            OutputWaveform::Triangle => {
                control = self
                    .control
                    .with_low(BitFlags::OPBITEN)
                    .with_high(BitFlags::MODE)
            }
            OutputWaveform::SquareMsbOfDac => return Err(Error::InvalidArgument),
            OutputWaveform::SquareMsbOfDacDiv2 => return Err(Error::InvalidArgument),
        };
        self.write_control(control)
    }

    /// Set the digital output
    pub fn set_sign_bit_output(&mut self, configuration: SignBitOutput) -> Result<(), Error<E>> {
        let control = match configuration {
            SignBitOutput::Disabled => self.control.with_low(BitFlags::OPBITEN),
            SignBitOutput::Comparator => self
                .control
                .with_high(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE)
                .with_high(BitFlags::SIGN_PIB)
                .with_high(BitFlags::DIV2),
            SignBitOutput::SquareMsbOfDac => self
                .control
                .with_high(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE)
                .with_low(BitFlags::SIGN_PIB)
                .with_high(BitFlags::DIV2),
            SignBitOutput::SquareMsbOfDacDiv2 => self
                .control
                .with_high(BitFlags::OPBITEN)
                .with_low(BitFlags::MODE)
                .with_low(BitFlags::SIGN_PIB)
                .with_low(BitFlags::DIV2),
        };
        self.write_control(control)
    }

    /// Set the control source used for the functions:
    /// frequency register selection, phase register selection,
    /// reset of internal registers, and DAC power-down.
    pub fn set_control_source(&mut self, source: ControlSource) -> Result<(), Error<E>> {
        let control = match source {
            ControlSource::Software => self.control.with_low(BitFlags::PIN_SW),
            ControlSource::HardwarePins => self.control.with_high(BitFlags::PIN_SW),
        };
        self.write_control(control)
    }
}
