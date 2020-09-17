pub fn comment(msg: &str) {
    println!("{}: {}", env!("CARGO_PKG_NAME"), msg);
}

pub fn usage() {
    let name = env!("CARGO_PKG_NAME");
    println!(
        "USAGE\n  {} record <command>\n  {} play <recording>",
        name, name
    );
}
