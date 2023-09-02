use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, world!");
    let now = SystemTime::now();
    let diff = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    println!("{}", diff.as_nanos());
}
