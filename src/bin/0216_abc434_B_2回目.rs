use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut birds = vec![(0, 0); m];

    for _ in 0..n {
        let bird_num: usize = iter.next().unwrap().parse().unwrap();
        let weight: u32 = iter.next().unwrap().parse().unwrap();
        birds[bird_num - 1].0 += weight;//配列内のタプルの要素に足し算が出来た。これは自分で考えられた
        birds[bird_num - 1].1 += 1;
    }

    for (weight, count) in birds {
        // let result = (weight / count) as f64; 整数で割ってからf64にしても意味無かった
        let result = weight as f64 / count as f64;
        println!("{:.10}", result);
    }
}
//0216_abc434_B_2回目