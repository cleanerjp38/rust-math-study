use std::cmp::Ordering;

fn count_subarrays(arr: &[i32], k: i32) -> usize {
    let mut right = 0;
    let mut current_sum = 0;
    let mut count = 0;

    for left in 0..arr.len() {
        // ヒント1：前回の尺取法と同じように、次に足しても K 以下なら right を進め、current_sum に足す
        // （ここに while ループを書く）
        while right < arr.len() && current_sum + arr[right] <= k {
            current_sum += arr[right];
            right += 1;
        }

        // ヒント2：right が進みきったので、left を左端とする部分列の個数を count に足す
        // count += ???;
        count += right - left;

        // ヒント3：次のループで left が進むための準備
        // 例外処理 (left == right) と、通常の current_sum からの引き算
        if left == right {
            //current_sum = 0; current_sumってリセットしなくていいのか？
            right += 1; //leftがrightを追い抜かないようにする
        } else {
            current_sum -= arr[left];
        }
    }

    count
}

fn has_target_sum(arr: &[i32], target: i32) -> bool {
    if arr.len() < 2 {
        return false;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    // ここに while ループを使って実装してみてください
    // ヒント1：left が right より小さい間ループを続けます。
    // ヒント2：let current_sum = arr[left] + arr[right];
    // ヒント3：current_sum と target を cmp で比較して match で分岐してみましょう。
    while left < right {
        let current_sum = arr[left] + arr[right];
        match current_sum.cmp(&target) {
             Ordering::Equal => return true,
             Ordering::Greater => right -= 1,
             Ordering::Less => left += 1,
        }
    }

    false
}

fn is_subsequence(a: &[i32], b: &[i32]) -> bool {
    // A が空配列の場合は常に true とする（何も選ばないという部分列）
    if a.is_empty() {
        return true;
    }

    let mut i = 0; // 配列 A 用のポインタ
    let mut j = 0; // 配列 B 用のポインタ

    // ここに while ループを使って実装してみてください
    // ヒント1：i と j がそれぞれ配列の長さ未満である間、ループを続けます。（両方の条件を満たす間）
    // ヒント2：もし a[i] と b[j] が同じなら、i を 1 進めます。
    // ヒント3：b[j] が同じであっても違っても、B の探索は前に進めるので j は必ず 1 進めます。
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    // ループを抜けた後、i が A の長さと同じになっていれば、
    // A の要素をすべて順番通りに見つけられたということ！
    i == a.len()
}

fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    // 答えを格納するベクタ（あらかじめ必要な長さを確保しておくと少し高速です）
    let mut result = Vec::with_capacity(a.len() + b.len());
    
    let mut i = 0;
    let mut j = 0;

    // 1. 両方の配列に要素が残っている間、小さい方を result に push していく
    // （ここに 1つ目の while ループを書く）
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else if a[i] > b[j] {
            result.push(b[j]);
            j += 1;
        } else {
            result.push(a[i]);
            result.push(b[j]);
            i += 1;
            j += 1;
        }
    }


    // 2. もし A の方に要素が残っていたら、残りをすべて result に push する
    // （ここに 2つ目の while ループを書く）
    while i < a.len() {
        result.push(a[i]);
        i += 1;
    }


    // 3. もし B の方に要素が残っていたら、残りをすべて result に push する
    // （ここに 3つ目の while ループを書く）
    if j < b.len() {
        //whileとextend_from_slice()を使い分けてみた
        result.extend_from_slice(&b[j..]);
    }

    result
}

fn move_zeroes(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr.len());
    let mut i = 0;

    // 1. まず、0 以外の数字だけを result に集める
    // ヒント：i が arr.len() より小さい間ループし、0 でなければ push する。
    // i を進めるのを忘れずに！
    while i < arr.len() {
        if arr[i] != 0 {
            result.push(arr[i]);
        }
        i += 1;
    }


    // 2. 足りない分の 0 を末尾に追加する
    // ヒント：現在の「result の長さ」が「元の配列 arr の長さ」より小さい間ループし、0 を push する。
    // ここで前回の教訓が活きます！何を条件にすれば無限ループにならないか？
    while result.len() < result.capacity() {
        result.push(0);
    }

    result
}

fn main() {
    let arr1 = vec![1, 2, 3];
    let k1 = 3;
    println!("Example 1: {}", count_subarrays(&arr1, k1)); // 4 が出力されればOK

    let arr2 = vec![4, 1, 3, 2, 5];
    let k2 = 6;
    println!("Example 2: {}", count_subarrays(&arr2, k2)); // 9 が出力されればOK

    let arr = vec![1, 2, 4, 7, 11, 15];
    
    println!("Example 1: {}", has_target_sum(&arr, 15)); // true
    println!("Example 2: {}", has_target_sum(&arr, 10)); // false
    println!("Example 3: {}", has_target_sum(&arr, 8));  // true (1 + 7)

    let a1 = vec![1, 3, 5];
    let b1 = vec![1, 2, 3, 4, 5];
    println!("Example 1: {}", is_subsequence(&a1, &b1)); // true

    let a2 = vec![1, 5, 3];
    let b2 = vec![1, 2, 3, 4, 5];
    println!("Example 2: {}", is_subsequence(&a2, &b2)); // false

    let a3 = vec![4, 4];
    let b3 = vec![1, 4, 2, 4, 3];
    println!("Example 3: {}", is_subsequence(&a3, &b3)); // true

    let a1 = vec![1, 3, 5];
    let b1 = vec![2, 4, 6, 7];
    // [1, 2, 3, 4, 5, 6, 7] が出力されればOK
    println!("Example 1: {:?}", merge_sorted_arrays(&a1, &b1));

    let a2 = vec![10, 20];
    let b2 = vec![5, 15, 25, 35];
    // [5, 10, 15, 20, 25, 35] が出力されればOK
    println!("Example 2: {:?}", merge_sorted_arrays(&a2, &b2));

    let arr1 = vec![0, 1, 0, 3, 12];
    println!("Example 1: {:?}", move_zeroes(&arr1)); // [1, 3, 12, 0, 0]

    let arr2 = vec![1, 2, 3];
    println!("Example 2: {:?}", move_zeroes(&arr2)); // [1, 2, 3]

    let arr3 = vec![0, 0, 0];
    println!("Example 3: {:?}", move_zeroes(&arr3)); // [0, 0, 0]
}
//0160_while_練習_5本