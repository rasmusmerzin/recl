use std::env::args;

mod record;
use record::record;

mod output;
use output::{comment, usage};

fn main() {
    let words: Vec<String> = args().collect();

    if let Some(operator) = words.get(1) {
        match operator.as_ref() {
            "record" => record(&words[2..], "log"),
            _ => {
                comment(&format!("unknown operator: {}", operator));
                usage();
            }
        }
    } else {
        usage();
    }
}
