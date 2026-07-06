use std::io::{self, BufRead, Write};//Writeって始めて見た
//stdout()を使うときはWriteトレイトが必要なのだろう

//構造体の中に構造体を作ってみたかった
struct Hero {
    name: String,
    status: HeroStatus,
    health: HeroHealth,
}

struct HeroStatus {
    hp: i32,
    mp: i32,
    strength: u32,
    toughness: u32,
    speed: u32,
    luck: u32,
}

//enumは、選択肢の中にデータを内包できる
//これ便利そうだけど、なんでstructでは駄目なのだろうか？HeroStatusのように構造体の入れ子にしたほうが安全なのかな？
enum HeroHealth {
    Normal,
    Staned,
    //ここで毒ダメージの設定ができるようになっている
    Poisoned {damage_per_turn: u32},
}

//ステータスを入力できるようにしたかった
fn input_status() -> HeroStatus {
    let mut stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();

    //lines.next()の時点で、標準入力の受け付けが始まるので、その前に入力案内を表示させる
    println!("ステータスを入力してください");
    print!(" HP: ");
    //↓がないと入力前にprintが表示されないそうだ。今度検証しよう
    io::stdout().flush().unwrap();

    let hp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    //ここ、逐一の入力なら、分ける必要はなかったか？
    let hp: i32 = hp_str.trim().parse().expect("数値を入力してください");


    //スコープ外にmpを出しておく
    let mp: i32; 
    //数値じゃない文字を入れたときの処理
    loop {
        print!(" MP: ");
        io::stdout().flush().unwrap();
        let mp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");

        //matchはいろんな書き方ができるんだな
        match mp_str.trim().parse::<i32>() {
            //Ok()の引数の意味がよくわからん
            Ok(num) => {
                mp = num;
                break;
            }
            //エラーだとbreakしないので、もう一度やり直し
            Err(_) => {
                println!("無効な入力です。数値を入力してください");
            }
        }
    }

    //MPの入力よりわかりやすいloopの処理
    //loopの結果を変数に入れるという書き方、いいね
    let strength: u32 = loop {
        print!(" STRENGTH: ");
        io::stdout().flush().unwrap();
        
        let st_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
        match st_str.trim().parse() {
            //numをloopから取り出して、strengthに入れている
            Ok(num) => break num,
            Err(_) => println!("【エラー】正の整数を入力してください。"),
        }
    };

    let toughness: u32 = loop {
    print!(" TOUGHNESS: ");
    io::stdout().flush().unwrap();

    let tn_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let num: u32 =  match tn_str.trim().parse() {
        //
        Ok(n) => n,
        Err(_) => { println!("【エラー】正の整数を入力してください。");
                    //continueがあるので、ここからloop開始まで戻る
                    continue;
            } 
        };
    
    //ユーザーが意図しない数値を入れてしまった場合の処理の実験
    print!("入力された数値は「{}」です。OK？(y/n)", num);
    io::stdout().flush().unwrap();
    let ans_str = lines.next().expect("INPUT NULL").expect("INPUT Error");

    if ans_str.trim().to_lowercase() == "y" {
            //"y"を入れないとこのスコープに入れないので、breakできずに最初に戻る
            break num;
        }
    };

    //----ここからは疲れたのでHPの入力方法のコピペ----
    print!(" SPEED: ");
    io::stdout().flush().unwrap();

    let sp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let speed: u32 = sp_str.trim().parse().expect("数値を入力してください");

    print!(" LUCK: ");
    io::stdout().flush().unwrap();

    let lu_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let luck: u32 = lu_str.trim().parse().expect("数値を入力してください");

    HeroStatus { hp, mp, strength, toughness, speed, luck }
}//毒ダメージの設定や状態変化までたどり着けず、今日は終了
//0102_loop_enum_Write_match_test