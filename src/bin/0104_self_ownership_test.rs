#[derive(Debug)]
struct TestOwnerStruct {
    a: u32,
    b: u32,
}

#[derive(Debug)]
enum TestOwnerEnum {
    A { data: u32 },
    B { data: u32 },
}

impl TestOwnerStruct {
    fn new() -> Self {
        Self { a: 10, b: 10 }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl TestOwnerEnum {
    fn new() -> Self {
        TestOwnerEnum::A { data: 10 }
    }

    fn num(&mut self, check_num: TestOwnerStruct) {
        match check_num {
            TestOwnerStruct { a, b } if a == b => {
                *self = TestOwnerEnum::B { data: a + b };
            }
            TestOwnerStruct { a, .. } => {
                *self = TestOwnerEnum::A { data: a };
            }
        }
    }
}

fn main() {
    let mut st = TestOwnerStruct::new();
    st.a = 50;
    
    let mut en = TestOwnerEnum::new();
    println!("初期状態: {:?}", en); 

    // 1. まず `st` を書き換えて、実験1を行う。
    // ※ st.reset() を先に行うことで、所有権が移動する前に st の状態変化を観察します。
    println!("stの変更前: {:?}", st); // a: 50, b: 10
    
    // 2. 所有権が移動する前に reset() の実験を行う
    st.reset(); 
    println!("reset後  : {:?}", st); // a: 10, b: 10（初期状態に戻る）

    // 3. 満を持して `st` を `en.num` に渡す（ここで st の所有権は移動し、st は使えなくなる）
    en.num(st);
    println!("実験1後(stを消費): {:?}", en); // a == b なので B { data: 20 } になる

    println!("-------------------");

    // 4. 新しい st2 を作って実験2
    let mut st2 = TestOwnerStruct::new();
    st2.a = 99; // あえて条件（a == b）を崩してみる
    en.num(st2); // ここで st2 の所有権が移動
    println!("実験2後(st2を消費): {:?}", en); // A { data: 99 } に戻る
}
//0104_self_ownership_test