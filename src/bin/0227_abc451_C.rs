use std::cmp::Reverse;
use std::io::{self, Read};
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: u32 = iter.next().unwrap().parse().unwrap();

        match a {
            1 => {
                heap.push(Reverse(b));
            }
            2 => {
                while let Some(&Reverse(x)) = heap.peek() {//ここはAIから教わった。なんでSome(&Reverse(x))と書く必要があるんだ？
                    if x <= b {
                        heap.pop();
                    } else {
                        break;
                    }
                }
            }
            _ => (),
        }

        // let count = heap.iter().count();　要素数を数えるなら、len()でいい
        let count = heap.len();
        println!("{}", count);
    }
}
//0227_abc451_C