fn chuumon() {
    let mut juice: Vec<u32> = (1..=10).collect();
    let kyaku: Vec<u32> = (1..=5).collect();

    //1個ずつ客にジュースを渡そうとした
    //for i in &kyaku {
        //for内で1個ずつチェックする時にiter()を使うのは、コンパイラが起こるらしい
        //if kyaku[i] == juice.iter().filter(i) {
            //println!("{}", i);
            //juice.iter().filter(i).
        //} else {
        //}
    //retain()<-保持する　!kyaku.contains(j)<-kyakuが保持している数値以外を
    juice.retain(|j| !kyaku.contains(j));
     println!("残りジュース: {:?}", juice);
}

fn main() {
    chuumon();
}
//0110_retain()_contain()_test