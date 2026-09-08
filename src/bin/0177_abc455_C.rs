use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let mut a_vec: Vec<u64> = Vec::with_capacity(n);

    for _ in 0..n {
        let a: u64 = iter.next().unwrap().parse().unwrap();
        a_vec.push(a);
    }

    let mut map: HashMap<u64, u64> = HashMap::new();
    for a in a_vec {
        *map.entry(a).or_insert(0) += 1;
    } 

    let mut sum_vec: Vec<u64> = Vec::new();
    //ここ、1番目が個数で2番目が数値で合っているのだろうか…
    //ちがった！(num, count)が正しい順番だった
    for (count, num) in map {//なので、厳密にはここは間違い
        let sum = count * num;
        sum_vec.push(sum);
    }

    sum_vec.sort();
    sum_vec.reverse();
    for _ in 0..k {
        //skip()とremove()のどっちを使うか悩んだ
        sum_vec.remove(0);
    }
    let ans: u64 = sum_vec.iter().sum();
    //remove()だと、その分前に要素を詰めるという余計な作業が発生する
    //skip()なら、その作業分早くなる
    //let ans: u64 = sum_vec.iter().skip(k).sum();
    println!("{}", ans);
}
//0177_abc455_C
//1．n,k,a_vecを受け取る
//2．HashMapで、同値のAiを数える（できたっけ？）：O(N)
//3．数値と個数を掛けて、新しい配列を作る：O(M)
//4．その配列を大きい順にソートして、前からk個分取り除く：O(MlogM)
//5．残った配列を足して、出力O(M)