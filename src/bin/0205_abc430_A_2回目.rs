use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let c: u32 = iter.next().unwrap().parse().unwrap();
    let d: u32 = iter.next().unwrap().parse().unwrap();

    if a <= c {
        if d < b {
             return println!("Yes");
        }
    }
    println!("No");
}
//0205_abc430_A_2回目