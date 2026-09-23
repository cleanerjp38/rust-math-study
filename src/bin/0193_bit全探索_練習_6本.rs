fn bit_search8() {
    let people  = ["A","b", "C", "D"];
    let n = people.len();

    for bit in 0..(1 << n) {//0..(1 << n)は0，1，10，11というように、ニ進数で1ずつ進む
        let mut count = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {//i桁目に1があるか。これをforで回しているので、ニ進数のbitにある1の数を数えている
                count += 1;
            }
        }
        println!("{:04b}:{}", bit, count);//04bで2進数の4桁表記
    }
}

fn bit_search9() {
    let people = ["A", "B", "C", "D"];
    let n = people.len();

    for bit in 0..(1 << n) {
        let mut count = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                count += 1;
            }
        }
        if count == 2 {//poepleのうち、2人が選ばれている場合、出力。つまり、ニ進数の中に1が二回出ていたら出力
            println!("{:04b}:{}", bit, count);
        }
    }
}

fn bit_search10 () {
    let a = [3, 8, 5, 12];
    let n  = a.len();
    let mut ans = 0;

    for bit in 0..(1 << n){
        let mut count = 0;
        let mut sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                count += 1;
                sum += a[i];
            }
        }
        if count == 2 {//数値を2つ選んだ内の最大値をansに入れている
            ans = ans.max(sum);
        }
    }

    println!("{}", ans);
}

fn bit_search11() {
    let prices = [3, 5, 6, 8];
    let values = [4, 7, 9, 13];
    let budget = 11;
    let n = 4;
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut count = 0;
        let mut prices_sum = 0;
        let mut values_sum = 0;

        for i in 0..n {
            if bit & (1 << i) != 0 {
                count += 1;
                prices_sum += prices[i];
                values_sum += values[i];
            }
        }
        if count <= 2 && prices_sum <= budget {//商品を2個以内、かつ価格の合計が11以下の場合
            ans = ans.max(values_sum);//価値の最大値をansに入れる
        }
    }

    println!("{}", ans);
}

fn bit_search12() {
    let a = [2, 4, 7, 9];
    let target = 11;
    let n = a.len();
    let mut count = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
            sum += a[i];
            }
        }
        if sum == target {//aの中で合計が11になる組み合わせの個数を数える
            count += 1;
        }
    }
    println!("{}", count);
}

fn bit_search13() {
    let a = [2, 3, 5, 8];
    let n = a.len();
    let mut ans = 0;

    for bit in 0..(1 << n) {
        let mut sum = 0;
        let mut one= false;

        for i in 0..n {
            if bit & (1 << i) != 0 && !one {
                sum += a[i];
                one = true;
            } else if bit & (1 << i) == 0 {
                one = false;
            }
        }
        ans = ans.max(sum);
    }

    println!("{}", ans);
}//失敗したコード
//本来は、「11」となった時点で、足すこともせずに次の処理に飛ぶ必要があった

fn main() {
    bit_search8();
    println!("");
    bit_search9();
    println!("");
    bit_search10();
    println!("");
    bit_search11();
    println!("");
    bit_search12();
    println!("");
    bit_search13();
}
//0193_bit全探索_練習_6本