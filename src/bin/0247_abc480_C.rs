use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        t: u32,
        mut a: [u32; n],
    }

    let mut x = 0;
    let mut ans = 0;

    a.sort();

    for aoki in a {
        if x >= t {
            break;
        }

        if x < aoki && aoki <= t {
            ans += (aoki - x) / k;
            if (aoki - x) % k != 0 { //あまり分があれば、カウントを1足す
                ans += 1;
            }
            x = aoki + k;
        }

        // while x < aoki { ループを回すと計算量が増えすぎる
        //     x += k;
        //     ans += 1;
        // }

        if x >= aoki {//この処理はなくていいらしい、なんでだ？
            x = aoki + k;
        }
    }

    if x < t {
        ans += (t - x) / k;
        if (t - x) % k != 0 {
            ans += 1;
        }
    }
    //以下は1日前に考えたコード
    // for aoki in a {
    //     if x >= t {
    //         break;
    //     }

    //     if aoki == 0 {
    //         x += k;
    //         continue;
    //     }

    //     if x + k < aoki {
    //         while x < aoki {
    //             x += k;
    //             ans += 1;
    //         }
    //     }

    //     if x >= aoki {
    //         x = aoki + k;
    //     }
    // }

    // if x < t {
    //     while x < t {
    //         x += k;
    //         ans += 1;
    //     }
    // }

    println!("{ans}");

}
//0247_abc480_C
// 1．現在時刻をx＝0とする
// 2．青木くん出現タイミングを1つ見る。
// x<tの間、 x >= a[i] となるまで、 x += k を繰り返す
// 3．また、次の青木くんが出現するタイミングを1つ見る
// 4．青木くんが出現し切るか、 x >= tとなるまで、　x += k をくりかえす
// 5．ループ終了時、 x < tのとき、　x >= t　となるまで、x += k をくりかえす
// 6．2と5でカウントした回数を出力
//xの意味が曖昧だったため、コメントアウトしたコードは失敗した
//xを”現在お湯が出る期限”とした
//whileだと計算量が多いため、整数の割り算で短縮した