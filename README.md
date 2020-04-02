# epoch-cli

epoch-cli is a tool for working with epoch timestamps.

### Features

* Only UTC time is used
* Can work with units of seconds, milliseconds, microseconds, or nanoseconds
* Can convert epoch timestamps into dates and times
* Can convert dates and times into epoch timestamps

### Installing with cargo

1. [Install rust](https://www.rust-lang.org/tools/install)
2. Run `cargo install epoch-cli`

This will install a binary on your system named `epoch`.

### Displaying the current epoch time

```
$ epoch
1585796573
$ epoch --ms
1585796603436
$ epoch --us
1585796667156364
$ epoch --ns
1585796681774366974
```  

### Converting an epoch timestamp to a datetime

```
$ epoch 1585796573
2020-04-02 03:02:53 UTC
$ epoch --ms 1585796603436
2020-04-02 03:03:23.436 UTC
$ epoch --us 1585796667156364
2020-04-02 03:04:27.156364 UTC
$ epoch --ns 1585796681774366974
```

### Converting parts of a datetime into an epoch

The full usage looks like `epoch --dt year month day [hour] [minute] [second] [millisecond] [microsecond] [nanosecond]`

Only year, month, and day are required.

```
$ epoch --dt 2020 04 01 17 08 55 20 30 40
1585760935
$ epoch --ns --dt 2020 04 01 17 08 55 20 30 40
1585760935020030040
$ epoch --ms --dt 2020 04 01
1585699200000
$ epoch --dt 2020 04 01 23 00 30
1585782030
```


### Full usage

```
USAGE:
    epoch [FLAGS] [OPTIONS] [epoch]

FLAGS:
    -h, --help       Prints help information
        --us         Sets the time unit to microseconds
        --ms         Sets the time unit to milliseconds
        --ns         Sets the time unit to nanoseconds
    -V, --version    Prints version information

OPTIONS:
        --dt <year month day [hour] [minute] [s] [ms] [us] [ns]>
            Convert parts of a date and time into an epoch timestamp.


ARGS:
    <epoch>    An (optional) epoch of seconds, milliseconds, microseconds, or nanoseconds. When present, converts
               the epoch into an UTC datetime.
```
