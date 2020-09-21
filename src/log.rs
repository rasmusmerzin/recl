use std::fs::{read, write};

pub type LogEntry = (u64, Vec<u8>);
pub type Log = Vec<LogEntry>;

pub fn log_from_file(filename: &str) -> Result<Log, String> {
    match read(filename) {
        Ok(bytes) => {
            let mut log: Log = Vec::new();
            for line in String::from_utf8_lossy(&bytes).lines() {
                let mut points = line.split_whitespace();
                if let Some(first) = points.next() {
                    if let Ok(ts) = first.parse::<u64>() {
                        let mut entry: LogEntry = (ts, Vec::new());
                        while let Some(next) = points.next() {
                            if let Ok(ch) = next.parse::<u8>() {
                                entry.1.push(ch);
                            }
                        }
                        log.push(entry);
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

    for (ts, chs) in log {
        output.push_str(&format!(
            "{} {}\n",
            ts,
            chs.iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }

    // prompt overwrite if file exists
    write(filename, &output)
}
