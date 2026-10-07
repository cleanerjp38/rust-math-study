use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let s: u32 = iter.next().unwrap().parse().unwrap();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    let x: u32 = iter.next().unwrap().parse().unwrap();

    let sequences: u32 = x / (a + b);
    let last_time: u32 = x % (a + b);

    let mut ans = s * sequences * a;

    if last_time <= a {
        ans += s * last_time;
    } else {
        ans += s * a;
    }

    println!("{}", ans);
}
//0207_abc428_A_2回目