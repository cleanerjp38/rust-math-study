use proconio::input;

fn main() {
    input! {
        n: usize,
        v: usize,
        w: [u32; n],
    }

    let mut ans = 0;

    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                if a + b + c + 3 <= v { //(a + 1) + (b + 1) + (c + 1)の1をまとめた
                    ans = ans.max(w[a] + w[b] + w[c]);
                }
            }
        }
    }

    println!("{ans}");
}
//0243_abc478_B_2回目
//N*N*N=1000000なので、三重ループができる
//全探索で、最大値を更新していく
//3つの要素のインデックス番号を保管する必要はないので、がんがん最大値を更新できる