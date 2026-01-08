use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let month = it.next().unwrap();
    let date: i32 = it.next().unwrap().parse().unwrap();

    if (month == "OCT" && date == 31) || (month == "DEC" && date == 25) {
        println!("yup");
    } else {
        println!("nope");
    }
}