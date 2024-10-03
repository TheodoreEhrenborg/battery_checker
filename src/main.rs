use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use cmd_lib::*;
use regex::Regex;

fn main() {
    match loop_and_watch() {
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

fn loop_and_watch() -> Result<()> {
    let re = Regex::new(r"([0-9]{1,3})%")?;
    let one_sec = std::time::Duration::from_secs(1);
    loop {
        let foo = run_fun!(acpi -b)?;
        let captures = re.captures(&foo).ok_or(anyhow!("regex failed"))?;
        let length = captures.len();
        if length != 2 {
            bail!("len(captures) = {}", length);
        }
        let correct_capture = &captures[1];
        let battery_int = correct_capture.parse::<i32>()?;
        if battery_int < 30 {
            // Note this is in milliseconds
            run_cmd!(notify-send -u critical -t 1000 "Battery level $battery_int")?;
        }
        if battery_int < 40 && battery_int >= 30 {
            run_cmd!(notify-send -t 1000 "Battery level $battery_int")?;
        }
        std::thread::sleep(one_sec);
    }
}
