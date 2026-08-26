fn while1() {
    let mut n = 3;

    let mut count = 0;
    //whileは「～でなくなる」ときまで回すのか、なーるほど
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
            //n /= 2; とも書ける
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    println!("{}", count);
}

fn while2(mut n: u32) -> u32 {
    let mut sum = 0;

    while n != 0 {
        sum += n % 10;//10でわった余りは一桁の数値
        n /= 10;//u32を10でわると、小数点は切り捨てになる
    }

    sum
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    // ここに while ループを使って実装してみてください
    // ヒント：a と b の値を同時に更新する際、一時的に値を保存する変数が必要になるかもしれません。

    //ユークリッドの互除法、知らなかった
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
        //(a, b) = (b, a % b);←上の式はこの1行で書けるそうだ。すごい
    }

    a
}

fn is_palindrome(arr: &[i32]) -> bool {
    // 空の配列や要素が1つの配列は、回文とみなす
    if arr.len() <= 1 {
        return true;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    // ここに while ループを使って実装してみてください
    // ヒント1：left が right より小さい間、ループを続けます。
    // ヒント2：もし arr[left] と arr[right] が違っていたら、その時点で回文ではないので false を返します。
    // ヒント3：同じなら、left を 1 増やし、right を 1 減らして次を調べます。

    //left != rightだとコンパイルエラー
    //要素数が2の場合、left = 0 + 1, right = 1 - 0, となり、leftがrightを追い抜いて無限ループに入るため
    while left < right {
        if arr[left] != arr[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    // ループを無事に抜けられたら、すべて一致したということ！
    true
}

fn binary_search(arr: &[i32], target: i32) -> bool {
    if arr.is_empty() {
        return false;
    }

    let mut left = 0;
    // Rustでは、右端を「配列の長さ」にしておく（半開区間）のが安全で美しいとされています
    let mut right = arr.len(); 

    // ここに while ループを使って実装してみてください
    // ヒント1：真ん中のインデックスは `let mid = left + (right - left) / 2;` で計算できます。
    // （ `(left + right) / 2` でも動きますが、上記の方が桁あふれを防げる安全な書き方です）

    while left < right {
        let mid = left + (right - left) / 2;

        //以下のif、matchで書けそうだな…
        if arr[mid] == target {
            return true;
        }
        if arr[mid] < target {
            left = mid + 1;
        } else if arr[mid] > target {
            right = mid;
        }
    }
    // 見つからずにループを抜けたら false
    false
}

fn run_length_encoding(arr: &[char]) -> Vec<(char, usize)> {
    let mut result = Vec::new();

    //for mut i in 0..arr.len() {　forで行けるかと思って試してみたが、iがリセットされるので上手くいかなかった
    let mut i = 0;
    while i < arr.len() {
        let current_char = arr[i];
        let mut count = 0;

        // ここに内側の while ループを実装してみてください
        // ヒント1：i が arr.len() より小さく、かつ arr[i] が current_char と同じ間ループします。
        // ヒント2：条件を満たしている間は、count と i をそれぞれ 1 ずつ増やします。
        while i < arr.len() && arr[i] == current_char {
            count += 1;
            i += 1;
        }

        result.push((current_char, count));
    }

    result
}

fn main() {
    while1();
    println!("Example 1: {}", while2(1234)); // 10 が出力されればOK
    println!("Example 2: {}", while2(805));  // 13 が出力されればOK
    println!("Example 3: {}", while2(7));    // 7 が出力されればOK
    
    println!("Example 1: {}", gcd(24, 18));     // 6 が出力されればOK
    println!("Example 2: {}", gcd(1071, 1029)); // 21 が出力されればOK
    println!("Example 3: {}", gcd(14, 35));     // 7 が出力されればOK

    let arr1 = vec![1, 2, 3, 2, 1];
    println!("Example 1: {}", is_palindrome(&arr1)); // true が出力されればOK

    let arr2 = vec![1, 2, 3, 3, 1];
    println!("Example 2: {}", is_palindrome(&arr2)); // false が出力されればOK

    let arr3 = vec![5, 5];
    println!("Example 3: {}", is_palindrome(&arr3)); // true が出力されればOK

    let arr = vec![10, 20, 30, 40, 50, 60, 70];
    
    println!("Example 1: {}", binary_search(&arr, 40)); // true が出力されればOK
    println!("Example 2: {}", binary_search(&arr, 45)); // false が出力されればOK
    println!("Example 3: {}", binary_search(&arr, 10)); // true (左端) が出力されればOK
    println!("Example 4: {}", binary_search(&arr, 70)); // true (右端) が出力されればOK

    let arr1 = vec!['A', 'A', 'A', 'B', 'B', 'C'];
    // [('A', 3), ('B', 2), ('C', 1)] が出力されればOK
    println!("Example 1: {:?}", run_length_encoding(&arr1));

    let arr2 = vec!['X', 'Y', 'Y', 'Z'];
    // [('X', 1), ('Y', 2), ('Z', 1)] が出力されればOK
    println!("Example 2: {:?}", run_length_encoding(&arr2));
}
//0159_while_練習_6本