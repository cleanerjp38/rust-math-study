use std::collections::VecDeque;

fn main() {
    let n: usize  = 7;
    let mut queue = VecDeque::new();
    let mut dist = vec![-1; n + 1];
    queue.push_back(0);
    dist[0] = 0;

    while let Some(x) = queue.pop_front() {
        //ここで x + 1 と x + 3 を調べる
        let next1 = x + 1;
        let next2 = x + 3;

        // N以下か？まだ訪問していないか？
        if next1 <= n && dist[next1] == -1 {
            dist[next1] = dist[x] + 1;// 距離を記録する
            queue.push_back(next1);// キューに追加する
        }

        // N以下か？まだ訪問していないか？
        if next2 <= n && dist[next2] == -1 {
            dist[next2] = dist[x] + 1;
            queue.push_back(next2);
        }
    }

    println!("{}", dist[n]);
}
//0226_BFS_練習_1本
// これから調べる数字を入れる箱を用意する。
// それぞれの数字に、何回で到達したかを記録する。
// 最初の数字0を箱に入れる。
// 箱から数字を1つ取り出し、そこから行ける数字を調べる。
// 初めて見つけた数字なら、距離を記録して箱に追加する。