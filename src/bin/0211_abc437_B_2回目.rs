use std::io::{self , Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut fishbone = Vec::with_capacity(h);

    for _ in 0..h {
        let mut row: HashSet<u32> = HashSet::new();
        for _ in 0..w {
            let a: u32 = iter.next().unwrap().parse().unwrap();
            row.insert(a);
        }
        fishbone.push(row);
    }

    let called_nums:HashSet<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;

    for row in fishbone {
        let count = row.intersection(&called_nums).count() as u32;
        ans = ans.max(count)
    }

    println!("{}", ans);
}
//0211_abc437_B_2回目