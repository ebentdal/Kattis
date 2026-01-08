use std::io::{self,Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();

    let doctor = lines.next().unwrap().trim();
    let jon = lines.next().unwrap().trim();

    if doctor.len() < jon.len() {
        println!("no");
    } else {
        println!("go");
    }
}

