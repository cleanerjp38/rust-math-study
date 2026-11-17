use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut ans = 0;
    for i in (n / 2)..n {
        ans += a[i];
    }

    println!("{ans}");
}
//0254_abc473_A
//Obsidianに格納しない

// nは偶数なので、n/2で半分にできる
// nはインデックス番号なので、 n/2..n は (n/2)+1~nまでの数値となる