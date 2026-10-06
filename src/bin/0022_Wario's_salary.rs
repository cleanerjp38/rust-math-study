mod gateway;

use gateway::Gateway;

struct Salary {
    x: f32,
    h: f32,
    t: f32,
}

impl Salary {
    fn calculate_salary(&self) -> f32 {
        if self.t <= self.h {
            // 残業なしの場合
            self.x * self.t
        } else {
            // 残業ありの場合
            let regular_pay = self.x * self.h;
            let overtime_pay = (self.t - self.h) * (self.x * 1.25);
            regular_pay + overtime_pay
        }
    }
}

fn main() {
    let mut gateway = Gateway::new();
    let input_x = gateway.next();
    let input_h = gateway.next();
    let input_t = gateway.next();

    let salary = Salary {
        x: input_x,
        h: input_h,
        t: input_t,
    };

    let result = salary.calculate_salary();
    println!(
        "ワリオの給与: {} 円（小数点以下切り捨て: {} 円）",
        result, result as u32
    );
}
//0022_Wario's_salary