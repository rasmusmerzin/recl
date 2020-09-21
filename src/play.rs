use crate::Log;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn play(log: &Log) {
    let start = Instant::now();
    for entry in log {
        let ts = start.elapsed().as_millis() as u64;
        if ts < entry.0 {
            sleep(Duration::from_millis(entry.0 - ts));
        }
        let _ = stdout().write(&entry.1);
        let _ = stdout().flush();
    }
}
