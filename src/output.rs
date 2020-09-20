pub fn comment(msg: &str) {
    println!("{}: {}", env!("CARGO_PKG_NAME"), msg);
}

pub fn usage() {
    let name = env!("CARGO_PKG_NAME");
    println!(
        "USAGE\n  {} record <file> <command>\n  {} play <file>",
        name, name
    );
}
