use std::io::{self, Read};
use std::collections::HashMap;
use std::cmp::Reverse;


fn practice1(n: usize, a: [u32; 5]) {
    let mut b: Vec<usize> = (1..=n).collect();
    // b.sort_by_key(|i| a[i - 1]); これだと得点の昇順となっている
    b.sort_by_key(|&i| (Reverse(a[i - 1]), i));//これで番号の精進となるらしい、どういうロジックかわからない
    for i in b {
        print!("{} ", i);
    }
}

fn practice2(n: usize, a: [&str; 7]) {
    let mut hashmap = HashMap::with_capacity(n);
    for s in a {
        *hashmap.entry(s).or_insert(0) += 1;
    }

    let mut ans = Vec::new();
    for (name, count) in &hashmap {
        if count >= &2 {
            ans.push(name);
        }
    }

    ans.sort();
    if ans.is_empty() {
        println!("None");
    } else {
        for name in ans {
            println!("{name}");
        }
    }
}

fn practice3() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let team: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let mut max_sum = 0;

    // for _ in 0..m {
    //     let a: usize = iter.next().unwrap().parse().unwrap();
    //     let b: usize = iter.next().unwrap().parse().unwrap();
    //     // ng_couple.push((a, b));

    //     for bit in 0..(1 << n) {
    //         let mut sum = 0;
    //         for i in 0..n {
    //             if bit & (1 << i) != 0 {
    //                 sum += team[i];
    //             }
    //         }
    //         if bit & (1 << a) != 0 && bit & (1 << b) != 0 {
    //             continue;
    //         }
    //         max_sum = max_sum.max(sum);
    //     }
    // }
    //標準入力の取得ごとに処理できるかと思ったが、ちゃんとペアの配列で取得しないと駄目だった。

    let mut ng_pair = Vec::new();

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        ng_pair.push((a, b));
    }

    'bits: for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += team[i];
            } 
        }

        for &(a, b) in &ng_pair {
            if bit & (1 << a) != 0 && bit & (1 << b) != 0 {
                continue 'bits;
            }
        }
        max_sum = max_sum.max(sum);
    }


    println!("{max_sum}");
}

fn practice4_failed() {//盛大に間違えた
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();

    let mut left = 0;
    let mut right = nums.len();

    for _ in 0..q {
        let x: u32 = iter.next().unwrap().parse().unwrap();

        let mut count = 0;
        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < x {
                left = mid + 1;
            }
            if nums[mid] == x {
                left = mid;
                while nums[left] == x {
                    left -= 1;
                }
                count += nums.len() - left + 1;
                break;
            }
            if nums[mid] > x {
                right = mid;
            }
        }
        println!("{count}");
    }
}

fn  practice4() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();



    for _ in 0..q {
        let x: u32 = iter.next().unwrap().parse().unwrap();

        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        println!("{}", nums.len() - left);//配列の総数から、除外される要素の個数を引けば良かったのか
    }


}


fn main() {
    practice1(5, [70, 90, 70, 100, 90]);
    println!("");
    practice2(7, 
        [
            "apple",
            "banana",
            "apple",
            "orange",
            "banana",
            "grape",
            "apple"
            ]
        );
    println!("");
    practice3();
    println!("");
    //practice4(); read_to_string()で標準入力を全て吸い取ってしまうため、コメントアウトしている
    println!("");
}
//0220_C問題に関する練習_4本