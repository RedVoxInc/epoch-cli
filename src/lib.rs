//! epoch-cli is a tool for working with epoch timestamps.
//!
//! ### Features
//!
//! * Incredibly fast with low resource usage
//! * Only UTC time is used
//! * Can work with units of seconds, milliseconds, microseconds, or nanoseconds
//! * Can convert epoch timestamps into dates and times
//! * Can convert dates and times into epoch timestamps
//! * Inspired by [https://www.epochconverter.com/](https://docs.rs/epoch-cli)
//!
//! ### Documentation
//!
//! Full documentation, instructions, and API are available at: [https://docs.rs/epoch-cli](https://docs.rs/epoch-cli)
//!
//! ### Installing with cargo
//!
//! 1. [Install rust](https://www.rust-lang.org/tools/install)
//! 2. Run `cargo install epoch-cli`
//!
//! This will install a binary on your system named `epoch`.
//!
//! ### Displaying the current epoch time
//!
//! ```text
//! $ epoch
//! 1585796573
//!
//! $ epoch --ms
//! 1585796603436
//!
//! $ epoch --us
//! 1585796667156364
//!
//! $ epoch --ns
//! 1585796681774366974
//!```
//!
//! ### Converting an epoch timestamp to a datetime
//!
//! ```text
//! $ epoch 1585796573
//! 2020-04-02 03:02:53 UTC
//!
//! $ epoch --ms 1585796603436
//! 2020-04-02 03:03:23.436 UTC
//!
//! $ epoch --us 1585796667156364
//! 2020-04-02 03:04:27.156364 UTC
//!
//! $ epoch --ns 1585796681774366974
//! 2020-04-02 03:04:41.774366974 UTC
//!```
//!
//! ### Converting parts of a datetime into an epoch
//!
//! The full usage looks like `epoch --dt year month day [hour] [minute] [second] [millisecond] [microsecond] [nanosecond]`
//!
//! Only year, month, and day are required.
//!
//! ```text
//! $ epoch --dt 2020 04 01 17 08 55 20 30 40
//! 1585760935
//!
//! $ epoch --ns --dt 2020 04 01 17 08 55 20 30 40
//! 1585760935020030040
//!
//! $ epoch --ms --dt 2020 04 01
//! 1585699200000
//!
//! $ epoch --dt 2020 04 01 23 00 30
//! 1585782030
//!```
//!
//! ### Full usage
//!
//! ```text
//! USAGE:
//! epoch [FLAGS] [OPTIONS] [epoch]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!         --us         Sets the time unit to microseconds
//!         --ms         Sets the time unit to milliseconds
//!         --ns         Sets the time unit to nanoseconds
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!         --dt <year month day [hour] [minute] [s] [ms] [us] [ns]>
//!             Convert parts of a date and time into an epoch timestamp.
//!
//!
//! ARGS:
//!     <epoch>    An (optional) epoch of seconds, milliseconds, microseconds, or nanoseconds.
//!                When present, converts the epoch into an UTC datetime.
//! ```

use crate::errors::{EpochError, Result};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

mod conversions;
pub mod errors;

#[derive(Debug, Default)]
pub struct Parts {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub microsecond: u32,
    pub nanosecond: u32,
}

#[derive(Debug)]
pub struct Epoch {
    pub datetime: OffsetDateTime,
}

impl TryFrom<Parts> for Epoch {
    type Error = EpochError;

    fn try_from(parts: Parts) -> Result<Self> {
        let total_ns =
            (parts.millisecond * 1_000_000) + (parts.microsecond * 1_000) + parts.nanosecond;
        let datetime = PrimitiveDateTime::new(
            Date::from_calendar_date(parts.year, parse_month(parts.month)?, parts.day)?,
            Time::from_hms_nano(parts.hour, parts.minute, parts.second, total_ns)?,
        )
        .assume_utc();

        Ok(Epoch::new(datetime))
    }
}

fn parse_month(value: u8) -> Result<Month> {
    match value {
        1 => Ok(Month::January),
        2 => Ok(Month::February),
        3 => Ok(Month::March),
        4 => Ok(Month::April),
        5 => Ok(Month::May),
        6 => Ok(Month::June),
        7 => Ok(Month::July),
        8 => Ok(Month::August),
        9 => Ok(Month::September),
        10 => Ok(Month::October),
        11 => Ok(Month::November),
        12 => Ok(Month::December),
        _ => Err(EpochError::new(
            "Illegal month integral. Only [1-12] are valid.",
        )),
    }
}

impl Epoch {
    pub const fn new(datetime: OffsetDateTime) -> Epoch {
        Epoch { datetime }
    }

    pub fn from_parts(parts: Parts) -> Result<Epoch> {
        parts.try_into()
    }

    pub fn from_epoch_s(epoch_s: i64) -> Result<Epoch> {
        let datetime = OffsetDateTime::from_unix_timestamp(epoch_s)?;
        Ok(Epoch::new(datetime))
    }

    pub fn from_epoch_ms(epoch_ms: i128) -> Result<Epoch> {
        Epoch::from_epoch_ns(epoch_ms * 1_000_000)
    }

    pub fn from_epoch_us(epoch_us: i128) -> Result<Epoch> {
        Epoch::from_epoch_ns(epoch_us * 1_000)
    }

    pub fn from_epoch_ns(epoch_ns: i128) -> Result<Epoch> {
        let datetime = OffsetDateTime::from_unix_timestamp_nanos(epoch_ns)?;
        Ok(Epoch::new(datetime))
    }

    pub fn epoch_s(&self) -> i64 {
        self.datetime.unix_timestamp()
    }

    pub fn epoch_ms(&self) -> i128 {
        self.epoch_ns() / 1_000_000
    }

    pub fn epoch_us(&self) -> i128 {
        self.epoch_ns() / 1_000
    }

    pub fn epoch_ns(&self) -> i128 {
        self.datetime.unix_timestamp_nanos()
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            datetime: OffsetDateTime::now_utc(),
        }
    }
}
