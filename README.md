# Timelane

[<img alt="github" src="https://img.shields.io/badge/github-kmichel/timelane-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/kmichel/timelane)
[<img alt="crates.io" src="https://img.shields.io/crates/v/timelane.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timelane)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timelane-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timelane)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/kmichel/timelane/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/kmichel/timelane/actions?query=branch%3Amain)

Convert between nanoseconds, seconds, days, years... in `const`.

The `timelane` crate defines lanes of time where each lane works at a
different scale: yearly, monthly, daily, etc. Each lane has sequentially
numbered marks. The crate implements scaler functions to convert a mark in
one lane into a mark in another lane.

For instance, mark 1 in the month lane is mark 1 in the day lane, mark 2 in
the month lane is mark 32 in the day lane.

`Mark` values are signed integers.

`Scaler` functions can be composed to convert `Mark` from non-adjacent
lanes, like converting a year mark into a second mark.

A mark from a small scale lane can also be converted to a mark in a larger
scale lane, like second to year, but you need to chose between rounding
down or rounding up. Scaling a second mark to a year mark and then back to
a second mark will give the beginning or the end of the year, depending on
the rounding mode.


All `Scaler` functions are `const` functions. Leap seconds are statically
defined.

This library will return incorrect results if the International Earth
Rotation and Reference Systems Service declares a new leap second.

However, the last leap second was in 2017 and the General Conference on
Weights and Measures resolved to eliminate leap seconds by or before 2035.

 - Years before 1AD use the astronomical year numbering: the year 1BC is
   mark `0`, 2BC is `-1`...
 - Leap years follow the proleptic gregorian calendar, and are defined even
   for negative years: 1BC is a leap year.
 - Because of leap seconds, some minutes will contain 61 seconds.
 - Before using this library, make sure you actually want to work with UTC.
   Many systems use GPS or TAI, which do not include leap seconds.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
