use std::ops::Index;

fn main() {
    println!("Hello, world!");
    let s = "leetcode";
    let sub = s.index(8..);
    println!("{} {}", sub, sub.is_empty());
}
