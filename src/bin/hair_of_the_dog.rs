use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n : usize = it.next().unwrap().parse().unwrap();

    let mut prev = it.next().unwrap();
    let mut hungover = 0;

    for _ in 1..n{
        let cur = it.next().unwrap();
        if prev == "drunk" && cur == "sober" {
            hungover += 1;
        } 
        prev = cur;
    }
    println!("{}", hungover);
}