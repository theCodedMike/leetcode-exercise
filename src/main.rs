fn main() {
    println!("Hello, world!");
    let mut ori = vec![1, 2, 3, 4];
    let len = ori.len() - 1;

    for _ in 0..=len {
        print!("  {:?}", ori);
        for i in 0..=len {
            for j in (i + 1)..=len {
                let mut clone = ori.clone();
                clone.swap(i, j);
                print!("  {:?}", clone);
            }
        }
        println!();
        ori.rotate_left(1);
    }
}
