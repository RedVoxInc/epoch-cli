use clap::{crate_version, load_yaml, value_t, values_t, App, ArgMatches};

use epoch_cli::{Epoch, Parts};

#[derive(Debug)]
enum Unit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl Unit {
    fn from_matches(matches: &ArgMatches) -> Unit {
        if matches.is_present("milliseconds") {
            Unit::Milliseconds
        } else if matches.is_present("microseconds") {
            Unit::Microseconds
        } else if matches.is_present("nanoseconds") {
            Unit::Nanoseconds
        } else {
            Unit::Seconds
        }
    }
}

fn display_epoch(epoch: Option<Epoch>, unit: &Unit) {
    let epoch = epoch.unwrap_or_default();
    let es = match unit {
        Unit::Seconds => epoch.epoch_s(),
        Unit::Milliseconds => epoch.epoch_ms(),
        Unit::Microseconds => epoch.epoch_us(),
        Unit::Nanoseconds => epoch.epoch_ns(),
    };
    println!("{}", es);
}

fn display_datetime(epoch: i64, unit: &Unit) {
    let epoch = match unit {
        Unit::Seconds => Epoch::from_epoch_s(epoch),
        Unit::Milliseconds => Epoch::from_epoch_ms(epoch),
        Unit::Microseconds => Epoch::from_epoch_us(epoch),
        Unit::Nanoseconds => Epoch::from_epoch_ns(epoch),
    };
    println!("{}", epoch.datetime)
}

pub fn run_cli() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    let unit = Unit::from_matches(&matches);

    if matches.is_present("epoch") {
        let epoch: i64 = value_t!(matches, "epoch", i64).unwrap();
        display_datetime(epoch, &unit);
    } else if matches.is_present("date_time_parts") {
        let parts_vec: Vec<u32> = values_t!(matches, "date_time_parts", u32).unwrap();
        let parts = Parts {
            year: parts_vec[0] as i32,
            month: parts_vec[1],
            day: parts_vec[2],
            hour: *parts_vec.get(3).unwrap_or(&0),
            minute: *parts_vec.get(4).unwrap_or(&0),
            second: *parts_vec.get(5).unwrap_or(&0),
            millisecond: *parts_vec.get(6).unwrap_or(&0),
            microsecond: *parts_vec.get(7).unwrap_or(&0),
            nanosecond: *parts_vec.get(8).unwrap_or(&0),
        };
        let epoch: Epoch = parts.into();
        display_epoch(Some(epoch), &unit);
    } else {
        display_epoch(None, &unit);
    }
}
