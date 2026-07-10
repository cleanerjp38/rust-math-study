fn main() {
    let mut sum_even = 0;
    //2次元配列ではどうなるのかやってみたかった
    let numbers = vec![vec![1, 2, 3, 4, 5]; 3];

    for n in &numbers {
        //nという配列をmが一つ一つの要素として受け取っている！
        for m in n {
            if m % 2 == 0 {
                sum_even += m;
            }
        }
    }
    println!("{}", sum_even);

    //----ここからAI生成コードと俺のコメント----
    //let iter_sum = numbers.iter()と書いたら、「:i32を記載しろ」ってコンパイラエラーが出た
    //おそらく、生成された要素が&i32だったから受け取れなくて、i32と指定するよう要求されたのかな？
    //どうやら、sum()はu32やi64といったいろんな型に変換して返せるようだ
    //なので、受け取り側は型を指定して受け取る必要がある
    let iter_sum: i32 = numbers.iter()
        //flatten()はイテレータからの配列を一本の流れにする
        .flatten()
        .filter(|&m| m % 2 == 0)
        .sum();
    println!("{}", iter_sum);
}
//0107_iter()_flatten()_2次元配列_test