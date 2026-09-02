use std::collections::HashMap;

fn byte_frequency(input: &str) -> HashMap<u8, usize> {
    let mut map = HashMap::new();
    
    // ここに実装を書いてください
    // input.as_bytes() を使って u8 の配列として処理します
    //「&i」は、iter()で流れてくる要素が参照なので、&iをつけることで、for内で「i」と書くと参照されていないiを使える
    for &i in input.as_bytes().iter() {
        //entry()は、流れてくるu8にたいして情報を付け加える
        //or_insertは、付加する情報の初期値
        *map.entry(i).or_insert(0) += 1;
    }
    map
}

fn take_until_comma(input: &str) -> Vec<u8> {
    let bytes = input.as_bytes();
    let mut result: Vec<u8> = Vec::new();
    let mut i = 0;
    
    // ここに while ループを使った実装を書いてください
    // 条件式では論理積（ && ）を使って2つの条件を結びつけます

    //bytes[i] != b','だけでもいいかと思ったが、文字列に','がない場合は無限ループになってしまう
    while i < bytes.len() && bytes[i] != b',' {
        result.push(bytes[i]);
        i += 1;
    }
    result
}

fn shift_bytes(input: &str) -> Vec<u8> {
    // ここに map を使った実装を書いてください
    input
        .as_bytes()//&[u8]の配列が生成される
        .iter()//イテレータによって&u8が一つずつ流れてくる
        .map(|&b| b + 1)//&bで参照を外し、+1でbyteに1を足している
        .collect::<Vec<u8>>()//それを配列に詰め直し、「;」をつけないことで返り値とする
}

fn main() {
    let result = byte_frequency("abaca");
    
    // b'a' は 3回出現する
    assert_eq!(result.get(&b'a'), Some(&3));
    // b'b' は 1回出現する
    assert_eq!(result.get(&b'b'), Some(&1));
    // b'z' は 存在しないので None
    assert_eq!(result.get(&b'z'), None);
    
    println!("すべてのテストをパスしました！");

//-----------------------------------
        // カンマの前までを取得
    assert_eq!(
        take_until_comma("apple,banana"), 
        vec![b'a', b'p', b'p', b'l', b'e']
    );
    
    // カンマが無い場合は最後まで取得
    assert_eq!(
        take_until_comma("hello"), 
        vec![b'h', b'e', b'l', b'l', b'o']
    );
    
    // 先頭がカンマの場合は空のVecが返る
    assert_eq!(
        take_until_comma(",rust"), 
        vec![]
    );
    
    println!("すべてのテストをパスしました！");

//--------------------------------------
        // b'a'(97) に 1 を足すと b'b'(98) になる
    assert_eq!(
        shift_bytes("abc"), 
        vec![b'b', b'c', b'd']
    );
    
    // 名作SFの有名なコンピュータ名のオマージュです
    assert_eq!(
        shift_bytes("HAL"), 
        vec![b'I', b'B', b'M']
    );
    
    println!("すべてのテストをパスしました！");
}
//0170_u8_while_HashMap_map_練習