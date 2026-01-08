use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n :i32 = input.trim().parse().unwrap();

    let binary = format!("{:b}", n);
    let reversed : String = binary.chars().rev().collect();

    let result = u32::from_str_radix(&reversed, 2).unwrap();

    println!("{}", result);
}
