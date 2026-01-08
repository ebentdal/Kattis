use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let a: f64 = input.trim().parse::<f64>().unwrap();

    let fence = 4.0 * a.sqrt();
    println!("{:.10}", fence);
}