use std::fs::write;

pub type Log = Vec<(u128, u8)>;

pub fn log_to_file(log: Log, filename: &str) -> std::io::Result<()> {
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

    write(filename, &output)
}
