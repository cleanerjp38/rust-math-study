use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().parse().unwrap();
    let mut counts = HashMap::new();

    for i in 0..=n - k {
        let sub = &s[i..i + k];//ここで&をつけないと、or_insert()に繋がらなかった
        *counts.entry(sub).or_insert(0) += 1;
    }

    let max_count = counts.values().max().unwrap();
    let mut ans = Vec::new();

    for (key, value) in &counts {
        if value == max_count {
            ans.push(key);
        }
    }
    
    ans.sort();
    for x in ans {
        println!("{}", x);
    }
}
//0200_abc428_B