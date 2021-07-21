use std::thread;
use std::time::Duration;

use chrono::{Local, Utc};
use clokwerk::Scheduler;
use clokwerk::Interval::*;

pub fn schedule() {
    let mut scheduler = Scheduler::with_tz(Utc);

    scheduler.every(
        Seconds(60)
    )
        .run(
            || {
                println!("Current time is {}", Local::now());
            }
        );

    thread::spawn(
        move || {
            loop {
                scheduler.run_pending();
                thread::sleep(Duration::from_millis(1000))
            }
        }
    );
}