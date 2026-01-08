use std::io::{self,Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n : usize = it.next().unwrap().parse().unwrap();

    let mut best : Option<u64> = None;

    for _ in 0..n{
        let m : u64 = it.next().unwrap().parse().unwrap();
        let o : u8 = it.next().unwrap().parse().unwrap();

        if o == 0 {
            best = match best {
                None => Some(m),
                Some(cur) => Some(cur.min(m)),
            };
        }
    }
    match best {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    };
}