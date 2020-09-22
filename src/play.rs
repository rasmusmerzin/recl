use crate::Log;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn play(log: &Log) {
    let start = Instant::now();
    for entry in log {
        let ts = start.elapsed().as_millis() as u64;
        if ts < entry.timestamp {
            sleep(Duration::from_millis(entry.timestamp - ts));
        }
        let _ = stdout().write(&entry.bytes);
        let _ = stdout().flush();
    }
}
