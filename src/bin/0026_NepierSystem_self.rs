mod gateway;
use gateway::Gateway;

trait DynamicGrowth {
    fn get_momentum(&self) -> f64;
}

struct NapierSystem {
    current_value: f64,
    start_value: f64,
}

impl DynamicGrowth for NapierSystem {
    fn get_momentum(&self) -> f64 {
        self.current_value
    }
}

impl NapierSystem {
    fn tick(&mut self, dt: f64) {
        let momentum = self.get_momentum();
        self.current_value += momentum * dt;
    }

    fn new() -> Self {
        println!("初期値を入力してください（例 1.0）:");
        let mut gateway = Gateway::new();
        let start_val = gateway.next();
        let current_val = start_val;

        Self {
            current_value: (current_val),
            start_value: (start_val),
        }
    }
}

fn main() {
    let mut e = NapierSystem::new();
    let dt = 0.0001;

    for _ in 0..10_000 {
        e.tick(dt);
    }

    println!("1秒後の値：{}", e.current_value);
    println!("ネピア数：{}", e.current_value / e.start_value);
}
//0026_NepierSystem_self