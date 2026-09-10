use std::collections::VecDeque;

fn queue1() {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    while let Some(x) = queue.pop_front() {
        println!("{}", x);
    }
}

fn queue2() {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    while let Some(x) = queue.pop_front() {
        if x == 20 {
            queue.push_back(40);
            queue.push_back(50);
        }
        println!("{}", x);
    }
}

fn queue3() {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(1);

    while let Some(mut x) = queue.pop_front() && x <= 5 {
        println!("{}", x);
        x += 1;
        queue.push_back(x);
    }
}

fn queue4() {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(1);

    while let Some(x) = queue.pop_front() && x <= 7 {
        println!("{}", x);
        if x < 4 {
            queue.push_back(x * 2);
            queue.push_back(x * 2 + 1);
        }
    }
}

fn queue5() {
    let mut graph  = vec![Vec::new(); 5];
    graph[1].push(2);
    graph[1].push(3);
    graph[2].push(4);
    graph[3].push(4);
    graph[4].push(1);

    let mut visited = vec![false; 5];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[1] = true;
    queue.push_back(1);
    while let Some(x) = queue.pop_front() {
        println!("{}", x);
        for &next in &graph[x] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
                //println!("{}", x); ここに出力を書いたらおかしくなった
                //ここに書くと、キューに詰める時に飛ばされた数値が出力されない
            }
        }
    }
}

fn main() {
    queue1();
    queue2();
    queue3();
    queue4();
    queue5();
}
//0179_VecDeque_練習_5本