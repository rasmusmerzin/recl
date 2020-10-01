use crate::Log;
use std::io::{stderr, stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn play(log: &Log) {
    let start = Instant::now();
    for entry in log {
        let ts = start.elapsed().as_millis() as u64;
        if ts < entry.timestamp {
            sleep(Duration::from_millis(entry.timestamp - ts));
        }
        let mut write: Box<dyn Write>;
        match &entry.channel {
            2 => write = Box::new(stderr()),
            _ => write = Box::new(stdout()),
        }
        let _ = write.write(&entry.bytes);
        let _ = write.flush();
    }
}
