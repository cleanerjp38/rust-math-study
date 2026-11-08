use proconio::input;

fn main() {
    input! {
        s: [String; 2],
    }

    let mut vec_char = Vec::new();
    for c in s {
        let row: Vec<char> = c.chars().collect();
        vec_char.push(row);
    }

    let mut a_idx = (0, 0);
    let mut d_idx = (0, 0);
    for (x,row) in vec_char.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            match c {
                'A' => {
                    a_idx = (x, y);
                }
                'D' => {
                    d_idx = (x, y);
                }
                _ => (),
            }
        }
    }

    if a_idx.0 == d_idx.0 || a_idx.1 == d_idx.1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
//0238_abc480_A_2回目
//i1 == i2 || j1 == j2 がOKなら成立する
//この条件にどうやってAとDを入れるんだ？
//enumerate()でインデックス番号を出して、タプルに入れた
//もっと行数の少ない書き方がありそうだ