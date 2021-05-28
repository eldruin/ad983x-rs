use ad983x::{Ad983x, FrequencyRegister};
use linux_embedded_hal::{Pin, Spidev};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let chip_select = Pin::new(25);
    let mut dds = Ad983x::new_ad9833(spi, chip_select);
    dds.reset().unwrap(); // reset is necessary before operation
    dds.set_frequency(FrequencyRegister::F0, 4724).unwrap();
    dds.enable().unwrap();
    // Given a 25 MHz clock, this now outputs a sine wave
    // with a frequency of 440 Hz, which is a standard
    // A4 tone.

    // Get SPI device and CS pin back
    let (_spi, _chip_select) = dds.destroy();
}
