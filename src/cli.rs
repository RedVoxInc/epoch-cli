use clap::Parser;
use epoch_cli::errors::{EpochError, Result};
use epoch_cli::{Epoch, Parts};
use std::convert::TryFrom;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, allow_negative_numbers = true)]
struct Cli {
    #[clap(
        help = "An (optional) epoch of seconds, milliseconds, microseconds, or nanoseconds. When present, converts the epoch into an UTC datetime."
    )]
    pub epoch: Option<i128>,

    #[clap(long = "ms", help = "Sets the time unit to milliseconds")]
    pub milliseconds: bool,

    #[clap(
        long = "us",
        conflicts_with = "milliseconds",
        help = "Sets the time unit to microseconds"
    )]
    pub microseconds: bool,

    #[clap(
        long = "ns",
        conflicts_with = "microseconds",
        conflicts_with = "milliseconds",
        help = "Sets the time unit to nanoseconds"
    )]
    pub nanoseconds: bool,

    #[clap(
        long = "dt",
        conflicts_with = "epoch",
        min_values = 3,
        max_values = 9,
        value_name = "year month day [hour] [minute] [s] [ms] [us] [ns]",
        help = "Convert parts of a date and time into an epoch timestamp."
    )]
    pub date_time_parts: Option<Vec<i64>>,
}

#[derive(Debug)]
enum Unit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl Unit {
    const fn from_cli(cli: &Cli) -> Unit {
        if cli.milliseconds {
            Unit::Milliseconds
        } else if cli.microseconds {
            Unit::Microseconds
        } else if cli.nanoseconds {
            Unit::Nanoseconds
        } else {
            Unit::Seconds
        }
    }
}

fn display_epoch(epoch: Option<Epoch>, unit: &Unit) -> Result<()> {
    let epoch = epoch.unwrap_or_default();
    let es = match unit {
        Unit::Seconds => epoch.epoch_s() as i128,
        Unit::Milliseconds => epoch.epoch_ms()?,
        Unit::Microseconds => epoch.epoch_us()?,
        Unit::Nanoseconds => epoch.epoch_ns(),
    };
    println!("{}", es);
    Ok(())
}

fn display_datetime(epoch: i128, unit: &Unit) -> Result<()> {
    let epoch = match unit {
        Unit::Seconds => Epoch::from_epoch_s(i64::try_from(epoch)?)?,
        Unit::Milliseconds => Epoch::from_epoch_ms(epoch)?,
        Unit::Microseconds => Epoch::from_epoch_us(epoch)?,
        Unit::Nanoseconds => Epoch::from_epoch_ns(epoch)?,
    };
    println!("{}", epoch.datetime);

    Ok(())
}

pub fn run() -> Result<()> {
    let cli: Cli = Cli::parse();
    let unit = Unit::from_cli(&cli);

    if let Some(epoch) = cli.epoch {
        display_datetime(epoch, &unit)?;
    } else if let Some(date_time_parts) = cli.date_time_parts {
        let parts = Parts {
            year: i32::try_from(get(&date_time_parts, 0)?)?,
            month: u8::try_from(get(&date_time_parts, 1)?)?,
            day: u8::try_from(get(&date_time_parts, 2)?)?,
            hour: u8::try_from(*date_time_parts.get(3).unwrap_or(&0))?,
            minute: u8::try_from(*date_time_parts.get(4).unwrap_or(&0))?,
            second: u8::try_from(*date_time_parts.get(5).unwrap_or(&0))?,
            millisecond: u32::try_from(*date_time_parts.get(6).unwrap_or(&0))?,
            microsecond: u32::try_from(*date_time_parts.get(7).unwrap_or(&0))?,
            nanosecond: u32::try_from(*date_time_parts.get(8).unwrap_or(&0))?,
        };
        let epoch: Epoch = parts.try_into()?;
        display_epoch(Some(epoch), &unit)?;
    } else {
        display_epoch(None, &unit)?;
    }

    Ok(())
}

fn get(values: &Vec<i64>, idx: usize) -> Result<i64> {
    let res = values
        .get(idx)
        .ok_or(EpochError::new("idx for part is out of bounds"))?;

    Ok(*res)
}
