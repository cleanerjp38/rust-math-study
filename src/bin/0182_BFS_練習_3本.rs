    use std::collections::VecDeque;
    use std::io::{self,Read};

    fn bfs1() {
        let mut queue = VecDeque::new();
        let mut graph = vec![Vec::new(); 9];
        let mut count = 0;
        let mut visited = vec![false; 9];

        graph[1].push(2);
        graph[1].push(3);
        graph[2].push(4);
        graph[3].push(5);
        graph[5].push(6);
        graph[7].push(8);

        visited[1] = true;
        queue.push_back(1);

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

    fn bfs2() {
        let mut queue = VecDeque::new();
        let mut graph = vec![Vec::new(); 8];
        let mut dist = vec![-1; 8];

        graph[1].push(2);
        graph[1].push(3);
        graph[2].push(4);
        graph[3].push(4);
        graph[4].push(5);
        graph[5].push(6);
        graph[3].push(7);

        queue.push_back(1);
        dist[1] = 0;

        while let Some(x) = queue.pop_front() {
            for &next in &graph[x] {
                if dist[next] == -1 {
                    dist[next] = dist[x] + 1;
                    //dist[next] += dist[x] + 1;だと、回答が全て0になった。何が違うんだろ？
                    queue.push_back(next);
                }
            }
        }

        for i in 1..=7 {
            println!("{}:{}", i, dist[i]);
        }
    }

    fn bfs3() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        let mut queue = VecDeque::new();
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut count = 0;
        let mut visited = vec![false; n + 1];

        for _ in 0..m {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            graph[a].push(b);
        }

        visited[1] = true;//これが抜けていた
        queue.push_back(1);

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

fn main() {
    bfs1();
    bfs2();
    bfs3();
}
//0182_BFS_練習_3本