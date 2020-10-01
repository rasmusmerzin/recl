use crate::log::{log_from_bytes, Log};
use std::io::{stderr, stdout, BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::thread::spawn;
use std::time::Instant;

pub fn record(cmd: &[String]) -> Result<Log, String> {
    match cmd.get(0) {
        Some(exe) => match Command::new(exe)
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => {
                let out = Mutex::new(child.stdout);
                let err = Mutex::new(child.stderr);

                let start = Instant::now();

                let mut handles = Vec::new();

                handles.push(spawn(move || {
                    let mut out = out.lock().unwrap();
                    if let Some(mut bytes) = out
                        .as_mut()
                        .map(|s| BufReader::new(s as &mut dyn Read).bytes())
                    {
                        Some(log_from_bytes(&mut bytes, &start, 1, &mut stdout()))
                    } else {
                        None
                    }
                }));

                handles.push(spawn(move || {
                    let mut err = err.lock().unwrap();
                    if let Some(mut bytes) = err
                        .as_mut()
                        .map(|s| BufReader::new(s as &mut dyn Read).bytes())
                    {
                        Some(log_from_bytes(&mut bytes, &start, 2, &mut stderr()))
                    } else {
                        None
                    }
                }));

                let mut log: Log = Vec::new();
                for handle in handles {
                    log.extend(handle.join().unwrap().unwrap());
                }
                log.sort_unstable_by_key(|entry| entry.timestamp);

                Ok(log)
            }
            Err(e) => Err(format!("{}", e)),
        },
        None => Err("No command given".to_string()),
    }
}
