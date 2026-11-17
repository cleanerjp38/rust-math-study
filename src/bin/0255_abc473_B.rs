use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut map = HashMap::with_capacity(n);

    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut ans = 0;

    for (x, count) in map {
        if count % 2 != 0 {
            ans += x;
        }
    }

    println!("{ans}");
}
//0255_abc473_B

// 「同じ整数が書かれている異なる2枚のカードを選び、それら2枚のカードを食べる。」
// これは、本来配列から消去するのが正道なのかもしれないが、
// HashMapでカウントして、2の倍数でない場合は足していくことにした