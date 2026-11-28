use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        ab: [(u64, u64); n],
    }
    let mut a = Vec::with_capacity(n);//おかずの美味しさ
    let mut b = Vec::with_capacity(n);//おかずの体積
    let mut a_max = 0;//美味しさの最大値

    for row in ab {
        a.push(row.0);
        b.push(row.1);
    }

    for bit in 0..(1 << n) {
        let mut a_sum = 0;
        let mut b_sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                a_sum += a[i];
                b_sum += b[i];
            }
        }

        if b_sum <= w {
            a_max = a_max.max(a_sum);
        }
    }

    println!("{a_max}");
}
//0265_abc479_C_2回目

// 体積以内で、美味しさの最大値を目指す
// それぞれのおかずの組み合わせを管理する
// それぞれのおかずをbitで管理する
// bit全探索