use std::env::args;

pub mod log;
use log::{log_from_file, log_to_file, Log};

mod record;
use record::record;

mod play;
use play::play;

mod output;
use output::{comment, usage};

fn main() {
    let words: Vec<String> = args().collect();

    if let Some(operator) = words.get(1) {
        match operator.as_ref() {
            "r" | "record" => match words.get(2) {
                Some(filename) => match record(&words[3..]) {
                    Ok(log) => match log_to_file(&log, filename) {
                        Ok(_) => comment("ok"),
                        Err(e) => comment(&format!("error: {}", e)),
                    },
                    Err(e) => comment(&format!("error: {}", e)),
                },
                None => usage(),
            },
            "p" | "play" => match words.get(2) {
                Some(filename) => match log_from_file(filename) {
                    Ok(log) => play(&log),
                    Err(e) => comment(&format!("error: {}", e)),
                },
                None => usage(),
            },
            "-v" | "--version" => println!(
                "{} version {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            _ => {
                comment(&format!("unknown operator: {}", operator));
                usage();
            }
        }
    } else {
        usage();
    }
}
