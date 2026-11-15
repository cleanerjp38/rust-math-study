use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let ans = (n % 3) + 1;
    println!("{ans}");
}
//0251_abc474_A

// 1～3の数値があり、入力された数値以外を出力する
// モジュラー算術で、入力値+1を出力