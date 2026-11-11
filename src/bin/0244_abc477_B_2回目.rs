use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u32,
        x: [u32; n],
    }

    let mut people = Vec::with_capacity(n);
    for (idx, i) in x.iter().enumerate() {
        people.push((i, idx + 1));
    }

    people.sort();

    let mut count = 0;
    let mut outliers =Vec::new();

    for a in 0..n {
        let left_ok = a == 0 || people[a].0 - people[a - 1].0 >= d;
        let right_ok = a == n - 1 || people[a + 1].0 - people[a].0 >= d;

        if left_ok && right_ok {
            count += 1;
            outliers.push(people[a].1);
        }
    }

    outliers.sort();
    println!("{count}");

    if outliers.len() == 0 {
        println!("");
        return;
    }

    for ans in outliers {
        print!("{ans} ");
    }
}
//0244_abc477_B_2回目
//1つの要素について全探索
//people[a].0 - people[a - 1].0 >= d && people[a + 1].0 - people[a].0 >= d が成立するとき、カウントとインデックスを詰める
//しかし、出力が”4,7”となった
//for a in 1..n - 1 { としていた
//people.push((i, idx)); と、idxが0始まりだった
//端を探索する方法がわからなかった
//なので、left_ok right_Ok は、AIのコードを真似した