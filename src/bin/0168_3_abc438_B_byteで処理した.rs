use std::io::{self, Read};

fn get_num() -> (usize, usize, Vec<u8>, Vec<u8>) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<u8> = iter.next().unwrap().as_bytes().to_vec();
    let t: Vec<u8> = iter.next().unwrap().as_bytes().to_vec();

    (n, m, s, t)
}

fn main() {
    let (n, m, s, t) = get_num();

    let mut min_ops = u32::MAX;
    for i in 0..=(n - m) {
        let mut current_ops = 0;

        for j in 0..m {
            let s_i = s[i + j];
            let t_i = t[j];

            let count = (s_i + 10 - t_i) % 10;
            current_ops += count as u32;
        }
        min_ops = min_ops.min(current_ops);
    }

    println!("{}",min_ops);
}
//0168_3_abc438_B_byteで処理