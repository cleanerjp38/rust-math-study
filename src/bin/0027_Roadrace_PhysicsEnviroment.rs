use std::io::{self, Write};

struct Gateway;

impl Gateway {
    fn get_input(prompt: &str) -> f64 {
        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line");

            match buf.trim().parse::<f64>() {
                Ok(val) if val > 0.0 => return val,
                _ => println!("Error: 正の数値を入力してください"),
            }
        }
    }
}

struct PhysicsEnvironment {
    rho: f64,     // 空気密度
    gravity: f64, // 重力加速度
}

struct Rider {
    height_m: f64,  // 身長
    weight_kg: f64, // 体重
    cd: f64,        // 抗力係数
}

impl PhysicsEnvironment {
    fn new() -> Self {
        Self {
            rho: 1.225,
            gravity: 9.81,
        }
    }
}

impl Rider {
    fn new(height_m: f64, weight_kg: f64) -> Self {
        Self {
            height_m,
            weight_kg,
            cd: 0.7,
        }
    }

    /// 身長から前面投影面積を推定する
    fn frontal_area(&self) -> f64 {
        0.02 * self.height_m + 0.15
    }

    /// 空気抵抗に打ち勝つための出力
    fn resistance_power(
        &self,
        v_kmh: f64,
        env: &PhysicsEnvironment,
        draft: f64,
    ) -> f64 {
        let v_ms = v_kmh / 3.6;

        0.5
            * env.rho
            * v_ms.powi(3)
            * self.cd
            * self.frontal_area()
            * draft
    }

    /// 10%勾配で重力に逆らうための出力
    fn climbing_power(
        &self,
        v_kmh: f64,
        env: &PhysicsEnvironment,
    ) -> f64 {
        let v_ms = v_kmh / 3.6;
        let grade: f64 = 0.10;
        let slope_angle = grade.atan();

        self.weight_kg
            * env.gravity
            * slope_angle.sin()
            * v_ms
    }
}

fn main() {
    let env = PhysicsEnvironment::new();

    println!("--- Miya式・自転車物理演算プロトコルを起動します ---");

    let rider = Rider {
        height_m: Gateway::get_input("はるさんの身長 (cm): ") / 100.0,
        weight_kg: Gateway::get_input("はるさんの体重 (kg): "),
        cd: 0.7,
    };

    let target_speed = Gateway::get_input("目標速度 (km/h): ");

    let power_solo = rider.resistance_power(target_speed, &env, 1.0);
    let power_draft = rider.resistance_power(target_speed, &env, 0.6);
    let power_climb =
        power_solo + rider.climbing_power(target_speed, &env);

    println!("\n【解析結果：10年後も通用する物理の真実】");
    println!("平地独走時の負荷: {:.2} W", power_solo);
    println!(
        "先輩の影に潜む時: {:.2} W (40%の負荷軽減に成功)",
        power_draft
    );
    println!(
        "文京区の激坂(10%)を駆け上がる時: {:.2} W",
        power_climb
    );

    println!(
        "\nプロトコル完了。この部品は将来の経済モデルにおける「移動効率解析」へ転用可能。"
    );
}
//0027_Roadrace_PhysicsEnviroment