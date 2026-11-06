use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize =iter.next().unwrap().parse().unwrap();
    let a: Vec<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut idx = 0;
    let mut ans = 0;

    while idx <= a.len() - 1 {
        ans += a[idx];
        idx += 2;
    }

    println!("{ans}");
}
//0231_abc479_A