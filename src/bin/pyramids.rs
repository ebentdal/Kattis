use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n : i64 = input.trim().parse().unwrap();

    let mut used : i64 = 0;
    let mut side : i64 = 1;
    let mut height : i64 = 0;

    while used + (side * side) <= n {
        used += side * side;
        side += 2;
        height += 1;
    }
    println!("{}", height);
}