use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }

    let mut ans = 0;
    for(idx, num) in a.iter().enumerate() {
        if (idx + 1) % 2 == 1 {
            ans += num;
        }
    }

    //こういう書き方もあるそうだ
    // let ans: u32 = a.iter()
    // .step_by(2)
    // .sum();

    println!("{ans}");
}
//0240_abc479_A_2回目
// インデックス番号＋1を2で割って、余りが1のとき、その番号は奇数