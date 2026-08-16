use std::io::{self, Read};

fn eat_chocolate() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut h: u32 = iter.next().unwrap().parse().unwrap();
    let mut w: u32 = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..q{
        let c: u32 = iter.next().unwrap().parse().unwrap();
        let r: u32 = iter.next().unwrap().parse().unwrap();

        match c {
            1 => {
                println!("{}", w * r);
                h = h - r;
            }
            2 => {
                println!("{}", h * r);
                w = w - r;
            }
            _ => (),
        }
    }
}

fn main() {
    eat_chocolate();
}
//0144_abc449_B_2回目