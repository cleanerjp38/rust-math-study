use std::io::{self, Read};

fn get_num() -> (usize, usize, u32, u32) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n :usize = iter. next().unwrap().parse().unwrap();
    let m :usize = iter. next().unwrap().parse().unwrap();
    let s :u32 = iter. next().unwrap().parse().unwrap();
    let t :u32 = iter. next().unwrap().parse().unwrap();

    (n, m, s, t)
}

fn main() {
    let (n, m, mut s, mut t) = get_num();

    let mut s_vec: Vec<u32> = Vec::with_capacity(n);
    while s > 0 {
        s_vec.push(s % 10);
        s /= 10;
    }

    let mut t_vec: Vec<u32> = Vec::with_capacity(m);
    while t > 0 {
        t_vec.push(t % 10);
        t /= 10;
    }

    let mut min_ops = u32::MAX;
    for i in 0..=(n - m) {
        let mut current_ops = 0;
        for j in 0..m {
            let s_i: u32 = s_vec[j + i];
            let t_i: u32 = t_vec[j];

            current_ops += (s_i + 10 -t_i) % 10;
        }
        min_ops = min_ops.min(current_ops);
    }

    println!("{}", min_ops);
}
//0168_abc468_B_間違えたコード
//数値を分解するならStringで受けとって、charで流すほうがやりやすいらしい