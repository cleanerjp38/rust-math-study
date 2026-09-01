use std::io::{self, Read};

fn get_num() -> (usize, usize, Vec<char>, Vec<char>) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().to_string();
    let t: String = iter.next().unwrap().to_string();

    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    (n, m, s_vec, t_vec)
}

fn main() {
    let (n, m, s_vec, t_vec) = get_num();
    
    let mut min_ops = u32::MAX;
    for i in 0..=(n - m) {
        let mut current_ops = 0;

        for j in 0..m {
            let s_i = s_vec[i + j].to_digit(10).unwrap();
            let t_i = t_vec[j].to_digit(10).unwrap();
            let count = (s_i + 10 - t_i) % 10;
            current_ops += count;
        }
        min_ops = min_ops.min(current_ops);
    }

    println!("{}", min_ops);
}
//0168_2_abc438_B_charで処理