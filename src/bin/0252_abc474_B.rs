use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }

    let mut group = n / 10;
    if n % 10 != 0 {
        group  += 1;
    }

    for i in 0..group {
        for j in (0 + (i * 10))..(10 + (i * 10)) {
            if p[j] / 10 != i as u32 {// ここでp[j]にアクセスしているので、範囲外アクセスが発生する可能性あり
                if p[j] % 10 != 0 {
                    println!("No");
                    return;
                }
            }

            if j == n - 1 {
                break;
            }
        }
    }

    println!("Yes");
}
//0252_abc474_B

// 10人ずつのグループの数値を見ていく
// 10の位の数値が外れている人がグループにいたら、Noを出力する
// 例外処理として、10や20といった「10で割り切れる値」はOKとする
// このループだと、jがnからはみ出ることが有り得るため、if文で離脱させている
// このコードだと、範囲外アクセスが発生するらしい