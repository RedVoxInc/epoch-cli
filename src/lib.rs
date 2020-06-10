//! # epoch-cli
//!
//! epoch-cli is a tool for working with epoch timestamps.
//!
//! ### Features
//!
//! * Only UTC time is used
//! * Can work with units of seconds, milliseconds, microseconds, or nanoseconds
//! * Can convert epoch timestamps into dates and times
//! * Can convert dates and times into epoch timestamps
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
//! ```
//! $ epoch
//! 1585796573
//! $ epoch --ms
//! 1585796603436
//! $ epoch --us
//! 1585796667156364
//! $ epoch --ns
//! 1585796681774366974
//! ```
//!
//! ### Converting an epoch timestamp to a datetime
//!
//! ```
//! $ epoch 1585796573
//! 2020-04-02 03:02:53 UTC
//! $ epoch --ms 1585796603436
//! 2020-04-02 03:03:23.436 UTC
//! $ epoch --us 1585796667156364
//! 2020-04-02 03:04:27.156364 UTC
//! $ epoch --ns 1585796681774366974
//! ```
//!
//! ### Converting parts of a datetime into an epoch
//!
//! The full usage looks like `epoch --dt year month day [hour] [minute] [second] [millisecond] [microsecond] [nanosecond]`
//!
//! Only year, month, and day are required.
//!
//! ```
//! $ epoch --dt 2020 04 01 17 08 55 20 30 40
//! 1585760935
//! $ epoch --ns --dt 2020 04 01 17 08 55 20 30 40
//! 1585760935020030040
//! $ epoch --ms --dt 2020 04 01
//! 1585699200000
//! $ epoch --dt 2020 04 01 23 00 30
//! 1585782030
//! ```
//!
//!
//! ### Full usage
//!
//! ```
//! USAGE:
//! epoch [FLAGS] [OPTIONS] [epoch]
//!
//! FLAGS:
//! -h, --help       Prints help information
//! --us         Sets the time unit to microseconds
//! --ms         Sets the time unit to milliseconds
//! --ns         Sets the time unit to nanoseconds
//! -V, --version    Prints version information
//!
//! OPTIONS:
//! --dt <year month day [hour] [minute] [s] [ms] [us] [ns]>
//! Convert parts of a date and time into an epoch timestamp.
//!
//!
//! ARGS:
//! <epoch>    An (optional) epoch of seconds, milliseconds, microseconds, or nanoseconds. When present, converts
//! the epoch into an UTC datetime.
//! ```

use chrono::{DateTime, Duration, TimeZone, Utc};

const NS_PER_US: u32 = 1_000;
const NS_PER_MS: u32 = 1_000_000;

#[inline]
fn us_to_ns(us: u32) -> u32 {
    us * NS_PER_US
}

#[inline]
fn ms_to_ns(ms: u32) -> u32 {
    ms * NS_PER_MS
}

#[derive(Debug, Default)]
pub struct Parts {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
    pub microsecond: u32,
    pub nanosecond: u32,
}

#[derive(Debug)]
pub struct Epoch {
    pub datetime: DateTime<Utc>,
}

impl From<Parts> for Epoch {
    fn from(parts: Parts) -> Self {
        let total_ns = parts.nanosecond + us_to_ns(parts.microsecond) + ms_to_ns(parts.millisecond);
        let datetime = Utc.ymd(parts.year, parts.month, parts.day).and_hms_nano(
            parts.hour,
            parts.minute,
            parts.second,
            total_ns,
        );
        Epoch::new(datetime)
    }
}

impl Epoch {
    pub fn new(datetime: DateTime<Utc>) -> Epoch {
        Epoch { datetime }
    }

    pub fn from_parts(parts: Parts) -> Epoch {
        parts.into()
    }

    pub fn from_epoch_s(epoch_s: i64) -> Epoch {
        let datetime = Utc.timestamp(epoch_s, 0);
        Epoch::new(datetime)
    }

    pub fn from_epoch_ms(epoch_ms: i64) -> Epoch {
        let datetime = Utc.timestamp_millis(epoch_ms);
        Epoch::new(datetime)
    }

    pub fn from_epoch_us(epoch_us: i64) -> Epoch {
        let epoch_ns = epoch_us * 1000;
        Epoch::from_epoch_ns(epoch_ns)
    }

    pub fn from_epoch_ns(epoch_ns: i64) -> Epoch {
        let datetime = Utc.timestamp_nanos(epoch_ns);
        Epoch::new(datetime)
    }

    pub fn epoch_s(&self) -> i64 {
        self.datetime.timestamp()
    }

    pub fn epoch_ms(&self) -> i64 {
        self.datetime.timestamp_millis()
    }

    pub fn epoch_us(&self) -> i64 {
        ((self.epoch_ns() as f64) / 1000.0).round() as i64
    }

    pub fn epoch_ns(&self) -> i64 {
        self.datetime.timestamp_nanos()
    }

    pub fn plus_duration(mut self, duration: Duration) -> Self {
        self.datetime = self.datetime + duration;
        self
    }

    pub fn minus_duration(mut self, duration: Duration) -> Self {
        self.datetime = self.datetime - duration;
        self
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            datetime: Utc::now(),
        }
    }
}
