use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(n);
    let mut subgrid = HashSet::new();

    for _ in 0..n {
        let row = iter.next().unwrap().chars().collect();
        grid.push(row);
    }

    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut pattern = Vec::new();
            for x in i..i + m {
                for y in j..j + m {
                    pattern.push(grid[x][y]);
                }
            }
            subgrid.insert(pattern);
        }
    }

    let ans = subgrid.iter().count();

    println!("{}", ans);
}
//0187_abc430_B