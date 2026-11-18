use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        q: usize,
        s: Chars,
        t: Chars,
        quarrys: [(usize, usize); q],
    }

    let mut point = Vec::new();

    if s.len() < t.len() {
        for _ in 0..q {
            println!("No");
        }
        return;
    }

    'point: for j in 0..s.len() {
        if s.len() - t.len() < j {
            break;
        }

        if s[j] == t[0] {
            let mut idx = 0;

            for i in j..s.len() {
                if s[i] == t[idx] {
                    idx += 1;
                } else {
                    continue 'point;
                }

                if idx == t.len() {
                    break;
                }
            }

            point.push(j);
        }
    }

    for (l, r) in quarrys {
        let mut left = 0;
        let mut right = point.len();
        let mut check = false; 

        while left < right {
            let mid = (right + left) / 2;
            if point[mid] >= l - 1 {
                if point[mid] + t.len() <= r {
                    check = true;
                    break;
                } else {
                    right  = mid;
                }
            } else {
                left = mid + 1;
            }
        }

        if check {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
//0256_abc477_C

// sのどこにtが出現するか調べる
// tの出現位置を配列に入れる（全探索）
// クエリとその出現位置を見比べて、クエリの指定範囲内ならYesを出力（二分探索）
// tがsより大きい場合、例外処理をする
