use std::cmp;
//cmpってなんだっけ←compare、大小の比較や並び替えに使う
use std::io::{self, Read};

fn get_num() -> (usize, Vec<u64>, Vec<u64>, usize, Vec<u64>) {
    let mut input = String::new();
    //read_to_string()で入力をまとめて受け取って、iterに詰める
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    //Vecの要素数が分かっているので、with_capacity()で空のVecを作っている
    //Vec![]だと要素の詰まった箱ができるので、push()でズレてしまう
    //Vec::new()だと、要素を詰めるごとに大きいVecの作り直し作業が入り、処理が重くなる
    //capacity()は配列の席数を数えるメソッド。with_capacity(数値)は空の席を人数分作るメソッド
    let mut h = Vec::with_capacity(n);
    let mut l = Vec::with_capacity(n);

    //iterに詰まっている要素をhとlの配列に交互に入れていく
    for _ in 0..n {
        h.push(iter.next().unwrap().parse::<u64>().unwrap());
        l.push(iter.next().unwrap().parse::<u64>().unwrap());
    }

    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut t = Vec::with_capacity(q);
    for _ in 0..q {
        t.push(iter.next().unwrap().parse::<u64>().unwrap());
    }
    (n, h, l, q, t)
}

fn check() {
    let (n, h, l, q, t) = get_num();
    let mut max_h = vec![0; n];

    max_h[n - 1] = h[n - 1];//この処理いる？←絶対にいるらしい
    //最後尾の人の身長が0だと、cmp::max()での計算がおかしくなるため
    //下のforで(0..n-1).rev()と書いている、つまりiはn-1番めからスタートする
    //n番目は…上のmax_h[n - 1]で手動で要素を入れている

    for i in (0..n - 1).rev() {
        //cmp::max()で2つの要素の最大値の比較をし、大きい方を取り出す
        //cmp::max()とmax()の違いってなんだ？
        //cmp::max()は関数、数値.max()はメソッド。具体的な違いはわからなかった
        max_h[i] = cmp::max(h[i], max_h[i + 1]);
        //「自分自身の身長」と「自分の1つ後ろの最大値」を比較して大きい方を残す←ロジックがよくわからない…
    }

        // クエリをQ回処理する
        for i in 0..q {
        // ★重要：質問ごとに left と right を初期位置に戻す
        let mut left = 0;
        let mut right = l.len();

        // 二分探索←while内のロジックがわからない…
        while left < right {
            //(right + left) / 2と書かないのは、数値のオーバーフローを防ぐためらしい
            let mid = left + (right - left) / 2;

            if l[mid] <= t[i] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // ★重要：質問が1つ終わるごとに、その時点での答えを出力する
        println!("{}", max_h[left]);
    }


    //----以下は間違えたコード-----

    //leftとrightはfor内で作る必要がある。毎回初期化する必要がある
    //let mut left = 0;
    //let mut right = l.len();

    //for i in 0..q {

        //while left < right {
            //while内のロジックがよくわからん
            //let mid = left + (right - left) / 2;
            
            //if l[mid] <= t[i] {
                //left = mid + 1;
            //} else {
                //right = mid;
            //}
        //}
    //max_h[left]
}

fn main() {
    check();
}
//0143_abc463_C_AI生成_読解