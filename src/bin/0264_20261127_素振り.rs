use proconio::input;

fn suburi_4() {
    let _n = 5;
    let k = 5;
    let a = [0, 3, 8, 20, 22];

    let mut open_time = 0;//いつまで開くかの時間
    let mut close_count = 0;//ドアが閉まった回数

    for i in a {
        if i == 0 && open_time == 0 {
            close_count += 1;
        }

        if i > open_time {
            close_count += 1;
        }
        open_time = i + k;
    }

    println!("{open_time} {close_count}");
}
// 最初は閉まっているのと、1≦nなので、一度はかならず開く
// i > open_time だと、一度閉まる。iのほうが大きいので、次に必ず開く
// i = 0のときどうするか。これだけは例外処理かな？


fn suburi_5() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let jump_count = a[n - 1].div_ceil(k);//飛んだ回数
    let dist = jump_count * k;//飛んだ距離

    println!("{jump_count} {dist}");
}
// 高橋君は数直線上の座標0にいます。
// 高橋君は1回のジャンプで、正の方向へKだけ移動できます。
// N個の目標地点Aが与えられます。
// 各目標地点について、現在の座標がその目標地点以上になるまでジャンプしてください。
// 目標地点に到達するか通り過ぎたら、現在の座標を維持したまま次の目標地点について同じ処理を行います。
// すべての目標地点の処理が終了したときのジャンプの合計回数と、最終的な座標を、この順番で出力してください。

//ん？？？　この問題、aの配列内の中間目標地点、関係なくない？
// 最後の目標地点まで何回のジャンプで行けたかを見ればいいだけでは？


fn main() {
    suburi_4();
    suburi_5();
}
//0264_20261127_素振り