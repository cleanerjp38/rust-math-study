use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut s_char: Vec<char> = input.trim().chars().collect();

    //数え上げにはHashMapを使うとよい
    let mut count_map: HashMap<char, usize> = HashMap::new();
    
    for &c in &s_char {
        //ここは*count_mapにしないとエラーになる。なんでだろう？
        //count_mapでは駄目
        *count_map
            .entry(c)//[c]という文字の箱を見る。ここではs_charから流れてくる1文字
            .or_insert(0) += 1;//その箱が空ならば、0を箱に入れる
    }

    let max_count = *count_map
        .values()//HashMapで数値だけの要素を取り出すのかな？
        .max()//その数値の中の最大値を返す
        .unwrap();//max()には「数値が存在しないOption」があるため、unwrap()を通す、のかな？

    //let result = s_char
    //retain()は「新しく削った配列を返す」わけではなく、「配列から”指定されたものを残す”」。
    //なので、let resultには空の要素しか入らない。
    s_char
        //count_map[&c]はcの出現回数を抜き出す
        //count_map[&c] == max_countでcount_map内の最大回数のcを指定、"!"でそれをひっくり返す
        .retain(|c| count_map[&c] != max_count);

        let result: String = s_char.into_iter().collect();
    println!("{}", result);
}
//0150_abc447_B_2回目