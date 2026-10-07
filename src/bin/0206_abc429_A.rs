use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let m: u32 = iter.next().unwrap().parse().unwrap();

    if n > m {
        for _ in 0..m {
            println!("OK");
        }

            for _ in 0..(n - m) {
            println!("Too Many Requests");
        }
        return;
    } else {
        for _ in 0..n {
            println!("OK");
        }
    }
}
//0206_abc429_A