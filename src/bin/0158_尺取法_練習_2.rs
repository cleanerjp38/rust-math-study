fn max_subarray_length(arr: &[i32], x: i32) -> usize {
    let mut right = 0;
    let mut current_sum = 0;
    let mut max_len = 0;

    for left in 0..arr.len() {
        //while current_sum <= x {　←間違えたコード
        //↓正直、なぜこの書き方なのか理解できていない
        while right < arr.len() && current_sum + arr[right] <= x {
            current_sum += arr[right];
            right += 1;
        }

        if max_len < right - left {
            max_len = right - left;
        }

        //----AIのコードの写経部分。この書き方も定型文っぽい----
        if right == left {
            right += 1;
        } else {
            current_sum -= arr[left];
        }
        //---------
    }
    max_len
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let x1 = 11;
    println!("Example 1: {}", max_subarray_length(&arr1, x1)); // 4が出力されればOK

    let arr2 = vec![8, 1, 4, 2, 10, 3];
    let x2 = 12;
    println!("Example 2: {}", max_subarray_length(&arr2, x2)); // 3が出力されればOK
}
//0158_尺取法_練習_2