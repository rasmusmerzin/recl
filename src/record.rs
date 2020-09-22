use crate::{Log, LogEntry};
use std::io::{stdout, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn record(cmd: &[String]) -> Result<Log, String> {
    match cmd.get(0) {
        Some(exe) => match Command::new(exe)
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            // .stdin(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stream) = child.stdout.as_mut() {
                    let mut log: Log = Vec::new();
                    let start = Instant::now();
                    let bytes = BufReader::new(stream).bytes();

                    let mut last_ts = 0u64;

                    for b in bytes {
                        if let Ok(b) = b {
                            let timestamp = start.elapsed().as_millis() as u64;
                            if timestamp == last_ts {
                                match log.last_mut() {
                                    Some(entry) => entry.bytes.push(b),
                                    None => log.push(LogEntry::new(1, timestamp, vec![b])),
                                }
                            } else {
                                log.push(LogEntry::new(1, timestamp, vec![b]));
                            }
                            last_ts = timestamp;

                            let _ = stdout().write(&[b]);
                            let _ = stdout().flush();
                        }
                    }

                    child.wait().unwrap();
                    Ok(log)
                } else {
                    child.kill().unwrap();
                    Err("Could not attach stream".to_string())
                }
            }
            Err(e) => Err(format!("{}", e)),
        },
        None => Err("No command given".to_string()),
    }
}
