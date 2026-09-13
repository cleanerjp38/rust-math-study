use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut queue = VecDeque::new();
    let mut graph = vec![Vec::new(); n + 1];
    let mut count = 0;
    let mut visited = vec![false; n + 1];

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        graph[a].push(b);
    }

    queue.push_back(1);
    visited[1] = true;

    while let Some(x) = queue.pop_front() {
        count += 1;
        
        for &next in &graph[x] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    println!("{}", count);
}
//0183_abc454_C