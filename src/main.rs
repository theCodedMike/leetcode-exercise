fn main() {
    println!("Hello, world!");
    let s = "";
    let x = s.contains(|c| !('0' <= c && c <= '9' || c == '+' || c == '-'));
    println!("{}", x);
}
