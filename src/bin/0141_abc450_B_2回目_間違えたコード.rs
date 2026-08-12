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
        
        //ここでVec::new()をしているが、それではmatrix[a][b]という項目は存在しなくなってしまう
        //もしそれをしたいなら、vec![vec![0; n]; n]で0を敷き詰めた箱を準備する必要がある
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        for a in 0..(n - 1) {
            let next_line = lines.next().unwrap().unwrap();
            let row = next_line.trim().split_whitespace();
            for (b, i) in row.enumerate() {
                let price: u32 = i.parse().unwrap();
                matrix[a][b] = price;
            }
        }
        Faretable { n, matrix}
    }

    fn has_cheaper_split(&self) {
        let mut check = false;
        for a in 0..self.n {
            for b in (a + 1)..self.n {
                for c in (b + 1)..self.n {
                    //ジャグ配列のため、b=2やc=1のときにそんな場所はないから、エラーになる
                    if self.matrix[a][c] > self.matrix[a][b] + self.matrix[b][c] {
                        // check = true自体に値はないため、ここでreturnすると関数自体が終了する
                        //なので、下のifが実行されなくなる
                        return check = true;
                        //ベストは、ここでprintln!()をすること
                        //println!("Yes");
                        //return;
                    }
                }
            }
        }
        if check {
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
//0141_abc450_B_2回目_間違えたコード