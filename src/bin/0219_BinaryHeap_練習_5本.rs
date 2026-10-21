use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, Read};

fn binary_heap1(a: u32, b: [u32; 5]) {
    let mut binary_heap = BinaryHeap::new();
    for i in b {
        binary_heap.push(i);
    }
    for _ in 0..a {
        let ans = binary_heap.pop().unwrap();
        println!("{ans}");
    } 
}

fn binary_heap2(a: u32, b: [u32; 6]) {
    let mut binary_heap = BinaryHeap::new();
    for i in b {
        binary_heap.push(Reverse(i));
    }

    for _ in 0..a {
        let ans = binary_heap.pop().unwrap().0;
        println!("{}", ans);
    }
}

fn binary_heap3(n :usize, k: usize, b: [u64; 6]) {
    let mut binary_heap: BinaryHeap<u64> = BinaryHeap::with_capacity(n);
    for i in b {
        binary_heap.push(i);
    }

    let mut ans = 0;
    for _ in 0..k {
        ans += binary_heap.pop().unwrap();
    }

    println!("{}", ans);
}

fn binary_heap4() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    
    let mut binary_heap = BinaryHeap::new();

    for _ in 0..n {
        let section: usize = iter.next().unwrap().parse().unwrap();

        match section {
            1 => {
                let task: u32 = iter.next().unwrap().parse().unwrap();
                binary_heap.push(task);
            }
            2 => {
                println!("{}",binary_heap.pop().unwrap());
            }
            _ => (),
        }
        // let task: u32 = iter.next().unwrap().parse().unwrap(); 「2」が出た時、2のあとには入力はないので、入力処理も分岐後に入れる必要があった

        // match section {
        //     1 => {
        //         binary_heap.push(task);
        //     }
        //     2 => {
        //         println!("{}",binary_heap.pop().unwrap());
        //     }
        //     _ => (),
        // }
        
    }
}

fn binary_heap5(a: usize, b: [u64; 4]) {
    let mut binary_heap = BinaryHeap::new();
    for i in b {
        binary_heap.push(Reverse(i));
    }

    let mut ans = 0;
    // while binary_heap.is_empty() {
    //     ans += binary_heap.pop().unwrap().0;
    // }

    while binary_heap.len() > 1 {
        let x = binary_heap.pop().unwrap().0;
        let y = binary_heap.pop().unwrap().0;
        let z = x + y;
        ans += z;
        binary_heap.push(Reverse(z));
    }

    println!("{ans}");
}


fn main() {
    binary_heap1(5, [3, 8, 1, 9, 4]);
    println!("");
    binary_heap2(6, [7, 2, 9, 2, 5, 1]);
    println!("");
    binary_heap3(6, 3, [4, 9, 2, 8, 5, 7]);
    println!("");
    binary_heap5(4, [2, 3, 4, 6]);
    println!("");
    binary_heap4();
}
//0219_BinaryHeap_練習_5本