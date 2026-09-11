use std::collections::VecDeque;

fn queue6() {
    let mut queue:VecDeque<u32> = VecDeque::new();
    queue.push_back(5);
    queue.push_back(10);
    queue.push_back(15);

    while let Some(x) = queue.pop_front() {
        println!("{}", x);
    }
}

fn queue7() {
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    while let Some(x) = queue.pop_front() {
        println!("{}", x);
        if x == 2 {
            queue.push_back(4);
            queue.push_back(5);
        }
    }
}

fn queue8() {
    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(x) = queue.pop_front() {
        println!("{}", x);
        if x < 6 {
            queue.push_back(x + 1);
        }
    }
}

fn queue9() {
    let mut queue = VecDeque::new();
    queue.push_back(1);

    while let Some(x) = queue.pop_front() && x <= 7 {
        println!("{}", x);
        if x < 4 {
            queue.push_back(x * 2);
            queue.push_back(x * 2 + 1);
        }
    }
}

fn queue10() {
    let mut queue = VecDeque::new();
    let mut graph = vec![Vec::new(); 5];
    graph[1].push(2);
    graph[1].push(3);
    graph[2].push(4);
    graph[3].push(4);
    graph[4].push(1);
    
    let mut visited = vec![false; 5];
    visited[1] = true;
    queue.push_back(1);

    while let Some(x) = queue.pop_front() {
        println!("{}", x);
        for &next in &graph[x] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
}

fn queue11() {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut graph = vec![Vec::new(); 7];
    let mut visited = vec![false; 7];
    let mut count = 0;

    graph[1].push(2);
    graph[1].push(3);
    graph[2].push(4);
    graph[3].push(5);
    graph[5].push(6);

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

fn queue12() {
    let mut queue = VecDeque::new();
    let mut graph = vec![Vec::new(); 7];
    let mut dist = vec![-1; 7];

    graph[1].push(2);
    graph[1].push(3);
    graph[2].push(4);
    graph[3].push(5);
    graph[4].push(6);
    graph[5].push(6);

    dist[1] = 0;
    queue.push_back(1);
    while let Some(x) = queue.pop_front() {
        for &next in &graph[x] {
            if dist[next] == -1 {
                //dist[next] += 1;
                dist[next] = dist[x] + 1;
                queue.push_back(next);
            }
        }
    }

    for i in 1..=6 {
        println!("{}:{}", i, dist[i]);
    }
}

fn main(){
    queue6();
    queue7();
    queue8();
    queue9();
    queue10();
    queue11();
    queue12();
}
//0180_VecDeque_練習_7本