# Rust AD9833/AD9837 Low-Power Programmable Waveform Generator / Direct Digital Synthesizer (DDS) Driver

<!-- TODO
[![crates.io](https://img.shields.io/crates/v/ad983x.svg)](https://crates.io/crates/ad983x)
[![Docs](https://docs.rs/ad983x/badge.svg)](https://docs.rs/ad983x)
-->
[![Build Status](https://travis-ci.org/eldruin/ad983x-rs.svg?branch=master)](https://travis-ci.org/eldruin/ad983x-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/ad983x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/ad983x-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the AD9833 and AD9837 low-power programmable waveform generators / direct digital synthesizers (DDS) using the [`embedded-hal`] traits.

## The devices

The AD9833 and AD9837 are low power, programmable waveform generators capable of producing sine, triangular, and square wave outputs. Waveform generation is required in various types of sensing, actuation, and time domain reflectometry (TDR) applications. The output frequency and phase are software programmable, allowing easy tuning. No external components are needed. The frequency registers are 28 bits wide: with a 25 MHz clock rate, resolution of 0.1 Hz can be achieved; with a 1 MHz clock rate, the AD9833 can be tuned to 0.004 Hz resolution.

The AD9833 and AD9837 are written to via a 3-wire serial interface (SPI). This serial interface operates at clock rates up to 40 MHz and is compatible with DSP and microcontroller standards. The device operates with a power supply from 2.3 V to 5.5 V.

Datasheets:
- [AD9833](https://www.analog.com/media/en/technical-documentation/data-sheets/ad9833.PDF)
- [AD9837](https://www.analog.com/media/en/technical-documentation/data-sheets/AD9837.PDF)

Application Note:
- [Programming the AD9833/AD9834](https://www.analog.com/media/en/technical-documentation/application-notes/AN-1070.pdf)

<!-- ## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
TODO
```
-->

## Status

- [X] Compatible with AD9833
- [ ] Compatible with AD9837

## Support

For questions, issues, feature requests, and other changes, please file an
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
