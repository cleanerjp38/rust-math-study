trait DynamicGrowth {
    fn get_momentum(&self) -> f64;
}

struct NapierSystem {
    current_value: f64,
}

impl DynamicGrowth for NapierSystem {
    fn get_momentum(&self) -> f64 {
        self.current_value
    }
}

impl NapierSystem {
    fn tick (&mut self, dt:f64) {
        let momentum = self.get_momentum();
        self.current_value += momentum * dt;
    }
}

fn main() {
    let mut e = NapierSystem {current_value: 1.0};
    let dt = 0.0001;
    for _ in 0..10_000{
        e.tick(dt);
    }
    println!("1秒後の値：{}", e.current_value);
}
//0025_NepierSystem
