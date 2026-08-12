use std::{io::{self, BufRead}, vec};

struct Faretable {
    n: usize,
    matrix: Vec<Vec<u32>>,
}

impl Faretable {
    fn get_table() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(|l| l.unwrap());
        let n: usize = lines.next().unwrap().trim().parse().unwrap();
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; n]; n];

        //zip()は別々のイテレータを一つに合わせるメソッド
        (0..n - 1).zip(lines).for_each(|(a, line)| {
            line.trim().split_whitespace()
                .enumerate()
                //for_each()は流れてくるイテレータに対して指定した処理をする。戻り値を返さない
                .for_each(|(i, val_str)| {
                    matrix[a][a + 1 + i] = val_str.parse().unwrap();
                });
        });

        Faretable { n, matrix }
    }

    fn has_cheaper_split(&self) {
        //any()は、イテレータの中に何か一つでも指定した条件がtrueになるものがあれば、返り値としてtrueを返す
        let found = (0..self.n).any(|a| {
            ((a + 1)..self.n).any(|b| {
                ((b + 1)..self.n).any(|c| {
                    self.matrix[a][c] > self.matrix[a][b] + self.matrix[b][c]
                })
            })
        });

        if found {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn main() {
    let table = Faretable::get_table();
    table.has_cheaper_split();
}
//0141_abc450_B_イテレータで書いた
//0.01sで完了