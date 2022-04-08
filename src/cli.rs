//! The construction of the CLI.

use clap::Parser;
use epoch_cli::errors::{EpochError, Result};
use epoch_cli::{DateTimeParts, Epoch};
use std::convert::TryFrom;

/// Defines the CLI through clap derive.
#[derive(Parser)]
#[clap(author, version, about, long_about = None, allow_negative_numbers = true)]
pub struct Cli {
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

/// Available epoch time bases
#[derive(Debug)]
enum Unit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl Unit {
    /// Extracts which time base to use from the CLI. Defaults to seconds.
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

/// Displays the epoch timestamp from a given Epoch and its unit.
/// If no epoch is provided, the current date/time is used.
fn display_epoch(epoch: Option<Epoch>, unit: &Unit) -> Result<()> {
    let epoch = epoch.unwrap_or_default();
    let epoch = match unit {
        Unit::Seconds => epoch.epoch_s() as i128,
        Unit::Milliseconds => epoch.epoch_ms()?,
        Unit::Microseconds => epoch.epoch_us()?,
        Unit::Nanoseconds => epoch.epoch_ns(),
    };
    println!("{}", epoch);
    Ok(())
}

/// Converts and displays the datetime from a given epoch timestamp and its unit.
fn display_datetime(epoch: i128, unit: &Unit) -> Result<()> {
    let epoch = match unit {
        Unit::Seconds => Epoch::from_epoch_s(i64::try_from(epoch)?)?,
        Unit::Milliseconds => Epoch::from_epoch_ms(epoch)?,
        Unit::Microseconds => Epoch::from_epoch_us(epoch)?,
        Unit::Nanoseconds => Epoch::from_epoch_ns(epoch)?,
    };
    println!("{}", epoch.fmt());

    Ok(())
}

/// Runs the CLI.
pub fn run(cli: Cli) -> Result<()> {
    let unit = Unit::from_cli(&cli);

    if let Some(epoch) = cli.epoch {
        // Epoch was provided. Convert and display date/time.
        display_datetime(epoch, &unit)?;
    } else if let Some(date_time_parts) = cli.date_time_parts {
        // Date/time parts were provided. Convert to epoch and display.
        let parts = DateTimeParts {
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
        // Nothing was provided, display current epoch.
        display_epoch(None, &unit)?;
    }

    Ok(())
}

/// Attempts to extract a specific value from the date/time parts. If the particular part is not
/// present, an EpochError is returned.
fn get(values: &Vec<i64>, idx: usize) -> Result<i64> {
    let res = values.get(idx).ok_or(EpochError {
        err: format!(
            "idx={} for parts is out of bounds=[0,{})",
            idx,
            values.len()
        ),
    })?;

    Ok(*res)
}
