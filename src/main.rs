extern crate chrono;
extern crate chrono_tz;

use chrono::{DateTime, Utc};
use chrono_tz::Europe::London;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    let uk = now.with_timezone(&London);
    let formatted = uk.format("It's %-l o'clock (%H:00)");

    println!("{}", formatted);
}
