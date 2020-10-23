use std::fs::{read, write};
use std::io::{BufReader, Read, Write};
use std::time::Instant;

pub struct LogEntry {
    pub channel: u8,
    pub timestamp: u64,
    pub bytes: Vec<u8>,
}

impl LogEntry {
    pub fn new(channel: u8, timestamp: u64, bytes: Vec<u8>) -> Self {
        Self {
            channel: channel,
            timestamp: timestamp,
            bytes: bytes,
        }
    }
}

pub type Log = Vec<LogEntry>;

pub fn log_from_file(filename: &str) -> Result<Log, String> {
    match read(filename) {
        Ok(bytes) => {
            let mut log: Log = Vec::new();
            for line in String::from_utf8_lossy(&bytes).lines() {
                let mut points = line.split_whitespace();
                if let Some(first) = points.next() {
                    if let Ok(channel) = first.parse::<u8>() {
                        if let Some(second) = points.next() {
                            if let Ok(timestamp) = second.parse::<u64>() {
                                let mut entry = LogEntry::new(channel, timestamp, Vec::new());
                                while let Some(next) = points.next() {
                                    if let Ok(ch) = next.parse::<u8>() {
                                        entry.bytes.push(ch);
                                    }
                                }
                                log.push(entry);
                            }
                        }
                    }
                }
            }
            Ok(log)
        }
        Err(e) => Err(format!("{}", e)),
    }
}

pub fn log_to_file(log: &Log, filename: &str) -> std::io::Result<()> {
    let mut output = String::new();

    for entry in log {
        output.push_str(&format!(
            "{} {} {}\n",
            entry.channel,
            entry.timestamp,
            entry
                .bytes
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }

    // prompt overwrite if file exists
    write(filename, &output)
}

pub fn log_bytes(from: &mut dyn Read, to: &mut dyn Write, start: &Instant, channel: u8) -> Log {
    let mut last_ts = 0u64;
    let mut log: Log = Vec::new();

    for b in BufReader::new(from).bytes() {
        if let Ok(b) = b {
            let timestamp = start.elapsed().as_millis() as u64;
            if timestamp == last_ts {
                match log.last_mut() {
                    Some(entry) => entry.bytes.push(b),
                    None => log.push(LogEntry::new(channel, timestamp, vec![b])),
                }
            } else {
                log.push(LogEntry::new(channel, timestamp, vec![b]));
            }
            let _ = to.write(&[b]);
            let _ = to.flush();
            last_ts = timestamp;
        }
    }

    log
}
