use std::io;

fn horses() -> Vec<u32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    //Nは使わないので、clear()でbufを空にする
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let t_line= buf.trim().split_whitespace();
    let t :Vec<u32> = t_line.map(|a| a.parse().unwrap()).collect();

    t
}

fn winner(t: Vec<u32>) -> (usize, usize, usize) {
    let mut pairs: Vec<(usize, u32)> = t
        //iter()は参照で返すが、into_iter()は所有権ごと渡す
        .into_iter()
        .enumerate()
        //map()にはこういう書き方もあるのか！！
        .map(|(idx, time)| (idx + 1, time))
        .collect();

    //sort_by_key()はタプルや構造体のどこの要素をソートするか指定できる
    //p.1はpタプルの2番目の要素を元に、昇順で並び替える
    //降順はreverse()か(|p| -(p.1 as i32))と書く
    pairs.sort_by_key(|p| p.1);

    (pairs[0].0, pairs[1].0, pairs[2].0)
}

fn main() {
    let t = horses();
    let (first, second, third) = winner(t);
    println!("{} {} {}", first, second, third);
}
//0121_abc440_B_failed


//----以下は、ifで順位を決めようとして失敗したコード----
//ソートでよかったし、なんならこの書き方でも不正解だった
//fn winner(t: Vec<u32>) {
//    let mut first = 0;
//    let mut second = 0;
//    let mut third = 0;
//    let mut first_idx: usize = 0;
//    let mut second_idx: usize = 0;
//    let mut third_idx: usize = 0;

//    for (idx, i) in t.iter().copied().enumerate(){
//        if third > i {
//            if second > i {
//                if first > i{
//                    second = first;
//                    second_idx = first_idx;
//                    first = i;
//                    first_idx = idx;                  
//                }
//            } else {
//                third = second;
//                third_idx = second_idx;
//                second = i;
//                second_idx = idx;
//            }
//        } else {
//            third = i;
//            third_idx = idx;
//        }
//    }
//    println!("{} {} {}", first_idx, second_idx, third_idx);
//}

