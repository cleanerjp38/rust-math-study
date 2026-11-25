fn suburi_1() {
    let _n = 5;
    let a = [-30, 50, -80, -40, 25];
    let mut battery = 100;

    for i in a {
        battery = battery - i;
        
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

    println!("{tank} {count}");
}
// タンクに水を足していく
// タンク＞＝kとなったらタンク＝0、排水回数を1カウント

fn suburi_3() {
    let _n = 8;
    let a = [10, 12, 15, 14, 16, 18, 20, 17];

    let mut today = 0;//今日の気温
    let mut yesterday = i32::MIN;//昨日の気温
    let mut count = 0;//昨日より温度が高かった回数

    for i in a {
        today = i;

        if today - yesterday > 0 {
            count += 1;
        }

        yesterday = today;
    }

    println!("{count}");
}
//初日は前日の気温はないので、最小値にしておく
// forで今日の気温を受け取り、todayに入れる
// 今日ー昨日＞0なら、count += 1
// for文の最後に、昨日＝今日で要素を入れる


fn main() {
    suburi_1();
    suburi_2();
    suburi_3();
}
//0262_20261125_素振り

//1.WA 2.WA 3.WA