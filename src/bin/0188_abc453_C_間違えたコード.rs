use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut count = 0;
    let mut start: f64 = 0.5;

    for _ in 0..n {
        let l: f64 = iter.next().unwrap().parse().unwrap();
        if l.abs() > (start + l).abs() {
            if l > start {
                count += 1;
                start += l;
            } else {
                start += l;
            }
        } else if l.abs() > (start - l).abs() {
            if l > start {
                count += 1;
                start -= l;
            } else {
                start -= l;
            }
        }
    }

    println!("{}", count);
}
//0188_abc453_C_間違えたコード
//貪欲法は罠らしい。反証：（40,50,90,1,1,1）
//bit全探索がいいそうだ