use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: Vec<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    for r in 0..n {
        let mut num = 0;

        if r == 0 {
            println!("-1");
            continue;
        } 
        for l in 0..r {
            if a[r] < a[l] {
                //num = l; インデックスが0始まりを忘れていた
                num = l + 1;
            }
        }

        if num == 0 {
            println!("-1");
        } else {
            println!("{}", num);
        }
    }
}
//0217_abc433_B_2回目
//自力で書けた