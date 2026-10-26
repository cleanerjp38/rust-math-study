use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: u32 = iter.next().unwrap().parse().unwrap();
    let a: Vec<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let mut count = 0;
    for i in 1..=a.len() - 1 {
        let j = i - 1;
        if a[i] > a[j] {
            if (a[j]..=a[i]).contains(&k) {
                count += 1;
            }
        }
        if a[i] < a[j] {
            if (a[i]..=a[j]).contains(&k) {
                count += 1;
            }
        }
    }
    let peak = a.iter().filter(|&x| x == &k).count();//両端がpeakだった場合に、余計に引かれてしまう
    // let peak = a[1..a.len()-1] 最初の指定で、両端を引いておくのが正解
    // .iter()
    // .filter(|&&x| x == k)
    // .count();

    println!("{}", count - peak);
}
//0224_abc481_B
//WA