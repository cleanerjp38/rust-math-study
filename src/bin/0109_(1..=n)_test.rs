fn hairetu() {
    let n = 5;
    //vec![1..(n as u32) ; n]と書くと、1~n-1までの範囲(Range)になるから、1個の要素ではないらしい
    //let ha = vec![1..(n as u32) ; n];
    //(1..=n)でイテレータとなるため、collect()で配列に出来る
    let ha: Vec<u32> = (1..=n).collect();
    for i in &ha {
        print!("{}", i);
    }
}

fn main() {
    hairetu();
}
//0109_(1..=n)_test