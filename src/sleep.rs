use std::thread;
use std::time::Duration;

pub fn sleep(seconds: f64) {
    thread::sleep(Duration::from_secs_f64(seconds));
}