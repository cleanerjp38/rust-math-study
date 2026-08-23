use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: u64 = iter.next().unwrap().parse().unwrap();

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(iter.next().unwrap().parse::<u64>().unwrap());
    }

    let mut count = 0u64;
    let mut sum = 0u64;   // 今見ている区間の合計
    let mut right = 0;    // 右端の指

    for left in 0..n {
        while right < n && sum + a[right] <= k {
            sum += a[right];
            right += 1;
        }
        count += (right - left) as u64;

        if right == left {
            right += 1;
        } else {
            sum -= a[left] as u64;
        }
    }
    println!("{}", count);
}
//0157_尺取法_練習