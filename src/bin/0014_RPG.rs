struct Player {
    name: String,
    level: u32,
    hp: i32,
}

impl Player {
    fn new(name: &str) -> Self {
        println!("---キャラクター「{}」が生成されました ---", name);
        Self {
            name: name.to_string(),
            level: 1,
            hp: 100,
        }
    }

    fn status(&self) {
        println!("[Status] Name: {}, Level: {}, HP: {}", self.name, self.level, self.hp);
    }

    fn take_damage(&mut self, damage: i32) {
    self.hp -= damage;
    println!("{} は {} のダメージを受けた！ (残りHP: {})", self.name, damage,self.hp);
    }
}

fn main() {
    let mut player = Player::new("Miya");

    player.status();
    player.take_damage(20);
    player.status();

    println!("--- 冒険は続く ---");
}
//0014_RPG
