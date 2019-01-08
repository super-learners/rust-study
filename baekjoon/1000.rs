use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("{}", input.trim().split_whitespace().filter_map(|s| s.parse::<i32>().ok()).sum::<i32>());
}