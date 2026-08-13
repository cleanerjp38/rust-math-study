use std::time::Instant;

fn for1() {
    let mut sum = 0;
    let start = Instant::now();
    for  i in 1..=100 {
        sum += i;
    }

    let d = start.elapsed();
    println!("{}", sum);
    println!("time:{:?}", d);
}

fn iter1() {
    let start = Instant::now();
    let sum: i32 = (1..=100).sum();

    let d = start.elapsed();
    println!("{}", sum);
    println!("time:{:?}", d);
}

fn for2() {
    let (x, y) = (48, 8);
    let mut ans = false;

    let start = Instant::now();
    for i in 0..100 {
        if (x + i) % (y + i) == 0 {
            ans = true;
            break;
            //breakはループから抜ける。returnは関数から抜ける
            //return; こう書いていて、この関数が動かなくて謎だった
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
    let d = start.elapsed();
    println!("time:{:?}", d);
}

fn iter2() {
    let (x, y) = (48, 8);

    let s = Instant::now();
    let ans = (0..100)
    .any(|i| (x + i) % (y + i) == 0);
    
    if ans {
        println!("yes");
    } else {
        println!("no");
    }

    let d = s.elapsed();
    println!("time:{:?}", d);
}

fn main() {
    for1();
    iter1();
    for2();
    iter2();
}
//0142_for_→iter_2本