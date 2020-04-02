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
