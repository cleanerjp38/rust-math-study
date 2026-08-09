use std::io::{self, BufRead};

fn square() -> u32 {
    let stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut hw = first_line.trim().split_whitespace();
    let h: usize = hw.next().unwrap().parse().unwrap();
    let w: usize = hw.next().unwrap().parse().unwrap();

    let mut grid = Vec::new();
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<char> = line
            .trim()
            //.split_whitespace()　そもそもスペースの入っていない文字列だった
            //.map(|s| s.parse().unwrap())
            .chars()
            .collect();
        grid.push(row);
    }

    let mut ans_count = 0;

    for h1 in 0..h {
        for h2 in h1..h {
            for w1 in 0..w {
                for w2 in w1..w {
                    let mut is_symmetric = true;

                    //以下はAIに教えてもらったコード
                    for i in h1..=h2 {
                        for j in w1..=w2 {
                            if grid[i][j] != grid[h1 + h2 - i][w1 + w2 - j] {
                                is_symmetric = false;
                                break;
                            }
                        }
                        if !is_symmetric {
                            break;
                        }
                    }
                    if is_symmetric {
                        ans_count += 1;
                    }
                }
            }
        }
    }

    ans_count
}

fn main() {
    println!("{}", square());
}
//0138_abc455_B_2二回目_自力で解けなかった