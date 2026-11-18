use std::collections::BinaryHeap;
use std::cmp::Reverse;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut heap = BinaryHeap::with_capacity(3);

    for &num in a.iter().take(3) {
        heap.push(Reverse(num));
    }

    let mut min = heap.pop().unwrap().0;
    println!("{}", min);

    for &num in a.iter().skip(3) {
        if num > min {
            heap.push(Reverse(num));
            min = heap.pop().unwrap().0;
        }
        println!("{min}")
    }
}
//0257_abc476_C

// nからまず、3つの数値を拾って、BinaryHeapに降順で入れる
// 3つの数値のうち、最小の物を.pop()で取り出して、出力
// nの残りも、同じように処理する