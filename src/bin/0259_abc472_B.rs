use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }

    let mut left_sum = 0;
    let all_sum: i64 = l.iter().sum();
    let mut min_sum = i64::MAX;

    for i in l {
        left_sum += i;
        let right_sum = all_sum - left_sum;
        if min_sum > (right_sum - left_sum).abs() {
            min_sum = (right_sum - left_sum).abs();
        }
    }

    println!("{min_sum}");
}
//0259_abc472_B

// ある境界線に対して、左の要素の合計値と右の要素の合計値の絶対値の最小値を求める。
// 右の要素の合計値＝全体の合計値ー左の要素の合計値
// 最小値を更新していき、出力
// 上手く.min()を使えればよかったのかも