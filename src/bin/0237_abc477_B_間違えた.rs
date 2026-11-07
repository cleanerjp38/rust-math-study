use proconio::input;

fn main() {
    input! {
        n: usize,
        d: u64,
        mut x: [u64; n],
    }

    let mut point: Vec<(u64, usize)> = Vec::with_capacity(n);
    for (i, &man) in x.iter().enumerate() {
        let idx = i + 1;
        point.push((man, idx));
    }

    point.sort();
    let mut count = 0;
    let mut result = Vec::new();

    for a in 0..n {
        // let b = a + 1;　aの1つ次のbは作る必要が無かったらしい
        // if b == n {
        //     break;
        // }

        // if point[a].0 - point[b].0 >= d || point[b].0 - point[a].0 >= d { u64だと引き算で負の数になる可能性がある
        let left_ok = a == 0 || point[a].0 - point[a - 1].0 >= d;
        let right_ok = a == n - 1 || point[a + 1].0 - point[a].0 >= d;

        if left_ok && right_ok {
            count += 1;
            // result.push(b);
            result.push(point[a].1);
        }
    }

    println!("{count}");
    for ans in result {
        print!("{ans} ");
    }
}
//0237_abc477_B_間違えた