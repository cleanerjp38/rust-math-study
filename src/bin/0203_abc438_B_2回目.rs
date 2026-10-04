use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<u8> = iter.next().unwrap().as_bytes().to_vec();//なんでcollect()じゃなくてto_vec()なんだろ
    let t: Vec<u8> = iter.next().unwrap().as_bytes().to_vec();//as_bytes()で生成されるのは、&[u8]。可変配列のVec<u8>ではない
    let mut ans = u32::MAX;

    for i in 0..=(n - m) {
        let mut current_ops = 0;
        for j in 0..m {
            let count = ((s[i + j] + 10) - t[j]) % 10;
            current_ops += count as u32; 
        }
        ans = ans.min(current_ops);
    }

    println!("{}", ans);
}
//0203_abc438_B_2回目