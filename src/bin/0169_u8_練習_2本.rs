fn count_byte(input: &str, target: u8) -> usize {
    // ここに実装を書いてください

    let s_vec: Vec<u8> = input.as_bytes().to_vec();
    let result = s_vec.iter().filter(|&s| s == &target).count();

    result
}

fn replace_byte(input: &str, target: u8, replacement: u8) -> Vec<u8> {
    // ここに実装を書いてください
    let s_vec: Vec<u8> = input.as_bytes().to_vec();

    let mut result: Vec<u8> = Vec::with_capacity(s_vec.capacity());
    for s in s_vec.iter() {
        if s == &target {
            result.push(replacement);
        } else {
            result.push(*s);
        }
    }

    result
}

fn main() {
    // b'p' や b'a' は u8 型の数値です
    assert_eq!(count_byte("apple", b'p'), 2);
    assert_eq!(count_byte("banana", b'a'), 3);
    assert_eq!(count_byte("hello", b'z'), 0);
    assert_eq!(count_byte("", b'a'), 0);
    
    println!("すべてのテストをパスしました！");

    // "hello" の 'l' を 'x' に置き換えると "hexxo" のバイト列になる
    assert_eq!(
        replace_byte("hello", b'l', b'x'),
        vec![b'h', b'e', b'x', b'x', b'o']
    );

    // "banana" の 'a' を '-' に置き換える
    assert_eq!(
        replace_byte("banana", b'a', b'-'),
        vec![b'b', b'-', b'n', b'-', b'n', b'-']
    );

    // ターゲットが存在しない場合は元のままのバイト列が返る
    assert_eq!(
        replace_byte("rust", b'z', b'x'),
        vec![b'r', b'u', b's', b't']
    );

    println!("すべてのテストをパスしました！");
}
//0169_u8_練習_2本