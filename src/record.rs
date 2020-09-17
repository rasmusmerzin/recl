use crate::output::comment;
use std::fs::write;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn record(cmd: &[String], filename: &str) {
    match cmd.get(0) {
        Some(exe) => match Command::new(exe)
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stream) = child.stdout.as_mut() {
                    let mut log: Vec<(u128, u8)> = Vec::new();
                    let start = Instant::now();
                    let bytes = BufReader::new(stream).bytes();

                    for b in bytes {
                        if let Ok(b) = b {
                            log.push((start.elapsed().as_millis(), b));
                            print!("{}", b as char);
                        }
                    }

                    child.wait().unwrap();

                    let mut output = String::new();
                    let mut last_ts = 0u128;
                    for (ts, ch) in log {
                        if !output.is_empty() {
                            if ts == last_ts {
                                output.push_str(&format!(" {}", ch));
                            } else {
                                output.push_str(&format!("\n{} {}", ts, ch));
                            }
                        } else {
                            output.push_str(&format!("{} {}", ts, ch));
                        }
                        last_ts = ts;
                    }
                    output.push_str("\n");

                    match write(filename, &output) {
                        Ok(_) => comment("ok"),
                        Err(e) => comment(&format!("error: {}", e)),
                    }
                } else {
                    comment("error: no stream");
                    child.kill().unwrap();
                }
            }
            Err(e) => comment(&format!("error: {}", e)),
        },
        None => comment("error: no command"),
    }
}
