struct Destiny {
    message: String,
}

//運命の元で蝉が生きることを定義
struct Cicada<'a> {
    destiny_ref: &'a Destiny,
}

//蝉の動作の定義
impl<'a> Cicada<'a>  {
    fn cry(&self, day:usize) {
        println!("【{}日目】{}（今日も蝉は鳴いている）", day, self.destiny_ref.message);
    }
}

//関所の作成　ここで入力情報を変更できる
fn destiny_gateway() -> Destiny {
    Destiny { message: String::from("ミーン！") }
}

fn main() {
    let cycle_of_life = destiny_gateway();//世界が回っている
   
    {//蝉の一生を{}の中で表現
        let miya_semi = Cicada {
            destiny_ref: &cycle_of_life
        };

        for day in 1..=7 {
            miya_semi.cry(day);
        }
    }

    {//蝉はいないが世界は続く
        println!("【8日目】蝉はいない。そしてどこかで命の循環が起こる。（{}）", cycle_of_life.message)
    }
}
//0019_蝉のコード

//ライフタイムは「｛｝を箱と見立てて、外箱で作って内箱で使う」が基本
//<'a>は参照する側、'aは参照される側　される側より前にする側が消失してはいけない
