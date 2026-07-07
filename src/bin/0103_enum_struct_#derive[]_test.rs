use std::io::{self, BufRead, Write};

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

enum HeroHealth {
    Normal,
    Staned,
    Poisoned {damage_per_turn: i32},
}

//ここで#[derive]を書かないと、enemy_attackを何度も使えないらしい
//まだよくわかってない
#[derive(Clone, Copy)]
enum Attacked {
    Punch,
    Smash,
    PoisonClow,
}

//ステータスを入力できるようにしたかった
fn input_status() -> HeroStatus {
    let mut stdin = io::stdin();
    let buf = io::BufReader::new(stdin.lock());
    let mut lines = buf.lines();

    println!("ステータスを入力してください");
    print!(" HP: ");
    io::stdout().flush().unwrap();

    let hp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let hp: i32 = hp_str.trim().parse().expect("数値を入力してください");


    let mp: i32; 
    loop {
        print!(" MP: ");
        io::stdout().flush().unwrap();
        let mp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");

        match mp_str.trim().parse::<i32>() {
            Ok(num) => {
                mp = num;
                break;
            }
            Err(_) => {
                println!("無効な入力です。数値を入力してください");
            }
        }
    }

    let strength: u32 = loop {
        print!(" STRENGTH: ");
        io::stdout().flush().unwrap();
        
        let st_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
        match st_str.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("【エラー】正の整数を入力してください。"),
        }
    };

    let toughness: u32 = loop {
    print!(" TOUGHNESS: ");
    io::stdout().flush().unwrap();

    let tn_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let num: u32 =  match tn_str.trim().parse() {
        Ok(n) => n,
        Err(_) => { println!("【エラー】正の整数を入力してください。");
                    continue;
            } 
        };
    
    print!("入力された数値は「{}」です。OK？(y/n)", num);
    io::stdout().flush().unwrap();
    let ans_str = lines.next().expect("INPUT NULL").expect("INPUT Error");

    if ans_str.trim().to_lowercase() == "y" {
            break num;
        }
    };

    print!(" SPEED: ");
    io::stdout().flush().unwrap();

    let sp_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let speed: u32 = sp_str.trim().parse().expect("数値を入力してください");

    print!(" LUCK: ");
    io::stdout().flush().unwrap();

    let lu_str = lines.next().expect("INPUT NULL").expect("INPUT Error");
    let luck: u32 = lu_str.trim().parse().expect("数値を入力してください");

    HeroStatus { hp, mp, strength, toughness, speed, luck }
}
//----ここまで、0102_loop_enum_Write_match_testのコピペ----

impl HeroHealth {
    //ここで「-> Self」で新しい自分を返すと、引数で「&mut self」と自分を更新することと被ってしまう
    //fn attacked(&mut self, enemy_attack:Attacked, hp: i32) -> Self {
    fn attacked(&mut self, enemy_attack:Attacked, hp: i32) {
        //match enemy_attack {
        //*selfは「&外し」。HeroHealthの状態を書き換える
        *self = match enemy_attack {
            Attacked::Punch => HeroHealth::Normal,
            Attacked::Smash => HeroHealth::Staned,
            Attacked::PoisonClow => HeroHealth::Poisoned { damage_per_turn: (hp - 10) },
        }
    }
}

impl HeroStatus {
    //->の先は型を宣言するので、self.hpのような変数は置けない
    //fn damage_cul(self, enemy_attack:Attacked) -> self.hp {
    fn damage_cul(&mut self, enemy_attack:Attacked) {
        match enemy_attack {
            Attacked::Punch => self.hp -= 20,
            Attacked::Smash => self.hp -= 30,
            Attacked::PoisonClow => self.hp -= 10,
        }
    }
}

impl Hero {
    fn receive_attack(&mut self, enemy_attack: Attacked) {
        println!("敵の攻撃！");
        
        self.status.damage_cul(enemy_attack);
        self.health.attacked(enemy_attack, self.status.hp);

        println!("{}の現在のHP: {}", self.name, self.status.hp);
    }
}

fn main() {
    let game_start = input_status();
    let mut my_hero = Hero{
        //名前も標準入力にすればよかった
        name: String::from("ワリオ"),
        status: game_start,
        health: HeroHealth::Normal,
    };

    my_hero.receive_attack(Attacked::PoisonClow);
}
//0103_enum_struct_#derive[]_test
