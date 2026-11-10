use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let base: u32 = m / n;
    let moduler: u32 = m % n;

    for _ in 0..moduler {
        println!("{}", base + 1);
    }

    for _ in 0..(n - moduler) {
        println!("{base}");
    }
}
//0242_abc478_A_2回目
//m / n がベース数（u32なので）
//m % n が余り
//シンプルに出力したが、配列にカウントしていったほうが良かったか？