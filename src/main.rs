use anyhow::{anyhow, bail, Result};
use clap::Parser;
use cmd_lib::*;
use regex::Regex;

/// Battery level checker with configurable thresholds
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Critical battery threshold (percentage)
    #[clap(short, long, default_value = "30")]
    critical_threshold: i32,

    /// Warning battery threshold (percentage)
    #[clap(short, long, default_value = "40")]
    warning_threshold: i32,

    /// Upper battery threshold (percentage)
    #[clap(short, long, default_value = "100")]
    upper_threshold: i32,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    match loop_and_watch(args.critical_threshold, args.warning_threshold, args.upper_threshold) {
        Ok(_) => {
            println!("Shouldn't have gotten here");
            run_cmd!(notify-send -t 0 "battery checker: shouldn't have gotten here").unwrap()
        }
        Err(err) => {
            println!("{}", err);
            run_cmd!(notify-send -t 0 "battery checker died with an error").unwrap()
        }
    }
}

fn loop_and_watch(critical_threshold: i32, warning_threshold: i32, upper_threshold: i32) -> Result<()> {
    let re = Regex::new(r"([0-9]{1,3})%")?;
    let ten_seconds = std::time::Duration::from_secs(10);
    loop {
        let acpi_output = run_fun!(acpi -b)?;
        let captures = re.captures(&acpi_output).ok_or(anyhow!("regex failed"))?;
        let length = captures.len();
        if length != 2 {
            bail!("len(captures) = {}, should be 2", length);
        }
        let battery_int = captures[1].parse::<i32>()?;
        if battery_int < critical_threshold {
            // Note this is in milliseconds
            run_cmd!(notify-send -u critical -t 9000 "Battery level $battery_int")?;
        }
        if battery_int < warning_threshold && battery_int >= critical_threshold {
            run_cmd!(notify-send -t 9000 "Battery level $battery_int")?;
        }
        if battery_int > upper_threshold {
            run_cmd!(notify-send -t 9000 "Battery level $battery_int")?;
        }
        std::thread::sleep(ten_seconds);
    }
}
