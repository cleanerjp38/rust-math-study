use std::io::{self, Read};

struct Faretable {
    n: usize,//気づけば、nは使用していなかった
    matrix: Vec<Vec<u32>>,
}

impl Faretable {
    fn get_vec() -> Faretable {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        let mut tokens = buffer.split_whitespace();
        let input_n: usize = tokens.next().unwrap().parse().unwrap();

        //vec!の練習もしてみた
        let mut matrix_2d = vec![vec![0; input_n]; input_n];

        //この書き方だと、正方形の配列が作成される
        for i in 0..input_n {
            for j in 0..input_n {
                //各欄に数値を入れていく作業
                let num: u32 = tokens.next().unwrap().parse().unwrap();
                matrix_2d[i][j] = num;
            }
        }

        Faretable { n: (input_n), matrix: (matrix_2d) }
    }

    fn count_matrix(&self) {
        //以下のコードだと要素を一個一個見ようとしている。
        //この書き方は、要素の数値を足し合わせたりする時に使えるかも？
        //let mut count_column: u32 = 0;
        //let mut count_row: u32 = 0;
        //for i in 0..self.n {
        //    count_row = self.matrix[i][0].count();
        //}
        //count()はiter()で流れてくる要素の数を数える機能。
        //なので、iter()の後ろでないと使えない
        let count_row = self.matrix.iter().count();
        println!("行の要素数は{}", count_row);

        //for j in 0..self.n {
        //    count_column = self.matrix[0][j].count();
        //}
        let count_column = self.matrix[0].iter().count();
        println!("列の要素数は{}", count_column);
    }
}

fn main() {
     let matrix_2d = Faretable::get_vec();
     matrix_2d.count_matrix();
}
//0098_count()_test