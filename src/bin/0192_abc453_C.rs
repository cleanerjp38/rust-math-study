use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let l: Vec<i64> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;
    for bit in 0.. (1 << n) {
        let mut before: i64 = 1;
        let mut after = 0;
        let mut count = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                after = before + (l[i] * 2);
            } else {
                after = before - (l[i] * 2);
            }

            if before * after < 0 {
                count += 1;
            }
            before = after;
        }

        ans = ans.max(count);
    }

    println!("{}", ans);
}
//0192_abc453_C