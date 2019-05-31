use {marker, Ad983x, BitFlags, Error, OutputWaveform, SpiInterface, SpiWrite};

impl<SPI, CS> Ad983x<SpiInterface<SPI, CS>, marker::Ad9833Ad9837> {
    /// Create a new instance of an AD9833 device.
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9833(spi: SPI, chip_select: CS) -> Self {
        Self::create(spi, chip_select)
    }
    /// Create a new instance of an AD9837 device.
    /// Remember to call `reset()` before using the device after power up.
    pub fn new_ad9837(spi: SPI, chip_select: CS) -> Self {
        // Behaves the same as AD9833
        Self::create(spi, chip_select)
    }
}

impl<CommE, PinE, DI> Ad983x<DI, marker::Ad9833Ad9837>
where
    DI: SpiWrite<Error = Error<CommE, PinE>>,
{
    /// Set the output waveform
    pub fn set_output_waveform(&mut self, waveform: OutputWaveform) -> Result<(), DI::Error> {
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
