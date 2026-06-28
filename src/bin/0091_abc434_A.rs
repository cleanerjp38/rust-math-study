use std::io;

struct Weights {
    w_kg: u32,
    b_g: u32,
}

impl Weights {
    fn balloons(&self) -> u32 {
        let mut balloons = 0;
        //u32なので小数点は切り捨てのため、1を足している
        //これだとw=20 b=20のときに答えがちがう
        //let balloons: u32 = ((self.w_kg * 1000) / self.b_g) + 1;
        //あので、余りが出るかどうかifで分岐させた
        if (self.w_kg * 1000) % self.b_g > 0 {
            balloons = ((self.w_kg * 1000) / self.b_g) + 1;
        } else {
            balloons = (self.w_kg * 1000) / self.b_g;
        }

        balloons
    }
}

//構造体に標準入力を入れる練習
fn get_numbers() -> Weights {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut inputs = buffer.split_whitespace();
    let w: u32 = inputs.next().unwrap().parse().unwrap();
    let b: u32 = inputs.next().unwrap().parse().unwrap();

    Weights { w_kg: w, b_g: b }
}

fn main() {
    let weights = get_numbers();
    println!("{}", weights.balloons());
}
//0091_abc434_A