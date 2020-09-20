use crate::Log;
use std::io::{stdout, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn record(cmd: &[String]) -> Result<Log, String> {
    match cmd.get(0) {
        Some(exe) => match Command::new(exe)
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stream) = child.stdout.as_mut() {
                    let mut log: Log = Vec::new();
                    let start = Instant::now();
                    let bytes = BufReader::new(stream).bytes();

                    for b in bytes {
                        if let Ok(b) = b {
                            log.push((start.elapsed().as_millis(), b));
                            print!("{}", b as char);
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
