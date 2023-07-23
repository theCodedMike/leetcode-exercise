fn main() {
    println!("Hello, world!");
    let arrs = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let new = arrs.into_iter().flatten().collect::<Vec<i32>>();
    println!("{:?}", new);
}
