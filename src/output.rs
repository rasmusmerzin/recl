pub fn comment(msg: &str) {
    println!("{}: {}", env!("CARGO_PKG_NAME"), msg);
}

pub fn usage() {
    let name = env!("CARGO_PKG_NAME");
    println!(
        "usage: {} r|record <file> <command>\n       {} p|play <file>",
        name, name
    );
}
