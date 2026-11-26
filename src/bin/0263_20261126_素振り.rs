fn suburi_1() {
    let _n = 5;
    let a = [-30, 50, -80, -40, 25];
    let mut battery = 100;

    for i in a {
        battery += i; //前回のコードでは”battery = battery - i”になっていた
        
        if battery > 100 {
            battery = 100;
        } else if battery < 0 {
            battery = 0;
        }
    }

    println!("{battery}");
}
//battery:バッテリー残量
// もし、バッテリー＞100なら、バッテリー＝100
// バッテリー＜0なら、バッテリー＝0
// for文でAを流していきバッテリー残量を加減していく

fn suburi_2() {
    let _n = 6;
    let k = 10;
    let a = [3, 4, 3, 8, 5, 2];

    let mut tank = 0;//貯水量
    let mut count = 0;//タンクの排水回数

    for i in a {
        tank += i;

        if tank >= k {
            tank = 0;
            count += 1;
        }
    }

    // println!("{tank} {count}");
    println!("{count} {tank}");
}
// タンクに水を足していく
// タンク＞＝kとなったらタンク＝0、排水回数を1カウント

fn suburi_3() {
    let _n = 8;
    let a = [10, 12, 15, 14, 16, 18, 20, 17];

    let mut yesterday = a[0];//昨日の気温
    let mut count = 0;//昨日より温度が高かった回数
    let mut ans = 0;//countの最大値

    for &today in a.iter().skip(1) {
        if today - yesterday > 0 {
            count += 1;
        } else {
            count = 0;
        }

        yesterday = today;
        ans = ans.max(count);
    }

    println!("{ans}");
}
// 初日は比較しないので、yesterdayに初日の数値を入れる
// 昨日より今日のほうが気温が大きい、つまり今日ー昨日>0
// その場合、カウントを1増やす。そうでない場合はカウントを0にする
// 最大値と現在のカウント数を比較する

fn main() {
    suburi_1();
    suburi_2();
    suburi_3();
}
//0263_20261126_素振り