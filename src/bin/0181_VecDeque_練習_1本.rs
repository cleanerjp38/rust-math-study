use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    let mut graph = vec![Vec::new(); 8];
    let mut dist = vec![-1; 8];

    dist[1] = 0;
    queue.push_back(1);

    graph[1].push(2);
    graph[1].push(3);
    graph[2].push(4);
    graph[3].push(5);
    graph[4].push(6);
    graph[5].push(7);

    while let Some(x) = queue.pop_front() {
        for &next in &graph[x] {
            if dist[next] == -1 {
                //----間違えたコード----
                //if next <= 6 {
                    //dist[next] = dist[x] + 1;　ここでdist[next]を加算するために、<=6という条件式になっている
                    //queue.push_back(next);　しかし、6で探索終了なのに、ここでキューに詰める作業は無駄。なので、以下の正解コードのほうが効率がよい
                //} else {
                    //println!("{}", dist[6]);
                    //return;
                //}
                dist[next] = dist[x] + 1;
                if next == 6 {
                    println!("{}", dist[next]);
                    return;//キューに詰める前にreturn
                }
                queue.push_back(next);
            }
        }
    }
}
//0181_VecDeque_練習_1本