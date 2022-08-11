use ad983x::{Ad983x, FrequencyRegister};
use embedded_hal::spi::blocking::ExclusiveDevice;
use linux_embedded_hal::{Spidev, SysfsPin};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let chip_select = SysfsPin::new(25);
    let dev = ExclusiveDevice::new(spi, chip_select);
    let mut dds = Ad983x::new_ad9833(dev);
    dds.reset().unwrap(); // reset is necessary before operation
    dds.set_frequency(FrequencyRegister::F0, 4724).unwrap();
    dds.enable().unwrap();
    // Given a 25 MHz clock, this now outputs a sine wave
    // with a frequency of 440 Hz, which is a standard
    // A4 tone.

    // Get device back
    let _dev = dds.destroy();
}
