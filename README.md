# Rust AD983x Low-Power Programmable Waveform Generator / Direct Digital Synthesizer (DDS) Driver

[![crates.io](https://img.shields.io/crates/v/ad983x.svg)](https://crates.io/crates/ad983x)
[![Docs](https://docs.rs/ad983x/badge.svg)](https://docs.rs/ad983x)
[![Build Status](https://github.com/eldruin/ad983x-rs/workflows/Build/badge.svg)](https://github.com/eldruin/ad983x-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/ad983x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/ad983x-rs?branch=master)

This is a platform agnostic Rust driver for the AD9833, AD9834, AD9837 and AD9838 low-power programmable waveform generators / direct digital synthesizers (DDS) using the [`embedded-hal`] traits.

This driver allows you to:
- Enable/disable/reset the device. See `enable()`.
- Set the frequency registers. See: `set_frequency()`.
- Select the output frequency register. See: `select_frequency()`.
- Set the phase registers. See: `set_phase()`.
- Select the output phase register. See: `select_phase()`.
- Set the frequency registers MSBs/LSBs separately. See: `set_frequency_msb()`.
- Set the output waveform. See: `set_output_waveform()`.
- Power down/up device parts. See: `set_powered_down()`.
- Select control source on AD9834/AD9838. See: `set_control_source()`.

[Introductory blog post](https://blog.eldruin.com/ad983x-waveform-generator-dds-driver-in-rust/)

## The devices

The AD9833, AD9834, AD9837 and AD9838 are low power, programmable waveform generators capable of producing sine, triangular, and square wave outputs. Waveform generation is required in various types of sensing, actuation, and time domain reflectometry (TDR) applications. The output frequency and phase are software programmable, allowing easy tuning. No external components are needed. The frequency registers are 28 bits wide: with a 25 MHz clock rate, resolution of 0.1 Hz can be achieved; with a 1 MHz clock rate, the AD9833 can be tuned to 0.004 Hz resolution.

The devices are written to via a 3-wire serial interface (SPI). This serial interface operates at clock rates up to 40 MHz and is compatible with DSP and microcontroller standards. The devices operate with a power supply from 2.3 V to 5.5 V.

Datasheets:
- [AD9833](https://www.analog.com/media/en/technical-documentation/data-sheets/ad9833.PDF)
- [AD9834](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9834.PDF)
- [AD9837](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9837.PDF)
- [AD9838](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9838.PDF)

Application Note:
- [Programming the AD9833/AD9834](https://www.analog.com/media/en/technical-documentation/application-notes/AN-1070.pdf)

Article explaining DDS using an AD9833:
- [All about direct digital synthesis](https://www.analog.com/en/analog-dialogue/articles/all-about-direct-digital-synthesis.html)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

I wrote an example MIDI player that plays Beethoven's ninth symphony in hardware :). See: [driver-examples].

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate ad983x;
extern crate linux_embedded_hal;

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
```

## Status

- [X] Compatible with AD9833/AD9837
- [X] Compatible with AD9834/AD9838
- [ ] Compatible with AD9832/AD9835

## Support

For questions, issues, feature requests like compatibility with similar devices
and other changes, please file an
[issue in the github project](https://github.com/eldruin/ad983x-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
