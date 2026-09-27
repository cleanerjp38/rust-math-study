use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: u64 = iter.next().unwrap().parse().unwrap();
    let a: Vec<u64> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    for i in 0..n {
        //let sum: u64 = a.iter().filter(|&x| x != &a[i]).sum(); これだと、要素の値が重複していたら駄目
        let total: u64 = a.iter().sum();
        if total - a[i] == m {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
//0197_abc429_B