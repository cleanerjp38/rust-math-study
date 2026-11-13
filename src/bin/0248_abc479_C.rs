use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        ab: [(u64, u64); n],
    }

    let mut max_sum = 0;

    for bit in 0..(1 << n) {
        let mut sum_a = 0;
        let mut sum_b = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum_a += ab[i].0;
                sum_b += ab[i].1;
            }
        }

        if sum_b <= w {
            max_sum = max_sum.max(sum_a);
        }
    }

    println!("{max_sum}");
}
//0248_abc479_C
//自力でロジック含めてさくっと書けた

// N個のおかずの組み合わせを見る
// 組み合わせの中で、体積がw以下でAの最大合計値を更新していく
// bit全探索

// max_sum:sum_aの最大値
// sum_a:Aの合計値
// sum_b:Bの合計値

// 20桁の2進数を使う
// 0選ばない　1選ぶ
// forループで、低い桁から見ていく
// 二重ループで、”現在の桁より低い桁を全探索” ←この捉え方はちがうらしい
// 外側のループ： おかずを選ぶ・選ばないの全組み合わせを列挙する。
// 内側のループ： その組み合わせで、どのおかずが選ばれているかを調べる。
// もし、sum_b<= wなら、max_sum.max(sum_a)
// ループ後、max_sumを出力