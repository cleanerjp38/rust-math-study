use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let mut x: u32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let w_vec: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut visited = vec![false; n];

    for _ in 0..q {
        let p: usize = iter.next().unwrap().parse().unwrap();
        if !visited[p - 1] {
            x += w_vec[p - 1];
            visited[p - 1] = true;
        } else {
            x -= w_vec[p - 1];
            visited[p - 1] = false;
        }
        println!("{}", x);
    }
}
//0186_abc431_B