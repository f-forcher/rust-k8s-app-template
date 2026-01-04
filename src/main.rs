use chrono::{SecondsFormat, Utc};
use std::thread;
use std::time::Duration;

fn main() {
    let mut counter: u64 = 0;

    loop {
        thread::sleep(Duration::from_secs(1));
        counter += 1;
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        println!(r#"{{"timestamp":"{}","counter":"{}"}}"#, timestamp, counter);
    }
}
