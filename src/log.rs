use std::fs::{read, write};

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
