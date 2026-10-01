mod gateway;
use gateway::Gateway;

struct RgbCard {
    r: i32,
    g: i32,
    b: i32,
}

impl RgbCard {
    fn as_int(&self) -> i32 {
        100*self.r + 10*self.g + self.b
    }
    fn is_multiple_of_4(&self) -> bool {
        self.as_int() % 4 == 0
    }
}

fn main() {
    let mut gateway =Gateway::new();
    let rgb_card = RgbCard {
        r: gateway.next(),
        g: gateway.next(),
        b: gateway.next(),
    };
    if rgb_card.is_multiple_of_4() {
        println!("YES");
    }else {
        println!("NO");
    }

}
//なんでas_intをfnで使わないのかわからなかったけど、is_multiple_of_4()の中で使われていたのか
//0018_RGBcard
