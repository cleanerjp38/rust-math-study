use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for _ in 0..h {
        // let row = iter.by_ref().take(w).map(|x| x.parse::<char>().unwrap()).collect(); 1行分は空白なく来るんだった（#####みたいに）
        let row = iter.next().unwrap().chars().collect();//文字列を.chars()で分解している
        grid.push(row);
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    let mut count = 0;
    

    for (x, row) in grid.iter().enumerate() {
        // let mut dots: Vec<(usize, usize)> = Vec::new();'.'の位置をつめる必要は無かった。for内で都度判定すればよかった

        for (y, _) in row.iter().enumerate() {
            // dotsに詰める作業をコメントアウト
            // match i {
            //     '.' => {
            //         count += 1;
            //         let point = (x, y);
            //         dots.push(point);
            //     }
            //     _ => (),
            // }

            if grid[x][y] == '.' && !visited[x][y] {
                visited[x][y] = true;
                queue.push_back((x, y)); //未探索な'.'のインデックス番号をqueueに詰める

                let mut touched_border = false;//'.'のマスの集合が外周に触れているかの判定
                while let Some((r, c)) = queue.pop_front() {
                    // if grid[x - 1][y] == '.' && !visited[x - 1][y] { xの値によっては範囲外になってしまう
                    if r > 0 && grid[r - 1][c] == '.' && !visited[r - 1][c] {
                        visited[r - 1][c] = true;
                        queue.push_back((r - 1, c)); 
                    }
                    if r + 1 < h && grid[r + 1][c] == '.' && !visited[r + 1][c] {
                        visited[r + 1][c] = true;
                        queue.push_back((r + 1, c)); 
                    }
                    if c > 0 && grid[r][c - 1] == '.' && !visited[r][c - 1] {
                        visited[r][c - 1] = true;
                        queue.push_back((r, c - 1)); 
                    }
                    if c + 1 < w && grid[r][c + 1] == '.' && !visited[r][c + 1] {
                        visited[r][c + 1] = true;
                        queue.push_back((r, c + 1)); 
                    }


                    if r == 0 || r == h - 1 || c == 0 || c == w -1 {
                        touched_border = true;
                    }
                }

                if !touched_border {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}
//0228_abc450_C