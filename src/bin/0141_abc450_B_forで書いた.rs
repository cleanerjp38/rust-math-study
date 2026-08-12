use std::io::{self, BufRead};

struct Faretable {
    n: usize,
    matrix: Vec<Vec<u32>>,
}

impl Faretable {
    fn get_table() -> Self {
        let stdin = io::stdin();
        let buf = io::BufReader::new(stdin.lock());
        let mut lines = buf.lines();
        let first_line = lines.next().unwrap().unwrap();
        let n: usize = first_line.trim().parse().unwrap();
        
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; n]; n];
        for a in 0..(n - 1) {
            let s_line = lines.next().unwrap().unwrap();
            let row = s_line.trim().split_whitespace();
            for (i, val_str) in row.enumerate() {
                let b = a + 1 + i;
                let price: u32 = val_str.parse().unwrap();
                matrix[a][b] = price;
            } 
        }
        Faretable { n, matrix }
    }
    
    fn has_cheaper_split(&self) {
        for a in 0..self.n { 
            for b in (a + 1)..self.n {
                for c in (b + 1)..self.n {
                    if self.matrix[a][c] > self.matrix[a][b] + self.matrix[b][c] {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
        println!("No");
    }
}

fn main() {
    let table = Faretable::get_table();
    table.has_cheaper_split();
}
//0141_abc450_B_forで書いた
//0.07sで完了