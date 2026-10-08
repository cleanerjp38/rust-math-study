use std::collections::HashSet;
use std::io::{self, Read};

fn hashset1() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let nums: HashSet<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    let ans = nums.len();

    println!("{}", ans);
}

fn hashset2() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    //let nums: HashSet<u32> = iter.take(n).map(|x| x.parse().unwrap()).collect(); この書き方だと、for内でのiterの呼び出しで借用チェッカーが怒った。なんでだ？
    //let nums: HashSet<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect(); by_ref()でiterに可変参照を返す必要があった
    let mut nums: HashSet<u32> = HashSet::new();

    for _ in 0..n {
        nums.insert(iter.next().unwrap().parse().unwrap());
    }

    for _ in 0..m {
        let b: u32 = iter.next().unwrap().parse().unwrap();
        if nums.contains(&b) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn hashset3() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let nums_a: HashSet<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let nums_b: HashSet<u32> = iter.by_ref().take(m).map(|x| x.parse().unwrap()).collect();
    let mut ans = 0;

    for i in nums_b {
        if nums_a.contains(&i) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn hashset4() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut people_id: HashSet<u32> = HashSet::new();
    let mut count = 0;

    for _ in 0..n {
        let person: u32 = iter.next().unwrap().parse().unwrap();
        if people_id.insert(person) {
            count += 1;
        }
        println!("{}", count);
    }
}

fn main() {
    hashset4();
}
//0210_HashSet_練習_4本