use std::io::{self, Read};

fn practice3() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let team: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
    let mut ng_pair = Vec::new();

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        ng_pair.push((a, b));
    }

    let mut max_sum = 0;

    'bit: for bit in 0..(1 << n) {
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += team[i];
            }
        }

        for (a, b) in &ng_pair {
            if bit & (1 << a) != 0 && bit & (1 << b) != 0 {
                continue 'bit;
            }
        }
        max_sum = max_sum.max(sum);
    }


    println!("{max_sum}");
}

fn practice4() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize =  iter.next().unwrap().parse().unwrap();
    let q: usize =  iter.next().unwrap().parse().unwrap();
    let nums: Vec<u32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();


    for _ in 0..q {
        let a: u32 =  iter.next().unwrap().parse().unwrap();
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            
            if nums[mid] < a {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        println!("{}", nums.len() - left);
    }
}

fn main() {
    // practice3();
    practice4();
}
//0221_二分探索とbit全探索の復習