use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace().skip(2);
    let s: String = iter.next().unwrap().to_string();
    let t: String = iter.next().unwrap().to_string();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let checker: String = s.chars().filter(|c| t.contains(*c)).collect();
    let s_pure: String = s.chars().filter(|c| !checker.contains(*c)).collect();
    let t_pure: String = t.chars().filter(|c| !checker.contains(*c)).collect();

    for _ in 0..q {
        let w: String = iter.next().unwrap().parse().unwrap();
        let is_takahashi = w.chars().any(|c| s_pure.contains(c));
        let is_aoki = w.chars().any(|c| t_pure.contains(c));

        if is_takahashi {
            println!("Takahashi");
        } else if is_aoki {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}
//0163_abc441_B_2回目
//any()の使い方の復習