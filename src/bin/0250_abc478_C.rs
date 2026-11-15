use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }

    let mut a_sort = a.clone();
    a_sort.sort();

    let mut left = 0;
    for i in 0..n {
        if a[i] != a_sort[i] {
            left = i;
            break;
        }
    }

    let mut right = 0;
    for j in (0..n).rev() {
        if a[j] != a_sort[j] {
            right = j;
            break;
        }
    }

    if right - left + 1 <= k {
        println!("Yes");
    } else {
        println!("No");
    }


    // let mut i = 0;
    
    // loop {
    //     let current = a[i];
    //     let next = a[i + 1];
    //     if current > next {
    //         break;
    //     }
    //     i += 1;
    // }

    // let left = i;

    // let mut right = 0;

    // for j in (0..n).rev() {
    //     let current = a[j];
    //     let next = a[j - 1];

    //     if current< next {
    //         break;
    //     }
    //     right += 1;
    // }

    // if  k + left + right + 2 < n {
    //     println!("No");
    // }
}
//0250_abc478_C

// 昇順の配列と元の配列の2つを準備する
// 前から1つずつ見て、昇順とズレているIdxをメモる
// 今度は、後ろから見て、ズレている箇所をメモる。
// この間の数値がk以内ならOK

//コメントアウトのコードは、最初に書いて書けなかったコード
// 配列を前から見ていって、ai < ai+1かどうか見ていく
// ちがったら、そのインデックス番号lをメモる
// 後ろからも同様 r
// l + r + k < n となったら、No
// l + r + k - nの値が、kの配列を動かせる範囲
// その動かせる配列を調べていく
// l内の最大値 < k内の最小値ならOK
// k内の最大値 < r内の最小値ならOK
// と書こうとして、挫折した