use std::env::args;

mod log;
use log::{log_to_file, Log};

mod record;
use record::record;

mod output;
use output::{comment, usage};

fn main() {
    let words: Vec<String> = args().collect();

    if let Some(operator) = words.get(1) {
        match operator.as_ref() {
            "record" => match words.get(2) {
                Some(file) => match record(&words[3..]) {
                    Ok(log) => match log_to_file(log, file) {
                        Ok(_) => comment("ok"),
                        Err(e) => comment(&format!("error: {}", e)),
                    },
                    Err(e) => comment(&format!("error: {}", e)),
                },
                None => usage(),
            },
            _ => {
                comment(&format!("unknown operator: {}", operator));
                usage();
            }
        }
    } else {
        usage();
    }
}
