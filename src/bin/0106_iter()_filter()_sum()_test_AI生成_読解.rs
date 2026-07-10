fn main() {
    //vec!で配列を作る
    let numbers = vec![1, 2, 3, 4, 5];

    let mut sum_even = 0;
    //forで偶数のみを足す
    //in &numberでその配列の要素一つ一つを取り出すのいいよね
    for n in &numbers {
        //2で割って余りなし
        if n % 2 == 0 {
            sum_even += n;
        }
    }
    println!("{}", sum_even);

    //iter()で配列の要素を一つずつ流している
    let iter_sum: i32 = numbers.iter()
        //filter()でnを仕分けする
        //(|&n| n % 2 == 0)は数学の{変数|条件式}と同じ
        //|&n|なのは、iter()は参照でありfilter()も参照するので、&&i32になってしまう
        //そこで|&n|と限定することで、&i32に戻している
        .filter(|&n| n % 2 == 0)
        //仕訳した要素を足す
        //sum()は多分、イテレータにしか使えなさそう
        //イテレータにしてないと、流れ落ちた要素は消えてなくなってしまうから
        .sum();
    println!("{}", iter_sum);
}
//0106_iter()_filter()_sum()_test_AI生成_読解