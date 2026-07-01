use std::io;

fn get_rows() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.split_whitespace();
    let row_vec: Vec<i32> = tokens.map(|i| i.parse().unwrap()).collect();

    row_vec
}

struct Faretable {
    n: usize,
    matrix: Vec<Vec<i32>>,
}

//変数がグチャグチャになったので、構造体でまとめた
impl Faretable {
    fn get_matrix() -> Faretable {
        let mut n_buffer = String::new();
        io::stdin().read_line(&mut n_buffer).unwrap();
        let n: usize = n_buffer.trim().parse().unwrap();
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for _ in 0..=n {
            let row = get_rows();
            matrix.push(row)
        }
        Faretable { n, matrix }
    }

    fn count_iter(&self) -> u32 {
        let mut count_iter = 0;
        for i in 0..self.n {
            //iter()は参照であって、数値の数を数える機能ではない
            //count_iter += self.matrix[i].iter();
            for _val in self.matrix[i].iter() {
                count_iter += 1;
            }
        }
        count_iter
    }
}

fn main() {
    let matrix_count = Faretable::get_matrix();
    println!("{}",matrix_count.count_iter());
}
//0096_2_iter()_test



